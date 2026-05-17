# Changelog

## 0.2.3

- Fixed version constraints for `apple-cf` and `apple-metal` dependencies
  to use the standard `>=X.Y, <X.(Y+2)` pattern for semantic versioning drift.

## 0.2.2

- Added `@available(macOS 26.0, *)` declaration attribute to the
  `mtk_view_current_mtl4_render_pass_descriptor` Swift bridge thunk, which
  references `MTKView.currentMTL4RenderPassDescriptor` — an API introduced
  in macOS 26 / iOS 26. The function body previously used an `if #available`
  guard; the outer `@available` attribute now makes the declaration
  self-documenting and SDK-portable so the bridge compiles correctly against
  any SDK that includes the macOS 26 headers.
- Bumped `apple-metal` version constraint to `>=0.5, <0.9` to track the
  `0.8.x` release.
- Added `#[allow(clippy::too_many_lines)]` to two large integration-test
  functions that tripped the lint after the dependency update.

## 0.2.1

- Added `MTKTextureLoader` completion-handler wrappers for URL, named, `CGImage`, `MDLTexture`, data, and batch loading APIs via safe Rust callbacks
- Added `TextureLoaderError`, `TextureLoaderCallback`, `TextureLoaderArrayCallback`, and `ModelError` wrappers for the remaining MetalKit string-enum and callback typealias surface
- Expanded the texture-loader smoke example, integration tests, and coverage audit to validate the callback API and close the remaining header gaps

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
