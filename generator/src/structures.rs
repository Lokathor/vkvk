use super::*;

#[derive(Debug, Clone, Default)]
pub struct RustStructure {
  pub name: StaticStr,
  pub comment: Option<StaticStr>,
  pub members: Vec<RustStructureMember>,
  pub returned_only: bool,
  pub struct_extends: Option<StaticStr>,
  pub allow_duplicate: bool,
}
impl Display for RustStructure {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let RustStructure {
      name,
      comment,
      members,
      returned_only,
      struct_extends,
      allow_duplicate,
    } = self;
    writeln!(f, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)")?;
    if comment.is_some() || struct_extends.is_some() || *allow_duplicate || *returned_only
    {
      writeln!(f, "///")?;
    }
    if let Some(comment) = comment {
      let comment = comment.strip_prefix("// ").unwrap_or(comment);
      writeln!(f, "/// {comment}")?;
    }
    if let Some(struct_extends) = struct_extends {
      for extend in struct_extends.split(',') {
        writeln!(f, "/// * Struct Extends: [`{extend}`]")?;
      }
    }
    if *allow_duplicate {
      writeln!(f, "/// * Duplicates Allowed")?;
    }
    if *returned_only {
      writeln!(f, "/// * Returned Only")?;
    }
    writeln!(f, "#[derive(Clone, Copy)]")?;
    writeln!(f, "#[repr(C)]")?;
    writeln!(f, "pub struct {name} {{")?;
    for m in members.iter() {
      core::fmt::Display::fmt(m, f)?;
    }
    writeln!(f, "}}")?;
    Ok(())
  }
}

#[derive(Debug, Clone, Default)]
pub struct RustStructureMember {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub ty_variant: TypeVariant,
  pub comment: Option<StaticStr>,
  pub optional: Option<StaticStr>,
  pub limit_type: Option<StaticStr>,
  pub value: Option<StaticStr>,
  pub len: Option<StaticStr>,
  pub alt_len: Option<StaticStr>,
  pub no_auto_validity: bool,
  pub deprecated: Option<StaticStr>,
  pub object_type: Option<StaticStr>,
  pub extern_sync: Option<StaticStr>,
  pub selector: Option<StaticStr>,
  pub bits: Option<u32>,
}
impl Display for RustStructureMember {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let RustStructureMember {
      name,
      ty,
      ty_variant,
      comment,
      optional,
      limit_type,
      value,
      len,
      alt_len,
      no_auto_validity,
      deprecated,
      object_type,
      extern_sync,
      selector,
      bits: _, /* TODO */
    } = self;
    if let Some(comment) = comment {
      let comment = comment.strip_prefix("// ").unwrap_or(comment);
      let comment = comment.replace('[', "\\[").replace(']', "\\]");
      writeln!(f, "  /// {comment}")?;
    }
    if let Some(optional) = *optional {
      writeln!(f, "  /// * Optional: {optional}")?;
    }
    if let Some(limit_type) = *limit_type {
      writeln!(f, "  /// * Limit Type: {limit_type}")?;
    }
    if let Some(value) = *value {
      writeln!(f, "  /// * Intended Value: `{value}`")?;
    }
    if let Some(len) = alt_len.or(*len) {
      // We favor Alt Len when available, because that's the one that (if present)
      // will look "like code" instead of being LaTeX or whatever.
      writeln!(f, "  /// * Len: `{len}`")?;
    }
    if let Some(object_type) = *object_type {
      writeln!(f, "  /// * Object Type: {object_type}")?;
    }
    if let Some(selector) = *selector {
      writeln!(f, "  /// * Selector: {selector}")?;
    }
    if let Some(extern_sync) = *extern_sync {
      writeln!(f, "  /// * Extern Sync: {extern_sync}")?;
    }
    if *no_auto_validity {
      writeln!(f, "  /// * No Auto-Validity")?;
    }
    if let Some(deprecated) = *deprecated {
      writeln!(f, "  #[deprecated = \"{deprecated}\"]")?;
    }
    let rust_ty = format_ty_and_variant(ty, *ty_variant);
    writeln!(f, "  pub {name}: {rust_ty},")?;
    Ok(())
  }
}

pub fn gather_structures(
  registry: &VulkanRegistry,
) -> BTreeMap<StaticStr, RustStructure> {
  let mut output = BTreeMap::new();

  for Structure {
    name,
    members,
    returned_only,
    struct_extends,
    comment,
    allow_duplicate,
  } in registry.structures.iter()
  {
    let s = output.entry(*name).or_insert(RustStructure::default());
    s.name = name;
    s.comment = *comment;
    s.returned_only = *returned_only;
    s.struct_extends = *struct_extends;
    s.allow_duplicate = *allow_duplicate;
    for Member {
      name,
      ty,
      ty_variant,
      optional,
      no_auto_validity,
      limit_type,
      comment,
      value,
      len,
      alt_len,
      deprecated,
      api,
      object_type,
      extern_sync,
      selection,
      selector,
      bitfields,
    } in members.iter().cloned()
    {
      if !api_vulkan(api) {
        continue;
      }
      assert!(selection.is_none());
      s.members.push(RustStructureMember {
        name: match name {
          "type" => "ty",
          otherwise => otherwise,
        },
        ty: filter_ty(ty),
        ty_variant,
        comment,
        optional,
        limit_type,
        value,
        len,
        alt_len,
        no_auto_validity,
        deprecated,
        object_type,
        extern_sync,
        selector,
        bits: bitfields,
      });
    }
  }

  output
}

pub fn write_out_structures<P: AsRef<Path>>(
  p: P, structures: &BTreeMap<StaticStr, RustStructure>,
) {
  let path = p.as_ref();
  std::fs::remove_file(path).unwrap();
  let mut f = OpenOptions::new().write(true).create(true).open(path).unwrap();
  writeln!(f, "#![allow(non_upper_case_globals)]").unwrap();
  writeln!(f).unwrap();
  writeln!(f, "use crate::prelude::*;").unwrap();
  writeln!(f).unwrap();
  for s in structures.values() {
    if BLOCKED_TYPES.contains(&s.name)
      || s.members.iter().map(|m| m.ty).any(|m_ty| BLOCKED_TYPES.contains(&m_ty))
      || s.members.iter().any(|m| m.bits.is_some())
    {
      continue;
    }
    writeln!(f, "{s}").unwrap();
  }
}
