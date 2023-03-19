use super::*;

/// Rusty [VkPipelineInputAssemblyStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineInputAssemblyStateCreateFlags,
  topology: VkPrimitiveTopology,
  primitive_restart_enable: VkBool32,
}
impl Default for PipelineInputAssemblyStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      topology: Default::default(),
      primitive_restart_enable: Default::default(),
    }
  }
}
impl PipelineInputAssemblyStateCreateInfo {
  /// Fills the given fields, other fields are defaulted.
  #[inline]
  #[must_use]
  pub fn new(topology: VkPrimitiveTopology, restart_enable: bool) -> Self {
    Self {
      topology,
      primitive_restart_enable: restart_enable.into(),
      ..Default::default()
    }
  }
}
