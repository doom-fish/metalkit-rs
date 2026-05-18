use crate::error::MetalKitError;
use crate::ffi;
use crate::private::{cstring_from_path, cstring_from_str, handle_type};
use apple_cf::cg::CGImage;
use apple_metal::{MetalDevice, MetalTexture};
use core::ffi::c_void;
use std::ffi::CStr;
use std::path::Path;
use std::ptr;

handle_type!(TextureLoader, "Wraps `MTKTextureLoader`.");

/// Exposes `MTKTextureLoaderOption*` dictionary keys.
pub mod texture_loader_option {
    use super::TextureLoaderOptionKey;

    /// Exposes `MTKTextureLoaderOptionAllocateMipmaps`.
    pub const ALLOCATE_MIPMAPS: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionAllocateMipmaps");
    /// Exposes `MTKTextureLoaderOptionGenerateMipmaps`.
    pub const GENERATE_MIPMAPS: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionGenerateMipmaps");
    /// Exposes `MTKTextureLoaderOptionSRGB`.
    pub const SRGB: TextureLoaderOptionKey = TextureLoaderOptionKey("MTKTextureLoaderOptionSRGB");
    /// Exposes `MTKTextureLoaderOptionTextureUsage`.
    pub const TEXTURE_USAGE: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionTextureUsage");
    /// Exposes `MTKTextureLoaderOptionTextureStorageMode`.
    pub const TEXTURE_STORAGE_MODE: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionTextureStorageMode");
    /// Exposes `MTKTextureLoaderOptionTextureCPUCacheMode`.
    pub const TEXTURE_CPU_CACHE_MODE: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionTextureCPUCacheMode");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Wraps an `MTKTextureLoaderOption*` dictionary key.
pub struct TextureLoaderOptionKey(&'static str);

impl TextureLoaderOptionKey {
    #[must_use]
    /// Returns the wrapped `MTKTextureLoaderOption*` key.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// Exposes `MTLCPUCacheMode` values used by `MTKTextureLoader` options.
pub mod texture_cpu_cache_mode {
    /// Mirrors `MTLCPUCacheModeDefaultCache`.
    pub const DEFAULT_CACHE: usize = 0;
    /// Mirrors `MTLCPUCacheModeWriteCombined`.
    pub const WRITE_COMBINED: usize = 1;
}

#[derive(Debug, Clone, Default)]
/// Collects option entries for `MTKTextureLoader` requests.
pub struct TextureLoaderOptions {
    generate_mipmaps: Option<bool>,
    allocate_mipmaps: Option<bool>,
    srgb: Option<bool>,
    texture_usage: Option<usize>,
    texture_storage_mode: Option<usize>,
    texture_cpu_cache_mode: Option<usize>,
}

impl TextureLoaderOptions {
    #[must_use]
    /// Creates an empty `MTKTextureLoader` options builder.
    pub const fn new() -> Self {
        Self {
            generate_mipmaps: None,
            allocate_mipmaps: None,
            srgb: None,
            texture_usage: None,
            texture_storage_mode: None,
            texture_cpu_cache_mode: None,
        }
    }

    #[must_use]
    /// Sets `MTKTextureLoaderOptionGenerateMipmaps`.
    pub const fn with_generate_mipmaps(mut self, value: bool) -> Self {
        self.generate_mipmaps = Some(value);
        self
    }

    #[must_use]
    /// Sets `MTKTextureLoaderOptionAllocateMipmaps`.
    pub const fn with_allocate_mipmaps(mut self, value: bool) -> Self {
        self.allocate_mipmaps = Some(value);
        self
    }

    #[must_use]
    /// Sets `MTKTextureLoaderOptionSRGB`.
    pub const fn with_srgb(mut self, value: bool) -> Self {
        self.srgb = Some(value);
        self
    }

    #[must_use]
    /// Sets `MTKTextureLoaderOptionTextureUsage`.
    pub const fn with_texture_usage(mut self, value: usize) -> Self {
        self.texture_usage = Some(value);
        self
    }

    #[must_use]
    /// Sets `MTKTextureLoaderOptionTextureStorageMode`.
    pub const fn with_texture_storage_mode(mut self, value: usize) -> Self {
        self.texture_storage_mode = Some(value);
        self
    }

    #[must_use]
    /// Sets `MTKTextureLoaderOptionTextureCPUCacheMode`.
    pub const fn with_texture_cpu_cache_mode(mut self, value: usize) -> Self {
        self.texture_cpu_cache_mode = Some(value);
        self
    }
}

impl From<&TextureLoaderOptions> for ffi::TextureLoaderOptionsRaw {
    fn from(options: &TextureLoaderOptions) -> Self {
        let mut raw = ffi::TextureLoaderOptionsRaw::default();

        if let Some(value) = options.generate_mipmaps {
            raw.flags |= ffi::OPTION_GENERATE_MIPMAPS;
            raw.generate_mipmaps = u8::from(value);
        }
        if let Some(value) = options.allocate_mipmaps {
            raw.flags |= ffi::OPTION_ALLOCATE_MIPMAPS;
            raw.allocate_mipmaps = u8::from(value);
        }
        if let Some(value) = options.srgb {
            raw.flags |= ffi::OPTION_SRGB;
            raw.srgb = u8::from(value);
        }
        if let Some(value) = options.texture_usage {
            raw.flags |= ffi::OPTION_TEXTURE_USAGE;
            raw.texture_usage = value as u64;
        }
        if let Some(value) = options.texture_storage_mode {
            raw.flags |= ffi::OPTION_TEXTURE_STORAGE_MODE;
            raw.texture_storage_mode = value as u64;
        }
        if let Some(value) = options.texture_cpu_cache_mode {
            raw.flags |= ffi::OPTION_TEXTURE_CPU_CACHE_MODE;
            raw.texture_cpu_cache_mode = value as u64;
        }

        raw
    }
}

impl TextureLoader {
    #[must_use]
    /// Creates an `MTKTextureLoader`.
    pub fn new(device: &MetalDevice) -> Option<Self> {
        unsafe { Self::from_raw(ffi::mtk_texture_loader_new(device.as_ptr())) }
    }

    #[must_use]
    /// Returns the raw pointer from `MTKTextureLoader.device`.
    pub fn device_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_texture_loader_device(self.as_ptr()) }
    }

    /// Mirrors `-[MTKTextureLoader newTextureWithContentsOfURL:options:error:]`.
    pub fn new_texture_from_url<P: AsRef<Path>>(
        &self,
        path: P,
        options: Option<&TextureLoaderOptions>,
    ) -> Result<MetalTexture, MetalKitError> {
        let c_path = cstring_from_path(path.as_ref())
            .ok_or_else(|| MetalKitError::new("path contains an interior NUL byte"))?;
        let options_raw = options.map(ffi::TextureLoaderOptionsRaw::from);
        let options_ptr = options_raw.as_ref().map_or(ptr::null(), ptr::from_ref);
        let mut error = ptr::null_mut();
        let texture = unsafe {
            ffi::mtk_texture_loader_new_texture_from_url(
                self.as_ptr(),
                c_path.as_ptr(),
                options_ptr,
                ptr::addr_of_mut!(error),
            )
        };
        texture_from_result(texture, error, "failed to load texture from URL")
    }

    /// Mirrors `-[MTKTextureLoader newTextureWithName:scaleFactor:bundle:options:error:]`.
    pub fn new_texture_named(
        &self,
        name: &str,
        scale_factor: f64,
        bundle_path: Option<&Path>,
        options: Option<&TextureLoaderOptions>,
    ) -> Result<MetalTexture, MetalKitError> {
        let c_name = cstring_from_str(name)
            .ok_or_else(|| MetalKitError::new("asset name contains an interior NUL byte"))?;
        let c_bundle_path = bundle_path
            .map(|path| {
                cstring_from_path(path)
                    .ok_or_else(|| MetalKitError::new("bundle path contains an interior NUL byte"))
            })
            .transpose()?;
        let options_raw = options.map(ffi::TextureLoaderOptionsRaw::from);
        let options_ptr = options_raw.as_ref().map_or(ptr::null(), ptr::from_ref);
        let mut error = ptr::null_mut();
        let texture = unsafe {
            ffi::mtk_texture_loader_new_texture_named(
                self.as_ptr(),
                c_name.as_ptr(),
                scale_factor,
                c_bundle_path
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                options_ptr,
                ptr::addr_of_mut!(error),
            )
        };
        texture_from_result(texture, error, "failed to load named texture")
    }

    /// Mirrors `-[MTKTextureLoader newTextureWithData:options:error:]`.
    pub fn new_texture_from_data(
        &self,
        data: &[u8],
        options: Option<&TextureLoaderOptions>,
    ) -> Result<MetalTexture, MetalKitError> {
        let options_raw = options.map(ffi::TextureLoaderOptionsRaw::from);
        let options_ptr = options_raw.as_ref().map_or(ptr::null(), ptr::from_ref);
        let mut error = ptr::null_mut();
        let bytes = if data.is_empty() {
            ptr::null()
        } else {
            data.as_ptr().cast::<c_void>()
        };
        let texture = unsafe {
            ffi::mtk_texture_loader_new_texture_from_data(
                self.as_ptr(),
                bytes,
                data.len(),
                options_ptr,
                ptr::addr_of_mut!(error),
            )
        };
        texture_from_result(texture, error, "failed to load texture from data")
    }

    /// Mirrors `-[MTKTextureLoader newTextureWithCGImage:options:error:]`.
    pub fn new_texture_from_cgimage(
        &self,
        image: &CGImage,
        options: Option<&TextureLoaderOptions>,
    ) -> Result<MetalTexture, MetalKitError> {
        let options_raw = options.map(ffi::TextureLoaderOptionsRaw::from);
        let options_ptr = options_raw.as_ref().map_or(ptr::null(), ptr::from_ref);
        let mut error = ptr::null_mut();
        let texture = unsafe {
            ffi::mtk_texture_loader_new_texture_from_cgimage(
                self.as_ptr(),
                image.as_ptr(),
                options_ptr,
                ptr::addr_of_mut!(error),
            )
        };
        texture_from_result(texture, error, "failed to load texture from CGImage")
    }
}

fn texture_from_result(
    texture: *mut c_void,
    error: *mut libc::c_char,
    fallback_message: &str,
) -> Result<MetalTexture, MetalKitError> {
    if texture.is_null() {
        Err(take_error(error).unwrap_or_else(|| MetalKitError::new(fallback_message)))
    } else {
        Ok(unsafe { MetalTexture::from_raw(texture) })
    }
}

fn take_error(ptr: *mut libc::c_char) -> Option<MetalKitError> {
    if ptr.is_null() {
        return None;
    }
    let message = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { libc::free(ptr.cast()) };
    Some(MetalKitError::new(message))
}
