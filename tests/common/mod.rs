#![allow(dead_code)]

use apple_metal::MetalDevice;
use metalkit::{GeometryType, Mesh, MeshBufferAllocator, ModelMesh};

pub const SYSTEM_ICON: &str =
    "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/PublicFolderIcon.icns";

pub fn device() -> MetalDevice {
    MetalDevice::system_default().expect("no Metal device available")
}

pub fn allocator(device: &MetalDevice) -> MeshBufferAllocator {
    MeshBufferAllocator::new(device).expect("failed to create mesh buffer allocator")
}

pub fn model_mesh(allocator: &MeshBufferAllocator) -> ModelMesh {
    ModelMesh::new_box(
        [1.0, 1.0, 1.0],
        [1, 1, 1],
        false,
        GeometryType::Triangles,
        allocator,
    )
    .expect("failed to create model mesh")
}

pub fn mesh(device: &MetalDevice, model_mesh: &ModelMesh) -> Mesh {
    Mesh::from_model_mesh(model_mesh, device).expect("failed to create metal mesh")
}
