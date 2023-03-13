use super::*;

#[derive(Debug, Clone, Default)]
pub struct RustUnion {
  pub name: StaticStr,
  pub comment: Option<StaticStr>,
  pub returned_only: bool,
  pub members: Vec<RustUnionMember>,
}
impl Display for RustUnion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let RustUnion { name, comment, returned_only, members } = self;
    writeln!(f, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (union)")?;
    if comment.is_some() || self.returned_only {
      writeln!(f, "///")?;
    }
    if let Some(comment) = comment {
      let comment = comment.strip_prefix("// ").unwrap_or(comment);
      writeln!(f, "/// {comment}")?;
    }
    if *returned_only {
      writeln!(f, "/// * Returned Only")?;
    }
    writeln!(f, "#[derive(Clone, Copy)]")?;
    writeln!(f, "#[repr(C)]")?;
    writeln!(f, "pub union {name} {{")?;
    for m in members.iter() {
      core::fmt::Display::fmt(m, f)?;
    }
    writeln!(f, "}}")?;
    writeln!(f, "impl Default for {name} {{")?;
    writeln!(f, "  #[inline]")?;
    writeln!(f, "  #[must_use]")?;
    writeln!(f, "  fn default() -> Self {{")?;
    let default_field_name = members
      .iter()
      .find(|m| {
        // "is not a pointer"
        !matches!(
          m.ty_variant,
          TypeVariant::ConstPtr
            | TypeVariant::MutPtr
            | TypeVariant::ConstPtrConstPtr
            | TypeVariant::MutPtrMutPtr
            | TypeVariant::ConstPtrArrayInt(_)
        )
      })
      .unwrap()
      .name;
    let default_field_name = fix_member_name(default_field_name);
    writeln!(f, "    Self {{ {default_field_name}: Default::default() }}")?;
    writeln!(f, "  }}")?;
    writeln!(f, "}}")?;
    Ok(())
  }
}

#[derive(Debug, Clone, Default)]
pub struct RustUnionMember {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub ty_variant: TypeVariant,
  pub selection: Option<StaticStr>,
  pub optional: Option<StaticStr>,
  pub len: Option<StaticStr>,
  pub no_auto_validity: bool,
}
impl Display for RustUnionMember {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let RustUnionMember {
      name,
      ty,
      ty_variant,
      selection,
      optional,
      len,
      no_auto_validity,
    } = self;
    if let Some(selection) = selection {
      for selection_name in selection.split(',') {
        writeln!(f, "  /// * Selection: [`{selection_name}`]")?;
      }
    }
    if let Some(optional) = optional {
      writeln!(f, "  /// * Optional: {optional}")?;
    }
    if let Some(len) = len {
      writeln!(f, "  /// * Len: {len}")?;
    }
    if *no_auto_validity {
      writeln!(f, "  /// * No Auto Validity")?;
    }
    let rust_ty = format_ty_and_variant(ty, *ty_variant);
    let name = fix_member_name(name);
    writeln!(f, "  pub {name}: {rust_ty},")?;
    Ok(())
  }
}

pub fn gather_unions(registry: &VulkanRegistry) -> BTreeMap<StaticStr, RustUnion> {
  let mut output = BTreeMap::new();

  for Union { name, members, comment, returned_only } in registry.unions.iter() {
    let u = output.entry(*name).or_insert(RustUnion::default());
    u.name = name;
    u.comment = *comment;
    u.returned_only = *returned_only;
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
      assert!(comment.is_none());
      assert!(limit_type.is_none());
      assert!(value.is_none());
      assert!(alt_len.is_none(), "{}", u.name);
      assert!(deprecated.is_none());
      assert!(object_type.is_none());
      assert!(extern_sync.is_none());
      assert!(selector.is_none());
      assert!(bitfields.is_none());
      u.members.push(RustUnionMember {
        name,
        ty: filter_ty(ty),
        ty_variant,
        optional,
        no_auto_validity,
        len,
        selection,
      });
    }
  }

  output
}

pub fn write_out_unions<P: AsRef<Path>>(p: P, unions: &BTreeMap<StaticStr, RustUnion>) {
  let path = p.as_ref();
  std::fs::remove_file(path).unwrap();
  let mut f = OpenOptions::new().write(true).create(true).open(path).unwrap();
  writeln!(f, "#![allow(non_upper_case_globals)]").unwrap();
  writeln!(f).unwrap();
  writeln!(f, "use crate::prelude::*;").unwrap();
  writeln!(f).unwrap();
  for u in unions.values() {
    if BLOCKED_TYPES.contains(&u.name)
      || u.members.iter().map(|m| m.ty).any(|m_ty| BLOCKED_TYPES.contains(&m_ty))
    {
      continue;
    }
    writeln!(f, "{u}").unwrap();
  }
}
