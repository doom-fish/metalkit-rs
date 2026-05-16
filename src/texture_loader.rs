use crate::error::MetalKitError;
use crate::ffi;
use crate::model_io_integration::ModelTexture;
use crate::private::{cstring_from_path, cstring_from_str, handle_type, take_c_string, take_error};
use apple_cf::cg::CGImage;
use apple_metal::{MetalDevice, MetalTexture};
use core::ffi::c_void;
use std::ffi::CString;
use std::path::Path;
use std::ptr;

handle_type!(TextureLoader);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureLoaderError(&'static str);

impl TextureLoaderError {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl core::fmt::Display for TextureLoaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)
    }
}

pub type TextureLoaderCallback = Box<dyn FnOnce(Result<MetalTexture, MetalKitError>) + Send + 'static>;
pub type TextureLoaderArrayCallback = Box<dyn FnOnce(TextureLoaderArrayOutcome) + Send + 'static>;

pub mod texture_loader_error {
    pub const DOMAIN: &str = "MTKTextureLoaderErrorDomain";
    pub const KEY: &str = "MTKTextureLoaderErrorKey";
}

impl TextureLoaderError {
    pub const DOMAIN: Self = Self(texture_loader_error::DOMAIN);
    pub const KEY: Self = Self(texture_loader_error::KEY);
}

pub mod texture_loader_option {
    use super::TextureLoaderOptionKey;

    pub const ALLOCATE_MIPMAPS: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionAllocateMipmaps");
    pub const GENERATE_MIPMAPS: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionGenerateMipmaps");
    pub const SRGB: TextureLoaderOptionKey = TextureLoaderOptionKey("MTKTextureLoaderOptionSRGB");
    pub const TEXTURE_USAGE: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionTextureUsage");
    pub const TEXTURE_STORAGE_MODE: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionTextureStorageMode");
    pub const TEXTURE_CPU_CACHE_MODE: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionTextureCPUCacheMode");
    pub const CUBE_LAYOUT: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionCubeLayout");
    pub const ORIGIN: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionOrigin");
    pub const LOAD_AS_ARRAY: TextureLoaderOptionKey =
        TextureLoaderOptionKey("MTKTextureLoaderOptionLoadAsArray");
}

pub mod texture_loader_cube_layout {
    use super::TextureLoaderCubeLayout;

    pub const VERTICAL: TextureLoaderCubeLayout =
        TextureLoaderCubeLayout("MTKTextureLoaderCubeLayoutVertical");
}

pub mod texture_loader_origin {
    use super::TextureLoaderOrigin;

    pub const TOP_LEFT: TextureLoaderOrigin = TextureLoaderOrigin("MTKTextureLoaderOriginTopLeft");
    pub const BOTTOM_LEFT: TextureLoaderOrigin =
        TextureLoaderOrigin("MTKTextureLoaderOriginBottomLeft");
    pub const FLIPPED_VERTICALLY: TextureLoaderOrigin =
        TextureLoaderOrigin("MTKTextureLoaderOriginFlippedVertically");
}

