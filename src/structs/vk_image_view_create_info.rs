use super::*;

/// Khronos: [VkImageViewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewCreateInfo {
  pub ty: VkStructureType,
  pub next: *const c_void,
  pub flags: VkImageViewCreateFlags,
  pub image: VkImage,
  pub view_type: VkImageViewType,
  pub format: VkFormat,
  pub components: VkComponentMapping,
  pub subresource_range: VkImageSubresourceRange,
}
impl Default for VkImageViewCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    VkImageViewCreateInfo {
      ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
      next: null(),
      flags: VkImageViewCreateFlags::default(),
      image: VkImage::zero(),
      view_type: VK_IMAGE_VIEW_TYPE_1D,
      format: VK_FORMAT_UNDEFINED,
      components: VkComponentMapping::default(),
      subresource_range: VkImageSubresourceRange::default(),
    }
  }
}
