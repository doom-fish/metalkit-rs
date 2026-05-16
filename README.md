# metalkit-rs

Safe `Rust` bindings for Apple's [`MetalKit`](https://developer.apple.com/documentation/metalkit) framework on `macOS`.

> **Status:** v0.2.1 expands the crate across `MTKView`, `MTKTextureLoader`, `MTKMesh`, `MTKMeshBuffer`, `MTKSubmesh`, `MTKMeshBufferAllocator`, and `Model I/O` integration helpers used by `MTKModel.h`.

## What v0.2.1 covers

- `View` + `ViewDelegate` wrappers for `MTKView`
- `TextureLoader` sync and completion-handler loading from URLs, data, `CGImage`, `MDLTexture`, plus named-texture and batch failure handling helpers
- `MeshBufferAllocator`, `MeshBuffer`, `Submesh`, and `Mesh` wrappers for `MetalKit` mesh bridging
- `ModelAsset`, `ModelMesh`, `ModelTexture`, and vertex-descriptor conversion helpers for `Model I/O` integration
- Raw-pointer accessors for `Metal` types that `apple-metal-rs` does not wrap yet (`CAMetalDrawable`, render-pass descriptors, preferred device)

See [`COVERAGE.md`](COVERAGE.md) for the audited header-level checklist.

## Requirements

- `macOS`
- Xcode command-line tools / `Swift` toolchain
- `Rust` 1.76+

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

## Notes

- `MTKView` render-pass descriptors and drawables are exposed as raw pointers for now because `apple-metal-rs` does not yet ship typed wrappers for those `Metal` objects.
- `TextureLoader` exposes both synchronous and completion-handler APIs; see [`COVERAGE.md`](COVERAGE.md) for the exact header-level audit status.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
