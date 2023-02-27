//! This module re-exports all the stuff from the crate.
//!
//! In case you want to just pull in the whole crate with one import:
//!
//! ```
//! // unleash the power of a glob import
//! use vkvk::prelude::*;
//! ```

pub use crate::{
  api_constants::*,
  base_types::*,
  version_1_0::{constants::*, data_types::*, *},
  vk_version::*,
};

#[cfg(feature = "VK_KHR_surface")]
pub use crate::ext::vk_khr_surface::*;

#[cfg(feature = "VK_KHR_swapchain")]
pub use crate::ext::vk_khr_swapchain::*;
