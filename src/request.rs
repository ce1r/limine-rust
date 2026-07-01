use crate::COMMON_MAGIC;
use crate::CURRENT_BASE_REVISION;
use core::cell::UnsafeCell;
use core::ptr::null_mut;

pub use crate::bootloader_info::BootloaderInfoRequest;
pub use crate::bootloader_performance::BootloaderPerformanceRequest;
pub use crate::date_at_boot::DateAtBootRequest;
pub use crate::efi_memory_map::EfiMemoryMapRequest;
pub use crate::efi_system_table::EfiSystemTableRequest;
pub use crate::entry_point::EntryPointRequest;
pub use crate::executable_address::ExecutableAddressRequest;
pub use crate::executable_cmdline::ExecutableCmdlineRequest;
pub use crate::executable_file::ExecutableFileRequest;
pub use crate::firmware_type::FirmwareTypeRequest;
pub use crate::flanterm::FlantermParamsRequest;
pub use crate::framebuffer::FramebufferRequest;
pub use crate::hhdm::HhdmRequest;
pub use crate::keep_iommu::KeepIommuRequest;
pub use crate::memory_map::MemoryMapRequest;
pub use crate::mp::MpRequest;
pub use crate::paging_mode::PagingModeRequest;
pub use crate::rsdp::RsdpRequest;
pub use crate::smbios::SmbiosRequest;
pub use crate::stack_size::StackSizeRequest;
pub use crate::tsc_frequency::TscFrequencyRequest;

/// Marks the beginning of requests.
#[repr(C)]
pub struct RequestsStartMarker([u64; 4]);

unsafe impl Send for RequestsStartMarker {}
unsafe impl Sync for RequestsStartMarker {}

impl RequestsStartMarker {
    pub const fn new() -> Self {
        Self([
            0xf6b8f4b39de7d1ae,
            0xfab91a6940fcb9cf,
            0x785c6ed015d3e316,
            0x181e920a7852b9d9,
        ])
    }
}

/// Marks the ending of requests.
#[repr(C)]
pub struct RequestsEndMarker([u64; 2]);

unsafe impl Send for RequestsEndMarker {}
unsafe impl Sync for RequestsEndMarker {}

impl RequestsEndMarker {
    pub const fn new() -> Self {
        Self([0xadc0e0531bb10d03, 0x9572709f31764c62])
    }
}

#[repr(C, align(8))]
pub(crate) struct RequestHeader<T> {
    pub magic: [u64; 2],
    pub id: [u64; 2],
    pub revision: u64,
    pub response: UnsafeCell<*mut T>,
}

unsafe impl<T> Send for RequestHeader<T> {}
unsafe impl<T> Sync for RequestHeader<T> {}

impl<T> RequestHeader<T> {
    pub const fn new(id: [u64; 2]) -> Self {
        Self {
            magic: COMMON_MAGIC,
            id,
            revision: CURRENT_BASE_REVISION,
            response: UnsafeCell::new(null_mut()),
        }
    }

    pub fn response(&self) -> Option<&'static T> {
        let ptr = unsafe { self.response.get().read_volatile() };

        if ptr.is_null() {
            None
        } else {
            unsafe { Some(&*(ptr.cast_const())) }
        }
    }
}
