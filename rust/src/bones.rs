use std::{sync::Arc, time::Duration};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::{sleep, Instant},
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, info_span, trace, Instrument};

use bp_scheduler::{
    actuator::Actuator,
    config::actuators::ActuatorSettings,
    dynamic_tracking::*,
    filter::Filter,
};
use buttplug::core::message::ActuatorType;

use config::{bodies::Race, body_parts::*};

use crate::{
    bridge::{ffi_bridge::*, *},
    Telekinesis,
};
use collision::Collision;
use ffi_bones::*;

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
pub enum TrackingState {
    Init,
    MovingOut,
    OuterTurn,
    MovingIn,
    InnerTurn,
}

fn get_body_for_actor(actor: &UnsafeActorPtr, races: &Vec<Race>) -> Option<Race> {
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
        _ => RaceSex::None,
    };

    let mut chosen_race = None;
    for race in races {
        if race.form_id == race_id && race.sex == actor_sex {
            chosen_race = Some(race.clone());
            debug!("found race {:?}", chosen_race);
        }
    }
    debug!("body: {:?}", chosen_race);
    chosen_race
}

pub fn lb_dynamic_tracking(lb: &mut Telekinesis, actor_vec: &ActorVec) {
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
        info!("player only scene");
        return;
    }

    let devices = lb.client.buttplug.devices();
    let (settings, enabled_position_actuators) =
        Filter::new(lb.client.device_settings.clone(), &devices)
            .connected()
            .enabled()
            .with_actuator_types(&[ActuatorType::Position])
            .result();
    lb.client.device_settings = settings;

    let mut starting_ramps = vec![];
    let player_actor = player_actor_opt.unwrap();
    let mut t_id = 0;

    let player_body = match get_body_for_actor(player_actor, &lb.races) {
        Some(race) => race,
        None => {
            error!(?lb.default_race_female, "player body not found using default");
            lb.default_race_female.clone().unwrap()
        }
    };

    // starts bone threads that monitor if any bone penetrates the player vaginally
    // (anal is simply included due to lack of distance, maybe this will be differentiated
    // at some point in the feature but I doubt it)

    for npc in npc_actors {
        t_id += 1;

        let npc_body = match get_body_for_actor(npc, &lb.races) {
            Some(race) => race,
            None => {
                error!(?lb.default_race_male, "player body not found using default");
                lb.default_race_male.clone().unwrap()
            }
        };

        let mut monitor_threads = vec![];

        if lb.consider_player_passive {
            // TODO: right now we always assume there is a strap-on, should we check that?

            let pen_bone = player_actor.get_bone(&player_body.penetrator_bone);
            info!("pen_bone is_null={}", pen_bone.ptr.is_null());

            let uses_strapon =
                player_actor.get_sex() == Sex::Female && npc.get_sex() == Sex::Female;

            fn get_actual_collision(
                use_strapon: bool,
                collision: Collision,
                penetrator_body: &Race,
            ) -> Collision {
                if use_strapon {
                    let mut c2 = collision;
                    c2.outer_distance += penetrator_body.strapon_extra_length;
                    c2
                } else {
                    collision
                }
            }

            // player on npc oral
            if npc_body.oral_collision.is_some() {
                monitor_threads.push((
                    get_actual_collision(
                        uses_strapon,
                        npc_body.oral_collision.unwrap(),
                        &player_body,
                    ),
                    npc.get_bone(&npc_body.oral_bone),
                    player_actor.get_bone(&player_body.penetrator_bone),
                    vec![TAG_PENIS, TAG_ORAL],
                    "oral active",
                ));
            }

            // player on npc anal
            if npc_body.anal_collision.is_some() {
                monitor_threads.push((
                    get_actual_collision(
                        uses_strapon,
                        npc_body.anal_collision.unwrap(),
                        &player_body,
                    ),
                    npc.get_bone(&npc_body.anal_bone),
                    player_actor.get_bone(&player_body.penetrator_bone),
                    vec![TAG_PENIS, TAG_VAGINAL, TAG_ANAL],
                    "anal/vaginal active",
                ));
            }

            // npc on player oral
            if player_body.oral_collision.is_some() {
                monitor_threads.push((
                    get_actual_collision(
                        uses_strapon,
                        player_body.oral_collision.unwrap(),
                        &npc_body,
                    ),
                    player_actor.get_bone(&player_body.oral_bone),
                    npc.get_bone(&npc_body.penetrator_bone),
                    vec![TAG_PENIS, TAG_ORAL],
                    "oral passive",
                ));
            }

            // npc on player anal
            if player_body.anal_collision.is_some() {
                monitor_threads.push((
                    get_actual_collision(
                        uses_strapon,
                        player_body.anal_collision.unwrap(),
                        &npc_body,
                    ),
                    player_actor.get_bone(&player_body.anal_bone),
                    npc.get_bone(&npc_body.penetrator_bone),
                    vec![TAG_PENIS, TAG_VAGINAL, TAG_ANAL],
                    "anal/vaginal passive",
                ));
            }
        }
        if monitor_threads.is_empty() {
            error!("no viable monitoring combination for scene actors")
        }
        for monitor_target in monitor_threads {
            let pen_signal = CancellationToken::new();
            let cancel_observation = observe_bones(
                lb,
                &monitor_target.1,
                &monitor_target.2,
                monitor_target.0,
                sender.clone(),
                pen_signal.clone(),
                global_cancel.clone(),
                monitor_target.4.into(),
            );
            starting_ramps.push((pen_signal, cancel_observation, monitor_target.3, t_id));
        }
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
                tracking_handle,
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
    tracking_handle: DynamicTrackingHandle,
) {
    let parts = body_parts
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut actuator_settings_clone = actuator_settings.clone();
    tokio::spawn(async move {
        let (_, actuators) = Filter::from_actuators(actuator_settings, actuators)
            .load_config(&mut actuator_settings_clone)
            .with_body_parts(&parts)
            .result();
        let mut dynamic = DynamicTracking {
            settings: dynamic_settings,
            signals: receiver,
            actuators,
            status: tracking_handle,
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
    global_cancel: CancellationToken,
    thread_name: String,
) -> CancellationToken {
    let cancel_me = CancellationToken::new();
    let bone1 = a1_bone.clone();
    let bone2 = a2_bone.clone();
    let cancellation_token = cancel_me.clone();
    if bone1.ptr.is_null() {
        error!(bone1.name, "bone null, stopping");
        return cancellation_token;
    }
    if bone2.ptr.is_null() {
        error!(bone2.name, "bone null, stopping");
        return cancellation_token;
    }

    let initial_timeout_ms = lb.dynamic_settings.initial_timeout_ms;
    let sampling_rate_ms = lb.dynamic_settings.sampling_rate_ms;

    let t_id = lb.tracking_counter;
    lb.tracking_counter += 1;
    lb.client.runtime.spawn(async move {
        let span = info_span!("observe_bones", id = t_id);
        async move {
            info!(
                "observation task {} '{}' started. bone1={} - bone2={}. collision_sphere={:?}",
                t_id, thread_name, bone1.name, bone2.name, collision_sphere
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
                            debug!(?state, dist, most_inward);
                        }
                        if !penetrated && dist < collision_sphere.outer_distance {
                            penetrated = true;
                            let _ = sender.send(TrackingSignal::Penetration(Instant::now()));
                        }
                    }
                    TrackingState::InnerTurn => {
                        if dist - most_inward > collision_sphere.error_tolerance {
                            let (most_in, most_out) =
                                collision_sphere.get_stroke_range(most_outward, most_inward);
                            let _ = sender.send(TrackingSignal::InnerTurn(
                                Instant::now(),
                                Margins::new(most_in, most_out),
                            ));
                            info!("sending inner turn!");
                            state = TrackingState::MovingOut;
                            debug!(?state, dist, most_inward);
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
                            debug!(?state, dist, most_outward);
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
                            debug!(?state, dist, most_outward);
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
                sleep(Duration::from_millis(sampling_rate_ms)).await;
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
