# MetalKit v0.2.0 coverage audit

This crate targets the MetalKit surface requested for `v0.2.0`: `MTKView`, `MTKTextureLoader`, `MTKMesh`, `MTKMeshBuffer`, `MTKSubmesh`, `MTKMeshBufferAllocator`, and the Model I/O integration APIs in `MTKModel.h`.

## Legend

- ✅ Implemented in the safe Rust API
- 🟡 Implemented, but exposed as a raw-pointer or JSON-backed helper because the lower-level dependency crate does not yet provide a typed wrapper
- ⚪ Audited but intentionally not wrapped in `v0.2.0`

## `MTKView.h`

| Header surface | Status | Notes |
| --- | --- | --- |
| `initWithFrame:device:` | ✅ | `View::new` |
| `delegate` / `setDelegate:` / `MTKViewDelegate` callbacks | ✅ | `ViewDelegate`, closure trampolines, explicit notify helpers for deterministic tests |
| `device` / `setDevice:` | ✅ | Raw Metal device pointer access |
| `currentDrawable` | 🟡 | Raw pointer getter |
| `framebufferOnly` | ✅ | Getter/setter |
| `depthStencilAttachmentTextureUsage` | ✅ | Getter/setter |
| `multisampleColorAttachmentTextureUsage` | ✅ | Getter/setter |
| `presentsWithTransaction` | ✅ | Getter/setter |
| `colorPixelFormat` | ✅ | Getter/setter |
| `depthStencilPixelFormat` | ✅ | Getter/setter |
| `depthStencilStorageMode` | ✅ | Getter/setter with macOS availability guard |
| `sampleCount` | ✅ | Getter/setter |
| `clearColor` / `clearDepth` / `clearStencil` | ✅ | Getter/setter |
| `depthStencilTexture` / `multisampleColorTexture` | 🟡 | Raw pointer getters |
| `releaseDrawables` | ✅ | `View::release_drawables` |
| `currentRenderPassDescriptor` | 🟡 | Raw pointer getter |
| `currentMTL4RenderPassDescriptor` | 🟡 | Raw pointer getter, macOS 26+ availability guard |
| `preferredFramesPerSecond` | ✅ | Getter/setter |
| `enableSetNeedsDisplay` | ✅ | Getter/setter |
| `autoResizeDrawable` | ✅ | Getter/setter |
| `drawableSize` | ✅ | Getter/setter |
| `preferredDrawableSize` | ✅ | Getter |
| `preferredDevice` | 🟡 | Raw pointer getter |
| `paused` | ✅ | Getter/setter |
| `colorspace` | ✅ | Getter/setter via `apple-cf::cg::CGColorSpace` |
| `draw` | ✅ | `View::draw` |
| keyed archiving support | ✅ | `View::archive_round_trip` |

## `MTKTextureLoader.h`

| Header surface | Status | Notes |
| --- | --- | --- |
| error domain / error key constants | ✅ | `texture_loader_error` |
| option constants: mipmaps / sRGB / usage / storage / CPU cache / cube layout / origin / load-as-array | ✅ | `TextureLoaderOptions` + constant modules |
| `newTextureWithContentsOfURL:options:error:` | ✅ | `new_texture_from_url` |
| `newTexturesWithContentsOfURLs:options:error:` | ✅ | `new_textures_from_urls` with per-element success/failure reporting |
| `newTextureWithData:options:error:` | ✅ | `new_texture_from_data` |
| `newTextureWithCGImage:options:error:` | ✅ | `new_texture_from_cgimage` |
| `newTextureWithMDLTexture:options:error:` | ✅ | `new_texture_from_model_texture` |
| `newTextureWithName:scaleFactor:bundle:options:error:` | ✅ | `new_texture_named` |
| `newTextureWithName:scaleFactor:displayGamut:bundle:options:error:` | ✅ | `new_texture_named_with_display_gamut` |
| array-of-names helpers | ✅ | `new_textures_named*`, represented as `TextureLoaderArrayOutcome` |
| completion-handler variants | ⚪ | Audited but not wrapped in `v0.2.0`; current crate focuses on the synchronous loading surface |

## `MTKModel.h` mesh bridge + Model I/O integration

| Header surface | Status | Notes |
| --- | --- | --- |
| `MTKMeshBufferAllocator` init/device/newBuffer APIs | ✅ | `MeshBufferAllocator` |
| `MTKMeshBuffer` allocator/zone/length/offset/type/buffer/name | ✅ | `MeshBuffer` |
| `MTKSubmesh` primitive/index/indexBuffer/indexCount/mesh/name | ✅ | `Submesh` |
| `MTKMesh(mesh:device:)` | ✅ | `Mesh::from_model_mesh` |
| `+newMeshesFromAsset:device:sourceMeshes:error:` | ✅ | `Mesh::new_meshes_from_asset` |
| `MTKMesh` vertexCount/name/vertexBuffers/vertexDescriptor/submeshes | ✅ | `Mesh` getters |
| `MTKModelIOVertexDescriptorFromMetal` | ✅ | `model_io_vertex_descriptor_from_metal` |
| `MTKModelIOVertexDescriptorFromMetalWithError` | ✅ | `try_model_io_vertex_descriptor_from_metal` |
| `MTKMetalVertexDescriptorFromModelIO` | ✅ | `metal_vertex_descriptor_from_model_io` |
| `MTKMetalVertexDescriptorFromModelIOWithError` | ✅ | `try_metal_vertex_descriptor_from_model_io` |
| `MTKModelIOVertexFormatFromMetal` | ✅ | `model_io_vertex_format_from_metal` |
| `MTKMetalVertexFormatFromModelIO` | ✅ | `metal_vertex_format_from_model_io` |
| supporting Model I/O helpers (`MDLAsset`, `MDLMesh`, `MDLURLTexture`) | ✅ | `ModelAsset`, `ModelMesh`, `ModelTexture` |
| typed Metal vertex-descriptor wrapper | 🟡 | JSON-backed descriptor inspection because `apple-metal-rs` does not yet expose `MTLVertexDescriptor` |

## Verification

Validated locally with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --example 01_texture_loader_smoke
cargo run --example 02_mesh_bridge_smoke
cargo run --example 03_model_io_integration_smoke
cargo run --example 04_mesh_buffer_allocator_smoke
cargo run --example 05_view_smoke
```
