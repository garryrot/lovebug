
use std::sync::{Arc, Mutex};
use events::LovebugEvent;
use lazy_static::lazy_static;
use tracing::{debug, error};

use crate::events::lb_qry_nxt_evt;
use futures::channel::oneshot;

mod events;
mod logging;
mod settings;

#[derive(Debug)]
pub struct Lovebug {
    events: crossbeam_channel::Receiver<LovebugEvent>
}

impl Lovebug {
    pub fn run_static<F, R>(func: F, default: R) -> R
    where
        F: FnOnce(&mut Lovebug) -> R,
        R: std::fmt::Debug
    {
        let a : Box<str>;
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

pub struct TaskContext(i32); // (oneshot::Sender<ffi::Ret>);

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
        fn lb_qry_nxt_evt() -> Vec<SKSEModEvent>;
        fn api_loaded() -> bool;
    }
    
    unsafe extern "C++" {
        include!("Tasks.h");

        fn shim_TaskInterface_AddTask(
            done: fn( ctx: SKSEModEvent, event_name: &str ),
            ctx: SKSEModEvent,
            event_name: &str,
        );
    }
}

mod task_interface {    
    use futures::channel::oneshot;
    use tracing::info;
    use crate::*;

    use self::ffi::SKSEModEvent;

    pub async fn add_task_async() {
        let (tx, rx) = oneshot::channel();
        let context = SKSEModEvent {
            event_name: "asdf".to_owned(),
            str_arg: "asdf".to_owned(),
            num_arg: 0.0
        };
        let event_name = "asdf";

        ffi::shim_TaskInterface_AddTask(
            |context, event_name| {
                let evt = SKSEModEvent::new("event_name", "str_arg", 1.0);
                info!("moep");
            },
            context,
            event_name
        );
        rx.await.unwrap()
    }
}

pub fn api_loaded() -> bool {
    true
}
