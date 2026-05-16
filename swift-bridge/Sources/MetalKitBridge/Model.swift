import Foundation
import Metal
import MetalKit
import ModelIO
import simd

private func mtkGeometryType(from rawValue: Int64) -> MDLGeometryType {
    MDLGeometryType(rawValue: Int(rawValue)) ?? .triangles
}

private func mtkMetalVertexFormat(from rawValue: UInt) -> MTLVertexFormat {
    MTLVertexFormat(rawValue: rawValue) ?? .invalid
}

private func mtkModelVertexFormat(from rawValue: UInt) -> MDLVertexFormat {
    MDLVertexFormat(rawValue: rawValue) ?? .invalid
}

private struct VertexDescriptorAttributeInfo: Encodable {
    let index: Int
    let format: UInt
    let offset: Int
    let bufferIndex: Int
    let name: String?
}

private struct VertexDescriptorLayoutInfo: Encodable {
    let index: Int
    let stride: Int
}

private struct VertexDescriptorInfo: Encodable {
    let attributes: [VertexDescriptorAttributeInfo]
    let layouts: [VertexDescriptorLayoutInfo]
}

final class MTKMeshesFromAssetResultBox: NSObject {
    let meshes: [MTKMesh]
    let sourceMeshes: [MDLMesh]

    init(meshes: [MTKMesh], sourceMeshes: [MDLMesh]) {
        self.meshes = meshes
        self.sourceMeshes = sourceMeshes
    }
}

private func mtkModelVertexDescriptorInfo(_ descriptor: MDLVertexDescriptor) -> VertexDescriptorInfo {
    var attributes: [VertexDescriptorAttributeInfo] = []
    for index in 0..<descriptor.attributes.count {
        guard let attribute = descriptor.attributes[index] as? MDLVertexAttribute,
              attribute.format != .invalid
        else {
            continue
        }
        attributes.append(.init(
            index: index,
            format: UInt(attribute.format.rawValue),
            offset: attribute.offset,
            bufferIndex: attribute.bufferIndex,
            name: attribute.name
        ))
    }

    var layouts: [VertexDescriptorLayoutInfo] = []
    for index in 0..<descriptor.layouts.count {
        guard let layout = descriptor.layouts[index] as? MDLVertexBufferLayout,
              layout.stride != 0
        else {
            continue
        }
        layouts.append(.init(index: index, stride: layout.stride))
    }

    return .init(attributes: attributes, layouts: layouts)
}

private func mtkMetalVertexDescriptorInfo(_ descriptor: MTLVertexDescriptor) -> VertexDescriptorInfo {
    var attributes: [VertexDescriptorAttributeInfo] = []
    for index in 0..<31 {
        guard let attribute = descriptor.attributes[index], attribute.format != .invalid else {
            continue
        }
        attributes.append(.init(
            index: index,
            format: attribute.format.rawValue,
            offset: attribute.offset,
            bufferIndex: attribute.bufferIndex,
            name: nil
        ))
    }

    var layouts: [VertexDescriptorLayoutInfo] = []
    for index in 0..<31 {
        guard let layout = descriptor.layouts[index], layout.stride != 0 else {
            continue
        }
        layouts.append(.init(index: index, stride: layout.stride))
    }

    return .init(attributes: attributes, layouts: layouts)
}

@_cdecl("mtk_model_asset_new")
public func mtk_model_asset_new(_ allocatorPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let allocator: MTKMeshBufferAllocator? = mtkBorrow(allocatorPtr, as: MTKMeshBufferAllocator.self)
    return mtkTakeRetained(MDLAsset(bufferAllocator: allocator))
}

@_cdecl("mtk_model_asset_new_with_url")
public func mtk_model_asset_new_with_url(
    _ path: UnsafePointer<CChar>?,
    _ vertexDescriptorPtr: UnsafeMutableRawPointer?,
    _ allocatorPtr: UnsafeMutableRawPointer?,
    _ preserveTopology: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let path else {
        outError?.pointee = mtkDup("invalid asset path")
        return nil
    }

    let url = URL(fileURLWithPath: String(cString: path))
    let vertexDescriptor: MDLVertexDescriptor? = mtkBorrow(vertexDescriptorPtr, as: MDLVertexDescriptor.self)
    let allocator: MTKMeshBufferAllocator? = mtkBorrow(allocatorPtr, as: MTKMeshBufferAllocator.self)
    var error: NSError?
    let asset = MDLAsset(
        url: url,
        vertexDescriptor: vertexDescriptor,
        bufferAllocator: allocator,
        preserveTopology: preserveTopology,
        error: &error
    )
    if let error {
        outError?.pointee = mtkNSErrorMessage(error)
        return nil
    }
    outError?.pointee = nil
    return mtkTakeRetained(asset)
}

