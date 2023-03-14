#![warn(missing_docs)]
#![cfg_attr(docs_rs, feature(doc_cfg))]

//! A library for interacting with the Vulkan graphical and compute API.
//!
//! ## Naming
//!
//! By default, constants and type names will be the same as the official Vulkan
//! names for the types and constants.
//!
//! Some of the types in this library wrap a "raw" vulkan value with "extra
//! stuff". The details of the stuff vary, but the naming scheme is always the
//! same: When a raw Vulkan type is wrapped the "rusty" type will be the same
//! name with the leading `Vk` stripped. So an `Instance` wraps a
//! [`VkInstance`][rvh-i] with extra stuff, a `PhysicalDevice` wraps a
//! [`VkPhysicalDevice`][rvh-pd] with extra stuff, and so on.
//!
//! [rvh-i]: raw_vulkan_handle::VkInstance
//! [rvh-pd]: raw_vulkan_handle::VkPhysicalDevice
//!
//! ## Usage
//!
//! Generally you are expected to import everything at once via the crate's
//! prelude:
//!
//! ```
//! use vkvk::prelude::*;
//! ```

#[macro_use]
mod macros;

pub mod api_constants;
pub mod base_types;
pub mod entry;
pub mod instance;
pub mod prelude;

#[allow(missing_docs)]
pub mod generated;
