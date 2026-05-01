use crate::events::ffi_event::*;

#[cxx::bridge]
pub mod ffi_event {
    #[derive(Debug)]
    pub struct ModEvent {
        pub event_name: String,
        pub str_arg: String,
        pub num_arg: f64,
    }

    unsafe extern "C++" {
        include!("Events.h");
        fn AddTask_ModEvent(done: fn(ctx: ModEvent), ctx: ModEvent);
        unsafe fn SendEvent(event: ModEvent);
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
}

pub fn send_mod_event(event: ModEvent) {
    AddTask_ModEvent(
        |context| {
            unsafe {
                SendEvent(context);
            }
        },
        event,
    );
}
