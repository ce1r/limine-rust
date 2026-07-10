use core::sync::atomic::AtomicPtr;
use core::sync::atomic::AtomicU64;

/// Returned by [`MpRequest`].
#[derive(Debug)]
#[repr(C)]
#[cfg_attr(test, limine_test::test_layout(limine_mp_response))]
pub struct MpResponse {
    revision: u64,
    flags: u32,

    /// The Local APIC ID of the bootstrap processor (BSP).
    pub bsp_lapic_id: u32,

    cpu_count: u64,
    cpus: *const *const Cpu,
}

unsafe impl Send for MpResponse {}
unsafe impl Sync for MpResponse {}

impl MpResponse {
    pub const fn cpus(&self) -> &[&Cpu] {
        unsafe { core::slice::from_raw_parts(self.cpus.cast(), self.cpu_count as usize) }
    }

    /// Returns whether X2APIC has been enabled.
    pub const fn is_x2apic_enabled(&self) -> bool {
        self.flags == 1
    }
}

#[repr(C)]
#[cfg_attr(test, limine_test::test_layout(limine_mp_info))]
pub struct Cpu {
    pub processor_id: u32,
    pub lapic_id: u32,
    reserved: u64,
    goto_address: AtomicPtr<()>,
    extra_argument: AtomicU64,
}
