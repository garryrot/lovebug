use tracing::info;
use crate::{
    events::ffi_event::*, 
    ffi
};

#[derive(Debug)]
pub enum LovebugEvent {
    Foo,
    Bar
}

#[cxx::bridge]
mod ffi_event {

    // Outgoing event
    // - SendModEvent (standard signature) on SKSE
    // - External Event on F4se
    #[derive(Debug)]
    pub struct SKSEModEvent {
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
        fn AddTask_SKSEModEvent(done: fn(ctx: SKSEModEvent), ctx: SKSEModEvent);
        unsafe fn SendEvent(form: *mut TESForm, event: SKSEModEvent);
    }
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

pub fn start_outgoing_event_thread( receiver: crossbeam_channel::Receiver<LovebugEvent> ) {
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
