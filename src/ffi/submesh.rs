use core::ffi::c_void;
use libc::c_char;

unsafe extern "C" {
    pub fn mtk_submesh_primitive_type(submesh: *mut c_void) -> usize;
    pub fn mtk_submesh_index_type(submesh: *mut c_void) -> usize;
    pub fn mtk_submesh_index_buffer(submesh: *mut c_void) -> *mut c_void;
    pub fn mtk_submesh_index_count(submesh: *mut c_void) -> usize;
    pub fn mtk_submesh_mesh(submesh: *mut c_void) -> *mut c_void;
    pub fn mtk_submesh_get_name(submesh: *mut c_void) -> *mut c_char;
    pub fn mtk_submesh_set_name(submesh: *mut c_void, name: *const c_char);
}
