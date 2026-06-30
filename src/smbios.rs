use crate::request::RequestHeader;
use core::num::NonZeroUsize;

#[repr(C, align(8))]
pub struct SmbiosRequest {
    header: RequestHeader<SmbiosResponse>,
}

unsafe impl Send for SmbiosRequest {}
unsafe impl Sync for SmbiosRequest {}

impl SmbiosRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x9e9046f11e095391, 0xaa4a520fefbde5ee]),
        }
    }

    pub fn response(&self) -> Option<&'static SmbiosResponse> {
        self.header.response()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SmbiosResponse {
    revision: u64,
    pub entry_32: Option<NonZeroUsize>,
    pub entry_64: Option<NonZeroUsize>,
}

unsafe impl Send for SmbiosResponse {}
unsafe impl Sync for SmbiosResponse {}
