#![allow(unused_imports)]

use std::{mem::size_of, num::NonZeroI32};
use vkvk::{vkGetInstanceProcAddr, Entry, VkInstance};

fn main() {
  let entry = Entry::LINKED;
  println!("get_max_instance_version: {:?}", entry.get_max_instance_version());
  println!("get_instance_layer_properties: {:?}", entry.get_instance_layer_properties());
}
