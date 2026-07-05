use crate::request::RequestHeader;

/// Returns a [`BootloaderPerformanceResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_bootloader_performance_request))]
pub struct BootloaderPerformanceRequest {
    header: RequestHeader<BootloaderPerformanceResponse>,
}

unsafe impl Send for BootloaderPerformanceRequest {}
unsafe impl Sync for BootloaderPerformanceRequest {}

impl BootloaderPerformanceRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x6b50ad9bf36d13ad, 0xdc4c7e88fc759e17]),
        }
    }

    pub fn response(&self) -> Option<&'static BootloaderPerformanceResponse> {
        self.header.response()
    }
}

/// Returned by [`BootloaderPerformanceRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_bootloader_performance_response))]
pub struct BootloaderPerformanceResponse {
    revision: u64,
    pub reset_usec: u64,
    pub init_usec: u64,
    pub exec_usec: u64,
}

unsafe impl Send for BootloaderPerformanceResponse {}
unsafe impl Sync for BootloaderPerformanceResponse {}
