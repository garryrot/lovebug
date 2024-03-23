use tracing::info;
use crate::LB;

use crate::ffi::SKSEModEvent;

#[derive(Debug)]
pub enum LovebugEvent {
    Foo,
    Bar
}

impl SKSEModEvent {
    pub fn new(event_name: &str, str_arg: &str, num_arg: f64) -> SKSEModEvent {
        SKSEModEvent {
            event_name: String::from(event_name),
            str_arg: String::from(str_arg),
            num_arg,
        }
    }

    pub fn from(event_name: &str, str_arg: &str) -> SKSEModEvent {
        SKSEModEvent {
            event_name: String::from(event_name),
            str_arg: String::from(str_arg),
            num_arg: 0.0,
        }
    }
}


/// Return type Vec cause cxx crate does not support Option
/// and Result enforces try catch with some weird template
/// I don't wanna get into
pub fn lb_qry_nxt_evt() -> Vec<SKSEModEvent> {
    let mut receiver = None;
    if let Ok(mut guard) = LB.state.lock() {
        if let Some(tk) = guard.take() {
            let evt_receiver = tk.events.clone();
            guard.replace(tk);
            receiver = Some(evt_receiver);
        }
    }
    match receiver {
        Some(receiver) => {
            if let Some(evt) = get_next_events_blocking(&receiver) {
                return vec![evt];
            }
            vec![]
        }
        None => vec![],
    }
}

pub fn get_next_events_blocking(connection_events: &crossbeam_channel::Receiver<LovebugEvent>) -> Option<SKSEModEvent> {
    if let Ok(result) = connection_events.recv() {
        info!("Sending SKSE Event: {:?}", result);
        let event = match result {
            LovebugEvent::Foo => SKSEModEvent { event_name: "Lovebug_Foo".to_string(), str_arg: "".into(), num_arg: 0.0 },
            LovebugEvent::Bar => SKSEModEvent { event_name: "Lovebug_Bar".to_string(), str_arg: "".into(), num_arg: 0.0 },
        };
        return Some(event);
    }
    None
}