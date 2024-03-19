mod internal {

    #![allow(non_snake_case)]
    #![allow(dead_code)]
    use std::{ffi::c_void, ptr};

    use core_audio_types_rs::audio_stream_packet_description::AudioStreamPacketDescription;
    use core_foundation::{
        base::{kCFAllocatorDefault, Boolean, CFRange, CFTypeID, OSStatus, TCFType},
        declare_TCFType, impl_TCFType,
        mach_port::CFAllocatorRef,
    };
    use core_video_rs::cv_image_buffer::CVImageBufferRef;

    use crate::{
        cm_block_buffer::CMBlockBufferRef,
        cm_format_description::CMFormatDescriptionRef,
        cm_sample_buffer_error::{CMSampleBufferError, NO_ERROR},
        cm_sample_timing_info::CMSampleTimingInfo,
        cm_time::CMTime,
        types::CMItemCount,
    };

    #[repr(C)]
    pub struct __CMSampleBufferRef(c_void);

    pub type CMSampleBufferRef = *mut __CMSampleBufferRef;

    declare_TCFType! {CMSampleBuffer, CMSampleBufferRef}
    impl_TCFType!(CMSampleBuffer, CMSampleBufferRef, CMSampleBufferGetTypeID);

    // In Apple's frameworks, a refcon parameter is often used as a way to pass user-defined data into a callback function. This is a common pattern in C and Objective-C APIs, especially those that deal with low-level system or hardware interactions.
    //
    // The term refcon stands for "reference constant". It's typically a void * pointer, which means it can point to any type of data. You're responsible for casting it to the correct type within your callback function.
    //
    // Here's a simple example:
    //
    // ;
    // In this example, SomeAppleAPIFunction is a hypothetical function in an Apple framework that takes a callback function and a refcon as parameters. We pass a pointer to data as the refcon, and then in MyCallbackFunction we cast the refcon back to a MyDataStruct * so we can access its fields.
    //
    // Remember that you need to ensure the data you're pointing to stays valid until the callback is called. If the data is a local variable in a function, and that function returns before the callback is called, then the refcon will be pointing to invalid memory. In such cases, you might need to dynamically allocate memory for the data (using malloc, for example) and then free it in the callback.
    type RefCon = *mut c_void;

    type CMSampleBufferMakeDataReadyCallback =
        extern "C" fn(sbuf: CMSampleBufferRef, makeDataReadyRefcon: RefCon) -> OSStatus;
    type CMSampleBufferMakeDataReadyHandler = extern "C" fn(sbuf: CMSampleBufferRef) -> OSStatus;

    type CMSampleBufferInvalidateHandler = extern "C" fn(sbuf: CMSampleBufferRef);
    type CMSampleBufferInvalidateCallback = extern "C" fn(sbuf: CMSampleBufferRef, refcon: RefCon);

    #[link(name = "CoreMedia", kind = "framework")]
    extern "C" {

        pub fn CMSampleBufferGetTypeID() -> CFTypeID;
        pub fn CMSampleBufferCreateReady(
            allocator: CFAllocatorRef,
            dataBuffer: CMBlockBufferRef,
            formatDescription: CMFormatDescriptionRef,
            sampleCount: CMItemCount,
            sampleTimingEntryCount: CMItemCount,
            sampleTimingArray: *const CMSampleTimingInfo,
            sampleSizeEntryCount: CMItemCount,
            sampleSizeArray: *const i64,
            sampleBufferOut: &mut CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMSampleBufferCreate(
            allocator: CFAllocatorRef,
            dataBuffer: CMBlockBufferRef,
            dataReady: Boolean,
            makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
            refcon: RefCon,
            formatDescription: CMFormatDescriptionRef,
            sampleCount: CMItemCount,
            sampleTimingEntryCount: CMItemCount,
            sampleTimingArray: *const CMSampleTimingInfo,
            sampleSizeEntryCount: CMItemCount,
            sampleSizeArray: *const i64,
            sampleBufferOut: *mut CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMSampleBufferCreateReadyWithImageBuffer(
            allocator: CFAllocatorRef,
            imageBuffer: CVImageBufferRef,
            formatDescription: CMFormatDescriptionRef,
            sampleTiming: *const CMSampleTimingInfo,
            sampleBufferOut: *mut CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMAudioSampleBufferCreateReadyWithPacketDescriptions(
            allocator: CFAllocatorRef,
            dataBuffer: CMBlockBufferRef,
            formatDescription: CMFormatDescriptionRef,
            sampleCount: CMItemCount,
            presentationTimeStamp: CMTime,
            packetDescriptions: *const AudioStreamPacketDescription,
            sampleBufferOut: *mut CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMSampleBufferCreateWithMakeDataReadyHandler(
            allocator: CFAllocatorRef,
            dataBuffer: CMBlockBufferRef,
            dataReady: Boolean,
            numSamples: CMItemCount,
            numSampleTimingEntries: CMItemCount,
            sampleTimingArray: *const CMSampleTimingInfo,
            numSampleSizeEntries: CMItemCount,
            sampleSizeArray: *const i64,
            sampleBufferOut: *mut CMSampleBufferRef,
            makeDataReadyHandler: CMSampleBufferMakeDataReadyHandler,
        ) -> OSStatus;

        pub fn CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler(
            allocator: CFAllocatorRef,
            imageBuffer: CVImageBufferRef,
            dataReady: Boolean,
            formatDescription: CMFormatDescriptionRef,
            sampleTiming: *const CMSampleTimingInfo,
            sampleBufferOut: *mut CMSampleBufferRef,
            makeDataReadyHandler: CMSampleBufferMakeDataReadyHandler,
        ) -> OSStatus;

        pub fn CMAudioSampleBufferCreateWithPacketDescriptionsAndMakeDataReadyHandler(
            allocator: CFAllocatorRef,
            dataBuffer: CMBlockBufferRef,
            dataReady: Boolean,
            formatDescription: CMFormatDescriptionRef,
            numSamples: u64,
            presentationTimeStamp: CMTime,
            packetDescriptions: *const c_void,
            sampleBufferOut: *mut CMSampleBufferRef,
            makeDataReadyHandler: CMSampleBufferMakeDataReadyHandler,
        ) -> OSStatus;

        pub fn CMSampleBufferCreateForImageBuffer(
            allocator: CFAllocatorRef,
            imageBuffer: CVImageBufferRef,
            dataReady: Boolean,
            makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
            refcon: RefCon,
            formatDescription: CMFormatDescriptionRef,
            sampleTiming: *const CMSampleTimingInfo,
            sampleBufferOut: *mut CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMAudioSampleBufferCreateWithPacketDescriptions(
            allocator: CFAllocatorRef,
            dataBuffer: CMBlockBufferRef,
            dataReady: Boolean,
            makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
            refcon: RefCon,
            formatDescription: CMFormatDescriptionRef,
            sampleCount: CMItemCount,
            presentationTimeStamp: CMTime,
            packetDescriptions: *const AudioStreamPacketDescription,
            sampleBufferOut: *mut CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMSampleBufferCreateCopy(
            allocator: CFAllocatorRef,
            sampleBuffer: CMSampleBufferRef,
            sampleBufferOut: *mut CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMSampleBufferCreateCopyWithNewTiming(
            allocator: CFAllocatorRef,
            sampleBuffer: CMSampleBufferRef,
            sampleTimingEntryCount: CMItemCount,
            sampleTimingArray: *const CMSampleTimingInfo,
            sampleBufferOut: *mut CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMSampleBufferCopySampleBufferForRange(
            allocator: CFAllocatorRef,
            sampleBuffer: CMSampleBufferRef,
            sampleRange: CFRange,
            sampleBufferOut: *mut CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMSampleBufferDataIsReady(sampleBuffer: CMSampleBufferRef) -> Boolean;

        pub fn CMSampleBufferSetDataReady(sampleBuffer: CMSampleBufferRef) -> OSStatus;

        pub fn CMSampleBufferSetDataFailed(
            sampleBuffer: CMSampleBufferRef,
            status: OSStatus,
        ) -> OSStatus;

        pub fn CMSampleBufferHasDataFailed(
            sampleBuffer: CMSampleBufferRef,
            statusOut: *mut OSStatus,
        ) -> Boolean;

        pub fn CMSampleBufferMakeDataReady(sampleBuffer: CMSampleBufferRef) -> OSStatus;

        pub fn CMSampleBufferTrackDataReadiness(
            sampleBuffer: CMSampleBufferRef,
            sampleBufferToTrack: CMSampleBufferRef,
        ) -> OSStatus;

        pub fn CMSampleBufferSetInvalidateHandler(
            sampleBuffer: CMSampleBufferRef,
            invalidateHandler: CMSampleBufferInvalidateHandler,
        ) -> OSStatus;

        pub fn CMSampleBufferInvalidate(sampleBuffer: CMSampleBufferRef) -> OSStatus;
        pub fn CMSampleBufferIsValid(sampleBuffer: CMSampleBufferRef) -> Boolean;
        pub fn CMSampleBufferSetInvalidateCallback(
            sampleBuffer: CMSampleBufferRef,
            callback: CMSampleBufferInvalidateCallback,
            refcon: RefCon,
        ) -> OSStatus;
    }

    pub(crate) fn empty() -> Result<CMSampleBuffer, CMSampleBufferError> {
        let mut sampleBufferOut: CMSampleBufferRef = ptr::null_mut();
        unsafe {
            let result = CMSampleBufferCreateReady(
                kCFAllocatorDefault,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                0,
                ptr::null_mut(),
                0,
                ptr::null(),
                &mut sampleBufferOut,
            );
            if result == NO_ERROR {
                Ok(CMSampleBuffer::wrap_under_create_rule(sampleBufferOut))
            } else {
                Err(CMSampleBufferError::from(result))
            }
        }
    }
}

pub use internal::{CMSampleBuffer, CMSampleBufferRef};

impl CMSampleBuffer {
    /// Creates a new [`CMSampleBuffer`].
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn new_empty() -> Self {
        internal::empty().expect("could not create empty sample buffer")
    }
}

impl Default for CMSampleBuffer {
    fn default() -> Self {
        Self::new_empty()
    }
}

#[cfg(test)]
mod test_cm_sample_buffer {
    use super::CMSampleBuffer;

    #[test]
    pub fn test_create_empty() {
        CMSampleBuffer::new_empty();
    }
}
