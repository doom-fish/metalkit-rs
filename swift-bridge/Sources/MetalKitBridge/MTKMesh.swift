import Foundation
import Metal
import MetalKit
import ModelIO

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

@_cdecl("mtk_mesh_new_meshes_from_asset")
public func mtk_mesh_new_meshes_from_asset(
    _ assetPtr: UnsafeMutableRawPointer?,
    _ devicePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let asset: MDLAsset = mtkBorrow(assetPtr, as: MDLAsset.self),
          let device: MTLDevice = mtkBorrow(devicePtr, as: MTLDevice.self)
    else {
        outError?.pointee = mtkDup("invalid MDLAsset or MTLDevice")
        return nil
    }

    do {
        let result = try MTKMesh.newMeshes(asset: asset, device: device)
        outError?.pointee = nil
        return mtkTakeRetained(MTKMeshesFromAssetResultBox(
            meshes: result.metalKitMeshes,
            sourceMeshes: result.modelIOMeshes
        ))
    } catch let error as NSError {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
}

@_cdecl("mtk_meshes_from_asset_result_mesh_count")
public func mtk_meshes_from_asset_result_mesh_count(_ resultPtr: UnsafeMutableRawPointer?) -> Int {
    guard let result: MTKMeshesFromAssetResultBox = mtkBorrow(resultPtr, as: MTKMeshesFromAssetResultBox.self) else {
        return 0
    }
    return result.meshes.count
}

@_cdecl("mtk_meshes_from_asset_result_mesh_at")
public func mtk_meshes_from_asset_result_mesh_at(
    _ resultPtr: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let result: MTKMeshesFromAssetResultBox = mtkBorrow(resultPtr, as: MTKMeshesFromAssetResultBox.self),
          index >= 0,
          index < result.meshes.count
    else {
        return nil
    }
    return mtkTakeRetained(result.meshes[index])
}

@_cdecl("mtk_meshes_from_asset_result_source_mesh_count")
public func mtk_meshes_from_asset_result_source_mesh_count(_ resultPtr: UnsafeMutableRawPointer?) -> Int {
    guard let result: MTKMeshesFromAssetResultBox = mtkBorrow(resultPtr, as: MTKMeshesFromAssetResultBox.self) else {
        return 0
    }
    return result.sourceMeshes.count
}

@_cdecl("mtk_meshes_from_asset_result_source_mesh_at")
public func mtk_meshes_from_asset_result_source_mesh_at(
    _ resultPtr: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let result: MTKMeshesFromAssetResultBox = mtkBorrow(resultPtr, as: MTKMeshesFromAssetResultBox.self),
          index >= 0,
          index < result.sourceMeshes.count
    else {
        return nil
    }
    return mtkTakeRetained(result.sourceMeshes[index])
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

@_cdecl("mtk_mesh_vertex_descriptor")
public func mtk_mesh_vertex_descriptor(_ meshPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let mesh: MTKMesh = mtkBorrow(meshPtr, as: MTKMesh.self) else {
        return nil
    }
    return mtkTakeRetained(mesh.vertexDescriptor)
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
