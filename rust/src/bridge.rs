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
        type Actor = crate::ffi::Actor;
    }
    
    unsafe extern "C++" {
        include!("Bridge.h");
        pub unsafe fn IsPlayer(actor: *const Actor) -> bool;
        pub unsafe fn GetSex(actor: *const Actor) -> Sex;
    } 
}
