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
    #[allow(deprecated)]
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
impl core::fmt::Debug for prelude::VkFormat {
  #[allow(non_upper_case_globals)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    use prelude::*;
    match *self {
      VK_FORMAT_UNDEFINED => write!(f, "VK_FORMAT_UNDEFINED"),
      VK_FORMAT_R4G4_UNORM_PACK8 => write!(f, "VK_FORMAT_R4G4_UNORM_PACK8"),
      VK_FORMAT_R4G4B4A4_UNORM_PACK16 => write!(f, "VK_FORMAT_R4G4B4A4_UNORM_PACK16"),
      VK_FORMAT_B4G4R4A4_UNORM_PACK16 => write!(f, "VK_FORMAT_B4G4R4A4_UNORM_PACK16"),
      VK_FORMAT_R5G6B5_UNORM_PACK16 => write!(f, "VK_FORMAT_R5G6B5_UNORM_PACK16"),
      VK_FORMAT_B5G6R5_UNORM_PACK16 => write!(f, "VK_FORMAT_B5G6R5_UNORM_PACK16"),
      VK_FORMAT_R5G5B5A1_UNORM_PACK16 => write!(f, "VK_FORMAT_R5G5B5A1_UNORM_PACK16"),
      VK_FORMAT_B5G5R5A1_UNORM_PACK16 => write!(f, "VK_FORMAT_B5G5R5A1_UNORM_PACK16"),
      VK_FORMAT_A1R5G5B5_UNORM_PACK16 => write!(f, "VK_FORMAT_A1R5G5B5_UNORM_PACK16"),
      VK_FORMAT_R8_UNORM => write!(f, "VK_FORMAT_R8_UNORM"),
      VK_FORMAT_R8_SNORM => write!(f, "VK_FORMAT_R8_SNORM"),
      VK_FORMAT_R8_USCALED => write!(f, "VK_FORMAT_R8_USCALED"),
      VK_FORMAT_R8_SSCALED => write!(f, "VK_FORMAT_R8_SSCALED"),
      VK_FORMAT_R8_UINT => write!(f, "VK_FORMAT_R8_UINT"),
      VK_FORMAT_R8_SINT => write!(f, "VK_FORMAT_R8_SINT"),
      VK_FORMAT_R8_SRGB => write!(f, "VK_FORMAT_R8_SRGB"),
      VK_FORMAT_R8G8_UNORM => write!(f, "VK_FORMAT_R8G8_UNORM"),
      VK_FORMAT_R8G8_SNORM => write!(f, "VK_FORMAT_R8G8_SNORM"),
      VK_FORMAT_R8G8_USCALED => write!(f, "VK_FORMAT_R8G8_USCALED"),
      VK_FORMAT_R8G8_SSCALED => write!(f, "VK_FORMAT_R8G8_SSCALED"),
      VK_FORMAT_R8G8_UINT => write!(f, "VK_FORMAT_R8G8_UINT"),
      VK_FORMAT_R8G8_SINT => write!(f, "VK_FORMAT_R8G8_SINT"),
      VK_FORMAT_R8G8_SRGB => write!(f, "VK_FORMAT_R8G8_SRGB"),
      VK_FORMAT_R8G8B8_UNORM => write!(f, "VK_FORMAT_R8G8B8_UNORM"),
      VK_FORMAT_R8G8B8_SNORM => write!(f, "VK_FORMAT_R8G8B8_SNORM"),
      VK_FORMAT_R8G8B8_USCALED => write!(f, "VK_FORMAT_R8G8B8_USCALED"),
      VK_FORMAT_R8G8B8_SSCALED => write!(f, "VK_FORMAT_R8G8B8_SSCALED"),
      VK_FORMAT_R8G8B8_UINT => write!(f, "VK_FORMAT_R8G8B8_UINT"),
      VK_FORMAT_R8G8B8_SINT => write!(f, "VK_FORMAT_R8G8B8_SINT"),
      VK_FORMAT_R8G8B8_SRGB => write!(f, "VK_FORMAT_R8G8B8_SRGB"),
      VK_FORMAT_B8G8R8_UNORM => write!(f, "VK_FORMAT_B8G8R8_UNORM"),
      VK_FORMAT_B8G8R8_SNORM => write!(f, "VK_FORMAT_B8G8R8_SNORM"),
      VK_FORMAT_B8G8R8_USCALED => write!(f, "VK_FORMAT_B8G8R8_USCALED"),
      VK_FORMAT_B8G8R8_SSCALED => write!(f, "VK_FORMAT_B8G8R8_SSCALED"),
      VK_FORMAT_B8G8R8_UINT => write!(f, "VK_FORMAT_B8G8R8_UINT"),
      VK_FORMAT_B8G8R8_SINT => write!(f, "VK_FORMAT_B8G8R8_SINT"),
      VK_FORMAT_B8G8R8_SRGB => write!(f, "VK_FORMAT_B8G8R8_SRGB"),
      VK_FORMAT_R8G8B8A8_UNORM => write!(f, "VK_FORMAT_R8G8B8A8_UNORM"),
      VK_FORMAT_R8G8B8A8_SNORM => write!(f, "VK_FORMAT_R8G8B8A8_SNORM"),
      VK_FORMAT_R8G8B8A8_USCALED => write!(f, "VK_FORMAT_R8G8B8A8_USCALED"),
      VK_FORMAT_R8G8B8A8_SSCALED => write!(f, "VK_FORMAT_R8G8B8A8_SSCALED"),
      VK_FORMAT_R8G8B8A8_UINT => write!(f, "VK_FORMAT_R8G8B8A8_UINT"),
      VK_FORMAT_R8G8B8A8_SINT => write!(f, "VK_FORMAT_R8G8B8A8_SINT"),
      VK_FORMAT_R8G8B8A8_SRGB => write!(f, "VK_FORMAT_R8G8B8A8_SRGB"),
      VK_FORMAT_B8G8R8A8_UNORM => write!(f, "VK_FORMAT_B8G8R8A8_UNORM"),
      VK_FORMAT_B8G8R8A8_SNORM => write!(f, "VK_FORMAT_B8G8R8A8_SNORM"),
      VK_FORMAT_B8G8R8A8_USCALED => write!(f, "VK_FORMAT_B8G8R8A8_USCALED"),
      VK_FORMAT_B8G8R8A8_SSCALED => write!(f, "VK_FORMAT_B8G8R8A8_SSCALED"),
      VK_FORMAT_B8G8R8A8_UINT => write!(f, "VK_FORMAT_B8G8R8A8_UINT"),
      VK_FORMAT_B8G8R8A8_SINT => write!(f, "VK_FORMAT_B8G8R8A8_SINT"),
      VK_FORMAT_B8G8R8A8_SRGB => write!(f, "VK_FORMAT_B8G8R8A8_SRGB"),
      VK_FORMAT_A8B8G8R8_UNORM_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_UNORM_PACK32"),
      VK_FORMAT_A8B8G8R8_SNORM_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_SNORM_PACK32"),
      VK_FORMAT_A8B8G8R8_USCALED_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_USCALED_PACK32"),
      VK_FORMAT_A8B8G8R8_SSCALED_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_SSCALED_PACK32"),
      VK_FORMAT_A8B8G8R8_UINT_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_UINT_PACK32"),
      VK_FORMAT_A8B8G8R8_SINT_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_SINT_PACK32"),
      VK_FORMAT_A8B8G8R8_SRGB_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_SRGB_PACK32"),
      VK_FORMAT_A2R10G10B10_UNORM_PACK32 => {
        write!(f, "VK_FORMAT_A2R10G10B10_UNORM_PACK32")
      }
      VK_FORMAT_A2R10G10B10_SNORM_PACK32 => {
        write!(f, "VK_FORMAT_A2R10G10B10_SNORM_PACK32")
      }
      VK_FORMAT_A2R10G10B10_USCALED_PACK32 => {
        write!(f, "VK_FORMAT_A2R10G10B10_USCALED_PACK32")
      }
      VK_FORMAT_A2R10G10B10_SSCALED_PACK32 => {
        write!(f, "VK_FORMAT_A2R10G10B10_SSCALED_PACK32")
      }
      VK_FORMAT_A2R10G10B10_UINT_PACK32 => write!(f, "VK_FORMAT_A2R10G10B10_UINT_PACK32"),
      VK_FORMAT_A2R10G10B10_SINT_PACK32 => write!(f, "VK_FORMAT_A2R10G10B10_SINT_PACK32"),
      VK_FORMAT_A2B10G10R10_UNORM_PACK32 => {
        write!(f, "VK_FORMAT_A2B10G10R10_UNORM_PACK32")
      }
      VK_FORMAT_A2B10G10R10_SNORM_PACK32 => {
        write!(f, "VK_FORMAT_A2B10G10R10_SNORM_PACK32")
      }
      VK_FORMAT_A2B10G10R10_USCALED_PACK32 => {
        write!(f, "VK_FORMAT_A2B10G10R10_USCALED_PACK32")
      }
      VK_FORMAT_A2B10G10R10_SSCALED_PACK32 => {
        write!(f, "VK_FORMAT_A2B10G10R10_SSCALED_PACK32")
      }
      VK_FORMAT_A2B10G10R10_UINT_PACK32 => write!(f, "VK_FORMAT_A2B10G10R10_UINT_PACK32"),
      VK_FORMAT_A2B10G10R10_SINT_PACK32 => write!(f, "VK_FORMAT_A2B10G10R10_SINT_PACK32"),
      VK_FORMAT_R16_UNORM => write!(f, "VK_FORMAT_R16_UNORM"),
      VK_FORMAT_R16_SNORM => write!(f, "VK_FORMAT_R16_SNORM"),
      VK_FORMAT_R16_USCALED => write!(f, "VK_FORMAT_R16_USCALED"),
      VK_FORMAT_R16_SSCALED => write!(f, "VK_FORMAT_R16_SSCALED"),
      VK_FORMAT_R16_UINT => write!(f, "VK_FORMAT_R16_UINT"),
      VK_FORMAT_R16_SINT => write!(f, "VK_FORMAT_R16_SINT"),
      VK_FORMAT_R16_SFLOAT => write!(f, "VK_FORMAT_R16_SFLOAT"),
      VK_FORMAT_R16G16_UNORM => write!(f, "VK_FORMAT_R16G16_UNORM"),
      VK_FORMAT_R16G16_SNORM => write!(f, "VK_FORMAT_R16G16_SNORM"),
      VK_FORMAT_R16G16_USCALED => write!(f, "VK_FORMAT_R16G16_USCALED"),
      VK_FORMAT_R16G16_SSCALED => write!(f, "VK_FORMAT_R16G16_SSCALED"),
      VK_FORMAT_R16G16_UINT => write!(f, "VK_FORMAT_R16G16_UINT"),
      VK_FORMAT_R16G16_SINT => write!(f, "VK_FORMAT_R16G16_SINT"),
      VK_FORMAT_R16G16_SFLOAT => write!(f, "VK_FORMAT_R16G16_SFLOAT"),
      VK_FORMAT_R16G16B16_UNORM => write!(f, "VK_FORMAT_R16G16B16_UNORM"),
      VK_FORMAT_R16G16B16_SNORM => write!(f, "VK_FORMAT_R16G16B16_SNORM"),
      VK_FORMAT_R16G16B16_USCALED => write!(f, "VK_FORMAT_R16G16B16_USCALED"),
      VK_FORMAT_R16G16B16_SSCALED => write!(f, "VK_FORMAT_R16G16B16_SSCALED"),
      VK_FORMAT_R16G16B16_UINT => write!(f, "VK_FORMAT_R16G16B16_UINT"),
      VK_FORMAT_R16G16B16_SINT => write!(f, "VK_FORMAT_R16G16B16_SINT"),
      VK_FORMAT_R16G16B16_SFLOAT => write!(f, "VK_FORMAT_R16G16B16_SFLOAT"),
      VK_FORMAT_R16G16B16A16_UNORM => write!(f, "VK_FORMAT_R16G16B16A16_UNORM"),
      VK_FORMAT_R16G16B16A16_SNORM => write!(f, "VK_FORMAT_R16G16B16A16_SNORM"),
      VK_FORMAT_R16G16B16A16_USCALED => write!(f, "VK_FORMAT_R16G16B16A16_USCALED"),
      VK_FORMAT_R16G16B16A16_SSCALED => write!(f, "VK_FORMAT_R16G16B16A16_SSCALED"),
      VK_FORMAT_R16G16B16A16_UINT => write!(f, "VK_FORMAT_R16G16B16A16_UINT"),
      VK_FORMAT_R16G16B16A16_SINT => write!(f, "VK_FORMAT_R16G16B16A16_SINT"),
      VK_FORMAT_R16G16B16A16_SFLOAT => write!(f, "VK_FORMAT_R16G16B16A16_SFLOAT"),
      VK_FORMAT_R32_UINT => write!(f, "VK_FORMAT_R32_UINT"),
      VK_FORMAT_R32_SINT => write!(f, "VK_FORMAT_R32_SINT"),
      VK_FORMAT_R32_SFLOAT => write!(f, "VK_FORMAT_R32_SFLOAT"),
      VK_FORMAT_R32G32_UINT => write!(f, "VK_FORMAT_R32G32_UINT"),
      VK_FORMAT_R32G32_SINT => write!(f, "VK_FORMAT_R32G32_SINT"),
      VK_FORMAT_R32G32_SFLOAT => write!(f, "VK_FORMAT_R32G32_SFLOAT"),
      VK_FORMAT_R32G32B32_UINT => write!(f, "VK_FORMAT_R32G32B32_UINT"),
      VK_FORMAT_R32G32B32_SINT => write!(f, "VK_FORMAT_R32G32B32_SINT"),
      VK_FORMAT_R32G32B32_SFLOAT => write!(f, "VK_FORMAT_R32G32B32_SFLOAT"),
      VK_FORMAT_R32G32B32A32_UINT => write!(f, "VK_FORMAT_R32G32B32A32_UINT"),
      VK_FORMAT_R32G32B32A32_SINT => write!(f, "VK_FORMAT_R32G32B32A32_SINT"),
      VK_FORMAT_R32G32B32A32_SFLOAT => write!(f, "VK_FORMAT_R32G32B32A32_SFLOAT"),
      VK_FORMAT_R64_UINT => write!(f, "VK_FORMAT_R64_UINT"),
      VK_FORMAT_R64_SINT => write!(f, "VK_FORMAT_R64_SINT"),
      VK_FORMAT_R64_SFLOAT => write!(f, "VK_FORMAT_R64_SFLOAT"),
      VK_FORMAT_R64G64_UINT => write!(f, "VK_FORMAT_R64G64_UINT"),
      VK_FORMAT_R64G64_SINT => write!(f, "VK_FORMAT_R64G64_SINT"),
      VK_FORMAT_R64G64_SFLOAT => write!(f, "VK_FORMAT_R64G64_SFLOAT"),
      VK_FORMAT_R64G64B64_UINT => write!(f, "VK_FORMAT_R64G64B64_UINT"),
      VK_FORMAT_R64G64B64_SINT => write!(f, "VK_FORMAT_R64G64B64_SINT"),
      VK_FORMAT_R64G64B64_SFLOAT => write!(f, "VK_FORMAT_R64G64B64_SFLOAT"),
      VK_FORMAT_R64G64B64A64_UINT => write!(f, "VK_FORMAT_R64G64B64A64_UINT"),
      VK_FORMAT_R64G64B64A64_SINT => write!(f, "VK_FORMAT_R64G64B64A64_SINT"),
      VK_FORMAT_R64G64B64A64_SFLOAT => write!(f, "VK_FORMAT_R64G64B64A64_SFLOAT"),
      VK_FORMAT_B10G11R11_UFLOAT_PACK32 => write!(f, "VK_FORMAT_B10G11R11_UFLOAT_PACK32"),
      VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => write!(f, "VK_FORMAT_E5B9G9R9_UFLOAT_PACK32"),
      VK_FORMAT_D16_UNORM => write!(f, "VK_FORMAT_D16_UNORM"),
      VK_FORMAT_X8_D24_UNORM_PACK32 => write!(f, "VK_FORMAT_X8_D24_UNORM_PACK32"),
      VK_FORMAT_D32_SFLOAT => write!(f, "VK_FORMAT_D32_SFLOAT"),
      VK_FORMAT_S8_UINT => write!(f, "VK_FORMAT_S8_UINT"),
      VK_FORMAT_D16_UNORM_S8_UINT => write!(f, "VK_FORMAT_D16_UNORM_S8_UINT"),
      VK_FORMAT_D24_UNORM_S8_UINT => write!(f, "VK_FORMAT_D24_UNORM_S8_UINT"),
      VK_FORMAT_D32_SFLOAT_S8_UINT => write!(f, "VK_FORMAT_D32_SFLOAT_S8_UINT"),
      VK_FORMAT_BC1_RGB_UNORM_BLOCK => write!(f, "VK_FORMAT_BC1_RGB_UNORM_BLOCK"),
      VK_FORMAT_BC1_RGB_SRGB_BLOCK => write!(f, "VK_FORMAT_BC1_RGB_SRGB_BLOCK"),
      VK_FORMAT_BC1_RGBA_UNORM_BLOCK => write!(f, "VK_FORMAT_BC1_RGBA_UNORM_BLOCK"),
      VK_FORMAT_BC1_RGBA_SRGB_BLOCK => write!(f, "VK_FORMAT_BC1_RGBA_SRGB_BLOCK"),
      VK_FORMAT_BC2_UNORM_BLOCK => write!(f, "VK_FORMAT_BC2_UNORM_BLOCK"),
      VK_FORMAT_BC2_SRGB_BLOCK => write!(f, "VK_FORMAT_BC2_SRGB_BLOCK"),
      VK_FORMAT_BC3_UNORM_BLOCK => write!(f, "VK_FORMAT_BC3_UNORM_BLOCK"),
      VK_FORMAT_BC3_SRGB_BLOCK => write!(f, "VK_FORMAT_BC3_SRGB_BLOCK"),
      VK_FORMAT_BC4_UNORM_BLOCK => write!(f, "VK_FORMAT_BC4_UNORM_BLOCK"),
      VK_FORMAT_BC4_SNORM_BLOCK => write!(f, "VK_FORMAT_BC4_SNORM_BLOCK"),
      VK_FORMAT_BC5_UNORM_BLOCK => write!(f, "VK_FORMAT_BC5_UNORM_BLOCK"),
      VK_FORMAT_BC5_SNORM_BLOCK => write!(f, "VK_FORMAT_BC5_SNORM_BLOCK"),
      VK_FORMAT_BC6H_UFLOAT_BLOCK => write!(f, "VK_FORMAT_BC6H_UFLOAT_BLOCK"),
      VK_FORMAT_BC6H_SFLOAT_BLOCK => write!(f, "VK_FORMAT_BC6H_SFLOAT_BLOCK"),
      VK_FORMAT_BC7_UNORM_BLOCK => write!(f, "VK_FORMAT_BC7_UNORM_BLOCK"),
      VK_FORMAT_BC7_SRGB_BLOCK => write!(f, "VK_FORMAT_BC7_SRGB_BLOCK"),
      VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK => write!(f, "VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK"),
      VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK => write!(f, "VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK"),
      VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK => {
        write!(f, "VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK")
      }
      VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK => {
        write!(f, "VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK")
      }
      VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK => {
        write!(f, "VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK")
      }
      VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK => {
        write!(f, "VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK")
      }
      VK_FORMAT_EAC_R11_UNORM_BLOCK => write!(f, "VK_FORMAT_EAC_R11_UNORM_BLOCK"),
      VK_FORMAT_EAC_R11_SNORM_BLOCK => write!(f, "VK_FORMAT_EAC_R11_SNORM_BLOCK"),
      VK_FORMAT_EAC_R11G11_UNORM_BLOCK => write!(f, "VK_FORMAT_EAC_R11G11_UNORM_BLOCK"),
      VK_FORMAT_EAC_R11G11_SNORM_BLOCK => write!(f, "VK_FORMAT_EAC_R11G11_SNORM_BLOCK"),
      VK_FORMAT_ASTC_4x4_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_4x4_UNORM_BLOCK"),
      VK_FORMAT_ASTC_4x4_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_4x4_SRGB_BLOCK"),
      VK_FORMAT_ASTC_5x4_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_5x4_UNORM_BLOCK"),
      VK_FORMAT_ASTC_5x4_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_5x4_SRGB_BLOCK"),
      VK_FORMAT_ASTC_5x5_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_5x5_UNORM_BLOCK"),
      VK_FORMAT_ASTC_5x5_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_5x5_SRGB_BLOCK"),
      VK_FORMAT_ASTC_6x5_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_6x5_UNORM_BLOCK"),
      VK_FORMAT_ASTC_6x5_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_6x5_SRGB_BLOCK"),
      VK_FORMAT_ASTC_6x6_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_6x6_UNORM_BLOCK"),
      VK_FORMAT_ASTC_6x6_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_6x6_SRGB_BLOCK"),
      VK_FORMAT_ASTC_8x5_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_8x5_UNORM_BLOCK"),
      VK_FORMAT_ASTC_8x5_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_8x5_SRGB_BLOCK"),
      VK_FORMAT_ASTC_8x6_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_8x6_UNORM_BLOCK"),
      VK_FORMAT_ASTC_8x6_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_8x6_SRGB_BLOCK"),
      VK_FORMAT_ASTC_8x8_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_8x8_UNORM_BLOCK"),
      VK_FORMAT_ASTC_8x8_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_8x8_SRGB_BLOCK"),
      VK_FORMAT_ASTC_10x5_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_10x5_UNORM_BLOCK"),
      VK_FORMAT_ASTC_10x5_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_10x5_SRGB_BLOCK"),
      VK_FORMAT_ASTC_10x6_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_10x6_UNORM_BLOCK"),
      VK_FORMAT_ASTC_10x6_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_10x6_SRGB_BLOCK"),
      VK_FORMAT_ASTC_10x8_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_10x8_UNORM_BLOCK"),
      VK_FORMAT_ASTC_10x8_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_10x8_SRGB_BLOCK"),
      VK_FORMAT_ASTC_10x10_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_10x10_UNORM_BLOCK"),
      VK_FORMAT_ASTC_10x10_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_10x10_SRGB_BLOCK"),
      VK_FORMAT_ASTC_12x10_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_12x10_UNORM_BLOCK"),
      VK_FORMAT_ASTC_12x10_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_12x10_SRGB_BLOCK"),
      VK_FORMAT_ASTC_12x12_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_12x12_UNORM_BLOCK"),
      VK_FORMAT_ASTC_12x12_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_12x12_SRGB_BLOCK"),
      other => write!(f, "VkFormat({})", other.0),
    }
  }
}
