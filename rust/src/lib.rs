
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use events::{start_event_thread, LovebugEvent};
use lazy_static::lazy_static;
use tracing::{debug, error};

mod events;
mod logging;
mod settings;

#[derive(Debug)]
pub struct Lovebug {
    event_sender: crossbeam_channel::Sender<LovebugEvent>
}

impl Lovebug {
    pub fn run_static<F, R>(func: F, default: R) -> R
    where
        F: FnOnce(&mut Lovebug) -> R,
        R: std::fmt::Debug
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
            state: Arc::new(Mutex::new(None))
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

    pub struct Ret { i: i32 }

    extern "Rust" {
        type TaskContext;
        fn lb_init() -> bool;
    }

    #[namespace = "RE"]
    unsafe extern "C++" {
        include!("PCH.h");
        type TESForm;
    }
    
    unsafe extern "C++" {
        include!("Bridge.h");
        fn AddTask_SKSEModEvent(done: fn( ctx: SKSEModEvent ), ctx: SKSEModEvent);
        fn GetFormById(id: i32, esp: &str) -> *mut TESForm;
        unsafe fn SendEvent(form: *mut TESForm, event: SKSEModEvent);
    }
}

pub fn lb_init() -> bool {
    if let Ok(mut guard) = LB.state.try_lock() {
        let (event_sender, recv) = crossbeam_channel::unbounded();
           start_event_thread(recv);
        let lb = Lovebug {
            event_sender
        };
        guard.replace(lb);
    } else {
        error!("init failed");
    }

    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(30));
            debug!("loop");
            if let Ok(mut guard) = LB.state.try_lock() {
                if let Some(inner) = &mut *guard {
                    inner.event_sender.send(LovebugEvent::Foo).unwrap();
                }
            } else {
                error!("init failed");
            }
        }
    });
    true
}
