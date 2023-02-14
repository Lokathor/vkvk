#![allow(clippy::missing_inline_in_public_items)]

pub mod vk_structure_type;
pub use vk_structure_type::*;

pub mod vk_color_space_khr;
pub use vk_color_space_khr::*;

pub mod vk_format;
pub use vk_format::*;

pub mod vk_present_mode_khr;
pub use vk_present_mode_khr::*;

pub mod vk_sharing_mode;
pub use vk_sharing_mode::*;

pub mod vk_image_type;
pub use vk_image_type::*;

pub mod vk_image_tiling;
pub use vk_image_tiling::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSystemAllocationScope(u32);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkInternalAllocationType(u32);

/// Khronos: [VkPhysicalDeviceType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceType.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPhysicalDeviceType(u32);
impl core::fmt::Debug for VkPhysicalDeviceType {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      VK_PHYSICAL_DEVICE_TYPE_OTHER => write!(f, "VK_PHYSICAL_DEVICE_TYPE_OTHER"),
      VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => write!(f, "VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU"),
      VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => write!(f, "VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU"),
      VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => write!(f, "VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU"),
      VK_PHYSICAL_DEVICE_TYPE_CPU => write!(f, "VK_PHYSICAL_DEVICE_TYPE_CPU"),
      other => write!(f, "VkPhysicalDeviceType({})", other.0),
    }
  }
}
pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = VkPhysicalDeviceType(0);
pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(1);
pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(2);
pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(3);
pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = VkPhysicalDeviceType(4);
