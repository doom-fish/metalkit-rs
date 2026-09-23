import AppKit
import CoreGraphics
import Metal
import MetalKit

typealias MTKViewDrawCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
typealias MTKViewResizeCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, Double, Double) -> Void
typealias MTKContextReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

final class MTKContextOwner {
    let context: UnsafeMutableRawPointer
    private let release: MTKContextReleaseCallback

    init?(_ context: UnsafeMutableRawPointer?, _ release: MTKContextReleaseCallback?) {
        guard let context, let release else { return nil }
        self.context = context
        self.release = release
    }

    deinit {
        release(context)
    }
}

final class MTKViewDelegateBox: NSObject, MTKViewDelegate {
    private let drawCallback: MTKViewDrawCallback?
    private let resizeCallback: MTKViewResizeCallback?
    private let owner: MTKContextOwner

    init(
        drawCallback: MTKViewDrawCallback?,
        resizeCallback: MTKViewResizeCallback?,
        owner: MTKContextOwner
    ) {
        self.drawCallback = drawCallback
        self.resizeCallback = resizeCallback
        self.owner = owner
    }

    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        resizeCallback?(owner.context, Unmanaged.passUnretained(view).toOpaque(), size.width, size.height)
    }

    func draw(in view: MTKView) {
        drawCallback?(owner.context, Unmanaged.passUnretained(view).toOpaque())
    }
}

private func mtkView(_ ptr: UnsafeMutableRawPointer?) -> MTKView? {
    guard Thread.isMainThread else { return nil }
    return mtkBorrow(ptr, as: MTKView.self)
}

private func mtkPixelFormat(from rawValue: UInt) -> MTLPixelFormat {
    MTLPixelFormat(rawValue: rawValue) ?? .invalid
}

private func mtkStorageMode(from rawValue: UInt) -> MTLStorageMode {
    MTLStorageMode(rawValue: rawValue) ?? .private
}

private func mtkTextureUsage(from rawValue: UInt) -> MTLTextureUsage {
    MTLTextureUsage(rawValue: rawValue)
}

