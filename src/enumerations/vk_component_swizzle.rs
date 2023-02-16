/// Khronos: [VkComponentSwizzle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentSwizzle.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkComponentSwizzle(u32);
impl core::fmt::Debug for VkComponentSwizzle {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      VK_COMPONENT_SWIZZLE_IDENTITY => write!(f, "VK_COMPONENT_SWIZZLE_IDENTITY"),
      VK_COMPONENT_SWIZZLE_ZERO => write!(f, "VK_COMPONENT_SWIZZLE_ZERO"),
      VK_COMPONENT_SWIZZLE_ONE => write!(f, "VK_COMPONENT_SWIZZLE_ONE"),
      VK_COMPONENT_SWIZZLE_R => write!(f, "VK_COMPONENT_SWIZZLE_R"),
      VK_COMPONENT_SWIZZLE_G => write!(f, "VK_COMPONENT_SWIZZLE_G"),
      VK_COMPONENT_SWIZZLE_B => write!(f, "VK_COMPONENT_SWIZZLE_B"),
      VK_COMPONENT_SWIZZLE_A => write!(f, "VK_COMPONENT_SWIZZLE_A"),
      other => write!(f, "VkComponentSwizzle({})", other.0),
    }
  }
}
pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = VkComponentSwizzle(0);
pub const VK_COMPONENT_SWIZZLE_ZERO: VkComponentSwizzle = VkComponentSwizzle(1);
pub const VK_COMPONENT_SWIZZLE_ONE: VkComponentSwizzle = VkComponentSwizzle(2);
pub const VK_COMPONENT_SWIZZLE_R: VkComponentSwizzle = VkComponentSwizzle(3);
pub const VK_COMPONENT_SWIZZLE_G: VkComponentSwizzle = VkComponentSwizzle(4);
pub const VK_COMPONENT_SWIZZLE_B: VkComponentSwizzle = VkComponentSwizzle(5);
pub const VK_COMPONENT_SWIZZLE_A: VkComponentSwizzle = VkComponentSwizzle(6);
