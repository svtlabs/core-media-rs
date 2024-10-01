#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
use std::{ffi::c_void, ptr};

use core_audio_types_rs::audio_stream_packet_description::AudioStreamPacketDescription;
use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFRange, CFTypeID, OSStatus, TCFType},
    declare_TCFType, impl_TCFType,
    mach_port::CFAllocatorRef,
};
use core_utils_rs::trampoline::{
    create_right_trampoline, TrampolineLeftCallback, TrampolineRefcon, TrampolineRightCallback,
};
use core_video_rs::cv_image_buffer::CVImageBufferRef;

use crate::{
    cm_block_buffer::{CMBlockBuffer, CMBlockBufferRef},
    cm_format_description::{CMFormatDescription, CMFormatDescriptionRef},
    cm_sample_buffer::error::CMSampleBufferError,
    cm_sample_timing_info::CMSampleTimingInfo,
    cm_time::CMTime,
    types::{CMItemCount, CMItemIndex},
};

use super::error::NO_ERROR;

#[repr(C)]
pub struct __CMSampleBufferRef(c_void);

pub type CMSampleBufferRef = *mut __CMSampleBufferRef;

declare_TCFType! {CMSampleBuffer, CMSampleBufferRef}
impl_TCFType!(CMSampleBuffer, CMSampleBufferRef, CMSampleBufferGetTypeID);

