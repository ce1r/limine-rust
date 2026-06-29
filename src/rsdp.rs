use crate::RequestHeader;
use core::ffi::c_void;

#[repr(C, align(8))]
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

#[derive(Debug)]
#[repr(C)]
pub struct RsdpResponse {
    revision: u64,
    pub address: *mut c_void,
}

unsafe impl Send for RsdpResponse {}
unsafe impl Sync for RsdpResponse {}
