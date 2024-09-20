pub mod error;
mod internal;

use core_foundation::base::CFAllocatorRef;
use error::CMSampleBufferError;
pub use internal::{CMSampleBuffer, CMSampleBufferRef};

use crate::{
    cm_block_buffer::CMBlockBuffer, cm_format_description::CMFormatDescription,
    cm_sample_timing_info::CMSampleTimingInfo, types::CMItemCount,
};

impl CMSampleBuffer {
    pub fn make_data_ready(&self) -> Result<(), CMSampleBufferError> {
        self.internal_make_data_ready()
    }
    #[allow(clippy::too_many_arguments)]
    pub fn create<TRefCon, TMakeDataReadyCallback>(
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
        Self::internal_create(
            allocator,
            block_buffer,
            data_ready,
            make_data_ready,
            refcon,
            format_description,
            sample_count,
            sample_timings,
            sample_sizes,
        )
    }
}
