use apple_cf::cg::CGContext;
use apple_metal::MetalDevice;
use metalkit::{TextureLoader, TextureLoaderOptions};
use std::error::Error;
use std::io;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

const SYSTEM_ICON: &str =
    "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/PublicFolderIcon.icns";
const CALLBACK_TIMEOUT: Duration = Duration::from_secs(10);

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

        let (tx, rx) = mpsc::channel();
        loader.new_texture_from_url_with_callback(SYSTEM_ICON, Some(&options), move |result| {
            tx.send(
                result
                    .map(|texture| (texture.width(), texture.height()))
                    .map_err(|error| error.to_string()),
            )
            .expect("send async texture result");
        })?;
        let async_texture = rx
            .recv_timeout(CALLBACK_TIMEOUT)
            .expect("texture callback timed out")
            .map_err(io::Error::other)?;
        assert_eq!(async_texture, (url_texture.width(), url_texture.height()));

        let (tx, rx) = mpsc::channel();
        loader.new_textures_from_urls_with_callback(
            &[
                Path::new(SYSTEM_ICON),
                Path::new("/definitely/missing-texture.png"),
            ],
            Some(&options),
            move |outcome| {
                tx.send((
                    outcome
                        .textures
                        .iter()
                        .filter(|texture| texture.is_some())
                        .count(),
                    outcome.error.is_some(),
                ))
                .expect("send async texture array result");
            },
        )?;
        let (loaded_count, had_error) = rx
            .recv_timeout(CALLBACK_TIMEOUT)
            .expect("texture array callback timed out");
        assert_eq!(loaded_count, 1);
        assert!(had_error);
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
