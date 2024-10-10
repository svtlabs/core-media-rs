use std::{
    alloc,
    ptr::{self},
};

use core_audio_types_rs::audio_buffer_list::AudioBufferList;
use core_foundation::base::{CFAllocatorRef, OSStatus, TCFType};

use crate::{
    cm_block_buffer::CMBlockBufferRef,
    cm_sample_buffer::{error::NO_ERROR, CMSampleBufferRef},
};

use super::{error::CMSampleBufferError, CMSampleBuffer};

pub const K_CMSAMPLE_BUFFER_FLAG_AUDIO_BUFFER_LIST_ASSURE16_BYTE_ALIGNMENT: u32 = 1 << 0;

impl CMSampleBuffer {
    pub fn internal_get_audio_buffer_list(&self) -> Result<AudioBufferList, CMSampleBufferError> {
        extern "C" {
            fn CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer(
                sbuf: CMSampleBufferRef,
                buffer_list_size_needed_out: *mut usize,
                buffer_list_out: *mut AudioBufferList,
                buffer_list_size: usize,
                bbuf_struct_allocator: CFAllocatorRef,
                bbuf_memory_allocator: CFAllocatorRef,
                flags: u32,
                block_buffer_out: *mut CMBlockBufferRef,
            ) -> OSStatus;
        }
        unsafe {
            let mut buffer_size = 0;
            CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer(
                self.as_concrete_TypeRef(),
                &mut buffer_size,
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut ptr::null_mut(),
            );
            let block_buffer_ref = self.internal_get_data_buffer()?;
            let layout = alloc::Layout::from_size_align(buffer_size, 16)
                .map_err(|_e| CMSampleBufferError::CouldNotGetDataBuffer)?;
            let audio_buffer_list_ptr = alloc::alloc(layout).cast::<AudioBufferList>();

            let result = CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer(
                self.clone().as_concrete_TypeRef(),
                ptr::null_mut(),
                audio_buffer_list_ptr as _,
                buffer_size,
                ptr::null_mut(),
                ptr::null_mut(),
                K_CMSAMPLE_BUFFER_FLAG_AUDIO_BUFFER_LIST_ASSURE16_BYTE_ALIGNMENT,
                &mut block_buffer_ref.as_concrete_TypeRef(),
            );

            if result != NO_ERROR {
                Err(CMSampleBufferError::from(result))
            } else {
                let buffer_list: AudioBufferList = ptr::read(audio_buffer_list_ptr);
                Ok(buffer_list)
            }
        }
    }
}
