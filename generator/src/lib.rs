#![allow(clippy::from_str_radix_10)]

use core::fmt::Display;
use std::{
  collections::{btree_map::Entry, BTreeMap},
  fs::OpenOptions,
  io::Write,
  path::Path,
};
use vk_dot_xml_parser::*;

mod enumerations;
pub use enumerations::*;

mod bitmasks;
pub use bitmasks::*;

/// Given an `api` attribute, is this item "vulkan" compatible?
///
/// The logic is that if "vulkan" is in the comma separated list then it's
/// vulkan compatible, OR if the value is `None` then it's compatible with all
/// APIs (including "vulkan").
fn api_vulkan(api: Option<StaticStr>) -> bool {
  api.map(|s| s.split(',').any(|s| s == "vulkan")).unwrap_or(true)
}
