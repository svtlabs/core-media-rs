mod internal {

    #![allow(dead_code)]
    #![allow(clippy::too_many_arguments)]
    use std::{ffi::c_void, ptr};

    use core_audio_types_rs::audio_stream_packet_description::AudioStreamPacketDescription;
    use core_foundation::{
        base::{kCFAllocatorDefault, Boolean, CFRange, CFTypeID, OSStatus, TCFType},
        declare_TCFType, impl_TCFType,
        mach_port::CFAllocatorRef,
    };
    use core_utils_rs::trampoline::{create_trampoline, TrampolineCallback, TrampolineRefcon};
    use core_video_rs::cv_image_buffer::CVImageBufferRef;

    use crate::{
        cm_block_buffer::{CMBlockBuffer, CMBlockBufferRef},
        cm_format_description::{CMFormatDescription, CMFormatDescriptionRef},
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

        fn CMSampleBufferCreate(
            allocator: CFAllocatorRef,
            dataBuffer: CMBlockBufferRef,
            dataReady: Boolean,
            makeDataReadyCallback: TrampolineCallback,
            refcon: TrampolineRefcon,
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
            makeDataReadyHandler: Option<TrampolineCallback>,
        ) -> OSStatus;

        pub fn CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler(
            allocator: CFAllocatorRef,
            imageBuffer: CVImageBufferRef,
            dataReady: Boolean,
            formatDescription: CMFormatDescriptionRef,
            sampleTiming: *const CMSampleTimingInfo,
            sampleBufferOut: *mut CMSampleBufferRef,
            makeDataReadyHandler: Option<TrampolineCallback>,
        ) -> OSStatus;

        pub fn CMAudioSampleBufferCreateWithPacketDescriptionsAndMakeDataReadyHandler(
            allocator: CFAllocatorRef,
            dataBuffer: CMBlockBufferRef,
            dataReady: Boolean,
            formatDescription: CMFormatDescriptionRef,
            numSamples: CMItemCount,
            presentationTimeStamp: CMTime,
            packetDescriptions: *const c_void,
            sampleBufferOut: *mut CMSampleBufferRef,
            makeDataReadyHandler: Option<TrampolineCallback>,
        ) -> OSStatus;

        fn CMSampleBufferCreateForImageBuffer(
            allocator: CFAllocatorRef,
            imageBuffer: CVImageBufferRef,
            dataReady: Boolean,
            makeDataReadyCallback: Option<TrampolineCallback>,
            refcon: Option<&TrampolineCallback>,
            formatDescription: CMFormatDescriptionRef,
            sampleTiming: *const CMSampleTimingInfo,
            sampleBufferOut: *mut CMSampleBufferRef,
        ) -> OSStatus;

        fn CMAudioSampleBufferCreateWithPacketDescriptions(
            allocator: CFAllocatorRef,
            dataBuffer: CMBlockBufferRef,
            dataReady: Boolean,
            makeDataReadyCallback: Option<TrampolineCallback>,
            refcon: Option<&TrampolineCallback>,
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
            invalidateHandler: Option<TrampolineCallback>,
        ) -> OSStatus;

        pub fn CMSampleBufferInvalidate(sampleBuffer: CMSampleBufferRef) -> OSStatus;
        pub fn CMSampleBufferIsValid(sampleBuffer: CMSampleBufferRef) -> Boolean;
        pub fn CMSampleBufferSetInvalidateCallback(
            sampleBuffer: CMSampleBufferRef,
            invalidateHandler: Option<TrampolineCallback>,
            refcon: Option<&TrampolineCallback>,
        ) -> OSStatus;
    }

    pub(crate) fn make_data_ready(sbuf: &CMSampleBuffer) -> Result<(), CMSampleBufferError> {
        let result = unsafe { CMSampleBufferMakeDataReady(sbuf.clone().as_concrete_TypeRef()) };
        if result == NO_ERROR {
            Ok(())
        } else {
            Err(CMSampleBufferError::from(result))
        }
    }

    pub(crate) fn create<TRefCon, TMakeDataReadyCallback>(
        // cm_block_buffer: &CMBlockBuffer,
        data_ready: bool,
        make_data_ready: Option<TMakeDataReadyCallback>,
        refcon: TRefCon,
        // format_description: &CMFormatDescription,
        sample_count: CMItemCount,
        sample_timings: &[CMSampleTimingInfo],
        sample_sizes: &[i64],
    ) -> Result<CMSampleBuffer, CMSampleBufferError>
    where
        TRefCon: 'static,
        TMakeDataReadyCallback: FnOnce(CMSampleBuffer, TRefCon) + 'static,
    {
        let mut sample_buffer_out: CMSampleBufferRef = ptr::null_mut();
        let (caller, closure) = create_trampoline(move || {
            if let Some(cb) = make_data_ready {
                let sample_buff =
                    unsafe { CMSampleBuffer::wrap_under_create_rule(sample_buffer_out) };
                cb(sample_buff, refcon)
            }
        });
        unsafe {
            let result = CMSampleBufferCreate(
                kCFAllocatorDefault,
                ptr::null_mut(), //cm_block_buffer.as_concrete_TypeRef(),
                data_ready.into(),
                caller,
                closure,
                // format_description.as_concrete_TypeRef(),
                ptr::null_mut(),
                sample_count,
                sample_timings.len() as CMItemCount,
                sample_timings.as_ptr(),
                sample_sizes.len() as CMItemCount,
                sample_sizes.as_ptr(),
                &mut sample_buffer_out,
            );
            if result == NO_ERROR {
                Ok(CMSampleBuffer::wrap_under_create_rule(sample_buffer_out))
            } else {
                Err(CMSampleBufferError::from(result))
            }
        }
    }

    pub(crate) fn create_ready(
        cm_block_buffer: &CMBlockBuffer,
        format_description: &CMFormatDescription,
        sample_count: CMItemCount,
        sample_timings: &[CMSampleTimingInfo],
        sample_sizes: &[i64],
    ) -> Result<CMSampleBuffer, CMSampleBufferError> {
        let mut sample_buffer_out: CMSampleBufferRef = ptr::null_mut();
        unsafe {
            let result = CMSampleBufferCreateReady(
                kCFAllocatorDefault,
                cm_block_buffer.as_concrete_TypeRef(),
                format_description.as_concrete_TypeRef(),
                sample_count,
                sample_timings.len() as CMItemCount,
                sample_timings.as_ptr(),
                sample_sizes.len() as CMItemCount,
                sample_sizes.as_ptr(),
                &mut sample_buffer_out,
            );
            if result == NO_ERROR {
                Ok(CMSampleBuffer::wrap_under_create_rule(sample_buffer_out))
            } else {
                Err(CMSampleBufferError::from(result))
            }
        }
    }

    pub(crate) fn empty() -> Result<CMSampleBuffer, CMSampleBufferError> {
        let mut sample_buffer_out: CMSampleBufferRef = ptr::null_mut();
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
                &mut sample_buffer_out,
            );
            if result == NO_ERROR {
                Ok(CMSampleBuffer::wrap_under_create_rule(sample_buffer_out))
            } else {
                Err(result)?
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

    use crate::cm_sample_buffer_error::CMSampleBufferError;

    use super::{
        internal::{create, make_data_ready},
        CMSampleBuffer,
    };
    #[test]
    fn test_create() -> Result<(), CMSampleBufferError> {
        let sample_count = 0;
        let sample_timings = vec![];
        let sample_sizes = vec![];

        let buf = create(
            true,
            Some(|_a, _b| {}),
            (),
            sample_count,
            &sample_timings,
            &sample_sizes,
        )?;
        make_data_ready(&buf)
    }

    #[test]
    pub fn test_create_empty() {
        CMSampleBuffer::new_empty();
    }
}
