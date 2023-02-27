#![allow(unused_imports)]

//! Used as "scratch space" binary to check output stuff.

use core::fmt::Write;
use magnesium::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use vkvk_generator::{
  bitmasks::define_bitmask,
  enumeration_types::{define_enumeration, define_enums},
  format_type_and_variant,
  handle_types::{define_handle, define_non_dispatchable_handle},
  strip_number, strip_vendor,
  structs_unions::{define_structure, define_union},
  var_name,
  vk_dot_xml_parser::{
    Command, CommandParam, Enumeration, EnumerationEntry, Extension, Registry,
    RequireListEntry, RequiredEnum, StaticStr, TypeEntry, TypeVariant, VendorTag,
  },
  FUNC_PTR_DECLS,
};

const XML: &str = include_str!("../../vk.xml");

fn main() {
  let mut iter = ElementIterator::new(XML)
    .filter_map(skip_comments)
    .map(trim_text)
    .filter_map(skip_empty_text_elements);
  assert_eq!(iter.next().unwrap().unwrap_start_tag().0, "registry");
  #[allow(unused)]
  let registry = Registry::from_iter(&mut iter);
  //

  //print_v1_0_data(&registry);
  print_v1_0_constants(&registry);
  //print_v1_0_fn_types(&registry);

  //print_extension(&registry, "VK_KHR_surface");
  //print_extension(&registry, "VK_KHR_swapchain");
}

#[derive(Debug, Clone, Default)]
pub struct ApiCapability {
  pub data: BTreeSet<StaticStr>,
  pub commands: BTreeSet<StaticStr>,
  pub constants: BTreeSet<StaticStr>,
}

pub fn determine_vulkan_1_0(registry: &Registry) -> ApiCapability {
  let vendors: Vec<StaticStr> = registry.vendor_tags.iter().map(|v| v.name).collect();
  //
  let feat = registry.features.iter().find(|feat| feat.name == "VK_VERSION_1_0").unwrap();
  //
  let mut out = ApiCapability::default();
  //
  for required_ty in feat.required_types.iter() {
    let type_entry = registry.types.iter().find(|t| t.name == *required_ty).unwrap();
    match type_entry.category {
      Some("handle") | Some("bitmask") | Some("enum") | Some("struct")
      | Some("union") => (),
      _ => continue,
    }
    for member_ty in
      type_entry.members.iter().map(|member| member.ty).chain(Some(*required_ty))
    {
      let ts: Vec<String> = if member_ty.ends_with("Flags")
        | member_ty.ends_with("FlagBits")
      {
        let x =
          member_ty.strip_suffix("Flags").or(member_ty.strip_suffix("FlagBits")).unwrap();
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
    let command_entry =
      registry.commands.iter().find(|c| c.name == *required_command).unwrap();
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
      if ty.name == "VkResult" {
        return String::from("");
      }
      format_ty(ty, &vendors)
    })
    .collect();
  println!("use crate::prelude::*;");
  println!();
  for data_type_string in data_type_strings.iter() {
    println!("{data_type_string}");
  }
  println!();
  println!("{FUNC_PTR_DECLS}");
}

pub fn print_v1_0_fn_types(registry: &Registry) {
  use core::fmt::Write;
  let vk_1_0 = determine_vulkan_1_0(registry);
  let commands: Vec<Command> = vk_1_0
    .commands
    .iter()
    .map(|c| registry.commands.iter().find(|rc| rc.name == *c).unwrap().clone())
    .collect();
  //
  println!("#![allow(dead_code)]");
  println!("#![allow(nonstandard_style)]");
  println!();
  println!("use crate::prelude::*;");
  println!();
  //
  let mut commands_by_first_arg: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
  for command in commands.iter() {
    let key = match command.params[0].ty {
      "VkInstance" => "VkInstance",
      "VkPhysicalDevice" => "VkInstance_VkPhysicalDevice",
      "VkDevice" => "VkDevice",
      "VkQueue" => "VkDevice_VkQueue",
      "VkCommandBuffer" => "VkDevice_VkCommandBuffer",
      _ => "NoHandle",
    };
    commands_by_first_arg.entry(key).or_default().push(command.name);
  }
  // print comment tables of which commands go with what handle
  for (ty, command_list) in commands_by_first_arg.iter() {
    println!("// {ty}");
    println!("/*");
    for command in command_list.iter() {
      println!("{}", *command);
    }
    println!("*/");
    println!();
  }
  // just print out all the types
  for command in commands.iter() {
    print!("{}", format_command_t(command));
  }
  for command in commands.iter() {
    print!("{}", format_pfn_command(command.name));
  }
  for command in commands.iter() {
    let name = command.name;
    println!("pub(crate) const {name}_NAME: &str = \"{name}\\0\";");
  }
}

pub fn format_command_t(command: &Command) -> String {
  let mut t = String::new();
  let name = command.name;
  writeln!(t, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)").ok();
  writeln!(t, "#[rustfmt::skip]").ok();
  writeln!(t, "pub(crate) type {name}_t = unsafe extern \"system\" fn(").ok();
  for param in command.params.iter() {
    let arg = var_name(param.name);
    let arg_ty = format_type_and_variant(param.ty, param.ty_variant);
    writeln!(t, "  {arg}: {arg_ty},").ok();
  }
  let return_ty = format_type_and_variant(command.return_ty, TypeVariant::Normal);
  writeln!(t, ") -> {return_ty};").ok();
  t
}

pub fn format_pfn_command(name: &str) -> String {
  let mut pfn = String::new();
  writeln!(pfn,"/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html)").ok();
  writeln!(pfn, "pub(crate) type PFN_{name} = Option<{name}_t>;").ok();
  pfn
}

pub fn print_v1_0_constants(registry: &Registry) {
  let vk_1_0 = determine_vulkan_1_0(registry);
  let mut constant_strings = Vec::new();
  'name_loop: for name in vk_1_0.constants.iter() {
    for enumeration in registry.enums.iter() {
      for entry in enumeration.entries.iter() {
        if entry.name == *name && (entry.api.is_none() || entry.api == Some("vulkan")) {
          if enumeration.name == "API Constants" {
            continue 'name_loop;
          }
          constant_strings.push(format_enum_entry(enumeration, entry));
          continue 'name_loop;
        }
      }
    }
    panic!("Constant Definition Not Found: {name:?}");
  }
  println!("#![allow(nonstandard_style)]");
  println!();
  println!("use crate::prelude::*;");
  println!();
  for s in constant_strings.iter() {
    println!("{s}");
  }
}

