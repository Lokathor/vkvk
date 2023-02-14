/// Khronos: [VkImageType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageType.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageType(u32);
impl core::fmt::Debug for VkImageType {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      VK_IMAGE_TYPE_1D => write!(f, "VK_IMAGE_TYPE_1D"),
      VK_IMAGE_TYPE_2D => write!(f, "VK_IMAGE_TYPE_2D"),
      VK_IMAGE_TYPE_3D => write!(f, "VK_IMAGE_TYPE_3D"),
      other => write!(f, "VkImageType({})", other.0),
    }
  }
}
pub const VK_IMAGE_TYPE_1D: VkImageType = VkImageType(0);
pub const VK_IMAGE_TYPE_2D: VkImageType = VkImageType(1);
pub const VK_IMAGE_TYPE_3D: VkImageType = VkImageType(2);
