use std::collections::HashSet;

use super::*;

/// Rusty version of [VkPipelineDynamicStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateInfo.html)
///
/// Contains an extra field on the end. This is compatible with single pointers,
/// but not with array pointers.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct PipelineDynamicStateCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkPipelineDynamicStateCreateFlags,
  dynamic_state_count: u32,
  dynamic_states: *mut VkDynamicState,
  // EXTRA
  dynamic_state_capacity: u32,
}
impl PipelineDynamicStateCreateInfo {
  /// Taking a hashset as input ensures that all entries are unique.
  #[inline]
  pub fn from_hash_set(set: HashSet<VkDynamicState>) -> Self {
    let mut dynamic_states: ManuallyDrop<Vec<_>> =
      ManuallyDrop::new(set.into_iter().collect());
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      dynamic_state_count: dynamic_states.len().try_into().unwrap(),
      dynamic_state_capacity: dynamic_states.capacity().try_into().unwrap(),
      dynamic_states: dynamic_states.as_mut_ptr(),
    }
  }
}
impl Drop for PipelineDynamicStateCreateInfo {
  #[inline]
  fn drop(&mut self) {
    drop(unsafe {
      Vec::from_raw_parts(
        self.dynamic_states,
        self.dynamic_state_count.try_into().unwrap(),
        self.dynamic_state_capacity.try_into().unwrap(),
      )
    });
  }
}
