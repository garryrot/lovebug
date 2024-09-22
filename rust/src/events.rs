use bp_scheduler::client::BpClient;
use tracing::info;
use buttplug::client::ButtplugClientEvent;
use crate::{
    events::ffi_event::*, 
    ffi
};
use futures_util::StreamExt;

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
pub fn start_outgoing_event_thread( client: &BpClient ) {
    let mut events = client.buttplug.event_stream();

    client.runtime.spawn( async move {
        fn send_mod_event(event: ModEvent) {
            AddTask_ModEvent(
                |context| {
                    let form = ffi::GetFormById(0x0F99, "Lovebug.esp"); // TODO: Is getting form even required?
                    unsafe { SendEvent(form, context); }
                },
                event
            );
        }
    
        while let Some(evt) = events.next().await {
            info!("got event: {:?}", evt);
            let str_arg = match evt {
                ButtplugClientEvent::DeviceAdded(device) => format!("Connected({})", device.name()),
                ButtplugClientEvent::DeviceRemoved(device) => format!("Disconnected({})", device.name()),
                ButtplugClientEvent::Error(buttplug_error) => format!("Error({:?})", buttplug_error),
                _ => String::new()
            };
            if str_arg != String::new() {
                send_mod_event(ModEvent::new("LbEvent", str_arg.as_str(), 0.0))   
            }
        }
    });
}
