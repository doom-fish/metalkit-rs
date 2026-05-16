use core::ffi::c_void;
use libc::c_char;

pub(crate) const OPTION_GENERATE_MIPMAPS: u32 = 1 << 0;
pub(crate) const OPTION_ALLOCATE_MIPMAPS: u32 = 1 << 1;
pub(crate) const OPTION_SRGB: u32 = 1 << 2;
pub(crate) const OPTION_TEXTURE_USAGE: u32 = 1 << 3;
pub(crate) const OPTION_TEXTURE_STORAGE_MODE: u32 = 1 << 4;
pub(crate) const OPTION_TEXTURE_CPU_CACHE_MODE: u32 = 1 << 5;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TextureLoaderOptionsRaw {
    pub flags: u32,
    pub generate_mipmaps: u8,
    pub allocate_mipmaps: u8,
    pub srgb: u8,
    pub reserved: u8,
    pub texture_usage: u64,
    pub texture_storage_mode: u64,
    pub texture_cpu_cache_mode: u64,
}

unsafe extern "C" {
    pub fn mtk_retain(ptr: *mut c_void) -> *mut c_void;
    pub fn mtk_release(ptr: *mut c_void);

    pub fn mtk_texture_loader_new(device: *mut c_void) -> *mut c_void;
    pub fn mtk_texture_loader_device(loader: *mut c_void) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_from_url(
        loader: *mut c_void,
        path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_named(
        loader: *mut c_void,
        name: *const c_char,
        scale_factor: f64,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_from_data(
        loader: *mut c_void,
        bytes: *const c_void,
        len: usize,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_from_cgimage(
        loader: *mut c_void,
        image: *mut c_void,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;

    pub fn mtk_mesh_buffer_allocator_new(device: *mut c_void) -> *mut c_void;
    pub fn mtk_mesh_buffer_allocator_device(allocator: *mut c_void) -> *mut c_void;
    pub fn mtk_mesh_buffer_allocator_new_buffer(
        allocator: *mut c_void,
        length: usize,
        buffer_type: usize,
    ) -> *mut c_void;
    pub fn mtk_mesh_buffer_allocator_new_buffer_with_data(
        allocator: *mut c_void,
        bytes: *const c_void,
        len: usize,
        buffer_type: usize,
    ) -> *mut c_void;

    pub fn mtk_mesh_buffer_length(buffer: *mut c_void) -> usize;
    pub fn mtk_mesh_buffer_offset(buffer: *mut c_void) -> usize;
    pub fn mtk_mesh_buffer_type(buffer: *mut c_void) -> usize;
    pub fn mtk_mesh_buffer_metal_buffer(buffer: *mut c_void) -> *mut c_void;
    pub fn mtk_mesh_buffer_copy_bytes(buffer: *mut c_void, dst: *mut c_void, len: usize) -> usize;
    pub fn mtk_mesh_buffer_get_name(buffer: *mut c_void) -> *mut c_char;
    pub fn mtk_mesh_buffer_set_name(buffer: *mut c_void, name: *const c_char);

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

    pub fn mtk_mesh_new_from_model_mesh(
        mesh: *mut c_void,
        device: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_mesh_vertex_count(mesh: *mut c_void) -> usize;
    pub fn mtk_mesh_get_name(mesh: *mut c_void) -> *mut c_char;
    pub fn mtk_mesh_set_name(mesh: *mut c_void, name: *const c_char);
    pub fn mtk_mesh_vertex_buffer_count(mesh: *mut c_void) -> usize;
    pub fn mtk_mesh_vertex_buffer_at(mesh: *mut c_void, index: usize) -> *mut c_void;
    pub fn mtk_mesh_submesh_count(mesh: *mut c_void) -> usize;
    pub fn mtk_mesh_submesh_at(mesh: *mut c_void, index: usize) -> *mut c_void;

    pub fn mtk_submesh_primitive_type(submesh: *mut c_void) -> usize;
    pub fn mtk_submesh_index_type(submesh: *mut c_void) -> usize;
    pub fn mtk_submesh_index_buffer(submesh: *mut c_void) -> *mut c_void;
    pub fn mtk_submesh_index_count(submesh: *mut c_void) -> usize;
    pub fn mtk_submesh_get_name(submesh: *mut c_void) -> *mut c_char;
    pub fn mtk_submesh_set_name(submesh: *mut c_void, name: *const c_char);
}
