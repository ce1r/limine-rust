use crate::request::RequestHeader;

/// Returns a [`StackSizeResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_stack_size_request))]
pub struct StackSizeRequest {
    header: RequestHeader<StackSizeResponse>,
    stack_size: u64,
}

unsafe impl Send for StackSizeRequest {}
unsafe impl Sync for StackSizeRequest {}

impl StackSizeRequest {
    pub const fn new(stack_size: u64) -> Self {
        Self {
            header: RequestHeader::new([0x224ef0460a8e8926, 0xe1cb0fc25f46ea3d]),
            stack_size,
        }
    }

    pub fn response(&self) -> Option<&'static StackSizeResponse> {
        self.header.response()
    }
}

/// Returned by [`StackSizeRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_stack_size_response))]
pub struct StackSizeResponse {
    revision: u64,
}

unsafe impl Send for StackSizeResponse {}
unsafe impl Sync for StackSizeResponse {}
