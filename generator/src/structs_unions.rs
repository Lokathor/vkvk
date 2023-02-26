use crate::vk_dot_xml_parser::{Member, TypeEntry, TypeVariant};
use core::fmt::Write;

use convert_case::{Case, Casing};

fn field_name(name: &str) -> String {
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
      if let Some(shorter) = s.strip_prefix("pfn_") {
        s = String::from(shorter);
      }
      s
    }
  }
}

pub fn define_structure(
  TypeEntry {
    name,
    comment,
    is_alias_for,
    returned_only,
    members,
    struct_extends,
    allow_duplicate,
    category,
    ty_src,
    bit_values,
    obj_type_enum,
    parent,
    deprecated,
    api: _,
    texts: _,
    requires: _,
  }: &TypeEntry,
) -> String {
  assert_eq!(category.unwrap(), "struct");
  assert!(ty_src.is_none());
  assert!(obj_type_enum.is_none());
  assert!(parent.is_none());
  assert!(deprecated.is_none());
  assert!(bit_values.is_none());
  //
  let mut f = String::new();
  if let Some(alias) = is_alias_for {
    assert!(comment.is_none());
    assert!(returned_only.is_none());
    assert!(struct_extends.is_none());
    assert!(allow_duplicate.is_none());
    assert!(members.is_empty());
    write!(f,
      "/// Khronos: [{alias}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{alias}.html)
      pub type {name} = {alias};").ok();
  } else {
    writeln!(f, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)").ok();
    if let Some(comment) = comment {
      writeln!(f, "  ///").ok();
      writeln!(f, "/// {comment}").ok();
    }
    if let Some(struct_extends) = struct_extends {
      write!(f, "/// * Struct Extends: ").ok();
      for (i, extended) in struct_extends.split(',').enumerate() {
        if i > 0 {
          write!(f, ", ").ok();
        }
        write!(f, "[{extended}]").ok();
      }
      writeln!(f).ok();
    }
    match returned_only {
      None => (),
      Some("true") => drop(writeln!(f, "/// * Returned Only")),
      Some(other) => panic!("{other:?}"),
    }
    match allow_duplicate {
      None => (),
      Some("false") => (),
      Some("true") => drop(writeln!(f, "/// * Duplicates Allowed")),
      other => panic!("{other:?}"),
    }
    writeln!(f, "#[derive(Clone, Copy)]").ok();
    writeln!(f, "#[repr(C)] pub struct {name} {{").ok();
    for Member {
      name,
      ty_variant,
      ty,
      comment,
      optional,
      no_auto_validity,
      limit_type,
      values,
      len,
      alt_len,
      object_type,
      extern_sync,
      selection,
      selector,
      deprecated,
      api,
    } in members.iter()
    {
      assert!(selection.is_none());
      match api {
        None | Some("vulkan") => (),
        _ => continue,
      }
      match object_type {
        None => (),
        Some("objectType") => (),
        other => panic!("{other:?}"),
      }
      //
      let field: String = field_name(name);
      let field_ty = format_type_and_variant(ty, *ty_variant);
      if let Some(comment) = comment {
        let comment = comment.replace('[', "\\[").replace(']', "\\]");
        writeln!(f, "  /// {comment}").ok();
      }
      if let Some(deprecated) = deprecated {
        writeln!(f, "  /// * Deprecated: {deprecated}").ok();
      }
      if let Some(len) = len {
        writeln!(f, "  /// * Len: `{len}`").ok();
      }
      if let Some(alt_len) = alt_len {
        writeln!(f, "  /// * Alt Len: `{alt_len}`").ok();
      }
      if let Some(limit_type) = limit_type {
        writeln!(f, "  /// * Limit Type: {limit_type}").ok();
      }
      if let Some(selector) = selector {
        writeln!(f, "  /// * Selects which field from `{selector}` should be used").ok();
      }
      if let Some(values) = values {
        write!(f, "  /// * Values: ").ok();
        for (i, extended) in values.split(',').enumerate() {
          if i > 0 {
            write!(f, ", ").ok();
          }
          write!(f, "[`{extended}`]").ok();
        }
        writeln!(f).ok();
      }
      match optional {
        None => (),
        Some("false") => drop(writeln!(f, "  /// * Required")),
        Some("true") => drop(writeln!(f, "  /// * Optional")),
        Some("false,true") => drop(writeln!(f, "  /// * Must be non-null, but may point at 0.")),
        Some("true,false") => {
          writeln!(f, "  /// * Optional, but if non-null must be point to valid data.").ok();
        }
        other => panic!("{other:?}"),
      }
      match no_auto_validity {
        None => (),
        Some("true") => drop(writeln!(f, "  /// * No Auto Validity")),
        other => panic!("{other:?}"),
      }
      match extern_sync {
        None => (),
        Some("true") => drop(writeln!(f, "  /// * Extern Sync")),
        other => panic!("{other:?}"),
      }
      writeln!(f, "  pub {field}: {field_ty},").ok();
    }
    writeln!(f, "}}").ok();
  }
  f
}

pub fn define_union(
  TypeEntry {
    name,
    comment,
    is_alias_for,
    returned_only,
    members,
    struct_extends,
    allow_duplicate,
    category,
    ty_src,
    bit_values,
    obj_type_enum,
    parent,
    deprecated,
    api: _,
    texts: _,
    requires: _,
  }: &TypeEntry,
) -> String {
  assert_eq!(category.unwrap(), "union");
  assert!(ty_src.is_none());
  assert!(obj_type_enum.is_none());
  assert!(parent.is_none());
  assert!(deprecated.is_none());
  assert!(bit_values.is_none());
  //
  let mut f = String::new();
  if let Some(alias) = is_alias_for {
    assert!(comment.is_none());
    assert!(returned_only.is_none());
    assert!(struct_extends.is_none());
    assert!(allow_duplicate.is_none());
    assert!(members.is_empty());
    write!(f,
      "/// Khronos: [{alias}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{alias}.html)
      pub type {name} = {alias};").ok();
  } else {
    writeln!(f, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)").ok();
    if let Some(comment) = comment {
      writeln!(f, "/// ").ok();
      let comment = comment.strip_prefix("// ").unwrap_or(comment);
      writeln!(f, "/// {comment}").ok();
    }
    if let Some(struct_extends) = struct_extends {
      write!(f, "/// * Struct Extends: ").ok();
      for (i, extended) in struct_extends.split(',').enumerate() {
        if i > 0 {
          write!(f, ", ").ok();
        }
        write!(f, "[{extended}]").ok();
      }
      writeln!(f).ok();
    }
    match returned_only {
      None => (),
      Some("true") => drop(writeln!(f, "/// * Returned Only")),
      Some(other) => panic!("{other:?}"),
    }
    match allow_duplicate {
      None => (),
      Some("false") => (),
      Some("true") => drop(writeln!(f, "/// * Duplicates Allowed")),
      other => panic!("{other:?}"),
    }
    writeln!(f, "#[derive(Clone, Copy)]").ok();
    writeln!(f, "#[repr(C)] pub union {name} {{").ok();
    for Member {
      name,
      ty_variant,
      ty,
      comment,
      optional,
      no_auto_validity,
      limit_type,
      values,
      len,
      alt_len,
      object_type,
      extern_sync,
      selection,
      selector,
      deprecated,
      api,
    } in members.iter()
    {
      assert!(selection.is_none());
      match api {
        None | Some("vulkan") => (),
        _ => continue,
      }
      match object_type {
        None => (),
        Some("objectType") => (),
        other => panic!("{other:?}"),
      }
      //
      let field: String = field_name(name);
      let field_ty = format_type_and_variant(ty, *ty_variant);
      if let Some(comment) = comment {
        writeln!(f, "  /// {comment}").ok();
      }
      if let Some(deprecated) = deprecated {
        writeln!(f, "  /// * Deprecated: {deprecated}").ok();
      }
      if let Some(len) = len {
        writeln!(f, "  /// * Len: `{len}`").ok();
      }
      if let Some(alt_len) = alt_len {
        writeln!(f, "  /// * Alt Len: `{alt_len}`").ok();
      }
      if let Some(limit_type) = limit_type {
        writeln!(f, "  /// * Limit Type: {limit_type}").ok();
      }
      if let Some(selector) = selector {
        writeln!(f, "  /// * Selects which field from `{selector}` should be used").ok();
      }
      if let Some(values) = values {
        write!(f, "  /// * Values: ").ok();
        for (i, extended) in values.split(',').enumerate() {
          if i > 0 {
            write!(f, ", ").ok();
          }
          write!(f, "[`{extended}`]").ok();
        }
        writeln!(f).ok();
      }
      match optional {
        None => (),
        Some("false") => drop(writeln!(f, "  /// * Required")),
        Some("true") => drop(writeln!(f, "  /// * Optional")),
        Some("false,true") => drop(writeln!(f, "  /// * Must be non-null, but may point at 0.")),
        Some("true,false") => {
          writeln!(f, "  /// * Optional, but if non-null must be point to valid data.").ok();
        }
        other => panic!("{other:?}"),
      }
      match no_auto_validity {
        None => (),
        Some("true") => drop(writeln!(f, "  /// * No Auto Validity")),
        other => panic!("{other:?}"),
      }
      match extern_sync {
        None => (),
        Some("true") => drop(writeln!(f, "  /// * Extern Sync")),
        other => panic!("{other:?}"),
      }
      writeln!(f, "  pub {field}: {field_ty},").ok();
    }
    writeln!(f, "}}").ok();
  }
  f
}

pub fn format_type_and_variant(ty: &str, ty_variant: TypeVariant) -> String {
  #[allow(clippy::useless_format)]
  match ty_variant {
    TypeVariant::Normal => format!("{ty}"),
    TypeVariant::ConstPtr => format!("*const {ty}"),
    TypeVariant::MutPtr => format!("*mut {ty}"),
    TypeVariant::MutPtrMutPtr => format!("*mut *mut {ty}"),
    TypeVariant::ConstArrayPtrLit(n) => format!("*const [{ty}; {n}]"),
    TypeVariant::ConstArrayPtrNamed(n) => format!("*const [{ty}; {n}]"),
    TypeVariant::MutPtrConstPtr => format!("*mut *const {ty}"),
    TypeVariant::ConstPtrConstPtr => format!("*const *const {ty}"),
    TypeVariant::ArrayLit(n) => format!("[{ty}; {n}]"),
    TypeVariant::ArrayOfArrayLit(a, b) => format!("[[{ty}; {a}]; {b}]"),
    TypeVariant::BitfieldsLit(n) => format!("{ty}{{:{n}}}"),
  }
}
