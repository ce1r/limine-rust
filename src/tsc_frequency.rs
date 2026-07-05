use crate::request::RequestHeader;

/// Returns a [`TscFrequencyResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_tsc_frequency_request))]
pub struct TscFrequencyRequest {
    header: RequestHeader<TscFrequencyResponse>,
}

unsafe impl Send for TscFrequencyRequest {}
unsafe impl Sync for TscFrequencyRequest {}

impl TscFrequencyRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x10f2ee1d87d195e4, 0xf747a2b78f6ddb31]),
        }
    }

    pub fn response(&self) -> Option<&'static TscFrequencyResponse> {
        self.header.response()
    }
}

/// Returned by [`TscFrequencyRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_tsc_frequency_response))]
pub struct TscFrequencyResponse {
    revision: u64,
    pub frequency: u64,
}

unsafe impl Send for TscFrequencyResponse {}
unsafe impl Sync for TscFrequencyResponse {}
