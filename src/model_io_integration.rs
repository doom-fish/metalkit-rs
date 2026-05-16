use crate::error::MetalKitError;
use crate::ffi;
use crate::mesh_buffer_allocator::MeshBufferAllocator;
use crate::private::{
    cstring_from_path, cstring_from_str, handle_type, parse_json, take_c_string, take_error,
};
use serde::Deserialize;
use std::path::Path;
use std::ptr;

handle_type!(ModelAsset);
handle_type!(ModelMesh);
handle_type!(ModelTexture);
handle_type!(MetalVertexDescriptor);
handle_type!(ModelVertexDescriptor);

pub mod model_error {
    pub const DOMAIN: &str = "MTKModelErrorDomain";
    pub const KEY: &str = "MTKModelErrorKey";
}

pub mod metal_vertex_format {
    pub const UCHAR4_NORMALIZED: usize = 14;
    pub const FLOAT2: usize = 29;
    pub const FLOAT3: usize = 30;
    pub const FLOAT4: usize = 31;
}

pub mod model_vertex_format {
    pub const UCHAR4_NORMALIZED: usize = 0x30004;
    pub const FLOAT2: usize = 0xC0002;
    pub const FLOAT3: usize = 0xC0003;
    pub const FLOAT4: usize = 0xC0004;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum GeometryType {
    Points = 0,
    Lines = 1,
    Triangles = 2,
    TriangleStrips = 3,
    Quads = 4,
    VariableTopology = 5,
}

impl GeometryType {
    #[must_use]
    pub const fn from_raw(value: i64) -> Option<Self> {
        match value {
            0 => Some(Self::Points),
            1 => Some(Self::Lines),
            2 => Some(Self::Triangles),
            3 => Some(Self::TriangleStrips),
            4 => Some(Self::Quads),
            5 => Some(Self::VariableTopology),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VertexDescriptorAttributeInfo {
    pub index: usize,
    pub format: usize,
    pub offset: usize,
    pub buffer_index: usize,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct VertexDescriptorLayoutInfo {
    pub index: usize,
    pub stride: usize,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct VertexDescriptorInfo {
    pub attributes: Vec<VertexDescriptorAttributeInfo>,
    pub layouts: Vec<VertexDescriptorLayoutInfo>,
}

impl ModelAsset {
    #[must_use]
    pub fn new(allocator: Option<&MeshBufferAllocator>) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::mtk_model_asset_new(
                allocator.map_or(ptr::null_mut(), MeshBufferAllocator::as_ptr),
            ))
        }
    }

    pub fn from_url<P: AsRef<Path>>(
        path: P,
        vertex_descriptor: Option<&ModelVertexDescriptor>,
        allocator: Option<&MeshBufferAllocator>,
        preserve_topology: bool,
    ) -> Result<Self, MetalKitError> {
        let c_path = cstring_from_path(path.as_ref())
            .ok_or_else(|| MetalKitError::new("path contains an interior NUL byte"))?;
        let mut error = ptr::null_mut();
        let asset = unsafe {
            ffi::mtk_model_asset_new_with_url(
                c_path.as_ptr(),
                vertex_descriptor.map_or(ptr::null_mut(), ModelVertexDescriptor::as_ptr),
                allocator.map_or(ptr::null_mut(), MeshBufferAllocator::as_ptr),
                preserve_topology,
                ptr::addr_of_mut!(error),
            )
        };
        if asset.is_null() {
            Err(take_error(error, "failed to create MDLAsset"))
        } else {
            Ok(unsafe { Self::from_raw_unchecked(asset) })
        }
    }

    #[must_use]
    pub fn can_import_file_extension(path_extension: &str) -> bool {
        cstring_from_str(path_extension.trim_start_matches('.')).is_some_and(|extension| unsafe {
            ffi::mtk_model_asset_can_import_file_extension(extension.as_ptr())
        })
    }

    #[must_use]
    pub fn with_meshes(allocator: Option<&MeshBufferAllocator>, meshes: &[&ModelMesh]) -> Option<Self> {
        let asset = Self::new(allocator)?;
        for mesh in meshes {
            if !asset.add_mesh(mesh) {
                return None;
            }
        }
        Some(asset)
    }

    #[must_use]
    pub fn count(&self) -> usize {
        unsafe { ffi::mtk_model_asset_count(self.as_ptr()) }
    }

    #[must_use]
    pub fn add_mesh(&self, mesh: &ModelMesh) -> bool {
        unsafe { ffi::mtk_model_asset_add_mesh(self.as_ptr(), mesh.as_ptr()) }
    }

    #[must_use]
    pub fn mesh_at(&self, index: usize) -> Option<ModelMesh> {
        unsafe { ModelMesh::from_raw(ffi::mtk_model_asset_mesh_at(self.as_ptr(), index)) }
    }

    #[must_use]
    pub fn meshes(&self) -> Vec<ModelMesh> {
        (0..self.count())
            .filter_map(|index| self.mesh_at(index))
            .collect()
    }
}

impl ModelMesh {
    pub fn new_box(
        extent: [f32; 3],
        segments: [u32; 3],
        inward_normals: bool,
        geometry_type: GeometryType,
        allocator: &MeshBufferAllocator,
    ) -> Result<Self, MetalKitError> {
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
            Err(take_error(error, "failed to create MDLMesh box"))
        } else {
            Ok(unsafe { Self::from_raw_unchecked(mesh) })
        }
    }

    /// Takes ownership of a raw `MDLMesh` pointer and wraps it as `ModelMesh`.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid `MDLMesh` object pointer that the caller is transferring
    /// to the returned wrapper.
    #[must_use]
    pub unsafe fn from_mdl_mesh_raw(ptr: *mut core::ffi::c_void) -> Option<Self> {
        Self::from_raw(ptr)
    }

    #[must_use]
    pub fn vertex_count(&self) -> usize {
        unsafe { ffi::mtk_model_mesh_vertex_count(self.as_ptr()) }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { ffi::mtk_model_mesh_get_name(self.as_ptr()) })
    }

    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::mtk_model_mesh_set_name(self.as_ptr(), name.as_ptr()) };
        }
    }

    #[must_use]
    pub const fn as_mdl_mesh_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl ModelTexture {
    pub fn from_url<P: AsRef<Path>>(path: P, name: Option<&str>) -> Result<Self, MetalKitError> {
        let c_path = cstring_from_path(path.as_ref())
            .ok_or_else(|| MetalKitError::new("path contains an interior NUL byte"))?;
        let c_name = name
            .map(|value| {
                cstring_from_str(value)
                    .ok_or_else(|| MetalKitError::new("texture name contains an interior NUL byte"))
            })
            .transpose()?;
        let mut error = ptr::null_mut();
        let texture = unsafe {
            ffi::mtk_model_texture_new_from_url(
                c_path.as_ptr(),
                c_name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                ptr::addr_of_mut!(error),
            )
        };
        if texture.is_null() {
            Err(take_error(error, "failed to create MDLTexture"))
        } else {
            Ok(unsafe { Self::from_raw_unchecked(texture) })
        }
    }
}

