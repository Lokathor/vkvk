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

/// Breaks off the vendor tag off of the end of an item name, if there is one.
pub fn break_vendor<'v>(
  registry: &VulkanRegistry, item: &'v str,
) -> (&'v str, Option<&'v str>) {
  for vendor in registry.vendors.iter().copied() {
    if let Some(stripped) = item.strip_suffix(vendor.name) {
      let stripped_len = stripped.len();
      return (&item[..stripped_len], Some(&item[stripped_len..]));
    }
  }
  (item, None)
}

pub fn break_number(item: &str) -> (&str, Option<&str>) {
  if let Some(stripped) = item.strip_suffix(|ch: char| ch.is_ascii_digit()) {
    let stripped_len = stripped.len();
    return (&item[..stripped_len], Some(&item[stripped_len..]));
  }
  (item, None)
}
