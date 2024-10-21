use std::{sync::Arc, time::Duration};

use buttplug::core::message::ActuatorType;
use config::bodies::{BodyType, Bone};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::{sleep, Instant},
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::{
    bridge::ffi_bridge::{GetSex, IsPlayer, Sex},
    ffi::*,
    Lovebug,
};
use ffi_bones::{ActorVec, GetBoneFromActor, GetDistance, NiAVObject};

use bp_scheduler::{
    actuator::Actuator, config::actuators::ActuatorSettings, dynamic_tracking::*, filter::Filter
};
use collision::Collision;

#[cxx::bridge]
mod ffi_bones {
    #[namespace = "RE"]
    unsafe extern "C++" {
        type Actor = crate::ffi::Actor;
        type NiAVObject;
    }

    unsafe extern "C++" {
        include!("Bones.h");
        type ActorVec;
        pub fn GetActor(self: &ActorVec, pos: i32) -> *const Actor;
        pub fn Size(self: &ActorVec) -> i32;
        unsafe fn GetDistance(boneA: *mut NiAVObject, boneB: *mut NiAVObject) -> f32;
        unsafe fn GetBoneFromActor(actor: *const Actor, bone: &str) -> *mut NiAVObject;
    }

    extern "Rust" {
        fn lb_dynamic_tracking(actors: &ActorVec);
        fn lb_dynamic_stop();
    }
}

struct BodyTypes {
    female: BodyType,
    male: BodyType
}

fn get_body_types() -> BodyTypes {
    let pelvis_collision = Collision {
        outer_distance: 14.5,
        depth: 10.0,
        min_stroke: 0.25,
    };

    BodyTypes { female: BodyType {
        name: "Fusion Girl".into(),
        is_female: true,
        genital_bone: Bone {
            name: "Pelvis_skin".into(),
            collision: Some(pelvis_collision),
        },
        oral_bone: Bone {
            name: "HEAD".into(),
            collision: Some(pelvis_collision),
        },
        anal_bone: Bone {
            name: "Pelvis_skin".into(),
            collision: Some(pelvis_collision),
        },
    }, male: BodyType {
        name: "Body Talk".into(),
        is_female: false,
        genital_bone: Bone {
            name: "Penis_01".into(),
            collision: None,
        },
        oral_bone: Bone {
            name: "HEAD".into(),
            collision: Some(pelvis_collision),
        },
        anal_bone: Bone {
            name: "Anus_01".into(),
            collision: Some(pelvis_collision),
        },
    } }
}

