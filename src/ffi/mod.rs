//! assimp`s C API

pub use crate::ffi::cexport::*;
pub use crate::ffi::cimport::*;
pub use crate::ffi::info::*;
pub use crate::ffi::log::DefaultLogStream::*;
pub use crate::ffi::log::*;
pub use crate::ffi::material::*;
pub use crate::ffi::types::*;

//TODO remove the stuff we don't need once we leave the experimental stage
#[allow(dead_code)]
mod cexport;
#[allow(dead_code)]
mod cimport;
#[allow(dead_code)]
mod info;
#[allow(dead_code)]
mod log;
#[allow(dead_code)]
mod material;
#[allow(dead_code)]
mod types;
