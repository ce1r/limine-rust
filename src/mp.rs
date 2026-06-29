use crate::RequestHeader;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::AtomicU64;

#[repr(C, align(8))]
pub struct MpRequest {
    header: RequestHeader,
    flags: u64,
}

unsafe impl Send for MpRequest {}
unsafe impl Sync for MpRequest {}

impl MpRequest {
    pub const fn new(enable_x2apic: bool) -> Self {
        Self {
            header: RequestHeader::new([0x95a67b819a1b857e, 0xa0b61b723b6a73e0]),
            flags: enable_x2apic as u64,
        }
    }

    pub fn response(&self) -> Option<&'static MpResponse> {
        self.header.response()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MpResponse {
    revision: u64,
    pub flags: u32,
    pub bsp_lapic_id: u32,
    pub cpu_count: u64,
    cpus: *const *const Cpu,
}

unsafe impl Send for MpResponse {}
unsafe impl Sync for MpResponse {}

#[repr(C)]
pub struct Cpu {
    pub processor_id: u32,
    pub lapic_id: u32,
    _reserved: u64,
    goto_address: AtomicPtr<()>,
    extra_argument: AtomicU64,
}
