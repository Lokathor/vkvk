use super::*;

#[derive(Debug, Clone, Default)]
pub struct RustAlias {
  pub name: StaticStr,
  pub alias_of: StaticStr,
}
impl Display for RustAlias {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let RustAlias { name, alias_of } = self;
    writeln!(f, "/// (Alias) Khronos: [{alias_of}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{alias_of}.html)")?;
    writeln!(f, "pub type {name} = {alias_of};")?;
    Ok(())
  }
}

pub fn gather_aliases(registry: &VulkanRegistry) -> BTreeMap<StaticStr, RustAlias> {
  let mut output = BTreeMap::default();

  for TypeAlias { name, alias_of, category: _ } in registry.type_aliases.iter() {
    *output.entry(*name).or_insert(RustAlias::default()) = RustAlias { name, alias_of };
  }

  output
}

pub fn write_out_aliases<P: AsRef<Path>>(p: P, aliases: &BTreeMap<StaticStr, RustAlias>) {
  let path = p.as_ref();
  std::fs::remove_file(path).unwrap();
  let mut f = OpenOptions::new().write(true).create(true).open(path).unwrap();
  writeln!(f, "#![allow(non_upper_case_globals)]").unwrap();
  writeln!(f).unwrap();
  writeln!(f, "use crate::prelude::*;").unwrap();
  writeln!(f).unwrap();
  for alias in aliases.values() {
    if BLOCKED_TYPES.contains(&alias.alias_of) {
      continue;
    }
    writeln!(f, "{alias}").unwrap();
  }
}
