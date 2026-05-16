import Foundation
import MetalKit

struct MTKRectRaw {
    var x: Double
    var y: Double
    var width: Double
    var height: Double
}

struct MTKSizeRaw {
    var width: Double
    var height: Double
}

struct MTKClearColorRaw {
    var red: Double
    var green: Double
    var blue: Double
    var alpha: Double
}

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

func mtkStrings(
    _ items: UnsafePointer<UnsafePointer<CChar>>?,
    count: Int
) -> [String]? {
    if count == 0 {
        return []
    }
    guard let items else { return nil }
    return (0..<count).map { String(cString: items[$0]) }
}

func mtkJSON<T: Encodable>(_ value: T) -> UnsafeMutablePointer<CChar>? {
    do {
        let data = try JSONEncoder().encode(value)
        let string = String(decoding: data, as: UTF8.self)
        return strdup(string)
    } catch {
        return mtkDup("failed to encode JSON: \(error)")
    }
}

@_cdecl("mtk_retain")
public func mtk_retain(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    mtkRetain(ptr)
}

@_cdecl("mtk_release")
public func mtk_release(_ ptr: UnsafeMutableRawPointer?) {
    mtkRelease(ptr)
}
