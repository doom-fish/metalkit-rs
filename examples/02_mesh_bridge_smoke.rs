use apple_metal::MetalDevice;
use metalkit::{GeometryType, Mesh, MeshBufferAllocator, ModelMesh};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let device = MetalDevice::system_default().expect("no Metal device available");
    let allocator =
        MeshBufferAllocator::new(&device).expect("failed to create mesh buffer allocator");
    let model_mesh = ModelMesh::new_box(
        [1.0, 1.0, 1.0],
        [1, 1, 1],
        false,
        GeometryType::Triangles,
        &allocator,
    )?;
    let mesh = Mesh::from_model_mesh(&model_mesh, &device)?;

    assert!(mesh.vertex_count() > 0);
    let vertex_buffers = mesh.vertex_buffers();
    assert!(!vertex_buffers.is_empty());
    let submeshes = mesh.submeshes();
    assert!(!submeshes.is_empty());
    assert!(submeshes[0].index_count() > 0);

    let mut preview = vec![0_u8; vertex_buffers[0].length().min(64)];
    assert!(vertex_buffers[0].copy_bytes(&mut preview) > 0);

    println!("✅ metalkit mesh bridge OK");
    Ok(())
}
