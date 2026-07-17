use crate::RequestHeader;
use core::ffi::c_void;

/// Returns a [`TpmEventLogResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_tpm_event_log_request))]
pub struct TpmEventLogRequest {
    header: RequestHeader<TpmEventLogResponse>,
}

unsafe impl Send for TpmEventLogRequest {}
unsafe impl Sync for TpmEventLogRequest {}

impl TpmEventLogRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x98e094fc7e76e979, 0xee8d8775c54e1d1f]),
        }
    }

    pub fn response(&self) -> Option<&'static TpmEventLogResponse> {
        self.header.response()
    }
}

/// Returned by [`TpmEventLogRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_tpm_event_log_response))]
pub struct TpmEventLogResponse {
    revision: u64,
    pub format: u64,
    pub size: u64,
    pub address: *const c_void,
}

unsafe impl Send for TpmEventLogResponse {}
unsafe impl Sync for TpmEventLogResponse {}
