use core::ffi::c_void;
use libc::c_char;

pub type ViewDrawCallback = unsafe extern "C" fn(user_data: *mut c_void, view: *mut c_void);
pub type ViewResizeCallback =
    unsafe extern "C" fn(user_data: *mut c_void, view: *mut c_void, width: f64, height: f64);

unsafe extern "C" {
    pub fn mtk_view_delegate_new(
        draw_callback: Option<ViewDrawCallback>,
        resize_callback: Option<ViewResizeCallback>,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn mtk_view_new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        device: *mut c_void,
    ) -> *mut c_void;
    pub fn mtk_view_archive_round_trip(
        view: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mtk_view_delegate(view: *mut c_void) -> *mut c_void;
    pub fn mtk_view_set_delegate(view: *mut c_void, delegate: *mut c_void);
    pub fn mtk_view_device(view: *mut c_void) -> *mut c_void;
    pub fn mtk_view_set_device(view: *mut c_void, device: *mut c_void);
    pub fn mtk_view_current_drawable(view: *mut c_void) -> *mut c_void;
    pub fn mtk_view_framebuffer_only(view: *mut c_void) -> bool;
    pub fn mtk_view_set_framebuffer_only(view: *mut c_void, value: bool);
    pub fn mtk_view_depth_stencil_attachment_texture_usage(view: *mut c_void) -> usize;
    pub fn mtk_view_set_depth_stencil_attachment_texture_usage(view: *mut c_void, value: usize);
    pub fn mtk_view_multisample_color_attachment_texture_usage(view: *mut c_void) -> usize;
    pub fn mtk_view_set_multisample_color_attachment_texture_usage(view: *mut c_void, value: usize);
    pub fn mtk_view_presents_with_transaction(view: *mut c_void) -> bool;
    pub fn mtk_view_set_presents_with_transaction(view: *mut c_void, value: bool);
    pub fn mtk_view_color_pixel_format(view: *mut c_void) -> usize;
    pub fn mtk_view_set_color_pixel_format(view: *mut c_void, value: usize);
    pub fn mtk_view_depth_stencil_pixel_format(view: *mut c_void) -> usize;
    pub fn mtk_view_set_depth_stencil_pixel_format(view: *mut c_void, value: usize);
    pub fn mtk_view_depth_stencil_storage_mode(view: *mut c_void) -> usize;
    pub fn mtk_view_set_depth_stencil_storage_mode(view: *mut c_void, value: usize);
    pub fn mtk_view_sample_count(view: *mut c_void) -> usize;
    pub fn mtk_view_set_sample_count(view: *mut c_void, value: usize);
    pub fn mtk_view_clear_color(
        view: *mut c_void,
        out_red: *mut f64,
        out_green: *mut f64,
        out_blue: *mut f64,
        out_alpha: *mut f64,
    );
    pub fn mtk_view_set_clear_color(view: *mut c_void, red: f64, green: f64, blue: f64, alpha: f64);
    pub fn mtk_view_clear_depth(view: *mut c_void) -> f64;
    pub fn mtk_view_set_clear_depth(view: *mut c_void, value: f64);
    pub fn mtk_view_clear_stencil(view: *mut c_void) -> u32;
    pub fn mtk_view_set_clear_stencil(view: *mut c_void, value: u32);
    pub fn mtk_view_depth_stencil_texture(view: *mut c_void) -> *mut c_void;
    pub fn mtk_view_multisample_color_texture(view: *mut c_void) -> *mut c_void;
    pub fn mtk_view_release_drawables(view: *mut c_void);
    pub fn mtk_view_current_render_pass_descriptor(view: *mut c_void) -> *mut c_void;
    pub fn mtk_view_current_mtl4_render_pass_descriptor(view: *mut c_void) -> *mut c_void;
    pub fn mtk_view_preferred_frames_per_second(view: *mut c_void) -> isize;
    pub fn mtk_view_set_preferred_frames_per_second(view: *mut c_void, value: isize);
    pub fn mtk_view_enable_set_needs_display(view: *mut c_void) -> bool;
    pub fn mtk_view_set_enable_set_needs_display(view: *mut c_void, value: bool);
    pub fn mtk_view_auto_resize_drawable(view: *mut c_void) -> bool;
    pub fn mtk_view_set_auto_resize_drawable(view: *mut c_void, value: bool);
    pub fn mtk_view_drawable_size(view: *mut c_void, out_width: *mut f64, out_height: *mut f64);
    pub fn mtk_view_set_drawable_size(view: *mut c_void, width: f64, height: f64);
    pub fn mtk_view_preferred_drawable_size(
        view: *mut c_void,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn mtk_view_preferred_device(view: *mut c_void) -> *mut c_void;
    pub fn mtk_view_is_paused(view: *mut c_void) -> bool;
    pub fn mtk_view_set_paused(view: *mut c_void, value: bool);
    pub fn mtk_view_colorspace(view: *mut c_void) -> *mut c_void;
    pub fn mtk_view_set_colorspace(view: *mut c_void, value: *mut c_void);
    pub fn mtk_view_draw(view: *mut c_void);
    pub fn mtk_view_notify_delegate_size_will_change(view: *mut c_void);
    pub fn mtk_view_notify_delegate_draw(view: *mut c_void);
}
