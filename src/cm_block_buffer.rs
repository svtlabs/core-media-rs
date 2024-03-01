mod internal {
    use std::ffi::c_void;

    use core_foundation::base::{CFTypeID, TCFType};
    use core_foundation::{declare_TCFType, impl_TCFType};

    #[repr(C)]
    pub struct __CMBlockBufferRef(c_void);

    pub type CMBlockBufferRef = *mut __CMBlockBufferRef;

    declare_TCFType! {CMBlockBuffer, CMBlockBufferRef}
    impl_TCFType!(CMBlockBuffer, CMBlockBufferRef, CMBlockBufferGetTypeID);

    extern "C" {
        pub fn CMBlockBufferGetTypeID() -> CFTypeID;
    }
}
pub use internal::{CMBlockBuffer, CMBlockBufferRef};
