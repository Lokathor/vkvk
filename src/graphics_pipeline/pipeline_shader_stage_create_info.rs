use core::marker::PhantomData;

use super::*;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct PipelineShaderStageCreateInfo<'m> {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineShaderStageCreateFlags,
  stage: VkShaderStageFlagBits,
  module: VkShaderModule,
  name: Option<ZString>,
  specialization_info: Option<Box<VkSpecializationInfo>>,
  //
  life: PhantomData<&'m VkShaderModule>,
}
impl<'m> PipelineShaderStageCreateInfo<'m> {
  pub fn new(stage: ShaderStage, module: &'m VkShaderModule, name: ZString) -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      stage: stage.into(),
      module: *module,
      name: Some(name),
      specialization_info: None,
      life: PhantomData,
    }
  }
}

/// True Rust enum of [VkShaderStageFlagBits], ensuring only 1 bit is set.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum ShaderStage {
  Vertex = VK_SHADER_STAGE_VERTEX_BIT.0,
  Fragment = VK_SHADER_STAGE_FRAGMENT_BIT.0,
}
impl From<ShaderStage> for VkShaderStageFlagBits {
  #[inline]
  fn from(value: ShaderStage) -> Self {
    VkShaderStageFlagBits(value as _)
  }
}
