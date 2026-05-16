mod common;

use apple_cf::cg::CGColorSpace;
use apple_metal::{pixel_format, storage_mode, texture_usage};
use metalkit::{ClearColor, Rect, Size, View, ViewDelegate, ViewDelegateCallbacks};
use std::sync::{Arc, Mutex};

fn approx_eq(left: f64, right: f64) {
    assert!((left - right).abs() < f64::EPSILON, "left={left}, right={right}");
}

#[test]
fn view_exposes_properties_and_delegate_callbacks() {
    let device = common::device();
    let view = View::new(
        Rect {
            x: 0.0,
            y: 0.0,
            width: 64.0,
            height: 64.0,
        },
        Some(&device),
    )
    .expect("view");

    assert!(!view.device_ptr().is_null());
    let _ = view.current_drawable_ptr();
    let _ = view.current_render_pass_descriptor_ptr();

    view.set_framebuffer_only(false);
    assert!(!view.framebuffer_only());
    view.set_framebuffer_only(true);
    assert!(view.framebuffer_only());

    view.set_depth_stencil_attachment_texture_usage(texture_usage::SHADER_READ | texture_usage::RENDER_TARGET);
    assert_eq!(
        view.depth_stencil_attachment_texture_usage(),
        texture_usage::SHADER_READ | texture_usage::RENDER_TARGET
    );
    view.set_multisample_color_attachment_texture_usage(texture_usage::RENDER_TARGET);
    assert_eq!(view.multisample_color_attachment_texture_usage(), texture_usage::RENDER_TARGET);

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
    assert_eq!(view.depth_stencil_pixel_format(), pixel_format::DEPTH32FLOAT);
    assert_eq!(view.depth_stencil_storage_mode(), storage_mode::PRIVATE);
    assert_eq!(view.sample_count(), 4);
    approx_eq(view.clear_depth(), 0.5);
    assert_eq!(view.clear_stencil(), 7);
    assert_eq!(view.preferred_frames_per_second(), 30);
    assert!(view.enable_set_needs_display());
    assert!(!view.auto_resize_drawable());
    assert_eq!(view.drawable_size(), Size { width: 128.0, height: 72.0 });
    assert!(view.is_paused());
    assert!(!view.colorspace_ptr().is_null());
    assert!(!view.preferred_device_ptr().is_null());

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

    view.notify_delegate_drawable_size_will_change();
    view.notify_delegate_draw();
    view.draw();
    view.release_drawables();

    assert!(*draw_count.lock().expect("draw count") >= 1);
    assert_eq!(resize_events.lock().expect("resize events").as_slice(), &[Size { width: 128.0, height: 72.0 }]);

    let cloned_view = view.archive_round_trip().expect("archive round trip");
    assert_eq!(cloned_view.color_pixel_format(), pixel_format::BGRA8UNORM_SRGB);
}
