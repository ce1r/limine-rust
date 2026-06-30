use crate::request::RequestHeader;

#[repr(C, align(8))]
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

#[repr(C)]
#[derive(Debug)]
pub struct DateAtBootResponse {
    revision: u64,
    timestamp: i64,
}

unsafe impl Send for DateAtBootResponse {}
unsafe impl Sync for DateAtBootResponse {}
