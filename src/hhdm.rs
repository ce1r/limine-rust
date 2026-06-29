use crate::RequestHeader;

#[repr(C, align(8))]
pub struct HhdmRequest {
    header: RequestHeader<HhdmResponse>,
}

unsafe impl Send for HhdmRequest {}
unsafe impl Sync for HhdmRequest {}

impl HhdmRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x48dcf1cb8ad2b852, 0x63984e959a98244b]),
        }
    }

    pub fn response(&self) -> Option<&'static HhdmResponse> {
        self.header.response()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct HhdmResponse {
    revision: u64,
    offset: u64,
}

unsafe impl Send for HhdmResponse {}
unsafe impl Sync for HhdmResponse {}
