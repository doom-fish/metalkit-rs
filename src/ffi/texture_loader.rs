use core::ffi::c_void;
use libc::c_char;

pub(crate) const OPTION_GENERATE_MIPMAPS: u32 = 1 << 0;
pub(crate) const OPTION_ALLOCATE_MIPMAPS: u32 = 1 << 1;
pub(crate) const OPTION_SRGB: u32 = 1 << 2;
pub(crate) const OPTION_TEXTURE_USAGE: u32 = 1 << 3;
pub(crate) const OPTION_TEXTURE_STORAGE_MODE: u32 = 1 << 4;
pub(crate) const OPTION_TEXTURE_CPU_CACHE_MODE: u32 = 1 << 5;
pub(crate) const OPTION_CUBE_LAYOUT: u32 = 1 << 6;
pub(crate) const OPTION_ORIGIN: u32 = 1 << 7;
pub(crate) const OPTION_LOAD_AS_ARRAY: u32 = 1 << 8;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TextureLoaderOptionsRaw {
    pub flags: u32,
    pub generate_mipmaps: u8,
    pub allocate_mipmaps: u8,
    pub srgb: u8,
    pub load_as_array: u8,
    pub texture_usage: u64,
    pub texture_storage_mode: u64,
    pub texture_cpu_cache_mode: u64,
    pub cube_layout: *const c_char,
    pub origin: *const c_char,
}

unsafe extern "C" {
    pub fn mtk_texture_loader_new(device: *mut c_void) -> *mut c_void;
    pub fn mtk_texture_loader_device(loader: *mut c_void) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_from_url(
        loader: *mut c_void,
        path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_textures_from_urls(
        loader: *mut c_void,
        paths: *const *const c_char,
        count: usize,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_array_count(result: *mut c_void) -> usize;
    pub fn mtk_texture_array_texture_at(result: *mut c_void, index: usize) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_named(
        loader: *mut c_void,
        name: *const c_char,
        scale_factor: f64,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_named_with_display_gamut(
        loader: *mut c_void,
        name: *const c_char,
        scale_factor: f64,
        display_gamut: usize,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_textures_named(
        loader: *mut c_void,
        names: *const *const c_char,
        count: usize,
        scale_factor: f64,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_textures_named_with_display_gamut(
        loader: *mut c_void,
        names: *const *const c_char,
        count: usize,
        scale_factor: f64,
        display_gamut: usize,
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
}
