use std::{sync::{atomic::{AtomicI64, Ordering}, Arc}, time::Duration};
use bp_scheduler::{config::actions::Selector, dynamic_tracking::{collision::Collision, Margins, TrackingSignal}};
use config::bodies::Race;
use tokio::{
    sync::mpsc::UnboundedSender,
    time::{sleep, Instant},
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info_span, trace, warn, Instrument};

use crate::{bridge::UnsafeActorPtr, Telekinesis};

#[derive(Clone, Debug)]
pub enum TrackingState {
    Init,
    MovingOut,
    OuterTurn,
    MovingIn,
    InnerTurn,
}

#[derive(Clone, Debug)]
pub struct BoneObserver {
    pub global_id: i32, // TODO: read only
    pub name: String, // TODO: read only
    pub settings: ObserverSettings, // TODO: read only
    pub stats: ObserverStats, // TODO: read only
    sender: UnboundedSender<TrackingSignal>,
    pub penetration_happened: CancellationToken,
    pub local_cancel: CancellationToken,
    pub global_cancel: CancellationToken,
}

impl BoneObserver {
    pub fn new(        
        name: String,
        sender: UnboundedSender<TrackingSignal>,
        global_cancel: CancellationToken,
        settings: ObserverSettings
    ) -> Self {

        BoneObserver { 
            penetration_happened: CancellationToken::new(),
            local_cancel: CancellationToken::new(),
            global_cancel,
            sender,
            name,
            global_id: -1,
            settings,
            stats: ObserverStats::default(),
        }
    }

    fn get_actual_collision(
        use_strapon: bool,
        collision: Collision,
        penetrator_body: &Race,
    ) -> Collision {
        if use_strapon {
            let mut c2: Collision = collision;
            c2.radius += penetrator_body.penis_extra_len;
            c2
        } else {
            collision
        }
    }

