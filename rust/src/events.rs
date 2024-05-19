use tracing::info;
use crate::{
    events::ffi_event::*, 
    ffi
};

#[derive(Debug)]
pub enum LovebugEvent {
    LovebugEvent,
    Bar
}

#[cxx::bridge]
mod ffi_event {

    #[derive(Debug)]
    pub struct ModEvent {
        pub event_name: String, 
        pub str_arg: String,
        pub num_arg: f64,
    }

    #[namespace = "RE"]
    unsafe extern "C++" {
        include!("Events.h");
        type TESForm = crate::ffi::TESForm;
    }

    unsafe extern "C++" {
        fn AddTask_ModEvent(done: fn(ctx: ModEvent), ctx: ModEvent);
        unsafe fn SendEvent(form: *mut TESForm, event: ModEvent);
    }
}

impl ModEvent {
    pub fn new(event_name: &str, str_arg: &str, num_arg: f64) -> ModEvent {
        ModEvent {
            event_name: String::from(event_name),
            str_arg: String::from(str_arg),
            num_arg,
        }
    }

    pub fn from(event_name: &str, str_arg: &str) -> ModEvent {
        ModEvent {
            event_name: String::from(event_name),
            str_arg: String::from(str_arg),
            num_arg: 0.0,
        }
    }
}

/// Sends outgoing events which can be consumed in-game by
///  - RegisterForModEvent (standard signature) on SKSE
///  - RegisterForExternalEvent on F4SE
/// Events can be sent by adding to the queue in the main struct
pub fn start_outgoing_event_thread( receiver: crossbeam_channel::Receiver<LovebugEvent> ) {
    std::thread::spawn( move || {
        fn send_mod_event(event: ModEvent) {
            AddTask_ModEvent(
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
                LovebugEvent::LovebugEvent => send_mod_event( ModEvent::new("LovebugEvent", "str_arg", 42.0) ),
                LovebugEvent::Bar => send_mod_event( ModEvent::new("Bar", "str_arg", 42.0) )
            }
        }
    });
}
