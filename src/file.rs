use core::ffi::c_char;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, limine_test::test_layout(limine_file))]
pub struct File {
    revision: u64,
    address: *mut c_void,
    size: u64,
    path: *const c_char,
    string: *const c_char,
    media_type: u32,
    unused: u32,
    pub tftp_ipv4: [u8; 4],
    pub tftp_port: u32,
    pub partition_index: u32,
    pub mbr_disk_id: u32,
    pub gpt_disk_uuid: Uuid,
    pub gpt_part_uuid: Uuid,
    pub part_uuid: Uuid,
}

impl File {
    pub const fn media_type(&self) -> MediaType {
        match self.media_type {
            0 => MediaType::Generic,
            1 => MediaType::Optical,
            2 => MediaType::TFTP,
            _ => MediaType::Unknown,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, limine_test::test_layout(limine_uuid))]
pub struct Uuid {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}

#[derive(Debug, Clone, Copy)]
pub enum MediaType {
    Generic,
    Optical,
    TFTP,
    Unknown,
}
