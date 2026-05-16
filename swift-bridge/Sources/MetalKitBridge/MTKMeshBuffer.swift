import Foundation
import Metal
import MetalKit
import ModelIO

@_cdecl("mtk_mesh_buffer_length")
public func mtk_mesh_buffer_length(_ bufferPtr: UnsafeMutableRawPointer?) -> Int {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self) else {
        return 0
    }
    return buffer.length
}

@_cdecl("mtk_mesh_buffer_allocator")
public func mtk_mesh_buffer_allocator(_ bufferPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self) else {
        return nil
    }
    return mtkTakeRetained(buffer.allocator)
}

@_cdecl("mtk_mesh_buffer_zone")
public func mtk_mesh_buffer_zone(_ bufferPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self),
          let zone = buffer.zone as AnyObject?
    else {
        return nil
    }
    return mtkTakeRetained(zone)
}

@_cdecl("mtk_mesh_buffer_offset")
public func mtk_mesh_buffer_offset(_ bufferPtr: UnsafeMutableRawPointer?) -> Int {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self) else {
        return 0
    }
    return buffer.offset
}

@_cdecl("mtk_mesh_buffer_type")
public func mtk_mesh_buffer_type(_ bufferPtr: UnsafeMutableRawPointer?) -> Int {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self) else {
        return 0
    }
    return Int(buffer.type.rawValue)
}

@_cdecl("mtk_mesh_buffer_metal_buffer")
public func mtk_mesh_buffer_metal_buffer(_ bufferPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self) else {
        return nil
    }
    return Unmanaged.passUnretained(buffer.buffer as AnyObject).toOpaque()
}

@_cdecl("mtk_mesh_buffer_copy_bytes")
public func mtk_mesh_buffer_copy_bytes(
    _ bufferPtr: UnsafeMutableRawPointer?,
    _ dst: UnsafeMutableRawPointer?,
    _ len: Int
) -> Int {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self),
          let dst,
          len > 0
    else {
        return 0
    }

    let contents = buffer.buffer.contents()
    let available = max(0, buffer.length - buffer.offset)
    let count = min(len, available)
    memcpy(dst, contents.advanced(by: buffer.offset), count)
    return count
}

@_cdecl("mtk_mesh_buffer_get_name")
public func mtk_mesh_buffer_get_name(_ bufferPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self) else {
        return nil
    }
    return mtkDup(buffer.name)
}

@_cdecl("mtk_mesh_buffer_set_name")
public func mtk_mesh_buffer_set_name(_ bufferPtr: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self),
          let name
    else {
        return
    }
    buffer.name = String(cString: name)
}
