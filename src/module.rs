use crate::RequestHeader;
use crate::file::File;
use core::ffi::CStr;
use core::ffi::c_char;

/// Returns a [`ModuleResponse`].
#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_module_request))]
pub struct ModuleRequest {
    header: RequestHeader<ModuleResponse>,
    internal_module_count: u64,
    internal_modules: *const *const InternalModule,
}

unsafe impl Send for ModuleRequest {}
unsafe impl Sync for ModuleRequest {}

impl ModuleRequest {
    pub const fn new(modules: &'static [*const InternalModule]) -> Self {
        Self {
            header: RequestHeader::new([0x3e7e279702be32af, 0xca1c4f3bd1280cee]),
            internal_module_count: modules.len() as u64,
            internal_modules: modules.as_ptr(),
        }
    }

    pub fn response(&self) -> Option<&'static ModuleResponse> {
        self.header.response()
    }
}

/// Returned by [`ModuleRequest`].
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, limine_test::test_layout(limine_module_response))]
pub struct ModuleResponse {
    revision: u64,
    module_count: u64,
    modules: *const *const File,
}

unsafe impl Send for ModuleResponse {}
unsafe impl Sync for ModuleResponse {}

#[repr(C, align(8))]
#[cfg_attr(test, limine_test::test_layout(limine_internal_module))]
pub struct InternalModule {
    path: *const c_char,
    string: *const c_char,
    flags: u64,
}

unsafe impl Send for InternalModule {}
unsafe impl Sync for InternalModule {}

impl InternalModule {
    pub const fn new(path: &CStr, string: &CStr, flags: u64) -> Self {
        Self {
            path: path.as_ptr(),
            string: string.as_ptr(),
            flags,
        }
    }
}
