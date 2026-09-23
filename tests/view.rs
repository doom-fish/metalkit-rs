mod common;

use apple_cf::cg::CGColorSpace;
use apple_metal::{pixel_format, storage_mode, texture_usage};
use core::ffi::c_void;
use metalkit::{ClearColor, Rect, Size, View, ViewDelegate, ViewDelegateCallbacks};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const FRAME: Rect = Rect {
    x: 0.0,
    y: 0.0,
    width: 64.0,
    height: 64.0,
};

extern "C" {
    fn objc_retain(object: *mut c_void) -> *mut c_void;
    fn objc_release(object: *mut c_void);
}

fn approx_eq(left: f64, right: f64) {
    assert!(
        (left - right).abs() < f64::EPSILON,
        "left={left}, right={right}"
    );
}

#[allow(clippy::too_many_lines)]
fn view_exposes_properties_and_delegate_callbacks() {
    let device = common::device();
    let view = View::new(FRAME, Some(&device)).expect("view");

    assert_eq!(
        view.device().map(|device| device.registry_id()),
        Some(device.registry_id())
    );

    view.set_framebuffer_only(false);
    assert!(!view.framebuffer_only());
    view.set_framebuffer_only(true);
    assert!(view.framebuffer_only());

    view.set_depth_stencil_attachment_texture_usage(
        texture_usage::SHADER_READ | texture_usage::RENDER_TARGET,
    );
    assert_eq!(
        view.depth_stencil_attachment_texture_usage(),
        texture_usage::SHADER_READ | texture_usage::RENDER_TARGET
    );
    view.set_multisample_color_attachment_texture_usage(texture_usage::RENDER_TARGET);
    assert_eq!(
        view.multisample_color_attachment_texture_usage(),
        texture_usage::RENDER_TARGET
    );

    view.set_presents_with_transaction(true);
    assert!(view.presents_with_transaction());

    view.set_color_pixel_format(pixel_format::BGRA8UNORM_SRGB);
    view.set_depth_stencil_pixel_format(pixel_format::DEPTH32FLOAT);
    view.set_depth_stencil_storage_mode(storage_mode::PRIVATE);
    view.set_sample_count(4);
    view.set_clear_color(ClearColor {
        red: 0.25,
        green: 0.5,
        blue: 0.75,
        alpha: 1.0,
    });
    view.set_clear_depth(0.5);
    view.set_clear_stencil(7);
    view.set_preferred_frames_per_second(30);
    view.set_enable_set_needs_display(true);
    view.set_auto_resize_drawable(false);
    view.set_drawable_size(Size {
        width: 128.0,
        height: 72.0,
    });
    view.set_paused(true);
    let color_space = CGColorSpace::display_p3();
    view.set_colorspace(Some(&color_space));

    assert_eq!(view.color_pixel_format(), pixel_format::BGRA8UNORM_SRGB);
    assert_eq!(
        view.depth_stencil_pixel_format(),
        pixel_format::DEPTH32FLOAT
    );
    assert_eq!(view.depth_stencil_storage_mode(), storage_mode::PRIVATE);
    assert_eq!(view.sample_count(), 4);
    approx_eq(view.clear_depth(), 0.5);
    assert_eq!(view.clear_stencil(), 7);
    assert_eq!(view.preferred_frames_per_second(), 30);
    assert!(view.enable_set_needs_display());
    assert!(!view.auto_resize_drawable());
    assert_eq!(
        view.drawable_size(),
        Size {
            width: 128.0,
            height: 72.0
        }
    );
    assert!(view.is_paused());
    assert_eq!(
        view.colorspace().map(|space| space.number_of_components()),
        Some(3)
    );
    assert!(view.preferred_device().is_some());

    let clear_color = view.clear_color();
    approx_eq(clear_color.red, 0.25);
    approx_eq(clear_color.green, 0.5);
    approx_eq(clear_color.blue, 0.75);
    approx_eq(clear_color.alpha, 1.0);

    let draw_count = Arc::new(Mutex::new(0usize));
    let resize_events = Arc::new(Mutex::new(Vec::<Size>::new()));
    let delegate = ViewDelegate::new(
        ViewDelegateCallbacks::new()
            .on_draw({
                let draw_count = Arc::clone(&draw_count);
                move |_| {
                    *draw_count.lock().expect("draw lock") += 1;
                }
            })
            .on_drawable_size_will_change({
                let resize_events = Arc::clone(&resize_events);
                move |_, size| {
                    resize_events.lock().expect("resize lock").push(size);
                }
            }),
    )
    .expect("delegate");
    view.set_delegate(Some(&delegate));
    assert!(!view.delegate_ptr().is_null());
    assert_eq!(view.delegate_ptr(), view.delegate_ptr());

    view.notify_delegate_drawable_size_will_change();
    view.notify_delegate_draw();
    view.draw();
    view.release_drawables();

    assert!(*draw_count.lock().expect("draw count") >= 1);
    assert_eq!(
        resize_events.lock().expect("resize events").as_slice(),
        &[Size {
            width: 128.0,
            height: 72.0
        }]
    );

    let cloned_view = view.archive_round_trip().expect("archive round trip");
    assert_eq!(
        cloned_view.color_pixel_format(),
        pixel_format::BGRA8UNORM_SRGB
    );
}

