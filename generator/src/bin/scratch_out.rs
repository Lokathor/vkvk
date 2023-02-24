#![allow(unused)]

//! Used as "scratch space" binary to check output stuff.

use magnesium::*;
use vkvk_generator::{
  bitmasks::define_bitmask,
  enumeration_types::{define_enumeration, define_enums},
  handle_types::{define_handle, define_non_dispatchable_handle},
  structures::define_structure,
  vk_dot_xml_parser::{Registry, StaticStr},
};

const XML: &str = include_str!("../../vk.xml");

fn main() {
  let mut iter = ElementIterator::new(XML)
    .filter_map(skip_comments)
    .map(trim_text)
    .filter_map(skip_empty_text_elements);
  assert_eq!(iter.next().unwrap().unwrap_start_tag().0, "registry");
  let registry = Registry::from_iter(&mut iter);
  if false {
    println!(
      "Registry {{
      platforms: {platforms_len},
      vendor_tags: {vendor_tags_len},
      types: {types_len},
      enums: {enums_len},
      commands: {commands_len},
      features: {features_len},
      extensions: {extensions_len},
      formats: {formats_len},
      spirv_extensions: {spirv_extensions_len},
      spirv_capabilities: {spirv_capabilities_len},
    }}
    ",
      platforms_len = registry.platforms.len(),
      vendor_tags_len = registry.vendor_tags.len(),
      types_len = registry.types.len(),
      enums_len = registry.enums.len(),
      commands_len = registry.commands.len(),
      features_len = registry.features.len(),
      extensions_len = registry.extensions.len(),
      formats_len = registry.formats.len(),
      spirv_extensions_len = registry.spirv_extensions.len(),
      spirv_capabilities_len = registry.spirv_capabilities.len(),
    );
  }
  //
  let vendors: Vec<StaticStr> = registry.vendor_tags.iter().map(|v| v.name).collect();
  //
  if true {
    let mut handle = 0;
    let mut bitmask = 0;
    let mut enumeration = 0;
    let mut structure = 0;
    for ty in registry.types.iter() {
      match ty.api {
        None | Some("vulkan") => (),
        _ => continue,
      }
      //
      match ty.category {
        Some("handle") => {
          handle += 1;
          let out = match ty.ty_src {
            Some("VK_DEFINE_HANDLE") => {
              define_handle(ty.name, ty.parent, ty.obj_type_enum.unwrap())
            }
            Some("VK_DEFINE_NON_DISPATCHABLE_HANDLE") => {
              define_non_dispatchable_handle(ty.name, ty.parent, ty.obj_type_enum.unwrap())
            }
            None if ty.is_alias_for.is_some() => {
              let name = ty.name;
              let alias = ty.is_alias_for.unwrap();
              format!("pub type {name} = {alias};")
            }
            _ => panic!("{ty:?}"),
          };
          println!("{out}");
          println!();
        }
        Some("bitmask") => {
          bitmask += 1;
          println!("{}", define_bitmask(ty, &vendors));
          println!();
        }
        Some("enum") => {
          enumeration += 1;
          println!("{}", define_enumeration(ty));
          println!();
        }
        Some("struct") => {
          structure += 1;
          println!("{}", define_structure(ty));
          println!();
        }
        _ => (),
      }
    }
    println!("//bitmask count: {bitmask}");
    println!("//enumeration count: {enumeration}");
    println!("//structure count: {structure}");
    println!();
  }

  if true {
    for enums in registry.enums.iter() {
      if enums.name == "API Constants" {
        continue;
      }
      println!("{}", define_enums(enums));
    }
  }

  if false {
    for command in registry.commands.iter() {
      if let Some(param) = command.params.get(0) {
        println!("{} // {}", param.ty, command.name);
      } else {
        println!("// {}", command.name);
      }
    }
  }
}

// TODO: define the structs

// TODO: define the fns

// TODO: use feature/extension info to apply cfgs?

// TODO: make more of the info be Optional!
