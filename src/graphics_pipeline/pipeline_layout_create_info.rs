use super::*;

/// Rusty [VkPipelineLayoutCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineLayoutCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineLayoutCreateFlags,
  set_layout_count: u32,
  set_layouts: *const VkDescriptorSetLayout,
  push_constant_range_count: u32,
  push_constant_ranges: *const VkPushConstantRange,
}
impl Default for PipelineLayoutCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      set_layout_count: Default::default(),
      set_layouts: core::ptr::null(),
      push_constant_range_count: Default::default(),
      push_constant_ranges: core::ptr::null(),
    }
  }
}
