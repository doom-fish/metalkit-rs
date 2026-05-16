use apple_cf::cg::CGColorSpace;
use apple_metal::{pixel_format, MetalDevice};
use metalkit::{ClearColor, Rect, Size, View, ViewDelegate, ViewDelegateCallbacks};
use std::error::Error;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn Error>> {
    let device = MetalDevice::system_default().expect("no Metal device available");
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

    view.set_color_pixel_format(pixel_format::BGRA8UNORM_SRGB);
    view.set_drawable_size(Size {
        width: 128.0,
        height: 72.0,
    });
    view.set_clear_color(ClearColor {
        red: 0.1,
        green: 0.2,
        blue: 0.3,
        alpha: 1.0,
    });
    view.set_colorspace(Some(&CGColorSpace::srgb()));

    let draw_count = Arc::new(Mutex::new(0usize));
    let delegate = ViewDelegate::new(ViewDelegateCallbacks::new().on_draw({
        let draw_count = Arc::clone(&draw_count);
        move |_| {
            *draw_count.lock().expect("draw lock") += 1;
        }
    }))
    .expect("delegate");
    view.set_delegate(Some(&delegate));
    view.notify_delegate_draw();

    assert_eq!(*draw_count.lock().expect("draw count"), 1);
    assert_eq!(view.color_pixel_format(), pixel_format::BGRA8UNORM_SRGB);
    assert_eq!(view.drawable_size(), Size { width: 128.0, height: 72.0 });

    let cloned_view = view.archive_round_trip()?;
    assert_eq!(cloned_view.color_pixel_format(), pixel_format::BGRA8UNORM_SRGB);
    assert!(cloned_view.drawable_size().width > 0.0);
    assert!(cloned_view.drawable_size().height > 0.0);

    println!("✅ metalkit view bridge OK");
    Ok(())
}
