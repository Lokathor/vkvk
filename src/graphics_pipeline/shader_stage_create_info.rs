use core::marker::PhantomData;

use super::*;

/// Rusty version of [VkPipelineShaderStageCreateInfo].
///
/// This is used as part of an array within a larger struct, so we must have the
/// *exact* same size and layout as the raw type, which unfortunately means that
/// we need to throw a PhantomData lifetime into the mix.
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
  // WE MUST NOT CHANGE THE STRUCT SIZE
  life: PhantomData<&'m VkShaderModule>,
}
impl<'m> PipelineShaderStageCreateInfo<'m> {
  /// Makes a new value from the provided data.
  #[inline]
  pub fn new(stage: ShaderStage, module: &'m ShaderModule, name: ZString) -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      stage: stage.into(),
      module: module.vk_shader_module,
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
