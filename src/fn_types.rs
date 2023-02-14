#![allow(nonstandard_style)]
use super::*;
/// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
pub type vkGetInstanceProcAddr_t =
  unsafe extern "system" fn(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
pub const vkGetInstanceProcAddr_NAME: &str = "vkGetInstanceProcAddr\0";
/// Khronos: [vkEnumerateInstanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html)
pub type vkEnumerateInstanceVersion_t =
  unsafe extern "system" fn(api_version: *mut VkVersion) -> VkResult;
pub const vkEnumerateInstanceVersion_NAME: &str = "vkEnumerateInstanceVersion\0";
/// Khronos: [vkEnumerateInstanceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
pub type vkEnumerateInstanceLayerProperties_t = unsafe extern "system" fn(
  property_count: *mut uint32_t,
  properties: *mut VkLayerProperties,
) -> VkResult;
pub const vkEnumerateInstanceLayerProperties_NAME: &str = "vkEnumerateInstanceLayerProperties\0";
/// Khronos: [vkEnumerateInstanceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
pub type vkEnumerateInstanceExtensionProperties_t = unsafe extern "system" fn(
  layer_name: *const u8,
  property_count: *mut uint32_t,
  properties: *mut VkExtensionProperties,
) -> VkResult;
pub const vkEnumerateInstanceExtensionProperties_NAME: &str =
  "vkEnumerateInstanceExtensionProperties\0";
/// Khronos: [vkCreateInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)
pub type vkCreateInstance_t = unsafe extern "system" fn(
  create_info: *const VkInstanceCreateInfo,
  allocator: *const VkAllocationCallbacks,
  instance: *mut VkInstance,
) -> VkResult;
pub const vkCreateInstance_NAME: &str = "vkCreateInstance\0";
/// Khronos: [vkDestroyInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)
pub type vkDestroyInstance_t =
  unsafe extern "system" fn(instance: VkInstance, allocator: *const VkAllocationCallbacks);
pub const vkDestroyInstance_NAME: &str = "vkDestroyInstance\0";
/// Khronos: [vkEnumeratePhysicalDevices](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)
pub type vkEnumeratePhysicalDevices_t = unsafe extern "system" fn(
  instance: VkInstance,
  physical_device_count: *mut uint32_t,
  physical_devices: *mut VkPhysicalDevice,
);
pub const vkEnumeratePhysicalDevices_NAME: &str = "vkEnumeratePhysicalDevices\0";
