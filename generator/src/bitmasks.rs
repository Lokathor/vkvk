use core::fmt::Write;

use crate::{strip_vendor, strip_number, vk_dot_xml_parser::TypeEntry};

/// Makes a bitmask defintion based on the flags name.
/// 
/// Inputs to this function should be something like
/// "FooFlags{optNum}{optVendor}", which is expanded to:
/// * A struct "FooFlagBits{optNum}{optVendor}"
/// * A type alias "FooFlags{optNum}{optVendor}"
/// 
/// **Alternately**, if the bitmask is declared as an alias for another then
/// *both* of the above names are generated as a type alias of whatever the
/// aliased type is.
/// 
/// We need both things, and only one of them can be the struct. We choose to
/// make the "FlagBits" version the struct based on the Khronos docs. Within
/// their docs they list the bit values within the "FlagBits" pages, while the
/// "Flags" pages just say it's a type alias.
/// 
/// The function does NOT define any of the flag bits constants themselves!
/// Those come from elsewhere within the registry.
pub fn define_bitmask(TypeEntry { name, api:_, category, texts:_, comment, requires:_,
  ty_src, is_alias_for, bit_values, obj_type_enum, parent, returned_only, members,
  struct_extends, allow_duplicate, deprecated }: &TypeEntry, vendors: &[&str]) -> String {
  assert_eq!(category.unwrap(), "bitmask");
  if is_alias_for.is_none() {
    assert!(["VkFlags", "VkFlags64"].contains(&ty_src.unwrap()));
  }
  assert!(comment.is_none());
  assert!(struct_extends.is_none());
  assert!(allow_duplicate.is_none());
  assert!(obj_type_enum.is_none());
  assert!(parent.is_none());
  assert!(returned_only.is_none());
  assert!(members.is_empty());
  assert!(deprecated.is_none());
  //
  let (flags_num, opt_vendor) = strip_vendor(name, vendors);
  let (flags, opt_num) = strip_number(flags_num);
  let num = opt_num.unwrap_or("");
  let ext = opt_vendor.unwrap_or("");
  let name = flags.strip_suffix("Flags").expect("flag_bits should end with FlagBits");
  //
  if let Some(suggested_flag_bits_name) = bit_values {
    let actual_flag_bits_name = format!("{name}FlagBits{num}{ext}");
    assert_eq!(*suggested_flag_bits_name, actual_flag_bits_name.as_str());
  }
  //
  let mut f = String::new();
  if let Some(alias) = is_alias_for {
    write!(f, 
      "/// Khronos: [{alias}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{alias}.html)
      pub type {name}FlagBits{num}{ext} = {alias};

      /// Khronos: [{alias}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{alias}.html)
      pub type {name}Flags{num}{ext} = {alias};"
    ).ok();
  } else {
    write!(f, 
      "/// Khronos: [{name}FlagBits{num}{ext}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}FlagBits{num}{ext}.html)
      #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
      #[repr(transparent)] pub struct {name}FlagBits{num}{ext}(pub u32);
      impl {name}FlagBits{num}{ext} {{ #[inline] #[must_use] pub const fn none() -> Self {{ Self(0) }} }}
      // {name}FlagBits{num}{ext});

      /// Khronos: [{name}FlagBits{num}{ext}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}FlagBits{num}{ext}.html)
      pub type {name}Flags{num}{ext} = {name}FlagBits{num}{ext};"
    ).ok();
  }
  f
}
