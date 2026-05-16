mod common;

#[test]
fn submesh_exposes_index_data_and_parent_mesh() {
    let device = common::device();
    let allocator = common::allocator(&device);
    let model_mesh = common::model_mesh(&allocator);
    let mesh = common::mesh(&device, &model_mesh);
    let submesh = mesh.submeshes().into_iter().next().expect("submesh");

    assert!(submesh.primitive_type().is_some());
    assert!(submesh.index_type().is_some());
    assert!(submesh.index_count() > 0);
    assert!(submesh.index_buffer().is_some());

    let parent_mesh = submesh.mesh().expect("parent mesh");
    assert_eq!(parent_mesh.vertex_count(), mesh.vertex_count());

    submesh.set_name("primary-submesh");
    assert_eq!(submesh.name().as_deref(), Some("primary-submesh"));
}
