use super::*;

/// Rusty [VkPipelineViewportStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateInfo.html) (structure)
#[derive(Clone, Debug)]
#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineViewportStateCreateFlags,
  viewport_count: u32,
  viewports: *const VkViewport,
  scissor_count: u32,
  scissors: *const VkRect2D,
}
impl PipelineViewportStateCreateInfo {
  pub const fn new_dynamic(viewport_count: u32, scissor_count: u32) -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: VkPipelineViewportStateCreateFlagBits(0),
      viewport_count,
      viewports: core::ptr::null(),
      scissor_count,
      scissors: core::ptr::null(),
    }
  }
}
