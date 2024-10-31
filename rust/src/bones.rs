use std::{sync::{atomic::{AtomicI64, Ordering}, Arc}, time::Duration};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::{sleep, Instant},
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, info_span, trace, Instrument};

use buttplug::core::message::ActuatorType;
use bp_scheduler::{
    actuator::Actuator, config::{actions::Control, actuators::ActuatorSettings}, dynamic_tracking::*, filter::Filter,
};

use config::{
    bodies::Race,
    body_parts::*,
};

use ffi_bones::*;
use crate::{
    bridge::{ffi_bridge::*, *},
    Telekinesis,
};
use collision::Collision;

use config::bodies::Sex as RaceSex;

#[cxx::bridge]
pub mod ffi_bones {
    #[namespace = "RE"]
    unsafe extern "C++" {
        type Actor = crate::bridge::ffi_bridge::Actor;
        type NiAVObject = crate::bridge::ffi_bridge::NiAVObject;
    }

    unsafe extern "C++" {
        include!("Bones.h");
        pub type ActorVec;
        pub fn GetActor(self: &ActorVec, pos: i32) -> *const Actor;
        pub fn Size(self: &ActorVec) -> i32;
        unsafe fn GetDistance(boneA: *const NiAVObject, boneB: *const NiAVObject) -> f32;
    }

    extern "Rust" {
        fn lb_dynamic_stop();
    }
}

