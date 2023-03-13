use super::*;

#[derive(Debug, Clone, Default)]
pub struct ExtensionStrConst {
  pub name: StaticStr,
  pub value: StaticStr,
}
impl Display for ExtensionStrConst {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let Self { name, value } = self;
    writeln!(f, "pub const {name}: ZStr<'static> = ZStr::from_lit(\"{value}\\0\");")
  }
}

#[derive(Debug, Clone, Default)]
pub struct ExtensionU32Const {
  pub name: StaticStr,
  pub value: StaticStr,
}
impl Display for ExtensionU32Const {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let Self { name, value } = self;
    writeln!(f, "pub const {name}: u32 = {value};")
  }
}

#[derive(Debug, Clone)]
pub enum NameOrU32 {
  Str(ExtensionStrConst),
  U32(ExtensionU32Const),
}
impl Default for NameOrU32 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::Str(ExtensionStrConst::default())
  }
}
impl Display for NameOrU32 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Str(e) => Display::fmt(&e, f),
      Self::U32(s) => Display::fmt(&s, f),
    }
  }
}

pub fn gather_ext_consts(registry: &VulkanRegistry) -> BTreeMap<StaticStr, NameOrU32> {
  let mut output = BTreeMap::new();

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
    for RequiredEnumValue { name, extends, value, comment, api } in
      requirement.required_value_enums.iter().cloned()
    {
      if !api_vulkan(api) || !extends.is_empty() {
        continue;
      }
      assert!(comment.is_none());
      let ns = output.entry(name).or_insert(NameOrU32::default());
      if name.strip_suffix("_SPEC_VERSION").is_some() {
        *ns = NameOrU32::U32(ExtensionU32Const { name, value })
      } else if name.strip_suffix("_EXTENSION_NAME").is_some() {
        *ns = NameOrU32::Str(ExtensionStrConst {
          name,
          value: value.strip_suffix("&quot;").unwrap().strip_prefix("&quot;").unwrap(),
        })
      } else if name.strip_suffix("_NUMBER").is_some() {
        *ns = NameOrU32::U32(ExtensionU32Const { name, value })
      } else {
        panic!("{name}");
      }
    }
  }
  output
}

pub fn write_out_strings<P: AsRef<Path>>(
  p: P, name_or_specs: &BTreeMap<StaticStr, NameOrU32>,
) {
  let path = p.as_ref();
  std::fs::remove_file(path).unwrap();
  let mut f = OpenOptions::new().write(true).create(true).open(path).unwrap();
  writeln!(f, "#![allow(non_upper_case_globals)]").unwrap();
  writeln!(f).unwrap();
  writeln!(f, "use zstring::ZStr;").unwrap();
  writeln!(f).unwrap();
  for name_or_spec in name_or_specs.values() {
    writeln!(f, "{name_or_spec}").unwrap();
  }
}
