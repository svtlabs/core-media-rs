#![allow(non_snake_case)]
use crate::cm_time::CMTime;

#[repr(C)]
pub struct CMSampleTimingInfo {
    pub duration: CMTime,
    pub presentationTimeStamp: CMTime,
    pub decodeTimeStamp: CMTime,
}

impl CMSampleTimingInfo {
    pub fn new(duration: CMTime, presentationTimeStamp: CMTime, decodeTimeStamp: CMTime) -> Self {
        CMSampleTimingInfo {
            duration,
            presentationTimeStamp,
            decodeTimeStamp,
        }
    }
}

impl Default for CMSampleTimingInfo {
    fn default() -> Self {
        CMSampleTimingInfo {
            duration: CMTime::default(),
            presentationTimeStamp: CMTime::default(),
            decodeTimeStamp: CMTime::default(),
        }
    }
}
