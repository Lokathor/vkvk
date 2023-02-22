#![allow(unused)]

//! Used as "scratch space" binary to check output stuff.

use magnesium::*;
use vkvk_generator::{
  bitmasks::define_bitmask,
  enumeration_types::define_enumeration,
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
  //
  let vendors: Vec<StaticStr> = registry.vendor_tags.iter().map(|v| v.name).collect();
  //
  for ty in registry.types.iter() {
    match ty.api {
      None | Some("vulkan") => (),
      _ => continue,
    }
    //
    match ty.category {
      Some("bitmask") if false => {
        println!("{}", define_bitmask(ty, &vendors));
        println!();
      }
      Some("enum") if false => {
        println!("{}", define_enumeration(ty));
        println!();
      }
      Some("struct") => {
        println!("{}", define_structure(ty));
        println!();
      }
      _ => (),
    }
  }
}

// TODO: define the structs

// TODO: define the fns

// TODO: use feature/extension info to apply cfgs?

// TODO: make more of the info be Optional!
