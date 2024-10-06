use core_foundation::base::TCFType;
use core_utils_rs::{impl_TCFType, declare_TCFType};
use std::ffi::c_void;

use core_foundation::base::CFTypeID;

#[repr(C)]
pub struct __CMSampleBufferRef(c_void);
pub type CMSampleBufferRef = *mut __CMSampleBufferRef;

declare_TCFType!(CMSampleBuffer<'a>, CMSampleBufferRef);
impl_TCFType!(
    CMSampleBuffer<'a>,
    CMSampleBufferRef,
    CMSampleBufferGetTypeID
);
extern "C" {
    pub fn CMSampleBufferGetTypeID() -> CFTypeID;
}

