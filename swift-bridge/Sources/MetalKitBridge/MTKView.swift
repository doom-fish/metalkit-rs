import AppKit
import CoreGraphics
import Metal
import MetalKit

typealias MTKViewDrawCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
typealias MTKViewResizeCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, Double, Double) -> Void

final class MTKViewDelegateBox: NSObject, MTKViewDelegate {
    let drawCallback: MTKViewDrawCallback?
    let resizeCallback: MTKViewResizeCallback?
    let userData: UnsafeMutableRawPointer?

    init(
        drawCallback: MTKViewDrawCallback?,
        resizeCallback: MTKViewResizeCallback?,
        userData: UnsafeMutableRawPointer?
    ) {
        self.drawCallback = drawCallback
        self.resizeCallback = resizeCallback
        self.userData = userData
    }

    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        resizeCallback?(userData, Unmanaged.passUnretained(view).toOpaque(), size.width, size.height)
    }

    func draw(in view: MTKView) {
        drawCallback?(userData, Unmanaged.passUnretained(view).toOpaque())
    }
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
    return (Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as! CGColorSpace)
}

@_cdecl("mtk_view_delegate_new")
func mtk_view_delegate_new(
    _ drawCallback: MTKViewDrawCallback?,
    _ resizeCallback: MTKViewResizeCallback?,
    _ userData: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    mtkTakeRetained(MTKViewDelegateBox(
        drawCallback: drawCallback,
        resizeCallback: resizeCallback,
        userData: userData
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
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        outError?.pointee = mtkDup("invalid MTKView")
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
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self),
          let delegate = view.delegate as AnyObject?
    else {
        return nil
    }
    return mtkTakeRetained(delegate)
}

@_cdecl("mtk_view_set_delegate")
func mtk_view_set_delegate(_ viewPtr: UnsafeMutableRawPointer?, _ delegatePtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.delegate = mtkBorrow(delegatePtr, as: MTKViewDelegateBox.self)
}

@_cdecl("mtk_view_device")
func mtk_view_device(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self),
          let device = view.device
    else {
        return nil
    }
    return Unmanaged.passUnretained(device as AnyObject).toOpaque()
}

@_cdecl("mtk_view_set_device")
func mtk_view_set_device(_ viewPtr: UnsafeMutableRawPointer?, _ devicePtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.device = mtkBorrow(devicePtr, as: MTLDevice.self)
}

@_cdecl("mtk_view_current_drawable")
func mtk_view_current_drawable(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self),
          let drawable = view.currentDrawable
    else {
        return nil
    }
    return Unmanaged.passUnretained(drawable as AnyObject).toOpaque()
}

@_cdecl("mtk_view_framebuffer_only")
func mtk_view_framebuffer_only(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return false
    }
    return view.framebufferOnly
}

@_cdecl("mtk_view_set_framebuffer_only")
func mtk_view_set_framebuffer_only(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.framebufferOnly = value
}

@_cdecl("mtk_view_depth_stencil_attachment_texture_usage")
func mtk_view_depth_stencil_attachment_texture_usage(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return 0
    }
    return view.depthStencilAttachmentTextureUsage.rawValue
}

@_cdecl("mtk_view_set_depth_stencil_attachment_texture_usage")
func mtk_view_set_depth_stencil_attachment_texture_usage(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.depthStencilAttachmentTextureUsage = mtkTextureUsage(from: value)
}

@_cdecl("mtk_view_multisample_color_attachment_texture_usage")
func mtk_view_multisample_color_attachment_texture_usage(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return 0
    }
    return view.multisampleColorAttachmentTextureUsage.rawValue
}

@_cdecl("mtk_view_set_multisample_color_attachment_texture_usage")
func mtk_view_set_multisample_color_attachment_texture_usage(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.multisampleColorAttachmentTextureUsage = mtkTextureUsage(from: value)
}

@_cdecl("mtk_view_presents_with_transaction")
func mtk_view_presents_with_transaction(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return false
    }
    return view.presentsWithTransaction
}

@_cdecl("mtk_view_set_presents_with_transaction")
func mtk_view_set_presents_with_transaction(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.presentsWithTransaction = value
}

@_cdecl("mtk_view_color_pixel_format")
func mtk_view_color_pixel_format(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return 0
    }
    return view.colorPixelFormat.rawValue
}

@_cdecl("mtk_view_set_color_pixel_format")
func mtk_view_set_color_pixel_format(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.colorPixelFormat = mtkPixelFormat(from: value)
}

@_cdecl("mtk_view_depth_stencil_pixel_format")
func mtk_view_depth_stencil_pixel_format(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return 0
    }
    return view.depthStencilPixelFormat.rawValue
}

@_cdecl("mtk_view_set_depth_stencil_pixel_format")
func mtk_view_set_depth_stencil_pixel_format(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.depthStencilPixelFormat = mtkPixelFormat(from: value)
}

@_cdecl("mtk_view_depth_stencil_storage_mode")
func mtk_view_depth_stencil_storage_mode(_ viewPtr: UnsafeMutableRawPointer?) -> UInt {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return 0
    }
    if #available(macOS 13.0, *) {
        return view.depthStencilStorageMode.rawValue
    }
    return MTLStorageMode.private.rawValue
}

@_cdecl("mtk_view_set_depth_stencil_storage_mode")
func mtk_view_set_depth_stencil_storage_mode(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    if #available(macOS 13.0, *) {
        view.depthStencilStorageMode = mtkStorageMode(from: value)
    }
}

@_cdecl("mtk_view_sample_count")
func mtk_view_sample_count(_ viewPtr: UnsafeMutableRawPointer?) -> Int {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return 0
    }
    return view.sampleCount
}

