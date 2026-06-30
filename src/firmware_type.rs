use crate::request::RequestHeader;

#[derive(Debug)]
pub enum FirmwareType {
    X86BIOS,
    EFI32,
    EFI64,
    SBI,
    Unknown,
}

#[repr(C, align(8))]
pub struct FirmwareTypeRequest {
    header: RequestHeader<FirmwareTypeResponse>,
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
    firmware_type: u64,
}

unsafe impl Send for FirmwareTypeResponse {}
unsafe impl Sync for FirmwareTypeResponse {}

impl FirmwareTypeResponse {
    pub fn firmware_type(&self) -> FirmwareType {
        match self.firmware_type {
            0 => FirmwareType::X86BIOS,
            1 => FirmwareType::EFI32,
            2 => FirmwareType::EFI64,
            3 => FirmwareType::SBI,
            _ => FirmwareType::Unknown,
        }
    }
}
