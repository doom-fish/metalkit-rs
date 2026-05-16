mod common;

use metalkit::MeshBufferType;

#[test]
fn mesh_buffer_round_trips_bytes_and_name() {
    let device = common::device();
    let allocator = common::allocator(&device);
    let source = [10_u8, 20, 30, 40, 50, 60, 70, 80];
    let buffer = allocator
        .new_buffer_with_data(&source, MeshBufferType::Vertex)
        .expect("mesh buffer");

    assert_eq!(buffer.offset(), 0);
    assert_eq!(buffer.length(), source.len());
    assert_eq!(buffer.buffer_type(), Some(MeshBufferType::Vertex));
    assert!(buffer.allocator().is_some());
    assert!(!buffer.metal_buffer_ptr().is_null());

    buffer.set_name("vertex-bytes");
    assert_eq!(buffer.name().as_deref(), Some("vertex-bytes"));

    let mut bytes = [0_u8; 8];
    let copied = buffer.copy_bytes(&mut bytes);
    assert_eq!(copied, source.len());
    assert_eq!(bytes, source);
}