private func mtkCGColorSpace(_ ptr: UnsafeMutableRawPointer?) -> CGColorSpace? {
    guard let ptr else { return nil }
    let object = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue()
    guard CFGetTypeID(object) == CGColorSpace.typeID else { return nil }
    return Unmanaged<CGColorSpace>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("mtk_view_delegate_new")
func mtk_view_delegate_new(
    _ drawCallback: MTKViewDrawCallback?,
    _ resizeCallback: MTKViewResizeCallback?,
    _ context: UnsafeMutableRawPointer?,
    _ release: MTKContextReleaseCallback?
) -> UnsafeMutableRawPointer? {
    guard let owner = MTKContextOwner(context, release) else { return nil }
    return mtkTakeRetained(MTKViewDelegateBox(
        drawCallback: drawCallback,
        resizeCallback: resizeCallback,
        owner: owner
    ))
}

@_cdecl("mtk_view_new")
func mtk_view_new(
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ devicePtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard Thread.isMainThread else { return nil }
    let device: MTLDevice? = mtkBorrow(devicePtr, as: MTLDevice.self)
    let view = MTKView(
        frame: CGRect(x: x, y: y, width: width, height: height),
        device: device
    )
    return mtkTakeRetained(view)
}

@_cdecl("mtk_view_archive_round_trip")
func mtk_view_archive_round_trip(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkView(viewPtr) else {
        outError?.pointee = mtkDup(Thread.isMainThread ? "invalid MTKView" : "MTKView must be used on the main thread")
        return nil
    }

    do {
        let data = try NSKeyedArchiver.archivedData(withRootObject: view, requiringSecureCoding: false)
        let unarchiver = try NSKeyedUnarchiver(forReadingFrom: data)
        unarchiver.requiresSecureCoding = false
        let object = unarchiver.decodeObject(forKey: NSKeyedArchiveRootObjectKey) as? MTKView
        unarchiver.finishDecoding()
        outError?.pointee = nil
        return mtkTakeRetained(object)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}

@_cdecl("mtk_view_delegate")
func mtk_view_delegate(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkView(viewPtr),
          let delegate = view.delegate as AnyObject?
    else {
        return nil
    }
    return Unmanaged.passUnretained(delegate).toOpaque()
}

@_cdecl("mtk_view_set_delegate")
func mtk_view_set_delegate(_ viewPtr: UnsafeMutableRawPointer?, _ delegatePtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.delegate = mtkBorrow(delegatePtr, as: MTKViewDelegateBox.self)
}

@_cdecl("mtk_view_device_registry_id")
func mtk_view_device_registry_id(_ viewPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let view: MTKView = mtkView(viewPtr), let device = view.device else { return 0 }
    return device.registryID
}

@_cdecl("mtk_view_set_device")
func mtk_view_set_device(_ viewPtr: UnsafeMutableRawPointer?, _ devicePtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.device = mtkBorrow(devicePtr, as: MTLDevice.self)
}

@_cdecl("mtk_view_copy_current_drawable")
func mtk_view_copy_current_drawable(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkView(viewPtr),
          let drawable = view.currentDrawable
    else {
        return nil
    }
    return mtkTakeRetained(drawable as AnyObject)
}

@_cdecl("mtk_view_framebuffer_only")
func mtk_view_framebuffer_only(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkView(viewPtr) else {
        return false
    }
    return view.framebufferOnly
}

@_cdecl("mtk_view_set_framebuffer_only")
func mtk_view_set_framebuffer_only(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.framebufferOnly = value
}

@_cdecl("mtk_view_depth_stencil_attachment_texture_usage")
func mtk_view_depth_stencil_attachment_texture_usage(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkView(viewPtr) else {
        return 0
    }
    return view.depthStencilAttachmentTextureUsage.rawValue
}

@_cdecl("mtk_view_set_depth_stencil_attachment_texture_usage")
func mtk_view_set_depth_stencil_attachment_texture_usage(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.depthStencilAttachmentTextureUsage = mtkTextureUsage(from: value)
}

@_cdecl("mtk_view_multisample_color_attachment_texture_usage")
func mtk_view_multisample_color_attachment_texture_usage(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkView(viewPtr) else {
        return 0
    }
    return view.multisampleColorAttachmentTextureUsage.rawValue
}

@_cdecl("mtk_view_set_multisample_color_attachment_texture_usage")
func mtk_view_set_multisample_color_attachment_texture_usage(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.multisampleColorAttachmentTextureUsage = mtkTextureUsage(from: value)
}

@_cdecl("mtk_view_presents_with_transaction")
func mtk_view_presents_with_transaction(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkView(viewPtr) else {
        return false
    }
    return view.presentsWithTransaction
}

@_cdecl("mtk_view_set_presents_with_transaction")
func mtk_view_set_presents_with_transaction(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.presentsWithTransaction = value
}

@_cdecl("mtk_view_color_pixel_format")
func mtk_view_color_pixel_format(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkView(viewPtr) else {
        return 0
    }
    return view.colorPixelFormat.rawValue
}

@_cdecl("mtk_view_set_color_pixel_format")
func mtk_view_set_color_pixel_format(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.colorPixelFormat = mtkPixelFormat(from: value)
}

@_cdecl("mtk_view_depth_stencil_pixel_format")
func mtk_view_depth_stencil_pixel_format(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkView(viewPtr) else {
        return 0
    }
    return view.depthStencilPixelFormat.rawValue
}

@_cdecl("mtk_view_set_depth_stencil_pixel_format")
func mtk_view_set_depth_stencil_pixel_format(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.depthStencilPixelFormat = mtkPixelFormat(from: value)
}

@_cdecl("mtk_view_depth_stencil_storage_mode")
func mtk_view_depth_stencil_storage_mode(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkView(viewPtr) else {
        return 0
    }
    if #available(macOS 13.0, *) {
        return view.depthStencilStorageMode.rawValue
    }
    return MTLStorageMode.private.rawValue
}

@_cdecl("mtk_view_set_depth_stencil_storage_mode")
func mtk_view_set_depth_stencil_storage_mode(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    if #available(macOS 13.0, *) {
        view.depthStencilStorageMode = mtkStorageMode(from: value)
    }
}

