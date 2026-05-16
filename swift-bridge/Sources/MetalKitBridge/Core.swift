import Foundation
import MetalKit

private struct TextureLoaderOptionsRaw {
    var flags: UInt32
    var generateMipmaps: UInt8
    var allocateMipmaps: UInt8
    var srgb: UInt8
    var reserved: UInt8
    var textureUsage: UInt64
    var textureStorageMode: UInt64
    var textureCPUCacheMode: UInt64
}

private let optionGenerateMipmaps: UInt32 = 1 << 0
private let optionAllocateMipmaps: UInt32 = 1 << 1
private let optionSRGB: UInt32 = 1 << 2
private let optionTextureUsage: UInt32 = 1 << 3
private let optionTextureStorageMode: UInt32 = 1 << 4
private let optionTextureCPUCacheMode: UInt32 = 1 << 5

func mtkRetain(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let ptr else { return nil }
    let object = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue()
    return Unmanaged.passRetained(object).toOpaque()
}

func mtkRelease(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

func mtkTakeRetained(_ object: AnyObject?) -> UnsafeMutableRawPointer? {
    guard let object else { return nil }
    return Unmanaged.passRetained(object).toOpaque()
}

func mtkBorrow<T>(_ ptr: UnsafeMutableRawPointer?, as _: T.Type) -> T? {
    guard let ptr else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as? T
}

func mtkDup(_ value: String?) -> UnsafeMutablePointer<CChar>? {
    guard let value else { return nil }
    return strdup(value)
}

func mtkNSErrorMessage(_ error: NSError?) -> UnsafeMutablePointer<CChar>? {
    guard let error else { return nil }
    return mtkDup("\(error.domain)(\(error.code)): \(error.localizedDescription)")
}

func mtkTextureLoaderOptions(from rawPtr: UnsafeRawPointer?) -> [MTKTextureLoader.Option: Any]? {
    guard let rawPtr else { return nil }
    let raw = rawPtr.assumingMemoryBound(to: TextureLoaderOptionsRaw.self).pointee
    var options: [MTKTextureLoader.Option: Any] = [:]

    if raw.flags & optionGenerateMipmaps != 0 {
        options[.generateMipmaps] = raw.generateMipmaps != 0
    }
    if raw.flags & optionAllocateMipmaps != 0 {
        options[.allocateMipmaps] = raw.allocateMipmaps != 0
    }
    if raw.flags & optionSRGB != 0 {
        options[.SRGB] = raw.srgb != 0
    }
    if raw.flags & optionTextureUsage != 0 {
        options[.textureUsage] = NSNumber(value: raw.textureUsage)
    }
    if raw.flags & optionTextureStorageMode != 0 {
        options[.textureStorageMode] = NSNumber(value: raw.textureStorageMode)
    }
    if raw.flags & optionTextureCPUCacheMode != 0 {
        options[.textureCPUCacheMode] = NSNumber(value: raw.textureCPUCacheMode)
    }

    return options.isEmpty ? nil : options
}

@_cdecl("mtk_retain")
public func mtk_retain(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    mtkRetain(ptr)
}

@_cdecl("mtk_release")
public func mtk_release(_ ptr: UnsafeMutableRawPointer?) {
    mtkRelease(ptr)
}
