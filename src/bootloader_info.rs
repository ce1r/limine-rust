use crate::request::RequestHeader;
use core::ffi::CStr;
use core::ffi::c_char;

/// Returns a [`BootloaderInfoResponse`].
#[repr(C, align(8))]
pub struct BootloaderInfoRequest {
    header: RequestHeader<BootloaderInfoResponse>,
}

unsafe impl Send for BootloaderInfoRequest {}
unsafe impl Sync for BootloaderInfoRequest {}

impl BootloaderInfoRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0xf55038d8e2a1202f, 0x279426fcf5f59740]),
        }
    }

    pub fn response(&self) -> Option<&'static BootloaderInfoResponse> {
        self.header.response()
    }
}

/// Returned by [`BootloaderInfoRequest`].
#[repr(C)]
#[derive(Debug)]
pub struct BootloaderInfoResponse {
    revision: u64,
    name: *const c_char,
    version: *const c_char,
}

unsafe impl Send for BootloaderInfoResponse {}
unsafe impl Sync for BootloaderInfoResponse {}

impl BootloaderInfoResponse {
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap() }
    }

    pub fn version(&self) -> &str {
        unsafe { CStr::from_ptr(self.version).to_str().unwrap() }
    }
}
