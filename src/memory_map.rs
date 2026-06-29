use crate::RequestHeader;

#[repr(u64)]
#[derive(Debug)]
pub enum EntryType {
    Usable = 0,
    Reserved = 1,
    AcpiReclaimable = 2,
    AcpiNVS = 3,
    BadMemory = 4,
    BootloaderReclaimable = 5,
    ExecutableAndModules = 6,
    Framebuffer = 7,
    ReservedMapped = 8,
}

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

#[repr(C)]
#[derive(Debug)]
pub struct MemoryMapResponse {
    revision: u64,
    entry_count: u64,
    entries: *const *const Entry,
}

unsafe impl Send for MemoryMapResponse {}
unsafe impl Sync for MemoryMapResponse {}

#[repr(C)]
#[derive(Debug)]
pub struct Entry {
    pub base: u64,
    pub length: u64,
    pub entry_type: EntryType,
}

impl MemoryMapResponse {
    pub fn entries(&self) -> &[&Entry] {
        unsafe {
            core::slice::from_raw_parts(self.entries as *const &Entry, self.entry_count as usize)
        }
    }
}