impl MetalVertexDescriptor {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::mtk_metal_vertex_descriptor_new()) }
    }

    #[must_use]
    pub fn set_attribute(
        &self,
        index: usize,
        format: usize,
        offset: usize,
        buffer_index: usize,
    ) -> bool {
        unsafe {
            ffi::mtk_metal_vertex_descriptor_set_attribute(
                self.as_ptr(),
                index,
                format,
                offset,
                buffer_index,
            )
        }
    }

    #[must_use]
    pub fn set_layout(&self, index: usize, stride: usize) -> bool {
        unsafe { ffi::mtk_metal_vertex_descriptor_set_layout(self.as_ptr(), index, stride) }
    }

    pub fn info(&self) -> Result<VertexDescriptorInfo, MetalKitError> {
        parse_json(
            unsafe { ffi::mtk_metal_vertex_descriptor_info_json(self.as_ptr()) },
            "MTLVertexDescriptor",
        )
    }
}

impl ModelVertexDescriptor {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::mtk_model_vertex_descriptor_new()) }
    }

    pub fn set_attribute(
        &self,
        index: usize,
        name: &str,
        format: usize,
        offset: usize,
        buffer_index: usize,
    ) -> Result<bool, MetalKitError> {
        let name = cstring_from_str(name)
            .ok_or_else(|| MetalKitError::new("attribute name contains an interior NUL byte"))?;
        Ok(unsafe {
            ffi::mtk_model_vertex_descriptor_set_attribute(
                self.as_ptr(),
                index,
                name.as_ptr(),
                format,
                offset,
                buffer_index,
            )
        })
    }

    #[must_use]
    pub fn set_layout(&self, index: usize, stride: usize) -> bool {
        unsafe { ffi::mtk_model_vertex_descriptor_set_layout(self.as_ptr(), index, stride) }
    }

    pub fn info(&self) -> Result<VertexDescriptorInfo, MetalKitError> {
        parse_json(
            unsafe { ffi::mtk_model_vertex_descriptor_info_json(self.as_ptr()) },
            "MDLVertexDescriptor",
        )
    }
}

