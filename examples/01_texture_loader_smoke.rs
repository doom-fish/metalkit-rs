use apple_cf::cg::CGContext;
use apple_metal::MetalDevice;
use metalkit::{TextureLoader, TextureLoaderOptions};
use std::error::Error;
use std::path::Path;

const SYSTEM_ICON: &str =
    "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/PublicFolderIcon.icns";

fn main() -> Result<(), Box<dyn Error>> {
    let device = MetalDevice::system_default().expect("no Metal device available");
    let loader = TextureLoader::new(&device).expect("failed to create texture loader");
    let options = TextureLoaderOptions::new()
        .with_srgb(true)
        .with_generate_mipmaps(false);

    if Path::new(SYSTEM_ICON).exists() {
        let url_texture = loader.new_texture_from_url(SYSTEM_ICON, Some(&options))?;
        let data_texture =
            loader.new_texture_from_data(&std::fs::read(SYSTEM_ICON)?, Some(&options))?;
        assert!(url_texture.width() > 0);
        assert!(url_texture.height() > 0);
        assert_eq!(url_texture.width(), data_texture.width());
        assert_eq!(url_texture.height(), data_texture.height());
    }

    let context = CGContext::new_rgba8(2, 2)?;
    context.set_rgb_fill_color(1.0, 0.0, 0.0, 1.0);
    context.fill_rect(0.0, 0.0, 1.0, 2.0);
    context.set_rgb_fill_color(0.0, 1.0, 0.0, 1.0);
    context.fill_rect(1.0, 0.0, 1.0, 2.0);
    let cg_image = context
        .snapshot_to_image()
        .expect("failed to snapshot fallback image");
    let cg_texture = loader.new_texture_from_cgimage(&cg_image, Some(&options))?;
    assert!(cg_texture.width() > 0);
    assert!(cg_texture.height() > 0);

    println!("✅ metalkit texture loader OK");
    Ok(())
}
