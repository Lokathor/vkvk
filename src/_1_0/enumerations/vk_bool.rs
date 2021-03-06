use super::*;

/// [VkBool32](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBool32.html)
///
/// The "truthy" type for Vulkan.
///
/// The canonical values are [`VK_FALSE`] and [`VK_TRUE`], though the type is
/// 32-bit, so *theoretically* any non-zero value is also a "true" value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkBool32(pub uint32_t);

impl From<bool> for VkBool32 {
  fn from(b: bool) -> Self {
    Self(b as _)
  }
}

impl From<VkBool32> for bool {
  fn from(vk_b: VkBool32) -> Self {
    vk_b.0 != 0
  }
}

pub const VK_FALSE: VkBool32 = VkBool32(0);

pub const VK_TRUE: VkBool32 = VkBool32(1);
