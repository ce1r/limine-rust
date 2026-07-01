use crate::request::RequestHeader;

/// The type of a memory region returned by the bootloader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNVS,
    BadMemory,
    BootloaderReclaimable,
    ExecutableAndModules,
    Framebuffer,
    ReservedMapped,
    Unknown,
}

/// Returns a [`MemoryMapResponse`].
#[repr(C, align(8))]
pub struct MemoryMapRequest {
    header: RequestHeader<MemoryMapResponse>,
}

unsafe impl Send for MemoryMapRequest {}
unsafe impl Sync for MemoryMapRequest {}

impl MemoryMapRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0x67cf3d9d378a806f, 0xe304acdfc50c3c62]),
        }
    }

    pub fn response(&self) -> Option<&'static MemoryMapResponse> {
        self.header.response()
    }
}

/// Returned by [`MemoryMapRequest`].
#[repr(C)]
#[derive(Debug)]
pub struct MemoryMapResponse {
    revision: u64,
    region_count: u64,
    regions: *const *const MemoryRegion,
}

unsafe impl Send for MemoryMapResponse {}
unsafe impl Sync for MemoryMapResponse {}

impl MemoryMapResponse {
    /// Returns a slice of all the memory regions.
    pub fn regions(&self) -> &[&MemoryRegion] {
        unsafe { core::slice::from_raw_parts(self.regions.cast(), self.region_count as usize) }
    }
}

/// A region of physical memory.
#[repr(C)]
#[derive(Debug)]
pub struct MemoryRegion {
    start: u64,
    size: u64,
    memory_type: u64,
}

impl MemoryRegion {
    /// Returns the starting physical address of the memory region.
    pub fn start(&self) -> u64 {
        self.start
    }

    /// Returns the ending physical address (exclusive) of the memory region.
    pub fn end(&self) -> u64 {
        self.start + self.size
    }

    /// Returns the size of the memory region in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Returns the [`MemoryType`] of the region.
    pub fn memory_type(&self) -> MemoryType {
        match self.memory_type {
            0 => MemoryType::Usable,
            1 => MemoryType::Reserved,
            2 => MemoryType::AcpiReclaimable,
            3 => MemoryType::AcpiNVS,
            4 => MemoryType::BadMemory,
            5 => MemoryType::BootloaderReclaimable,
            6 => MemoryType::ExecutableAndModules,
            7 => MemoryType::Framebuffer,
            8 => MemoryType::ReservedMapped,
            _ => MemoryType::Unknown,
        }
    }

    /// Returns whether the memory region is [`MemoryType::Usable`].
    pub fn is_usable(&self) -> bool {
        self.memory_type() == MemoryType::Usable
    }
}
