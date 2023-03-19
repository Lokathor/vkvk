use super::*;

/// Rusty [VkPipelineRasterizationStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[allow(missing_docs)]
pub struct PipelineRasterizationStateCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineRasterizationStateCreateFlags,
  pub depth_clamp_enable: VkBool32,
  pub rasterizer_discard_enable: VkBool32,
  pub polygon_mode: VkPolygonMode,
  pub cull_mode: VkCullModeFlags,
  pub front_face: VkFrontFace,
  pub depth_bias_enable: VkBool32,
  pub depth_bias_constant_factor: c_float,
  pub depth_bias_clamp: c_float,
  pub depth_bias_slope_factor: c_float,
  pub line_width: c_float,
}
impl Default for PipelineRasterizationStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      depth_clamp_enable: Default::default(),
      rasterizer_discard_enable: Default::default(),
      polygon_mode: Default::default(),
      cull_mode: Default::default(),
      front_face: Default::default(),
      depth_bias_enable: Default::default(),
      depth_bias_constant_factor: Default::default(),
      depth_bias_clamp: Default::default(),
      depth_bias_slope_factor: Default::default(),
      line_width: Default::default(),
    }
  }
}
