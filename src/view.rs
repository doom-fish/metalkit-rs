use crate::error::MetalKitError;
use crate::ffi;
use crate::private::{handle_type, take_error};
use apple_cf::cg::CGColorSpace;
use apple_metal::MetalDevice;
use core::ffi::c_void;
use std::ptr;

handle_type!(View);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

type DrawCallback = Box<dyn FnMut(&View)>;
type ResizeCallback = Box<dyn FnMut(&View, Size)>;

#[derive(Default)]
pub struct ViewDelegateCallbacks {
    draw: Option<DrawCallback>,
    resize: Option<ResizeCallback>,
}

impl ViewDelegateCallbacks {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            draw: None,
            resize: None,
        }
    }

    #[must_use]
    pub fn on_draw<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&View) + 'static,
    {
        self.draw = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_drawable_size_will_change<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&View, Size) + 'static,
    {
        self.resize = Some(Box::new(callback));
        self
    }
}

struct DelegateState {
    draw: Option<DrawCallback>,
    resize: Option<ResizeCallback>,
}

pub struct ViewDelegate {
    ptr: *mut c_void,
    state: *mut DelegateState,
}

impl core::fmt::Debug for ViewDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ViewDelegate")
            .field("ptr", &self.ptr)
            .field("state", &self.state)
            .finish()
    }
}

impl Drop for ViewDelegate {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::mtk_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
        if !self.state.is_null() {
            unsafe { drop(Box::from_raw(self.state)) };
            self.state = ptr::null_mut();
        }
    }
}

unsafe extern "C" fn view_draw_trampoline(user_data: *mut c_void, view: *mut c_void) {
    if user_data.is_null() || view.is_null() {
        return;
    }

    let state = unsafe { &mut *user_data.cast::<DelegateState>() };
    if let Some(callback) = state.draw.as_mut() {
        let view = unsafe { View::from_raw_borrowed(view) };
        callback(&view);
    }
}

unsafe extern "C" fn view_resize_trampoline(
    user_data: *mut c_void,
    view: *mut c_void,
    width: f64,
    height: f64,
) {
    if user_data.is_null() || view.is_null() {
        return;
    }

    let state = unsafe { &mut *user_data.cast::<DelegateState>() };
    if let Some(callback) = state.resize.as_mut() {
        let view = unsafe { View::from_raw_borrowed(view) };
        callback(&view, Size { width, height });
    }
}

impl ViewDelegate {
    #[must_use]
    pub fn new(callbacks: ViewDelegateCallbacks) -> Option<Self> {
        let state = Box::new(DelegateState {
            draw: callbacks.draw,
            resize: callbacks.resize,
        });
        let state_ptr = Box::into_raw(state);
        let ptr = unsafe {
            ffi::mtk_view_delegate_new(
                (*state_ptr)
                    .draw
                    .as_ref()
                    .map(|_| view_draw_trampoline as ffi::ViewDrawCallback),
                (*state_ptr)
                    .resize
                    .as_ref()
                    .map(|_| view_resize_trampoline as ffi::ViewResizeCallback),
                state_ptr.cast::<c_void>(),
            )
        };
        if ptr.is_null() {
            unsafe { drop(Box::from_raw(state_ptr)) };
            None
        } else {
            Some(Self {
                ptr,
                state: state_ptr,
            })
        }
    }

