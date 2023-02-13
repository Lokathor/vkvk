#![allow(nonstandard_style)]
use super::*;
/// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
pub type vkGetInstanceProcAddr_t =
  unsafe extern "system" fn(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
/// Khronos: [vkEnumerateInstanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html)
pub type vkEnumerateInstanceVersion_t =
  unsafe extern "system" fn(api_version: *mut VkVersion) -> VkResult;
/// Khronos: [vkEnumerateInstanceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
pub type vkEnumerateInstanceLayerProperties_t = unsafe extern "system" fn(
  property_count: *mut uint32_t,
  properties: *mut VkLayerProperties,
) -> VkResult;
/// Khronos: [vkEnumerateInstanceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
pub type vkEnumerateInstanceExtensionProperties_t = unsafe extern "system" fn(
  layer_name: *const u8,
  property_count: *mut uint32_t,
  properties: *mut VkExtensionProperties,
) -> VkResult;