@_cdecl("mtk_view_sample_count")
func mtk_view_sample_count(_ viewPtr: UnsafeMutableRawPointer?) -> Int {
    guard let view: MTKView = mtkView(viewPtr) else {
        return 0
    }
    return view.sampleCount
}

@_cdecl("mtk_view_set_sample_count")
func mtk_view_set_sample_count(_ viewPtr: UnsafeMutableRawPointer?, _ value: Int) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.sampleCount = value
}

@_cdecl("mtk_view_clear_color")
func mtk_view_clear_color(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ outRed: UnsafeMutablePointer<Double>?,
    _ outGreen: UnsafeMutablePointer<Double>?,
    _ outBlue: UnsafeMutablePointer<Double>?,
    _ outAlpha: UnsafeMutablePointer<Double>?
) {
    let color = (mtkView(viewPtr)?.clearColor) ?? MTLClearColorMake(0, 0, 0, 1)
    outRed?.pointee = color.red
    outGreen?.pointee = color.green
    outBlue?.pointee = color.blue
    outAlpha?.pointee = color.alpha
}

@_cdecl("mtk_view_set_clear_color")
func mtk_view_set_clear_color(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ red: Double,
    _ green: Double,
    _ blue: Double,
    _ alpha: Double
) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.clearColor = MTLClearColorMake(red, green, blue, alpha)
}

@_cdecl("mtk_view_clear_depth")
func mtk_view_clear_depth(_ viewPtr: UnsafeMutableRawPointer?) -> Double {
    guard let view: MTKView = mtkView(viewPtr) else {
        return 1.0
    }
    return view.clearDepth
}

@_cdecl("mtk_view_set_clear_depth")
func mtk_view_set_clear_depth(_ viewPtr: UnsafeMutableRawPointer?, _ value: Double) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.clearDepth = value
}

@_cdecl("mtk_view_clear_stencil")
func mtk_view_clear_stencil(_ viewPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let view: MTKView = mtkView(viewPtr) else {
        return 0
    }
    return view.clearStencil
}

@_cdecl("mtk_view_set_clear_stencil")
func mtk_view_set_clear_stencil(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt32) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.clearStencil = value
}

@_cdecl("mtk_view_copy_depth_stencil_texture")
func mtk_view_copy_depth_stencil_texture(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkView(viewPtr),
          let texture = view.depthStencilTexture
    else {
        return nil
    }
    return mtkTakeRetained(texture as AnyObject)
}

@_cdecl("mtk_view_copy_multisample_color_texture")
func mtk_view_copy_multisample_color_texture(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkView(viewPtr),
          let texture = view.multisampleColorTexture
    else {
        return nil
    }
    return mtkTakeRetained(texture as AnyObject)
}

@_cdecl("mtk_view_release_drawables")
func mtk_view_release_drawables(_ viewPtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.releaseDrawables()
}

@_cdecl("mtk_view_copy_current_render_pass_descriptor")
func mtk_view_copy_current_render_pass_descriptor(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkView(viewPtr),
          let descriptor = view.currentRenderPassDescriptor
    else {
        return nil
    }
    return mtkTakeRetained(descriptor)
}

@_cdecl("mtk_view_supports_mtl4_render_pass_descriptor")
func mtk_view_supports_mtl4_render_pass_descriptor() -> Bool {
    if #available(macOS 26.0, *) {
        return true
    }
    return false
}

