use crate::file::File;
use crate::request::RequestHeader;

/// Returns a [`ExecutableFileResponse`].
#[repr(C, align(8))]
pub struct ExecutableFileRequest {
    header: RequestHeader<ExecutableFileResponse>,
}

unsafe impl Send for ExecutableFileRequest {}
unsafe impl Sync for ExecutableFileRequest {}

impl ExecutableFileRequest {
    pub const fn new() -> Self {
        Self {
            header: RequestHeader::new([0xad97e90e83f1ed67, 0x31eb5d1c5ff23b69]),
        }
    }

    pub fn response(&self) -> Option<&'static ExecutableFileResponse> {
        self.header.response()
    }
}

/// Returned by [`ExecutableFileRequest`].
#[repr(C)]
#[derive(Debug)]
pub struct ExecutableFileResponse {
    revision: u64,
    executable_file: *const File,
}

unsafe impl Send for ExecutableFileResponse {}
unsafe impl Sync for ExecutableFileResponse {}

impl ExecutableFileResponse {
    pub fn executable_file(&self) -> &File {
        unsafe { &*self.executable_file }
    }
}