fn format_enum_entry(
  Enumeration { name: rust_ty, ty: _, comment: _, bitwidth, entries: _ }: &Enumeration,
  entry @ EnumerationEntry {
    name: symbol_name,
    value,
    comment,
    ty: entry_ty,
    alias,
    bitpos,
    api,
    deprecated,
  }: &EnumerationEntry,
) -> String {
  let mut f = String::new();
  match api {
    None | Some("vulkan") => (),
    _ => panic!("illegal: {entry:?}"),
  }
  if let Some(comment) = comment {
    writeln!(f, "/// {comment}").ok();
  }
  match (value, bitpos) {
    (Some(value), None) => {
      assert!(entry_ty.is_none(), "{entry:?}");
      if let Some(alias) = alias {
        writeln!(f, "/// * Alias For [`{alias}`]").ok();
        if let Some(deprecated) = deprecated {
          writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
        }
        writeln!(f, "pub const {symbol_name}: {rust_ty} = {alias};").ok();
      } else if *rust_ty == "VkResult" {
        if let Some(deprecated) = deprecated {
          writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
        }
        writeln!(
          f,
          "pub const {symbol_name}: {rust_ty} = {rust_ty}(NonZeroI32::new({value}));"
        )
        .ok();
      } else {
        if let Some(deprecated) = deprecated {
          writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
        }
        writeln!(f, "pub const {symbol_name}: {rust_ty} = {rust_ty}({value});").ok();
      };
    }
    (None, Some(bitpos)) => {
      assert!(entry_ty.is_none(), "{entry:?}");
      assert!(value.is_none(), "{entry:?}");
      let bitwidth = bitwidth.unwrap_or("32");
      if let Some(deprecated) = deprecated {
        writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
      }
      writeln!(
        f,
        "pub const {symbol_name}: {rust_ty} = {rust_ty}(1_u{bitwidth} << {bitpos});"
      )
      .ok();
    }
    (None, None) => {
      let alias = alias.unwrap();
      writeln!(f, "/// * Alias For [`{alias}`]").ok();
      if let Some(deprecated) = deprecated {
        writeln!(f, "#[deprecated = \"{deprecated}\"]").ok();
      }
      writeln!(f, "pub const {symbol_name}: {rust_ty} = {alias};").ok();
    }
    _ => panic!("{entry:?}"),
  }
  f
}