pub fn lb_dynamic_tracking(actor_vec: &ActorVec) {
    info!("lb_dynamic_tracking Actors={}", actor_vec.Size());
    let actors_in = from_actor_vec(actor_vec);

    let dynamic_settings = DynamicSettings {
        move_at_start: true,
        min_resolution_ms: 80,
        min_duration_ms: 200,
        default_stroke_ms: 400,
        default_stroke_in: 0.0,
        default_stroke_out: 1.0,
        stroke_window_ms: 2_000,
        min_turn_dist: 0.25,
    };

    Lovebug::run_static(
        |lb| {
            if let Some(token) = lb.dynamic_task.take() {
                token.cancel();
            }
            let cancellation_token = CancellationToken::new();
            lb.dynamic_task = Some(cancellation_token.clone());

            let (sender, receiver) = unbounded_channel::<TrackingSignal>();

            for actor in &actors_in {
                info!("actor {:?} Player:{}", actor.get_sex(), actor.is_player());
            }

            let player_actor_opt = actors_in.iter().find(|x| x.is_player());
            if player_actor_opt.is_none() {
                error!("no player animation, stopping bone tracking");
                return;
            };

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
                debug!("player female");
                let body_types = get_body_types();

                // starts bone threads that minotir of any bone penetrates the player vaginally
                // (anal is simply included due to lack of distance, maybe this will be differentiated 
                // at some point in the feature but I doubt it)
                if !npc_male_actors.is_empty() {
                    for npc in npc_male_actors {
                        t_id += 1;
                        let first_pen_1 = CancellationToken::new();
                        let female_collision = body_types.female.genital_bone.clone();
                        let genital_bone_player = &player_actor.get_bone(&female_collision.name);
                        let genital_bone_other = &npc.get_bone(&body_types.male.genital_bone.name);
                        let stop_track = observe_bones(
                            lb,
                            genital_bone_player,
                            genital_bone_other,
                            female_collision.collision.unwrap(),
                            sender.clone(),
                            first_pen_1.clone(),
                            t_id,
                        );
                        starting_ramps.push((
                            first_pen_1,
                            stop_track,
                            vec!["penis", "vaginal", "anal"],
                            t_id,
                        ));
                    }
                } else {
                    error!("only female actors TODO");
                }
                // any actor penetration player oral
                //  -> penis, oral
            } else {
                error!("player male TODO");
                // player penetrates vaginal
                //   -> penis, vaginal, anal

                // player penetraties oral
                //   -> penis, oral
            }

            if !starting_ramps.is_empty() {
                let setting_clone = lb.client.device_settings.clone();
                lb.client.runtime.spawn(async move {
                    let mut winner = None;
                    while winner.is_none() {
                        sleep(Duration::from_millis(200)).await;
                        for (i, ramp) in starting_ramps.iter().enumerate() {
                            if ramp.0.is_cancelled() {
                                debug!("t_id={} penetrated, closing all remaining thredas", ramp.3);
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
                        dynamic_settings,
                        setting_clone,
                        &winner.unwrap().2,
                        receiver,
                        enabled_position_actuators,
                    );
                });
            }

            fn start_control_thread(
                dynamic_settings: DynamicSettings,
                actuator_settings: ActuatorSettings,
                body_parts: &[&str],
                receiver: UnboundedReceiver<TrackingSignal>,
                actuators: Vec<Arc<Actuator>>,
            ) {
                let parts = body_parts.iter().map(|s| s.to_string()).collect::<Vec<String>>();
                tokio::spawn(async move {
                    let (_, actuators) =  Filter::from_actuators(actuator_settings, actuators).with_body_parts(&parts).result();
                    let dynamic = DynamicTracking {
                        settings: dynamic_settings,
                        signals: receiver,
                        actuators,
                    };
                    info!(?dynamic.settings, "control task started with settings");
                    let _ = dynamic.track_mirror().await;
                });
            }

            fn observe_bones(
                lb: &Lovebug,
                a1_bone: &UnsafeAvObjectPtr,
                a2_bone: &UnsafeAvObjectPtr,
                collision_sphere: Collision,
                sender: UnboundedSender<TrackingSignal>,
                first_pen: CancellationToken,
                t_id: i32,
            ) -> CancellationToken {
                let cancel_me = CancellationToken::new();
                let bone1 = a1_bone.clone();
                let bone2 = a2_bone.clone();
                let cancellation_token = cancel_me.clone();

                let MIN_STROKE_LENGTH = 0.25;

                lb.client.runtime.spawn(async move {
                    info!("observation task {} started {} - {}", t_id, bone1.name, bone2.name);
                    sleep(Duration::from_millis(1200)).await;
                    let mut penetrating = false;
                    let mut dir_inward = false;
                    let mut last_distance = f32::MAX;
                    let mut most_outward = f32::MAX;
                    let mut most_inward = 0.0;

                    let mut stroke_len = 0.0;
                    let mut turn_done = false;

                    while !cancellation_token.is_cancelled() {
                        let dist = bone1.get_distance(&bone2);
                        if dist < collision_sphere.outer_distance {
                            if !penetrating {
                                first_pen.cancel();
                                info!("sending penetration {}", dist);
                                let _ = sender.send(TrackingSignal::Penetration(Instant::now()));
                            }
                            penetrating = true;
                        }

                        debug!("dist = {}", dist);
                        let diff = last_distance - dist;
                        if diff > 0.0 && !dir_inward {
                            most_outward = dist;
                            info!(most_outward, most_inward, "moving inward now");

                            dir_inward = true;
                            penetrating = false;
                            turn_done = false;
                            stroke_len = 0.0;
                        } else if diff < 0.0 && dir_inward {
                            most_inward = dist;
                            info!(most_outward, most_inward, "moving outward now");
                            
                            dir_inward = false;
                            penetrating = false;
                            turn_done = false;
                            stroke_len = 0.0;
                        }
                        stroke_len += diff;

                        debug!(stroke_len);
                        if f32::abs(stroke_len) > MIN_STROKE_LENGTH && !turn_done {
                            debug!("SENDING!");
                            turn_done = true;
                            let (from, to) = collision_sphere.get_stroke_range(most_outward, most_inward);
                            if dir_inward {
                                let _ = sender.send(TrackingSignal::OuterTurn(
                                    Instant::now(),
                                    Margins::new(from, to),
                                ));
                                info!(from, to, dist, "sending outward complete");
                            } else {
                                let _ = sender.send(TrackingSignal::InnerTurn(
                                    Instant::now(),
                                    Margins::new(from, to),
                                ));
                                info!(from, to, dist, "sending inward complete");
                            }
                            
                        } 

                        last_distance = dist;
                        sleep(Duration::from_millis(50)).await;
                    }
                    info!(t_id, "tracking thread stopped");
                });
                cancel_me
            }
        },
        (),
    );
}

pub enum TrackingState {
    Init,
    MovingOut,
    TurningOutward,
    MovingIn,
    TurningInward
}

#[derive(Clone)]
struct UnsafeActorPtr {
    pub actor: *const Actor,
}
impl UnsafeActorPtr {
    fn get_bone(&self, bone_name: &str) -> UnsafeAvObjectPtr {
        let av_object = unsafe { GetBoneFromActor(self.actor, bone_name) };
        UnsafeAvObjectPtr {
            av_object,
            name: bone_name.to_owned(),
        }
    }
    fn get_sex(&self) -> Sex {
        unsafe { GetSex(self.actor) }
    }
    fn is_player(&self) -> bool {
        unsafe { IsPlayer(self.actor) }
    }
}
unsafe impl Send for UnsafeActorPtr {}

#[derive(Clone)]
struct UnsafeAvObjectPtr {
    pub name: String,
    pub av_object: *mut NiAVObject,
}
impl UnsafeAvObjectPtr {
    fn get_distance(&self, other: &UnsafeAvObjectPtr) -> f32 {
        if self.av_object.is_null() {
            error!("bone is null {}", self.name);
            f32::MAX
        } else if other.av_object.is_null() {
            error!("bone is null {}", other.name);
            f32::MAX
        } else {
            unsafe { GetDistance(self.av_object, other.av_object) }
        }
    }
}
unsafe impl Send for UnsafeAvObjectPtr {}

fn from_actor_vec(actors: &ActorVec) -> Vec<UnsafeActorPtr> {
    let mut vec_clone = vec![];
    for i in 0..actors.Size() {
        vec_clone.push(UnsafeActorPtr {
            actor: actors.GetActor(i),
        });
    }
    vec_clone
}

pub fn lb_dynamic_stop() {
    info!("lb_dynamic_stop");
    Lovebug::run_static(
        |lb| {
            if let Some(token) = lb.dynamic_task.take() {
                token.cancel();
            }
        },
        (),
    );
}
