import CoreGraphics
import Foundation
import Metal
import MetalKit
import ModelIO

private struct TextureLoaderOptionsRaw {
    var flags: UInt32
    var generateMipmaps: UInt8
    var allocateMipmaps: UInt8
    var srgb: UInt8
    var loadAsArray: UInt8
    var textureUsage: UInt64
    var textureStorageMode: UInt64
    var textureCPUCacheMode: UInt64
    var cubeLayout: UnsafePointer<CChar>?
    var origin: UnsafePointer<CChar>?
}

private let optionGenerateMipmaps: UInt32 = 1 << 0
private let optionAllocateMipmaps: UInt32 = 1 << 1
private let optionSRGB: UInt32 = 1 << 2
private let optionTextureUsage: UInt32 = 1 << 3
private let optionTextureStorageMode: UInt32 = 1 << 4
private let optionTextureCPUCacheMode: UInt32 = 1 << 5
private let optionCubeLayout: UInt32 = 1 << 6
private let optionOrigin: UInt32 = 1 << 7
private let optionLoadAsArray: UInt32 = 1 << 8

final class MTKTextureArrayBox: NSObject {
    let textures: [AnyObject?]

    init(_ textures: [AnyObject?]) {
        self.textures = textures
    }
}

public typealias MTKTextureLoaderRustCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<CChar>?
) -> Void

public typealias MTKTextureLoaderRustArrayCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<CChar>?
) -> Void

private func mtkCompleteTextureLoad(
    callback: MTKTextureLoaderRustCallback?,
    userData: UnsafeMutableRawPointer?,
    texture: MTLTexture?,
    error: Error?
) {
    guard let callback else { return }
    callback(userData, mtkTakeRetained(texture.map { $0 as AnyObject }), mtkNSErrorMessage(error as NSError?))
}

private func mtkCompleteTextureArrayLoad(
    callback: MTKTextureLoaderRustArrayCallback?,
    userData: UnsafeMutableRawPointer?,
    textures: [AnyObject],
    expectedCount: Int,
    error: Error?
) {
    guard let callback else { return }
    var boxedTextures = textures.map { texture in
        texture is NSNull ? nil : texture
    }
    if boxedTextures.count < expectedCount {
        boxedTextures.append(contentsOf: repeatElement(nil, count: expectedCount - boxedTextures.count))
    } else if boxedTextures.count > expectedCount {
        boxedTextures = Array(boxedTextures.prefix(expectedCount))
    }
    let result = MTKTextureArrayBox(boxedTextures)
    callback(userData, mtkTakeRetained(result), mtkNSErrorMessage(error as NSError?))
}

private func mtkCompleteTextureLoadWithMessage(
    callback: MTKTextureLoaderRustCallback?,
    userData: UnsafeMutableRawPointer?,
    message: String
) {
    callback?(userData, nil, mtkDup(message))
}

private func mtkCompleteTextureArrayLoadWithMessage(
    callback: MTKTextureLoaderRustArrayCallback?,
    userData: UnsafeMutableRawPointer?,
    message: String
) {
    callback?(userData, nil, mtkDup(message))
}

private func mtkTextureLoaderOptions(from rawPtr: UnsafeRawPointer?) -> [MTKTextureLoader.Option: Any]? {
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
    if raw.flags & optionCubeLayout != 0, let cubeLayout = raw.cubeLayout {
        options[.cubeLayout] = String(cString: cubeLayout)
    }
    if raw.flags & optionOrigin != 0, let origin = raw.origin {
        options[.origin] = String(cString: origin)
    }
    if raw.flags & optionLoadAsArray != 0 {
        if #available(macOS 14.0, *) {
            options[.loadAsArray] = raw.loadAsArray != 0
        }
    }

    return options.isEmpty ? nil : options
}

private func mtkTexturePaths(
    _ paths: UnsafePointer<UnsafePointer<CChar>>?,
    count: Int
) -> [URL]? {
    mtkStrings(paths, count: count)?.map(URL.init(fileURLWithPath:))
}

@_cdecl("mtk_texture_array_count")
public func mtk_texture_array_count(_ resultPtr: UnsafeMutableRawPointer?) -> Int {
    guard let box: MTKTextureArrayBox = mtkBorrow(resultPtr, as: MTKTextureArrayBox.self) else {
        return 0
    }
    return box.textures.count
}

