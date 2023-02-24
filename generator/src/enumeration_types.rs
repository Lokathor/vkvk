use core::fmt::Write;

use crate::vk_dot_xml_parser::{TypeEntry, Enumeration, EnumerationEntry};

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

// TODO: if the ty said "bitmask" then we should emit impls for bitwise ops on this type.
pub fn define_enums(Enumeration { name: rust_ty, ty:_, comment: _, bitwidth, entries }: &Enumeration) -> String {
  let mut f = String::new();
  for entry @ EnumerationEntry { name: symbol_name, value, comment, ty: entry_ty, alias, bitpos, api, deprecated } in entries.iter() {
    match api {
      None | Some("vulkan") => (),
      _ => continue,
    }
    if let Some(comment) = comment {
      writeln!(f, "/// {comment}").ok();
    }
    match (value, bitpos) {
      (Some(value), None) => {
        assert!(entry_ty.is_none(), "{entry:?}");
        if let Some(alias) = alias {
          writeln!(f, "/// * Alias For [`{alias}`]").ok();
          if let Some(deprecated) = deprecated {
            writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
          }
          writeln!(f, "pub const {symbol_name}: {rust_ty} = {alias};").ok();
        } else if let Some(abs) = value.strip_prefix('-') {
          if let Some(deprecated) = deprecated {
            writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
          }
          writeln!(f, "pub const {symbol_name}: {rust_ty} = {rust_ty}({abs}_u32.wrapping_neg());").ok();
        } else {
          if let Some(deprecated) = deprecated {
            writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
          }
          writeln!(f, "pub const {symbol_name}: {rust_ty} = {rust_ty}({value});").ok();
        };
      }
      (None, Some(bitpos)) => {
        assert!(entry_ty.is_none(), "{entry:?}");
        assert!(value.is_none(), "{entry:?}");
        let bitwidth = bitwidth.unwrap_or("32");
        if let Some(deprecated) = deprecated {
          writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
        }
        writeln!(f, "pub const {symbol_name}: {rust_ty} = {rust_ty}(1_u{bitwidth} << {bitpos});").ok();
      }
      (None, None) => {
        let alias = alias.unwrap();
        writeln!(f, "/// * Alias For [`{alias}`]").ok();
        if let Some(deprecated) = deprecated {
          writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
        }
        writeln!(f, "pub const {symbol_name}: {rust_ty} = {alias};").ok();
      }
      _ => panic!("{entry:?}"),
    }
  }
  f
}