pub fn print_extension(registry: &Registry, name: &str) {
  let vendors: Vec<StaticStr> = registry.vendor_tags.iter().map(|v| v.name).collect();
  let Extension {
    name: _,
    requires,
    comment: _,
    number,
    ty: _,
    author: _,
    contact: _,
    supported,
    platform: _,
    special_use: _,
    deprecated_by: _,
    promoted_to: _,
    obsoleted_by: _,
    provisional: _,
    sort_order: _,
    depends: _,
    require_lists,
  } = registry.extensions.iter().find(|ex| ex.name == name).unwrap();
  assert!(requires.is_none() || *requires == Some("vulkan"));
  assert!(supported.unwrap().split(',').any(|s| s == "vulkan"));
  //
  println!("#![allow(clippy::double_parens)]");
  println!("#![allow(nonstandard_style)]");
  println!("#![allow(unused_parens)]");
  println!("#![allow(dead_code)]");
  println!();
  println!("use crate::prelude::*;");
  println!();
  for RequireListEntry { enums, types, commands, depends, api, comment: _ } in
    require_lists.iter()
  {
    match api {
      None | Some("vulkan") => (),
      _ => continue,
    }
    match depends {
      None => (),
      Some("VK_VERSION_1_1") => continue,
      other => panic!("{other:?}"),
    }
    //
    for req_enum @ RequiredEnum {
      name,
      value,
      offset,
      extends,
      dir,
      extnumber,
      bitpos,
      comment,
      alias,
      deprecated,
      api,
      protect,
    } in enums.iter()
    {
      match api {
        None | Some("vulkan") => (),
        _ => continue,
      }
      if let Some(protect) = protect {
        println!("/* Skipping `{name}`, protected by {protect} */");
        continue;
      }
      //
      if let Some(comment) = comment {
        println!("/// {comment}");
      }
      if let Some(deprecated) = deprecated {
        println!("#[deprecated = \"{deprecated}\"]");
      }
      if let Some(alias) = alias {
        println!("/* ALIAS: {alias} is an alias for {name} */");
      } else if let Some(extends) = extends {
        let extnumber = extnumber.or(*number).unwrap();
        if let Some(offset) = offset {
          let dir = dir.unwrap_or("");
          let vk_result_prefix =
            if *extends == "VkResult" { "core::num::NonZeroI32::new" } else { "" };
          let vk_result_suffix = if *extends == "VkResult" { "" } else { " as u32" };
          println!(
            "pub const {name}: {extends} = {extends}({vk_result_prefix}({dir}extension_enumeration_value({extnumber},{offset})){vk_result_suffix});"
          );
        } else if let Some(bitpos) = bitpos {
          println!("pub const {name}: {extends} = {extends}(1_u32 << {bitpos});");
        } else {
          panic!("{req_enum:?}");
        };
      } else if let Some(value) = value {
        if value.contains('&') {
          let value = value.strip_prefix("&quot;").unwrap();
          let value = value.strip_suffix("&quot;").unwrap();
          println!("pub const {name}: &str = \"{value}\\0\";");
        } else {
          println!("pub const {name}: u32 = {value};");
        }
      } else {
        panic!("{req_enum:?}");
      }
    }
    println!();
    for ty_str in types.iter() {
      if ty_str.contains("Flags") {
        continue;
      }
      let mut ty = registry.types.iter().find(|t| t.name == *ty_str).unwrap().clone();
      if ty_str.contains("Flags") || ty_str.contains("FlagBits") {
        ty.category = Some("bitmask");
      }
      println!("{}", format_ty(&ty, &vendors));
      for enumeration in registry.enums.iter().filter(|e| e.name == *ty_str) {
        println!("{}", define_enums(enumeration));
      }
    }
    println!();
    for command_str in commands.iter() {
      let command = registry.commands.iter().find(|c| c.name == *command_str).unwrap();
      print!("{}", format_command_t(command));
    }
    println!();
    for command_str in commands.iter() {
      print!("{}", format_pfn_command(command_str));
    }
    for command_str in commands.iter() {
      println!("pub const {command_str}_NAME: &str = \"{command_str}\\0\";");
    }
    println!();
  }
}
