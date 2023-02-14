use super::*;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkImageFormatProperties {
  pub max_extent: VkExtent3D,
  pub max_mip_levels: uint32_t,
  pub max_array_layers: uint32_t,
  pub sample_counts: VkSampleCountFlags,
  pub max_resource_size: VkDeviceSize,
}
