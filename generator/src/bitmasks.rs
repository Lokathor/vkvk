use convert_case::{Case, Casing};

use super::*;

#[derive(Debug, Clone, Default)]
pub struct Flags {
  pub core_name: StaticStr,
  pub digits: Option<StaticStr>,
  pub vendor: Option<StaticStr>,
  pub is_64_bit: bool,
  pub xml_defines_bits: bool,
  pub comment: Option<StaticStr>,
  pub positions: BTreeMap<StaticStr, ConstBitPos>,
  pub values: BTreeMap<StaticStr, ConstNewtypeValue>,
  pub aliases: BTreeMap<StaticStr, ConstAlias>,
}
impl Display for Flags {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let Flags {
      core_name,
      digits,
      vendor,
      is_64_bit,
      xml_defines_bits,
      comment,
      positions,
      values,
      aliases,
    } = self;
    let digits = digits.unwrap_or("");
    let vendor = vendor.unwrap_or("");
    let ty = if *is_64_bit { "u64" } else { "u32" };
    writeln!(f, "define_bitmask!(")?;
    if *xml_defines_bits {
      writeln!(f, "  /// Khronos: [{core_name}FlagBits{digits}{vendor}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{core_name}FlagBits{digits}{vendor}.html) (bitmask)")?;
    } else {
      writeln!(f, "  /// Khronos: [{core_name}Flags{digits}{vendor}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{core_name}Flags{digits}{vendor}.html) (bitmask, no bits defined)")?;
    }
    if let Some(comment) = comment {
      writeln!(f, "///")?;
      writeln!(f, "/// {comment}")?;
    }
    writeln!(f, "  {core_name}FlagBits{digits}{vendor}({ty})")?;
    writeln!(f, ");")?;

    if *xml_defines_bits {
      writeln!(f, "/// Khronos: [{core_name}FlagBits{digits}{vendor}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{core_name}FlagBits{digits}{vendor}.html) (bitmask)")?;
    } else {
      writeln!(f, "/// Khronos: [{core_name}Flags{digits}{vendor}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{core_name}Flags{digits}{vendor}.html) (bitmask, no bits defined)")?;
    }
    writeln!(
      f,
      "pub type {core_name}Flags{digits}{vendor} = {core_name}FlagBits{digits}{vendor};"
    )?;

    for position in positions.values() {
      core::fmt::Display::fmt(position, f)?;
    }
    for value in values.values() {
      core::fmt::Display::fmt(value, f)?;
    }
    for alias in aliases.values() {
      core::fmt::Display::fmt(alias, f)?;
    }

    writeln!(f, "impl core::fmt::Debug for {core_name}FlagBits{digits}{vendor} {{")?;
    writeln!(f, "#[allow(clippy::missing_inline_in_public_items)]")?;
    writeln!(
      f,
      "  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{"
    )?;
    writeln!(f, "    let mut x = f.debug_set();")?;
    if !positions.is_empty() {
      writeln!(f, "    for (bit_val, bit_name) in [")?;
      for position in positions.values() {
        let bit_val = 1_u64 << position.bit;
        let bit_name = position.bit_name.to_case(Case::Snake);
        writeln!(f, "      ({bit_val}, \"{bit_name}\"),")?;
      }
      writeln!(f, "    ] {{")?;
      writeln!(f, "      if (self.0 & bit_val) != 0 {{")?;
      writeln!(f, "        x.entry(&bit_name);")?;
      writeln!(f, "      }}")?;
      writeln!(f, "    }}")?;
    }
    writeln!(f, "    x.finish()")?;
    writeln!(f, "  }}")?;
    writeln!(f, "}}")?;
    if !positions.is_empty() {
      writeln!(f, "impl {core_name}FlagBits{digits}{vendor} {{")?;
      for position in positions.values() {
        let bit_val = 1_u64 << position.bit;
        let prefix = if position.bit_name.starts_with(|ch: char| ch.is_ascii_digit()) {
          "_"
        } else {
          ""
        };
        let bit_name = position.bit_name.to_case(Case::Snake);
        writeln!(f, "  #[inline]")?;
        writeln!(f, "  #[must_use]")?;
        writeln!(f, "  pub const fn {prefix}{bit_name}(self) -> bool {{")?;
        writeln!(f, "    (self.0 & {bit_val}) != 0")?;
        writeln!(f, "  }}")?;
      }
      writeln!(f, "}}")?;
    }
    Ok(())
  }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConstBitPos {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub bit: u32,
  pub bit_name: StaticStr,
  pub comment: Option<StaticStr>,
}
impl Display for ConstBitPos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ConstBitPos { name, bit, comment, ty, bit_name: _ } = self;
    if let Some(comment) = comment {
      writeln!(f, r#"/// {comment}"#)?;
    }
    writeln!(f, "pub const {name}: {ty} = {ty}(1 << {bit});")
  }
}

