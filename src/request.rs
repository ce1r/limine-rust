use core::cell::UnsafeCell;
use core::ptr::null_mut;

use crate::COMMON_MAGIC;
use crate::CURRENT_BASE_REVISION;

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
pub struct RequestHeader {
    pub magic: [u64; 2],
    pub id: [u64; 2],
    pub revision: u64,
    pub response: UnsafeCell<*mut ()>,
}

impl RequestHeader {
    pub const fn new(id: [u64; 2]) -> Self {
        Self {
            magic: COMMON_MAGIC,
            id,
            revision: CURRENT_BASE_REVISION,
            response: UnsafeCell::new(null_mut()),
        }
    }

    pub fn response<T>(&self) -> Option<&'static T> {
        let ptr = unsafe { self.response.get().read_volatile() };

        if ptr.is_null() {
            None
        } else {
            unsafe { Some(&*(ptr as *const T)) }
        }
    }
}
