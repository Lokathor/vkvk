use super::*;

/// Khronos: [VkImageSubresourceRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceRange.html)
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct VkImageSubresourceRange {
  pub aspect_mask: VkImageAspectFlags,
  pub base_mip_level: uint32_t,
  pub level_count: uint32_t,
  pub base_array_layer: uint32_t,
  pub layer_count: uint32_t,
}
