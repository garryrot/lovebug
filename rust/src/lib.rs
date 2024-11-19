use bodies::Race;
use bones::{lb_dynamic_tracking, DynamicTrackingHandle};
use cxx::{CxxString, CxxVector};
use dd::start_dd_workaround;
use input::{read_input_duration, read_input_string};
use lazy_static::lazy_static;
use tokio::task::JoinHandle;
use variables::VariableStore;

use std::{
    sync::{ atomic::AtomicI64, Arc, Mutex},
    time::Duration,
};
use tracing::{debug, error, info};

use crate::bones::ffi_bones::ActorVec;

use bp_scheduler::{
    client::BpClient,
    config::{
        actions::*,
        actuators::ActuatorSettings,
        client::{ClientSettings, InProcessFeatures},
        connection::ConnectionType,
        read::*,
        write::try_write,
    },
    dynamic_tracking::DynamicSettings,
    speed::Speed,
};

use ::config::*;
use events::start_outgoing_event_thread;
use triggers::Triggers;

pub static CONFIG_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2";
pub static PATTERNS_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Patterns";
pub static ACTIONS_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Actions";
pub static TRIGGERS_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Triggers";
pub static RACES_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Races";
pub static VARIABLES_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Variables";

pub static DEFAULT_RACE_MALE: &str = "DefaultRaceMale.json";
pub static DEFAULT_RACE_FEMALE: &str = "DefaultRaceFemale.json";
pub static BONE_TRACKING: &str = "BoneTracking.json";
pub static LOGGING_SETTINGS: &str = "Logging.json";
pub static DEVICE_SETTINGS: &str = "Devices.json";

mod bones;
pub mod bridge;
mod dd;
mod events;
mod input;
mod logging;
mod mcm;
mod variables;

#[derive(Debug)]
pub struct Telekinesis {
    client: BpClient,
    triggers: Triggers,
    dynamic_task: DynamicTrackingHandle,
    dynamic_settings: DynamicSettings,
    tracking_counter: i32,
    races: Vec<Race>,
    default_race_male: Option<Race>,
    default_race_female: Option<Race>,
    variables: VariableStore,
    variable_update_thread: Option<JoinHandle<()>>,
}

impl Telekinesis {
    pub fn run_static<F, R>(func: F, default: R) -> R
    where
        F: FnOnce(&mut Telekinesis) -> R,
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

    pub fn run_static_no_return<F>(func: F)
    where
        F: FnOnce(&mut Telekinesis)
    {
        if let Ok(mut guard) = LB.state.try_lock() {
            match guard.take() {
                Some(mut tk) => {
                    func(&mut tk);
                    guard.replace(tk);
                }
                None => error!("State empty"),
            }
        } else {
            error!("failed locking mutex");
        }
    }

    pub fn run_static_destroy<F>(func: F)
    where
        F: FnOnce(&mut Telekinesis),
    {
        if let Ok(mut guard) = LB.state.try_lock() {
            match guard.take() {
                Some(mut tk) => {
                    func(&mut tk);
                }
                None => error!("State empty"),
            }
        }
    }

    pub fn store_devices(&mut self) {
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
    pub state: Arc<Mutex<Option<Telekinesis>>>,
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
        fn lb_connect(
            connection: i32,
            port: &str,
            host: &str,
            bluetooth: bool,
            xinput: bool,
            serial: bool,
        ) -> bool;
        fn lb_disconnect();
        fn lb_action(action: &str, speed: i32, time_sec: f32) -> i32;
        fn lb_scene(
            scene: &str,
            scene_tags: &CxxVector<CxxString>,
            speed: i32,
            time_sec: f32,
            actors: &ActorVec,
        ) -> i32;
        fn lb_update(id: i32, speed: i32) -> bool;
        fn lb_stop(id: i32) -> bool;
        fn lb_actor_value_changed(form_id: u32, value: f32);
        fn lb_process_event(event_name: &str, str_arg: &str, num_arg: f32) -> i32;
    }
}

pub fn lb_actor_value_changed(form_id: u32, value: f32) {
    Telekinesis::run_static_no_return(|lb| {
        let _ = lb.variables.update(form_id, value);
    });
}

