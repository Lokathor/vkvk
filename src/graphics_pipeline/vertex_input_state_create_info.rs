use super::*;

/// Rusty version of [VkPipelineVertexInputStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineVertexInputStateCreateFlags,
  vertex_binding_description_count: u32,
  vertex_binding_descriptions: *const VkVertexInputBindingDescription,
  vertex_attribute_description_count: u32,
  vertex_attribute_descriptions: *const VkVertexInputAttributeDescription,
}
impl Default for PipelineVertexInputStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      vertex_binding_description_count: Default::default(),
      vertex_binding_descriptions: core::ptr::null(),
      vertex_attribute_description_count: Default::default(),
      vertex_attribute_descriptions: core::ptr::null(),
    }
  }
}
