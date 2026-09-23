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
#[derive(Debug, Clone, Copy)]
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

impl Default for TextureLoaderOptionsRaw {
    fn default() -> Self {
        Self {
            flags: 0,
            generate_mipmaps: 0,
            allocate_mipmaps: 0,
            srgb: 0,
            load_as_array: 0,
            texture_usage: 0,
            texture_storage_mode: 0,
            texture_cpu_cache_mode: 0,
            cube_layout: core::ptr::null(),
            origin: core::ptr::null(),
        }
    }
}

// `TextureLoaderOptionsRaw` is passed by pointer across the Rust <-> Swift
// `@_cdecl` FFI boundary (Swift reads it as `TextureLoaderOptionsRaw` via
// `assumingMemoryBound`). If its size or alignment drifts from what the Swift
// side expects, the option marshalling silently corrupts. These compile-time
// assertions pin the layout so accidental field reordering / type changes fail
// the build immediately. They pin size and alignment only. The cross-language
// `mtk_verify_ffi_layout` check guards the Swift side agreeing with these values.
const _: () = assert!(core::mem::size_of::<TextureLoaderOptionsRaw>() == 48);
const _: () = assert!(core::mem::align_of::<TextureLoaderOptionsRaw>() == 8);

pub type TextureLoaderCallback =
    unsafe extern "C" fn(user_data: *mut c_void, texture: *mut c_void, error: *mut c_char);
pub type TextureLoaderArrayCallback =
    unsafe extern "C" fn(user_data: *mut c_void, result: *mut c_void, error: *mut c_char);

unsafe extern "C" {
    /// Cross-language ABI check implemented in the Swift bridge.
    ///
    /// Returns `true` only if the Swift `MemoryLayout` (size, stride and
    /// alignment) of `TextureLoaderOptionsRaw` matches the values pinned on
    /// the Rust side. A `false` return means the Rust and Swift layouts
    /// genuinely disagree, which is a real ABI bug.
    ///
    /// Only referenced from the `ffi_layout_matches_swift` test; the
    /// compile-time `const _` asserts above cover non-test builds.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn mtk_verify_ffi_layout() -> bool;
    pub fn mtk_texture_loader_new(device: *mut c_void) -> *mut c_void;
    pub fn mtk_texture_loader_device(loader: *mut c_void) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_from_url(
        loader: *mut c_void,
        path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_from_url_with_callback(
        loader: *mut c_void,
        path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        callback: Option<TextureLoaderCallback>,
        user_data: *mut c_void,
    );
    pub fn mtk_texture_loader_new_textures_from_urls(
        loader: *mut c_void,
        paths: *const *const c_char,
        count: usize,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_textures_from_urls_with_callback(
        loader: *mut c_void,
        paths: *const *const c_char,
        count: usize,
        options: *const TextureLoaderOptionsRaw,
        callback: Option<TextureLoaderArrayCallback>,
        user_data: *mut c_void,
    );
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
    pub fn mtk_texture_loader_new_texture_named_with_callback(
        loader: *mut c_void,
        name: *const c_char,
        scale_factor: f64,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        callback: Option<TextureLoaderCallback>,
        user_data: *mut c_void,
    );
    pub fn mtk_texture_loader_new_texture_named_with_display_gamut(
        loader: *mut c_void,
        name: *const c_char,
        scale_factor: f64,
        display_gamut: usize,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_named_with_display_gamut_with_callback(
        loader: *mut c_void,
        name: *const c_char,
        scale_factor: f64,
        display_gamut: usize,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        callback: Option<TextureLoaderCallback>,
        user_data: *mut c_void,
    );
    pub fn mtk_texture_loader_new_textures_named(
        loader: *mut c_void,
        names: *const *const c_char,
        count: usize,
        scale_factor: f64,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_textures_named_with_callback(
        loader: *mut c_void,
        names: *const *const c_char,
        count: usize,
        scale_factor: f64,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        callback: Option<TextureLoaderArrayCallback>,
        user_data: *mut c_void,
    );
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
    pub fn mtk_texture_loader_new_textures_named_with_display_gamut_with_callback(
        loader: *mut c_void,
        names: *const *const c_char,
        count: usize,
        scale_factor: f64,
        display_gamut: usize,
        bundle_path: *const c_char,
        options: *const TextureLoaderOptionsRaw,
        callback: Option<TextureLoaderArrayCallback>,
        user_data: *mut c_void,
    );
    pub fn mtk_texture_loader_new_texture_from_data(
        loader: *mut c_void,
        bytes: *const c_void,
        len: usize,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_from_data_with_callback(
        loader: *mut c_void,
        bytes: *const c_void,
        len: usize,
        options: *const TextureLoaderOptionsRaw,
        callback: Option<TextureLoaderCallback>,
        user_data: *mut c_void,
    );
    pub fn mtk_texture_loader_new_texture_from_cgimage(
        loader: *mut c_void,
        image: *mut c_void,
        options: *const TextureLoaderOptionsRaw,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_texture_loader_new_texture_from_cgimage_with_callback(
        loader: *mut c_void,
        image: *mut c_void,
        options: *const TextureLoaderOptionsRaw,
        callback: Option<TextureLoaderCallback>,
        user_data: *mut c_void,
    );
}

#[cfg(test)]
mod tests {
    use super::{mtk_verify_ffi_layout, TextureLoaderOptionsRaw};
    use core::mem::{align_of, size_of};

    #[test]
    fn texture_loader_options_raw_layout() {
        // u32 + 4 x u8 (8), then three 8-byte-aligned u64 (24) plus two
        // pointers (16) => 48 bytes, 8-byte aligned.
        assert_eq!(
            size_of::<TextureLoaderOptionsRaw>(),
            48,
            "TextureLoaderOptionsRaw size drifted"
        );
        assert_eq!(
            align_of::<TextureLoaderOptionsRaw>(),
            8,
            "TextureLoaderOptionsRaw alignment drifted"
        );
    }

    #[test]
    fn ffi_layout_matches_swift() {
        // SAFETY: `mtk_verify_ffi_layout` takes no arguments and only reads
        // compile-time `MemoryLayout` constants in the Swift bridge.
        let matches = unsafe { mtk_verify_ffi_layout() };
        assert!(
            matches,
            "Swift FFI struct layout disagrees with Rust layout (ABI mismatch)"
        );
    }
}
