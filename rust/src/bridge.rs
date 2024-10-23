#[cxx::bridge]
pub mod ffi_bridge {

    #[derive(Debug)]
    pub enum Sex {
        None = -1,
        Male = 0,
        Female = 1,
        Total = 2
    }

    #[namespace = "RE"]
    unsafe extern "C++" {
        type TESForm;
        type TESRace;
        type Actor = crate::ffi::Actor;
    }
    
    unsafe extern "C++" {
        include!("Bridge.h");
        pub unsafe fn IsPlayer(actor: *const Actor) -> bool;
        pub unsafe fn GetSex(actor: *const Actor) -> Sex;
        pub unsafe fn GetRace(actor: *const Actor) -> *const TESRace;
        
        // Methods cannot be accessed as members because it would require 
        // wrapping it in a smart point and allow rust to manage its memory, 
        // but the actor is managed by the game engine. Maybe we can
        // wrap it into a smart pointer that simply won't delete it
        // but whats the point at this point
        
        // TESForm
        pub unsafe fn GetFormID(form: *const TESForm) -> u32;

        // TESRace
        pub unsafe fn AsForm(form: *const TESRace) -> *const TESForm;
    } 
}
