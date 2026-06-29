use crate::RequestHeader;

#[repr(C, align(8))]
pub struct StackSizeRequest {
    header: RequestHeader<StackSizeResponse>,
    size: u64,
}

unsafe impl Send for StackSizeRequest {}
unsafe impl Sync for StackSizeRequest {}

impl StackSizeRequest {
    pub const fn new(size: u64) -> Self {
        Self {
            header: RequestHeader::new([0x224ef0460a8e8926, 0xe1cb0fc25f46ea3d]),
            size,
        }
    }

    pub fn response(&self) -> Option<&'static StackSizeResponse> {
        self.header.response()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct StackSizeResponse {
    revision: u64,
}

unsafe impl Send for StackSizeResponse {}
unsafe impl Sync for StackSizeResponse {}
