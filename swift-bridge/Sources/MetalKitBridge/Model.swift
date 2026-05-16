import Foundation
import Metal
import MetalKit
import ModelIO
import simd

private func mtkMeshBufferType(from rawValue: Int) -> MDLMeshBufferType {
    MDLMeshBufferType(rawValue: UInt(rawValue)) ?? .vertex
}

private func mtkGeometryType(from rawValue: Int) -> MDLGeometryType {
    MDLGeometryType(rawValue: rawValue) ?? .triangles
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

@_cdecl("mtk_mesh_buffer_length")
public func mtk_mesh_buffer_length(_ bufferPtr: UnsafeMutableRawPointer?) -> Int {
    guard let buffer: MTKMeshBuffer = mtkBorrow(bufferPtr, as: MTKMeshBuffer.self) else {
        return 0
    }
    return buffer.length
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

@_cdecl("mtk_model_mesh_new_box")
public func mtk_model_mesh_new_box(
    _ allocatorPtr: UnsafeMutableRawPointer?,
    _ extentX: Float,
    _ extentY: Float,
    _ extentZ: Float,
    _ segmentsX: UInt32,
    _ segmentsY: UInt32,
    _ segmentsZ: UInt32,
    _ inwardNormals: Bool,
    _ geometryType: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let allocator: MTKMeshBufferAllocator = mtkBorrow(allocatorPtr, as: MTKMeshBufferAllocator.self) else {
        outError?.pointee = mtkDup("invalid MTKMeshBufferAllocator")
        return nil
    }

    let mesh = MDLMesh(
        boxWithExtent: SIMD3<Float>(extentX, extentY, extentZ),
        segments: SIMD3<UInt32>(segmentsX, segmentsY, segmentsZ),
        inwardNormals: inwardNormals,
        geometryType: mtkGeometryType(from: Int(geometryType)),
        allocator: allocator
    )
    outError?.pointee = nil
    return mtkTakeRetained(mesh)
}

@_cdecl("mtk_model_mesh_vertex_count")
public func mtk_model_mesh_vertex_count(_ meshPtr: UnsafeMutableRawPointer?) -> Int {
    guard let mesh: MDLMesh = mtkBorrow(meshPtr, as: MDLMesh.self) else {
        return 0
    }
    return mesh.vertexCount
}

@_cdecl("mtk_model_mesh_get_name")
public func mtk_model_mesh_get_name(_ meshPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let mesh: MDLMesh = mtkBorrow(meshPtr, as: MDLMesh.self) else {
        return nil
    }
    return mtkDup(mesh.name)
}

@_cdecl("mtk_model_mesh_set_name")
public func mtk_model_mesh_set_name(_ meshPtr: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let mesh: MDLMesh = mtkBorrow(meshPtr, as: MDLMesh.self),
          let name
    else {
        return
    }
    mesh.name = String(cString: name)
}

@_cdecl("mtk_mesh_new_from_model_mesh")
public func mtk_mesh_new_from_model_mesh(
    _ meshPtr: UnsafeMutableRawPointer?,
    _ devicePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let mesh: MDLMesh = mtkBorrow(meshPtr, as: MDLMesh.self),
          let device: MTLDevice = mtkBorrow(devicePtr, as: MTLDevice.self)
    else {
        outError?.pointee = mtkDup("invalid MDLMesh or MTLDevice")
        return nil
    }

    do {
        let metalMesh = try MTKMesh(mesh: mesh, device: device)
        outError?.pointee = nil
        return mtkTakeRetained(metalMesh)
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}

@_cdecl("mtk_mesh_vertex_count")
public func mtk_mesh_vertex_count(_ meshPtr: UnsafeMutableRawPointer?) -> Int {
    guard let mesh: MTKMesh = mtkBorrow(meshPtr, as: MTKMesh.self) else {
        return 0
    }
    return mesh.vertexCount
}

@_cdecl("mtk_mesh_get_name")
public func mtk_mesh_get_name(_ meshPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let mesh: MTKMesh = mtkBorrow(meshPtr, as: MTKMesh.self) else {
        return nil
    }
    return mtkDup(mesh.name)
}

@_cdecl("mtk_mesh_set_name")
public func mtk_mesh_set_name(_ meshPtr: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let mesh: MTKMesh = mtkBorrow(meshPtr, as: MTKMesh.self),
          let name
    else {
        return
    }
    mesh.name = String(cString: name)
}

@_cdecl("mtk_mesh_vertex_buffer_count")
public func mtk_mesh_vertex_buffer_count(_ meshPtr: UnsafeMutableRawPointer?) -> Int {
    guard let mesh: MTKMesh = mtkBorrow(meshPtr, as: MTKMesh.self) else {
        return 0
    }
    return mesh.vertexBuffers.count
}

@_cdecl("mtk_mesh_vertex_buffer_at")
public func mtk_mesh_vertex_buffer_at(
    _ meshPtr: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let mesh: MTKMesh = mtkBorrow(meshPtr, as: MTKMesh.self),
          index >= 0,
          index < mesh.vertexBuffers.count
    else {
        return nil
    }
    return mtkTakeRetained(mesh.vertexBuffers[index])
}

@_cdecl("mtk_mesh_submesh_count")
public func mtk_mesh_submesh_count(_ meshPtr: UnsafeMutableRawPointer?) -> Int {
    guard let mesh: MTKMesh = mtkBorrow(meshPtr, as: MTKMesh.self) else {
        return 0
    }
    return mesh.submeshes.count
}

@_cdecl("mtk_mesh_submesh_at")
public func mtk_mesh_submesh_at(
    _ meshPtr: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let mesh: MTKMesh = mtkBorrow(meshPtr, as: MTKMesh.self),
          index >= 0,
          index < mesh.submeshes.count
    else {
        return nil
    }
    return mtkTakeRetained(mesh.submeshes[index])
}

@_cdecl("mtk_submesh_primitive_type")
public func mtk_submesh_primitive_type(_ submeshPtr: UnsafeMutableRawPointer?) -> Int {
    guard let submesh: MTKSubmesh = mtkBorrow(submeshPtr, as: MTKSubmesh.self) else {
        return 0
    }
    return Int(submesh.primitiveType.rawValue)
}

@_cdecl("mtk_submesh_index_type")
public func mtk_submesh_index_type(_ submeshPtr: UnsafeMutableRawPointer?) -> Int {
    guard let submesh: MTKSubmesh = mtkBorrow(submeshPtr, as: MTKSubmesh.self) else {
        return 0
    }
    return Int(submesh.indexType.rawValue)
}

@_cdecl("mtk_submesh_index_buffer")
public func mtk_submesh_index_buffer(_ submeshPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let submesh: MTKSubmesh = mtkBorrow(submeshPtr, as: MTKSubmesh.self) else {
        return nil
    }
    return mtkTakeRetained(submesh.indexBuffer)
}

@_cdecl("mtk_submesh_index_count")
public func mtk_submesh_index_count(_ submeshPtr: UnsafeMutableRawPointer?) -> Int {
    guard let submesh: MTKSubmesh = mtkBorrow(submeshPtr, as: MTKSubmesh.self) else {
        return 0
    }
    return submesh.indexCount
}

@_cdecl("mtk_submesh_get_name")
public func mtk_submesh_get_name(_ submeshPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let submesh: MTKSubmesh = mtkBorrow(submeshPtr, as: MTKSubmesh.self) else {
        return nil
    }
    return mtkDup(submesh.name)
}

@_cdecl("mtk_submesh_set_name")
public func mtk_submesh_set_name(_ submeshPtr: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let submesh: MTKSubmesh = mtkBorrow(submeshPtr, as: MTKSubmesh.self),
          let name
    else {
        return
    }
    submesh.name = String(cString: name)
}
