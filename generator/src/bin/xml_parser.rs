#![allow(unused_mut)]

use magnesium::*;
use vkvk_generator::Registry;

const XML: &str = include_str!("../../vk.xml");

fn main() {
  let mut iter = ElementIterator::new(XML)
    .filter_map(skip_comments)
    .map(trim_text)
    .filter_map(skip_empty_text_elements);
  assert_eq!(iter.next().unwrap().unwrap_start_tag().0, "registry");
  let registry = Registry::from_iter(&mut iter);
  println!("# platforms: {}", registry.platforms.len());
  println!("# org_tags: {}", registry.vendor_tags.len());
  println!("# types: {}", registry.types.len());
  {
    let mut include = 0;
    let mut imported = 0;
    let mut define = 0;
    let mut base_type = 0;
    let mut bitmask = 0;
    let mut handle = 0;
    let mut enumeration = 0;
    let mut funcpointer = 0;
    let mut struct_def = 0;
    let mut union_def = 0;
    for ty in registry.types.iter() {
      match ty.category {
        "include" => include += 1,
        "" => imported += 1,
        "define" => define += 1,
        "basetype" => base_type += 1,
        "bitmask" => bitmask += 1,
        "handle" => handle += 1,
        "enum" => enumeration += 1,
        "funcpointer" => funcpointer += 1,
        "struct" => struct_def += 1,
        "union" => union_def += 1,
        _ => panic!("{ty:?}"),
      }
    }
    println!("include: {include}");
    println!("imported: {imported}");
    println!("define: {define}");
    println!("base_type: {base_type}");
    println!("bitmask: {bitmask}");
    println!("handle: {handle}");
    println!("enumeration: {enumeration}");
    println!("funcpointer: {funcpointer}");
    println!("struct_def: {struct_def}");
    println!("union_def: {union_def}");
  }
  println!("# enums: {}", registry.enums.len());
  println!("# commands: {}", registry.commands.len());
  println!("# features: {}", registry.features.len());
  println!("# extensions: {}", registry.extensions.len());
  println!("# formats: {}", registry.formats.len());
  println!("# spirv_extensions: {}", registry.spirv_extensions.len());
  println!("# spirv_capabilities: {}", registry.spirv_capabilities.len());
}