@_cdecl("mtk_model_asset_can_import_file_extension")
public func mtk_model_asset_can_import_file_extension(_ pathExtension: UnsafePointer<CChar>?) -> Bool {
    guard let pathExtension else { return false }
    return MDLAsset.canImportFileExtension(String(cString: pathExtension))
}

@_cdecl("mtk_model_asset_count")
public func mtk_model_asset_count(_ assetPtr: UnsafeMutableRawPointer?) -> Int {
    guard let asset: MDLAsset = mtkBorrow(assetPtr, as: MDLAsset.self) else {
        return 0
    }
    return asset.count
}

@_cdecl("mtk_model_asset_add_mesh")
public func mtk_model_asset_add_mesh(
    _ assetPtr: UnsafeMutableRawPointer?,
    _ meshPtr: UnsafeMutableRawPointer?
) -> Bool {
    guard let asset: MDLAsset = mtkBorrow(assetPtr, as: MDLAsset.self),
          let mesh: MDLMesh = mtkBorrow(meshPtr, as: MDLMesh.self)
    else {
        return false
    }
    asset.add(mesh)
    return true
}

@_cdecl("mtk_model_asset_mesh_at")
public func mtk_model_asset_mesh_at(
    _ assetPtr: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let asset: MDLAsset = mtkBorrow(assetPtr, as: MDLAsset.self),
          index >= 0,
          index < asset.count,
          let mesh = asset.object(at: index) as? MDLMesh
    else {
        return nil
    }
    return mtkTakeRetained(mesh)
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
        geometryType: mtkGeometryType(from: geometryType),
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

@_cdecl("mtk_model_texture_new_from_url")
public func mtk_model_texture_new_from_url(
    _ path: UnsafePointer<CChar>?,
    _ name: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let path else {
        outError?.pointee = mtkDup("invalid texture path")
        return nil
    }
    let url = URL(fileURLWithPath: String(cString: path))
    if !FileManager.default.fileExists(atPath: url.path) {
        outError?.pointee = mtkDup("texture path does not exist")
        return nil
    }
    let texture = MDLURLTexture(url: url, name: name.map { String(cString: $0) })
    outError?.pointee = nil
    return mtkTakeRetained(texture)
}

@_cdecl("mtk_metal_vertex_descriptor_new")
public func mtk_metal_vertex_descriptor_new() -> UnsafeMutableRawPointer? {
    mtkTakeRetained(MTLVertexDescriptor())
}

@_cdecl("mtk_metal_vertex_descriptor_set_attribute")
public func mtk_metal_vertex_descriptor_set_attribute(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ format: UInt,
    _ offset: Int,
    _ bufferIndex: Int
) -> Bool {
    guard let descriptor: MTLVertexDescriptor = mtkBorrow(descriptorPtr, as: MTLVertexDescriptor.self),
          (0..<31).contains(index),
          (0..<31).contains(bufferIndex)
    else {
        return false
    }
    guard let attribute = descriptor.attributes[index] else {
        return false
    }
    attribute.format = mtkMetalVertexFormat(from: format)
    attribute.offset = offset
    attribute.bufferIndex = bufferIndex
    return true
}

@_cdecl("mtk_metal_vertex_descriptor_set_layout")
public func mtk_metal_vertex_descriptor_set_layout(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ stride: Int
) -> Bool {
    guard let descriptor: MTLVertexDescriptor = mtkBorrow(descriptorPtr, as: MTLVertexDescriptor.self),
          (0..<31).contains(index)
    else {
        return false
    }
    descriptor.layouts[index].stride = stride
    return true
}

@_cdecl("mtk_metal_vertex_descriptor_info_json")
public func mtk_metal_vertex_descriptor_info_json(
    _ descriptorPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let descriptor: MTLVertexDescriptor = mtkBorrow(descriptorPtr, as: MTLVertexDescriptor.self) else {
        return nil
    }
    return mtkJSON(mtkMetalVertexDescriptorInfo(descriptor))
}

@_cdecl("mtk_model_vertex_descriptor_new")
public func mtk_model_vertex_descriptor_new() -> UnsafeMutableRawPointer? {
    mtkTakeRetained(MDLVertexDescriptor())
}

@_cdecl("mtk_model_vertex_descriptor_set_attribute")
public func mtk_model_vertex_descriptor_set_attribute(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ name: UnsafePointer<CChar>?,
    _ format: UInt,
    _ offset: Int,
    _ bufferIndex: Int
) -> Bool {
    guard let descriptor: MDLVertexDescriptor = mtkBorrow(descriptorPtr, as: MDLVertexDescriptor.self),
          index >= 0,
          bufferIndex >= 0,
          let name
    else {
        return false
    }
    while descriptor.attributes.count <= index {
        descriptor.attributes.add(NSNull())
    }
    descriptor.attributes[index] = MDLVertexAttribute(
        name: String(cString: name),
        format: mtkModelVertexFormat(from: format),
        offset: offset,
        bufferIndex: bufferIndex
    )
    return true
}

@_cdecl("mtk_model_vertex_descriptor_set_layout")
public func mtk_model_vertex_descriptor_set_layout(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ stride: Int
) -> Bool {
    guard let descriptor: MDLVertexDescriptor = mtkBorrow(descriptorPtr, as: MDLVertexDescriptor.self),
          index >= 0
    else {
        return false
    }
    while descriptor.layouts.count <= index {
        descriptor.layouts.add(NSNull())
    }
    descriptor.layouts[index] = MDLVertexBufferLayout(stride: stride)
    return true
}

@_cdecl("mtk_model_vertex_descriptor_info_json")
public func mtk_model_vertex_descriptor_info_json(
    _ descriptorPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let descriptor: MDLVertexDescriptor = mtkBorrow(descriptorPtr, as: MDLVertexDescriptor.self) else {
        return nil
    }
    return mtkJSON(mtkModelVertexDescriptorInfo(descriptor))
}

@_cdecl("mtk_model_io_vertex_descriptor_from_metal")
public func mtk_model_io_vertex_descriptor_from_metal(
    _ descriptorPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let descriptor: MTLVertexDescriptor = mtkBorrow(descriptorPtr, as: MTLVertexDescriptor.self) else {
        return nil
    }
    return mtkTakeRetained(MTKModelIOVertexDescriptorFromMetal(descriptor))
}

@_cdecl("mtk_model_io_vertex_descriptor_from_metal_with_error")
public func mtk_model_io_vertex_descriptor_from_metal_with_error(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let descriptor: MTLVertexDescriptor = mtkBorrow(descriptorPtr, as: MTLVertexDescriptor.self) else {
        outError?.pointee = mtkDup("invalid MTLVertexDescriptor")
        return nil
    }
    let result = MTKModelIOVertexDescriptorFromMetal(descriptor)
    outError?.pointee = nil
    return mtkTakeRetained(result)
}

@_cdecl("mtk_metal_vertex_descriptor_from_model_io")
public func mtk_metal_vertex_descriptor_from_model_io(
    _ descriptorPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let descriptor: MDLVertexDescriptor = mtkBorrow(descriptorPtr, as: MDLVertexDescriptor.self) else {
        return nil
    }
    guard let result = MTKMetalVertexDescriptorFromModelIO(descriptor) else {
        return nil
    }
    return mtkTakeRetained(result)
}

@_cdecl("mtk_metal_vertex_descriptor_from_model_io_with_error")
public func mtk_metal_vertex_descriptor_from_model_io_with_error(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let descriptor: MDLVertexDescriptor = mtkBorrow(descriptorPtr, as: MDLVertexDescriptor.self) else {
        outError?.pointee = mtkDup("invalid MDLVertexDescriptor")
        return nil
    }
    guard let result = MTKMetalVertexDescriptorFromModelIO(descriptor) else {
        outError?.pointee = mtkDup("failed to convert MDLVertexDescriptor to MTLVertexDescriptor")
        return nil
    }
    outError?.pointee = nil
    return mtkTakeRetained(result)
}

@_cdecl("mtk_model_io_vertex_format_from_metal")
public func mtk_model_io_vertex_format_from_metal(_ vertexFormat: UInt) -> UInt {
    UInt(MTKModelIOVertexFormatFromMetal(mtkMetalVertexFormat(from: vertexFormat)).rawValue)
}

@_cdecl("mtk_metal_vertex_format_from_model_io")
public func mtk_metal_vertex_format_from_model_io(_ vertexFormat: UInt) -> UInt {
    MTKMetalVertexFormatFromModelIO(mtkModelVertexFormat(from: vertexFormat)).rawValue
}
