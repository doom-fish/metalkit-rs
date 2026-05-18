use crate::ffi;
use crate::mesh::Mesh;
use crate::mesh_buffer::MeshBuffer;
use crate::private::{cstring_from_str, handle_type, take_c_string};

handle_type!(Submesh, "Wraps `MTKSubmesh`.");

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
    #[must_use]
    /// Converts a raw `MTLPrimitiveType` value into `PrimitiveType`.
    pub const fn from_raw(value: usize) -> Option<Self> {
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
    #[must_use]
    /// Converts a raw `MTLIndexType` value into `IndexType`.
    pub const fn from_raw(value: usize) -> Option<Self> {
        match value {
            0 => Some(Self::UInt16),
            1 => Some(Self::UInt32),
            _ => None,
        }
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
    /// Returns `MTKSubmesh.mesh`.
    pub fn mesh(&self) -> Option<Mesh> {
        unsafe { Mesh::from_raw(ffi::mtk_submesh_mesh(self.as_ptr())) }
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
