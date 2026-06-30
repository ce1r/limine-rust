use crate::request::RequestHeader;
use core::ffi::CStr;
use core::ffi::c_char;

#[repr(C, align(8))]
pub struct ExecutableCmdlineRequest {
    header: RequestHeader<ExecutableCmdlineResponse>,
}

unsafe impl Send for ExecutableCmdlineRequest {}
unsafe impl Sync for ExecutableCmdlineRequest {}

impl ExecutableCmdlineRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x4b161536e598651e, 0xb390ad4a2f1f303a]),
        }
    }

    pub fn response(&self) -> Option<&'static ExecutableCmdlineResponse> {
        self.header.response()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ExecutableCmdlineResponse {
    revision: u64,
    cmdline: *const c_char,
}

unsafe impl Send for ExecutableCmdlineResponse {}
unsafe impl Sync for ExecutableCmdlineResponse {}

impl ExecutableCmdlineResponse {
    pub fn cmdline(&self) -> &str {
        unsafe { CStr::from_ptr(self.cmdline).to_str().unwrap() }
    }
}
