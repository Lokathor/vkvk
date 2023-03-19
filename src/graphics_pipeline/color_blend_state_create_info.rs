use super::*;

/// Rusty [VkPipelineColorBlendStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html) (structure)
#[derive(Clone, Debug)]
#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineColorBlendStateCreateFlags,
  logic_op_enable: VkBool32,
  logic_op: VkLogicOp,
  attachment_count: u32,
  attachments: *mut VkPipelineColorBlendAttachmentState,
  blend_constants: [c_float; 4],
  //
  attachment_capacity: u32,
}
impl Default for PipelineColorBlendStateCreateInfo {
  /// The default configures the single color attachment for pre-multiplied
  /// alpha
  #[inline]
  #[must_use]
  fn default() -> Self {
    let mut attachments: ManuallyDrop<Vec<_>> =
      ManuallyDrop::new(vec![VkPipelineColorBlendAttachmentState {
        blend_enable: true.into(),
        src_color_blend_factor: VK_BLEND_FACTOR_ONE,
        dst_color_blend_factor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        color_blend_op: VK_BLEND_OP_ADD,
        src_alpha_blend_factor: VK_BLEND_FACTOR_ONE,
        dst_alpha_blend_factor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        alpha_blend_op: VK_BLEND_OP_ADD,
        color_write_mask: VK_COLOR_COMPONENT_R_BIT
          | VK_COLOR_COMPONENT_G_BIT
          | VK_COLOR_COMPONENT_B_BIT
          | VK_COLOR_COMPONENT_A_BIT,
      }]);
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      logic_op_enable: Default::default(),
      logic_op: Default::default(),
      attachment_count: attachments.len().try_into().unwrap(),
      attachment_capacity: attachments.capacity().try_into().unwrap(),
      blend_constants: [Default::default(); 4],
      attachments: attachments.as_mut_ptr(),
    }
  }
}
impl Drop for PipelineColorBlendStateCreateInfo {
  #[inline]
  fn drop(&mut self) {
    drop(unsafe {
      Vec::from_raw_parts(
        self.attachments,
        self.attachment_count.try_into().unwrap(),
        self.attachment_capacity.try_into().unwrap(),
      )
    });
  }
}
impl PipelineColorBlendStateCreateInfo {
  /// Runs a closure using the attachments list.
  #[inline]
  pub fn attachments_mut<F: FnOnce(&mut Vec<VkPipelineColorBlendAttachmentState>)>(
    &mut self, op: F,
  ) {
    fake_ptr_len_cap!(
      self.attachments,
      self.attachment_count,
      self.attachment_capacity,
      op
    );
  }
}
