use crate::request::RequestHeader;
use core::ffi::c_void;

/// Returns a [`EfiMemoryMapResponse`].
#[repr(C, align(8))]
pub struct EfiMemoryMapRequest {
    header: RequestHeader<EfiMemoryMapResponse>,
}

unsafe impl Send for EfiMemoryMapRequest {}
unsafe impl Sync for EfiMemoryMapRequest {}

impl EfiMemoryMapRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x7df62a431d6872d5, 0xa4fcdfb3e57306c8]),
        }
    }

    pub fn response(&self) -> Option<&'static EfiMemoryMapResponse> {
        self.header.response()
    }
}

/// Returned by [`EfiMemoryMapRequest`].
#[repr(C)]
#[derive(Debug)]
pub struct EfiMemoryMapResponse {
    revision: u64,
    memory_map: *const c_void,
    memory_map_size: u64,
    descriptor_size: u64,
    descriptor_version: u64,
}

unsafe impl Send for EfiMemoryMapResponse {}
unsafe impl Sync for EfiMemoryMapResponse {}
