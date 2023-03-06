use super::*;

#[derive(Debug, Clone, Default)]
pub struct Enumeration {
  pub name: StaticStr,
  pub values: BTreeMap<StaticStr, ConstNewtypeValue>,
  pub aliases: BTreeMap<StaticStr, ConstAlias>,
  pub signed: bool,
}
impl Display for Enumeration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let Enumeration { name, values, aliases, signed } = self;
    writeln!(f, "define_enumeration!(")?;
    writeln!(f, "  /// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)")?;
    if *signed {
      writeln!(f, "  {name}(i32)")?;
    } else {
      writeln!(f, "  {name}(u32)")?;
    }
    writeln!(f, ");")?;
    for value in values.values() {
      core::fmt::Display::fmt(value, f)?;
    }
    for alias in aliases.values() {
      core::fmt::Display::fmt(alias, f)?;
    }
    writeln!(f, "impl core::fmt::Debug for {name} {{")?;
    writeln!(
      f,
      "  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{"
    )?;
    writeln!(f, "    let s = match *self {{")?;
    for value in values.values() {
      let const_name = value.name;
      writeln!(f, "      {const_name} => \"{const_name}\",")?;
    }
    writeln!(f, "      other => return write!(f, \"{name}({{}})\", other.0),")?;
    writeln!(f, "    }};")?;
    writeln!(f, "    write!(f, \"{{s}}\")")?;
    writeln!(f, "  }}")?;
    writeln!(f, "}}")?;
    Ok(())
  }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConstNewtypeValue {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub value: i32,
  pub comment: Option<StaticStr>,
}
impl Display for ConstNewtypeValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ConstNewtypeValue { name, ty, value, comment } = self;
    if let Some(comment) = comment {
      writeln!(f, r#"/// {comment}"#)?;
    }
    writeln!(f, "pub const {name}: {ty} = {ty}({value});")
  }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConstAlias {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub alias_of: StaticStr,
  pub deprecated: Option<StaticStr>,
  pub comment: Option<StaticStr>,
}
impl Display for ConstAlias {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ConstAlias { name, ty, alias_of, deprecated, comment } = self;
    writeln!(f, "/// Alias of [`{alias_of}`]")?;
    if let Some(comment) = comment {
      writeln!(f, "///")?;
      writeln!(f, r#"/// {comment}"#)?;
    }
    if let Some(reason) = deprecated {
      writeln!(f, r#"#[deprecated = "{reason}"]"#)?;
    }
    writeln!(f, "pub const {name}: {ty} = {alias_of};")
  }
}

pub fn write_out_enumerations<P: AsRef<Path>>(
  p: P, enumerations: &BTreeMap<StaticStr, Enumeration>,
) {
  let path = p.as_ref();
  std::fs::remove_file(path).unwrap();
  let mut f = OpenOptions::new().write(true).create(true).open(path).unwrap();
  writeln!(f, "#![allow(non_upper_case_globals)]").unwrap();
  writeln!(f).unwrap();
  for enumeration in enumerations.values() {
    writeln!(f, "{enumeration}").unwrap();
  }
}

const LOST_DEVICE: &str = "The logical device has been lost. Spec: [5.2.3. Lost Device](https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device).";

pub fn gather_enumerations(
  registry: &VulkanRegistry,
) -> BTreeMap<StaticStr, Enumeration> {
  let mut output = BTreeMap::new();

  for enum_ty in registry.enumeration_types.iter() {
    if enum_ty.name.contains("Flags") || enum_ty.name.contains("FlagBits") {
      continue;
    }
    let e = output.entry(enum_ty.name).or_insert(Enumeration::default());
    let name = enum_ty.name;
    e.name = name;
  }

  for enum_group in registry.enums_groups.iter() {
    if enum_group.name.contains("Flags") || enum_group.name.contains("FlagBits") {
      continue;
    }
    let e = output.get_mut(enum_group.name).unwrap();
    if enum_group.ty != Some("enum") {
      continue;
    }
    assert!(enum_group.bit_positions.is_empty());
    for EnumAlias { name, alias_of, api, deprecated } in
      enum_group.aliases.iter().cloned()
    {
      if !api_vulkan(api) {
        continue;
      }
      let alias =
        ConstAlias { name, ty: enum_group.name, alias_of, deprecated, comment: None };
      match e.aliases.entry(name) {
        Entry::Vacant(ve) => {
          ve.insert(alias);
        }
        Entry::Occupied(oe) => assert_eq!(oe.get(), &alias),
      }
    }
    for EnumValue { name, value, comment } in enum_group.values.iter().cloned() {
      let value = if let Some(hex) = value.strip_prefix("0x") {
        i32::from_str_radix(hex, 16).unwrap()
      } else {
        i32::from_str_radix(value, 10).unwrap()
      };
      let mut v = ConstNewtypeValue { name, ty: enum_group.name, value, comment };
      if v.name == "VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE" {
        v.comment = None;
      }
      if let Some(comment) = v.comment {
        if comment.contains("devsandqueues-lost-device") {
          v.comment = Some(LOST_DEVICE);
        }
      }
      if v.value.is_negative() {
        e.signed = true;
      }
      match e.values.entry(name) {
        Entry::Vacant(ve) => {
          ve.insert(v);
        }
        Entry::Occupied(oe) => assert_eq!(oe.get(), &v),
      }
    }
  }

  let feature_requirements = registry
    .features
    .iter()
    .filter(|f| api_vulkan(Some(f.api)))
    .flat_map(|f| f.requirements.iter());
  let extension_requirements =
    registry.extensions.iter().flat_map(|x| x.requirements.iter());
  let all_requirements = feature_requirements.chain(extension_requirements);

  for requirement in all_requirements {
    if !api_vulkan(requirement.api) {
      continue;
    }
    for RequiredEnumAlias { name, alias_of, extends, comment, api, deprecated } in
      requirement.required_alias_enums.iter().cloned()
    {
      if !api_vulkan(api) {
        continue;
      }
      if let Some(ty) = extends {
        if ty.contains("Flags") || ty.contains("FlagBits") {
          continue;
        }
        let e = output.get_mut(ty).unwrap();
        let alias = ConstAlias { name, ty, alias_of, deprecated, comment };
        match e.aliases.entry(name) {
          Entry::Vacant(ve) => {
            ve.insert(alias);
          }
          Entry::Occupied(oe) => assert_eq!(oe.get(), &alias),
        }
      } else {
        // If the alias doesn't isn't typed as an enumeration we've found so far
        // I guess we don't need to care, since this function is only about
        // gathering up enumerations.
      }
    }
    for RequiredEnumBitpos { name, extends, .. } in
      requirement.required_bitpos_enums.iter().cloned()
    {
      if output.contains_key(extends) {
        panic!("{name} declared as bitpos of enumeration {extends}");
      }
    }
    for RequiredEnumOffset {
      name,
      extends,
      extension_number,
      offset,
      comment,
      is_negative,
      api,
      protect,
    } in requirement.required_offset_enums.iter().cloned()
    {
      if protect.is_some() {
        continue;
      }
      if !api_vulkan(api) {
        continue;
      }
      if extends.contains("Flags") || extends.contains("FlagBits") {
        continue;
      }
      let e = output.get_mut(extends).unwrap();
      let mut v = ConstNewtypeValue {
        name,
        ty: extends,
        value: if is_negative {
          -extension_enumeration_value(extension_number, offset)
        } else {
          extension_enumeration_value(extension_number, offset)
        },
        comment,
      };
      if v.name == "VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE" {
        v.comment = None;
      }
      if v.value.is_negative() {
        e.signed = true;
      }
      match e.values.entry(name) {
        Entry::Vacant(ve) => {
          ve.insert(v);
        }
        Entry::Occupied(oe) => {
          assert_eq!(oe.get(), &v)
        }
      }
    }
    for RequiredEnumValue { name, extends, value, comment, api } in
      requirement.required_value_enums.iter().cloned()
    {
      if !api_vulkan(api) {
        continue;
      }
      if extends.is_empty() || extends.contains("Flags") || extends.contains("FlagBits") {
        continue;
      }
      let e = output.get_mut(extends).unwrap();
      let mut value =
        ConstNewtypeValue { name, ty: extends, value: value.parse().unwrap(), comment };
      if value.name == "VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE" {
        value.comment = None;
      }
      match e.values.entry(name) {
        Entry::Vacant(ve) => {
          ve.insert(value);
        }
        Entry::Occupied(oe) => {
          assert_eq!(oe.get(), &value)
        }
      }
    }
  }

  output
}

/// Makes a vulkan enumeration value, according to the [specification][vk].
///
/// [vk]: https://registry.khronos.org/vulkan/specs/1.3/styleguide.html#_assigning_extension_token_values
pub(crate) const fn extension_enumeration_value(ext: i32, offset: i32) -> i32 {
  const BASE_VALUE: i32 = 1000000000;
  const RANGE_SIZE: i32 = 1000;
  BASE_VALUE + (ext - 1) * RANGE_SIZE + offset
}
