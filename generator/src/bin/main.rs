use vk_dot_xml_parser::*;
use vkvk_generator::*;

const XML: &str = include_str!("../../vk.xml");

fn main() {
  let registry = VulkanRegistry::from_static_str(XML);
  let enumerations = gather_enumerations(&registry);
  write_out_enumerations("../src/generated/enumerations.rs", &enumerations);
}