extern "C" {

    pub fn CMSampleBufferGetTypeID() -> CFTypeID;
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
        makeDataReadyHandler: Option<TrampolineLeftCallback<OSStatus>>,
    ) -> OSStatus;

    pub fn CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        dataReady: Boolean,
        formatDescription: CMFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
        makeDataReadyHandler: Option<TrampolineLeftCallback<OSStatus>>,
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
        makeDataReadyHandler: Option<TrampolineLeftCallback<OSStatus>>,
    ) -> OSStatus;

    fn CMSampleBufferCreateForImageBuffer(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        dataReady: Boolean,
        makeDataReadyCallback: Option<TrampolineLeftCallback<OSStatus>>,
        refcon: Option<&TrampolineLeftCallback>,
        formatDescription: CMFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;

    fn CMAudioSampleBufferCreateWithPacketDescriptions(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: Boolean,
        makeDataReadyCallback: Option<TrampolineLeftCallback<OSStatus>>,
        refcon: Option<&TrampolineLeftCallback>,
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

    pub fn CMSampleBufferTrackDataReadiness(
        sampleBuffer: CMSampleBufferRef,
        sampleBufferToTrack: CMSampleBufferRef,
    ) -> OSStatus;

    pub fn CMSampleBufferSetInvalidateHandler(
        sampleBuffer: CMSampleBufferRef,
        invalidateHandler: Option<TrampolineLeftCallback>,
    ) -> OSStatus;

    pub fn CMSampleBufferInvalidate(sampleBuffer: CMSampleBufferRef) -> OSStatus;
    pub fn CMSampleBufferIsValid(sampleBuffer: CMSampleBufferRef) -> Boolean;
    pub fn CMSampleBufferSetInvalidateCallback(
        sampleBuffer: CMSampleBufferRef,
        invalidateHandler: Option<TrampolineLeftCallback>,
        refcon: Option<&TrampolineLeftCallback>,
    ) -> OSStatus;
}
impl CMSampleBuffer {
    pub(super) fn internal_get_num_samples(&self) -> CMItemCount {
        extern "C" {
            pub fn CMSampleBufferGetNumSamples(sampleBuffer: CMSampleBufferRef) -> CMItemCount;
        }
        unsafe { CMSampleBufferGetNumSamples(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_get_total_sample_size(&self) -> isize {
        extern "C" {
            pub fn CMSampleBufferGetTotalSampleSize(sampleBuffer: CMSampleBufferRef) -> isize;
        }
        unsafe { CMSampleBufferGetTotalSampleSize(self.as_concrete_TypeRef()) }
    }
    pub(super) fn internal_get_sample_size(&self, at: CMItemIndex) -> isize {
        extern "C" {
            pub fn CMSampleBufferGetSampleSize(
                sample_buffer: CMSampleBufferRef,
                at: CMItemIndex,
            ) -> isize;
        }

        unsafe { CMSampleBufferGetSampleSize(self.as_concrete_TypeRef(), at) }
    }
    pub(super) fn internal_get_sample_size_array(&self) -> Result<Vec<isize>, CMSampleBufferError> {
        extern "C" {
            pub fn CMSampleBufferGetSampleSizeArray(
                sample_buffer: CMSampleBufferRef,
                array_to_fill: *mut isize,
                entry_count: CMItemCount,
                entires_needed_out: *mut CMItemCount,
            ) -> OSStatus;
        }
        let mut entries_needed_out: CMItemCount = 0;

        let result = unsafe {
            CMSampleBufferGetSampleSizeArray(
                self.as_concrete_TypeRef(),
                ptr::null_mut(),
                0,
                &mut entries_needed_out,
            )
        };

        if result != NO_ERROR {
            return Err(CMSampleBufferError::from(result));
        }
        let mut result_vec = vec![0isize; entries_needed_out as usize];
        let result = unsafe {
            CMSampleBufferGetSampleSizeArray(
                self.as_concrete_TypeRef(),
                result_vec.as_mut_ptr(),
                entries_needed_out,
                &mut entries_needed_out,
            )
        };

        if result == NO_ERROR {
            Ok(result_vec)
        } else {
            Err(CMSampleBufferError::from(result))
        }
    }
    pub(super) fn internal_get_audio_buffer_list(&self) {}
    pub(super) fn internal_get_format_description(
        &self,
    ) -> Result<CMFormatDescription, CMSampleBufferError> {
        extern "C" {
            pub fn CMSampleBufferGetFormatDescription(
                sampleBuffer: CMSampleBufferRef,
            ) -> CMFormatDescriptionRef;
        }
        let reference = unsafe { CMSampleBufferGetFormatDescription(self.as_concrete_TypeRef()) };
        if reference.is_null() {
            Err(CMSampleBufferError::CouldNotGetFormatDescription)
        } else {
            Ok(unsafe { CMFormatDescription::wrap_under_get_rule(reference) })
        }
    }
    pub(super) fn internal_make_data_ready(&self) -> Result<(), CMSampleBufferError> {
        extern "C" {
            pub fn CMSampleBufferMakeDataReady(sampleBuffer: CMSampleBufferRef) -> OSStatus;

        }
        let result = unsafe { CMSampleBufferMakeDataReady(self.clone().as_concrete_TypeRef()) };
        if result == NO_ERROR {
            Ok(())
        } else {
            Err(CMSampleBufferError::from(result))
        }
    }
    pub(super) fn internal_create<TRefCon, TMakeDataReadyCallback>(
        allocator: CFAllocatorRef,
        block_buffer: &CMBlockBuffer,
        data_ready: bool,
        make_data_ready: TMakeDataReadyCallback,
        refcon: TRefCon,
        format_description: &CMFormatDescription,
        sample_count: CMItemCount,
        sample_timings: &[CMSampleTimingInfo],
        sample_sizes: &[i64],
    ) -> Result<Self, CMSampleBufferError>
    where
        TRefCon: Send + 'static,
        TMakeDataReadyCallback:
            Send + FnOnce(CMSampleBufferRef, TRefCon) -> Result<(), CMSampleBufferError> + 'static,
    {
        extern "C" {
            fn CMSampleBufferCreate(
                allocator: CFAllocatorRef,
                dataBuffer: CMBlockBufferRef,
                dataReady: Boolean,
                makeDataReadyCallback: TrampolineRightCallback<OSStatus, CMSampleBufferRef>,
                refcon: TrampolineRefcon,
                formatDescription: CMFormatDescriptionRef,
                sampleCount: CMItemCount,
                sampleTimingEntryCount: CMItemCount,
                sampleTimingArray: *const CMSampleTimingInfo,
                sampleSizeEntryCount: CMItemCount,
                sampleSizeArray: *const i64,
                sampleBufferOut: *mut CMSampleBufferRef,
            ) -> OSStatus;

        }
        let mut sample_buffer_out: CMSampleBufferRef = ptr::null_mut();

        let (caller, closure) = create_right_trampoline(move |r: CMSampleBufferRef| {
            make_data_ready(r, refcon)
                .map(|_| NO_ERROR)
                .unwrap_or_else(|e| e.into())
        });

        unsafe {
            let result = CMSampleBufferCreate(
                allocator,
                block_buffer.clone().as_concrete_TypeRef(),
                data_ready.into(),
                caller,
                closure,
                format_description.clone().as_concrete_TypeRef(),
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
    pub(super) fn internal_create_ready(
        cm_block_buffer: &CMBlockBuffer,
        format_description: &CMFormatDescription,
        sample_count: CMItemCount,
        sample_timings: &[CMSampleTimingInfo],
        sample_sizes: &[i64],
    ) -> Result<CMSampleBuffer, CMSampleBufferError> {
        extern "C" {
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

        }
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
}
