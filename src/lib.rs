#![no_std]
#![allow(unused_imports)]

#[macro_use]
mod macros;

pub mod api_constants;
pub mod base_types;
pub mod prelude;
pub mod vk_version;

pub mod version_1_0;

/* VK_KHR_surface

pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
pub const VK_KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface\0";
pub const VK_ERROR_SURFACE_LOST_KHR: VkResult = VkResult(NonZeroI32::new(-1000000000));
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = VkResult(NonZeroI32::new(-1000000001));
pub const VK_OBJECT_TYPE_SURFACE_KHR: VkObjectType = VkObjectType(NonZeroI32::new(1000000000));

define_non_dispatchable_handle!(VkSurfaceKHR);

define_bitmask!(VkSurfaceTransformFlagBitsKHR);
pub type VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagBitsKHR;

define_enumeration!(VkPresentModeKHR);

define_enumeration!(VkColorSpaceKHR);

define_bitmask!(VkCompositeAlphaFlagBitsKHR);
pub type VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagBitsKHR;

pub struct VkSurfaceCapabilitiesKHR {
  // TODO
}

pub struct VkSurfaceFormatKHR {
  // TODO
}

// vkDestroySurfaceKHR (?)
// vkGetPhysicalDeviceSurfaceSupportKHR (?)
// vkGetPhysicalDeviceSurfaceCapabilitiesKHR (?)
// vkGetPhysicalDeviceSurfaceFormatsKHR (?)
// vkGetPhysicalDeviceSurfacePresentModesKHR (?)

<extension name="VK_KHR_surface" number="1" type="instance" author="KHR" contact="" supported="vulkan,vulkansc">
  <require>
    <enum value="25" name="VK_KHR_SURFACE_SPEC_VERSION"/>
    <enum value="&quot;VK_KHR_surface&quot;" name="VK_KHR_SURFACE_EXTENSION_NAME"/>
    <enum offset="0" extends="VkResult" dir="-" name="VK_ERROR_SURFACE_LOST_KHR"/>
    <enum offset="1" extends="VkResult" dir="-" name="VK_ERROR_NATIVE_WINDOW_IN_USE_KHR"/>
    <enum offset="0" extends="VkObjectType" name="VK_OBJECT_TYPE_SURFACE_KHR"/>
    <type name="VkSurfaceKHR"/>
    <type name="VkSurfaceTransformFlagBitsKHR"/>
    <type name="VkPresentModeKHR"/>
    <type name="VkColorSpaceKHR"/>
    <type name="VkCompositeAlphaFlagBitsKHR"/>
    <type name="VkCompositeAlphaFlagsKHR"/>
    <type name="VkSurfaceCapabilitiesKHR"/>
    <type name="VkSurfaceFormatKHR"/>
    <command name="vkDestroySurfaceKHR"/>
    <command name="vkGetPhysicalDeviceSurfaceSupportKHR"/>
    <command name="vkGetPhysicalDeviceSurfaceCapabilitiesKHR"/>
    <command name="vkGetPhysicalDeviceSurfaceFormatsKHR"/>
    <command name="vkGetPhysicalDeviceSurfacePresentModesKHR"/>
  </require>
</extension>

*/

/* VK_KHR_swapchain

pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_swapchain\0";
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = VkStructureType(1000001000);
pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR: VkStructureType = VkStructureType(1000001001);
pub const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR: VkImageLayout = VkImageLayout(1000001002);
pub const VK_SUBOPTIMAL_KHR: VkResult = VkResult(1000001003);
pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult = VkResult(-1000001004);
pub const VK_OBJECT_TYPE_SWAPCHAIN_KHR: VkObjectType = VkObjectType(1000001000);

define_bitmask!(VkSwapchainCreateFlagBitsKHR);
pub type VkSwapchainCreateFlagsKHR = VkSwapchainCreateFlagBitsKHR;

pub struct VkSwapchainCreateInfoKHR {
  // TODO
}

define_non_dispatchable_handle!(VkSwapchainKHR);

pub struct VkPresentInfoKHR {
  // TODO
}

// vkCreateSwapchainKHR (device)
// vkDestroySwapchainKHR (device)
// vkGetSwapchainImagesKHR (device)
// vkAcquireNextImageKHR (device)
// vkQueuePresentKHR (queue)

// Adds more in 1.1!

*/
