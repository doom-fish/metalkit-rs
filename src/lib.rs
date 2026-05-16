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
mod model;
mod private;
mod texture;

pub use error::MetalKitError;
pub use model::{
    GeometryType, IndexType, Mesh, MeshBuffer, MeshBufferAllocator, MeshBufferType, ModelMesh,
    PrimitiveType, Submesh,
};
pub use texture::{
    texture_cpu_cache_mode, texture_loader_option, TextureLoader, TextureLoaderOptionKey,
    TextureLoaderOptions,
};

pub mod prelude {
    pub use crate::{
        GeometryType, Mesh, MeshBuffer, MeshBufferAllocator, MeshBufferType, ModelMesh, Submesh,
        TextureLoader, TextureLoaderOptions,
    };
}
