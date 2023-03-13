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
    writeln!(f, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (structure)")?;
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
    if !members.iter().any(|m| {
      m.ty.starts_with("PFN_")
        || (m.ty_variant == TypeVariant::Normal && m.ty == "VkClearValue")
    }) {
      writeln!(f, "#[derive(Debug)]")?;
    }
    writeln!(f, "#[repr(C)]")?;
    writeln!(f, "pub struct {name} {{")?;
    for m in members.iter() {
      core::fmt::Display::fmt(m, f)?;
    }
    writeln!(f, "}}")?;
    writeln!(f, "impl Default for {name} {{")?;
    writeln!(f, "  #[inline]")?;
    writeln!(f, "  #[must_use]")?;
    writeln!(f, "  fn default() -> Self {{")?;
    if members.iter().any(|m| m.deprecated.is_some()) {
      writeln!(f, "    #[allow(deprecated)]")?;
    }
    writeln!(f, "    Self {{")?;
    for m in members.iter() {
      let member_name = fix_member_name(m.name);
      let default_expr: String = if let Some(const_name) = m.value {
        const_name.to_string()
      } else {
        format_default_expr(m.ty, m.ty_variant)
      };
      writeln!(f, "      {member_name}: {default_expr},")?;
    }
    writeln!(f, "    }}")?;
    writeln!(f, "  }}")?;
    writeln!(f, "}}")?;
    Ok(())
  }
}

fn format_default_expr(ty: StaticStr, ty_variant: TypeVariant) -> String {
  if [
    "HWND",
    "HINSTANCE",
    "HMONITOR",
    "LPCWSTR",
    "HANDLE",
    "MTLTexture_id",
    "MTLSharedEvent_id",
    "MTLBuffer_id",
    "MTLCommandQueue_id",
    "MTLDevice_id",
    "IOSurfaceRef",
  ]
  .contains(&ty)
  {
    return "core::ptr::null_mut()".to_string();
  }
  match ty_variant {
    TypeVariant::Normal => "Default::default()".to_string(),
    TypeVariant::ArraySym(s) if ty == "u8" && s != "VK_UUID_SIZE" => {
      "Default::default()".to_string()
    }
    TypeVariant::ArraySym(s) => format!("[Default::default(); {s}]"),
    TypeVariant::ArrayInt(n) => format!("[Default::default(); {n}]"),
    TypeVariant::ArrayArrayInt(j, k) => format!("[[Default::default(); {j}]; {k}]"),
    TypeVariant::ConstPtr
    | TypeVariant::ConstPtrConstPtr
    | TypeVariant::ConstPtrArrayInt(_) => "core::ptr::null()".to_string(),
    TypeVariant::MutPtr | TypeVariant::MutPtrMutPtr => {
      "core::ptr::null_mut()".to_string()
    }
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
      let chunks: Vec<String> = len.split_whitespace().map(fix_member_name).collect();
      let len = chunks.join(" ");
      // We favor Alt Len when available, because that's the one that (if present)
      // will look "like code" instead of being LaTeX or whatever.
      writeln!(f, "  /// * Len: `{len}`")?;
    }
    if let Some(object_type) = *object_type {
      writeln!(f, "  /// * Object Type: {object_type}")?;
    }
    if let Some(selector) = *selector {
      let selector = fix_member_name(selector);
      writeln!(f, "  /// * Active Variant Selected By: {selector}")?;
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
    let name = fix_member_name(name);
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
      name: member_name,
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
      let ty = if *name == "VkLayerProperties" && member_name == "specVersion" {
        "VkVersion"
      } else {
        ty
      };
      s.members.push(RustStructureMember {
        name: match member_name {
          "type" => "ty",
          otherwise => otherwise,
        },
        ty: fix_ty(ty),
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
    if HANDWRITTEN_TYPES.contains(&s.name)
      || BLOCKED_TYPES.contains(&s.name)
      || s.members.iter().map(|m| m.ty).any(|m_ty| BLOCKED_TYPES.contains(&m_ty))
      || s.members.iter().any(|m| m.bits.is_some())
    {
      continue;
    }
    writeln!(f, "{s}").unwrap();
  }
}
