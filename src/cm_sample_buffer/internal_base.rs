use core_foundation::base::TCFType;
use std::ffi::c_void;

use core_foundation::{base::CFTypeID, declare_TCFType, impl_TCFType};

#[repr(C)]
pub struct __CMSampleBufferRef(c_void);

pub type CMSampleBufferRef = *mut __CMSampleBufferRef;

declare_TCFType! {CMSampleBuffer, CMSampleBufferRef}
impl_TCFType!(CMSampleBuffer, CMSampleBufferRef, CMSampleBufferGetTypeID);

extern "C" {
    pub fn CMSampleBufferGetTypeID() -> CFTypeID;
}