pub mod texture_cpu_cache_mode {
    pub const DEFAULT_CACHE: usize = 0;
    pub const WRITE_COMBINED: usize = 1;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureLoaderOptionKey(&'static str);

impl TextureLoaderOptionKey {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureLoaderCubeLayout(&'static str);

impl TextureLoaderCubeLayout {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureLoaderOrigin(&'static str);

impl TextureLoaderOrigin {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum DisplayGamut {
    SRGB = 1,
    P3 = 2,
}

pub struct TextureLoaderArrayOutcome {
    pub textures: Vec<Option<MetalTexture>>,
    pub error: Option<MetalKitError>,
}

impl core::fmt::Debug for TextureLoaderArrayOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TextureLoaderArrayOutcome")
            .field("textures_len", &self.textures.len())
            .field("loaded_count", &self.textures.iter().filter(|texture| texture.is_some()).count())
            .field("error", &self.error)
            .finish()
    }
}

impl TextureLoaderArrayOutcome {
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.error.is_none()
    }
}

#[derive(Debug, Clone, Default)]
pub struct TextureLoaderOptions {
    generate_mipmaps: Option<bool>,
    allocate_mipmaps: Option<bool>,
    srgb: Option<bool>,
    texture_usage: Option<usize>,
    texture_storage_mode: Option<usize>,
    texture_cpu_cache_mode: Option<usize>,
    cube_layout: Option<TextureLoaderCubeLayout>,
    origin: Option<TextureLoaderOrigin>,
    load_as_array: Option<bool>,
}

impl TextureLoaderOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            generate_mipmaps: None,
            allocate_mipmaps: None,
            srgb: None,
            texture_usage: None,
            texture_storage_mode: None,
            texture_cpu_cache_mode: None,
            cube_layout: None,
            origin: None,
            load_as_array: None,
        }
    }

    #[must_use]
    pub const fn with_generate_mipmaps(mut self, value: bool) -> Self {
        self.generate_mipmaps = Some(value);
        self
    }

    #[must_use]
    pub const fn with_allocate_mipmaps(mut self, value: bool) -> Self {
        self.allocate_mipmaps = Some(value);
        self
    }

    #[must_use]
    pub const fn with_srgb(mut self, value: bool) -> Self {
        self.srgb = Some(value);
        self
    }

    #[must_use]
    pub const fn with_texture_usage(mut self, value: usize) -> Self {
        self.texture_usage = Some(value);
        self
    }

    #[must_use]
    pub const fn with_texture_storage_mode(mut self, value: usize) -> Self {
        self.texture_storage_mode = Some(value);
        self
    }

    #[must_use]
    pub const fn with_texture_cpu_cache_mode(mut self, value: usize) -> Self {
        self.texture_cpu_cache_mode = Some(value);
        self
    }

    #[must_use]
    pub const fn with_cube_layout(mut self, value: TextureLoaderCubeLayout) -> Self {
        self.cube_layout = Some(value);
        self
    }

    #[must_use]
    pub const fn with_origin(mut self, value: TextureLoaderOrigin) -> Self {
        self.origin = Some(value);
        self
    }

    #[must_use]
    pub const fn with_load_as_array(mut self, value: bool) -> Self {
        self.load_as_array = Some(value);
        self
    }
}

#[derive(Debug)]
struct TextureLoaderOptionsOwned {
    raw: ffi::TextureLoaderOptionsRaw,
    _cube_layout: Option<CString>,
    _origin: Option<CString>,
}

impl TextureLoaderOptionsOwned {
    fn new(options: Option<&TextureLoaderOptions>) -> Result<Option<Self>, MetalKitError> {
        let Some(options) = options else {
            return Ok(None);
        };

        let cube_layout = options
            .cube_layout
            .map(TextureLoaderCubeLayout::as_str)
            .map(|value| {
                CString::new(value)
                    .map_err(|_| MetalKitError::new("cube layout contains an interior NUL byte"))
            })
            .transpose()?;
        let origin = options
            .origin
            .map(TextureLoaderOrigin::as_str)
            .map(|value| {
                CString::new(value)
                    .map_err(|_| MetalKitError::new("origin contains an interior NUL byte"))
            })
            .transpose()?;

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
        if options.cube_layout.is_some() {
            raw.flags |= ffi::OPTION_CUBE_LAYOUT;
            raw.cube_layout = cube_layout.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        }
        if options.origin.is_some() {
            raw.flags |= ffi::OPTION_ORIGIN;
            raw.origin = origin.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        }
        if let Some(value) = options.load_as_array {
            raw.flags |= ffi::OPTION_LOAD_AS_ARRAY;
            raw.load_as_array = u8::from(value);
        }

        Ok(Some(Self {
            raw,
            _cube_layout: cube_layout,
            _origin: origin,
        }))
    }

