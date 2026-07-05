use crate::request::RequestHeader;

/// Returns a [`SmbiosResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_smbios_request))]
pub struct SmbiosRequest {
    header: RequestHeader<SmbiosResponse>,
}

unsafe impl Send for SmbiosRequest {}
unsafe impl Sync for SmbiosRequest {}

impl SmbiosRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x9e9046f11e095391, 0xaa4a520fefbde5ee]),
        }
    }

    pub fn response(&self) -> Option<&'static SmbiosResponse> {
        self.header.response()
    }
}

/// Returned by [`SmbiosRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_smbios_response))]
pub struct SmbiosResponse {
    revision: u64,
    pub entry_32: *const u8,
    pub entry_64: *const u8,
}

unsafe impl Send for SmbiosResponse {}
unsafe impl Sync for SmbiosResponse {}
