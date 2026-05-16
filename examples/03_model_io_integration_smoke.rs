use apple_metal::MetalDevice;
use metalkit::{
    metal_vertex_descriptor_from_model_io, metal_vertex_format, model_io_vertex_descriptor_from_metal,
    model_vertex_format, MetalVertexDescriptor, ModelAsset, ModelTexture, ModelVertexDescriptor,
    TextureLoader,
};
use std::error::Error;

const SYSTEM_ICON: &str =
    "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/PublicFolderIcon.icns";

fn main() -> Result<(), Box<dyn Error>> {
    let device = MetalDevice::system_default().expect("no Metal device available");
    let allocator = metalkit::MeshBufferAllocator::new(&device).expect("mesh buffer allocator");
    let model_mesh = metalkit::ModelMesh::new_box(
        [1.0, 1.0, 1.0],
        [1, 1, 1],
        false,
        metalkit::GeometryType::Triangles,
        &allocator,
    )?;
    let asset = ModelAsset::with_meshes(Some(&allocator), &[&model_mesh]).expect("asset");
    assert_eq!(asset.count(), 1);

    let texture_loader = TextureLoader::new(&device).expect("texture loader");
    let model_texture = ModelTexture::from_url(SYSTEM_ICON, Some("icon"))?;
    let texture = texture_loader.new_texture_from_model_texture(&model_texture, None)?;
    assert!(texture.width() > 0);

    let metal_descriptor = MetalVertexDescriptor::new().expect("metal descriptor");
    assert!(metal_descriptor.set_attribute(0, metal_vertex_format::FLOAT3, 0, 0));
    assert!(metal_descriptor.set_layout(0, 12));
    let model_descriptor = model_io_vertex_descriptor_from_metal(&metal_descriptor).expect("model descriptor");
    assert_eq!(model_descriptor.info()?.attributes.len(), 1);

    let explicit_model_descriptor = ModelVertexDescriptor::new().expect("empty model descriptor");
    assert!(explicit_model_descriptor.set_attribute(0, "POSITION", model_vertex_format::FLOAT3, 0, 0)?);
    assert!(explicit_model_descriptor.set_layout(0, 12));
    let round_trip = metal_vertex_descriptor_from_model_io(&explicit_model_descriptor).expect("round-trip metal descriptor");
    assert_eq!(round_trip.info()?.attributes.len(), 1);

    println!("✅ metalkit Model I/O integration OK");
    Ok(())
}
