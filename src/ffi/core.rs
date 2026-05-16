use core::ffi::c_void;

unsafe extern "C" {
    pub fn mtk_retain(ptr: *mut c_void) -> *mut c_void;
    pub fn mtk_release(ptr: *mut c_void);
}
