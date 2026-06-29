use core::ffi::c_char;
use core::ffi::c_void;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum MediaType {
    Generic = 0,
    Optical = 1,
    TFTP = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Uuid {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct File {
    revision: u64,
    address: *mut c_void,
    size: u64,
    path: *const c_char,
    string: *const c_char,
    pub media_type: MediaType,
    _unused: u32,
    pub tftp_ipv4: [u8; 4],
    pub tftp_port: u32,
    pub partition_index: u32,
    pub mbr_disk_id: u32,
    pub gpt_disk_uuid: Uuid,
    pub gpt_part_uuid: Uuid,
    pub part_uuid: Uuid,
}