pub fn gather_bitmasks(
  registry: &VulkanRegistry,
) -> BTreeMap<(StaticStr, Option<StaticStr>, Option<StaticStr>), Flags> {
  let mut output = BTreeMap::new();

  for bitmask in registry.bitmasks.iter() {
    if !api_vulkan(bitmask.api) {
      continue;
    }
    assert!(bitmask.name.contains("Flags"));
    let (vendorless, vendor) = break_vendor(registry, bitmask.name);
    let (numberless, digits) = break_number(vendorless);
    let core_name = numberless.strip_suffix("Flags").unwrap();
    let flags = output.entry((core_name, digits, vendor)).or_insert(Flags::default());
    flags.core_name = core_name;
    flags.digits = digits;
    flags.vendor = vendor;
    flags.is_64_bit = bitmask.flags64;
    flags.xml_defines_bits = if bitmask.flags64 {
      bitmask.bit_values.is_some()
    } else {
      bitmask.requires.is_some()
    };
  }

  // Note(Lokathor): registry.enumeration_types.iter() would let us check for the
  // "FlagBits" parts of each entry, but the data is just a name anyway so it
  // doesn't matter.

  for EnumsGroup {
    name: group_name,
    comment,
    values,
    aliases,
    bit_positions,
    ty,
    is_64_bit,
  } in registry.enums_groups.iter()
  {
    if *ty != Some("bitmask") {
      continue;
    }
    assert!(group_name.contains("FlagBits"));
    let (vendorless, vendor) = break_vendor(registry, group_name);
    let (numberless, digits) = break_number(vendorless);
    let core_name = numberless.strip_suffix("FlagBits").unwrap();
    let flags = output.get_mut(&(core_name, digits, vendor)).unwrap();
    flags.comment = *comment;
    assert_eq!(*is_64_bit, flags.is_64_bit, "{group_name}");

    let screaming_core = format!("{core_name}{}", digits.unwrap_or(""))
      .to_case(Case::ScreamingSnake)
      .replace("H_264", "H264")
      .replace("H_265", "H265");

    for EnumBitPosition { name, bit, comment } in bit_positions.iter().cloned() {
      let no_type_words =
        name.strip_prefix(&screaming_core).unwrap().strip_prefix('_').unwrap();
      let (no_vendor, _) = break_vendor(registry, no_type_words);
      let no_vendor = no_vendor.strip_suffix('_').unwrap_or(no_vendor);
      let bit_name = no_vendor.strip_suffix("_BIT").unwrap_or(no_vendor);
      //
      let pos = ConstBitPos { name, ty: group_name, bit, comment, bit_name };
      match flags.positions.entry(name) {
        Entry::Vacant(ve) => {
          ve.insert(pos);
        }
        Entry::Occupied(oe) => assert_eq!(oe.get(), &pos),
      }
    }

    for EnumValue { name, value, comment } in values.iter().cloned() {
      let value = if let Some(hex) = value.strip_prefix("0x") {
        i32::from_str_radix(hex, 16).unwrap()
      } else {
        i32::from_str_radix(value, 10).unwrap()
      };
      let value = ConstNewtypeValue { name, ty: group_name, value, comment };
      match flags.values.entry(name) {
        Entry::Vacant(ve) => {
          ve.insert(value);
        }
        Entry::Occupied(oe) => assert_eq!(oe.get(), &value),
      }
    }

    for EnumAlias { name, alias_of, api, deprecated } in aliases.iter().cloned() {
      if !api_vulkan(api) {
        continue;
      }
      let alias =
        ConstAlias { name, ty: group_name, alias_of, deprecated, comment: None };
      match flags.aliases.entry(name) {
        Entry::Vacant(ve) => {
          ve.insert(alias);
        }
        Entry::Occupied(oe) => assert_eq!(oe.get(), &alias),
      }
    }
  }

  let feature_requirements = registry
    .features
    .iter()
    .filter(|f| api_vulkan(Some(f.api)))
    .flat_map(|f| f.requirements.iter());
  let extension_requirements = registry
    .extensions
    .iter()
    .filter(|x| !BLOCKED_EXTENSIONS.contains(&x.name))
    .flat_map(|x| x.requirements.iter());
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
        if !(ty.contains("Flags") || ty.contains("FlagBits")) {
          continue;
        }
        let (vendorless, vendor) = break_vendor(registry, ty);
        let (numberless, digits) = break_number(vendorless);
        let core_name = numberless.strip_suffix("FlagBits").unwrap();
        let flags = output.get_mut(&(core_name, digits, vendor)).unwrap();
        let alias = ConstAlias { name, ty, alias_of, deprecated, comment };
        match flags.aliases.entry(name) {
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
    for RequiredEnumBitpos { name, extends, bitpos, comment, protect } in
      requirement.required_bitpos_enums.iter().cloned()
    {
      if protect.is_some() {
        continue;
      }
      let (vendorless, vendor) = break_vendor(registry, extends);
      let (numberless, digits) = break_number(vendorless);
      let core_name = numberless.strip_suffix("FlagBits").unwrap();
      let flags = output.get_mut(&(core_name, digits, vendor)).unwrap();
      let screaming_core = format!("{core_name}{}", digits.unwrap_or(""))
        .to_case(Case::ScreamingSnake)
        .replace("H_264", "H264")
        .replace("H_265", "H265");
      let no_type_words =
        name.strip_prefix(&screaming_core).unwrap().strip_prefix('_').unwrap();
      let (no_vendor, _) = break_vendor(registry, no_type_words);
      let no_vendor = no_vendor.strip_suffix('_').unwrap_or(no_vendor);
      let bit_name = no_vendor.strip_suffix("_BIT").unwrap_or(no_vendor);
      //
      let pos = ConstBitPos { name, ty: extends, bit: bitpos, comment, bit_name };
      match flags.positions.entry(name) {
        Entry::Vacant(ve) => {
          ve.insert(pos);
        }
        Entry::Occupied(oe) => assert_eq!(oe.get(), &pos),
      }
    }
    for RequiredEnumOffset { name, extends, api, protect, .. } in
      requirement.required_offset_enums.iter().cloned()
    {
      if protect.is_some()
        || !api_vulkan(api)
        || !(extends.contains("Flags") || extends.contains("FlagBits"))
      {
        continue;
      }
      let (vendorless, vendor) = break_vendor(registry, extends);
      let (numberless, digits) = break_number(vendorless);
      let core_name = numberless.strip_suffix("FlagBits").unwrap();
      if output.contains_key(&(core_name, digits, vendor)) {
        panic!("{name} declared as offset of bitmask {extends}");
      }
    }
    for RequiredEnumValue { name, extends, value, comment, api } in
      requirement.required_value_enums.iter().cloned()
    {
      if !api_vulkan(api)
        || extends.is_empty()
        || !(extends.contains("Flags") || extends.contains("FlagBits"))
      {
        continue;
      }
      let (vendorless, vendor) = break_vendor(registry, extends);
      let (numberless, digits) = break_number(vendorless);
      let core_name = numberless.strip_suffix("FlagBits").unwrap();
      let flags = output.get_mut(&(core_name, digits, vendor)).unwrap();
      let mut value =
        ConstNewtypeValue { name, ty: extends, value: value.parse().unwrap(), comment };
      if value.name == "VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE" {
        value.comment = None;
      }
      match flags.values.entry(name) {
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

pub fn write_out_bitmasks<P: AsRef<Path>>(
  p: P, bitmasks: &BTreeMap<(StaticStr, Option<StaticStr>, Option<StaticStr>), Flags>,
) {
  let path = p.as_ref();
  std::fs::remove_file(path).unwrap();
  let mut f = OpenOptions::new().write(true).create(true).open(path).unwrap();
  writeln!(f, "#![allow(non_upper_case_globals)]").unwrap();
  writeln!(f).unwrap();
  for bitmask in bitmasks.values() {
    writeln!(f, "{bitmask}").unwrap();
  }
}