    #[must_use]
    const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl View {
    #[must_use]
    pub fn new(frame: Rect, device: Option<&MetalDevice>) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::mtk_view_new(
                frame.x,
                frame.y,
                frame.width,
                frame.height,
                device.map_or(ptr::null_mut(), MetalDevice::as_ptr),
            ))
        }
    }

    pub fn archive_round_trip(&self) -> Result<Self, MetalKitError> {
        let mut error = ptr::null_mut();
        let view = unsafe { ffi::mtk_view_archive_round_trip(self.as_ptr(), ptr::addr_of_mut!(error)) };
        if view.is_null() {
            Err(take_error(error, "failed to archive and unarchive MTKView"))
        } else {
            Ok(unsafe { Self::from_raw_unchecked(view) })
        }
    }

    #[must_use]
    pub fn delegate_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_view_delegate(self.as_ptr()) }
    }

    pub fn set_delegate(&self, delegate: Option<&ViewDelegate>) {
        unsafe {
            ffi::mtk_view_set_delegate(
                self.as_ptr(),
                delegate.map_or(ptr::null_mut(), ViewDelegate::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn device_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_view_device(self.as_ptr()) }
    }

    pub fn set_device(&self, device: Option<&MetalDevice>) {
        unsafe {
            ffi::mtk_view_set_device(
                self.as_ptr(),
                device.map_or(ptr::null_mut(), MetalDevice::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn current_drawable_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_view_current_drawable(self.as_ptr()) }
    }

    #[must_use]
    pub fn framebuffer_only(&self) -> bool {
        unsafe { ffi::mtk_view_framebuffer_only(self.as_ptr()) }
    }

    pub fn set_framebuffer_only(&self, value: bool) {
        unsafe { ffi::mtk_view_set_framebuffer_only(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn depth_stencil_attachment_texture_usage(&self) -> usize {
        unsafe { ffi::mtk_view_depth_stencil_attachment_texture_usage(self.as_ptr()) }
    }

    pub fn set_depth_stencil_attachment_texture_usage(&self, value: usize) {
        unsafe { ffi::mtk_view_set_depth_stencil_attachment_texture_usage(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn multisample_color_attachment_texture_usage(&self) -> usize {
        unsafe { ffi::mtk_view_multisample_color_attachment_texture_usage(self.as_ptr()) }
    }

    pub fn set_multisample_color_attachment_texture_usage(&self, value: usize) {
        unsafe {
            ffi::mtk_view_set_multisample_color_attachment_texture_usage(self.as_ptr(), value)
        };
    }

    #[must_use]
    pub fn presents_with_transaction(&self) -> bool {
        unsafe { ffi::mtk_view_presents_with_transaction(self.as_ptr()) }
    }

    pub fn set_presents_with_transaction(&self, value: bool) {
        unsafe { ffi::mtk_view_set_presents_with_transaction(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn color_pixel_format(&self) -> usize {
        unsafe { ffi::mtk_view_color_pixel_format(self.as_ptr()) }
    }

    pub fn set_color_pixel_format(&self, value: usize) {
        unsafe { ffi::mtk_view_set_color_pixel_format(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn depth_stencil_pixel_format(&self) -> usize {
        unsafe { ffi::mtk_view_depth_stencil_pixel_format(self.as_ptr()) }
    }

    pub fn set_depth_stencil_pixel_format(&self, value: usize) {
        unsafe { ffi::mtk_view_set_depth_stencil_pixel_format(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn depth_stencil_storage_mode(&self) -> usize {
        unsafe { ffi::mtk_view_depth_stencil_storage_mode(self.as_ptr()) }
    }

    pub fn set_depth_stencil_storage_mode(&self, value: usize) {
        unsafe { ffi::mtk_view_set_depth_stencil_storage_mode(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn sample_count(&self) -> usize {
        unsafe { ffi::mtk_view_sample_count(self.as_ptr()) }
    }

    pub fn set_sample_count(&self, value: usize) {
        unsafe { ffi::mtk_view_set_sample_count(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn clear_color(&self) -> ClearColor {
        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;
        let mut alpha = 1.0;
        unsafe {
            ffi::mtk_view_clear_color(
                self.as_ptr(),
                ptr::addr_of_mut!(red),
                ptr::addr_of_mut!(green),
                ptr::addr_of_mut!(blue),
                ptr::addr_of_mut!(alpha),
            )
        };
        ClearColor {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn set_clear_color(&self, value: ClearColor) {
        unsafe {
            ffi::mtk_view_set_clear_color(
                self.as_ptr(),
                value.red,
                value.green,
                value.blue,
                value.alpha,
            )
        };
    }

    #[must_use]
    pub fn clear_depth(&self) -> f64 {
        unsafe { ffi::mtk_view_clear_depth(self.as_ptr()) }
    }

    pub fn set_clear_depth(&self, value: f64) {
        unsafe { ffi::mtk_view_set_clear_depth(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn clear_stencil(&self) -> u32 {
        unsafe { ffi::mtk_view_clear_stencil(self.as_ptr()) }
    }

    pub fn set_clear_stencil(&self, value: u32) {
        unsafe { ffi::mtk_view_set_clear_stencil(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn depth_stencil_texture_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_view_depth_stencil_texture(self.as_ptr()) }
    }

    #[must_use]
    pub fn multisample_color_texture_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_view_multisample_color_texture(self.as_ptr()) }
    }

    pub fn release_drawables(&self) {
        unsafe { ffi::mtk_view_release_drawables(self.as_ptr()) };
    }

    #[must_use]
    pub fn current_render_pass_descriptor_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_view_current_render_pass_descriptor(self.as_ptr()) }
    }

    #[must_use]
    pub fn current_mtl4_render_pass_descriptor_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_view_current_mtl4_render_pass_descriptor(self.as_ptr()) }
    }

    #[must_use]
    pub fn preferred_frames_per_second(&self) -> isize {
        unsafe { ffi::mtk_view_preferred_frames_per_second(self.as_ptr()) }
    }

    pub fn set_preferred_frames_per_second(&self, value: isize) {
        unsafe { ffi::mtk_view_set_preferred_frames_per_second(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn enable_set_needs_display(&self) -> bool {
        unsafe { ffi::mtk_view_enable_set_needs_display(self.as_ptr()) }
    }

    pub fn set_enable_set_needs_display(&self, value: bool) {
        unsafe { ffi::mtk_view_set_enable_set_needs_display(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn auto_resize_drawable(&self) -> bool {
        unsafe { ffi::mtk_view_auto_resize_drawable(self.as_ptr()) }
    }

    pub fn set_auto_resize_drawable(&self, value: bool) {
        unsafe { ffi::mtk_view_set_auto_resize_drawable(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn drawable_size(&self) -> Size {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe {
            ffi::mtk_view_drawable_size(
                self.as_ptr(),
                ptr::addr_of_mut!(width),
                ptr::addr_of_mut!(height),
            )
        };
        Size { width, height }
    }

    pub fn set_drawable_size(&self, value: Size) {
        unsafe { ffi::mtk_view_set_drawable_size(self.as_ptr(), value.width, value.height) };
    }

    #[must_use]
    pub fn preferred_drawable_size(&self) -> Size {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe {
            ffi::mtk_view_preferred_drawable_size(
                self.as_ptr(),
                ptr::addr_of_mut!(width),
                ptr::addr_of_mut!(height),
            )
        };
        Size { width, height }
    }

    #[must_use]
    pub fn preferred_device_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_view_preferred_device(self.as_ptr()) }
    }

    #[must_use]
    pub fn is_paused(&self) -> bool {
        unsafe { ffi::mtk_view_is_paused(self.as_ptr()) }
    }

    pub fn set_paused(&self, value: bool) {
        unsafe { ffi::mtk_view_set_paused(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn colorspace_ptr(&self) -> *mut c_void {
        unsafe { ffi::mtk_view_colorspace(self.as_ptr()) }
    }

    pub fn set_colorspace(&self, value: Option<&CGColorSpace>) {
        unsafe {
            ffi::mtk_view_set_colorspace(
                self.as_ptr(),
                value.map_or(ptr::null_mut(), CGColorSpace::as_ptr),
            )
        };
    }

    pub fn draw(&self) {
        unsafe { ffi::mtk_view_draw(self.as_ptr()) };
    }

    pub fn notify_delegate_drawable_size_will_change(&self) {
        unsafe { ffi::mtk_view_notify_delegate_size_will_change(self.as_ptr()) };
    }

    pub fn notify_delegate_draw(&self) {
        unsafe { ffi::mtk_view_notify_delegate_draw(self.as_ptr()) };
    }
}
