use vk_dot_xml_parser::*;
use vkvk_generator::*;

const XML: &str = include_str!("../../vk.xml");

fn main() {
  let registry = VulkanRegistry::from_static_str(XML);
  //
  let enumerations = gather_enumerations(&registry);
  write_out_enumerations("../src/generated/enumerations.rs", &enumerations);
  //
  let bitmasks = gather_bitmasks(&registry);
  write_out_bitmasks("../src/generated/bitmasks.rs", &bitmasks);
  //
  let unions = gather_unions(&registry);
  write_out_unions("../src/generated/unions.rs", &unions);
  //
  let structures = gather_structures(&registry);
  write_out_structures("../src/generated/structures.rs", &structures);
  //
  let aliases = gather_aliases(&registry);
  write_out_aliases("../src/generated/aliases.rs", &aliases);
}
