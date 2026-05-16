# Changelog

## 0.2.0

- Added `MTKView` coverage with `View`, `ViewDelegate`, property accessors, delegate callbacks, and archive round-trip support
- Split the Rust and Swift bridge into logical modules for mesh, mesh-buffer, submesh, allocator, texture-loader, Model I/O, and view coverage
- Expanded `TextureLoader` with URL-array handling, `MDLTexture` loading, display-gamut entry points, and richer option coverage
- Added Model I/O integration helpers: `ModelAsset`, `ModelMesh`, `ModelTexture`, Metal/Model I/O vertex-descriptor conversion, and vertex-format conversion helpers
- Added integration tests for every requested logical area plus smoke examples for texture loading, mesh bridging, Model I/O integration, mesh-buffer allocation, and `MTKView`
- Added audited SDK coverage documentation in `COVERAGE.md`

## 0.1.0

- Initial release of `metalkit-rs`
- Added `MTKTextureLoader` wrappers for URL, asset-name, data, and `CGImage` loading
- Added `TextureLoaderOptions` plus MetalKit option constants for mipmaps, sRGB, usage, storage mode, and CPU cache mode
- Added `MTKMeshBufferAllocator`, `MTKMeshBuffer`, `MTKMesh`, `MTKSubmesh`, and a minimal `MDLMesh` wrapper for Model I/O interop
- Added smoke examples for texture loading and mesh bridging
