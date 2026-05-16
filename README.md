# metalkit-rs

Safe Rust bindings for Apple's [MetalKit](https://developer.apple.com/documentation/metalkit) framework on macOS.

> **Status:** v0.1.0 covers `MTKTextureLoader`, the core texture-loader option constants, and the `MTKMesh` / `MTKSubmesh` / `MTKMeshBuffer` / `MTKMeshBufferAllocator` bridge used with Model I/O meshes.

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

## Highlights

- `TextureLoader` for loading Metal textures from URLs, asset names, `Data`, and `CGImage`
- `TextureLoaderOptions` for `generateMipmaps`, `allocateMipmaps`, `SRGB`, `textureUsage`, `textureStorageMode`, and `textureCPUCacheMode`
- `MeshBufferAllocator` and `ModelMesh` helpers for building Model I/O meshes backed by Metal buffers
- `Mesh`, `Submesh`, and `MeshBuffer` wrappers for the `MetalKit` rendering bridge

## Smoke examples

Run the texture-loader smoke test with:

```bash
cargo run --example 01_texture_loader_smoke
```

Run the Model I/O mesh bridge smoke test with:

```bash
cargo run --example 02_mesh_bridge_smoke
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
