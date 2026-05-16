#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API Documentation

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::borrow_as_ptr)]
#![allow(clippy::ref_as_ptr)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::tuple_array_conversions)]
#![allow(clippy::use_self)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

mod error;
pub mod ffi;
mod mesh;
mod mesh_buffer;
mod mesh_buffer_allocator;
mod model_io_integration;
mod private;
mod submesh;
mod texture_loader;
mod view;

pub use error::MetalKitError;
pub use mesh::{Mesh, MeshAssetConversion};
pub use mesh_buffer::{MeshBuffer, MeshBufferType, MeshBufferZone};
pub use mesh_buffer_allocator::MeshBufferAllocator;
pub use model_io_integration::{
    metal_vertex_descriptor_from_model_io, metal_vertex_format, metal_vertex_format_from_model_io,
    model_error, model_io_vertex_descriptor_from_metal, model_io_vertex_format_from_metal,
    model_vertex_format, try_metal_vertex_descriptor_from_model_io,
    try_model_io_vertex_descriptor_from_metal, GeometryType, MetalVertexDescriptor, ModelAsset,
    ModelMesh, ModelTexture, ModelVertexDescriptor, VertexDescriptorAttributeInfo,
    VertexDescriptorInfo, VertexDescriptorLayoutInfo,
};
pub use submesh::{IndexType, PrimitiveType, Submesh};
pub use texture_loader::{
    texture_cpu_cache_mode, texture_loader_cube_layout, texture_loader_error,
    texture_loader_option, texture_loader_origin, DisplayGamut, TextureLoader,
    TextureLoaderArrayOutcome, TextureLoaderCubeLayout, TextureLoaderOptionKey,
    TextureLoaderOptions, TextureLoaderOrigin,
};
pub use view::{ClearColor, Rect, Size, View, ViewDelegate, ViewDelegateCallbacks};

pub mod prelude {
    pub use crate::{
        metal_vertex_descriptor_from_model_io, model_io_vertex_descriptor_from_metal, ClearColor,
        DisplayGamut, GeometryType, Mesh, MeshAssetConversion, MeshBuffer,
        MeshBufferAllocator, MeshBufferType, ModelAsset, ModelMesh, ModelTexture,
        ModelVertexDescriptor, PrimitiveType, Rect, Size, Submesh, TextureLoader,
        TextureLoaderOptions, View, ViewDelegate, ViewDelegateCallbacks,
    };
}
