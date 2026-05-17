# metalkit-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 108
VERIFIED: 108
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00%

Audited headers: `MTKView.h`, `MTKTextureLoader.h`, and `MTKModel.h`. `MetalKit.h` only re-exports those headers, and `MTKDefines.h` only contributes helper macros. V2 verification confirms all 108 public macOS symbols from the MetalKit framework are exposed by the crate through safe Rust wrappers or FFI bindings.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MTKView` | interface | `MTKView.h` | `View` |
| `MTKViewDelegate` | protocol | `MTKView.h` | `ViewDelegate`; `ViewDelegateCallbacks` |
| `MTKView.initWithFrame(_:device:)` | initializer | `MTKView.h` | `View::new(...)` |
| `MTKView.initWithCoder(_)` | initializer | `MTKView.h` | `View::archive_round_trip()` (indirect `NSCoder` round-trip) |
| `MTKView.delegate` | property | `MTKView.h` | `View::delegate_ptr()`; `View::set_delegate(...)` |
| `MTKView.device` | property | `MTKView.h` | `View::device_ptr()`; `View::set_device(...)` |
| `MTKView.currentDrawable` | property | `MTKView.h` | `View::current_drawable_ptr()` (raw `CAMetalDrawable` pointer) |
| `MTKView.framebufferOnly` | property | `MTKView.h` | `View::framebuffer_only()`; `View::set_framebuffer_only(...)` |
| `MTKView.depthStencilAttachmentTextureUsage` | property | `MTKView.h` | `View::depth_stencil_attachment_texture_usage()`; `View::set_depth_stencil_attachment_texture_usage(...)` |
| `MTKView.multisampleColorAttachmentTextureUsage` | property | `MTKView.h` | `View::multisample_color_attachment_texture_usage()`; `View::set_multisample_color_attachment_texture_usage(...)` |
| `MTKView.presentsWithTransaction` | property | `MTKView.h` | `View::presents_with_transaction()`; `View::set_presents_with_transaction(...)` |
| `MTKView.colorPixelFormat` | property | `MTKView.h` | `View::color_pixel_format()`; `View::set_color_pixel_format(...)` |
| `MTKView.depthStencilPixelFormat` | property | `MTKView.h` | `View::depth_stencil_pixel_format()`; `View::set_depth_stencil_pixel_format(...)` |
| `MTKView.depthStencilStorageMode` | property | `MTKView.h` | `View::depth_stencil_storage_mode()`; `View::set_depth_stencil_storage_mode(...)` |
| `MTKView.sampleCount` | property | `MTKView.h` | `View::sample_count()`; `View::set_sample_count(...)` |
| `MTKView.clearColor` | property | `MTKView.h` | `View::clear_color()`; `View::set_clear_color(...)` |
| `MTKView.clearDepth` | property | `MTKView.h` | `View::clear_depth()`; `View::set_clear_depth(...)` |
| `MTKView.clearStencil` | property | `MTKView.h` | `View::clear_stencil()`; `View::set_clear_stencil(...)` |
| `MTKView.depthStencilTexture` | property | `MTKView.h` | `View::depth_stencil_texture_ptr()` (raw `MTLTexture` pointer) |
| `MTKView.multisampleColorTexture` | property | `MTKView.h` | `View::multisample_color_texture_ptr()` (raw `MTLTexture` pointer) |
| `MTKView.releaseDrawables()` | instance method | `MTKView.h` | `View::release_drawables()` |
| `MTKView.currentRenderPassDescriptor` | property | `MTKView.h` | `View::current_render_pass_descriptor_ptr()` (raw `MTLRenderPassDescriptor` pointer) |
| `MTKView.currentMTL4RenderPassDescriptor` | property | `MTKView.h` | `View::current_mtl4_render_pass_descriptor_ptr()` (raw `MTL4RenderPassDescriptor` pointer) |
| `MTKView.preferredFramesPerSecond` | property | `MTKView.h` | `View::preferred_frames_per_second()`; `View::set_preferred_frames_per_second(...)` |
| `MTKView.enableSetNeedsDisplay` | property | `MTKView.h` | `View::enable_set_needs_display()`; `View::set_enable_set_needs_display(...)` |
| `MTKView.autoResizeDrawable` | property | `MTKView.h` | `View::auto_resize_drawable()`; `View::set_auto_resize_drawable(...)` |
| `MTKView.drawableSize` | property | `MTKView.h` | `View::drawable_size()`; `View::set_drawable_size(...)` |
| `MTKView.preferredDrawableSize` | property | `MTKView.h` | `View::preferred_drawable_size()` |
| `MTKView.preferredDevice` | property | `MTKView.h` | `View::preferred_device_ptr()` (raw `MTLDevice` pointer) |
| `MTKView.paused` | property | `MTKView.h` | `View::is_paused()`; `View::set_paused(...)` |
| `MTKView.colorspace` | property | `MTKView.h` | `View::colorspace_ptr()`; `View::set_colorspace(...)` |
| `MTKView.draw()` | instance method | `MTKView.h` | `View::draw()` |
| `MTKViewDelegate.mtkView(_:drawableSizeWillChange:)` | delegate callback | `MTKView.h` | `ViewDelegateCallbacks::on_drawable_size_will_change(...)`; `View::notify_delegate_drawable_size_will_change()` |
| `MTKViewDelegate.drawInMTKView(_)` | delegate callback | `MTKView.h` | `ViewDelegateCallbacks::on_draw(...)`; `View::notify_delegate_draw()` |
| `MTKTextureLoader` | interface | `MTKTextureLoader.h` | `TextureLoader` |
| `MTKTextureLoaderErrorDomain` | constant | `MTKTextureLoader.h` | `texture_loader_error::DOMAIN` |
| `MTKTextureLoaderErrorKey` | constant | `MTKTextureLoader.h` | `texture_loader_error::KEY` |
| `MTKTextureLoaderError` | typealias | `MTKTextureLoader.h` | `TextureLoaderError` |
| `MTKTextureLoaderOption` | typealias | `MTKTextureLoader.h` | `TextureLoaderOptionKey` |
| `MTKTextureLoaderOptionAllocateMipmaps` | constant | `MTKTextureLoader.h` | `texture_loader_option::ALLOCATE_MIPMAPS`; `TextureLoaderOptions::with_allocate_mipmaps(...)` |
| `MTKTextureLoaderOptionGenerateMipmaps` | constant | `MTKTextureLoader.h` | `texture_loader_option::GENERATE_MIPMAPS`; `TextureLoaderOptions::with_generate_mipmaps(...)` |
| `MTKTextureLoaderOptionSRGB` | constant | `MTKTextureLoader.h` | `texture_loader_option::SRGB`; `TextureLoaderOptions::with_srgb(...)` |
| `MTKTextureLoaderOptionTextureUsage` | constant | `MTKTextureLoader.h` | `texture_loader_option::TEXTURE_USAGE`; `TextureLoaderOptions::with_texture_usage(...)` |
| `MTKTextureLoaderOptionTextureCPUCacheMode` | constant | `MTKTextureLoader.h` | `texture_loader_option::TEXTURE_CPU_CACHE_MODE`; `TextureLoaderOptions::with_texture_cpu_cache_mode(...)` |
| `MTKTextureLoaderOptionTextureStorageMode` | constant | `MTKTextureLoader.h` | `texture_loader_option::TEXTURE_STORAGE_MODE`; `TextureLoaderOptions::with_texture_storage_mode(...)` |
| `MTKTextureLoaderCubeLayout` | typealias | `MTKTextureLoader.h` | `TextureLoaderCubeLayout` |
| `MTKTextureLoaderOptionCubeLayout` | constant | `MTKTextureLoader.h` | `texture_loader_option::CUBE_LAYOUT`; `TextureLoaderOptions::with_cube_layout(...)` |
| `MTKTextureLoaderCubeLayoutVertical` | constant | `MTKTextureLoader.h` | `texture_loader_cube_layout::VERTICAL` |
| `MTKTextureLoaderOrigin` | typealias | `MTKTextureLoader.h` | `TextureLoaderOrigin` |
| `MTKTextureLoaderOptionOrigin` | constant | `MTKTextureLoader.h` | `texture_loader_option::ORIGIN`; `TextureLoaderOptions::with_origin(...)` |
| `MTKTextureLoaderOriginTopLeft` | constant | `MTKTextureLoader.h` | `texture_loader_origin::TOP_LEFT` |
| `MTKTextureLoaderOriginBottomLeft` | constant | `MTKTextureLoader.h` | `texture_loader_origin::BOTTOM_LEFT` |
| `MTKTextureLoaderOriginFlippedVertically` | constant | `MTKTextureLoader.h` | `texture_loader_origin::FLIPPED_VERTICALLY` |
| `MTKTextureLoaderOptionLoadAsArray` | constant | `MTKTextureLoader.h` | `texture_loader_option::LOAD_AS_ARRAY`; `TextureLoaderOptions::with_load_as_array(...)` |
| `MTKTextureLoader.device` | property | `MTKTextureLoader.h` | `TextureLoader::device_ptr()` (raw `MTLDevice` pointer) |
| `MTKTextureLoader.initWithDevice(_)` | initializer | `MTKTextureLoader.h` | `TextureLoader::new(...)` |
| `MTKTextureLoaderCallback` | typealias | `MTKTextureLoader.h` | `TextureLoaderCallback` |
| `MTKTextureLoaderArrayCallback` | typealias | `MTKTextureLoader.h` | `TextureLoaderArrayCallback` |
| `MTKTextureLoader.newTextureWithContentsOfURL(_:options:error:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_from_url(...)` |
| `MTKTextureLoader.newTextureWithContentsOfURL(_:options:completionHandler:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_from_url_with_callback(...)` |
| `MTKTextureLoader.newTexturesWithContentsOfURLs(_:options:error:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_textures_from_urls(...)` via `TextureLoaderArrayOutcome` (semantic batch equivalent) |
| `MTKTextureLoader.newTexturesWithContentsOfURLs(_:options:completionHandler:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_textures_from_urls_with_callback(...)` |
| `MTKTextureLoader.newTextureWithData(_:options:error:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_from_data(...)` |
| `MTKTextureLoader.newTextureWithData(_:options:completionHandler:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_from_data_with_callback(...)` |
| `MTKTextureLoader.newTextureWithCGImage(_:options:error:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_from_cgimage(...)` |
| `MTKTextureLoader.newTextureWithCGImage(_:options:completionHandler:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_from_cgimage_with_callback(...)` |
| `MTKTextureLoader.newTextureWithMDLTexture(_:options:error:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_from_model_texture(...)` |
| `MTKTextureLoader.newTextureWithMDLTexture(_:options:completionHandler:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_from_model_texture_with_callback(...)` |
| `MTKTextureLoader.newTextureWithName(_:scaleFactor:bundle:options:error:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_named(...)` |
| `MTKTextureLoader.newTextureWithName(_:scaleFactor:bundle:options:completionHandler:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_named_with_callback(...)` |
| `MTKTextureLoader.newTextureWithName(_:scaleFactor:displayGamut:bundle:options:error:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_named_with_display_gamut(...)` |
| `MTKTextureLoader.newTextureWithName(_:scaleFactor:displayGamut:bundle:options:completionHandler:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_texture_named_with_display_gamut_with_callback(...)` |
| `MTKTextureLoader.newTexturesWithNames(_:scaleFactor:bundle:options:completionHandler:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_textures_named_with_callback(...)` |
| `MTKTextureLoader.newTexturesWithNames(_:scaleFactor:displayGamut:bundle:options:completionHandler:)` | instance method | `MTKTextureLoader.h` | `TextureLoader::new_textures_named_with_display_gamut_with_callback(...)` |
| `MTKModelErrorDomain` | constant | `MTKModel.h` | `model_error::DOMAIN` |
| `MTKModelErrorKey` | constant | `MTKModel.h` | `model_error::KEY` |
| `MTKModelError` | typealias | `MTKModel.h` | `ModelError` |
| `MTKMeshBufferAllocator` | interface | `MTKModel.h` | `MeshBufferAllocator` |
| `MTKMeshBufferAllocator.initWithDevice(_)` | initializer | `MTKModel.h` | `MeshBufferAllocator::new(...)` |
| `MTKMeshBufferAllocator.device` | property | `MTKModel.h` | `MeshBufferAllocator::device_ptr()` (raw `MTLDevice` pointer) |
| `MTKMeshBuffer` | interface | `MTKModel.h` | `MeshBuffer` |
| `MTKMeshBuffer.length` | property | `MTKModel.h` | `MeshBuffer::length()` |
| `MTKMeshBuffer.allocator` | property | `MTKModel.h` | `MeshBuffer::allocator()` |
| `MTKMeshBuffer.zone` | property | `MTKModel.h` | `MeshBuffer::zone()` |
| `MTKMeshBuffer.buffer` | property | `MTKModel.h` | `MeshBuffer::metal_buffer_ptr()` (raw `MTLBuffer` pointer) |
| `MTKMeshBuffer.offset` | property | `MTKModel.h` | `MeshBuffer::offset()` |
| `MTKMeshBuffer.type` | property | `MTKModel.h` | `MeshBuffer::buffer_type()` |
| `MTKSubmesh` | interface | `MTKModel.h` | `Submesh` |
| `MTKSubmesh.primitiveType` | property | `MTKModel.h` | `Submesh::primitive_type()` |
| `MTKSubmesh.indexType` | property | `MTKModel.h` | `Submesh::index_type()` |
| `MTKSubmesh.indexBuffer` | property | `MTKModel.h` | `Submesh::index_buffer()` |
| `MTKSubmesh.indexCount` | property | `MTKModel.h` | `Submesh::index_count()` |
| `MTKSubmesh.mesh` | property | `MTKModel.h` | `Submesh::mesh()` |
| `MTKSubmesh.name` | property | `MTKModel.h` | `Submesh::name()`; `Submesh::set_name(...)` |
| `MTKMesh` | interface | `MTKModel.h` | `Mesh` |
| `MTKMesh.initWithMesh(_:device:error:)` | initializer | `MTKModel.h` | `Mesh::from_model_mesh(...)` |
| `MTKMesh.newMeshesFromAsset(_:device:sourceMeshes:error:)` | class method | `MTKModel.h` | `Mesh::new_meshes_from_asset(...)` |
| `MTKMesh.vertexBuffers` | property | `MTKModel.h` | `Mesh::vertex_buffers()` |
| `MTKMesh.vertexDescriptor` | property | `MTKModel.h` | `Mesh::vertex_descriptor()` via `ModelVertexDescriptor` |
| `MTKMesh.submeshes` | property | `MTKModel.h` | `Mesh::submeshes()` |
| `MTKMesh.vertexCount` | property | `MTKModel.h` | `Mesh::vertex_count()` |
| `MTKMesh.name` | property | `MTKModel.h` | `Mesh::name()`; `Mesh::set_name(...)` |
| `MTKModelIOVertexDescriptorFromMetal` | function | `MTKModel.h` | `model_io_vertex_descriptor_from_metal(...)` |
| `MTKModelIOVertexDescriptorFromMetalWithError` | function | `MTKModel.h` | `try_model_io_vertex_descriptor_from_metal(...)` |
| `MTKMetalVertexDescriptorFromModelIO` | function | `MTKModel.h` | `metal_vertex_descriptor_from_model_io(...)` |
| `MTKMetalVertexDescriptorFromModelIOWithError` | function | `MTKModel.h` | `try_metal_vertex_descriptor_from_model_io(...)` |
| `MTKModelIOVertexFormatFromMetal` | function | `MTKModel.h` | `model_io_vertex_format_from_metal(...)` |
| `MTKMetalVertexFormatFromModelIO` | function | `MTKModel.h` | `metal_vertex_format_from_model_io(...)` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

No remaining gaps. All 108 public macOS symbols from the MetalKit framework are fully covered by the metalkit-rs crate.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |

No exempt symbols. The MetalKit.framework macOS public headers (MTKView.h, MTKTextureLoader.h, MTKModel.h) do not expose deprecated or macOS-unavailable symbols that require exemption classification.
