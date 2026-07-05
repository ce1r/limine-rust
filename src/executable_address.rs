use crate::request::RequestHeader;

/// Returns a [`ExecutableAddressResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_executable_address_request))]
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

/// Returned by [`ExecutableAddressRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_executable_address_response))]
pub struct ExecutableAddressResponse {
    revision: u64,
    physical_base: u64,
    virtual_base: u64,
}

unsafe impl Send for ExecutableAddressResponse {}
unsafe impl Sync for ExecutableAddressResponse {}
