#![allow(clippy::double_parens)]
#![allow(nonstandard_style)]
#![allow(unused_parens)]
#![allow(dead_code)]

use crate::prelude::*;

pub const VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION: u32 = 1;
pub const VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME: &str =
  "VK_KHR_portability_enumeration\0";
pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR: VkInstanceCreateFlagBits =
  VkInstanceCreateFlagBits(1_u32 << 0);
