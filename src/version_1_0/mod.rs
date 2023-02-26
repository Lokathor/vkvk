//! Data types for the 1.0 version of the API.

mod data_types;
pub use data_types::*;

pub(crate) mod fn_types;

#[cfg_attr(windows, link(name = "vulkan-1"))]
#[cfg_attr(not(windows), link(name = "vulkan"))]
#[allow(dead_code)]
extern "system" {
  /// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
  fn vkGetInstanceProcAddr(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
}
