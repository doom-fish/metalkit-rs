# MetalKit coverage audit

This crate targets `MTKView`, `MTKTextureLoader`, `MTKMesh`, `MTKMeshBuffer`, `MTKSubmesh`, `MTKMeshBufferAllocator`, and the Model I/O integration APIs in `MTKModel.h`.

## Legend

- ✅ Implemented in the safe Rust API
- 🟡 Implemented, but exposed as a raw-pointer or JSON-backed helper because the lower-level dependency crate does not yet provide a typed wrapper
- ⚪ Audited but intentionally not wrapped

`MTKView` must be used on the main thread: `View::new` returns an error elsewhere, and `View` is `!Send`.

## `MTKView.h`

| Header surface | Status | Notes |
| --- | --- | --- |
| `initWithFrame:device:` | ✅ | `View::new` |
| `delegate` / `setDelegate:` / `MTKViewDelegate` callbacks | ✅ | `ViewDelegate` (`Send` closures in a context the Swift delegate owns), explicit notify helpers for deterministic tests; `delegate_ptr` is a +0 identity pointer |
| `device` / `setDevice:` | ✅ | `View::device` returns an apple-metal `MetalDevice` |
| `currentDrawable` | ✅ | `View::current_drawable` returns an apple-metal `MetalDrawable` (opaque handle; apple-metal has no drawable methods yet) |
| `framebufferOnly` | ✅ | Getter/setter |
| `depthStencilAttachmentTextureUsage` | ✅ | Getter/setter |
| `multisampleColorAttachmentTextureUsage` | ✅ | Getter/setter |
| `presentsWithTransaction` | ✅ | Getter/setter |
| `colorPixelFormat` | ✅ | Getter/setter |
| `depthStencilPixelFormat` | ✅ | Getter/setter |
| `depthStencilStorageMode` | ✅ | Getter/setter with macOS availability guard |
| `sampleCount` | ✅ | Getter/setter |
| `clearColor` / `clearDepth` / `clearStencil` | ✅ | Getter/setter |
| `depthStencilTexture` / `multisampleColorTexture` | ✅ | `MetalTexture` getters |
| `releaseDrawables` | ✅ | `View::release_drawables` |
| `currentRenderPassDescriptor` | ✅ | `View::current_render_pass_descriptor` returns an apple-metal `MetalRenderPassDescriptor` (opaque handle; apple-metal cannot yet build an encoder from it) |
| `currentMTL4RenderPassDescriptor` | ✅ | `Metal4RenderPassDescriptor`; run-time macOS 26 check that returns an error on older systems |
| `preferredFramesPerSecond` | ✅ | Getter/setter |
| `enableSetNeedsDisplay` | ✅ | Getter/setter |
| `autoResizeDrawable` | ✅ | Getter/setter |
| `drawableSize` | ✅ | Getter/setter |
| `preferredDrawableSize` | ✅ | Getter |
| `preferredDevice` | ✅ | `View::preferred_device` returns an apple-metal `MetalDevice` |
| `paused` | ✅ | Getter/setter |
| `colorspace` | ✅ | Getter/setter via `apple-cf::cg::CGColorSpace` (the setter checks the CF type) |
| `draw` | ✅ | `View::draw` |
| keyed archiving support | ✅ | `View::archive_round_trip` |

## `MTKTextureLoader.h`

| Header surface | Status | Notes |
| --- | --- | --- |
| error string-enum + domain / error key constants | ✅ | `TextureLoaderError` + `texture_loader_error` |
| option constants: mipmaps / sRGB / usage / storage / CPU cache / cube layout / origin / load-as-array | ✅ | `TextureLoaderOptions` + constant modules |
| `newTextureWithContentsOfURL:options:error:` | ✅ | `new_texture_from_url` |
| `newTexturesWithContentsOfURLs:options:error:` | ✅ | `new_textures_from_urls` with per-element success/failure reporting |
| `newTextureWithData:options:error:` | ✅ | `new_texture_from_data` |
| `newTextureWithCGImage:options:error:` | ✅ | `new_texture_from_cgimage` |
| `newTextureWithMDLTexture:options:error:` | ✅ | `new_texture_from_model_texture` |
| `newTextureWithName:scaleFactor:bundle:options:error:` | ✅ | `new_texture_named` |
| `newTextureWithName:scaleFactor:displayGamut:bundle:options:error:` | ✅ | `new_texture_named_with_display_gamut` |
| array-of-names helpers | ✅ | `new_textures_named*`, represented as `TextureLoaderArrayOutcome` |
| completion-handler variants | ✅ | `*_with_callback` wrappers + `TextureLoaderCallback` / `TextureLoaderArrayCallback` |

## `MTKModel.h` mesh bridge + Model I/O integration

| Header surface | Status | Notes |
| --- | --- | --- |
| model error string-enum + domain / error key constants | ✅ | `ModelError` + `model_error` |
| `MTKMeshBufferAllocator` init/device/newBuffer APIs | ✅ | `MeshBufferAllocator`; `device` is only a raw pointer (`device_ptr`) |
| `MTKMeshBuffer` allocator/zone/length/offset/type/buffer/name | ✅ | `MeshBuffer`; `buffer` is only a raw pointer (`metal_buffer_ptr`) because apple-metal has no public owning `MetalBuffer` constructor |
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
