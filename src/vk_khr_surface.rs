use crate::*;

/// Khronos: [VkSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html) (non-dispatchable handle)
/// * Parent: [VkInstance]
/// * Object Type Enum: [`VK_OBJECT_TYPE_SURFACE_KHR`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSurfaceKHR(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkSurfaceKHR {}
unsafe impl Sync for VkSurfaceKHR {}
impl VkSurfaceKHR {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    #[cfg(target_pointer_width = "64")]
    return Self(core::ptr::null_mut());
    #[cfg(not(target_pointer_width = "64"))]
    return Self(0);
  }
}
impl Default for VkSurfaceKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}

pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
pub const VK_KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface\0";

/// Khronos: [VkSurfaceTransformFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSurfaceTransformFlagBitsKHR(pub u32);
impl VkSurfaceTransformFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkSurfaceTransformFlagBitsKHR);

/// Khronos: [VkSurfaceTransformFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html)
pub type VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagBitsKHR;

/// Khronos: [VkPresentModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPresentModeKHR(pub u32);

/// Khronos: [VkColorSpaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorSpaceKHR.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkColorSpaceKHR(pub u32);

/// Khronos: [VkCompositeAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkCompositeAlphaFlagBitsKHR(pub u32);
impl VkCompositeAlphaFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkCompositeAlphaFlagBitsKHR);

/// Khronos: [VkCompositeAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html)
pub type VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagBitsKHR;

/// Khronos: [VkSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
  /// Supported minimum number of images for the surface
  pub min_image_count: uint32_t,
  /// Supported maximum number of images for the surface, 0 for unlimited
  pub max_image_count: uint32_t,
  /// Current image width and height for the surface, (0, 0) if undefined
  pub current_extent: VkExtent2D,
  /// Supported minimum image width and height for the surface
  pub min_image_extent: VkExtent2D,
  /// Supported maximum image width and height for the surface
  pub max_image_extent: VkExtent2D,
  /// Supported maximum number of image layers for the surface
  pub max_image_array_layers: uint32_t,
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

/*

<enum offset="0" extends="VkResult" dir="-"                     name="VK_ERROR_SURFACE_LOST_KHR"/>
<enum offset="1" extends="VkResult" dir="-"                     name="VK_ERROR_NATIVE_WINDOW_IN_USE_KHR"/>
<enum offset="0" extends="VkObjectType"                         name="VK_OBJECT_TYPE_SURFACE_KHR"/>
<command name="vkDestroySurfaceKHR"/>
<command name="vkGetPhysicalDeviceSurfaceSupportKHR"/>
<command name="vkGetPhysicalDeviceSurfaceCapabilitiesKHR"/>
<command name="vkGetPhysicalDeviceSurfaceFormatsKHR"/>
<command name="vkGetPhysicalDeviceSurfacePresentModesKHR"/>

*/
