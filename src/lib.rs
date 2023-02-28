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

impl core::fmt::Debug for prelude::VkLayerProperties {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkLayerProperties")
      .field("layer_name", &self.layer_name)
      .field("spec_version", &self.spec_version)
      .field("implementation_version", &self.implementation_version)
      .field("description", &self.description)
      .finish()
  }
}
impl core::fmt::Debug for prelude::VkExtensionProperties {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkExtensionProperties")
      .field("extension_name", &self.extension_name)
      .field("spec_version", &self.spec_version)
      .finish()
  }
}
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
