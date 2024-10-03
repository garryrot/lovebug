use ::config::*;
use bp_scheduler::{
    client::{input::*, settings::*, status::get_known_actuator_ids, BpClient},
    config::{
        actions::{ActionRef, Strength},
        read::read_config,
    },
    speed::Speed,
};
use cxx::{CxxString, CxxVector};
use events::start_outgoing_event_thread;
use lazy_static::lazy_static;
use triggers::Triggers;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info};

pub static SETTINGS_FILE: &str = "Settings.json";
pub static SETTINGS_PATH: &str = "Data\\F4SE\\Plugins\\Lovebug";
pub static PATTERNS_DIR: &str = "Data\\F4SE\\Plugins\\Lovebug\\Patterns";
pub static ACTIONS_DIR: &str = "Data\\F4SE\\Plugins\\Lovebug\\Actions";
pub static TRIGGERS_DIR: &str = "Data\\F4SE\\Plugins\\Lovebug\\Triggers";

mod events;
mod logging;
mod settings;

#[derive(Debug)]
pub struct Lovebug {
    client: BpClient,
    triggers: Triggers
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

    fn enable_all(&mut self) {
        for actuator in
            get_known_actuator_ids(self.client.buttplug.devices(), &self.client.settings)
        {
            self.client
                .settings
                .device_settings
                .set_enabled(&actuator, true);
        }
    }
}

#[derive(Debug)]
pub struct LbApi {
    pub state: Arc<Mutex<Option<Lovebug>>>,
}

pub struct TaskContext(i32);

lazy_static! {
    static ref LB: LbApi = {
        LbApi {
            state: Arc::new(Mutex::new(None)),
        }
    };
}

#[cxx::bridge]
mod ffi {
    pub struct Ret {
        i: i32,
    }

    #[namespace = "RE"]
    unsafe extern "C++" {
        include!("PCH.h");
        type TESForm;
        // fn GetFormID(self: &TESForm) -> u32;
        // fn GetRawFormID(self: &TESForm) -> u32;
        // fn GetLocalFormID(self: &TESForm) -> u32;
    }

    #[namespace = "SKSE"]
    unsafe extern "C++" {
        include!("PCH.h");
        // Unsupported F4
        // type ModCallbackEvent;
    }

    extern "Rust" {
        type TaskContext;
        fn lb_init() -> bool;
        fn lb_action(action: &str, speed: i32, time_sec: f32) -> i32;
        fn lb_scene(
            scene: &str,
            scene_tags: &CxxVector<CxxString>,
            speed: i32,
            time_sec: f32,
        ) -> i32;
        fn lb_update(id: i32, speed: i32) -> bool;
        fn lb_stop(id: i32) -> bool;
        unsafe fn lb_process_event(event_name: &str, str_arg: &str, num_arg: &f32) -> bool;
    }

    unsafe extern "C++" {
        include!("Bridge.h");
        fn GetFormById(id: i32, esp: &str) -> *mut TESForm;
    }
}

fn get_settings() -> TkSettings {
    let mut settings = TkSettings::try_read_or_default(
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
            triggers: Triggers::default()
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
                get_duration_from_secs(time_secs),
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
                    get_duration_from_secs(time_secs),
                )
            }
            -1
        },
        -1,
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
