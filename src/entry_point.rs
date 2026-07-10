use crate::RequestHeader;

type EntryPoint = extern "C" fn() -> !;

/// Returns a [`EntryPointResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_entry_point_request))]
pub struct EntryPointRequest {
    header: RequestHeader<EntryPointResponse>,
    entry: EntryPoint,
}

unsafe impl Send for EntryPointRequest {}
unsafe impl Sync for EntryPointRequest {}

impl EntryPointRequest {
    pub const fn new(entry: EntryPoint) -> Self {
        Self {
            header: RequestHeader::new([0x13d86c035a1cd3e1, 0x2b0caa89d8f3026a]),
            entry,
        }
    }

    pub fn response(&self) -> Option<&'static EntryPointResponse> {
        self.header.response()
    }
}

/// Returned by [`EntryPointRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_entry_point_response))]
pub struct EntryPointResponse {
    revision: u64,
}

unsafe impl Send for EntryPointResponse {}
unsafe impl Sync for EntryPointResponse {}
