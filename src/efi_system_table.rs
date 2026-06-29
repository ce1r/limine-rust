use crate::RequestHeader;

#[repr(C, align(8))]
pub struct EfiSystemTableRequest {
    header: RequestHeader<EfiSystemTableResponse>,
}

unsafe impl Send for EfiSystemTableRequest {}
unsafe impl Sync for EfiSystemTableRequest {}

impl EfiSystemTableRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x5ceba5163eaaf6d6, 0x0a6981610cf65fcc]),
        }
    }

    pub fn response(&self) -> Option<&'static EfiSystemTableResponse> {
        self.header.response()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiSystemTableResponse {
    revision: u64,
    pub address: *mut (),
}

unsafe impl Send for EfiSystemTableResponse {}
unsafe impl Sync for EfiSystemTableResponse {}
