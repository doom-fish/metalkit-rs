use crate::error::MetalKitError;
use crate::ffi;
use crate::private::{cstring_from_str, handle_type};
use apple_metal::MetalDevice;
use core::ffi::c_void;
use std::ffi::CStr;
use std::ptr;

handle_type!(MeshBufferAllocator, "Wraps `MTKMeshBufferAllocator`.");
handle_type!(MeshBuffer, "Wraps `MTKMeshBuffer`.");
handle_type!(ModelMesh, "Wraps `MDLMesh`.");
handle_type!(Mesh, "Wraps `MTKMesh`.");
handle_type!(Submesh, "Wraps `MTKSubmesh`.");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
/// Mirrors `MDLGeometryType`.
pub enum GeometryType {
    /// Mirrors `MDLGeometryTypePoints`.
    Points = 0,
    /// Mirrors `MDLGeometryTypeLines`.
    Lines = 1,
    /// Mirrors `MDLGeometryTypeTriangles`.
    Triangles = 2,
    /// Mirrors `MDLGeometryTypeTriangleStrips`.
    TriangleStrips = 3,
    /// Mirrors `MDLGeometryTypeQuads`.
    Quads = 4,
    /// Mirrors `MDLGeometryTypeVariableTopology`.
    VariableTopology = 5,
}

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
    const fn from_raw(value: usize) -> Option<Self> {
        match value {
            1 => Some(Self::Vertex),
            2 => Some(Self::Index),
            3 => Some(Self::Custom),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
/// Mirrors `MTLPrimitiveType` as exposed by `MTKSubmesh`.
pub enum PrimitiveType {
    /// Mirrors `MTLPrimitiveTypePoint`.
    Point = 0,
    /// Mirrors `MTLPrimitiveTypeLine`.
    Line = 1,
    /// Mirrors `MTLPrimitiveTypeLineStrip`.
    LineStrip = 2,
    /// Mirrors `MTLPrimitiveTypeTriangle`.
    Triangle = 3,
    /// Mirrors `MTLPrimitiveTypeTriangleStrip`.
    TriangleStrip = 4,
}

impl PrimitiveType {
    const fn from_raw(value: usize) -> Option<Self> {
        match value {
            0 => Some(Self::Point),
            1 => Some(Self::Line),
            2 => Some(Self::LineStrip),
            3 => Some(Self::Triangle),
            4 => Some(Self::TriangleStrip),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
/// Mirrors `MTLIndexType` as exposed by `MTKSubmesh`.
pub enum IndexType {
    /// Mirrors `MTLIndexTypeUInt16`.
    UInt16 = 0,
    /// Mirrors `MTLIndexTypeUInt32`.
    UInt32 = 1,
}

impl IndexType {
    const fn from_raw(value: usize) -> Option<Self> {
        match value {
            0 => Some(Self::UInt16),
            1 => Some(Self::UInt32),
            _ => None,
        }
    }
}

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

impl MeshBuffer {
    #[must_use]
    /// Returns `MTKMeshBuffer.length`.
    pub fn length(&self) -> usize {
        unsafe { ffi::mtk_mesh_buffer_length(self.as_ptr()) }
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

impl ModelMesh {
    /// Creates an `MDLMesh` box with MetalKit-compatible allocation.
    pub fn new_box(
        extent: [f32; 3],
        segments: [u32; 3],
        inward_normals: bool,
        geometry_type: GeometryType,
        allocator: &MeshBufferAllocator,
    ) -> Result<Self, MetalKitError> {
        if !matches!(
            geometry_type,
            GeometryType::Lines | GeometryType::Triangles | GeometryType::Quads
        ) {
            return Err(MetalKitError::new(
                "ModelMesh::new_box only supports GeometryType::Lines, ::Triangles, or ::Quads",
            ));
        }

        let mut error = ptr::null_mut();
        let mesh = unsafe {
            ffi::mtk_model_mesh_new_box(
                allocator.as_ptr(),
                extent[0],
                extent[1],
                extent[2],
                segments[0],
                segments[1],
                segments[2],
                inward_normals,
                geometry_type as i64,
                ptr::addr_of_mut!(error),
            )
        };

        if mesh.is_null() {
            Err(take_error(error)
                .unwrap_or_else(|| MetalKitError::new("failed to create MDLMesh box")))
        } else {
            Ok(unsafe { Self::from_raw_unchecked(mesh) })
        }
    }

    /// Wrap a raw `MDLMesh *` pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid `MDLMesh *` whose ownership is being transferred to the returned value.
    #[must_use]
    pub unsafe fn from_mdl_mesh_raw(ptr: *mut c_void) -> Option<Self> {
        Self::from_raw(ptr)
    }

    #[must_use]
    /// Returns `MDLMesh.vertexCount`.
    pub fn vertex_count(&self) -> usize {
        unsafe { ffi::mtk_model_mesh_vertex_count(self.as_ptr()) }
    }

    #[must_use]
    /// Returns `MDLMesh.name`.
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { ffi::mtk_model_mesh_get_name(self.as_ptr()) })
    }

    /// Sets `MDLMesh.name`.
    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::mtk_model_mesh_set_name(self.as_ptr(), name.as_ptr()) };
        }
    }

    #[must_use]
    /// Returns the wrapped raw `MDLMesh` pointer.
    pub const fn as_mdl_mesh_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Mesh {
    /// Creates an `MTKMesh` from an `MDLMesh`.
    pub fn from_model_mesh(mesh: &ModelMesh, device: &MetalDevice) -> Result<Self, MetalKitError> {
        let mut error = ptr::null_mut();
        let raw_mesh = unsafe {
            ffi::mtk_mesh_new_from_model_mesh(
                mesh.as_ptr(),
                device.as_ptr(),
                ptr::addr_of_mut!(error),
            )
        };
        if raw_mesh.is_null() {
            Err(take_error(error).unwrap_or_else(|| MetalKitError::new("failed to create MTKMesh")))
        } else {
            Ok(unsafe { Self::from_raw_unchecked(raw_mesh) })
        }
    }

    /// Wrap a raw `MDLMesh *` pointer and create an `MTKMesh` from it.
    ///
    /// # Safety
    ///
    /// `mesh` must point to a valid `MDLMesh` object that remains alive for the duration of the call.
    pub unsafe fn from_mdl_mesh_raw(
        mesh: *mut c_void,
        device: &MetalDevice,
    ) -> Result<Self, MetalKitError> {
        if mesh.is_null() {
            return Err(MetalKitError::new("MDLMesh pointer was null"));
        }
        let model_mesh = ModelMesh::from_raw_borrowed(mesh);
        Self::from_model_mesh(&model_mesh, device)
    }

    #[must_use]
    /// Returns `MTKMesh.vertexCount`.
    pub fn vertex_count(&self) -> usize {
        unsafe { ffi::mtk_mesh_vertex_count(self.as_ptr()) }
    }

    #[must_use]
    /// Returns `MTKMesh.name`.
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { ffi::mtk_mesh_get_name(self.as_ptr()) })
    }

    /// Sets `MTKMesh.name`.
    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::mtk_mesh_set_name(self.as_ptr(), name.as_ptr()) };
        }
    }

    #[must_use]
    /// Returns `MTKMesh.vertexBuffers`.
    pub fn vertex_buffers(&self) -> Vec<MeshBuffer> {
        let count = unsafe { ffi::mtk_mesh_vertex_buffer_count(self.as_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                MeshBuffer::from_raw(ffi::mtk_mesh_vertex_buffer_at(self.as_ptr(), index))
            })
            .collect()
    }

    #[must_use]
    /// Returns `MTKMesh.submeshes`.
    pub fn submeshes(&self) -> Vec<Submesh> {
        let count = unsafe { ffi::mtk_mesh_submesh_count(self.as_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                Submesh::from_raw(ffi::mtk_mesh_submesh_at(self.as_ptr(), index))
            })
            .collect()
    }
}

