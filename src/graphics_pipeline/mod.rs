#![allow(non_snake_case)]

//! Structures to create and work with a graphics pipeline.

use core::marker::PhantomData;

use crate::prelude::*;

mod shader_stage_create_info;
pub use shader_stage_create_info::*;

mod dynamic_state_create_info;
pub use dynamic_state_create_info::*;

mod vertex_input_state_create_info;
pub use vertex_input_state_create_info::*;

mod input_assembly_state_create_info;
pub use input_assembly_state_create_info::*;

mod rasterization_state_create_info;
pub use rasterization_state_create_info::*;

mod multisample_state_create_info;
pub use multisample_state_create_info::*;

mod color_blend_state_create_info;
pub use color_blend_state_create_info::*;

mod viewport_state_create_info;
pub use viewport_state_create_info::*;

impl Device {
  #[inline]
  pub unsafe fn vk_create_pipeline_layout(
    &self, info: &VkPipelineLayoutCreateInfo,
  ) -> Result<VkPipelineLayout, VkError> {
    let Some(vkCreatePipelineLayout) = self.0.fns.CreatePipelineLayout else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let mut vk_pipeline_layout = VkPipelineLayout::NULL;
    let r = vkCreatePipelineLayout(
      *self.0.vk_device.read().unwrap(),
      info,
      null(),
      &mut vk_pipeline_layout,
    );
    if r == VK_SUCCESS {
      Ok(vk_pipeline_layout)
    } else {
      Err(VkError::new(r.0).unwrap())
    }
  }
  #[inline]
  pub unsafe fn vk_destroy_pipeline_layout(&self, vk_pipeline_layout: VkPipelineLayout) {
    let Some(vkDestroyPipelineLayout) = self.0.fns.DestroyPipelineLayout else {
      return;
    };
    vkDestroyPipelineLayout(*self.0.vk_device.read().unwrap(), vk_pipeline_layout, null())
  }

  #[inline]
  pub unsafe fn vk_create_render_pass(
    &self, attachments: &[VkAttachmentDescription], subpasses: &[VkSubpassDescription],
    dependencies: &[VkSubpassDependency],
  ) -> Result<VkRenderPass, VkError> {
    let Some(vkCreateRenderPass) = self.0.fns.CreateRenderPass else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let info = VkRenderPassCreateInfo {
      attachments: attachments.as_ptr(),
      attachment_count: attachments.len().try_into().unwrap(),
      subpasses: subpasses.as_ptr(),
      subpass_count: subpasses.len().try_into().unwrap(),
      dependencies: dependencies.as_ptr(),
      dependency_count: dependencies.len().try_into().unwrap(),
      ..Default::default()
    };
    let mut vk_render_pass = VkRenderPass::NULL;
    let r = vkCreateRenderPass(
      *self.0.vk_device.read().unwrap(),
      &info,
      null(),
      &mut vk_render_pass,
    );
    if r == VK_SUCCESS {
      Ok(vk_render_pass)
    } else {
      Err(VkError::new(r.0).unwrap())
    }
  }
  /// ## Sync
  /// On the render pass.
  #[inline]
  pub unsafe fn vk_destroy_render_pass(&self, vk_render_pass: VkRenderPass) {
    let Some(vkDestroyRenderPass) = self.0.fns.DestroyRenderPass else {
      return;
    };
    vkDestroyRenderPass(*self.0.vk_device.read().unwrap(), vk_render_pass, null())
  }
  #[inline]
  pub unsafe fn create_graphics_pipeline(
    &self, info: &GraphicsPipelineCreateInfo<'_>,
  ) -> Result<VkPipeline, VkError> {
    let Some(vkCreateGraphicsPipelines) = self.0.fns.CreateGraphicsPipelines else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let mut vk_pipeline = VkPipeline::NULL;
    let r = vkCreateGraphicsPipelines(
      *self.0.vk_device.read().unwrap(),
      VkPipelineCache::NULL,
      1,
      info as *const GraphicsPipelineCreateInfo as *const VkGraphicsPipelineCreateInfo,
      null(),
      &mut vk_pipeline,
    );
    if r == VK_SUCCESS {
      Ok(vk_pipeline)
    } else {
      Err(VkError::new(r.0).unwrap())
    }
  }
  /// ## Sync
  /// On the pipeline.
  #[inline]
  pub unsafe fn vk_destroy_pipeline(&self, vk_pipeline: VkPipeline) {
    let Some(vkDestroyGraphicsPipeline) = self.0.fns.DestroyPipeline else {
      return;
    };
    vkDestroyGraphicsPipeline(*self.0.vk_device.read().unwrap(), vk_pipeline, null())
  }
}

/// Rusty version of [VkGraphicsPipelineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineCreateInfo.html)
///
/// This is used as part of an array within a larger fn call, so we must have
/// the *exact* same size and layout as the raw type, which unfortunately means
/// that we need to throw a PhantomData lifetime into the mix (on top of the
/// lifetimes that the contained fields already have).
#[derive(Clone, Debug)]
#[repr(C)]
pub struct GraphicsPipelineCreateInfo<'a> {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineCreateFlags,
  stage_count: u32,
  stages: *const PipelineShaderStageCreateInfo<'a>,
  /// Describes the layout of the vertex data going in to the pipeline.
  pub vertex_input_state: Option<Box<PipelineVertexInputStateCreateInfo>>,
  /// Describes how the input vertex data is turned into geometric primitives.
  pub input_assembly_state: Option<Box<PipelineInputAssemblyStateCreateInfo>>,
  tessellation_state: Option<Box<VkPipelineTessellationStateCreateInfo>>,
  pub viewport_state: Option<Box<PipelineViewportStateCreateInfo>>,
  /// Determines how vertex geometry turns into fragments.
  pub rasterization_state: Option<Box<PipelineRasterizationStateCreateInfo>>,
  /// Controls for how multisampling will work.
  pub multisample_state: Option<Box<PipelineMultisampleStateCreateInfo>>,
  depth_stencil_state: Option<Box<VkPipelineDepthStencilStateCreateInfo>>,
  /// Configures color blending (both pipeline-global and also per-attachment).
  pub color_blend_state: Option<Box<PipelineColorBlendStateCreateInfo>>,
  /// Dynamic state for the pipeline. When things are set in here as dynamic
  /// then their static version elsewhere in the creation info will be ignored.
  pub dynamic_state: Option<Box<PipelineDynamicStateCreateInfo>>,
  pub layout: VkPipelineLayout,
  pub render_pass: VkRenderPass,
  pub subpass: u32,
  base_pipeline_handle: VkPipeline,
  base_pipeline_index: i32,
  //
  life: PhantomData<&'a [PipelineShaderStageCreateInfo<'a>]>,
}
impl<'a> GraphicsPipelineCreateInfo<'a> {
  /// Makes a value over the provided shader stages slice, other fields default.
  #[inline]
  pub fn from_shader_stage_info_slice(
    stages: &'a [PipelineShaderStageCreateInfo<'a>],
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
      base_pipeline_index: -1,
      life: PhantomData,
    }
  }
}