impl UnsafeAvObjectPtr {
    fn get_distance(&self, other: &UnsafeAvObjectPtr) -> f32 {
        if self.ptr.is_null() {
            error!("bone is null {}", self.name);
            f32::MAX
        } else if other.ptr.is_null() {
            error!("bone is null {}", other.name);
            f32::MAX
        } else {
            unsafe { GetDistance(self.ptr, other.ptr) }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DynamicTrackingHandle {
    pub cancel: Option<CancellationToken>,
    pub cur_avg_ms: Arc<AtomicI64>,
    pub cur_depth: Arc<AtomicI64>
}

impl DynamicTrackingHandle {
    pub fn reset(&mut self) {
        self.cur_avg_ms.store(0, Ordering::Relaxed);
        self.cur_depth.store(0, Ordering::Relaxed);
    }
}

impl Default for DynamicTrackingHandle {
    fn default() -> Self {
        Self { 
            cancel: None, 
            cur_avg_ms: Arc::new(AtomicI64::new(0)), 
            cur_depth: Arc::new(AtomicI64::new(0))
        }
    }
}

#[derive(Clone, Debug)]
pub enum TrackingState {
    Init,
    MovingOut,
    OuterTurn,
    MovingIn,
    InnerTurn,
}

fn get_body_for_actor(
    actor: &UnsafeActorPtr,
    races: &Vec<Race>
) -> Option<Race> {
    let race_form: UnsafeTESFormPtr = actor.get_race().into();
    let race_id = race_form.get_form_id();
    let sex = actor.get_sex();
    debug!(
        "fetching body for sex={:?} player={} race_id={:X}",
        sex,
        actor.is_player(),
        race_id
    );

    let actor_sex = match actor.get_sex() {
        Sex::Female => RaceSex::Female,
        Sex::Male => RaceSex::Male,
        _ => RaceSex::None
    };
    
    let mut chosen_race = None;
    for race in races {
        if race.form_id == race_id &&
           race.sex == actor_sex
        {
            chosen_race = Some(race.clone());
            debug!("found race {:?}", chosen_race);
        }
    }
    debug!("body: {:?}", chosen_race);
    chosen_race
}

pub fn lb_dynamic_tracking(lb: &mut Telekinesis, actor_vec: &ActorVec, _control: Control) {
    info!("lb_dynamic_tracking Actors={}", actor_vec.Size());
    let actors_in = from_actor_vec(actor_vec);

    if let Some(token) = lb.dynamic_task.cancel.take() {
        token.cancel();
    }
    let global_cancel = CancellationToken::new();
    lb.dynamic_task.reset();
    lb.dynamic_task.cancel = Some(global_cancel.clone());

    let (sender, receiver) = unbounded_channel::<TrackingSignal>();

    for actor in &actors_in {
        info!("actor {:?} Player:{}", actor.get_sex(), actor.is_player());
    }

    let player_actor_opt = actors_in.iter().find(|x| x.is_player());
    if player_actor_opt.is_none() {
        error!("no player animation, stopping bone tracking");
        return;
    };

    if lb.default_race_female.is_none() || lb.default_race_male.is_none() {
        error!("default race is not defined");
        return;
    }

    let npc_actors = actors_in
        .iter()
        .filter(|x| !x.is_player())
        .collect::<Vec<&UnsafeActorPtr>>();
    if npc_actors.is_empty() {
        error!("no other actors, stop bone tracking");
    }

    let devices = lb.client.buttplug.devices();
    let (settings, enabled_position_actuators) =
        Filter::new(lb.client.device_settings.clone(), &devices)
            .connected()
            .enabled()
            .with_actuator_types(&[ActuatorType::Position])
            .result();
    lb.client.device_settings = settings;

    let npc_male_actors = actors_in
        .iter()
        .filter(|x| !x.is_player() && x.get_sex() == Sex::Male)
        .collect::<Vec<&UnsafeActorPtr>>();

    let mut starting_ramps = vec![];
    let player_actor = player_actor_opt.unwrap();
    let mut t_id = 0;
    if player_actor.get_sex() == Sex::Female {
        let player_body = match get_body_for_actor(player_actor, &lb.races) {
            Some(race) => race,
            None => {
                error!(?lb.default_race_female, "player body not found using default");
                lb.default_race_female.clone().unwrap()
            },
        };

        // starts bone threads that monitor if any bone penetrates the player vaginally
        // (anal is simply included due to lack of distance, maybe this will be differentiated
        // at some point in the feature but I doubt it)
        if !npc_male_actors.is_empty() {
            for npc in npc_male_actors {
                t_id += 1;

                let npc_body = match get_body_for_actor(npc, &lb.races ) {
                    Some(race) => race,
                    None => { 
                        error!(?lb.default_race_male, "player body not found using default");
                        lb.default_race_male.clone().unwrap() 
                    },
                };

                let player_pelvis = &player_body.genital_bone;
                let player_head = &player_body.oral_bone;
                let npc_penis = &npc_body.genital_bone;

                if let Some(oral_collision) = player_body.oral_collision {
                    // F/M oral collision
                    // Use collision sphere of player head
                    let pen_signal = CancellationToken::new();
                    let cancel_observation = observe_bones(
                        lb,
                        &player_actor.get_bone(player_head),
                        &npc.get_bone(npc_penis),
                        oral_collision,
                        sender.clone(),
                        pen_signal.clone(),
                        global_cancel.clone()
                    );
                    starting_ramps.push((
                        pen_signal,
                        cancel_observation,
                        vec![TAG_PENIS, TAG_ORAL],
                        t_id,
                    ));
                } else {
                    error!(?player_head, "actor has no head collision");
                }

                if let Some(penis_collision) = npc_body.genital_collision {
                    // F/M genital collision
                    // use collision sphere of male and genital collision
                    let pen_signal = CancellationToken::new();
                    let cancel_observation = observe_bones(
                        lb,
                        &player_actor.get_bone(player_pelvis),
                        &npc.get_bone(npc_penis),
                        penis_collision,
                        sender.clone(),
                        pen_signal.clone(),
                        global_cancel.clone()
                    );
                    starting_ramps.push((
                        pen_signal,
                        cancel_observation,
                        vec![TAG_PENIS, TAG_VAGINAL, TAG_ANAL],
                        t_id,
                    ));

                } else {
                    error!(?npc_penis, "actor has no penis collision");
                }
            }
        } else {
            error!("F/F not implemented yet");
        }
        // any actor penetration player oral
        //  -> penis, oral
    } else {
        error!("player male not implemented yet");
        // player penetrates vaginal
        //   -> penis, vaginal, anal

        // player penetraties oral
        //   -> penis, oral
    }

    let tracking_handle = lb.dynamic_task.clone();
    if !starting_ramps.is_empty() {
        let dynamic_settings_clone = lb.dynamic_settings.clone();
        let actuator_settings_clone = lb.client.device_settings.clone();
        lb.client.runtime.spawn(async move {
            let mut winner = None;
            while winner.is_none() {
                sleep(Duration::from_millis(200)).await;
                for (i, ramp) in starting_ramps.iter().enumerate() {
                    if ramp.0.is_cancelled() {
                        debug!("id={} penetrated, closing all remaining thredas", ramp.3);
                        for (j, loser_thread) in starting_ramps.iter().enumerate() {
                            if j != i {
                                debug!("cancelling other tracking thread {}", j);
                                loser_thread.1.cancel();
                            }
                        }
                        winner = Some(ramp);
                        break;
                    }
                }
            }
            start_control_thread(
                dynamic_settings_clone,
                actuator_settings_clone,
                &winner.unwrap().2,
                receiver,
                enabled_position_actuators,
                tracking_handle.clone().cur_avg_ms,
                tracking_handle.cur_depth
            );
        });
    }
}

fn start_control_thread(
    dynamic_settings: DynamicSettings,
    actuator_settings: ActuatorSettings,
    body_parts: &[&str],
    receiver: UnboundedReceiver<TrackingSignal>,
    actuators: Vec<Arc<Actuator>>,
    cur_avg_ms: Arc<AtomicI64>,
    cur_depth: Arc<AtomicI64>
) {
    let parts = body_parts
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    tokio::spawn(async move {
        let (_, actuators) = Filter::from_actuators(actuator_settings, actuators)
            .with_body_parts(&parts)
            .result();
        let mut dynamic = DynamicTracking {
            settings: dynamic_settings,
            signals: receiver,
            actuators,
            cur_avg_ms,
            cur_depth,
        };
        info!(?dynamic.settings, ?parts, "control task started with settings");
        let _ = dynamic.track_mirror().await;
    });
}

fn observe_bones(
    lb: &mut Telekinesis,
    a1_bone: &UnsafeAvObjectPtr,
    a2_bone: &UnsafeAvObjectPtr,
    collision_sphere: Collision,
    sender: UnboundedSender<TrackingSignal>,
    first_pen: CancellationToken,
    global_cancel: CancellationToken
) -> CancellationToken {
    let cancel_me = CancellationToken::new();
    let bone1 = a1_bone.clone();
    let bone2 = a2_bone.clone();
    let cancellation_token = cancel_me.clone();
    if bone1.ptr.is_null() {
        error!( bone1.name, "bone null, stopping");
        return cancellation_token;
    }
    if bone2.ptr.is_null() {
        error!( bone2.name, "bone null, stopping");
        return cancellation_token;
    }

    let initial_timeout_ms = lb.dynamic_settings.initial_timeout_ms;
    let sample_ms = lb.dynamic_settings.sample_ms;

    let t_id = lb.tracking_counter;
    lb.tracking_counter += 1;
    lb.client.runtime.spawn(async move {
        let span = info_span!("observe_bones", id = t_id);
        async move {
            info!(
                "observation task {} started. bone1={} - bone2={}. collision_sphere={:?}",
                t_id, bone1.name, bone2.name, collision_sphere
            );
            sleep(Duration::from_millis(initial_timeout_ms)).await;
            let mut last_dist = f32::MAX;
            let mut most_outward = f32::MAX;
            let mut most_inward = f32::MAX;
            let mut penetrated = false;
            let mut state = TrackingState::Init;
            while !cancellation_token.is_cancelled() && !global_cancel.is_cancelled() {
                let dist = bone1.get_distance(&bone2);
                let diff = last_dist - dist;
                trace!("dist = {}", dist);
                match state {
                    TrackingState::Init => {
                        if dist < collision_sphere.outer_distance {
                            state = TrackingState::MovingIn;
                            info!(?state, dist);
                            first_pen.cancel();
                            let _ = sender.send(TrackingSignal::Penetration(Instant::now()));
                            penetrated = true;
                        }
                    }
                    TrackingState::MovingIn => {
                        if diff < 0.0 {
                            state = TrackingState::InnerTurn;
                            most_inward = dist;
                            info!(?state, dist, most_inward);
                        }
                        if !penetrated && dist < collision_sphere.outer_distance {
                            penetrated = true;
                            let _ = sender.send(TrackingSignal::Penetration(Instant::now()));
                        }
                    }
                    TrackingState::InnerTurn => {
                        if dist - most_inward > collision_sphere.error_tolerance {
                            let (from, to) =
                                collision_sphere.get_stroke_range(most_outward, most_inward);
                            let _ = sender.send(TrackingSignal::InnerTurn(
                                Instant::now(),
                                Margins::new(from, to),
                            ));
                            info!("sending inner turn!");
                            state = TrackingState::MovingOut;
                            info!(?state, dist, most_inward);
                        } else if dist - most_inward < -collision_sphere.error_tolerance {
                            state = TrackingState::MovingIn;
                            error!(
                                ?state,
                                dist, most_inward, "dist - most_inward < -ERROR_TOLERANCE"
                            );
                        }
                    }
                    TrackingState::MovingOut => {
                        if diff > 0.0 {
                            state = TrackingState::OuterTurn;
                            most_outward = dist;
                            info!(?state, dist, most_outward);
                        }
                    }
                    TrackingState::OuterTurn => {
                        if most_outward - dist > collision_sphere.error_tolerance {
                            let (from, to) =
                                collision_sphere.get_stroke_range(most_outward, most_inward);
                            let _ = sender.send(TrackingSignal::OuterTurn(
                                Instant::now(),
                                Margins::new(from, to),
                            ));
                            info!("sending outer turn!");
                            penetrated = false;
                            state = TrackingState::MovingIn;
                            info!(?state, dist, most_outward);
                        } else if most_outward - dist < -collision_sphere.error_tolerance {
                            state = TrackingState::MovingOut;
                            error!(
                                ?state,
                                dist, most_inward, "most_outward - dist < -ERROR_TOLERANCE"
                            );
                        }
                    }
                };

                last_dist = dist;
                sleep(Duration::from_millis(sample_ms)).await;
            }
            if global_cancel.is_cancelled() {
                let _ = sender.send(TrackingSignal::Stop);
            }
            info!("observation task stopped");
        }
        .instrument(span)
        .await
    });
    cancel_me
}

fn from_actor_vec(actors: &ActorVec) -> Vec<UnsafeActorPtr> {
    let mut vec_clone = vec![];
    for i in 0..actors.Size() {
        vec_clone.push(UnsafeActorPtr {
            ptr: actors.GetActor(i),
        });
    }
    vec_clone
}

pub fn lb_dynamic_stop() {
    info!("lb_dynamic_stop");
    Telekinesis::run_static(
        |lb| {
            if let Some(token) = lb.dynamic_task.cancel.take() {
                token.cancel();
            }
        },
        (),
    );
}
