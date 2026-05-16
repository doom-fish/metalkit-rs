import Foundation
import Metal
import MetalKit

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

@_cdecl("mtk_submesh_mesh")
public func mtk_submesh_mesh(_ submeshPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let submesh: MTKSubmesh = mtkBorrow(submeshPtr, as: MTKSubmesh.self),
          let mesh = submesh.mesh
    else {
        return nil
    }
    return mtkTakeRetained(mesh)
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
