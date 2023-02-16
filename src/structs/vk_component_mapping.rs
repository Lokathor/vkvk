use super::*;

/// Khronos: [VkComponentMapping](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentMapping.html)
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct VkComponentMapping {
  pub r: VkComponentSwizzle,
  pub g: VkComponentSwizzle,
  pub b: VkComponentSwizzle,
  pub a: VkComponentSwizzle,
}
impl VkComponentMapping {
  pub const IDENTITY: Self = Self {
    r: VK_COMPONENT_SWIZZLE_IDENTITY,
    g: VK_COMPONENT_SWIZZLE_IDENTITY,
    b: VK_COMPONENT_SWIZZLE_IDENTITY,
    a: VK_COMPONENT_SWIZZLE_IDENTITY,
  };
}
