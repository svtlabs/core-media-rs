use core_foundation::base::OSStatus;
use thiserror::Error;

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {

    static kCMSampleBufferError_AllocationFailed: OSStatus;
    static kCMSampleBufferError_AlreadyHasDataBuffer: OSStatus;
    static kCMSampleBufferError_ArrayTooSmall: OSStatus;
    static kCMSampleBufferError_BufferHasNoSampleSizes: OSStatus;
    static kCMSampleBufferError_BufferHasNoSampleTimingInfo: OSStatus;
    static kCMSampleBufferError_BufferNotReady: OSStatus;
    static kCMSampleBufferError_CannotSubdivide: OSStatus;
    static kCMSampleBufferError_DataCanceled: OSStatus;
    static kCMSampleBufferError_DataFailed: OSStatus;
    static kCMSampleBufferError_InvalidEntryCount: OSStatus;
    static kCMSampleBufferError_InvalidMediaFormat: OSStatus;
    static kCMSampleBufferError_InvalidMediaTypeForOperation: OSStatus;
    static kCMSampleBufferError_InvalidSampleData: OSStatus;
    static kCMSampleBufferError_Invalidated: OSStatus;
    static kCMSampleBufferError_RequiredParameterMissing: OSStatus;
    static kCMSampleBufferError_SampleIndexOutOfRange: OSStatus;
    static kCMSampleBufferError_SampleTimingInfoInvalid: OSStatus;

}

#[derive(Error, Debug, Clone)]
pub enum CMSampleBufferError {
    #[error("The system failed to allocate memory.")]
    AllocationFailed,
    #[error("An attempt to set data on a sample buffer failed because that buffer already contains media data.")]
    AlreadyHasDataBuffer,
    #[error("The output array isn’t large enough to hold the requested array.")]
    ArrayTooSmall,
    #[error("A request for sample sizes on a buffer failed because the buffer doesn’t provide that information.")]
    BufferHasNoSampleSizes,
    #[error("A request for sample timing on a buffer failed because the buffer doesn’t contain that information.")]
    BufferHasNoSampleTimingInfo,
    #[error("The system can’t make the buffer’s data ready for use.")]
    BufferNotReady,
    #[error("A sample buffer doesn’t contain sample sizes.")]
    CannotSubdivide,
    #[error("A sample buffer canceled its data-loading operation.")]
    DataCanceled,
    #[error("A sample buffer failed to load its data.")]
    DataFailed,
    #[error("A timing or size value isn’t within the allowed range.")]
    InvalidEntryCount,
    #[error("The media format doesn’t match the sample buffer’s format description.")]
    InvalidMediaFormat,
    #[error("The media type that the format description defines isn’t a value for the requested operation.")]
    InvalidMediaTypeForOperation,
    #[error("The sample buffer contains bad data.")]
    InvalidSampleData,
    #[error("A sample buffer invalidated its data.")]
    Invalidated,
    #[error("A required parameter’s value is invalid.")]
    RequiredParameterMissing,
    #[error("The sample index is outside the range of samples that the buffer contains.")]
    SampleIndexOutOfRange,
    #[error("The sample buffer unexpectedly contains nonnumeric sample-timing information.")]
    SampleTimingInfoInvalid,
    #[error("An unknown error occurred with code {0}")]
    UnknownError(OSStatus),
}

pub const NO_ERROR: OSStatus = 0;

impl From<OSStatus> for CMSampleBufferError {
    fn from(value: OSStatus) -> Self {
        unsafe {
            if value == kCMSampleBufferError_AllocationFailed {
                CMSampleBufferError::AllocationFailed
            } else if value == kCMSampleBufferError_AlreadyHasDataBuffer {
                CMSampleBufferError::AlreadyHasDataBuffer
            } else if value == kCMSampleBufferError_ArrayTooSmall {
                CMSampleBufferError::ArrayTooSmall
            } else if value == kCMSampleBufferError_BufferHasNoSampleSizes {
                CMSampleBufferError::BufferHasNoSampleSizes
            } else if value == kCMSampleBufferError_BufferHasNoSampleTimingInfo {
                CMSampleBufferError::BufferHasNoSampleTimingInfo
            } else if value == kCMSampleBufferError_BufferNotReady {
                CMSampleBufferError::BufferNotReady
            } else if value == kCMSampleBufferError_CannotSubdivide {
                CMSampleBufferError::CannotSubdivide
            } else if value == kCMSampleBufferError_DataCanceled {
                CMSampleBufferError::DataCanceled
            } else if value == kCMSampleBufferError_DataFailed {
                CMSampleBufferError::DataFailed
            } else if value == kCMSampleBufferError_InvalidEntryCount {
                CMSampleBufferError::InvalidEntryCount
            } else if value == kCMSampleBufferError_InvalidMediaFormat {
                CMSampleBufferError::InvalidMediaFormat
            } else if value == kCMSampleBufferError_InvalidMediaTypeForOperation {
                CMSampleBufferError::InvalidMediaTypeForOperation
            } else if value == kCMSampleBufferError_InvalidSampleData {
                CMSampleBufferError::InvalidSampleData
            } else if value == kCMSampleBufferError_Invalidated {
                CMSampleBufferError::Invalidated
            } else if value == kCMSampleBufferError_RequiredParameterMissing {
                CMSampleBufferError::RequiredParameterMissing
            } else if value == kCMSampleBufferError_SampleIndexOutOfRange {
                CMSampleBufferError::SampleIndexOutOfRange
            } else if value == kCMSampleBufferError_SampleTimingInfoInvalid {
                CMSampleBufferError::SampleTimingInfoInvalid
            } else {
                CMSampleBufferError::UnknownError(value)
            }
        }
    }
}
