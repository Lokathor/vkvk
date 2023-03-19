//! Structures to create and work with a graphics pipeline.

use core::marker::PhantomData;

use crate::prelude::*;

mod pipeline_shader_stage_create_info;
pub use pipeline_shader_stage_create_info::*;

mod pipeline_dynamic_state_create_info;
pub use pipeline_dynamic_state_create_info::*;

/// Rusty version of [VkGraphicsPipelineCreateInfo]
///
/// This is used as part of an array within a larger fn call, so we must have
/// the *exact* same size and layout as the raw type, which unfortunately means
/// that we need to throw a PhantomData lifetime into the mix (on top of the
/// lifetimes that the contained fields already have).
#[derive(Clone, Debug)]
#[repr(C)]
pub struct GraphicsPipelineCreateInfo<'m, 'si> {
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
  //
  life: PhantomData<&'si [PipelineShaderStageCreateInfo<'m>]>,
}
impl<'m, 'si> GraphicsPipelineCreateInfo<'m, 'si> {
  /// Makes a value over the provided shader stages slice, other fields default.
  #[inline]
  pub fn from_shader_module_info_slice(
    stages: &'si [PipelineShaderStageCreateInfo<'m>],
  ) -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      stage_count: stages.len().try_into().unwrap(),
      stages: stages.as_ptr(),
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
      life: PhantomData,
    }
  }
}
