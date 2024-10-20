use std::{sync::{Arc, Mutex}, time::Duration};
use lazy_static::lazy_static;
use cxx::{CxxString, CxxVector};
use tracing::{debug, error, info};
use tokio_util::sync::CancellationToken;

use buttplug::{client::LinearCommand, core::message::ActuatorType};

use ::config::*;
use find::Triggers;
use events::start_outgoing_event_thread;
use bp_scheduler::{
    client::BpClient,
    config::{
        actions::{ActionRef, Strength}, client::ClientSettings, read::read_config
    },
    speed::Speed,
};

pub static SETTINGS_FILE: &str = "Settings.json";
pub static SETTINGS_PATH: &str = "Data\\F4SE\\Plugins\\Lovebug";
pub static PATTERNS_DIR: &str =  "Data\\F4SE\\Plugins\\Lovebug\\Patterns";
pub static ACTIONS_DIR: &str =   "Data\\F4SE\\Plugins\\Lovebug\\Actions";
pub static TRIGGERS_DIR: &str =  "Data\\F4SE\\Plugins\\Lovebug\\Triggers";

mod events;
mod logging;
mod settings;
mod bones;
mod bridge;

#[derive(Debug)]
pub struct Lovebug {
    client: BpClient,
    triggers: Triggers,
    dynamic_task: Option<CancellationToken>
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
    #[namespace = "RE"]
    unsafe extern "C++" {
        include!("PCH.h");
        type Actor;
    }
   
    extern "Rust" {
        fn lb_init() -> bool;
        fn lb_action(action: &str, speed: i32, time_sec: f32) -> i32;
        fn lb_scene(
            scene: &str,
            scene_tags: &CxxVector<CxxString>,
            speed: i32,
            time_sec: f32,
        ) -> i32;
        fn lb_stroke(ms: i32, pos: f32) -> bool;
        fn lb_update(id: i32, speed: i32) -> bool;
        fn lb_stop(id: i32) -> bool;
        unsafe fn lb_process_event(event_name: &str, str_arg: &str, num_arg: &f32) -> bool;
    }
}

fn get_settings() -> ClientSettings {
    let mut settings = ClientSettings::try_read_or_default(
        SETTINGS_PATH,
        SETTINGS_FILE,
    );
    settings.pattern_path = String::from(PATTERNS_DIR);
    settings.action_path = String::from(ACTIONS_DIR);
    settings
}

pub fn lb_init() -> bool {
    if let Ok(mut guard) = LB.state.try_lock() {
        info!("lb_init");
        let client = BpClient::connect(get_settings()).unwrap();
        let mut lb = Lovebug {
            client,
            triggers: Triggers::default(),
            dynamic_task: None
        };

        start_outgoing_event_thread(&lb.client);

        lb.client.read_actions();
        lb.triggers.load_triggers(read_config(TRIGGERS_DIR.into()));
        
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
            lb.client.dispatch_refs(
                vec![ActionRef {
                    action: action_name.into(),
                    strength: Strength::Constant(100),
                }],
                vec![],
                Speed::new(speed.into()),
                read_input_duration(time_secs),
            )
        },
        -1,
    );
    -1
}

pub fn lb_scene(scene_name: &str, scene_tags: &CxxVector<CxxString>, speed: i32, time_secs: f32) -> i32 {
    info!(scene_name, speed, time_secs, "lb_scene");
    Lovebug::run_static(
        |lb| {
            let tags = read_input_string(scene_tags);
            let scene = lb.triggers.find_scene(scene_name, &tags);   
            info!("matched scene {:?}", scene);
            if let Some(scene) = scene {
                return lb.client.dispatch_refs(
                    scene.actions,
                    vec![],
                    Speed::new(speed.into()),
                    read_input_duration(time_secs),
                )
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
                    device.linear(&LinearCommand::Linear(ms as u32, pos.into())).await.unwrap();
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


fn read_input_string(list: &CxxVector<CxxString>) -> Vec<String> {
    // automatically discards any empty strings to account for papyrus
    // inability to do dynamic array sizes
    list.iter()
        .filter(|d| !d.is_empty())
        .map(|d| d.to_string_lossy().into_owned())
        .collect()
}

fn read_input_duration(secs: f32) -> Duration {
    if secs > 0.0 {
        Duration::from_millis((secs * 1000.0) as u64)
    } else {
        Duration::MAX
    }
}

fn _read_input_actuators(actuator: &str) -> ActuatorType {
    let lower = actuator.to_ascii_lowercase();
    match lower.as_str() {
        "constrict" => ActuatorType::Constrict,
        "inflate" => ActuatorType::Inflate,
        "oscillate" => ActuatorType::Oscillate,
        "vibrate" => ActuatorType::Vibrate,
        _ => {
            error!("unknown actuator {:?}", lower);
            ActuatorType::Vibrate
        }
    }
}