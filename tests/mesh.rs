mod common;

use metalkit::{Mesh, ModelAsset};

#[test]
fn mesh_wraps_model_mesh_and_asset_conversion() {
    let device = common::device();
    let allocator = common::allocator(&device);
    let model_mesh = common::model_mesh(&allocator);
    let mesh = Mesh::from_model_mesh(&model_mesh, &device).expect("mesh");

    assert!(mesh.vertex_count() > 0);
    assert!(!mesh.vertex_buffers().is_empty());
    assert!(!mesh.submeshes().is_empty());
    assert!(mesh.vertex_descriptor().is_some());

    mesh.set_name("box-mesh");
    assert_eq!(mesh.name().as_deref(), Some("box-mesh"));

    let asset = ModelAsset::with_meshes(Some(&allocator), &[&model_mesh]).expect("asset");
    let conversion = Mesh::new_meshes_from_asset(&asset, &device).expect("asset conversion");
    assert_eq!(conversion.meshes.len(), 1);
    assert_eq!(conversion.source_meshes.len(), 1);
    assert!(conversion.meshes[0].vertex_count() > 0);
}
