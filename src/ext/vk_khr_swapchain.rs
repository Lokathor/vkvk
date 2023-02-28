#![allow(clippy::double_parens)]
#![allow(nonstandard_style)]
#![allow(unused_parens)]
#![allow(dead_code)]

use crate::prelude::*;

pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_swapchain\0";
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType =
  VkStructureType((extension_enumeration_value(2, 0)) as u32);
pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR: VkStructureType =
  VkStructureType((extension_enumeration_value(2, 1)) as u32);
pub const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR: VkImageLayout =
  VkImageLayout((extension_enumeration_value(2, 2)) as u32);
pub const VK_SUBOPTIMAL_KHR: VkResult =
  VkResult(core::num::NonZeroI32::new(extension_enumeration_value(2, 3)));
pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult =
  VkResult(core::num::NonZeroI32::new(-extension_enumeration_value(2, 4)));
pub const VK_OBJECT_TYPE_SWAPCHAIN_KHR: VkObjectType =
  VkObjectType((extension_enumeration_value(2, 0)) as u32);

define_bitmask!(
  /// Khronos: [VkSwapchainCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) (bitmask)
  VkSwapchainCreateFlagBitsKHR
);
/// Khronos: [VkSwapchainCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) (bitmask)
pub type VkSwapchainCreateFlagsKHR = VkSwapchainCreateFlagBitsKHR;

/// Khronos: [VkSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateInfoKHR.html)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
  /// * Values: [`VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkSwapchainCreateFlagsKHR,
  /// The swapchain's target surface
  pub surface: VkSurfaceKHR,
  /// Minimum number of presentation images the application needs
  pub min_image_count: u32,
  /// Format of the presentation images
  pub image_format: VkFormat,
  /// Colorspace of the presentation images
  pub image_color_space: VkColorSpaceKHR,
  /// Dimensions of the presentation images
  pub image_extent: VkExtent2D,
  /// Determines the number of views for multiview/stereo presentation
  pub image_array_layers: u32,
  /// Bits indicating how the presentation images will be used
  pub image_usage: VkImageUsageFlags,
  /// Sharing mode used for the presentation images
  pub image_sharing_mode: VkSharingMode,
  /// Number of queue families having access to the images in case of concurrent
  /// sharing mode
  /// * Optional
  pub queue_family_index_count: u32,
  /// Array of queue family indices having access to the images in case of
  /// concurrent sharing mode
  /// * Len: `queueFamilyIndexCount`
  /// * No Auto Validity
  pub queue_family_indices: *const u32,
  /// The transform, relative to the device's natural orientation, applied to
  /// the image content prior to presentation
  pub pre_transform: VkSurfaceTransformFlagBitsKHR,
  /// The alpha blending mode used when compositing this surface with other
  /// surfaces in the window system
  pub composite_alpha: VkCompositeAlphaFlagBitsKHR,
  /// Which presentation mode to use for presents on this swap chain
  pub present_mode: VkPresentModeKHR,
  /// Specifies whether presentable images may be affected by window clip
  /// regions
  pub clipped: VkBool32,
  /// Existing swap chain to replace, if any
  /// * Optional
  pub old_swapchain: VkSwapchainKHR,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkSwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainKHR.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_SWAPCHAIN_KHR`]
  VkSwapchainKHR
);

/// Khronos: [VkPresentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentInfoKHR.html)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VkPresentInfoKHR {
  /// * Values: [`VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Number of semaphores to wait for before presenting
  /// * Optional
  pub wait_semaphore_count: u32,
  /// Semaphores to wait for before presenting
  /// * Len: `waitSemaphoreCount`
  pub wait_semaphores: *const VkSemaphore,
  /// Number of swapchains to present in this call
  pub swapchain_count: u32,
  /// Swapchains to present an image from
  /// * Len: `swapchainCount`
  pub swapchains: *const VkSwapchainKHR,
  /// Indices of which presentable images to present
  /// * Len: `swapchainCount`
  pub image_indices: *const u32,
  /// Optional (i.e. if non-NULL) VkResult for each swapchain
  /// * Len: `swapchainCount`
  /// * Optional
  pub results: *mut VkResult,
}

/// Khronos: [vkCreateSwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html)
#[rustfmt::skip]
pub(crate) type vkCreateSwapchainKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkSwapchainCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  swapchain: *mut VkSwapchainKHR,
) -> VkResult;
/// Khronos: [vkDestroySwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html)
#[rustfmt::skip]
pub(crate) type vkDestroySwapchainKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkGetSwapchainImagesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html)
#[rustfmt::skip]
pub(crate) type vkGetSwapchainImagesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  swapchain_image_count: *mut u32,
  swapchain_images: *mut VkImage,
) -> VkResult;
/// Khronos: [vkAcquireNextImageKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html)
#[rustfmt::skip]
pub(crate) type vkAcquireNextImageKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  timeout: u64,
  semaphore: VkSemaphore,
  fence: VkFence,
  image_index: *mut u32,
) -> VkResult;
/// Khronos: [vkQueuePresentKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html)
#[rustfmt::skip]
pub(crate) type vkQueuePresentKHR_t = unsafe extern "system" fn(
  queue: VkQueue,
  present_info: *const VkPresentInfoKHR,
) -> VkResult;

/// Khronos: [vkCreateSwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html)
pub(crate) type PFN_vkCreateSwapchainKHR = Option<vkCreateSwapchainKHR_t>;
/// Khronos: [vkDestroySwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html)
pub(crate) type PFN_vkDestroySwapchainKHR = Option<vkDestroySwapchainKHR_t>;
/// Khronos: [vkGetSwapchainImagesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html)
pub(crate) type PFN_vkGetSwapchainImagesKHR = Option<vkGetSwapchainImagesKHR_t>;
/// Khronos: [vkAcquireNextImageKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html)
pub(crate) type PFN_vkAcquireNextImageKHR = Option<vkAcquireNextImageKHR_t>;
/// Khronos: [vkQueuePresentKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html)
pub(crate) type PFN_vkQueuePresentKHR = Option<vkQueuePresentKHR_t>;
pub const vkCreateSwapchainKHR_NAME: &str = "vkCreateSwapchainKHR\0";
pub const vkDestroySwapchainKHR_NAME: &str = "vkDestroySwapchainKHR\0";
pub const vkGetSwapchainImagesKHR_NAME: &str = "vkGetSwapchainImagesKHR\0";
pub const vkAcquireNextImageKHR_NAME: &str = "vkAcquireNextImageKHR\0";
pub const vkQueuePresentKHR_NAME: &str = "vkQueuePresentKHR\0";
