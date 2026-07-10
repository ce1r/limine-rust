use crate::RequestHeader;
use core::ffi::c_void;

/// Returns a [`EfiMemoryMapResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_efi_memmap_request))]
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
#[cfg_attr(test, limine_test::test_layout(limine_efi_memmap_response))]
pub struct EfiMemoryMapResponse {
    revision: u64,
    memmap: *const c_void,
    memmap_size: u64,
    desc_size: u64,
    desc_version: u64,
}

unsafe impl Send for EfiMemoryMapResponse {}
unsafe impl Sync for EfiMemoryMapResponse {}
