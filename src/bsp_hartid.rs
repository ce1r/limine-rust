use crate::RequestHeader;

/// Returns a [`BspHartidResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_riscv_bsp_hartid_request))]
pub struct BspHartidRequest {
    header: RequestHeader<BspHartidResponse>,
}

unsafe impl Send for BspHartidRequest {}
unsafe impl Sync for BspHartidRequest {}

impl BspHartidRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x1369359f025525f9, 0x2ff2a56178391bb6]),
        }
    }

    pub fn response(&self) -> Option<&'static BspHartidResponse> {
        self.header.response()
    }
}

/// Returned by [`BspHartidRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_riscv_bsp_hartid_response))]
pub struct BspHartidResponse {
    revision: u64,
    pub bsp_hartid: i64,
}

unsafe impl Send for BspHartidResponse {}
unsafe impl Sync for BspHartidResponse {}
