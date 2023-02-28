#![no_std]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::unit_arg)]
#![warn(missing_docs)]

//! A library for interacting with the Vulkan graphical and compute API.
//!
//! ## Naming
//!
//! Some types in this library wrap a "raw" vulkan value with "extra stuff". The
//! details of the stuff vary, but the naming scheme is always the same: When a
//! raw Vulkan type is wrapped the "rusty" type will be the same name with the
//! leading `Vk` stripped. So an [`Instance`] wraps a [`VkInstance`] with extra
//! stuff, a [`PhysicalDevice`] wraps a [`VkPhysicalDevice`] with extra stuff,
//! and so on.

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
      flags: prelude::VkInstanceCreateFlags::default(),
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
impl Default for prelude::VkDeviceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      ty: prelude::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
      next: core::ptr::null(),
      flags: prelude::VkDeviceCreateFlags::default(),
      queue_create_info_count: 0,
      queue_create_infos: core::ptr::null(),
      enabled_layer_count: 0,
      enabled_layer_names: core::ptr::null(),
      enabled_extension_count: 0,
      enabled_extension_names: core::ptr::null(),
      enabled_features: core::ptr::null(),
    }
  }
}
impl Default for prelude::VkDeviceQueueCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      ty: prelude::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
      next: core::ptr::null(),
      flags: prelude::VkDeviceQueueCreateFlagBits::default(),
      queue_family_index: 0,
      queue_count: 0,
      queue_priorities: core::ptr::null(),
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
#[cfg(feature = "VK_KHR_swapchain")]
impl Default for prelude::VkSwapchainCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      ty: prelude::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: prelude::VkSwapchainCreateFlagBitsKHR::default(),
      surface: prelude::VkSurfaceKHR::NULL,
      min_image_count: 0,
      image_format: prelude::VkFormat::default(),
      image_color_space: prelude::VkColorSpaceKHR::default(),
      image_extent: prelude::VkExtent2D::default(),
      image_array_layers: 0,
      image_usage: prelude::VkImageUsageFlagBits::default(),
      image_sharing_mode: prelude::VkSharingMode::default(),
      queue_family_index_count: 0,
      queue_family_indices: core::ptr::null(),
      pre_transform: prelude::VkSurfaceTransformFlagBitsKHR::default(),
      composite_alpha: prelude::VkCompositeAlphaFlagBitsKHR::default(),
      present_mode: prelude::VkPresentModeKHR::default(),
      clipped: prelude::VkBool32::FALSE,
      old_swapchain: prelude::VkSwapchainKHR::NULL,
    }
  }
}
