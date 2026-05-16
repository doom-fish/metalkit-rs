import CoreGraphics
import Foundation
import Metal
import MetalKit

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
    let bundle = bundlePath.map { Bundle(path: String(cString: $0)) }
    do {
        let texture = try loader.newTexture(
            name: String(cString: name),
            scaleFactor: CGFloat(scaleFactor),
            bundle: bundle ?? nil,
            options: options
        )
        outError?.pointee = nil
        return mtkTakeRetained(texture as AnyObject)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
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
    let image = Unmanaged<AnyObject>.fromOpaque(imagePtr).takeUnretainedValue()
    let cgImage = image as! CGImage

    do {
        let texture = try loader.newTexture(cgImage: cgImage, options: options)
        outError?.pointee = nil
        return mtkTakeRetained(texture as AnyObject)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}
