use crate::request::RequestHeader;

#[repr(C, align(8))]
pub struct ExecutableAddressRequest {
    header: RequestHeader<ExecutableAddressResponse>,
}

unsafe impl Send for ExecutableAddressRequest {}
unsafe impl Sync for ExecutableAddressRequest {}

impl ExecutableAddressRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x71ba76863cc55f63, 0xb2644a48c516a487]),
        }
    }

    pub fn response(&self) -> Option<&'static ExecutableAddressResponse> {
        self.header.response()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ExecutableAddressResponse {
    revision: u64,
    physical_base: u64,
    virtual_base: u64,
}

unsafe impl Send for ExecutableAddressResponse {}
unsafe impl Sync for ExecutableAddressResponse {}
