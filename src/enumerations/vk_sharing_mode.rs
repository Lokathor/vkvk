/// Khronos: [VkSharingMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSharingMode(u32);
impl core::fmt::Debug for VkSharingMode {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      VK_SHARING_MODE_EXCLUSIVE => write!(f, "VK_SHARING_MODE_EXCLUSIVE"),
      VK_SHARING_MODE_CONCURRENT => write!(f, "VK_SHARING_MODE_CONCURRENT"),
      other => write!(f, "VkSharingMode({})", other.0),
    }
  }
}

pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = VkSharingMode(0);
pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = VkSharingMode(1);
