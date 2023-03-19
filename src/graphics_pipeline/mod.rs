//! Structures to create and work with a graphics pipeline.

use crate::prelude::*;

mod pipeline_shader_stage_create_info;
pub use pipeline_shader_stage_create_info::*;

mod pipeline_dynamic_state_create_info;
pub use pipeline_dynamic_state_create_info::*;

// TODO: handle VkPipelineShaderStageCreateInfo

/// Rusty version of [VkGraphicsPipelineCreateInfo]
#[derive(Clone, Debug)]
#[repr(C)]
pub struct GraphicsPipelineCreateInfo<'m> {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineCreateFlags,
  stage_count: u32,
  stages: *const PipelineShaderStageCreateInfo<'m>,
  vertex_input_state: Option<Box<VkPipelineVertexInputStateCreateInfo>>,
  input_assembly_state: Option<Box<VkPipelineInputAssemblyStateCreateInfo>>,
  tessellation_state: Option<Box<VkPipelineTessellationStateCreateInfo>>,
  viewport_state: Option<Box<VkPipelineViewportStateCreateInfo>>,
  rasterization_state: Option<Box<VkPipelineRasterizationStateCreateInfo>>,
  multisample_state: Option<Box<VkPipelineMultisampleStateCreateInfo>>,
  depth_stencil_state: Option<Box<VkPipelineDepthStencilStateCreateInfo>>,
  color_blend_state: Option<Box<VkPipelineColorBlendStateCreateInfo>>,
  dynamic_state: Option<Box<PipelineDynamicStateCreateInfo>>,
  layout: VkPipelineLayout,
  render_pass: VkRenderPass,
  subpass: u32,
  base_pipeline_handle: VkPipeline,
  base_pipeline_index: i32,
}
impl<'m> Default for GraphicsPipelineCreateInfo<'m> {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      stage_count: Default::default(),
      stages: core::ptr::null(),
      vertex_input_state: None,
      input_assembly_state: None,
      tessellation_state: None,
      viewport_state: None,
      rasterization_state: None,
      multisample_state: None,
      depth_stencil_state: None,
      color_blend_state: None,
      dynamic_state: None,
      layout: Default::default(),
      render_pass: Default::default(),
      subpass: Default::default(),
      base_pipeline_handle: Default::default(),
      base_pipeline_index: Default::default(),
    }
  }
}
