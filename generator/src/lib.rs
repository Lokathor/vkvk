#![allow(clippy::drop_ref)]

use vk_dot_xml_parser::TypeVariant;

pub mod bitmasks;
pub mod enumeration_types;
pub mod handle_types;
pub mod structs_unions;
pub mod vk_dot_xml_parser;

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

pub fn var_name(name: &str) -> String {
  use convert_case::{Case, Casing};
  match name {
    "type" => String::from("ty"),
    "sType" => String::from("ty"),
    _ => {
      let mut s = name.to_case(Case::Snake);
      s = s.replace("2_d", "2d");
      s = s.replace("3_d", "3d");
      if let Some(shorter) = s.strip_prefix("p_") {
        s = String::from(shorter);
      }
      if let Some(shorter) = s.strip_prefix("pp_") {
        s = String::from(shorter);
      }
      if let Some(shorter) = s.strip_prefix("pfn_") {
        s = String::from(shorter);
      }
      s
    }
  }
}

pub fn format_type_and_variant(ty: &str, ty_variant: TypeVariant) -> String {
  #[allow(clippy::useless_format)]
  match ty_variant {
    TypeVariant::Normal => format!("{ty}"),
    TypeVariant::ConstPtr => format!("*const {ty}"),
    TypeVariant::MutPtr => format!("*mut {ty}"),
    TypeVariant::MutPtrMutPtr => format!("*mut *mut {ty}"),
    TypeVariant::ConstArrayPtrLit(n) => format!("*const [{ty}; {n}]"),
    TypeVariant::MutPtrConstPtr => format!("*mut *const {ty}"),
    TypeVariant::ConstPtrConstPtr => format!("*const *const {ty}"),
    TypeVariant::ArrayLit(n) => format!("[{ty}; {n}]"),
    TypeVariant::ArrayNamed(n) if ty == "u8" && !n.contains("UUID") => {
      format!("ArrayZString<{n}>")
    }
    TypeVariant::ArrayNamed(n) => format!("[{ty}; {n}]"),
    TypeVariant::ArrayOfArrayLit(a, b) => format!("[[{ty}; {a}]; {b}]"),
    TypeVariant::BitfieldsLit(n) => format!("{ty}{{:{n}}}"),
  }
}

pub const FUNC_PTR_DECLS: &str = r#"
#[allow(nonstandard_style)]
pub type vkVoidFunction_t = unsafe extern "system" fn();
#[allow(nonstandard_style)]
pub type vkAllocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: usize,
  alignment: usize,
  allocation_scope: VkSystemAllocationScope,
);
#[allow(nonstandard_style)]
pub type vkReallocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  original: *mut c_void,
  size: usize,
  alignment: usize,
  allocation_scope: VkSystemAllocationScope,
);
#[allow(nonstandard_style)]
pub type vkFreeFunction_t =
  unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
#[allow(nonstandard_style)]
pub type vkInternalAllocationNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: usize,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);
#[allow(nonstandard_style)]
pub type vkInternalFreeNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: usize,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);

#[allow(nonstandard_style)]
pub type PFN_vkVoidFunction = Option<vkVoidFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkAllocationFunction = Option<vkAllocationFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkReallocationFunction = Option<vkReallocationFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkFreeFunction = Option<vkFreeFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkInternalAllocationNotification = Option<vkInternalAllocationNotification_t>;
#[allow(nonstandard_style)]
pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification_t>;
"#;
