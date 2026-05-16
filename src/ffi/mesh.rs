use core::ffi::c_void;
use libc::c_char;

unsafe extern "C" {
    pub fn mtk_mesh_new_from_model_mesh(
        mesh: *mut c_void,
        device: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_mesh_new_meshes_from_asset(
        asset: *mut c_void,
        device: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_meshes_from_asset_result_mesh_count(result: *mut c_void) -> usize;
    pub fn mtk_meshes_from_asset_result_mesh_at(result: *mut c_void, index: usize) -> *mut c_void;
    pub fn mtk_meshes_from_asset_result_source_mesh_count(result: *mut c_void) -> usize;
    pub fn mtk_meshes_from_asset_result_source_mesh_at(
        result: *mut c_void,
        index: usize,
    ) -> *mut c_void;
    pub fn mtk_mesh_vertex_count(mesh: *mut c_void) -> usize;
    pub fn mtk_mesh_get_name(mesh: *mut c_void) -> *mut c_char;
    pub fn mtk_mesh_set_name(mesh: *mut c_void, name: *const c_char);
    pub fn mtk_mesh_vertex_buffer_count(mesh: *mut c_void) -> usize;
    pub fn mtk_mesh_vertex_buffer_at(mesh: *mut c_void, index: usize) -> *mut c_void;
    pub fn mtk_mesh_vertex_descriptor(mesh: *mut c_void) -> *mut c_void;
    pub fn mtk_mesh_submesh_count(mesh: *mut c_void) -> usize;
    pub fn mtk_mesh_submesh_at(mesh: *mut c_void, index: usize) -> *mut c_void;
}
