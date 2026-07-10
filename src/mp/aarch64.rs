use core::sync::atomic::AtomicPtr;
use core::sync::atomic::AtomicU64;

/// Returned by [`MpRequest`].
#[derive(Debug)]
#[repr(C)]
#[cfg_attr(test, limine_test::test_layout(limine_mp_response))]
pub struct MpResponse {
    revision: u64,
    flags: u64,
    pub bsp_mpidr: u64,
    cpu_count: u64,
    cpus: *const *const Cpu,
}

unsafe impl Send for MpResponse {}
unsafe impl Sync for MpResponse {}

impl MpResponse {
    pub const fn cpus(&self) -> &[&Cpu] {
        unsafe { core::slice::from_raw_parts(self.cpus.cast(), self.cpu_count as usize) }
    }
}

#[repr(C)]
#[cfg_attr(test, limine_test::test_layout(limine_mp_info))]
pub struct Cpu {
    pub processor_id: u32,
    reserved1: u32,
    pub mpidr: u64,
    reserved: u64,
    goto_address: AtomicPtr<()>,
    extra_argument: AtomicU64,
}
