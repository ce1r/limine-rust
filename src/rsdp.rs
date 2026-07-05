use crate::request::RequestHeader;
use core::ffi::c_void;

/// Returns a [`RsdpResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_rsdp_request))]
pub struct RsdpRequest {
    header: RequestHeader<RsdpResponse>,
}

unsafe impl Send for RsdpRequest {}
unsafe impl Sync for RsdpRequest {}

impl RsdpRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0xc5e77b6b397e7b43, 0x27637845accdcf3c]),
        }
    }

    pub fn response(&self) -> Option<&'static RsdpResponse> {
        self.header.response()
    }
}

/// Returned by [`RsdpRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_rsdp_response))]
pub struct RsdpResponse {
    revision: u64,
    address: *const c_void,
}

unsafe impl Send for RsdpResponse {}
unsafe impl Sync for RsdpResponse {}

impl RsdpResponse {
    pub const fn address(&self) -> *const u8 {
        self.address.cast()
    }
}
