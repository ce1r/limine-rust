use crate::RequestHeader;

/// Returns a [`DateAtBootResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_date_at_boot_request))]
pub struct DateAtBootRequest {
    header: RequestHeader<DateAtBootResponse>,
}

unsafe impl Send for DateAtBootRequest {}
unsafe impl Sync for DateAtBootRequest {}

impl DateAtBootRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x502746e184c088aa, 0xfbc5ec83e6327893]),
        }
    }

    pub fn response(&self) -> Option<&'static DateAtBootResponse> {
        self.header.response()
    }
}

/// Returned by [`DateAtBootRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_date_at_boot_response))]
pub struct DateAtBootResponse {
    revision: u64,
    timestamp: i64,
}

unsafe impl Send for DateAtBootResponse {}
unsafe impl Sync for DateAtBootResponse {}
