/// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html)
pub type VkInstanceCreateFlags = VkInstanceCreateFlagBits;
/// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkInstanceCreateFlagBits(u32);
/// specifies that the instance will enumerate available Vulkan
/// Portability-compliant physical devices and groups in addition to the Vulkan
/// physical devices and groups that are enumerated by default.
pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR: VkInstanceCreateFlagBits =
  VkInstanceCreateFlagBits(0x00000001);
