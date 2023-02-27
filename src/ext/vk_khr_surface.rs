#![allow(clippy::double_parens)]
#![allow(clippy::eq_op)]
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
#![allow(nonstandard_style)]
#![allow(unused_parens)]
#![allow(dead_code)]

use crate::prelude::*;

pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
pub const VK_KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface\0";
pub const VK_ERROR_SURFACE_LOST_KHR: VkResult =
  VkResult(core::num::NonZeroI32::new(-(1000000000 + (1 - 1) * 1000 + 0)));
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult =
  VkResult(core::num::NonZeroI32::new(-(1000000000 + (1 - 1) * 1000 + 1)));
pub const VK_OBJECT_TYPE_SURFACE_KHR: VkObjectType =
  VkObjectType((1000000000 + (1 - 1) * 1000 + 0));

define_non_dispatchable_handle!(
  /// Khronos: [VkSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html) (non-dispatchable handle)
  /// * Parent: [VkInstance]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_SURFACE_KHR`]
  VkSurfaceKHR
);

define_bitmask!(
  /// Khronos: [VkSurfaceTransformFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) (bitmask)
  VkSurfaceTransformFlagBitsKHR
);
/// Khronos: [VkSurfaceTransformFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) (bitmask)
pub type VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagBitsKHR;

pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1_u32 << 0);
pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1_u32 << 1);
pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1_u32 << 2);
pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1_u32 << 3);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1_u32 << 4);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1_u32 << 5);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1_u32 << 6);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1_u32 << 7);
pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1_u32 << 8);

define_enumeration!(
  /// Khronos: [VkPresentModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html) (enumeration)
  VkPresentModeKHR
);

pub const VK_PRESENT_MODE_IMMEDIATE_KHR: VkPresentModeKHR = VkPresentModeKHR(0);
pub const VK_PRESENT_MODE_MAILBOX_KHR: VkPresentModeKHR = VkPresentModeKHR(1);
pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = VkPresentModeKHR(2);
pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: VkPresentModeKHR = VkPresentModeKHR(3);

define_enumeration!(
  /// Khronos: [VkColorSpaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorSpaceKHR.html) (enumeration)
  VkColorSpaceKHR
);

pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VkColorSpaceKHR(0);
/// * Alias For [`VK_COLOR_SPACE_SRGB_NONLINEAR_KHR`]
#[deprecated = "aliased"]
pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;

define_bitmask!(
  /// Khronos: [VkCompositeAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html) (bitmask)
  VkCompositeAlphaFlagBitsKHR
);
/// Khronos: [VkCompositeAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html) (bitmask)
pub type VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagBitsKHR;

pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagBitsKHR =
  VkCompositeAlphaFlagBitsKHR(1_u32 << 0);
pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR =
  VkCompositeAlphaFlagBitsKHR(1_u32 << 1);
pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR =
  VkCompositeAlphaFlagBitsKHR(1_u32 << 2);
pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: VkCompositeAlphaFlagBitsKHR =
  VkCompositeAlphaFlagBitsKHR(1_u32 << 3);

/// Khronos: [VkSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
  /// Supported minimum number of images for the surface
  pub min_image_count: u32,
  /// Supported maximum number of images for the surface, 0 for unlimited
  pub max_image_count: u32,
  /// Current image width and height for the surface, (0, 0) if undefined
  pub current_extent: VkExtent2D,
  /// Supported minimum image width and height for the surface
  pub min_image_extent: VkExtent2D,
  /// Supported maximum image width and height for the surface
  pub max_image_extent: VkExtent2D,
  /// Supported maximum number of image layers for the surface
  pub max_image_array_layers: u32,
  /// 1 or more bits representing the transforms supported
  pub supported_transforms: VkSurfaceTransformFlagsKHR,
  /// The surface's current transform relative to the device's natural
  /// orientation
  pub current_transform: VkSurfaceTransformFlagBitsKHR,
  /// 1 or more bits representing the alpha compositing modes supported
  pub supported_composite_alpha: VkCompositeAlphaFlagsKHR,
  /// Supported image usage flags for the surface
  pub supported_usage_flags: VkImageUsageFlags,
}

/// Khronos: [VkSurfaceFormatKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceFormatKHR {
  /// Supported pair of rendering format
  pub format: VkFormat,
  /// and color space for the surface
  pub color_space: VkColorSpaceKHR,
}

/// Khronos: [vkDestroySurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html)
#[rustfmt::skip]
pub(crate) type vkDestroySurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  surface: VkSurfaceKHR,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkGetPhysicalDeviceSurfaceSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceSurfaceSupportKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  queue_family_index: u32,
  surface: VkSurfaceKHR,
  supported: *mut VkBool32,
);
/// Khronos: [vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceSurfaceCapabilitiesKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  surface: VkSurfaceKHR,
  surface_capabilities: *mut VkSurfaceCapabilitiesKHR,
);
/// Khronos: [vkGetPhysicalDeviceSurfaceFormatsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceSurfaceFormatsKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  surface: VkSurfaceKHR,
  surface_format_count: *mut u32,
  surface_formats: *mut VkSurfaceFormatKHR,
);
/// Khronos: [vkGetPhysicalDeviceSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceSurfacePresentModesKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  surface: VkSurfaceKHR,
  present_mode_count: *mut u32,
  present_modes: *mut VkPresentModeKHR,
);

/// Khronos: [vkDestroySurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html)
pub(crate) type PFN_vkDestroySurfaceKHR = Option<vkDestroySurfaceKHR_t>;
/// Khronos: [vkGetPhysicalDeviceSurfaceSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
pub(crate) type PFN_vkGetPhysicalDeviceSurfaceSupportKHR =
  Option<vkGetPhysicalDeviceSurfaceSupportKHR_t>;
/// Khronos: [vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
pub(crate) type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR =
  Option<vkGetPhysicalDeviceSurfaceCapabilitiesKHR_t>;
/// Khronos: [vkGetPhysicalDeviceSurfaceFormatsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
pub(crate) type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR =
  Option<vkGetPhysicalDeviceSurfaceFormatsKHR_t>;
/// Khronos: [vkGetPhysicalDeviceSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
pub(crate) type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR =
  Option<vkGetPhysicalDeviceSurfacePresentModesKHR_t>;