    pub fn observe(&mut self, lb: &mut Telekinesis, penetrator_body: &UnsafeActorPtr, passive_body: &UnsafeActorPtr) {
        let penetrator_bone = penetrator_body.get_bone(&self.settings.penetrator_bone);
        let passive_bone = passive_body.get_bone(&self.settings.passive_bone);

        let cancellation_token = self.local_cancel.clone();
        if penetrator_bone.ptr.is_null() {
            error!(penetrator_bone.name, "bone null, stopping");
            return;
        }
        if passive_bone.ptr.is_null() {
            error!(passive_bone.name, "bone null, stopping");
        }
    
        let initial_timeout_ms = lb.bone_tracking_config.stroker_settings.initial_timeout_ms;
        let sampling_rate_ms = lb.bone_tracking_config.stroker_settings.sampling_rate_ms;
    
        self.global_id = lb.tracking_counter;
        lb.tracking_counter += 1;

        let _self = self.clone();

        lb.client.runtime.spawn(async move {
            let span = info_span!("observe", t=_self.global_id);
            async move {
                let collision = BoneObserver::get_actual_collision(_self.settings.uses_strapon, _self.settings.collision, &_self.settings.penetrator_body);
                debug!(settings=?_self.settings, "observing...");
                sleep(Duration::from_millis(initial_timeout_ms)).await;
                let mut last_dist = f32::MAX;
                let mut most_outward = f32::MAX;
                let mut most_inward = f32::MAX;
                let mut penetrated = false;
                let mut state = TrackingState::Init;
    
                while !cancellation_token.is_cancelled() && !_self.global_cancel.is_cancelled() {
                    let dist = penetrator_bone.get_distance(&passive_bone);
    
                    _self.stats.stat_min_distance.fetch_min(dist as i64, Ordering::Relaxed);
    
                    let diff = last_dist - dist;
                    trace!("dist = {}", dist);
                    match state {
                        TrackingState::Init => {
                            if dist < collision.radius {
                                state = TrackingState::MovingIn;
                                _self.penetration_happened.cancel();
                                
                                debug!(?state, dist, "penetration!");
                                _self.stats.stat_pens.fetch_add(1, Ordering::Relaxed);
                                let _ = _self.sender.send(TrackingSignal::Penetration(Instant::now()));
                                penetrated = true;
                            }
                        }
                        TrackingState::MovingIn => {
                            if diff < 0.0 {
                                state = TrackingState::InnerTurn;
                                most_inward = dist;
                                trace!(?state, dist, most_inward);
                            }
                            if !penetrated && dist < collision.radius {
                                penetrated = true;
                                debug!(?state, dist, "penetration!");
                                _self.stats.stat_pens.fetch_add(1, Ordering::Relaxed);
                                let _ = _self.sender.send(TrackingSignal::Penetration(Instant::now()));
                            }
                        }
                        TrackingState::InnerTurn => {
                            if dist - most_inward > collision.error_tolerance {
                                let (most_in, most_out) =
                                collision.get_stroke_range(most_outward, most_inward);
                                let _ = _self.sender.send(TrackingSignal::InnerTurn(
                                    Instant::now(),
                                    Margins::new(most_in, most_out),
                                ));
                                state = TrackingState::MovingOut;
                                debug!(?state, dist, most_inward, "sending inner turn!");
                            } else if dist - most_inward < -collision.error_tolerance {
                                state = TrackingState::MovingIn;
                                warn!(                                ?state,
                                    dist, most_inward, "dist - most_inward < -ERROR_TOLERANCE"
                                );
                            }
                        }
                        TrackingState::MovingOut => {
                            if diff > 0.0 {
                                state = TrackingState::OuterTurn;
                                most_outward = dist;
                                trace!(?state, dist, most_outward);
                            }
                        }
                        TrackingState::OuterTurn => {
                            if most_outward - dist > collision.error_tolerance {
                                let (from, to) =
                                collision.get_stroke_range(most_outward, most_inward);
                                let _ = _self.sender.send(TrackingSignal::OuterTurn(
                                    Instant::now(),
                                    Margins::new(from, to),
                                ));
                                penetrated = false;
                                state = TrackingState::MovingIn;
                                debug!(?state, dist, most_outward, "sending outer turn!");
                            } else if most_outward - dist < -collision.error_tolerance {
                                state = TrackingState::MovingOut;
                                warn!(
                                    ?state,
                                    dist, most_inward, "most_outward - dist < -ERROR_TOLERANCE"
                                );
                            }
                        }
                    };
    
                    last_dist = dist;
                    sleep(Duration::from_millis(sampling_rate_ms)).await;
                    _self.stats.stat_run_ms.store(_self.stats.stat_start_time.elapsed().as_millis() as i64, Ordering::Relaxed);
                }
                if _self.global_cancel.is_cancelled() {
                    let _ = _self.sender.send(TrackingSignal::Stop);
                }
            }
            .instrument(span)
            .await;
    
        });
    }
}

#[derive(Clone, Debug)]
pub struct ObserverSettings {
    uses_strapon: bool,
    collision: Collision,
    penetrator_body: Race,
    pub selector: Selector, // TODO: should this be here?
    penetrator_bone: String,
    passive_bone: String,
}

impl ObserverSettings {
    pub fn new( uses_strapon: bool, collision: Collision, penetrator_body: &Race, passive_bone_name: &str, selector: Selector) -> ObserverSettings {
        ObserverSettings { 
            uses_strapon, 
            penetrator_body: penetrator_body.clone(), 
            selector, 
            penetrator_bone: penetrator_body.penetrator_bone.clone(), 
            passive_bone: passive_bone_name.into(), 
            collision
        }
    }
}

#[derive(Clone, Debug)]
pub struct ObserverStats {
    pub stat_pens: Arc<AtomicI64>,
    pub stat_min_distance: Arc<AtomicI64>,
    pub stat_run_ms: Arc<AtomicI64>,
    pub stat_start_time: Instant,
}

impl Default for ObserverStats {
    fn default() -> Self {
        Self { 
            stat_pens: Arc::new(AtomicI64::new(0)), 
            stat_min_distance: Arc::new(AtomicI64::new(i64::MAX)), 
            stat_run_ms: Arc::new(AtomicI64::new(0)), 
            stat_start_time: Instant::now()
        }
    }
}