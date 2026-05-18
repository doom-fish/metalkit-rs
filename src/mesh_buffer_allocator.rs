use crate::ffi;
use crate::mesh_buffer::{MeshBuffer, MeshBufferType};
use crate::private::handle_type;
use apple_metal::MetalDevice;
use core::ffi::c_void;
use std::ptr;

handle_type!(MeshBufferAllocator, "Wraps `MTKMeshBufferAllocator`.");

impl MeshBufferAllocator {
    #[must_use]
    /// Creates an `MTKMeshBufferAllocator`.
    pub fn new(device: &MetalDevice) -> Option<Self> {
        unsafe { Self::from_raw(ffi::mtk_mesh_buffer_allocator_new(device.as_ptr())) }
    }

    #[must_use]
    /// Returns the raw pointer from `MTKMeshBufferAllocator.device`.
    pub fn device_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_mesh_buffer_allocator_device(self.as_ptr()) }
    }

    #[must_use]
    /// Calls `-[MTKMeshBufferAllocator newBufferWithLength:type:]`.
    pub fn new_buffer(&self, length: usize, buffer_type: MeshBufferType) -> Option<MeshBuffer> {
        unsafe {
            MeshBuffer::from_raw(ffi::mtk_mesh_buffer_allocator_new_buffer(
                self.as_ptr(),
                length,
                buffer_type as usize,
            ))
        }
    }

    #[must_use]
    /// Calls `-[MTKMeshBufferAllocator newBufferWithData:type:]`.
    pub fn new_buffer_with_data(
        &self,
        data: &[u8],
        buffer_type: MeshBufferType,
    ) -> Option<MeshBuffer> {
        let bytes = if data.is_empty() {
            ptr::null()
        } else {
            data.as_ptr().cast::<c_void>()
        };
        unsafe {
            MeshBuffer::from_raw(ffi::mtk_mesh_buffer_allocator_new_buffer_with_data(
                self.as_ptr(),
                bytes,
                data.len(),
                buffer_type as usize,
            ))
        }
    }
}
