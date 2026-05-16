mod common;

use apple_cf::cg::CGContext;
use metalkit::{
    texture_loader_error, DisplayGamut, ModelTexture, TextureLoader, TextureLoaderArrayCallback,
    TextureLoaderCallback, TextureLoaderError, TextureLoaderOptions,
};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

const CORE_TYPES_BUNDLE: &str = "/System/Library/CoreServices/CoreTypes.bundle";
const CALLBACK_TIMEOUT: Duration = Duration::from_secs(10);

#[test]
fn texture_loader_handles_urls_data_cgimages_mdltextures_and_array_failures() {
    let _: Option<TextureLoaderCallback> = None;
    let _: Option<TextureLoaderArrayCallback> = None;

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

    assert_eq!(TextureLoaderError::DOMAIN.as_str(), texture_loader_error::DOMAIN);
    assert_eq!(TextureLoaderError::KEY.as_str(), texture_loader_error::KEY);
    assert!(from_url.width() > 0);
    assert_eq!(from_url.width(), from_data.width());
    assert_eq!(from_url.height(), from_data.height());
    assert_eq!(from_url.width(), from_model_texture.width());
    assert!(from_cg_image.width() > 0);

    let url_array = loader
        .new_textures_from_urls(
            &[
                Path::new(common::SYSTEM_ICON),
                Path::new("/definitely/missing-texture.png"),
            ],
            Some(&options),
        )
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
        .new_textures_named(
            &["missing-a", "missing-b"],
            1.0,
            Some(Path::new(CORE_TYPES_BUNDLE)),
            None,
        )
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

#[test]
fn texture_loader_completion_handlers_load_urls_data_cgimages_and_mdltextures() {
    let device = common::device();
    let loader = TextureLoader::new(&device).expect("texture loader");
    let options = TextureLoaderOptions::new()
        .with_srgb(true)
        .with_generate_mipmaps(false)
        .with_allocate_mipmaps(false);
    let icon_bytes = std::fs::read(common::SYSTEM_ICON).expect("icon bytes");
    let model_texture = ModelTexture::from_url(common::SYSTEM_ICON, Some("icon")).expect("model texture");

    let context = CGContext::new_rgba8(2, 2).expect("bitmap context");
    context.set_rgb_fill_color(1.0, 0.0, 0.0, 1.0);
    context.fill_rect(0.0, 0.0, 2.0, 2.0);
    let cg_image = context.snapshot_to_image().expect("cg image");

    let from_url = receive_texture_size_result(|tx| {
        loader
            .new_texture_from_url_with_callback(common::SYSTEM_ICON, Some(&options), move |result| {
                tx.send(result.map(|texture| (texture.width(), texture.height())).map_err(|error| error.to_string()))
                    .expect("send url callback result");
            })
            .expect("schedule url callback");
    })
    .expect("url callback outcome");
    assert!(from_url.0 > 0);
    assert!(from_url.1 > 0);

    let from_data = receive_texture_size_result(|tx| {
        loader
            .new_texture_from_data_with_callback(&icon_bytes, Some(&options), move |result| {
                tx.send(result.map(|texture| (texture.width(), texture.height())).map_err(|error| error.to_string()))
                    .expect("send data callback result");
            })
            .expect("schedule data callback");
    })
    .expect("data callback outcome");
    assert_eq!(from_url, from_data);

    let from_cgimage = receive_texture_size_result(|tx| {
        loader
            .new_texture_from_cgimage_with_callback(&cg_image, Some(&options), move |result| {
                tx.send(result.map(|texture| (texture.width(), texture.height())).map_err(|error| error.to_string()))
                    .expect("send cgimage callback result");
            })
            .expect("schedule cgimage callback");
    })
    .expect("cgimage callback outcome");
    assert!(from_cgimage.0 > 0);
    assert!(from_cgimage.1 > 0);

    let from_model_texture = receive_texture_size_result(|tx| {
        loader
            .new_texture_from_model_texture_with_callback(&model_texture, Some(&options), move |result| {
                tx.send(result.map(|texture| (texture.width(), texture.height())).map_err(|error| error.to_string()))
                    .expect("send model texture callback result");
            })
            .expect("schedule model texture callback");
    })
    .expect("model texture callback outcome");
    assert_eq!(from_url, from_model_texture);
}

#[test]
fn texture_loader_completion_handlers_surface_array_and_named_errors() {
    let device = common::device();
    let loader = TextureLoader::new(&device).expect("texture loader");
    let options = TextureLoaderOptions::new()
        .with_srgb(true)
        .with_generate_mipmaps(false)
        .with_allocate_mipmaps(false);

    let (url_statuses, url_error) = receive_array_outcome(|tx| {
        loader
            .new_textures_from_urls_with_callback(
                &[
                    Path::new(common::SYSTEM_ICON),
                    Path::new("/definitely/missing-texture.png"),
                ],
                Some(&options),
                move |outcome| {
                    tx.send((
                        outcome.textures.iter().map(Option::is_some).collect(),
                        outcome.error.map(|error| error.to_string()),
                    ))
                    .expect("send url array callback result");
                },
            )
            .expect("schedule url array callback");
    });
    assert_eq!(url_statuses, vec![true, false]);
    assert!(url_error.is_some());

    let named_error = receive_texture_size_result(|tx| {
        loader
            .new_texture_named_with_callback(
                "missing-texture",
                1.0,
                Some(Path::new(CORE_TYPES_BUNDLE)),
                None,
                move |result| {
                    tx.send(result.map(|texture| (texture.width(), texture.height())).map_err(|error| error.to_string()))
                        .expect("send named callback result");
                },
            )
            .expect("schedule named callback");
    })
    .expect_err("named callback should fail");
    assert!(!named_error.is_empty());

    let named_gamut_error = receive_texture_size_result(|tx| {
        loader
            .new_texture_named_with_display_gamut_with_callback(
                "missing-texture",
                1.0,
                DisplayGamut::SRGB,
                Some(Path::new(CORE_TYPES_BUNDLE)),
                None,
                move |result| {
                    tx.send(result.map(|texture| (texture.width(), texture.height())).map_err(|error| error.to_string()))
                        .expect("send named gamut callback result");
                },
            )
            .expect("schedule named gamut callback");
    })
    .expect_err("named gamut callback should fail");
    assert!(!named_gamut_error.is_empty());

    let (named_statuses, named_array_error) = receive_array_outcome(|tx| {
        loader
            .new_textures_named_with_callback(
                &["missing-a", "missing-b"],
                1.0,
                Some(Path::new(CORE_TYPES_BUNDLE)),
                None,
                move |outcome| {
                    tx.send((
                        outcome.textures.iter().map(Option::is_some).collect(),
                        outcome.error.map(|error| error.to_string()),
                    ))
                    .expect("send named array callback result");
                },
            )
            .expect("schedule named array callback");
    });
    assert_eq!(named_statuses, vec![false, false]);
    assert!(named_array_error.is_some());

    let (named_gamut_statuses, named_gamut_array_error) = receive_array_outcome(|tx| {
        loader
            .new_textures_named_with_display_gamut_with_callback(
                &["missing-a", "missing-b"],
                1.0,
                DisplayGamut::P3,
                Some(Path::new(CORE_TYPES_BUNDLE)),
                None,
                move |outcome| {
                    tx.send((
                        outcome.textures.iter().map(Option::is_some).collect(),
                        outcome.error.map(|error| error.to_string()),
                    ))
                    .expect("send named gamut array callback result");
                },
            )
            .expect("schedule named gamut array callback");
    });
    assert_eq!(named_gamut_statuses, vec![false, false]);
    assert!(named_gamut_array_error.is_some());
}

fn receive_texture_size_result<F>(
    schedule: F,
) -> Result<(usize, usize), String>
where
    F: FnOnce(mpsc::Sender<Result<(usize, usize), String>>),
{
    let (tx, rx) = mpsc::channel();
    schedule(tx);
    rx.recv_timeout(CALLBACK_TIMEOUT)
        .expect("texture callback timed out")
}

fn receive_array_outcome<F>(schedule: F) -> (Vec<bool>, Option<String>)
where
    F: FnOnce(mpsc::Sender<(Vec<bool>, Option<String>)>),
{
    let (tx, rx) = mpsc::channel();
    schedule(tx);
    rx.recv_timeout(CALLBACK_TIMEOUT)
        .expect("texture array callback timed out")
}
