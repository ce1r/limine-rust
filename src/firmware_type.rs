use crate::request::RequestHeader;

/// Returns a [`FirmwareTypeResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_firmware_type_request))]
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

/// Returned by [`FirmwareTypeRequest`].
#[derive(Debug)]
#[repr(C)]
#[cfg_attr(test, limine_test::test_layout(limine_firmware_type_response))]
pub struct FirmwareTypeResponse {
    revision: u64,
    firmware_type: u64,
}

unsafe impl Send for FirmwareTypeResponse {}
unsafe impl Sync for FirmwareTypeResponse {}

impl FirmwareTypeResponse {
    pub const fn firmware_type(&self) -> FirmwareType {
        match self.firmware_type {
            0 => FirmwareType::X86BIOS,
            1 => FirmwareType::EFI32,
            2 => FirmwareType::EFI64,
            3 => FirmwareType::SBI,
            _ => FirmwareType::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum FirmwareType {
    X86BIOS,
    EFI32,
    EFI64,
    SBI,
    Unknown,
}