@_cdecl("mtk_view_set_sample_count")
func mtk_view_set_sample_count(_ viewPtr: UnsafeMutableRawPointer?, _ value: Int) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
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
    let color = (mtkBorrow(viewPtr, as: MTKView.self)?.clearColor) ?? MTLClearColorMake(0, 0, 0, 1)
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
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.clearColor = MTLClearColorMake(red, green, blue, alpha)
}

@_cdecl("mtk_view_clear_depth")
func mtk_view_clear_depth(_ viewPtr: UnsafeMutableRawPointer?) -> Double {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return 1.0
    }
    return view.clearDepth
}

@_cdecl("mtk_view_set_clear_depth")
func mtk_view_set_clear_depth(_ viewPtr: UnsafeMutableRawPointer?, _ value: Double) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.clearDepth = value
}

@_cdecl("mtk_view_clear_stencil")
func mtk_view_clear_stencil(_ viewPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return 0
    }
    return view.clearStencil
}

@_cdecl("mtk_view_set_clear_stencil")
func mtk_view_set_clear_stencil(_ viewPtr: UnsafeMutableRawPointer?, _ value: UInt32) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.clearStencil = value
}

@_cdecl("mtk_view_depth_stencil_texture")
func mtk_view_depth_stencil_texture(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self),
          let texture = view.depthStencilTexture
    else {
        return nil
    }
    return Unmanaged.passUnretained(texture as AnyObject).toOpaque()
}

@_cdecl("mtk_view_multisample_color_texture")
func mtk_view_multisample_color_texture(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self),
          let texture = view.multisampleColorTexture
    else {
        return nil
    }
    return Unmanaged.passUnretained(texture as AnyObject).toOpaque()
}

@_cdecl("mtk_view_release_drawables")
func mtk_view_release_drawables(_ viewPtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.releaseDrawables()
}

@_cdecl("mtk_view_current_render_pass_descriptor")
func mtk_view_current_render_pass_descriptor(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self),
          let descriptor = view.currentRenderPassDescriptor
    else {
        return nil
    }
    return Unmanaged.passUnretained(descriptor).toOpaque()
}

@_cdecl("mtk_view_current_mtl4_render_pass_descriptor")
func mtk_view_current_mtl4_render_pass_descriptor(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return nil
    }
    if #available(macOS 26.0, *) {
        guard let descriptor = view.currentMTL4RenderPassDescriptor else {
            return nil
        }
        return Unmanaged.passUnretained(descriptor).toOpaque()
    }
    return nil
}

@_cdecl("mtk_view_preferred_frames_per_second")
func mtk_view_preferred_frames_per_second(_ viewPtr: UnsafeMutableRawPointer?) -> Int {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return 0
    }
    return view.preferredFramesPerSecond
}

@_cdecl("mtk_view_set_preferred_frames_per_second")
func mtk_view_set_preferred_frames_per_second(_ viewPtr: UnsafeMutableRawPointer?, _ value: Int) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.preferredFramesPerSecond = value
}

@_cdecl("mtk_view_enable_set_needs_display")
func mtk_view_enable_set_needs_display(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return false
    }
    return view.enableSetNeedsDisplay
}

@_cdecl("mtk_view_set_enable_set_needs_display")
func mtk_view_set_enable_set_needs_display(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.enableSetNeedsDisplay = value
}

@_cdecl("mtk_view_auto_resize_drawable")
func mtk_view_auto_resize_drawable(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return false
    }
    return view.autoResizeDrawable
}

@_cdecl("mtk_view_set_auto_resize_drawable")
func mtk_view_set_auto_resize_drawable(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
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
    let size = (mtkBorrow(viewPtr, as: MTKView.self)?.drawableSize) ?? .zero
    outWidth?.pointee = size.width
    outHeight?.pointee = size.height
}

@_cdecl("mtk_view_set_drawable_size")
func mtk_view_set_drawable_size(_ viewPtr: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
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
    let size = (mtkBorrow(viewPtr, as: MTKView.self)?.preferredDrawableSize) ?? .zero
    outWidth?.pointee = size.width
    outHeight?.pointee = size.height
}

@_cdecl("mtk_view_preferred_device")
func mtk_view_preferred_device(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self),
          let device = view.preferredDevice
    else {
        return nil
    }
    return Unmanaged.passUnretained(device as AnyObject).toOpaque()
}

@_cdecl("mtk_view_is_paused")
func mtk_view_is_paused(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return false
    }
    return view.isPaused
}

@_cdecl("mtk_view_set_paused")
func mtk_view_set_paused(_ viewPtr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.isPaused = value
}

@_cdecl("mtk_view_colorspace")
func mtk_view_colorspace(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self),
          let colorspace = view.colorspace
    else {
        return nil
    }
    return Unmanaged.passUnretained(colorspace as AnyObject).toOpaque()
}

@_cdecl("mtk_view_set_colorspace")
func mtk_view_set_colorspace(_ viewPtr: UnsafeMutableRawPointer?, _ value: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.colorspace = mtkCGColorSpace(value)
}

@_cdecl("mtk_view_draw")
func mtk_view_draw(_ viewPtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.draw()
}

@_cdecl("mtk_view_notify_delegate_size_will_change")
func mtk_view_notify_delegate_size_will_change(_ viewPtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.delegate?.mtkView(view, drawableSizeWillChange: view.drawableSize)
}

@_cdecl("mtk_view_notify_delegate_draw")
func mtk_view_notify_delegate_draw(_ viewPtr: UnsafeMutableRawPointer?) {
    guard let view: MTKView = mtkBorrow(viewPtr, as: MTKView.self) else {
        return
    }
    view.delegate?.draw(in: view)
}
