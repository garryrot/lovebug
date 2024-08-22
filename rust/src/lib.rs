use ::config::*;
 use bp_scheduler::{
     client::{
         client::*,
         connection::*,
         input::*,
         settings::*,
     },
     settings::devices::BpSettings,
     speed::Speed,
 };
use cxx::{CxxString, CxxVector};
use events::{start_outgoing_event_thread, LovebugEvent};
// use ffi::{ModCallbackEvent, TESForm};
use lazy_static::lazy_static;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tracing::{debug, error, info, warn};

// use crate::ffi::CloneInto;

pub static SETTINGS_FILE: &str = "Settings.json";
pub static SETTINGS_PATH: &str = "Data\\SKSE\\Plugins\\Lovebug";
pub static PATTERNS_DIR: &str = "Data\\SKSE\\Plugins\\Lovebug\\Patterns";
pub static ACTIONS_DIR: &str = "Data\\SKSE\\Plugins\\Lovebug\\Actions";
pub static TRIGGERS_DIR: &str = "Data\\SKSE\\Plugins\\Lovebug\\Triggers";

mod config;
mod events;
mod logging;
mod settings;

#[derive(Debug)]
pub struct Lovebug {
    client: BpClient,
    event_sender: crossbeam_channel::Sender<LovebugEvent>,
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
        // fn lb_action(
        //     action: &str,
        //     speed: i32,
        //     time_sec: f32,
        //     body_parts: &CxxVector<CxxString>,
        // ) -> i32;
        unsafe fn lb_process_event_bridge(event_name: &str, str_arg: &str, num_arg: &f32) -> bool;

        // Unsupported F4
        // unsafe fn lb_process_event(form: *const ModCallbackEvent, sender: *const TESForm);
    }

    unsafe extern "C++" {
        include!("Bridge.h");
        fn GetFormById(id: i32, esp: &str) -> *mut TESForm;
    }
}

fn get_settings() -> TkSettings {
    TkSettings::try_read_or(
        SETTINGS_PATH,
        SETTINGS_FILE,
        TkSettings {
            version: 2,
            log_level: TkLogLevel::Debug,
            connection: TkConnectionType::InProcess,
            device_settings: BpSettings { devices: vec![] },
            pattern_path: String::from(PATTERNS_DIR),
            action_path: String::from(ACTIONS_DIR),
        },
    )
}

pub fn lb_init() -> bool {
    if let Ok(mut guard) = LB.state.try_lock() {
        let (event_sender, recv) = crossbeam_channel::unbounded();
        if let Ok(triggers) = read_triggers(TRIGGERS_DIR) {}

        start_outgoing_event_thread(recv);

        let lb = Lovebug {
            client: BpClient::connect(get_settings()).unwrap(),
            event_sender,
        };

        guard.replace(lb);
    } else {
        error!("init failed");
    }
    true
}

pub fn lb_action(
    action_name: &str,
    speed: i32,
    time_secs: f32,
    body_parts: &CxxVector<CxxString>,
) -> i32 {
    let body_parts_2 = read_input_string(body_parts);

    Lovebug::run_static(
        |lb| {
            lb.client.dispatch_name(
                action_name,
                body_parts_2,
                Speed::new(speed.into()),
                get_duration_from_secs(time_secs),
            )
        },
        -1,
    );

    -1
}

unsafe fn lb_process_event_bridge(event_name: &str, str_arg: &str, num_arg: &f32) -> bool {
    info!("lb_process_event_bridge");
    let form_id = 0;
    debug!(
        "EventBridge {:#010x} {} {} {}",
        form_id, event_name, str_arg, num_arg
    );

    // Lovebug::run_static( |lb| {
    //     if let Err(error) = lb.event_sender.try_send(LovebugEvent::LovebugEvent) {
    //         error!("{:?}", error.to_string());
    //     }
    //     true
    // }, true );

    false
}

// unsafe fn lb_process_event(event: *const ModCallbackEvent, sender: *const TESForm) {
//     if let Some(sender) = sender.as_ref() {
//         let form_id = sender.GetFormID();

//         let mod_event = CloneInto(event);
//         warn!(
//             "Event {:#010x} {} str={} num={}",
//             form_id, mod_event.event_name, mod_event.str_arg, mod_event.num_arg
//         );
//         // warn!("{:#010x} (editor id)", local_id);
//     }
// }
