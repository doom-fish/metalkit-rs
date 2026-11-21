# metalkit-rs

Safe `Rust` bindings for Apple's [`MetalKit`](https://developer.apple.com/documentation/metalkit) framework on `macOS`.

The crate covers `MTKView`, `MTKTextureLoader`, `MTKMesh`, `MTKMeshBuffer`, `MTKSubmesh`, `MTKMeshBufferAllocator`, and the `Model I/O` integration helpers in `MTKModel.h`.

## What it covers

- `View` + `ViewDelegate` wrappers for `MTKView`. The drawable, render-pass descriptors, depth/stencil and multisample textures, devices and color space come back as owned `apple-metal` / `apple-cf` values. The drawable and render-pass descriptors are `apple-metal` opaque handles without methods yet; use `as_ptr` to hand them to other Metal code.
- `TextureLoader` sync and completion-handler loading from URLs, data, `CGImage`, `MDLTexture`, plus named-texture and batch failure handling helpers
- `MeshBufferAllocator`, `MeshBuffer`, `Submesh`, and `Mesh` wrappers for `MetalKit` mesh bridging
- `ModelAsset`, `ModelMesh`, `ModelTexture`, and vertex-descriptor conversion helpers for `Model I/O` integration

See [`COVERAGE.md`](COVERAGE.md) for the audited header-level checklist. The audit finds a Rust item for all 108 audited symbols; three of them (`MTKTextureLoader.device`, `MTKMeshBufferAllocator.device`, `MTKMeshBuffer.buffer`) are raw pointer getters only.

## Requirements

- `macOS` 11 or later; `View::current_mtl4_render_pass_descriptor` needs macOS 26 and returns an error before that
- Xcode command-line tools / `Swift` toolchain
- `Rust` 1.82+

```toml
[dependencies]
metalkit-rs = "0.4.0"
```

The library is imported as `metalkit`.

## Threading

`MTKView` is an `AppKit` view: `View::new` returns an error off the main thread, and `View` is `!Send`, so every call on it stays on the main thread. `MTKView` calls its delegate on the main thread, and `ViewDelegateCallbacks` closures must be `Send`. A draw that the draw callback triggers on the same view is skipped.

## Quick start

```rust,no_run
use apple_metal::MetalDevice;
use metalkit::{TextureLoader, TextureLoaderOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = MetalDevice::system_default().expect("no Metal device");
    let loader = TextureLoader::new(&device).expect("loader");

    let texture = loader.new_texture_from_url(
        "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/PublicFolderIcon.icns",
        Some(&TextureLoaderOptions::new().with_srgb(true)),
    )?;

    assert!(texture.width() > 0);
    assert!(texture.height() > 0);
    Ok(())
}
```

## Examples

```bash
cargo run --example 01_texture_loader_smoke
cargo run --example 02_mesh_bridge_smoke
cargo run --example 03_model_io_integration_smoke
cargo run --example 04_mesh_buffer_allocator_smoke
cargo run --example 05_view_smoke
```

## Validation

```bash
cargo clippy --all-targets -- -D warnings
cargo test
```

CI uses published crates from crates.io.
The Clippy duplicate dependency exceptions are `apple-cf` and `doom-fish-utils`.
`apple-metal` 0.10.4 needs `doom-fish-utils` 0.4, and its default `iosurface` feature needs `apple-cf` 0.11.
The `iosurface` feature exposes the `IOSurfaceMetalExt` trait.
When the dependency graph has one version of each crate, remove the exceptions.

## Notes

- `apple-metal` can't yet create a render encoder from a render-pass descriptor or present a drawable, so a full `MTKView` draw loop still needs your own Metal code for those two steps.
- `TextureLoader` exposes both synchronous and completion-handler APIs; see [`COVERAGE.md`](COVERAGE.md) for the exact header-level audit status.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
