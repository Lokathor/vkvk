/// Khronos: [VkImageTiling](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageTiling.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageTiling(u32);
impl core::fmt::Debug for VkImageTiling {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      VK_IMAGE_TILING_OPTIMAL => write!(f, "VK_IMAGE_TILING_OPTIMAL"),
      VK_IMAGE_TILING_LINEAR => write!(f, "VK_IMAGE_TILING_LINEAR"),
      other => write!(f, "VkImageTiling({})", other.0),
    }
  }
}
pub const VK_IMAGE_TILING_OPTIMAL: VkImageTiling = VkImageTiling(0);
pub const VK_IMAGE_TILING_LINEAR: VkImageTiling = VkImageTiling(1);