impl Submesh {
    #[must_use]
    /// Returns `MTKSubmesh.primitiveType`.
    pub fn primitive_type(&self) -> Option<PrimitiveType> {
        PrimitiveType::from_raw(unsafe { ffi::mtk_submesh_primitive_type(self.as_ptr()) })
    }

    #[must_use]
    /// Returns `MTKSubmesh.indexType`.
    pub fn index_type(&self) -> Option<IndexType> {
        IndexType::from_raw(unsafe { ffi::mtk_submesh_index_type(self.as_ptr()) })
    }

    #[must_use]
    /// Returns `MTKSubmesh.indexBuffer`.
    pub fn index_buffer(&self) -> Option<MeshBuffer> {
        unsafe { MeshBuffer::from_raw(ffi::mtk_submesh_index_buffer(self.as_ptr())) }
    }

    #[must_use]
    /// Returns `MTKSubmesh.indexCount`.
    pub fn index_count(&self) -> usize {
        unsafe { ffi::mtk_submesh_index_count(self.as_ptr()) }
    }

    #[must_use]
    /// Returns `MTKSubmesh.name`.
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { ffi::mtk_submesh_get_name(self.as_ptr()) })
    }

    /// Sets `MTKSubmesh.name`.
    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::mtk_submesh_set_name(self.as_ptr(), name.as_ptr()) };
        }
    }
}

fn take_c_string(ptr: *mut libc::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let value = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { libc::free(ptr.cast()) };
    Some(value)
}

fn take_error(ptr: *mut libc::c_char) -> Option<MetalKitError> {
    take_c_string(ptr).map(MetalKitError::new)
}