pub fn lb_connect(
    connection: i32,
    port: &str,
    host: &str,
    bluetooth: bool,
    xinput: bool,
    serial: bool,
) -> bool {
    // TODO: Do this in to background thread to avoid small UI stutter
    if let Ok(mut guard) = LB.state.try_lock() {
        let settings = ClientSettings {
            connection: match connection {
                0 => ConnectionType::InProcess,
                1 => ConnectionType::WebSocket(format!("{}:{}", host, port)),
                _ => ConnectionType::Test,
            },
            in_process_features: InProcessFeatures {
                bluetooth,
                serial,
                xinput,
            },
            pattern_path: String::from(PATTERNS_DIR),
        };
        info!(?settings, "lb_connect");
        let client = BpClient::connect(
            settings,
            read_or_default::<ActuatorSettings>(CONFIG_DIR, DEVICE_SETTINGS),
        );
        if let Err(a) = client {
            error!(?a, "connection error");
            return false;
        }

        let dynamic_task = DynamicTrackingHandle::default();

        let vars = read_variables();

        let variables = VariableStore::new( vars, &dynamic_task);
        let mut lb = Telekinesis {
            client: client.unwrap(),
            triggers: Triggers::default(),
            dynamic_task,
            tracking_counter: 0,
            races: vec![],
            default_race_male: None,
            default_race_female: None,
            dynamic_settings: DynamicSettings::default(),
            variables,
            variable_update_thread: None,
        };
        lb.client.read_actions(ACTIONS_DIR);
        lb.read_races();

        start_outgoing_event_thread(&lb.client);
        start_dd_workaround(&mut lb);

        lb.triggers
            .load_triggers(read_config_dir(TRIGGERS_DIR.into()));
        lb.client.scan_for_devices();

        guard.replace(lb);
    } else {
        error!("init failed");
    }
    true
}

fn read_variables() -> Vec<config::variables::ConfigVariable> {
    let vars = read_config_dir(VARIABLES_DIR.into());
    for var in &vars {
        debug!(?var, "read variable");
    }
    info!("read {} variables...", vars.len());
    vars
}

pub fn lb_disconnect() {
    Telekinesis::run_static_destroy(|lb| {
        lb.client.stop_all();
        lb.client.disconnect();
    });
}

pub fn lb_action(action_name: &str, speed: i32, time_secs: f32) -> i32 {
    info!(action_name, speed, time_secs, "lb_action");
    Telekinesis::run_static(
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
    Telekinesis::run_static(
        |lb| {
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

pub fn lb_update(handle: i32, speed: i32) -> bool {
    info!(handle, speed, "lb_update");
    Telekinesis::run_static(
        |lb| lb.client.update(handle, Speed::new(speed.into())),
        false,
    )
}

pub fn lb_stop(handle: i32) -> bool {
    info!(handle, "lb_stop");
    Telekinesis::run_static(|lb| lb.client.stop(handle), false)
}

fn lb_process_event(event_name: &str, str_arg: &str, num_arg: f32) -> i32 {
    info!(event_name, str_arg, num_arg, "lb_process_event");
    Telekinesis::run_static(
        |lb| {
            let _stopped_event = lb.triggers.find_stopped_events(event_name);

            if let Some(start_event) = lb.triggers.find_started_events(event_name) {
                let converted = get_actions_from_refs(lb, start_event.actions);
                return lb.client.dispatch_refs(
                    converted,
                    vec![],
                    Speed::max(),
                    Duration::from_secs(999999),
                );
            }
            -1
        },
        -1,
    )
}

fn get_actions_from_refs(
    lb: &mut Telekinesis,
    action_refs: Vec<ActionRef>,
) -> Vec<(Strength, Action)> {
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
                    Variable::PlayerActorValue(name) => {
                        if let Some(var) =  lb.variables.get(&name) {
                            var.clone()
                        } else {
                            error!(name, "unknown player actor value");
                            Arc::new(AtomicI64::new(0))
                        }
                    },
                }),
                Stren::Funscript(x, y) => Strength::Funscript(x, y),
                Stren::RandomFunscript(x, y) => Strength::RandomFunscript(x, y),
            };
            result.push((strn, action.clone()));
        }
    }
    result
}
