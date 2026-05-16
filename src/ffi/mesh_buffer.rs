use core::ffi::c_void;
use libc::c_char;

unsafe extern "C" {
    pub fn mtk_mesh_buffer_length(buffer: *mut c_void) -> usize;
    pub fn mtk_mesh_buffer_allocator(buffer: *mut c_void) -> *mut c_void;
    pub fn mtk_mesh_buffer_zone(buffer: *mut c_void) -> *mut c_void;
    pub fn mtk_mesh_buffer_offset(buffer: *mut c_void) -> usize;
    pub fn mtk_mesh_buffer_type(buffer: *mut c_void) -> usize;
    pub fn mtk_mesh_buffer_metal_buffer(buffer: *mut c_void) -> *mut c_void;
    pub fn mtk_mesh_buffer_copy_bytes(buffer: *mut c_void, dst: *mut c_void, len: usize) -> usize;
    pub fn mtk_mesh_buffer_get_name(buffer: *mut c_void) -> *mut c_char;
    pub fn mtk_mesh_buffer_set_name(buffer: *mut c_void, name: *const c_char);
}