@_cdecl("mtk_texture_array_texture_at")
public func mtk_texture_array_texture_at(
    _ resultPtr: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let box: MTKTextureArrayBox = mtkBorrow(resultPtr, as: MTKTextureArrayBox.self),
          index >= 0,
          index < box.textures.count,
          let texture = box.textures[index]
    else {
        return nil
    }
    return mtkTakeRetained(texture)
}

@_cdecl("mtk_texture_loader_new")
public func mtk_texture_loader_new(_ devicePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let device: MTLDevice = mtkBorrow(devicePtr, as: MTLDevice.self) else {
        return nil
    }
    return mtkTakeRetained(MTKTextureLoader(device: device))
}

@_cdecl("mtk_texture_loader_device")
public func mtk_texture_loader_device(_ loaderPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self) else {
        return nil
    }
    return Unmanaged.passUnretained(loader.device as AnyObject).toOpaque()
}

@_cdecl("mtk_texture_loader_new_texture_from_url")
public func mtk_texture_loader_new_texture_from_url(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let path
    else {
        outError?.pointee = mtkDup("invalid MTKTextureLoader or path")
        return nil
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    do {
        let url = URL(fileURLWithPath: String(cString: path))
        let texture = try loader.newTexture(URL: url, options: options)
        outError?.pointee = nil
        return mtkTakeRetained(texture as AnyObject)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}

@_cdecl("mtk_texture_loader_new_texture_from_url_with_callback")
public func mtk_texture_loader_new_texture_from_url_with_callback(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ callback: MTKTextureLoaderRustCallback?,
    _ userData: UnsafeMutableRawPointer?
) {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let path
    else {
        mtkCompleteTextureLoadWithMessage(
            callback: callback,
            userData: userData,
            message: "invalid MTKTextureLoader or path"
        )
        return
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let url = URL(fileURLWithPath: String(cString: path))
    loader.newTexture(URL: url, options: options) { texture, error in
        mtkCompleteTextureLoad(callback: callback, userData: userData, texture: texture, error: error)
    }
}

@_cdecl("mtk_texture_loader_new_textures_from_urls")
public func mtk_texture_loader_new_textures_from_urls(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ paths: UnsafePointer<UnsafePointer<CChar>>?,
    _ count: Int,
    _ optionsPtr: UnsafeRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let urls = mtkTexturePaths(paths, count: count)
    else {
        outError?.pointee = mtkDup("invalid MTKTextureLoader or URL list")
        return nil
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    var firstError: NSError?
    var textures: [AnyObject?] = []
    textures.reserveCapacity(urls.count)

    for url in urls {
        do {
            let texture = try loader.newTexture(URL: url, options: options)
            textures.append(texture as AnyObject)
        } catch let error as NSError {
            if firstError == nil {
                firstError = error
            }
            textures.append(nil)
        }
    }

    outError?.pointee = mtkNSErrorMessage(firstError)
    return mtkTakeRetained(MTKTextureArrayBox(textures))
}

@_cdecl("mtk_texture_loader_new_textures_from_urls_with_callback")
public func mtk_texture_loader_new_textures_from_urls_with_callback(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ paths: UnsafePointer<UnsafePointer<CChar>>?,
    _ count: Int,
    _ optionsPtr: UnsafeRawPointer?,
    _ callback: MTKTextureLoaderRustArrayCallback?,
    _ userData: UnsafeMutableRawPointer?
) {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let urls = mtkTexturePaths(paths, count: count)
    else {
        mtkCompleteTextureArrayLoadWithMessage(
            callback: callback,
            userData: userData,
            message: "invalid MTKTextureLoader or URL list"
        )
        return
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    loader.newTextures(URLs: urls, options: options) { textures, error in
        mtkCompleteTextureArrayLoad(
            callback: callback,
            userData: userData,
            textures: textures,
            expectedCount: urls.count,
            error: error
        )
    }
}

@_cdecl("mtk_texture_loader_new_texture_named")
public func mtk_texture_loader_new_texture_named(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ scaleFactor: Double,
    _ bundlePath: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let name
    else {
        outError?.pointee = mtkDup("invalid MTKTextureLoader or texture name")
        return nil
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let bundle = bundlePath.flatMap { Bundle(path: String(cString: $0)) }
    do {
        let texture = try loader.newTexture(
            name: String(cString: name),
            scaleFactor: CGFloat(scaleFactor),
            bundle: bundle,
            options: options
        )
        outError?.pointee = nil
        return mtkTakeRetained(texture as AnyObject)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}

@_cdecl("mtk_texture_loader_new_texture_named_with_callback")
public func mtk_texture_loader_new_texture_named_with_callback(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ scaleFactor: Double,
    _ bundlePath: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ callback: MTKTextureLoaderRustCallback?,
    _ userData: UnsafeMutableRawPointer?
) {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let name
    else {
        mtkCompleteTextureLoadWithMessage(
            callback: callback,
            userData: userData,
            message: "invalid MTKTextureLoader or texture name"
        )
        return
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let bundle = bundlePath.flatMap { Bundle(path: String(cString: $0)) }
    loader.newTexture(
        name: String(cString: name),
        scaleFactor: CGFloat(scaleFactor),
        bundle: bundle,
        options: options
    ) { texture, error in
        mtkCompleteTextureLoad(callback: callback, userData: userData, texture: texture, error: error)
    }
}

@_cdecl("mtk_texture_loader_new_texture_named_with_display_gamut")
public func mtk_texture_loader_new_texture_named_with_display_gamut(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ scaleFactor: Double,
    _ displayGamut: Int,
    _ bundlePath: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let name,
          let gamut = NSDisplayGamut(rawValue: displayGamut)
    else {
        outError?.pointee = mtkDup("invalid MTKTextureLoader, texture name, or display gamut")
        return nil
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let bundle = bundlePath.flatMap { Bundle(path: String(cString: $0)) }
    do {
        let texture = try loader.newTexture(
            name: String(cString: name),
            scaleFactor: CGFloat(scaleFactor),
            displayGamut: gamut,
            bundle: bundle,
            options: options
        )
        outError?.pointee = nil
        return mtkTakeRetained(texture as AnyObject)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}

@_cdecl("mtk_texture_loader_new_texture_named_with_display_gamut_with_callback")
public func mtk_texture_loader_new_texture_named_with_display_gamut_with_callback(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ scaleFactor: Double,
    _ displayGamut: Int,
    _ bundlePath: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ callback: MTKTextureLoaderRustCallback?,
    _ userData: UnsafeMutableRawPointer?
) {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let name,
          let gamut = NSDisplayGamut(rawValue: displayGamut)
    else {
        mtkCompleteTextureLoadWithMessage(
            callback: callback,
            userData: userData,
            message: "invalid MTKTextureLoader, texture name, or display gamut"
        )
        return
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let bundle = bundlePath.flatMap { Bundle(path: String(cString: $0)) }
    loader.newTexture(
        name: String(cString: name),
        scaleFactor: CGFloat(scaleFactor),
        displayGamut: gamut,
        bundle: bundle,
        options: options
    ) { texture, error in
        mtkCompleteTextureLoad(callback: callback, userData: userData, texture: texture, error: error)
    }
}

@_cdecl("mtk_texture_loader_new_textures_named")
public func mtk_texture_loader_new_textures_named(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ names: UnsafePointer<UnsafePointer<CChar>>?,
    _ count: Int,
    _ scaleFactor: Double,
    _ bundlePath: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let names = mtkStrings(names, count: count)
    else {
        outError?.pointee = mtkDup("invalid MTKTextureLoader or texture name list")
        return nil
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let bundle = bundlePath.flatMap { Bundle(path: String(cString: $0)) }
    var firstError: NSError?
    var textures: [AnyObject?] = []
    textures.reserveCapacity(names.count)

    for name in names {
        do {
            let texture = try loader.newTexture(
                name: name,
                scaleFactor: CGFloat(scaleFactor),
                bundle: bundle,
                options: options
            )
            textures.append(texture as AnyObject)
        } catch let error as NSError {
            if firstError == nil {
                firstError = error
            }
            textures.append(nil)
        }
    }

    outError?.pointee = mtkNSErrorMessage(firstError)
    return mtkTakeRetained(MTKTextureArrayBox(textures))
}

@_cdecl("mtk_texture_loader_new_textures_named_with_callback")
public func mtk_texture_loader_new_textures_named_with_callback(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ names: UnsafePointer<UnsafePointer<CChar>>?,
    _ count: Int,
    _ scaleFactor: Double,
    _ bundlePath: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ callback: MTKTextureLoaderRustArrayCallback?,
    _ userData: UnsafeMutableRawPointer?
) {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let names = mtkStrings(names, count: count)
    else {
        mtkCompleteTextureArrayLoadWithMessage(
            callback: callback,
            userData: userData,
            message: "invalid MTKTextureLoader or texture name list"
        )
        return
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let bundle = bundlePath.flatMap { Bundle(path: String(cString: $0)) }
    loader.newTextures(
        names: names,
        scaleFactor: CGFloat(scaleFactor),
        bundle: bundle,
        options: options
    ) { textures, error in
        mtkCompleteTextureArrayLoad(
            callback: callback,
            userData: userData,
            textures: textures,
            expectedCount: names.count,
            error: error
        )
    }
}

@_cdecl("mtk_texture_loader_new_textures_named_with_display_gamut")
public func mtk_texture_loader_new_textures_named_with_display_gamut(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ names: UnsafePointer<UnsafePointer<CChar>>?,
    _ count: Int,
    _ scaleFactor: Double,
    _ displayGamut: Int,
    _ bundlePath: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let names = mtkStrings(names, count: count),
          let gamut = NSDisplayGamut(rawValue: displayGamut)
    else {
        outError?.pointee = mtkDup("invalid MTKTextureLoader, texture name list, or display gamut")
        return nil
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let bundle = bundlePath.flatMap { Bundle(path: String(cString: $0)) }
    var firstError: NSError?
    var textures: [AnyObject?] = []
    textures.reserveCapacity(names.count)

    for name in names {
        do {
            let texture = try loader.newTexture(
                name: name,
                scaleFactor: CGFloat(scaleFactor),
                displayGamut: gamut,
                bundle: bundle,
                options: options
            )
            textures.append(texture as AnyObject)
        } catch let error as NSError {
            if firstError == nil {
                firstError = error
            }
            textures.append(nil)
        }
    }

    outError?.pointee = mtkNSErrorMessage(firstError)
    return mtkTakeRetained(MTKTextureArrayBox(textures))
}

@_cdecl("mtk_texture_loader_new_textures_named_with_display_gamut_with_callback")
public func mtk_texture_loader_new_textures_named_with_display_gamut_with_callback(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ names: UnsafePointer<UnsafePointer<CChar>>?,
    _ count: Int,
    _ scaleFactor: Double,
    _ displayGamut: Int,
    _ bundlePath: UnsafePointer<CChar>?,
    _ optionsPtr: UnsafeRawPointer?,
    _ callback: MTKTextureLoaderRustArrayCallback?,
    _ userData: UnsafeMutableRawPointer?
) {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let names = mtkStrings(names, count: count),
          let gamut = NSDisplayGamut(rawValue: displayGamut)
    else {
        mtkCompleteTextureArrayLoadWithMessage(
            callback: callback,
            userData: userData,
            message: "invalid MTKTextureLoader, texture name list, or display gamut"
        )
        return
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let bundle = bundlePath.flatMap { Bundle(path: String(cString: $0)) }
    loader.newTextures(
        names: names,
        scaleFactor: CGFloat(scaleFactor),
        displayGamut: gamut,
        bundle: bundle,
        options: options
    ) { textures, error in
        mtkCompleteTextureArrayLoad(
            callback: callback,
            userData: userData,
            textures: textures,
            expectedCount: names.count,
            error: error
        )
    }
}

@_cdecl("mtk_texture_loader_new_texture_from_data")
public func mtk_texture_loader_new_texture_from_data(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ bytes: UnsafeRawPointer?,
    _ len: Int,
    _ optionsPtr: UnsafeRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self) else {
        outError?.pointee = mtkDup("invalid MTKTextureLoader")
        return nil
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let data = len == 0 ? Data() : Data(bytes: bytes!, count: len)
    do {
        let texture = try loader.newTexture(data: data, options: options)
        outError?.pointee = nil
        return mtkTakeRetained(texture as AnyObject)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}

@_cdecl("mtk_texture_loader_new_texture_from_data_with_callback")
public func mtk_texture_loader_new_texture_from_data_with_callback(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ bytes: UnsafeRawPointer?,
    _ len: Int,
    _ optionsPtr: UnsafeRawPointer?,
    _ callback: MTKTextureLoaderRustCallback?,
    _ userData: UnsafeMutableRawPointer?
) {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self) else {
        mtkCompleteTextureLoadWithMessage(callback: callback, userData: userData, message: "invalid MTKTextureLoader")
        return
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let data = len == 0 ? Data() : Data(bytes: bytes!, count: len)
    loader.newTexture(data: data, options: options) { texture, error in
        mtkCompleteTextureLoad(callback: callback, userData: userData, texture: texture, error: error)
    }
}

@_cdecl("mtk_texture_loader_new_texture_from_cgimage")
public func mtk_texture_loader_new_texture_from_cgimage(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ imagePtr: UnsafeMutableRawPointer?,
    _ optionsPtr: UnsafeRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let imagePtr
    else {
        outError?.pointee = mtkDup("invalid MTKTextureLoader or CGImage")
        return nil
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let image = Unmanaged<AnyObject>.fromOpaque(imagePtr).takeUnretainedValue() as! CGImage

    do {
        let texture = try loader.newTexture(cgImage: image, options: options)
        outError?.pointee = nil
        return mtkTakeRetained(texture as AnyObject)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}

@_cdecl("mtk_texture_loader_new_texture_from_cgimage_with_callback")
public func mtk_texture_loader_new_texture_from_cgimage_with_callback(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ imagePtr: UnsafeMutableRawPointer?,
    _ optionsPtr: UnsafeRawPointer?,
    _ callback: MTKTextureLoaderRustCallback?,
    _ userData: UnsafeMutableRawPointer?
) {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let imagePtr
    else {
        mtkCompleteTextureLoadWithMessage(
            callback: callback,
            userData: userData,
            message: "invalid MTKTextureLoader or CGImage"
        )
        return
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    let image = Unmanaged<AnyObject>.fromOpaque(imagePtr).takeUnretainedValue() as! CGImage
    loader.newTexture(cgImage: image, options: options) { texture, error in
        mtkCompleteTextureLoad(callback: callback, userData: userData, texture: texture, error: error)
    }
}

@_cdecl("mtk_texture_loader_new_texture_from_model_texture")
public func mtk_texture_loader_new_texture_from_model_texture(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ texturePtr: UnsafeMutableRawPointer?,
    _ optionsPtr: UnsafeRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let texture: MDLTexture = mtkBorrow(texturePtr, as: MDLTexture.self)
    else {
        outError?.pointee = mtkDup("invalid MTKTextureLoader or MDLTexture")
        return nil
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    do {
        let metalTexture = try loader.newTexture(texture: texture, options: options)
        outError?.pointee = nil
        return mtkTakeRetained(metalTexture as AnyObject)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}

@_cdecl("mtk_texture_loader_new_texture_from_model_texture_with_callback")
public func mtk_texture_loader_new_texture_from_model_texture_with_callback(
    _ loaderPtr: UnsafeMutableRawPointer?,
    _ texturePtr: UnsafeMutableRawPointer?,
    _ optionsPtr: UnsafeRawPointer?,
    _ callback: MTKTextureLoaderRustCallback?,
    _ userData: UnsafeMutableRawPointer?
) {
    guard let loader: MTKTextureLoader = mtkBorrow(loaderPtr, as: MTKTextureLoader.self),
          let texture: MDLTexture = mtkBorrow(texturePtr, as: MDLTexture.self)
    else {
        mtkCompleteTextureLoadWithMessage(
            callback: callback,
            userData: userData,
            message: "invalid MTKTextureLoader or MDLTexture"
        )
        return
    }

    let options = mtkTextureLoaderOptions(from: optionsPtr)
    loader.newTexture(texture: texture, options: options) { metalTexture, error in
        mtkCompleteTextureLoad(
            callback: callback,
            userData: userData,
            texture: metalTexture,
            error: error
        )
    }
}
