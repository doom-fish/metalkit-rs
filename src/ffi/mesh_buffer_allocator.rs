use core::ffi::c_void;

unsafe extern "C" {
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
}
