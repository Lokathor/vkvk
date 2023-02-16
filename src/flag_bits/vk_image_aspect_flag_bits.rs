/// Khronos: [VkImageAspectFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html)
pub type VkImageAspectFlags = VkImageAspectFlagBits;
/// Khronos: [VkImageAspectFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageAspectFlagBits(pub u32);
pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(0x00000001);
pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(0x00000002);
pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(0x00000004);
pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(0x00000008);