@_cdecl("mtk_view_copy_current_mtl4_render_pass_descriptor")
func mtk_view_copy_current_mtl4_render_pass_descriptor(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 26.0, *),
          let view: MTKView = mtkView(viewPtr),
          let descriptor = view.currentMTL4RenderPassDescriptor
    else {
        return nil
    }
    return mtkTakeRetained(descriptor)
}

@_cdecl("mtk_view_preferred_frames_per_second")
func mtk_view_preferred_frames_per_second(_ viewPtr: UnsafeMutableRawPointer?) -> Int {
    guard let view: MTKView = mtkView(viewPtr) else {
        return 0
    }
    return view.preferredFramesPerSecond
}

@_cdecl("mtk_view_set_preferred_frames_per_second")
func mtk_view_set_preferred_frames_per_second(_ viewPtr: UnsafeMutableRawPointer?, _ value: Int) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.preferredFramesPerSecond = value
}

@_cdecl("mtk_view_enable_set_needs_display")
func mtk_view_enable_set_needs_display(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkView(viewPtr) else {
        return false
    }
    return view.enableSetNeedsDisplay
}

@_cdecl("mtk_view_set_enable_set_needs_display")
func mtk_view_set_enable_set_needs_display(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.enableSetNeedsDisplay = value
}

@_cdecl("mtk_view_auto_resize_drawable")
func mtk_view_auto_resize_drawable(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkView(viewPtr) else {
        return false
    }
    return view.autoResizeDrawable
}

@_cdecl("mtk_view_set_auto_resize_drawable")
func mtk_view_set_auto_resize_drawable(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.autoResizeDrawable = value
}

@_cdecl("mtk_view_drawable_size")
func mtk_view_drawable_size(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    let size = (mtkView(viewPtr)?.drawableSize) ?? .zero
    outWidth?.pointee = size.width
    outHeight?.pointee = size.height
}

@_cdecl("mtk_view_set_drawable_size")
func mtk_view_set_drawable_size(_ viewPtr: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.drawableSize = CGSize(width: width, height: height)
}

@_cdecl("mtk_view_preferred_drawable_size")
func mtk_view_preferred_drawable_size(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    let size = (mtkView(viewPtr)?.preferredDrawableSize) ?? .zero
    outWidth?.pointee = size.width
    outHeight?.pointee = size.height
}

@_cdecl("mtk_view_preferred_device_registry_id")
func mtk_view_preferred_device_registry_id(_ viewPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let view: MTKView = mtkView(viewPtr), let device = view.preferredDevice else { return 0 }
    return device.registryID
}

@_cdecl("mtk_view_is_paused")
func mtk_view_is_paused(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkView(viewPtr) else {
        return false
    }
    return view.isPaused
}

@_cdecl("mtk_view_set_paused")
func mtk_view_set_paused(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.isPaused = value
}

@_cdecl("mtk_view_copy_colorspace")
func mtk_view_copy_colorspace(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkView(viewPtr),
          let colorspace = view.colorspace
    else {
        return nil
    }
    return mtkTakeRetained(colorspace)
}

@_cdecl("mtk_view_set_colorspace")
func mtk_view_set_colorspace(_ viewPtr: UnsafeMutableRawPointer?, _ value: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.colorspace = mtkCGColorSpace(value)
}

@_cdecl("mtk_view_draw")
func mtk_view_draw(_ viewPtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.draw()
}

@_cdecl("mtk_view_notify_delegate_size_will_change")
func mtk_view_notify_delegate_size_will_change(_ viewPtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.delegate?.mtkView(view, drawableSizeWillChange: view.drawableSize)
}

@_cdecl("mtk_view_notify_delegate_draw")
func mtk_view_notify_delegate_draw(_ viewPtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkView(viewPtr) else {
        return
    }
    view.delegate?.draw(in: view)
}
