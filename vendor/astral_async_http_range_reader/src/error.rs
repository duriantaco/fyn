use std::sync::Arc;

/// Error type used for [`crate::AsyncHttpRangeReader`]
#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AsyncHttpRangeReaderError {
    /// The server does not support range requests
    #[error("range requests are not supported")]
    HttpRangeRequestUnsupported,

    /// Other HTTP error
    #[error(transparent)]
    HttpError(#[from] Arc<reqwest_middleware::Error>),

    /// An error occurred during transport
    #[error("an error occurred during transport: {0}")]
    TransportError(#[source] Arc<reqwest_middleware::Error>),

    /// An IO error occurred
    #[error("io error occurred: {0}")]
    IoError(#[source] Arc<std::io::Error>),

    /// Content-Range header is missing from response
    #[error("content-range header is missing from response")]
    ContentRangeMissing,

    /// Content-Length header is missing from response
    #[error("content-length header is missing from response")]
    ContentLengthMissing,

    /// Memory mapping the file failed
    #[error("memory mapping the file failed")]
    MemoryMapError(#[source] Arc<std::io::Error>),

    /// Error from `http-content-range`
    #[error("invalid Content-Range header: {0}")]
    ContentRangeParser(String),

    /// The server returned an invalid range response
    #[error(
        "request and response range mismatch, \
        expected {expected_start}-{expected_end_inclusive}/{expected_complete_length}, \
        got {actual_start}-{actual_end_inclusive}/{actual_complete_length}"
    )]
    RangeMismatch {
        expected_start: u64,
        expected_end_inclusive: u64,
        expected_complete_length: usize,
        actual_start: u64,
        actual_end_inclusive: u64,
        actual_complete_length: u64,
    },

    /// The server returned more bytes than the range request asked for
    #[error("range response returned more than the expected {expected} bytes")]
    ResponseTooLong { expected: u64 },

    /// The server returned fewer bytes than the range request asked for
    #[error("expected {expected} bytes from range response, got {actual}")]
    ResponseTooShort { expected: u64, actual: u64 },
}

impl From<std::io::Error> for AsyncHttpRangeReaderError {
    fn from(err: std::io::Error) -> Self {
        AsyncHttpRangeReaderError::IoError(Arc::new(err))
    }
}

impl From<reqwest_middleware::Error> for AsyncHttpRangeReaderError {
    fn from(err: reqwest_middleware::Error) -> Self {
        AsyncHttpRangeReaderError::TransportError(Arc::new(err))
    }
}

impl From<reqwest::Error> for AsyncHttpRangeReaderError {
    fn from(err: reqwest::Error) -> Self {
        AsyncHttpRangeReaderError::TransportError(Arc::new(err.into()))
    }
}
