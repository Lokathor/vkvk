use super::*;

/// Rusty [VkPipelineMultisampleStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineMultisampleStateCreateFlags,
  rasterization_samples: VkSampleCountFlagBits,
  sample_shading_enable: VkBool32,
  min_sample_shading: c_float,
  sample_mask: *const VkSampleMask,
  alpha_to_coverage_enable: VkBool32,
  alpha_to_one_enable: VkBool32,
}
impl Default for PipelineMultisampleStateCreateInfo {
  /// Defaults to 1 sample
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      rasterization_samples: VK_SAMPLE_COUNT_1_BIT,
      sample_shading_enable: Default::default(),
      min_sample_shading: Default::default(),
      sample_mask: core::ptr::null(),
      alpha_to_coverage_enable: Default::default(),
      alpha_to_one_enable: Default::default(),
    }
  }
}
