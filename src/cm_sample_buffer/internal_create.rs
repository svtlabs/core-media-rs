#![allow(clippy::too_many_arguments)]
use std::ptr;

use core_foundation::base::{Boolean, CFAllocatorRef, OSStatus, TCFType};
use core_utils_rs::trampoline::{
    create_right_trampoline, TrampolineRefcon, TrampolineRightCallback,
};

use crate::{
    cm_block_buffer::{CMBlockBuffer, CMBlockBufferRef},
    cm_format_description::{CMFormatDescription, CMFormatDescriptionRef},
    cm_sample_buffer::{error::NO_ERROR, internal_base::CMSampleBuffer},
    cm_sample_timing_info::CMSampleTimingInfo,
    types::CMItemCount,
};

use super::{error::CMSampleBufferError, internal_base::CMSampleBufferRef};

impl CMSampleBuffer {
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
        TRefCon: 'static + Send,
        TMakeDataReadyCallback:
            'static + Send + FnOnce(CMSampleBufferRef, TRefCon) -> Result<(), CMSampleBufferError>,
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
        allocator: CFAllocatorRef,
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
                allocator,
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