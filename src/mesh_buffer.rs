use crate::ffi;
use crate::mesh_buffer_allocator::MeshBufferAllocator;
use crate::private::{cstring_from_str, handle_type, take_c_string};
use core::ffi::c_void;
use std::ptr;

handle_type!(MeshBuffer, "Wraps `MTKMeshBuffer`.");
handle_type!(MeshBufferZone, "Wraps `MTKMeshBufferZone`.");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
/// Mirrors `MTKMeshBufferType`.
pub enum MeshBufferType {
    /// Mirrors `MTKMeshBufferTypeVertex`.
    Vertex = 1,
    /// Mirrors `MTKMeshBufferTypeIndex`.
    Index = 2,
    /// Mirrors `MTKMeshBufferTypeCustom`.
    Custom = 3,
}

impl MeshBufferType {
    #[must_use]
    /// Converts a raw `MTKMeshBufferType` value into `MeshBufferType`.
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
    /// Returns `MTKMeshBuffer.length`.
    pub fn length(&self) -> usize {
        unsafe { ffi::mtk_mesh_buffer_length(self.as_ptr()) }
    }

    #[must_use]
    /// Returns `MTKMeshBuffer.allocator`.
    pub fn allocator(&self) -> Option<MeshBufferAllocator> {
        unsafe { MeshBufferAllocator::from_raw(ffi::mtk_mesh_buffer_allocator(self.as_ptr())) }
    }

    #[must_use]
    /// Returns `MTKMeshBuffer.zone`.
    pub fn zone(&self) -> Option<MeshBufferZone> {
        unsafe { MeshBufferZone::from_raw(ffi::mtk_mesh_buffer_zone(self.as_ptr())) }
    }

    #[must_use]
    /// Returns `MTKMeshBuffer.offset`.
    pub fn offset(&self) -> usize {
        unsafe { ffi::mtk_mesh_buffer_offset(self.as_ptr()) }
    }

    #[must_use]
    /// Returns `MTKMeshBuffer.type`.
    pub fn buffer_type(&self) -> Option<MeshBufferType> {
        MeshBufferType::from_raw(unsafe { ffi::mtk_mesh_buffer_type(self.as_ptr()) })
    }

    #[must_use]
    /// Returns the raw `MTLBuffer` pointer from `MTKMeshBuffer.buffer`.
    pub fn metal_buffer_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_mesh_buffer_metal_buffer(self.as_ptr()) }
    }

    #[must_use]
    /// Copies bytes from `MTKMeshBuffer` into the provided slice.
    pub fn copy_bytes(&self, dst: &mut [u8]) -> usize {
        let dst_ptr = if dst.is_empty() {
            ptr::null_mut()
        } else {
            dst.as_mut_ptr().cast::<c_void>()
        };
        unsafe { ffi::mtk_mesh_buffer_copy_bytes(self.as_ptr(), dst_ptr, dst.len()) }
    }

    #[must_use]
    /// Returns `MTKMeshBuffer.name`.
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { ffi::mtk_mesh_buffer_get_name(self.as_ptr()) })
    }

    /// Sets `MTKMeshBuffer.name`.
    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::mtk_mesh_buffer_set_name(self.as_ptr(), name.as_ptr()) };
        }
    }
}