fn drawing_resources_are_typed_owned_handles() {
    let device = common::device();
    let view = View::new(FRAME, Some(&device)).expect("view");
    view.set_auto_resize_drawable(false);
    view.set_drawable_size(Size {
        width: 32.0,
        height: 16.0,
    });
    view.set_depth_stencil_pixel_format(pixel_format::DEPTH32FLOAT);

    let descriptor = view
        .current_render_pass_descriptor()
        .expect("render pass descriptor");
    assert!(!descriptor.as_ptr().is_null());
    let drawable = view.current_drawable().expect("drawable");
    assert!(!drawable.as_ptr().is_null());
    let depth = view.depth_stencil_texture().expect("depth texture");
    assert_eq!(depth.pixel_format(), pixel_format::DEPTH32FLOAT);
    assert_eq!((depth.width(), depth.height()), (32, 16));
    drop(view);
    assert_eq!(depth.pixel_format(), pixel_format::DEPTH32FLOAT);
    assert!(!descriptor.as_ptr().is_null());

    let msaa_view = View::new(FRAME, Some(&device)).expect("view");
    msaa_view.set_auto_resize_drawable(false);
    msaa_view.set_drawable_size(Size {
        width: 8.0,
        height: 8.0,
    });
    msaa_view.set_sample_count(4);
    assert!(msaa_view.current_render_pass_descriptor().is_some());
    let multisample = msaa_view
        .multisample_color_texture()
        .expect("multisample texture");
    assert_eq!((multisample.width(), multisample.height()), (8, 8));

    match msaa_view.current_mtl4_render_pass_descriptor() {
        Ok(descriptor) => {
            if let Some(descriptor) = descriptor {
                assert!(!descriptor.as_ptr().is_null());
            }
        }
        Err(error) => assert!(error.to_string().contains("macOS 26.0")),
    }
}

fn dropped_delegate_is_never_called_through_a_leaked_box() {
    let view = View::new(FRAME, Some(&common::device())).expect("view");
    let calls = Arc::new(AtomicUsize::new(0));
    let delegate = ViewDelegate::new(ViewDelegateCallbacks::new().on_draw({
        let calls = Arc::clone(&calls);
        move |_| {
            calls.fetch_add(1, Ordering::SeqCst);
        }
    }))
    .expect("delegate");
    view.set_delegate(Some(&delegate));
    let leaked = unsafe { objc_retain(view.delegate_ptr()) };
    assert!(!leaked.is_null());
    view.notify_delegate_draw();
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    drop(delegate);
    view.notify_delegate_draw();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(Arc::strong_count(&calls), 2);

    unsafe { objc_release(leaked) };
    assert_eq!(Arc::strong_count(&calls), 1);
    view.notify_delegate_draw();
    assert!(view.delegate_ptr().is_null());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

fn reentrant_draw_is_skipped() {
    let view = View::new(FRAME, Some(&common::device())).expect("view");
    let calls = Arc::new(AtomicUsize::new(0));
    let delegate = ViewDelegate::new(ViewDelegateCallbacks::new().on_draw({
        let calls = Arc::clone(&calls);
        move |view| {
            if calls.fetch_add(1, Ordering::SeqCst) == 0 {
                view.notify_delegate_draw();
            }
        }
    }))
    .expect("delegate");
    view.set_delegate(Some(&delegate));

    view.notify_delegate_draw();

    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

fn views_are_refused_off_the_main_thread() {
    let error = std::thread::spawn(|| View::new(FRAME, None).map(drop))
        .join()
        .expect("worker")
        .expect_err("off-main view");
    assert!(error.to_string().contains("main thread"), "{error}");
}

fn main() {
    assert!(unsafe { libc::pthread_main_np() } != 0);
    view_exposes_properties_and_delegate_callbacks();
    drawing_resources_are_typed_owned_handles();
    dropped_delegate_is_never_called_through_a_leaked_box();
    reentrant_draw_is_skipped();
    views_are_refused_off_the_main_thread();
    println!("view: 5 checks passed");
}
