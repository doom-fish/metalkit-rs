use crate::ffi;
use crate::mesh_buffer_allocator::MeshBufferAllocator;
use crate::private::{cstring_from_str, handle_type, take_c_string};
use core::ffi::c_void;
use std::ptr;

handle_type!(MeshBuffer);
handle_type!(MeshBufferZone);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum MeshBufferType {
    Vertex = 1,
    Index = 2,
    Custom = 3,
}

impl MeshBufferType {
    #[must_use]
    pub const fn from_raw(value: usize) -> Option<Self> {
        match value {
            1 => Some(Self::Vertex),
            2 => Some(Self::Index),
            3 => Some(Self::Custom),
            _ => None,
        }
    }
}

impl MeshBuffer {
    #[must_use]
    pub fn length(&self) -> usize {
        unsafe { ffi::mtk_mesh_buffer_length(self.as_ptr()) }
    }

    #[must_use]
    pub fn allocator(&self) -> Option<MeshBufferAllocator> {
        unsafe { MeshBufferAllocator::from_raw(ffi::mtk_mesh_buffer_allocator(self.as_ptr())) }
    }

    #[must_use]
    pub fn zone(&self) -> Option<MeshBufferZone> {
        unsafe { MeshBufferZone::from_raw(ffi::mtk_mesh_buffer_zone(self.as_ptr())) }
    }

    #[must_use]
    pub fn offset(&self) -> usize {
        unsafe { ffi::mtk_mesh_buffer_offset(self.as_ptr()) }
    }

    #[must_use]
    pub fn buffer_type(&self) -> Option<MeshBufferType> {
        MeshBufferType::from_raw(unsafe { ffi::mtk_mesh_buffer_type(self.as_ptr()) })
    }

    #[must_use]
    pub fn metal_buffer_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_mesh_buffer_metal_buffer(self.as_ptr()) }
    }

    #[must_use]
    pub fn copy_bytes(&self, dst: &mut [u8]) -> usize {
        let dst_ptr = if dst.is_empty() {
            ptr::null_mut()
        } else {
            dst.as_mut_ptr().cast::<c_void>()
        };
        unsafe { ffi::mtk_mesh_buffer_copy_bytes(self.as_ptr(), dst_ptr, dst.len()) }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { ffi::mtk_mesh_buffer_get_name(self.as_ptr()) })
    }

    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::mtk_mesh_buffer_set_name(self.as_ptr(), name.as_ptr()) };
        }
    }
}
