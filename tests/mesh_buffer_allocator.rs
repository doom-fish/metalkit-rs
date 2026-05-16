mod common;

use metalkit::MeshBufferType;

#[test]
fn allocator_creates_vertex_and_index_buffers() {
    let device = common::device();
    let allocator = common::allocator(&device);

    assert!(!allocator.device_ptr().is_null());

    let vertex_buffer = allocator
        .new_buffer(64, MeshBufferType::Vertex)
        .expect("vertex buffer");
    assert_eq!(vertex_buffer.length(), 64);
    assert_eq!(vertex_buffer.buffer_type(), Some(MeshBufferType::Vertex));
    assert!(!vertex_buffer.metal_buffer_ptr().is_null());

    let index_buffer = allocator
        .new_buffer_with_data(&[1, 2, 3, 4, 5, 6], MeshBufferType::Index)
        .expect("index buffer");
    assert_eq!(index_buffer.length(), 6);
    assert_eq!(index_buffer.buffer_type(), Some(MeshBufferType::Index));
}
