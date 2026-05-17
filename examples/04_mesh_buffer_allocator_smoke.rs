use apple_metal::MetalDevice;
use metalkit::{MeshBufferAllocator, MeshBufferType};

fn main() {
    let device = MetalDevice::system_default().expect("no Metal device available");
    let allocator = MeshBufferAllocator::new(&device).expect("mesh buffer allocator");

    let empty = allocator
        .new_buffer(32, MeshBufferType::Vertex)
        .expect("vertex buffer");
    assert_eq!(empty.length(), 32);
    assert!(!empty.metal_buffer_ptr().is_null());

    let filled = allocator
        .new_buffer_with_data(&[1, 2, 3, 4, 5, 6, 7, 8], MeshBufferType::Index)
        .expect("index buffer");
    assert_eq!(filled.buffer_type(), Some(MeshBufferType::Index));

    let mut bytes = [0_u8; 8];
    assert_eq!(filled.copy_bytes(&mut bytes), 8);
    assert_eq!(bytes, [1, 2, 3, 4, 5, 6, 7, 8]);

    println!("✅ metalkit mesh buffer allocator OK");
}
