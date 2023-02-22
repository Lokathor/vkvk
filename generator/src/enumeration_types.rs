use core::fmt::Write;

use crate::vk_dot_xml_parser::TypeEntry;

pub fn define_enumeration(TypeEntry { name, api:_, category, texts:_, comment, requires:_,
  ty_src, is_alias_for, bit_values, obj_type_enum, parent, returned_only, members,
  struct_extends, allow_duplicate, deprecated }: &TypeEntry
) -> String {
  assert_eq!(category.unwrap(), "enum");
  assert!(comment.is_none());
  assert!(ty_src.is_none());
  assert!(struct_extends.is_none());
  assert!(allow_duplicate.is_none());
  assert!(obj_type_enum.is_none());
  assert!(parent.is_none());
  assert!(returned_only.is_none());
  assert!(members.is_empty());
  assert!(deprecated.is_none());
  assert!(bit_values.is_none());
  //
  let mut f = String::new();
  if let Some(alias) = is_alias_for {
    write!(f, 
      "/// Khronos: [{alias}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{alias}.html)
      pub type {name} = {alias};"
    ).ok();
  } else {
    write!(f, 
      "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)
      #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
      #[repr(transparent)] pub struct {name}(pub u32);"
    ).ok();
  }
  f
}
