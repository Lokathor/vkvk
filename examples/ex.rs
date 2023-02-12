#![allow(unused_imports)]

use std::{mem::size_of, num::NonZeroI32};
use vkvk::{vkGetInstanceProcAddr, Entry, VkInstance};

fn main() {
  let entry = Entry::LINKED;
  println!("{:?}", entry.get_max_instance_version());
}
