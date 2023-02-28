#![no_std]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

//! A library for interacting with the Vulkan graphical and compute API.

extern crate alloc;
extern crate std;

#[macro_use]
mod macros;

pub mod prelude;

pub mod version_1_0;

pub mod ext;

pub mod entry;

pub mod instance;

pub mod device;

impl Default for prelude::VkInstanceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      ty: prelude::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
      next: core::ptr::null(),
      flags: prelude::VkInstanceCreateFlags::none(),
      application_info: core::ptr::null(),
      enabled_layer_count: 0,
      enabled_layer_names: core::ptr::null(),
      enabled_extension_count: 0,
      enabled_extension_names: core::ptr::null(),
    }
  }
}
impl Default for prelude::VkApplicationInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      ty: prelude::VK_STRUCTURE_TYPE_APPLICATION_INFO,
      next: core::ptr::null(),
      application_name: core::ptr::null(),
      application_version: 0,
      engine_name: core::ptr::null(),
      engine_version: 0,
      api_version: prelude::VkVersion::API_1_0,
    }
  }
}
impl Default for prelude::VkClearColorValue {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { uint_32: [0_u32; 4] }
  }
}
impl Default for prelude::VkClearValue {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { color: prelude::VkClearColorValue::default() }
  }
}
impl core::fmt::Debug for prelude::VkClearValue {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkClearValue")
      .field("color", &unsafe { self.color })
      .field("depth_stencil", &unsafe { self.depth_stencil })
      .finish()
  }
}
impl core::fmt::Debug for prelude::VkClearColorValue {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkClearColorValue(union)")
      .field("float_32", &unsafe { self.float_32 })
      .field("int_32", &unsafe { self.int_32 })
      .field("uint_32", &unsafe { self.uint_32 })
      .finish()
  }
}
