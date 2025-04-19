use std::{sync::{atomic::Ordering, Arc}, time::Duration};

use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver},
    time::sleep,
};

use tokio_util::sync::CancellationToken;

use tracing::{debug, error, info, trace, warn};

use buttplug::core::message::ActuatorType;

use bp_scheduler::{
    actuator::Actuator,
    config::{actions::Selector, actuators::ActuatorSettings},
    dynamic_tracking::*,
    filter::Filter,
};
use config::{bodies::Race, body_parts::*, bodies::Sex as RaceSex};
use ffi_bones::*;

use crate::{bone_tracking::{observer::{BoneObserver, ObserverSettings}, config::BoneTrackingSettings}, bridge::{ffi_bridge::Sex, UnsafeActorPtr, UnsafeAvObjectPtr, UnsafeTESFormPtr}, Telekinesis};

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
    pub fn get_distance(&self, other: &UnsafeAvObjectPtr) -> f32 {
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

fn get_body_for_actor(actor: &UnsafeActorPtr, races: &Vec<Race>) -> Option<Race> {
    let race_form: UnsafeTESFormPtr = actor.get_race().into();
    let race_id = race_form.get_form_id();
    let sex = actor.get_sex();
    let is_player = actor.is_player();
    trace!(?sex, ?race_id, is_player, "fetching race for body");
    trace!(?races);
    let actor_sex = match sex {
        Sex::Female => RaceSex::Female,
        Sex::Male => RaceSex::Male,
        _ => RaceSex::None,
    };

    let mut chosen_race = None;
    for race in races {
        if race.form_id == race_id && race.sex == actor_sex {
            chosen_race = Some(race.clone());
        }
    }
    if chosen_race.is_none() {
        warn!("body for race {:#X} not found", race_id);
    }
    trace!(?chosen_race);
    chosen_race
}

pub fn start_bone_tracking(lb: &mut Telekinesis, actor_vec: &ActorVec) {
    let actors_in = from_actor_vec(actor_vec);
    info!("start_bone_tracking");
    for (i, actor) in actors_in.iter().enumerate() {
        info!(sex=?actor.get_sex(), is_player=actor.is_player(), "Actor[{}] Race={:#X}", i, actor.get_race().get_form_id());
    }

    if let Some(token) = lb.dynamic_task.cancel.take() {
        token.cancel();
    }
    let global_cancel = CancellationToken::new();
    lb.dynamic_task.reset();
    lb.dynamic_task.cancel = Some(global_cancel.clone());

    let (sender, receiver) = unbounded_channel::<TrackingSignal>();
    let player_actor_opt = actors_in.iter().find(|x| x.is_player());
    if player_actor_opt.is_none() {
        error!("no player animation, stopping bone tracking");
        return;
    };

    if lb.bone_tracking_config.default_race_female.is_none()
        || lb.bone_tracking_config.default_race_male.is_none()
    {
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

    let player_body = match get_body_for_actor(player_actor, &lb.bone_tracking_config.races) {
        Some(race) => race,
        None => {
            error!("using default female race {:?} for player", lb.bone_tracking_config.default_race_female);
            lb.bone_tracking_config.default_race_female.clone().unwrap()
        }
    };

    // starts bone threads that monitor if any bone penetrates the player vaginally
    // (anal is simply included due to lack of distance, maybe this will be differentiated
    // at some point in the feature but I doubt it)

    for npc in npc_actors {

        let npc_body = match get_body_for_actor(npc, &lb.bone_tracking_config.races) {
            Some(race) => race,
            None => {
                error!("using default male race {:?} for npc race {:#}", lb.bone_tracking_config.default_race_male, npc.get_race().get_form_id());
                lb.bone_tracking_config.default_race_male.clone().unwrap()
            }
        };

        debug!(?player_body, ?npc_body, "Starting bone monitoring threads for Player + NPC({})", npc_body.name);
        if lb.bone_tracking_config.consider_player_passive {
            // TODO: right now we always assume there is a strap-on, should we check that?
            let uses_strapon =
                player_actor.get_sex() == Sex::Female && npc.get_sex() == Sex::Female;
                
            // player on npc oral
            if npc_body.oral_collision.is_some() {
                let settings = ObserverSettings::new(
                    uses_strapon,
                    npc_body.oral_collision.unwrap(),
                    &player_body,
                    &npc_body.oral_bone,
                    Selector::body_parts(vec![TAG_PENIS.into(), TAG_ORAL.into()])
                );
                let mut observer = BoneObserver::new(
                    "oral (player penetrates)".into(),
                    sender.clone(), 
                    global_cancel.clone(),
                    settings
                );
                observer.observe(lb, player_actor,  npc);
                starting_ramps.push(observer);
            }

            // player on npc anal
            if npc_body.anal_collision.is_some() {
                let settings = ObserverSettings::new(
                    uses_strapon,
                    npc_body.anal_collision.unwrap(),
                    &player_body,
                    &npc_body.anal_bone,
                    Selector::body_parts(vec![
                        TAG_PENIS.into(),
                        TAG_VAGINAL.into(),
                        TAG_ANAL.into(),
                    ])
                );
                let mut observer = BoneObserver::new(
                    "anal (player penetrates)".into(), 
                    sender.clone(), 
                    global_cancel.clone(),
                    settings
                );
                observer.observe(lb, player_actor, npc );
                starting_ramps.push(observer);
            }

            // npc on player oral
            if player_body.oral_collision.is_some() {
                let settings = ObserverSettings::new(
                    uses_strapon,
                    player_body.oral_collision.unwrap(),
                    &npc_body,
                    &player_body.oral_bone,
                    Selector::body_parts(vec![TAG_PENIS.into(), TAG_ORAL.into()])
                );

                let mut observer = BoneObserver::new(
                    "oral (player penetrated)".into(),
                    sender.clone(), 
                    global_cancel.clone(),
                    settings
                );
                observer.observe(lb, npc, player_actor);
                starting_ramps.push(observer);
            }

            // npc on player anal
            if player_body.anal_collision.is_some() {
                let settings = ObserverSettings::new(
                    uses_strapon,
                    player_body.anal_collision.unwrap(),
                    &npc_body,
                    &player_body.anal_bone,
                    Selector::body_parts(vec![TAG_PENIS.into(), TAG_ORAL.into()])
                );
                let mut observer = BoneObserver::new(
                    "anal (player penetrated)".into(),
                    sender.clone(), 
                    global_cancel.clone(),
                    settings
                );
                observer.observe(lb, npc, player_actor);
                starting_ramps.push(observer);
            }
        }
        if starting_ramps.is_empty() {
            error!("no viable monitoring combination for scene actors")
        }
    }

    let tracking_handle = lb.dynamic_task.clone();    
    if !starting_ramps.is_empty() {
        let dynamic_settings_clone = lb.bone_tracking_config.clone();
        let actuator_settings_clone = lb.client.device_settings.clone();
        lb.client.runtime.spawn(async move {
            let mut winner: Option<&BoneObserver> = None;
            while !global_cancel.is_cancelled() && winner.is_none() {
                sleep(Duration::from_millis(200)).await;
                for (i, observer) in starting_ramps.iter().enumerate() {
                    if observer.penetration_happened.is_cancelled() {
                        info!("success! t_id={} penetrated, closing other threads", observer.global_id);
                        for (j, loser_thread) in starting_ramps.iter().enumerate() {
                            if j != i {
                                loser_thread.local_cancel.cancel();
                            }
                        }
                        winner = Some(observer);
                        break;
                    }
                }
            }

            if !global_cancel.is_cancelled() {
                start_control_thread(
                    dynamic_settings_clone,
                    actuator_settings_clone,
                    receiver,
                    enabled_position_actuators,
                    tracking_handle,
                    winner.unwrap().clone()
                );
                info!("Collision successful");
            } else {
                info!("No collision detected in any of the threads");
                for observer in starting_ramps {
                    let stop_time = observer.stats.stat_run_ms.load(Ordering::Relaxed);
                    debug!(?observer, "'{}' ran for {}ms...", observer.name, stop_time); 
                    // TODO: Only log stats id and settigns, not cancellation tokens
                }
            }
        });
    }
}

fn start_control_thread(
    dynamic_settings: BoneTrackingSettings,
    actuator_settings: ActuatorSettings,
    receiver: UnboundedReceiver<TrackingSignal>,
    actuators: Vec<Arc<Actuator>>,
    tracking_handle: DynamicTrackingHandle,
    observer: BoneObserver
) {
    let mut actuator_setting = actuator_settings.clone();
    let selector_clone = observer.settings.selector.clone();

    tokio::spawn(async move {
        let (_, actuators) = Filter::from_actuators(actuator_settings, actuators)
            .load_config(&mut actuator_setting)
            .with_selector(&selector_clone)
            .result();
        
        let mut dynamic = DynamicTracking {
            settings: dynamic_settings.stroker_settings,
            signals: receiver,
            actuators,
            status: tracking_handle,
        };
        info!(?dynamic.settings, ?selector_clone, ?actuator_setting, "success! moving stroker");
        let _ = dynamic.track_mirror().await;

        info!("bone tracking finished");
    });
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
