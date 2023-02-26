#![allow(unused)]

//! Used as "scratch space" binary to check output stuff.

use magnesium::*;
use std::collections::{BTreeSet, HashSet};
use vkvk_generator::{
  bitmasks::define_bitmask,
  enumeration_types::{define_enumeration, define_enums},
  handle_types::{define_handle, define_non_dispatchable_handle},
  strip_number, strip_vendor,
  structs_unions::{define_structure, define_union},
  vk_dot_xml_parser::{Registry, StaticStr, TypeEntry, VendorTag},
  FUNC_PTR_DECLS,
};

const XML: &str = include_str!("../../vk.xml");

fn main() {
  let mut iter = ElementIterator::new(XML)
    .filter_map(skip_comments)
    .map(trim_text)
    .filter_map(skip_empty_text_elements);
  assert_eq!(iter.next().unwrap().unwrap_start_tag().0, "registry");
  let registry = Registry::from_iter(&mut iter);
  //
  let vendors: Vec<StaticStr> = registry.vendor_tags.iter().map(|v| v.name).collect();
  //
  let vk_1_0 = determine_vulkan_1_0(&registry);
  let data_type_strings: Vec<String> = vk_1_0
    .data
    .iter()
    .map(|ty_name| {
      let ty = registry.types.iter().find(|ty_entry| ty_entry.name == *ty_name).unwrap();
      format_ty(ty, &vendors)
    })
    .collect();
  println!("use crate::api_constants::*;");
  println!("use crate::base_types::*;");
  println!("use crate::vk_version::*;");
  for data_type_string in data_type_strings.iter() {
    println!("{data_type_string}");
  }
  println!();
  println!("{FUNC_PTR_DECLS}");
}

#[derive(Debug, Clone, Default)]
struct ApiCapability {
  data: BTreeSet<StaticStr>,
  commands: BTreeSet<StaticStr>,
  constants: BTreeSet<StaticStr>,
}

fn determine_vulkan_1_0(registry: &Registry) -> ApiCapability {
  let vendors: Vec<StaticStr> = registry.vendor_tags.iter().map(|v| v.name).collect();
  //
  let feat = registry.features.iter().find(|feat| feat.name == "VK_VERSION_1_0").unwrap();
  //
  let mut out = ApiCapability::default();
  //
  for required_ty in feat.required_types.iter() {
    let type_entry = registry.types.iter().find(|t| t.name == *required_ty).unwrap();
    match type_entry.category {
      Some("handle") | Some("bitmask") | Some("enum") | Some("struct") | Some("union") => (),
      _ => continue,
    }
    for member_ty in type_entry.members.iter().map(|member| member.ty).chain(Some(*required_ty)) {
      let ts: Vec<String> = if member_ty.ends_with("Flags") | member_ty.ends_with("FlagBits") {
        let x = member_ty.strip_suffix("Flags").or(member_ty.strip_suffix("FlagBits")).unwrap();
        vec![format!("{x}Flags"), format!("{x}FlagBits")]
      } else {
        vec![String::from(member_ty)]
      };
      for t in ts.into_iter() {
        if out.data.insert(match t.as_str() {
          "u8" | "u32" | "i32" | "u64" | "usize" | "c_void" | "c_float" => continue,
          other if other.starts_with("PFN_") => continue,
          other => {
            if out.data.contains(other) {
              continue;
            } else {
              Box::leak(t.clone().into_boxed_str())
            }
          }
        }) {
          for enumeration in registry.enums.iter().filter(|e| e.name == t) {
            for entry in enumeration.entries.iter() {
              out.constants.insert(entry.name);
            }
          }
        }
      }
    }
  }
  for required_command in feat.required_commands.iter() {
    let command_entry = registry.commands.iter().find(|c| c.name == *required_command).unwrap();
    for param in command_entry.params.iter() {
      out.data.insert(match param.ty {
        "u8" | "u32" | "i32" | "u64" | "usize" | "c_void" | "c_float" => continue,
        other if other.starts_with("PFN_") => continue,
        other => other,
      });
    }
    out.commands.insert(command_entry.name);
  }
  out.data.retain(|d| !strip_number(strip_vendor(d, &vendors).0).0.ends_with("Flags"));
  for required_enums in feat.required_enums.iter() {
    out.constants.insert(required_enums.name);
  }
  //
  out
}

fn format_ty(ty: &TypeEntry, vendors: &[&str]) -> String {
  match ty.category {
    Some("handle") => match ty.ty_src {
      Some("VK_DEFINE_HANDLE") => define_handle(ty.name, ty.parent, ty.obj_type_enum.unwrap()),
      Some("VK_DEFINE_NON_DISPATCHABLE_HANDLE") => {
        define_non_dispatchable_handle(ty.name, ty.parent, ty.obj_type_enum.unwrap())
      }
      None if ty.is_alias_for.is_some() => {
        let name = ty.name;
        let alias = ty.is_alias_for.unwrap();
        format!("pub type {name} = {alias};")
      }
      _ => panic!("{ty:?}"),
    },
    Some("bitmask") => define_bitmask(ty, vendors),
    Some("enum") => define_enumeration(ty),
    Some("struct") => define_structure(ty),
    Some("union") => define_union(ty),
    Some("basetype") => String::new(),
    other => panic!("{}: {other:?}", ty.name),
  }
}
