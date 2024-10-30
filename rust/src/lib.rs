use bodies::Race;
use bones::{lb_dynamic_tracking, DynamicTrackingHandle};
use cxx::{CxxString, CxxVector};
use input::{read_input_duration, read_input_string};
use lazy_static::lazy_static;

use std::sync::{Arc, Mutex};
use tracing::{debug, error, info};

use crate::bones::ffi_bones::ActorVec;

use buttplug::client::LinearCommand;

use bp_scheduler::{
    actuator::Actuators,
    client::BpClient,
    config::{
        actions::*, actuators::ActuatorSettings, client::ClientSettings, read::*, write::try_write,
    },
    dynamic_tracking::DynamicSettings,
    speed::Speed,
};

use ::config::*;
use body_parts::*;
use events::start_outgoing_event_thread;
use triggers::Triggers;

pub static CONFIG_DIR: &str = "Data\\F4SE\\Plugins\\Lovebug";
pub static PATTERNS_DIR: &str = "Data\\F4SE\\Plugins\\Lovebug\\Patterns";
pub static ACTIONS_DIR: &str = "Data\\F4SE\\Plugins\\Lovebug\\Actions";
pub static TRIGGERS_DIR: &str = "Data\\F4SE\\Plugins\\Lovebug\\Triggers";
pub static RACES_DIR: &str = "Data\\F4SE\\Plugins\\Lovebug\\Races";

pub static DEFAULT_RACE_MALE: &str = "DefaultRaceMale.json";
pub static DEFAULT_RACE_FEMALE: &str = "DefaultRaceFemale.json";
pub static BONE_TRACKING: &str = "BoneTracking.json";
pub static CLIENT_SETTINGS: &str = "Connection.json";
pub static DEVICE_SETTINGS: &str = "Devices.json";

mod bones;
pub mod bridge;
mod events;
mod input;
mod logging;

#[derive(Debug)]
pub struct Lovebug {
    client: BpClient,
    triggers: Triggers,
    dynamic_task: DynamicTrackingHandle,
    dynamic_settings: DynamicSettings,
    tracking_counter: i32,
    races: Vec<Race>,
    default_race_male: Option<Race>,
    default_race_female: Option<Race>,
}

impl Lovebug {
    pub fn run_static<F, R>(func: F, default: R) -> R
    where
        F: FnOnce(&mut Lovebug) -> R,
        R: std::fmt::Debug,
    {
        if let Ok(mut guard) = LB.state.try_lock() {
            match guard.take() {
                Some(mut tk) => {
                    let result = func(&mut tk);
                    guard.replace(tk);
                    debug!("result: {:?}", result);
                    return result;
                }
                None => error!("State empty"),
            }
        } else {
            error!("failed locking mutex");
        }
        default
    }

    pub fn refresh_devices(&mut self) {
        let devices = self.client.buttplug.devices();
        for actuator in devices.flatten_actuators() {
            self.client
                .device_settings
                .set_enabled(actuator.identifier(), true);
            self.client.device_settings.set_body_parts(
                actuator.identifier(),
                &[
                    TAG_ANAL,
                    TAG_CLIT,
                    TAG_NIPPLE,
                    TAG_ORAL,
                    TAG_PENIS,
                    TAG_VAGINAL,
                ],
            );
        }
        try_write(&self.client.device_settings, CONFIG_DIR, DEVICE_SETTINGS);
    }

    pub fn read_races(&mut self) {
        self.races = read_config_dir(RACES_DIR.into());
        self.default_race_male = Some(read_or_default(CONFIG_DIR, DEFAULT_RACE_MALE));
        self.default_race_female = Some(read_or_default(CONFIG_DIR, DEFAULT_RACE_FEMALE));
        self.dynamic_settings = read_or_default(CONFIG_DIR, BONE_TRACKING);
    }
}

#[derive(Debug)]
pub struct LbApi {
    pub state: Arc<Mutex<Option<Lovebug>>>,
}

lazy_static! {
    static ref LB: LbApi = {
        LbApi {
            state: Arc::new(Mutex::new(None)),
        }
    };
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type ActorVec = crate::bones::ffi_bones::ActorVec;
    }
    extern "Rust" {
        fn lb_init() -> bool;
        fn lb_action(action: &str, speed: i32, time_sec: f32) -> i32;
        fn lb_scene(
            scene: &str,
            scene_tags: &CxxVector<CxxString>,
            speed: i32,
            time_sec: f32,
            actors: &ActorVec,
        ) -> i32;
        fn lb_stroke(ms: i32, pos: f32) -> bool;
        fn lb_update(id: i32, speed: i32) -> bool;
        fn lb_stop(id: i32) -> bool;
        unsafe fn lb_process_event(event_name: &str, str_arg: &str, num_arg: &f32) -> bool;
    }
}

