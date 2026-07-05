use crate::request::RequestHeader;
use core::ffi::c_void;

/// Returns a [`EfiSystemTableResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_efi_system_table_request))]
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

/// Returned by [`EfiSystemTableRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_efi_system_table_response))]
pub struct EfiSystemTableResponse {
    revision: u64,
    address: *const c_void,
}

unsafe impl Send for EfiSystemTableResponse {}
unsafe impl Sync for EfiSystemTableResponse {}

impl EfiSystemTableResponse {
    pub const fn address(&self) -> *const u8 {
        self.address.cast()
    }
}
