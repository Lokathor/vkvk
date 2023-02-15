use super::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewCreateInfo {
  pub ty: VkStructureType,
  pub next: *const c_void,
  pub flags: VkImageViewCreateFlags,
  pub image: VkImage,
  pub view_type: VkImageViewType,
  pub format: VkFormat,
  //pub components: VkComponentMapping,
  //pub subresource_range: VkImageSubresourceRange,
}
