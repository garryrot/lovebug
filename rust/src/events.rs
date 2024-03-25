use tracing::{debug, info};
use crate::{ffi, LB};

use crate::ffi::*;

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

pub fn start_event_thread( receiver: crossbeam_channel::Receiver<LovebugEvent> ) {
    std::thread::spawn( move || {
        fn send_mod_event(event: SKSEModEvent) {
            AddTask_SKSEModEvent(
                |context| {
                    let form = ffi::GetFormById(0x12C5, "Lovebug.esp");
                    unsafe { SendEvent(form, context); }
                },
                event
            );
        }
    
        while let Ok(evt) = receiver.recv() {
            info!("got event: {:?}", evt);
            match evt {
                LovebugEvent::Foo => send_mod_event( SKSEModEvent::new("event_foo", "str_arg", 0.0) ),
                LovebugEvent::Bar => send_mod_event( SKSEModEvent::new("event_bar", "str_arg", 0.0) )
            }
        }
    });
}
