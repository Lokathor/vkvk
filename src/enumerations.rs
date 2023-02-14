#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSystemAllocationScope(u32);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkInternalAllocationType(u32);

/// Khronos: [VkStructureType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStructureType.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkStructureType(u32);
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = VkStructureType(0);
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = VkStructureType(1);
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = VkStructureType(2);
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = VkStructureType(3);

/// Khronos: [VkPhysicalDeviceType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceType.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPhysicalDeviceType(u32);
impl core::fmt::Debug for VkPhysicalDeviceType {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      VK_PHYSICAL_DEVICE_TYPE_OTHER => write!(f, "VK_PHYSICAL_DEVICE_TYPE_OTHER"),
      VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => write!(f, "VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU"),
      VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => write!(f, "VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU"),
      VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => write!(f, "VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU"),
      VK_PHYSICAL_DEVICE_TYPE_CPU => write!(f, "VK_PHYSICAL_DEVICE_TYPE_CPU"),
      other => write!(f, "VkPhysicalDeviceType({})", other.0),
    }
  }
}
pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = VkPhysicalDeviceType(0);
pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(1);
pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(2);
pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(3);
pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = VkPhysicalDeviceType(4);
