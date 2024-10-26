 //! Basically all of the in-game types are exposed as raw pointers
 //! 
 //! cxx-crate methods only seem to work with smart pointers
 //! which would require allowing rust to manage the instances memory, 
 //! but since the memory is managed by the game engine this 
 //! seems to be not possible in a sensible way.
 
 use ffi_bridge::*;

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
    
    unsafe extern "C++" {
        include!("Bridge.h");
        pub unsafe fn GetFormID(form: *const TESForm) -> u32;

        pub unsafe fn AsForm(form: *const TESRace) -> *const TESForm;

        pub unsafe fn IsPlayer(actor: *const Actor) -> bool;
        pub unsafe fn GetSex(actor: *const Actor) -> Sex;
        pub unsafe fn GetRace(actor: *const Actor) -> *const TESRace;
        pub unsafe fn GetBone(actor: *const Actor, bone: &str) -> *const NiAVObject;
    } 
}

#[derive(Clone)]
pub struct UnsafeTESFormPtr {
    ptr: *const TESForm
}
impl UnsafeTESFormPtr {
    pub fn get_form_id(&self) -> u32 {
        unsafe { GetFormID(self.ptr) }
    }
}

#[derive(Clone)]
pub struct UnsafeActorPtr {
    pub ptr: *const Actor,
}
impl UnsafeActorPtr {
    pub fn get_sex(&self) -> Sex {
        unsafe { GetSex(self.ptr) }
    }
    pub fn is_player(&self) -> bool {
        unsafe { IsPlayer(self.ptr) }
    }
    pub fn get_race(&self) -> UnsafeTESRacePtr {
        UnsafeTESRacePtr { ptr: unsafe { GetRace(self.ptr) } }
    }    
    pub fn get_bone(&self, bone_name: &str) -> UnsafeAvObjectPtr {
        let ptr = unsafe { GetBone(self.ptr, bone_name) };
        UnsafeAvObjectPtr {
            ptr,
            name: bone_name.to_owned(),
        }
    }
}
unsafe impl Send for UnsafeActorPtr {}

#[derive(Clone)]
pub struct UnsafeAvObjectPtr {
    pub name: String,
    pub ptr: *const NiAVObject,
}
unsafe impl Send for UnsafeAvObjectPtr {}

#[derive(Clone)]
pub struct UnsafeTESRacePtr {
    pub ptr: *const TESRace
}
impl From<UnsafeTESRacePtr> for UnsafeTESFormPtr {
    fn from(value: UnsafeTESRacePtr) -> Self {
        UnsafeTESFormPtr { 
            ptr: unsafe { AsForm(value.ptr) } 
        }
    }
}
