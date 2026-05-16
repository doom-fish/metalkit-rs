use crate::ffi;
use crate::mesh::Mesh;
use crate::mesh_buffer::MeshBuffer;
use crate::private::{cstring_from_str, handle_type, take_c_string};

handle_type!(Submesh);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum PrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

impl PrimitiveType {
    #[must_use]
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
pub enum IndexType {
    UInt16 = 0,
    UInt32 = 1,
}

impl IndexType {
    #[must_use]
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
    pub fn primitive_type(&self) -> Option<PrimitiveType> {
        PrimitiveType::from_raw(unsafe { ffi::mtk_submesh_primitive_type(self.as_ptr()) })
    }

    #[must_use]
    pub fn index_type(&self) -> Option<IndexType> {
        IndexType::from_raw(unsafe { ffi::mtk_submesh_index_type(self.as_ptr()) })
    }

    #[must_use]
    pub fn index_buffer(&self) -> Option<MeshBuffer> {
        unsafe { MeshBuffer::from_raw(ffi::mtk_submesh_index_buffer(self.as_ptr())) }
    }

    #[must_use]
    pub fn index_count(&self) -> usize {
        unsafe { ffi::mtk_submesh_index_count(self.as_ptr()) }
    }

    #[must_use]
    pub fn mesh(&self) -> Option<Mesh> {
        unsafe { Mesh::from_raw(ffi::mtk_submesh_mesh(self.as_ptr())) }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { ffi::mtk_submesh_get_name(self.as_ptr()) })
    }

    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::mtk_submesh_set_name(self.as_ptr(), name.as_ptr()) };
        }
    }
}