#[must_use]
pub fn model_io_vertex_descriptor_from_metal(
    descriptor: &MetalVertexDescriptor,
) -> Option<ModelVertexDescriptor> {
    unsafe {
        ModelVertexDescriptor::from_raw(ffi::mtk_model_io_vertex_descriptor_from_metal(
            descriptor.as_ptr(),
        ))
    }
}

pub fn try_model_io_vertex_descriptor_from_metal(
    descriptor: &MetalVertexDescriptor,
) -> Result<ModelVertexDescriptor, MetalKitError> {
    let mut error = ptr::null_mut();
    let descriptor = unsafe {
        ffi::mtk_model_io_vertex_descriptor_from_metal_with_error(
            descriptor.as_ptr(),
            ptr::addr_of_mut!(error),
        )
    };
    if descriptor.is_null() {
        Err(take_error(
            error,
            "failed to convert MTLVertexDescriptor to MDLVertexDescriptor",
        ))
    } else {
        Ok(unsafe { ModelVertexDescriptor::from_raw_unchecked(descriptor) })
    }
}

#[must_use]
pub fn metal_vertex_descriptor_from_model_io(
    descriptor: &ModelVertexDescriptor,
) -> Option<MetalVertexDescriptor> {
    unsafe {
        MetalVertexDescriptor::from_raw(ffi::mtk_metal_vertex_descriptor_from_model_io(
            descriptor.as_ptr(),
        ))
    }
}

pub fn try_metal_vertex_descriptor_from_model_io(
    descriptor: &ModelVertexDescriptor,
) -> Result<MetalVertexDescriptor, MetalKitError> {
    let mut error = ptr::null_mut();
    let descriptor = unsafe {
        ffi::mtk_metal_vertex_descriptor_from_model_io_with_error(
            descriptor.as_ptr(),
            ptr::addr_of_mut!(error),
        )
    };
    if descriptor.is_null() {
        Err(take_error(
            error,
            "failed to convert MDLVertexDescriptor to MTLVertexDescriptor",
        ))
    } else {
        Ok(unsafe { MetalVertexDescriptor::from_raw_unchecked(descriptor) })
    }
}

#[must_use]
pub fn model_io_vertex_format_from_metal(vertex_format: usize) -> usize {
    unsafe { ffi::mtk_model_io_vertex_format_from_metal(vertex_format) }
}

#[must_use]
pub fn metal_vertex_format_from_model_io(vertex_format: usize) -> usize {
    unsafe { ffi::mtk_metal_vertex_format_from_model_io(vertex_format) }
}