pub fn lb_init() -> bool {
    if let Ok(mut guard) = LB.state.try_lock() {
        info!("lb_init");
        let client = BpClient::connect(
            ClientSettings {
                pattern_path: String::from(PATTERNS_DIR),
                ..read_or_default::<ClientSettings>(CONFIG_DIR, CLIENT_SETTINGS)
            },
            read_or_default::<ActuatorSettings>(CONFIG_DIR, DEVICE_SETTINGS),
        )
        .unwrap();
        let mut lb = Lovebug {
            client,
            triggers: Triggers::default(),
            dynamic_task: DynamicTrackingHandle::default(),
            tracking_counter: 0,
            races: vec![],
            default_race_male: None,
            default_race_female: None,
            dynamic_settings: DynamicSettings::default(),
        };
        lb.client.read_actions(ACTIONS_DIR);
        lb.read_races();

        start_outgoing_event_thread(&lb.client);

        lb.triggers
            .load_triggers(read_config_dir(TRIGGERS_DIR.into()));
        lb.client.scan_for_devices();

        guard.replace(lb);
    } else {
        error!("init failed");
    }
    true
}

pub fn lb_action(action_name: &str, speed: i32, time_secs: f32) -> i32 {
    info!(action_name, speed, time_secs, "lb_action");
    Lovebug::run_static(
        |lb| {
            let actions = get_actions_from_refs(
                lb,
                vec![ActionRef {
                    action: action_name.into(),
                    strength: Stren::Constant(100),
                }],
            );
            lb.client.dispatch_refs(
                actions,
                vec![],
                Speed::new(speed.into()),
                read_input_duration(time_secs),
            )
        },
        -1,
    );
    -1
}

pub fn lb_scene(
    scene_name: &str,
    scene_tags: &CxxVector<CxxString>,
    speed: i32,
    time_secs: f32,
    actor_vec: &ActorVec,
) -> i32 {
    info!(scene_name, speed, time_secs, "lb_scene");
    Lovebug::run_static(
        |lb| {
            lb.refresh_devices();

            let tags = read_input_string(scene_tags);
            let scene = lb.triggers.find_scene(scene_name, &tags);
            debug!(?scene, "matched scene");

            if let Some(scene) = scene {
                let mut actions = get_actions_from_refs(lb, scene.actions);
                let mut do_stroke = None;
                for action in actions.iter_mut() {
                    if action.1.do_bone_tracking {
                        let has_stroker = action
                            .1
                            .control
                            .iter()
                            .find(|x| matches!(x, Control::Stroke(_, _)));
                        if has_stroker.is_some() {
                            do_stroke = has_stroker.cloned();
                        }
                        action
                            .1
                            .control
                            .retain(|x| matches!(x, Control::Scalar(_, _)));
                    }
                }
                if let Some(stroke) = do_stroke {
                    lb_dynamic_tracking(lb, actor_vec, stroke);
                }
                return lb.client.dispatch_refs(
                    actions,
                    vec![],
                    Speed::new(speed.into()),
                    read_input_duration(time_secs),
                );
            }
            -1
        },
        -1,
    )
}

pub fn lb_stroke(ms: i32, pos: f32) -> bool {
    info!(ms, pos, "lb_stroke");
    Lovebug::run_static(
        |lb| {
            let devices = lb.client.buttplug.devices();
            lb.client.runtime.spawn(async move {
                for device in devices {
                    device
                        .linear(&LinearCommand::Linear(ms as u32, pos.into()))
                        .await
                        .unwrap();
                }
            });
            true
        },
        false,
    )
}

pub fn lb_update(handle: i32, speed: i32) -> bool {
    info!(handle, speed, "lb_update");
    Lovebug::run_static(
        |lb| lb.client.update(handle, Speed::new(speed.into())),
        false,
    )
}

pub fn lb_stop(handle: i32) -> bool {
    info!(handle, "lb_stop");
    Lovebug::run_static(|lb| lb.client.stop(handle), false)
}

unsafe fn lb_process_event(event_name: &str, str_arg: &str, num_arg: &f32) -> bool {
    info!("lb_event");
    let form_id = 0;
    debug!(
        "EventBridge {:#010x} {} {} {}",
        form_id, event_name, str_arg, num_arg
    );
    false
}

fn get_actions_from_refs(lb: &mut Lovebug, action_refs: Vec<ActionRef>) -> Vec<(Strength, Action)> {
    let mut result = vec![];
    for action_ref in action_refs {
        if let Some(action) = lb
            .client
            .actions
            .0
            .iter()
            .find(|x| x.name == action_ref.action)
        {
            let strn = match action_ref.strength {
                Stren::Constant(x) => Strength::Constant(x),
                Stren::Variable(var) => Strength::Variable(match var {
                    Variable::BoneTrackingRate => lb.dynamic_task.cur_avg_ms.clone(),
                    Variable::BoneTrackingDepth => lb.dynamic_task.cur_depth.clone(),
                }),
                Stren::Funscript(x, y) => Strength::Funscript(x, y),
                Stren::RandomFunscript(x, y) => Strength::RandomFunscript(x, y),
            };
            result.push((strn, action.clone()));
        }
    }
    result
}
