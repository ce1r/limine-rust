use crate::RequestHeader;

/// Returns a [`KeepIommuResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_x86_64_keep_iommu_request))]
pub struct KeepIommuRequest {
    header: RequestHeader<KeepIommuResponse>,
}

unsafe impl Send for KeepIommuRequest {}
unsafe impl Sync for KeepIommuRequest {}

impl KeepIommuRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x8ebaabe51f490179, 0x2aa86a59ffb4ab0f]),
        }
    }

    pub fn response(&self) -> Option<&'static KeepIommuResponse> {
        self.header.response()
    }
}

/// Returned by [`KeepIommuRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_x86_64_keep_iommu_response))]
pub struct KeepIommuResponse {
    revision: u64,
}

unsafe impl Send for KeepIommuResponse {}
unsafe impl Sync for KeepIommuResponse {}
