#![no_std]
#![cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "loongarch64"
))]

use core::cell::UnsafeCell;
use core::ptr::null_mut;

mod bootloader_info;
mod bootloader_performance;

#[cfg(target_arch = "riscv64")]
mod bsp_hartid;

mod date_at_boot;
mod dtb;
mod efi_memory_map;
mod efi_system_table;
mod entry_point;
mod executable_address;
mod executable_cmdline;
mod executable_file;
mod file;
mod firmware_type;
mod flanterm;
mod framebuffer;
mod hhdm;

#[cfg(target_arch = "x86_64")]
mod keep_iommu;

mod memory_map;
mod module;
mod mp;
mod paging_mode;
mod rsdp;
mod smbios;
mod stack_size;
mod tpm;
mod tsc_frequency;

pub mod request {
    pub use crate::bootloader_info::BootloaderInfoRequest;
    pub use crate::bootloader_performance::BootloaderPerformanceRequest;

    #[cfg(target_arch = "riscv64")]
    pub use crate::bsp_hartid::BspHartidRequest;

    pub use crate::date_at_boot::DateAtBootRequest;
    pub use crate::dtb::DtbRequest;
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

    #[cfg(target_arch = "x86_64")]
    pub use crate::keep_iommu::KeepIommuRequest;

    pub use crate::memory_map::MemoryMapRequest;
    pub use crate::module::ModuleRequest;
    pub use crate::mp::MpRequest;
    pub use crate::paging_mode::PagingModeRequest;
    pub use crate::rsdp::RsdpRequest;
    pub use crate::smbios::SmbiosRequest;
    pub use crate::stack_size::StackSizeRequest;
    pub use crate::tpm::TpmEventLogRequest;
    pub use crate::tsc_frequency::TscFrequencyRequest;
}

pub mod response {
    pub use crate::bootloader_info::BootloaderInfoResponse;
    pub use crate::bootloader_performance::BootloaderPerformanceResponse;

    #[cfg(target_arch = "riscv64")]
    pub use crate::bsp_hartid::BspHartidResponse;

    pub use crate::date_at_boot::DateAtBootResponse;
    pub use crate::dtb::DtbResponse;
    pub use crate::efi_memory_map::EfiMemoryMapResponse;
    pub use crate::efi_system_table::EfiSystemTableResponse;
    pub use crate::entry_point::EntryPointResponse;
    pub use crate::executable_address::ExecutableAddressResponse;
    pub use crate::executable_cmdline::ExecutableCmdlineResponse;
    pub use crate::executable_file::ExecutableFileResponse;
    pub use crate::firmware_type::FirmwareTypeResponse;
    pub use crate::flanterm::FlantermParamsResponse;
    pub use crate::framebuffer::FramebufferResponse;
    pub use crate::hhdm::HhdmResponse;

    #[cfg(target_arch = "x86_64")]
    pub use crate::keep_iommu::KeepIommuResponse;

    pub use crate::memory_map::MemoryMapResponse;
    pub use crate::module::ModuleResponse;
    pub use crate::mp::MpResponse;
    pub use crate::paging_mode::PagingModeResponse;
    pub use crate::rsdp::RsdpResponse;
    pub use crate::smbios::SmbiosResponse;
    pub use crate::stack_size::StackSizeResponse;
    pub use crate::tpm::TpmEventLogResponse;
    pub use crate::tsc_frequency::TscFrequencyResponse;
}

pub use file::File;
pub use file::MediaType;
pub use file::Uuid;
pub use firmware_type::FirmwareType;
pub use flanterm::ParamEntry;
pub use flanterm::Rotation;
pub use framebuffer::Framebuffer;
pub use framebuffer::VideoMode;
pub use memory_map::MemoryRegion;
pub use memory_map::MemoryType;
pub use mp::Cpu;
pub use paging_mode::PagingMode;

/// The first 2 magic numbers common to every request.
pub const COMMON_MAGIC: [u64; 2] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b];

/// The currently supported base revision (6).
pub const CURRENT_BASE_REVISION: u64 = 6;

#[repr(C)]
pub struct BaseRevision {
    magic: [u64; 2],
    revision: u64,
}

unsafe impl Send for BaseRevision {}
unsafe impl Sync for BaseRevision {}

impl BaseRevision {
    pub const fn new() -> Self {
        Self {
            magic: [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc],
            revision: CURRENT_BASE_REVISION,
        }
    }

    pub const fn is_supported(&self) -> bool {
        self.revision == 0
    }
}
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
    pub id: [u64; 4],
    pub revision: u64,
    pub response: UnsafeCell<*mut T>,
}

unsafe impl<T> Send for RequestHeader<T> {}
unsafe impl<T> Sync for RequestHeader<T> {}

impl<T> RequestHeader<T> {
    pub const fn new(id: [u64; 2]) -> Self {
        Self {
            id: [COMMON_MAGIC[0], COMMON_MAGIC[1], id[0], id[1]],
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

#[cfg(test)]
pub mod bindings {
    #![allow(
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
        dead_code
    )]
    include!(concat!(env!("OUT_DIR"), "/limine.rs"));
}

#[doc(hidden)]
#[macro_export]
macro_rules! size_of_field {
    ($struct:ty, $field:ident) => {{
        fn size_of_field<F: FnOnce(T) -> U, T, U>(_f: F) -> usize {
            core::mem::size_of::<U>()
        }

        size_of_field(|s: $struct| s.$field)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout() {
        assert_eq!(core::mem::size_of::<RequestHeader<()>>(), 48);
        assert_eq!(core::mem::align_of::<RequestHeader<()>>(), 8);
        assert_eq!(core::mem::offset_of!(RequestHeader<()>, id), 0);
        assert_eq!(core::mem::offset_of!(RequestHeader<()>, revision), 32);
        assert_eq!(core::mem::offset_of!(RequestHeader<()>, response), 40);

        assert_eq!(
            core::mem::size_of::<[u64; 4]>(),
            crate::size_of_field!(RequestHeader<()>, id)
        );

        assert_eq!(
            core::mem::size_of::<u64>(),
            crate::size_of_field!(RequestHeader<()>, revision)
        );

        assert_eq!(
            core::mem::size_of::<UnsafeCell<*mut ()>>(),
            crate::size_of_field!(RequestHeader<()>, response)
        );
    }
}
