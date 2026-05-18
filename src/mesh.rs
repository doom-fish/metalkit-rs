use crate::error::MetalKitError;
use crate::ffi;
use crate::mesh_buffer::MeshBuffer;
use crate::model_io_integration::{ModelAsset, ModelMesh, ModelVertexDescriptor};
use crate::private::{cstring_from_str, handle_type, take_c_string, take_error};
use crate::submesh::Submesh;
use apple_metal::MetalDevice;
use std::ptr;

handle_type!(Mesh, "Wraps `MTKMesh`.");

#[derive(Debug, Clone)]
/// Mirrors the paired `MTKMesh` and `MDLMesh` arrays returned by asset conversion APIs.
pub struct MeshAssetConversion {
    /// Contains the converted `MTKMesh` values.
    pub meshes: Vec<Mesh>,
    /// Contains the source `MDLMesh` values paired with `meshes`.
    pub source_meshes: Vec<ModelMesh>,
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
            Err(take_error(error, "failed to create MTKMesh"))
        } else {
            Ok(unsafe { Self::from_raw_unchecked(raw_mesh) })
        }
    }

    /// Converts every mesh in an `MDLAsset` into `MTKMesh` values.
    pub fn new_meshes_from_asset(
        asset: &ModelAsset,
        device: &MetalDevice,
    ) -> Result<MeshAssetConversion, MetalKitError> {
        let mut error = ptr::null_mut();
        let result = unsafe {
            ffi::mtk_mesh_new_meshes_from_asset(
                asset.as_ptr(),
                device.as_ptr(),
                ptr::addr_of_mut!(error),
            )
        };
        if result.is_null() {
            return Err(take_error(
                error,
                "failed to create MTKMesh objects from MDLAsset",
            ));
        }

        let mesh_count = unsafe { ffi::mtk_meshes_from_asset_result_mesh_count(result) };
        let source_mesh_count =
            unsafe { ffi::mtk_meshes_from_asset_result_source_mesh_count(result) };
        let meshes = (0..mesh_count)
            .filter_map(|index| unsafe {
                Mesh::from_raw(ffi::mtk_meshes_from_asset_result_mesh_at(result, index))
            })
            .collect();
        let source_meshes = (0..source_mesh_count)
            .filter_map(|index| unsafe {
                ModelMesh::from_raw(ffi::mtk_meshes_from_asset_result_source_mesh_at(
                    result, index,
                ))
            })
            .collect();
        unsafe { ffi::mtk_release(result) };
        Ok(MeshAssetConversion {
            meshes,
            source_meshes,
        })
    }

    /// Wraps a borrowed `MDLMesh` pointer by converting it into a new `MTKMesh`.
    ///
    /// # Safety
    ///
    /// `mesh` must be a valid, non-null `MDLMesh` pointer that remains alive for the
    /// duration of this call.
    pub unsafe fn from_mdl_mesh_raw(
        mesh: *mut core::ffi::c_void,
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
    /// Returns `MTKMesh.vertexDescriptor`.
    pub fn vertex_descriptor(&self) -> Option<ModelVertexDescriptor> {
        unsafe { ModelVertexDescriptor::from_raw(ffi::mtk_mesh_vertex_descriptor(self.as_ptr())) }
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