    #[must_use]
    const fn as_ptr(&self) -> *const ffi::TextureLoaderOptionsRaw {
        ptr::from_ref(&self.raw)
    }
}

#[derive(Debug)]
struct RetainedHandle {
    ptr: *mut c_void,
}

impl RetainedHandle {
    fn new(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr: unsafe { ffi::mtk_retain(ptr) },
            })
        }
    }
}

impl Drop for RetainedHandle {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::mtk_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

struct CallbackState<F> {
    callback: Option<F>,
    fallback_message: &'static str,
    _retained: Vec<RetainedHandle>,
}

impl<F> CallbackState<F> {
    fn into_user_data<I>(callback: F, fallback_message: &'static str, retained: I) -> *mut c_void
    where
        I: IntoIterator<Item = *mut c_void>,
    {
        let retained = retained.into_iter().filter_map(RetainedHandle::new).collect();
        Box::into_raw(Box::new(Self {
            callback: Some(callback),
            fallback_message,
            _retained: retained,
        }))
        .cast::<c_void>()
    }
}

unsafe extern "C" fn texture_loader_callback_trampoline<F>(
    user_data: *mut c_void,
    texture: *mut c_void,
    error: *mut libc::c_char,
) where
    F: FnOnce(Result<MetalTexture, MetalKitError>) + Send + 'static,
{
    if user_data.is_null() {
        if !texture.is_null() {
            unsafe { ffi::mtk_release(texture) };
        }
        drop(take_c_string(error));
        return;
    }

    let mut state = unsafe { Box::from_raw(user_data.cast::<CallbackState<F>>()) };
    let Some(callback) = state.callback.take() else {
        if !texture.is_null() {
            unsafe { ffi::mtk_release(texture) };
        }
        drop(take_c_string(error));
        return;
    };
    let outcome = texture_from_result(texture, error, state.fallback_message);
    run_callback(callback, outcome);
}

unsafe extern "C" fn texture_loader_array_callback_trampoline<F>(
    user_data: *mut c_void,
    result: *mut c_void,
    error: *mut libc::c_char,
) where
    F: FnOnce(TextureLoaderArrayOutcome) + Send + 'static,
{
    if user_data.is_null() {
        if !result.is_null() {
            unsafe { ffi::mtk_release(result) };
        }
        drop(take_c_string(error));
        return;
    }

    let mut state = unsafe { Box::from_raw(user_data.cast::<CallbackState<F>>()) };
    let Some(callback) = state.callback.take() else {
        if !result.is_null() {
            unsafe { ffi::mtk_release(result) };
        }
        drop(take_c_string(error));
        return;
    };
    let outcome = texture_array_from_result_with_fallback(
        result,
        error,
        Some(state.fallback_message),
    );
    run_callback(callback, outcome);
}

fn run_callback<F, T>(callback: F, value: T)
where
    F: FnOnce(T),
{
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(value)));
}

impl TextureLoader {
    #[must_use]
    pub fn new(device: &MetalDevice) -> Option<Self> {
        unsafe { Self::from_raw(ffi::mtk_texture_loader_new(device.as_ptr())) }
    }

