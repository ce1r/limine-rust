use crate::RequestHeader;
use core::ffi::c_void;

/// Returns a [`DtbResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_dtb_request))]
pub struct DtbRequest {
    header: RequestHeader<DtbResponse>,
}

unsafe impl Send for DtbRequest {}
unsafe impl Sync for DtbRequest {}

impl DtbRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0xb40ddb48fb54bac7, 0x545081493f81ffb7]),
        }
    }

    pub fn response(&self) -> Option<&'static DtbResponse> {
        self.header.response()
    }
}

/// Returned by [`DtbRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_dtb_response))]
pub struct DtbResponse {
    revision: u64,
    pub dtb_ptr: *const c_void,
}

unsafe impl Send for DtbResponse {}
unsafe impl Sync for DtbResponse {}
