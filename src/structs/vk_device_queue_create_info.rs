use super::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
  pub ty: VkStructureType,
  pub next: *const c_void,
  pub flags: VkDeviceQueueCreateFlags,
  pub queue_family_index: uint32_t,
  pub queue_count: uint32_t,
  pub queue_priorities: *const float,
}
impl Default for VkDeviceQueueCreateInfo {
  #[inline]
  fn default() -> Self {
    Self {
      ty: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
      next: null(),
      flags: VkDeviceQueueCreateFlags::default(),
      queue_family_index: 0,
      queue_count: 0,
      queue_priorities: null(),
    }
  }
}
