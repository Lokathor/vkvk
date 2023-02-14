/// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html)
pub type VkInstanceCreateFlags = VkInstanceCreateFlagBits;
/// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkInstanceCreateFlagBits(pub u32);
/// specifies that the instance will enumerate available Vulkan
/// Portability-compliant physical devices and groups in addition to the Vulkan
/// physical devices and groups that are enumerated by default.
pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR: VkInstanceCreateFlagBits =
  VkInstanceCreateFlagBits(0x00000001);

/// Khronos: [VkSampleCountFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html)
pub type VkSampleCountFlags = VkSampleCountFlagBits;
/// Khronos: [VkSampleCountFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSampleCountFlagBits(pub u32);
impl core::fmt::Debug for VkSampleCountFlagBits {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    if (self.0 & VK_SAMPLE_COUNT_1_BIT.0) != 0 {
      x.entry(&1_u32);
    }
    if (self.0 & VK_SAMPLE_COUNT_2_BIT.0) != 0 {
      x.entry(&2_u32);
    }
    if (self.0 & VK_SAMPLE_COUNT_4_BIT.0) != 0 {
      x.entry(&4_u32);
    }
    if (self.0 & VK_SAMPLE_COUNT_8_BIT.0) != 0 {
      x.entry(&8_u32);
    }
    if (self.0 & VK_SAMPLE_COUNT_16_BIT.0) != 0 {
      x.entry(&16_u32);
    }
    if (self.0 & VK_SAMPLE_COUNT_32_BIT.0) != 0 {
      x.entry(&32_u32);
    }
    if (self.0 & VK_SAMPLE_COUNT_64_BIT.0) != 0 {
      x.entry(&64_u32);
    }
    x.finish()
  }
}
/// specifies an image with 1 sample per pixel.
pub const VK_SAMPLE_COUNT_1_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(0x00000001);
/// specifies an image with 2 samples per pixel.
pub const VK_SAMPLE_COUNT_2_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(0x00000002);
/// specifies an image with 4 samples per pixel.
pub const VK_SAMPLE_COUNT_4_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(0x00000004);
/// specifies an image with 8 samples per pixel.
pub const VK_SAMPLE_COUNT_8_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(0x00000008);
/// specifies an image with 16 samples per pixel.
pub const VK_SAMPLE_COUNT_16_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(0x00000010);
/// specifies an image with 32 samples per pixel.
pub const VK_SAMPLE_COUNT_32_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(0x00000020);
/// specifies an image with 64 samples per pixel.
pub const VK_SAMPLE_COUNT_64_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(0x00000040);

/// Khronos: [VkQueueFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html)
pub type VkQueueFlags = VkQueueFlagBits;
/// Khronos: [VkQueueFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkQueueFlagBits(pub u32);
impl VkQueueFlagBits {
  #[inline]
  #[must_use]
  pub const fn graphics(self) -> bool {
    (self.0 & VK_QUEUE_GRAPHICS_BIT.0) != 0
  }
}
impl core::fmt::Debug for VkQueueFlagBits {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    if self.graphics() {
      x.entry(&"Graphics");
    }
    if (self.0 & VK_QUEUE_COMPUTE_BIT.0) != 0 {
      x.entry(&"Compute");
    }
    if (self.0 & VK_QUEUE_TRANSFER_BIT.0) != 0 {
      x.entry(&"Transfer");
    }
    if (self.0 & VK_QUEUE_SPARSE_BINDING_BIT.0) != 0 {
      x.entry(&"Binding");
    }
    if (self.0 & VK_QUEUE_PROTECTED_BIT.0) != 0 {
      x.entry(&"Protected");
    }
    x.finish()
  }
}
/// specifies that queues in this queue family support graphics operations.
pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlagBits = VkQueueFlagBits(0x00000001);
/// specifies that queues in this queue family support compute operations.
pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlagBits = VkQueueFlagBits(0x00000002);
/// specifies that queues in this queue family support transfer operations.
pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlagBits = VkQueueFlagBits(0x00000004);
/// specifies that queues in this queue family support sparse memory management
/// operations.
///
/// If any of the sparse resource features are enabled, then at least one queue
/// family must support this bit.
pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlagBits = VkQueueFlagBits(0x00000008);
///  specifies that queues in this queue family support the
/// VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT bit.
///
/// If the physical device supports the protectedMemory feature, at least one of
/// its queue families must support this bit.
pub const VK_QUEUE_PROTECTED_BIT: VkQueueFlagBits = VkQueueFlagBits(0x00000010);

/// Khronos: [VkDeviceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlags.html)
pub type VkDeviceCreateFlags = VkDeviceCreateFlagBits;
/// Khronos: [VkDeviceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlags.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkDeviceCreateFlagBits(pub u32);

/// Khronos: [VkDeviceQueueCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html)
pub type VkDeviceQueueCreateFlags = VkDeviceQueueCreateFlagBits;
/// Khronos: [VkDeviceQueueCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkDeviceQueueCreateFlagBits(pub u32);
