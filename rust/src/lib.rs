use ::config::{read_triggers, TRIGGERS_DIR};
use events::{start_event_thread, LovebugEvent};
use ffi::{ModCallbackEvent, TESForm};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, warn};

use crate::ffi::CloneInto;

mod config;
mod events;
mod logging;
mod settings;

#[derive(Debug)]
pub struct Lovebug {
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
    #[derive(Debug)]
    pub struct SKSEModEvent {
        pub event_name: String,
        pub str_arg: String,
        pub num_arg: f64,
    }

    pub struct Ret {
        i: i32,
    }

    #[namespace = "RE"]
    unsafe extern "C++" {
        include!("PCH.h");
        type TESForm;
        fn GetFormID(self: &TESForm) -> u32;
        fn GetRawFormID(self: &TESForm) -> u32;
        fn GetLocalFormID(self: &TESForm) -> u32;
    }

    #[namespace = "SKSE"]
    unsafe extern "C++" {
        include!("PCH.h");
        type ModCallbackEvent;
    }

    extern "Rust" {
        type TaskContext;
        fn lb_init() -> bool;
        unsafe fn lb_process_event_bridge(
            event_name: &str,
            str_arg: &str,
            num_arg: &f32,
            // sender: *const TESForm,
        ) -> bool;
        unsafe fn lb_process_event(form: *const ModCallbackEvent, sender: *const TESForm);
    }

    unsafe extern "C++" {
        include!("Bridge.h");
        fn AddTask_SKSEModEvent(done: fn(ctx: SKSEModEvent), ctx: SKSEModEvent);
        fn GetFormById(id: i32, esp: &str) -> *mut TESForm;
        unsafe fn SendEvent(form: *mut TESForm, event: SKSEModEvent);
        unsafe fn CloneInto(event: *const ModCallbackEvent) -> SKSEModEvent;
    }
}

pub fn lb_init() -> bool {
    if let Ok(mut guard) = LB.state.try_lock() {
        let (event_sender, recv) = crossbeam_channel::unbounded();
        if let Ok(triggers) = read_triggers(TRIGGERS_DIR) {}
        start_event_thread(recv);

        let lb = Lovebug { event_sender };
        guard.replace(lb);
    } else {
        error!("init failed");
    }
    true
}

unsafe fn lb_process_event_bridge(
    event_name: &str,
    str_arg: &str,
    num_arg: &f32,
) -> bool {
    let form_id = 0;
    debug!(
        "EventBridge {:#010x} {} {} {}",
        form_id, event_name, str_arg, num_arg
    );
    false
}

unsafe fn lb_process_event(event: *const ModCallbackEvent, sender: *const TESForm) {
    if let Some(sender) = sender.as_ref() {
        let form_id = sender.GetFormID();

        let mod_event = CloneInto(event);
        warn!(
            "Event {:#010x} {} str={} num={}",
            form_id, mod_event.event_name, mod_event.str_arg, mod_event.num_arg
        );
        // warn!("{:#010x} (editor id)", local_id);
    }
}
