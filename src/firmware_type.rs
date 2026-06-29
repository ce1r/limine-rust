use crate::RequestHeader;

#[repr(u64)]
#[derive(Debug)]
pub enum FirmwareType {
    X86BIOS = 0,
    EFI32 = 1,
    EFI64 = 2,
    SBI = 3,
}

#[repr(C, align(8))]
pub struct FirmwareTypeRequest {
    header: RequestHeader,
}

unsafe impl Send for FirmwareTypeRequest {}
unsafe impl Sync for FirmwareTypeRequest {}

impl FirmwareTypeRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x8c2f75d90bef28a8, 0x7045a4688eac00c3]),
        }
    }

    pub fn response(&self) -> Option<&'static FirmwareTypeResponse> {
        self.header.response()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct FirmwareTypeResponse {
    revision: u64,
    pub firmware_type: FirmwareType,
}

unsafe impl Send for FirmwareTypeResponse {}
unsafe impl Sync for FirmwareTypeResponse {}
