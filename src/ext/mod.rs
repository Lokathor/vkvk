//! Extensions to the Vulkan API.

/// Khronos: [VK_KHR_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html) (instance extension)
#[allow(missing_docs)]
#[cfg(feature = "VK_KHR_surface")]
pub mod vk_khr_surface;

/// Khronos: [VK_KHR_swapchain](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html) (device extension)
#[allow(missing_docs)]
#[cfg(feature = "VK_KHR_swapchain")]
pub mod vk_khr_swapchain;

/// Makes a vulkan enumeration value, according to the [specification][vk].
///
/// [vk]: https://registry.khronos.org/vulkan/specs/1.3/styleguide.html#_assigning_extension_token_values
pub(crate) const fn extension_enumeration_value(ext: i32, offset: i32) -> i32 {
  const BASE_VALUE: i32 = 1000000000;
  const RANGE_SIZE: i32 = 1000;
  BASE_VALUE + (ext - 1) * RANGE_SIZE + offset
}
