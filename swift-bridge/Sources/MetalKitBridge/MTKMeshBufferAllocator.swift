import Foundation
import Metal
import MetalKit

private func mtkMeshBufferType(from rawValue: Int) -> MDLMeshBufferType {
    MDLMeshBufferType(rawValue: UInt(rawValue)) ?? .vertex
}

@_cdecl("mtk_mesh_buffer_allocator_new")
public func mtk_mesh_buffer_allocator_new(_ devicePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let device: MTLDevice = mtkBorrow(devicePtr, as: MTLDevice.self) else {
        return nil
    }
    return mtkTakeRetained(MTKMeshBufferAllocator(device: device))
}

@_cdecl("mtk_mesh_buffer_allocator_device")
public func mtk_mesh_buffer_allocator_device(_ allocatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let allocator: MTKMeshBufferAllocator = mtkBorrow(allocatorPtr, as: MTKMeshBufferAllocator.self) else {
        return nil
    }
    return Unmanaged.passUnretained(allocator.device as AnyObject).toOpaque()
}

@_cdecl("mtk_mesh_buffer_allocator_new_buffer")
public func mtk_mesh_buffer_allocator_new_buffer(
    _ allocatorPtr: UnsafeMutableRawPointer?,
    _ length: Int,
    _ bufferType: Int
) -> UnsafeMutableRawPointer? {
    guard let allocator: MTKMeshBufferAllocator = mtkBorrow(allocatorPtr, as: MTKMeshBufferAllocator.self) else {
        return nil
    }
    let buffer = allocator.newBuffer(from: nil, length: length, type: mtkMeshBufferType(from: bufferType)) as? MTKMeshBuffer
    return mtkTakeRetained(buffer)
}

@_cdecl("mtk_mesh_buffer_allocator_new_buffer_with_data")
public func mtk_mesh_buffer_allocator_new_buffer_with_data(
    _ allocatorPtr: UnsafeMutableRawPointer?,
    _ bytes: UnsafeRawPointer?,
    _ len: Int,
    _ bufferType: Int
) -> UnsafeMutableRawPointer? {
    guard let allocator: MTKMeshBufferAllocator = mtkBorrow(allocatorPtr, as: MTKMeshBufferAllocator.self) else {
        return nil
    }
    let data = len == 0 ? Data() : Data(bytes: bytes!, count: len)
    let buffer = allocator.newBuffer(with: data, type: mtkMeshBufferType(from: bufferType)) as? MTKMeshBuffer
    return mtkTakeRetained(buffer)
}
