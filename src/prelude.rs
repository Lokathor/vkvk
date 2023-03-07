//! This module re-exports all the stuff from the crate.
//!
//! In case you want to just pull in the whole crate with one import:
//!
//! ```
//! // unleash the power of a glob import
//! use vkvk::prelude::*;
//! ```

pub use alloc::{boxed::Box, string::String, vec::Vec};
pub use core::{
  ffi::{c_float, c_void},
  num::{NonZeroI32, NonZeroU32},
  ptr::{null, null_mut},
};

pub use raw_vulkan_handle::*;

pub use crate::{api_constants::*, base_types::*, generated::enumerations::*};
