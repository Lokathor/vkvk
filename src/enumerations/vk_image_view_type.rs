/// Khronos: [VkImageViewType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewType.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageViewType(u32);
impl core::fmt::Debug for VkImageViewType {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      VK_IMAGE_VIEW_TYPE_2D => write!(f, "VK_IMAGE_VIEW_TYPE_2D"),
      VK_IMAGE_VIEW_TYPE_1D => write!(f, "VK_IMAGE_VIEW_TYPE_1D"),
      VK_IMAGE_VIEW_TYPE_3D => write!(f, "VK_IMAGE_VIEW_TYPE_3D"),
      VK_IMAGE_VIEW_TYPE_CUBE => write!(f, "VK_IMAGE_VIEW_TYPE_CUBE"),
      VK_IMAGE_VIEW_TYPE_1D_ARRAY => write!(f, "VK_IMAGE_VIEW_TYPE_1D_ARRAY"),
      VK_IMAGE_VIEW_TYPE_2D_ARRAY => write!(f, "VK_IMAGE_VIEW_TYPE_2D_ARRAY"),
      VK_IMAGE_VIEW_TYPE_CUBE_ARRAY => write!(f, "VK_IMAGE_VIEW_TYPE_CUBE_ARRAY"),
      other => write!(f, "VkImageTiling({})", other.0),
    }
  }
}
pub const VK_IMAGE_VIEW_TYPE_1D: VkImageViewType = VkImageViewType(0);
pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = VkImageViewType(1);
pub const VK_IMAGE_VIEW_TYPE_3D: VkImageViewType = VkImageViewType(2);
pub const VK_IMAGE_VIEW_TYPE_CUBE: VkImageViewType = VkImageViewType(3);
pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: VkImageViewType = VkImageViewType(4);
pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: VkImageViewType = VkImageViewType(5);
pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: VkImageViewType = VkImageViewType(6);
