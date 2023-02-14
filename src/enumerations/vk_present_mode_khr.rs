/// Khronos: [VkPresentModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPresentModeKHR(u32);
impl core::fmt::Debug for VkPresentModeKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      VK_PRESENT_MODE_IMMEDIATE_KHR => write!(f, "VK_PRESENT_MODE_IMMEDIATE_KHR"),
      VK_PRESENT_MODE_MAILBOX_KHR => write!(f, "VK_PRESENT_MODE_MAILBOX_KHR"),
      VK_PRESENT_MODE_FIFO_KHR => write!(f, "VK_PRESENT_MODE_FIFO_KHR"),
      VK_PRESENT_MODE_FIFO_RELAXED_KHR => write!(f, "VK_PRESENT_MODE_FIFO_RELAXED_KHR"),
      VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR => {
        write!(f, "VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR")
      }
      VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR => {
        write!(f, "VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR")
      }
      other => write!(f, "VkPresentModeKHR({})", other.0),
    }
  }
}
pub const VK_PRESENT_MODE_IMMEDIATE_KHR: VkPresentModeKHR = VkPresentModeKHR(0);
pub const VK_PRESENT_MODE_MAILBOX_KHR: VkPresentModeKHR = VkPresentModeKHR(1);
pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = VkPresentModeKHR(2);
pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: VkPresentModeKHR = VkPresentModeKHR(3);
pub const VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR: VkPresentModeKHR =
  VkPresentModeKHR(1000111000);
pub const VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR: VkPresentModeKHR =
  VkPresentModeKHR(1000111001);
