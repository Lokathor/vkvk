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
) -> VkResult;
pub const vkEnumeratePhysicalDevices_NAME: &str = "vkEnumeratePhysicalDevices\0";

/// Khronos: [vkGetPhysicalDeviceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html)
pub type vkGetPhysicalDeviceProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  properties: *mut VkPhysicalDeviceProperties,
);
pub const vkGetPhysicalDeviceProperties_NAME: &str = "vkGetPhysicalDeviceProperties\0";

/// Khronos: [vkGetPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
pub type vkGetPhysicalDeviceFeatures_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  features: *mut VkPhysicalDeviceFeatures,
);
pub const vkGetPhysicalDeviceFeatures_NAME: &str = "vkGetPhysicalDeviceFeatures\0";

/// Khronos: [vkGetPhysicalDeviceQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
pub type vkGetPhysicalDeviceQueueFamilyProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  queue_family_property_count: *mut uint32_t,
  queue_family_properties: *mut VkQueueFamilyProperties,
);
pub const vkGetPhysicalDeviceQueueFamilyProperties_NAME: &str =
  "vkGetPhysicalDeviceQueueFamilyProperties\0";

/// Khronos: [vkCreateDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)
pub type vkCreateDevice_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  create_info: *const VkDeviceCreateInfo,
  allocator: *const VkAllocationCallbacks,
  device: *mut VkDevice,
) -> VkResult;
pub const vkCreateDevice_NAME: &str = "vkCreateDevice\0";

/// Khronos: [vkEnumerateDeviceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
pub type vkEnumerateDeviceExtensionProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  layer_name: *const u8,
  property_count: *mut uint32_t,
  properties: *mut VkExtensionProperties,
) -> VkResult;
pub const vkEnumerateDeviceExtensionProperties_NAME: &str =
  "vkEnumerateDeviceExtensionProperties\0";
