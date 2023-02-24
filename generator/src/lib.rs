#![allow(clippy::drop_ref)]

pub mod base_types;
pub mod bitmasks;
pub mod enumeration_types;
pub mod handle_types;
pub mod structures;
pub mod vk_dot_xml_parser;
pub mod vk_version;
// TODO: API Constants module

/// Strips the vendor tag off of the end of an item name, if there is one.
///
/// ```rust
/// # use vkvk_generator::strip_vendor;
/// let vendors = ["KHR", "IMG"];
/// assert_eq!(strip_vendor("vkCreateSwapchainKHR", &vendors), ("vkCreateSwapchain", Some("KHR")));
/// assert_eq!(strip_vendor("vkCreateInstance", &vendors), ("vkCreateInstance", None));
/// ```
pub fn strip_vendor<'v>(name: &'v str, vendors: &[&str]) -> (&'v str, Option<&'v str>) {
  for vendor in vendors.iter().copied() {
    if let Some(stripped) = name.strip_suffix(vendor) {
      let stripped_len = stripped.len();
      return (&name[..stripped_len], Some(&name[stripped_len..]));
    }
  }
  (name, None)
}

/// Strips the digits off of the end of an item name, if there is one.
///
/// ```rust
/// # use vkvk_generator::strip_number;
/// assert_eq!(strip_number("VkAccessFlagBits2"), ("VkAccessFlagBits", Some("2")));
/// assert_eq!(strip_number("vkCreateInstance"), ("vkCreateInstance", None));
/// ```
pub fn strip_number(name: &str) -> (&str, Option<&str>) {
  if let Some(stripped) = name.strip_suffix(|ch: char| ch.is_ascii_digit()) {
    let stripped_len = stripped.len();
    return (&name[..stripped_len], Some(&name[stripped_len..]));
  }
  (name, None)
}
