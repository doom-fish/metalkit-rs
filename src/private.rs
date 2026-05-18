use crate::error::MetalKitError;
use serde::de::DeserializeOwned;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

macro_rules! handle_type {
    ($name:ident, $doc:literal) => {
        handle_type!(#[doc = $doc] $name);
    };

    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        pub struct $name {
            pub(crate) ptr: *mut core::ffi::c_void,
            owned: bool,
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ptr", &self.ptr)
                    .field("owned", &self.owned)
                    .finish()
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                let ptr = unsafe { crate::ffi::mtk_retain(self.ptr) };
                Self { ptr, owned: true }
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if self.owned && !self.ptr.is_null() {
                    unsafe { crate::ffi::mtk_release(self.ptr) };
                    self.ptr = core::ptr::null_mut();
                }
            }
        }

        #[allow(dead_code)]
        impl $name {
            pub(crate) unsafe fn from_raw(ptr: *mut core::ffi::c_void) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { ptr, owned: true })
                }
            }

            pub(crate) const unsafe fn from_raw_unchecked(ptr: *mut core::ffi::c_void) -> Self {
                Self { ptr, owned: true }
            }

            pub(crate) const unsafe fn from_raw_borrowed(ptr: *mut core::ffi::c_void) -> Self {
                Self { ptr, owned: false }
            }

            #[must_use]
            pub(crate) const fn as_ptr(&self) -> *mut core::ffi::c_void {
                self.ptr
            }
        }
    };
}

pub(crate) use handle_type;

pub(crate) fn cstring_from_str(value: &str) -> Option<CString> {
    CString::new(value).ok()
}

pub(crate) fn cstring_from_path(path: &Path) -> Option<CString> {
    CString::new(path.as_os_str().as_bytes()).ok()
}

pub(crate) fn take_c_string(ptr: *mut libc::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let value = unsafe { std::ffi::CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { libc::free(ptr.cast()) };
    Some(value)
}

pub(crate) fn take_error(ptr: *mut libc::c_char, fallback_message: &str) -> MetalKitError {
    take_c_string(ptr).map_or_else(|| MetalKitError::new(fallback_message), MetalKitError::new)
}

pub(crate) fn parse_json<T: DeserializeOwned>(
    ptr: *mut libc::c_char,
    type_name: &str,
) -> Result<T, MetalKitError> {
    let json = take_c_string(ptr)
        .ok_or_else(|| MetalKitError::new(format!("missing {type_name} JSON payload")))?;
    serde_json::from_str(&json)
        .map_err(|err| MetalKitError::new(format!("failed to parse {type_name} JSON: {err}")))
}
