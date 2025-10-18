 //! Basically all of the in-game types are exposed as raw pointers
 //! 
 //! cxx-crate methods only seem to work with smart pointers
 //! which would require allowing rust to manage the instances memory, 
 //! but since the memory is managed by the game engine this 
 //! seems to be not possible in a sensible way.

pub mod enum_form_id {
    /// Actor Value
    pub const K_AVIF : u32 = 0x62;
}

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
        pub type TESForm;
        pub type TESRace;
        pub type Actor;
        pub type NiAVObject;
    }
    
    #[allow(clippy::missing_safety_doc)]
    unsafe extern "C++" {
        include!("Bridge.h");
        pub unsafe fn PlayerCharacter_GetSingleton() -> *const Actor;

        // TESForm
        pub unsafe fn TESForm_GetFormByEditorID(editorId: &str) -> *const TESForm;
        pub unsafe fn GetFormID(form: *const TESForm) -> u32;
        pub unsafe fn GetSavedFormType(form: *const TESForm) -> u32; // enum_form_id
        pub unsafe fn RaceAsForm(form: *const TESRace) -> *const TESForm;
        pub unsafe fn ActorAsForm(form: *const Actor) -> *const TESForm;
        pub unsafe fn GetFormByID(a_formID: i32) -> *const TESForm;
        pub unsafe fn Form_GetEditorID(form: *const TESForm) -> String;

        // Actor
        pub unsafe fn IsPlayer(actor: *const Actor) -> bool;
        pub unsafe fn GetSex(actor: *const Actor) -> Sex;
        pub unsafe fn GetRace(actor: *const Actor) -> *const TESRace;
        pub unsafe fn GetBone(actor: *const Actor, bone: &str) -> *const NiAVObject;
        pub unsafe fn ContainsKeyword(actor: *const Actor, editorId: &str) -> bool;
        pub unsafe fn GetPlayerActorValue(actorValueEditorId: &str) -> f32; 
    } 
}
