#![allow(unused_imports)]

use std::{mem::size_of, num::NonZeroI32};
use vkvk::{CreateRequest, Entry, VkInstance, VkInstanceCreateFlagBits, VkVersion};

fn main() {
  let entry = Entry::LINKED;
  println!("get_max_instance_version: {:?}", entry.get_max_instance_version());
  println!("get_instance_layer_properties: {:?}", entry.get_available_layers());
  println!("get_instance_extension_properties(None): {:?}", entry.get_available_extensions(None));
  println!(
    "create_instance: {:?}",
    entry.create_instance(CreateRequest {
      api_version: VkVersion::major_minor_patch(1, 1, 0),
      application_name: String::from("demo"),
      engine_name: String::from("demo engine"),
      application_version: 1,
      engine_version: 1,
      enabled_extensions: Vec::new(),
      enabled_layers: Vec::new(),
      flags: VkInstanceCreateFlagBits::default(),
    })
  );
}
