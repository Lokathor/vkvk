//! This module re-exports all the stuff from the crate.
//!
//! In case you want to just pull in the whole crate with one import:
//!
//! ```
//! // unleash the power of a glob import
//! use vkvk::prelude::*;
//! ```

pub(crate) use crate::{ext::extension_enumeration_value, version_1_0::fn_types::*};

pub use crate::{
  device::*,
  entry::*,
  instance::*,
  version_1_0::{api_constants::*, base_types::*, constants::*, data_types::*, *},
};
pub use alloc::{boxed::Box, string::String, vec::Vec};
pub use core::{
  ffi::{c_float, c_void},
  num::NonZeroI32,
  ptr::{null, null_mut},
};

#[cfg(feature = "VK_KHR_surface")]
pub use crate::ext::vk_khr_surface::*;

#[cfg(feature = "VK_KHR_swapchain")]
pub use crate::ext::vk_khr_swapchain::*;
