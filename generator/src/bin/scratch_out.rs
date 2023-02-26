#![allow(unused_imports)]

//! Used as "scratch space" binary to check output stuff.

use magnesium::*;
use std::collections::{BTreeSet, HashSet};
use vkvk_generator::{
  bitmasks::define_bitmask,
  enumeration_types::{define_enumeration, define_enums},
  format_type_and_variant,
  handle_types::{define_handle, define_non_dispatchable_handle},
  strip_number, strip_vendor,
  structs_unions::{define_structure, define_union},
  var_name,
  vk_dot_xml_parser::{Command, CommandParam, Registry, StaticStr, TypeEntry, VendorTag},
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
  print_v1_0_fns(&registry);
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

pub fn format_ty(ty: &TypeEntry, vendors: &[&str]) -> String {
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

pub fn print_v1_0_data(registry: &Registry) {
  let vk_1_0 = determine_vulkan_1_0(registry);
  let vendors: Vec<StaticStr> = registry.vendor_tags.iter().map(|v| v.name).collect();
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
  println!();
  for data_type_string in data_type_strings.iter() {
    println!("{data_type_string}");
  }
  println!();
  println!("{FUNC_PTR_DECLS}");
}

fn print_v1_0_fns(registry: &Registry) {
  use core::fmt::Write;
  let vk_1_0 = determine_vulkan_1_0(registry);
  let commands: Vec<Command> = vk_1_0
    .commands
    .iter()
    .map(|c| registry.commands.iter().find(|rc| rc.name == *c).unwrap().clone())
    .collect();
  let mut global_commands = vec![];
  let mut instance_commands = vec![];
  let mut device_commands = vec![];
  for command in commands {
    if ["VkDevice", "VkQueue", "VkCommandBuffer"].contains(&command.params[0].ty) {
      device_commands.push(command.clone());
    } else if ["VkInstance", "VkPhysicalDevice"].contains(&command.params[0].ty) {
      instance_commands.push(command.clone());
    } else {
      global_commands.push(command.clone());
    }
  }
  println!("/* global commands: {} */", global_commands.len());
  println!("/* instance commands: {} */", instance_commands.len());
  println!("/* device commands: {} */", device_commands.len());
  println!();
  for command_list in [global_commands, instance_commands, device_commands] {
    let mut t = String::new();
    let mut pfn = String::new();
    for command in command_list.iter() {
      let name = command.name;
      writeln!(t, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)").ok();
      let mut notes = String::new();
      for CommandParam {
        name,
        ty_variant: _,
        ty: _,
        optional,
        extern_sync,
        len,
        alt_len,
        no_auto_validity,
        stride,
        object_type,
        valid_structs,
        api,
      } in command.params.iter()
      {
        let mut inner_notes: Vec<String> = Vec::new();
        match optional {
          None => (),
          Some("true") => inner_notes.push(String::from("Optional")),
          Some("false") => inner_notes.push(String::from("Required")),
          Some("false,true") => {
            inner_notes.push(String::from("Non-null pointer (but can point at 0)"))
          }
          other => panic!("{other:?}"),
        }
        if !inner_notes.is_empty() {
          let arg = var_name(name);
          write!(notes, "/// * `{arg}`: ").ok();
          for (i, inner) in inner_notes.iter().enumerate() {
            if i > 0 {
              write!(notes, ", ").ok();
            }
            write!(notes, "{inner}").ok();
          }
          writeln!(notes).ok();
        }
      }
      if !notes.is_empty() {
        writeln!(t, "/// ").ok();
        writeln!(t, "/// # Parameter Notes").ok();
        write!(t, "{notes}").ok();
      }
      writeln!(t, "#[rustfmt::skip]").ok();
      writeln!(t, "pub(crate) type {name}_t = unsafe extern \"system\" fn(").ok();
      for param in command.params.iter() {
        let arg = var_name(param.name);
        let arg_ty = format_type_and_variant(param.ty, param.ty_variant);
        writeln!(t, "  {arg}: {arg_ty},").ok();
      }
      writeln!(t, ");").ok();
      //
      writeln!(pfn,"/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)").ok();
      writeln!(pfn, "pub(crate) type PFN_{name} = Option<{name}_t>;").ok();
    }
    println!("{t}");
    //println!();
    println!("{pfn}");
  }
  //
}
