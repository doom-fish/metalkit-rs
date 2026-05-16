mod common;

use apple_cf::cg::CGContext;
use metalkit::{DisplayGamut, ModelTexture, TextureLoader, TextureLoaderOptions};
use std::path::Path;

const CORE_TYPES_BUNDLE: &str = "/System/Library/CoreServices/CoreTypes.bundle";

#[test]
fn texture_loader_handles_urls_data_cgimages_mdltextures_and_array_failures() {
    let device = common::device();
    let loader = TextureLoader::new(&device).expect("texture loader");
    let options = TextureLoaderOptions::new()
        .with_srgb(true)
        .with_generate_mipmaps(false)
        .with_allocate_mipmaps(false);

    let from_url = loader
        .new_texture_from_url(common::SYSTEM_ICON, Some(&options))
        .expect("texture from url");
    let from_data = loader
        .new_texture_from_data(&std::fs::read(common::SYSTEM_ICON).expect("icon bytes"), Some(&options))
        .expect("texture from data");
    let from_model_texture = loader
        .new_texture_from_model_texture(
            &ModelTexture::from_url(common::SYSTEM_ICON, Some("icon")).expect("model texture"),
            Some(&options),
        )
        .expect("texture from mdl texture");

    let context = CGContext::new_rgba8(2, 2).expect("bitmap context");
    context.set_rgb_fill_color(1.0, 0.0, 0.0, 1.0);
    context.fill_rect(0.0, 0.0, 2.0, 2.0);
    let cg_image = context.snapshot_to_image().expect("cg image");
    let from_cg_image = loader
        .new_texture_from_cgimage(&cg_image, Some(&options))
        .expect("texture from cgimage");

    assert!(from_url.width() > 0);
    assert_eq!(from_url.width(), from_data.width());
    assert_eq!(from_url.height(), from_data.height());
    assert_eq!(from_url.width(), from_model_texture.width());
    assert!(from_cg_image.width() > 0);

    let url_array = loader
        .new_textures_from_urls(&[Path::new(common::SYSTEM_ICON), Path::new("/definitely/missing-texture.png")], Some(&options))
        .expect("texture array from urls");
    assert_eq!(url_array.textures.len(), 2);
    assert!(url_array.textures[0].is_some());
    assert!(url_array.textures[1].is_none());
    assert!(url_array.error.is_some());

    assert!(loader
        .new_texture_named("missing-texture", 1.0, Some(Path::new(CORE_TYPES_BUNDLE)), None)
        .is_err());
    assert!(loader
        .new_texture_named_with_display_gamut(
            "missing-texture",
            1.0,
            DisplayGamut::SRGB,
            Some(Path::new(CORE_TYPES_BUNDLE)),
            None,
        )
        .is_err());

    let named_array = loader
        .new_textures_named(&["missing-a", "missing-b"], 1.0, Some(Path::new(CORE_TYPES_BUNDLE)), None)
        .expect("named texture array");
    assert_eq!(named_array.textures.len(), 2);
    assert!(named_array.textures.iter().all(Option::is_none));
    assert!(named_array.error.is_some());

    let gamut_array = loader
        .new_textures_named_with_display_gamut(
            &["missing-a", "missing-b"],
            1.0,
            DisplayGamut::P3,
            Some(Path::new(CORE_TYPES_BUNDLE)),
            None,
        )
        .expect("gamut texture array");
    assert_eq!(gamut_array.textures.len(), 2);
    assert!(gamut_array.textures.iter().all(Option::is_none));
    assert!(gamut_array.error.is_some());
}
