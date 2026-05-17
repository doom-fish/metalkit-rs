mod common;

use metalkit::{
    metal_vertex_descriptor_from_model_io, metal_vertex_format, metal_vertex_format_from_model_io,
    model_error, model_io_vertex_descriptor_from_metal, model_io_vertex_format_from_metal,
    model_vertex_format, MetalVertexDescriptor, ModelAsset, ModelError, ModelTexture,
    ModelVertexDescriptor,
};

#[test]
fn model_asset_texture_and_vertex_descriptor_helpers_work() {
    let device = common::device();
    let allocator = common::allocator(&device);
    let model_mesh = common::model_mesh(&allocator);

    assert!(ModelAsset::can_import_file_extension("obj"));

    let asset = ModelAsset::with_meshes(Some(&allocator), &[&model_mesh]).expect("asset");
    assert_eq!(asset.count(), 1);
    assert_eq!(
        asset.mesh_at(0).expect("mesh").vertex_count(),
        model_mesh.vertex_count()
    );

    let texture = ModelTexture::from_url(common::SYSTEM_ICON, Some("icon")).expect("model texture");
    let loader = metalkit::TextureLoader::new(&device).expect("texture loader");
    let metal_texture = loader
        .new_texture_from_model_texture(&texture, None)
        .expect("metal texture from mdl texture");
    assert!(metal_texture.width() > 0);
    assert!(metal_texture.height() > 0);

    let metal_descriptor = MetalVertexDescriptor::new().expect("metal descriptor");
    assert!(metal_descriptor.set_attribute(0, metal_vertex_format::FLOAT3, 0, 0));
    assert!(metal_descriptor.set_layout(0, 12));
    let metal_info = metal_descriptor.info().expect("metal descriptor info");
    assert_eq!(metal_info.attributes.len(), 1);
    assert_eq!(metal_info.layouts.len(), 1);

    let model_descriptor = model_io_vertex_descriptor_from_metal(&metal_descriptor)
        .expect("model descriptor from metal descriptor");
    let model_info = model_descriptor.info().expect("model descriptor info");
    assert_eq!(model_info.attributes.len(), 1);
    assert_eq!(model_info.layouts.len(), 1);

    let explicit_model_descriptor = ModelVertexDescriptor::new().expect("empty model descriptor");
    assert!(explicit_model_descriptor
        .set_attribute(0, "POSITION", model_vertex_format::FLOAT3, 0, 0)
        .expect("set model attribute"));
    assert!(explicit_model_descriptor.set_layout(0, 12));
    let round_trip_metal = metal_vertex_descriptor_from_model_io(&explicit_model_descriptor)
        .expect("metal descriptor from model descriptor");
    let round_trip_info = round_trip_metal
        .info()
        .expect("round-trip metal descriptor info");
    assert_eq!(round_trip_info.attributes.len(), 1);
    assert_eq!(round_trip_info.layouts.len(), 1);

    assert_eq!(
        model_io_vertex_format_from_metal(metal_vertex_format::FLOAT3),
        model_vertex_format::FLOAT3
    );
    assert_eq!(
        metal_vertex_format_from_model_io(model_vertex_format::FLOAT3),
        metal_vertex_format::FLOAT3
    );
    assert_eq!(ModelError::DOMAIN.as_str(), model_error::DOMAIN);
    assert_eq!(ModelError::KEY.as_str(), model_error::KEY);
}
