use bridge::ffi_bridge::*;

pub mod bridge;

#[derive(Clone)]
pub struct UnsafeTESFormPtr {
    ptr: *const TESForm
}
impl UnsafeTESFormPtr {
    pub fn get_form_id(&self) -> u32 {
        unsafe { GetFormID(self.ptr) }
    }
    pub fn get_editor_id(&self) -> String {
        unsafe {
            Form_GetEditorID(self.ptr)
        }
    }
}
impl From<UnsafeActorPtr> for UnsafeTESFormPtr {
    fn from(value: UnsafeActorPtr) -> Self {
        UnsafeTESFormPtr { 
            ptr: unsafe { ActorAsForm(value.ptr) } 
        }
    }
}
impl From<UnsafeTESRacePtr> for UnsafeTESFormPtr {
    fn from(value: UnsafeTESRacePtr) -> Self {
        UnsafeTESFormPtr { 
            ptr: unsafe { RaceAsForm(value.ptr) } 
        }
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
impl UnsafeTESRacePtr {
    pub fn get_form_id(&self) -> u32 {
        let race_form: UnsafeTESFormPtr = self.clone().into();
        race_form.get_form_id()
    }
}
