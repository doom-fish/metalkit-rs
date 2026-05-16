use core::ffi::c_void;
use libc::c_char;

unsafe extern "C" {
    pub fn mtk_model_asset_new(allocator: *mut c_void) -> *mut c_void;
    pub fn mtk_model_asset_new_with_url(
        path: *const c_char,
        vertex_descriptor: *mut c_void,
        allocator: *mut c_void,
        preserve_topology: bool,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_model_asset_can_import_file_extension(path_extension: *const c_char) -> bool;
    pub fn mtk_model_asset_count(asset: *mut c_void) -> usize;
    pub fn mtk_model_asset_add_mesh(asset: *mut c_void, mesh: *mut c_void) -> bool;
    pub fn mtk_model_asset_mesh_at(asset: *mut c_void, index: usize) -> *mut c_void;

    pub fn mtk_model_mesh_new_box(
        allocator: *mut c_void,
        extent_x: f32,
        extent_y: f32,
        extent_z: f32,
        segments_x: u32,
        segments_y: u32,
        segments_z: u32,
        inward_normals: bool,
        geometry_type: i64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_model_mesh_vertex_count(mesh: *mut c_void) -> usize;
    pub fn mtk_model_mesh_get_name(mesh: *mut c_void) -> *mut c_char;
    pub fn mtk_model_mesh_set_name(mesh: *mut c_void, name: *const c_char);

    pub fn mtk_model_texture_new_from_url(
        path: *const c_char,
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;

    pub fn mtk_metal_vertex_descriptor_new() -> *mut c_void;
    pub fn mtk_metal_vertex_descriptor_set_attribute(
        descriptor: *mut c_void,
        index: usize,
        format: usize,
        offset: usize,
        buffer_index: usize,
    ) -> bool;
    pub fn mtk_metal_vertex_descriptor_set_layout(
        descriptor: *mut c_void,
        index: usize,
        stride: usize,
    ) -> bool;
    pub fn mtk_metal_vertex_descriptor_info_json(descriptor: *mut c_void) -> *mut c_char;

    pub fn mtk_model_vertex_descriptor_new() -> *mut c_void;
    pub fn mtk_model_vertex_descriptor_set_attribute(
        descriptor: *mut c_void,
        index: usize,
        name: *const c_char,
        format: usize,
        offset: usize,
        buffer_index: usize,
    ) -> bool;
    pub fn mtk_model_vertex_descriptor_set_layout(
        descriptor: *mut c_void,
        index: usize,
        stride: usize,
    ) -> bool;
    pub fn mtk_model_vertex_descriptor_info_json(descriptor: *mut c_void) -> *mut c_char;

    pub fn mtk_model_io_vertex_descriptor_from_metal(descriptor: *mut c_void) -> *mut c_void;
    pub fn mtk_model_io_vertex_descriptor_from_metal_with_error(
        descriptor: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_metal_vertex_descriptor_from_model_io(descriptor: *mut c_void) -> *mut c_void;
    pub fn mtk_metal_vertex_descriptor_from_model_io_with_error(
        descriptor: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_model_io_vertex_format_from_metal(vertex_format: usize) -> usize;
    pub fn mtk_metal_vertex_format_from_model_io(vertex_format: usize) -> usize;

    pub fn mtk_texture_loader_new_texture_from_model_texture(
        loader: *mut c_void,
        texture: *mut c_void,
        options: *const super::texture_loader::TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
