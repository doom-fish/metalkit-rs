# Changelog

All notable changes to `metalkit-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-11-21

### Changed

- Mesh buffer copies honor the suballocation offset and the logical mesh length.
- Bindings for Core Graphics types use apple-cf 0.12.

## [0.3.2] - 2026-10-26

### Fixed

- Update the test assertions for Clippy 1.99 on hosted runners.

## [0.3.1] - 2026-10-26

### Fixed

- Build the Swift bridge with the default SwiftPM build engine and use its reported product directory.
- Export the C entry points from the Swift static library.

## [0.3.0] - 2026-09-24

### Security

- `View::delegate_ptr` returned the delegate box at +1 while every other
  getter returned +0, so each call leaked a reference. A leaked box outlived
  its `ViewDelegate`, whose drop freed the closure state, and stayed the
  view's weak delegate, so the next `draw(in:)` read freed memory. The getter
  now returns +0, and the closure state lives in a reference-counted context
  owned by the Swift delegate box, which a dropped `ViewDelegate` deactivates.
- Panics in `MTKViewDelegate` callbacks are contained instead of unwinding
  into Swift.

### Fixed

- `View::current_mtl4_render_pass_descriptor` called a macOS 26 API with no
  run-time check against the macOS 11 minimum and aborted on macOS 11 to 15;
  it now returns an error there.
- `View::new` created an `AppKit` view on any thread; it now refuses to run
  off the main thread, and the bridge refuses view calls off the main thread.
- A draw that the draw callback triggers on the same view is skipped instead of
  deadlocking.
- The color-space setter and the `CGImage` texture loaders force-cast their
  arguments with `as!`, which Swift does not check for CF types; they now
  check the CF type ID. The data loaders and `MeshBufferAllocator` no longer
  force-unwrap the byte pointer.
- COVERAGE files counted raw pointer getters as covered; README gave a stale
  status and no minimums.

### Changed

- **Breaking:** `View::new` returns `Result<View, MetalKitError>`.
- **Breaking:** `ViewDelegateCallbacks::on_draw` and
  `on_drawable_size_will_change` require `Send` closures.
- **Breaking:** the `View` raw pointer getters are replaced by typed getters
  that return owned values: `device` and `preferred_device` (`MetalDevice`),
  `current_drawable` (`MetalDrawable`), `current_render_pass_descriptor`
  (`MetalRenderPassDescriptor`), `current_mtl4_render_pass_descriptor`
  (`Result<Option<Metal4RenderPassDescriptor>, MetalKitError>`),
  `depth_stencil_texture` and `multisample_color_texture` (`MetalTexture`) and
  `colorspace` (`CGColorSpace`). `delegate_ptr` stays as a +0 pointer.
- Requires `apple-cf >=0.11, <0.12` and `apple-metal >=0.10, <0.11`, and adds
  `doom-fish-utils >=0.4.1, <0.5`; `rust-version` is 1.82 (was 1.76).
- The `TextureLoaderOptionsRaw` layout is pinned with compile-time size and
  alignment checks and a cross-language `mtk_verify_ffi_layout` check.

### Removed

- `View::device_ptr`, `preferred_device_ptr`, `current_drawable_ptr`,
  `current_render_pass_descriptor_ptr`,
  `current_mtl4_render_pass_descriptor_ptr`, `depth_stencil_texture_ptr`,
  `multisample_color_texture_ptr` and `colorspace_ptr`.

## [0.2.5] - 2026-05-18

- Added one-line Rustdoc coverage across the public MetalKit surface in `src/`, including wrapper types, enums, option constants, and view/loader/model helpers.
- Raised the public API doc pass to full coverage for the crate source set.

## [0.2.4] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

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