    #[must_use]
    pub fn device_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_texture_loader_device(self.as_ptr()) }
    }

    pub fn new_texture_from_url<P: AsRef<Path>>(
        &self,
        path: P,
        options: Option<&TextureLoaderOptions>,
    ) -> Result<MetalTexture, MetalKitError> {
        let c_path = cstring_from_path(path.as_ref())
            .ok_or_else(|| MetalKitError::new("path contains an interior NUL byte"))?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
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

    pub fn new_texture_from_url_with_callback<P, F>(
        &self,
        path: P,
        options: Option<&TextureLoaderOptions>,
        callback: F,
    ) -> Result<(), MetalKitError>
    where
        P: AsRef<Path>,
        F: FnOnce(Result<MetalTexture, MetalKitError>) + Send + 'static,
    {
        let c_path = cstring_from_path(path.as_ref())
            .ok_or_else(|| MetalKitError::new("path contains an interior NUL byte"))?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let user_data = CallbackState::into_user_data(
            callback,
            "failed to load texture from URL",
            [self.as_ptr()],
        );
        unsafe {
            ffi::mtk_texture_loader_new_texture_from_url_with_callback(
                self.as_ptr(),
                c_path.as_ptr(),
                options_ptr,
                Some(texture_loader_callback_trampoline::<F>),
                user_data,
            )
        };
        Ok(())
    }

    pub fn new_textures_from_urls<P: AsRef<Path>>(
        &self,
        paths: &[P],
        options: Option<&TextureLoaderOptions>,
    ) -> Result<TextureLoaderArrayOutcome, MetalKitError> {
        let (_owned_paths, raw_paths) = c_strings_from_paths(paths)?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let mut error = ptr::null_mut();
        let result = unsafe {
            ffi::mtk_texture_loader_new_textures_from_urls(
                self.as_ptr(),
                if raw_paths.is_empty() {
                    ptr::null()
                } else {
                    raw_paths.as_ptr()
                },
                raw_paths.len(),
                options_ptr,
                ptr::addr_of_mut!(error),
            )
        };
        Ok(texture_array_from_result(result, error))
    }

    pub fn new_textures_from_urls_with_callback<P, F>(
        &self,
        paths: &[P],
        options: Option<&TextureLoaderOptions>,
        callback: F,
    ) -> Result<(), MetalKitError>
    where
        P: AsRef<Path>,
        F: FnOnce(TextureLoaderArrayOutcome) + Send + 'static,
    {
        let (_owned_paths, raw_paths) = c_strings_from_paths(paths)?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let user_data = CallbackState::into_user_data(
            callback,
            "failed to load textures from URLs",
            [self.as_ptr()],
        );
        unsafe {
            ffi::mtk_texture_loader_new_textures_from_urls_with_callback(
                self.as_ptr(),
                if raw_paths.is_empty() {
                    ptr::null()
                } else {
                    raw_paths.as_ptr()
                },
                raw_paths.len(),
                options_ptr,
                Some(texture_loader_array_callback_trampoline::<F>),
                user_data,
            )
        };
        Ok(())
    }

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
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
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

    pub fn new_texture_named_with_callback<F>(
        &self,
        name: &str,
        scale_factor: f64,
        bundle_path: Option<&Path>,
        options: Option<&TextureLoaderOptions>,
        callback: F,
    ) -> Result<(), MetalKitError>
    where
        F: FnOnce(Result<MetalTexture, MetalKitError>) + Send + 'static,
    {
        let c_name = cstring_from_str(name)
            .ok_or_else(|| MetalKitError::new("asset name contains an interior NUL byte"))?;
        let c_bundle_path = bundle_path
            .map(|path| {
                cstring_from_path(path)
                    .ok_or_else(|| MetalKitError::new("bundle path contains an interior NUL byte"))
            })
            .transpose()?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let user_data = CallbackState::into_user_data(
            callback,
            "failed to load named texture",
            [self.as_ptr()],
        );
        unsafe {
            ffi::mtk_texture_loader_new_texture_named_with_callback(
                self.as_ptr(),
                c_name.as_ptr(),
                scale_factor,
                c_bundle_path
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                options_ptr,
                Some(texture_loader_callback_trampoline::<F>),
                user_data,
            )
        };
        Ok(())
    }

    pub fn new_texture_named_with_display_gamut(
        &self,
        name: &str,
        scale_factor: f64,
        display_gamut: DisplayGamut,
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
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let mut error = ptr::null_mut();
        let texture = unsafe {
            ffi::mtk_texture_loader_new_texture_named_with_display_gamut(
                self.as_ptr(),
                c_name.as_ptr(),
                scale_factor,
                display_gamut as usize,
                c_bundle_path
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                options_ptr,
                ptr::addr_of_mut!(error),
            )
        };
        texture_from_result(
            texture,
            error,
            "failed to load named texture with display gamut",
        )
    }

    pub fn new_texture_named_with_display_gamut_with_callback<F>(
        &self,
        name: &str,
        scale_factor: f64,
        display_gamut: DisplayGamut,
        bundle_path: Option<&Path>,
        options: Option<&TextureLoaderOptions>,
        callback: F,
    ) -> Result<(), MetalKitError>
    where
        F: FnOnce(Result<MetalTexture, MetalKitError>) + Send + 'static,
    {
        let c_name = cstring_from_str(name)
            .ok_or_else(|| MetalKitError::new("asset name contains an interior NUL byte"))?;
        let c_bundle_path = bundle_path
            .map(|path| {
                cstring_from_path(path)
                    .ok_or_else(|| MetalKitError::new("bundle path contains an interior NUL byte"))
            })
            .transpose()?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let user_data = CallbackState::into_user_data(
            callback,
            "failed to load named texture with display gamut",
            [self.as_ptr()],
        );
        unsafe {
            ffi::mtk_texture_loader_new_texture_named_with_display_gamut_with_callback(
                self.as_ptr(),
                c_name.as_ptr(),
                scale_factor,
                display_gamut as usize,
                c_bundle_path
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                options_ptr,
                Some(texture_loader_callback_trampoline::<F>),
                user_data,
            )
        };
        Ok(())
    }

    pub fn new_textures_named(
        &self,
        names: &[&str],
        scale_factor: f64,
        bundle_path: Option<&Path>,
        options: Option<&TextureLoaderOptions>,
    ) -> Result<TextureLoaderArrayOutcome, MetalKitError> {
        let (_owned_names, raw_names) = c_strings_from_strs(names, "asset name")?;
        let c_bundle_path = bundle_path
            .map(|path| {
                cstring_from_path(path)
                    .ok_or_else(|| MetalKitError::new("bundle path contains an interior NUL byte"))
            })
            .transpose()?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let mut error = ptr::null_mut();
        let result = unsafe {
            ffi::mtk_texture_loader_new_textures_named(
                self.as_ptr(),
                if raw_names.is_empty() {
                    ptr::null()
                } else {
                    raw_names.as_ptr()
                },
                raw_names.len(),
                scale_factor,
                c_bundle_path
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                options_ptr,
                ptr::addr_of_mut!(error),
            )
        };
        Ok(texture_array_from_result(result, error))
    }

    pub fn new_textures_named_with_callback<F>(
        &self,
        names: &[&str],
        scale_factor: f64,
        bundle_path: Option<&Path>,
        options: Option<&TextureLoaderOptions>,
        callback: F,
    ) -> Result<(), MetalKitError>
    where
        F: FnOnce(TextureLoaderArrayOutcome) + Send + 'static,
    {
        let (_owned_names, raw_names) = c_strings_from_strs(names, "asset name")?;
        let c_bundle_path = bundle_path
            .map(|path| {
                cstring_from_path(path)
                    .ok_or_else(|| MetalKitError::new("bundle path contains an interior NUL byte"))
            })
            .transpose()?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let user_data = CallbackState::into_user_data(
            callback,
            "failed to load named textures",
            [self.as_ptr()],
        );
        unsafe {
            ffi::mtk_texture_loader_new_textures_named_with_callback(
                self.as_ptr(),
                if raw_names.is_empty() {
                    ptr::null()
                } else {
                    raw_names.as_ptr()
                },
                raw_names.len(),
                scale_factor,
                c_bundle_path
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                options_ptr,
                Some(texture_loader_array_callback_trampoline::<F>),
                user_data,
            )
        };
        Ok(())
    }

    pub fn new_textures_named_with_display_gamut(
        &self,
        names: &[&str],
        scale_factor: f64,
        display_gamut: DisplayGamut,
        bundle_path: Option<&Path>,
        options: Option<&TextureLoaderOptions>,
    ) -> Result<TextureLoaderArrayOutcome, MetalKitError> {
        let (_owned_names, raw_names) = c_strings_from_strs(names, "asset name")?;
        let c_bundle_path = bundle_path
            .map(|path| {
                cstring_from_path(path)
                    .ok_or_else(|| MetalKitError::new("bundle path contains an interior NUL byte"))
            })
            .transpose()?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let mut error = ptr::null_mut();
        let result = unsafe {
            ffi::mtk_texture_loader_new_textures_named_with_display_gamut(
                self.as_ptr(),
                if raw_names.is_empty() {
                    ptr::null()
                } else {
                    raw_names.as_ptr()
                },
                raw_names.len(),
                scale_factor,
                display_gamut as usize,
                c_bundle_path
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                options_ptr,
                ptr::addr_of_mut!(error),
            )
        };
        Ok(texture_array_from_result(result, error))
    }

    pub fn new_textures_named_with_display_gamut_with_callback<F>(
        &self,
        names: &[&str],
        scale_factor: f64,
        display_gamut: DisplayGamut,
        bundle_path: Option<&Path>,
        options: Option<&TextureLoaderOptions>,
        callback: F,
    ) -> Result<(), MetalKitError>
    where
        F: FnOnce(TextureLoaderArrayOutcome) + Send + 'static,
    {
        let (_owned_names, raw_names) = c_strings_from_strs(names, "asset name")?;
        let c_bundle_path = bundle_path
            .map(|path| {
                cstring_from_path(path)
                    .ok_or_else(|| MetalKitError::new("bundle path contains an interior NUL byte"))
            })
            .transpose()?;
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let user_data = CallbackState::into_user_data(
            callback,
            "failed to load named textures with display gamut",
            [self.as_ptr()],
        );
        unsafe {
            ffi::mtk_texture_loader_new_textures_named_with_display_gamut_with_callback(
                self.as_ptr(),
                if raw_names.is_empty() {
                    ptr::null()
                } else {
                    raw_names.as_ptr()
                },
                raw_names.len(),
                scale_factor,
                display_gamut as usize,
                c_bundle_path
                    .as_ref()
                    .map_or(ptr::null(), |path| path.as_ptr()),
                options_ptr,
                Some(texture_loader_array_callback_trampoline::<F>),
                user_data,
            )
        };
        Ok(())
    }

    pub fn new_texture_from_data(
        &self,
        data: &[u8],
        options: Option<&TextureLoaderOptions>,
    ) -> Result<MetalTexture, MetalKitError> {
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
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

    pub fn new_texture_from_data_with_callback<F>(
        &self,
        data: &[u8],
        options: Option<&TextureLoaderOptions>,
        callback: F,
    ) -> Result<(), MetalKitError>
    where
        F: FnOnce(Result<MetalTexture, MetalKitError>) + Send + 'static,
    {
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let bytes = if data.is_empty() {
            ptr::null()
        } else {
            data.as_ptr().cast::<c_void>()
        };
        let user_data = CallbackState::into_user_data(
            callback,
            "failed to load texture from data",
            [self.as_ptr()],
        );
        unsafe {
            ffi::mtk_texture_loader_new_texture_from_data_with_callback(
                self.as_ptr(),
                bytes,
                data.len(),
                options_ptr,
                Some(texture_loader_callback_trampoline::<F>),
                user_data,
            )
        };
        Ok(())
    }

    pub fn new_texture_from_cgimage(
        &self,
        image: &CGImage,
        options: Option<&TextureLoaderOptions>,
    ) -> Result<MetalTexture, MetalKitError> {
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
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

    pub fn new_texture_from_cgimage_with_callback<F>(
        &self,
        image: &CGImage,
        options: Option<&TextureLoaderOptions>,
        callback: F,
    ) -> Result<(), MetalKitError>
    where
        F: FnOnce(Result<MetalTexture, MetalKitError>) + Send + 'static,
    {
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let user_data = CallbackState::into_user_data(
            callback,
            "failed to load texture from CGImage",
            [self.as_ptr()],
        );
        unsafe {
            ffi::mtk_texture_loader_new_texture_from_cgimage_with_callback(
                self.as_ptr(),
                image.as_ptr(),
                options_ptr,
                Some(texture_loader_callback_trampoline::<F>),
                user_data,
            )
        };
        Ok(())
    }

    pub fn new_texture_from_model_texture(
        &self,
        texture: &ModelTexture,
        options: Option<&TextureLoaderOptions>,
    ) -> Result<MetalTexture, MetalKitError> {
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let mut error = ptr::null_mut();
        let texture = unsafe {
            ffi::mtk_texture_loader_new_texture_from_model_texture(
                self.as_ptr(),
                texture.as_ptr(),
                options_ptr,
                ptr::addr_of_mut!(error),
            )
        };
        texture_from_result(texture, error, "failed to load texture from MDLTexture")
    }

    pub fn new_texture_from_model_texture_with_callback<F>(
        &self,
        texture: &ModelTexture,
        options: Option<&TextureLoaderOptions>,
        callback: F,
    ) -> Result<(), MetalKitError>
    where
        F: FnOnce(Result<MetalTexture, MetalKitError>) + Send + 'static,
    {
        let options = TextureLoaderOptionsOwned::new(options)?;
        let options_ptr = options.as_ref().map_or(ptr::null(), TextureLoaderOptionsOwned::as_ptr);
        let user_data = CallbackState::into_user_data(
            callback,
            "failed to load texture from MDLTexture",
            [self.as_ptr(), texture.as_ptr()],
        );
        unsafe {
            ffi::mtk_texture_loader_new_texture_from_model_texture_with_callback(
                self.as_ptr(),
                texture.as_ptr(),
                options_ptr,
                Some(texture_loader_callback_trampoline::<F>),
                user_data,
            )
        };
        Ok(())
    }
}

fn texture_from_result(
    texture: *mut c_void,
    error: *mut libc::c_char,
    fallback_message: &str,
) -> Result<MetalTexture, MetalKitError> {
    if texture.is_null() {
        Err(take_error(error, fallback_message))
    } else {
        drop(take_c_string(error));
        Ok(unsafe { MetalTexture::from_raw(texture) })
    }
}

fn texture_array_from_result(
    result: *mut c_void,
    error: *mut libc::c_char,
) -> TextureLoaderArrayOutcome {
    texture_array_from_result_with_fallback(result, error, None)
}

fn texture_array_from_result_with_fallback(
    result: *mut c_void,
    error: *mut libc::c_char,
    fallback_message: Option<&str>,
) -> TextureLoaderArrayOutcome {
    let error = take_c_string(error)
        .map(MetalKitError::new)
        .or_else(|| result.is_null().then_some(fallback_message).flatten().map(MetalKitError::new));
    let textures = if result.is_null() {
        Vec::new()
    } else {
        let count = unsafe { ffi::mtk_texture_array_count(result) };
        let textures = (0..count)
            .map(|index| {
                let texture = unsafe { ffi::mtk_texture_array_texture_at(result, index) };
                if texture.is_null() {
                    None
                } else {
                    Some(unsafe { MetalTexture::from_raw(texture) })
                }
            })
            .collect();
        unsafe { ffi::mtk_release(result) };
        textures
    };

    TextureLoaderArrayOutcome { textures, error }
}

fn c_strings_from_paths<P: AsRef<Path>>(
    paths: &[P],
) -> Result<(Vec<CString>, Vec<*const libc::c_char>), MetalKitError> {
    let owned = paths
        .iter()
        .map(|path| {
            cstring_from_path(path.as_ref())
                .ok_or_else(|| MetalKitError::new("path contains an interior NUL byte"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let raw = owned.iter().map(|path| path.as_ptr()).collect();
    Ok((owned, raw))
}

fn c_strings_from_strs(
    values: &[&str],
    label: &str,
) -> Result<(Vec<CString>, Vec<*const libc::c_char>), MetalKitError> {
    let owned = values
        .iter()
        .map(|value| {
            CString::new(*value)
                .map_err(|_| MetalKitError::new(format!("{label} contains an interior NUL byte")))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let raw = owned.iter().map(|value| value.as_ptr()).collect();
    Ok((owned, raw))
}
