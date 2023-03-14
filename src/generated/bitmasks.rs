#![allow(non_upper_case_globals)]

define_bitmask!(
  /// Khronos: [VkAccelerationStructureCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html) (bitmask)
  VkAccelerationStructureCreateFlagBitsKHR(u32)
);
/// Khronos: [VkAccelerationStructureCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html) (bitmask)
pub type VkAccelerationStructureCreateFlagsKHR = VkAccelerationStructureCreateFlagBitsKHR;
pub const VK_ACCELERATION_STRUCTURE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT:
  VkAccelerationStructureCreateFlagBitsKHR =
  VkAccelerationStructureCreateFlagBitsKHR(1 << 3);
pub const VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR:
  VkAccelerationStructureCreateFlagBitsKHR =
  VkAccelerationStructureCreateFlagBitsKHR(1 << 0);
pub const VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV:
  VkAccelerationStructureCreateFlagBitsKHR =
  VkAccelerationStructureCreateFlagBitsKHR(1 << 2);
impl core::fmt::Debug for VkAccelerationStructureCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "descriptor_buffer_capture_replay"),
      (1, "device_address_capture_replay"),
      (4, "motion"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkAccelerationStructureCreateFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn descriptor_buffer_capture_replay(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn device_address_capture_replay(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn motion(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkAccelerationStructureMotionInfoFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoFlagsNV.html) (bitmask, no bits defined)
  VkAccelerationStructureMotionInfoFlagBitsNV(u32)
);
/// Khronos: [VkAccelerationStructureMotionInfoFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoFlagsNV.html) (bitmask, no bits defined)
pub type VkAccelerationStructureMotionInfoFlagsNV =
  VkAccelerationStructureMotionInfoFlagBitsNV;
impl core::fmt::Debug for VkAccelerationStructureMotionInfoFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkAccelerationStructureMotionInstanceFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceFlagsNV.html) (bitmask, no bits defined)
  VkAccelerationStructureMotionInstanceFlagBitsNV(u32)
);
/// Khronos: [VkAccelerationStructureMotionInstanceFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceFlagsNV.html) (bitmask, no bits defined)
pub type VkAccelerationStructureMotionInstanceFlagsNV =
  VkAccelerationStructureMotionInstanceFlagBitsNV;
impl core::fmt::Debug for VkAccelerationStructureMotionInstanceFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkAccessFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html) (bitmask)
  VkAccessFlagBits(u32)
);
/// Khronos: [VkAccessFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html) (bitmask)
pub type VkAccessFlags = VkAccessFlagBits;
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR: VkAccessFlagBits =
  VkAccessFlagBits(1 << 21);
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR: VkAccessFlagBits =
  VkAccessFlagBits(1 << 22);
/// Controls coherency of color attachment reads
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 7);
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 19);
/// Controls coherency of color attachment writes
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 8);
pub const VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV: VkAccessFlagBits =
  VkAccessFlagBits(1 << 17);
pub const VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV: VkAccessFlagBits =
  VkAccessFlagBits(1 << 18);
/// read access flag for reading conditional rendering predicate
pub const VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 20);
/// Controls coherency of depth/stencil attachment reads
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 9);
/// Controls coherency of depth/stencil attachment writes
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 10);
pub const VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 24);
pub const VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits =
  VkAccessFlagBits(1 << 23);
/// Controls coherency of host reads
pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 13);
/// Controls coherency of host writes
pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 14);
/// Controls coherency of index reads
pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 1);
/// Controls coherency of indirect command reads
pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 0);
/// Controls coherency of input attachment reads
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 4);
/// Controls coherency of memory reads
pub const VK_ACCESS_MEMORY_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 15);
/// Controls coherency of memory writes
pub const VK_ACCESS_MEMORY_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 16);
/// Controls coherency of shader reads
pub const VK_ACCESS_SHADER_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 5);
/// Controls coherency of shader writes
pub const VK_ACCESS_SHADER_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 6);
/// Controls coherency of transfer reads
pub const VK_ACCESS_TRANSFER_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 11);
/// Controls coherency of transfer writes
pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 12);
pub const VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 26);
pub const VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 27);
pub const VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 25);
/// Controls coherency of uniform buffer reads
pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 3);
/// Controls coherency of vertex attribute reads
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits =
  VkAccessFlagBits(1 << 2);
pub const VK_ACCESS_NONE: VkAccessFlagBits = VkAccessFlagBits(0);
/// Alias of [`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR`]
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV: VkAccessFlagBits =
  VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR;
/// Alias of [`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`]
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV: VkAccessFlagBits =
  VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR;
/// Alias of [`VK_ACCESS_NONE`]
pub const VK_ACCESS_NONE_KHR: VkAccessFlagBits = VK_ACCESS_NONE;
/// Alias of [`VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR`]
pub const VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV: VkAccessFlagBits =
  VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR;
impl core::fmt::Debug for VkAccessFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2097152, "acceleration_structure_read"),
      (4194304, "acceleration_structure_write"),
      (128, "color_attachment_read"),
      (524288, "color_attachment_read_noncoherent"),
      (256, "color_attachment_write"),
      (131072, "command_preprocess_read"),
      (262144, "command_preprocess_write"),
      (1048576, "conditional_rendering_read"),
      (512, "depth_stencil_attachment_read"),
      (1024, "depth_stencil_attachment_write"),
      (16777216, "fragment_density_map_read"),
      (8388608, "fragment_shading_rate_attachment_read"),
      (8192, "host_read"),
      (16384, "host_write"),
      (2, "index_read"),
      (1, "indirect_command_read"),
      (16, "input_attachment_read"),
      (32768, "memory_read"),
      (65536, "memory_write"),
      (32, "shader_read"),
      (64, "shader_write"),
      (2048, "transfer_read"),
      (4096, "transfer_write"),
      (67108864, "transform_feedback_counter_read"),
      (134217728, "transform_feedback_counter_write"),
      (33554432, "transform_feedback_write"),
      (8, "uniform_read"),
      (4, "vertex_attribute_read"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkAccessFlagBits {
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_read(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_write(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_read(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_read_noncoherent(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_write(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn command_preprocess_read(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn command_preprocess_write(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn conditional_rendering_read(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn depth_stencil_attachment_read(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn depth_stencil_attachment_write(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_map_read(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shading_rate_attachment_read(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_read(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_write(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn index_read(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn indirect_command_read(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn input_attachment_read(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn memory_read(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn memory_write(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_read(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_write(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_read(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_write(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback_counter_read(self) -> bool {
    (self.0 & 67108864) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback_counter_write(self) -> bool {
    (self.0 & 134217728) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback_write(self) -> bool {
    (self.0 & 33554432) != 0
  }
  #[inline]
  #[must_use]
  pub const fn uniform_read(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_attribute_read(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkAccessFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html) (bitmask)
  VkAccessFlagBits2(u64)
);
/// Khronos: [VkAccessFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html) (bitmask)
pub type VkAccessFlags2 = VkAccessFlagBits2;
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 21);
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 22);
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 7);
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 19);
pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 8);
pub const VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 17);
pub const VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 18);
/// read access flag for reading conditional rendering predicate
pub const VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 20);
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 9);
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 10);
pub const VK_ACCESS_2_DESCRIPTOR_BUFFER_READ_BIT_EXT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 41);
pub const VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 24);
pub const VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 23);
pub const VK_ACCESS_2_HOST_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 13);
pub const VK_ACCESS_2_HOST_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 14);
pub const VK_ACCESS_2_INDEX_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 1);
pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 0);
pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 4);
pub const VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 39);
pub const VK_ACCESS_2_MEMORY_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 15);
pub const VK_ACCESS_2_MEMORY_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 16);
pub const VK_ACCESS_2_MICROMAP_READ_BIT_EXT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 44);
pub const VK_ACCESS_2_MICROMAP_WRITE_BIT_EXT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 45);
pub const VK_ACCESS_2_OPTICAL_FLOW_READ_BIT_NV: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 42);
pub const VK_ACCESS_2_OPTICAL_FLOW_WRITE_BIT_NV: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 43);
pub const VK_ACCESS_2_RESERVED_46_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 46);
pub const VK_ACCESS_2_SHADER_BINDING_TABLE_READ_BIT_KHR: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 40);
pub const VK_ACCESS_2_SHADER_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 5);
pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 32);
pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 33);
pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 34);
pub const VK_ACCESS_2_SHADER_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 6);
pub const VK_ACCESS_2_TRANSFER_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 11);
pub const VK_ACCESS_2_TRANSFER_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 12);
pub const VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 26);
pub const VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 27);
pub const VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 25);
pub const VK_ACCESS_2_UNIFORM_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 3);
pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 2);
pub const VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 35);
pub const VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR: VkAccessFlagBits2 =
  VkAccessFlagBits2(1 << 36);
pub const VK_ACCESS_2_NONE: VkAccessFlagBits2 = VkAccessFlagBits2(0);
/// Alias of [`VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`]
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV: VkAccessFlagBits2 =
  VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR;
/// Alias of [`VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`]
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV: VkAccessFlagBits2 =
  VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR;
/// Alias of [`VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`]
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT;
/// Alias of [`VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`]
pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`]
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT;
/// Alias of [`VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`]
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_HOST_READ_BIT`]
pub const VK_ACCESS_2_HOST_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_HOST_READ_BIT;
/// Alias of [`VK_ACCESS_2_HOST_WRITE_BIT`]
pub const VK_ACCESS_2_HOST_WRITE_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_HOST_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_INDEX_READ_BIT`]
pub const VK_ACCESS_2_INDEX_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_INDEX_READ_BIT;
/// Alias of [`VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`]
pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT;
/// Alias of [`VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`]
pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT;
/// Alias of [`VK_ACCESS_2_MEMORY_READ_BIT`]
pub const VK_ACCESS_2_MEMORY_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_MEMORY_READ_BIT;
/// Alias of [`VK_ACCESS_2_MEMORY_WRITE_BIT`]
pub const VK_ACCESS_2_MEMORY_WRITE_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_MEMORY_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_NONE`]
pub const VK_ACCESS_2_NONE_KHR: VkAccessFlagBits2 = VK_ACCESS_2_NONE;
/// Alias of [`VK_ACCESS_2_SHADER_READ_BIT`]
pub const VK_ACCESS_2_SHADER_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_SHADER_READ_BIT;
/// Alias of [`VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`]
pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_SHADER_SAMPLED_READ_BIT;
/// Alias of [`VK_ACCESS_2_SHADER_STORAGE_READ_BIT`]
pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_SHADER_STORAGE_READ_BIT;
/// Alias of [`VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`]
pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_SHADER_WRITE_BIT`]
pub const VK_ACCESS_2_SHADER_WRITE_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_SHADER_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR`]
pub const VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV: VkAccessFlagBits2 =
  VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR;
/// Alias of [`VK_ACCESS_2_TRANSFER_READ_BIT`]
pub const VK_ACCESS_2_TRANSFER_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_TRANSFER_READ_BIT;
/// Alias of [`VK_ACCESS_2_TRANSFER_WRITE_BIT`]
pub const VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_TRANSFER_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_UNIFORM_READ_BIT`]
pub const VK_ACCESS_2_UNIFORM_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_UNIFORM_READ_BIT;
/// Alias of [`VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`]
pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR: VkAccessFlagBits2 =
  VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT;
impl core::fmt::Debug for VkAccessFlagBits2 {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2097152, "acceleration_structure_read"),
      (4194304, "acceleration_structure_write"),
      (128, "color_attachment_read"),
      (524288, "color_attachment_read_noncoherent"),
      (256, "color_attachment_write"),
      (131072, "command_preprocess_read"),
      (262144, "command_preprocess_write"),
      (1048576, "conditional_rendering_read"),
      (512, "depth_stencil_attachment_read"),
      (1024, "depth_stencil_attachment_write"),
      (2199023255552, "descriptor_buffer_read"),
      (16777216, "fragment_density_map_read"),
      (8388608, "fragment_shading_rate_attachment_read"),
      (8192, "host_read"),
      (16384, "host_write"),
      (2, "index_read"),
      (1, "indirect_command_read"),
      (16, "input_attachment_read"),
      (549755813888, "invocation_mask_read"),
      (32768, "memory_read"),
      (65536, "memory_write"),
      (17592186044416, "micromap_read"),
      (35184372088832, "micromap_write"),
      (4398046511104, "optical_flow_read"),
      (8796093022208, "optical_flow_write"),
      (70368744177664, "reserved_46"),
      (1099511627776, "shader_binding_table_read"),
      (32, "shader_read"),
      (4294967296, "shader_sampled_read"),
      (8589934592, "shader_storage_read"),
      (17179869184, "shader_storage_write"),
      (64, "shader_write"),
      (2048, "transfer_read"),
      (4096, "transfer_write"),
      (67108864, "transform_feedback_counter_read"),
      (134217728, "transform_feedback_counter_write"),
      (33554432, "transform_feedback_write"),
      (8, "uniform_read"),
      (4, "vertex_attribute_read"),
      (34359738368, "video_decode_read"),
      (68719476736, "video_decode_write"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkAccessFlagBits2 {
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_read(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_write(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_read(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_read_noncoherent(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_write(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn command_preprocess_read(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn command_preprocess_write(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn conditional_rendering_read(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn depth_stencil_attachment_read(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn depth_stencil_attachment_write(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn descriptor_buffer_read(self) -> bool {
    (self.0 & 2199023255552) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_map_read(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shading_rate_attachment_read(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_read(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_write(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn index_read(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn indirect_command_read(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn input_attachment_read(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn invocation_mask_read(self) -> bool {
    (self.0 & 549755813888) != 0
  }
  #[inline]
  #[must_use]
  pub const fn memory_read(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn memory_write(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn micromap_read(self) -> bool {
    (self.0 & 17592186044416) != 0
  }
  #[inline]
  #[must_use]
  pub const fn micromap_write(self) -> bool {
    (self.0 & 35184372088832) != 0
  }
  #[inline]
  #[must_use]
  pub const fn optical_flow_read(self) -> bool {
    (self.0 & 4398046511104) != 0
  }
  #[inline]
  #[must_use]
  pub const fn optical_flow_write(self) -> bool {
    (self.0 & 8796093022208) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_46(self) -> bool {
    (self.0 & 70368744177664) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_binding_table_read(self) -> bool {
    (self.0 & 1099511627776) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_read(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_sampled_read(self) -> bool {
    (self.0 & 4294967296) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_storage_read(self) -> bool {
    (self.0 & 8589934592) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_storage_write(self) -> bool {
    (self.0 & 17179869184) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_write(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_read(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_write(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback_counter_read(self) -> bool {
    (self.0 & 67108864) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback_counter_write(self) -> bool {
    (self.0 & 134217728) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback_write(self) -> bool {
    (self.0 & 33554432) != 0
  }
  #[inline]
  #[must_use]
  pub const fn uniform_read(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_attribute_read(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_read(self) -> bool {
    (self.0 & 34359738368) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_write(self) -> bool {
    (self.0 & 68719476736) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkAcquireProfilingLockFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html) (bitmask)
  VkAcquireProfilingLockFlagBitsKHR(u32)
);
/// Khronos: [VkAcquireProfilingLockFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html) (bitmask)
pub type VkAcquireProfilingLockFlagsKHR = VkAcquireProfilingLockFlagBitsKHR;
impl core::fmt::Debug for VkAcquireProfilingLockFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkAndroidSurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidSurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
  VkAndroidSurfaceCreateFlagBitsKHR(u32)
);
/// Khronos: [VkAndroidSurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidSurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkAndroidSurfaceCreateFlagsKHR = VkAndroidSurfaceCreateFlagBitsKHR;
impl core::fmt::Debug for VkAndroidSurfaceCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkAttachmentDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionFlagBits.html) (bitmask)
  VkAttachmentDescriptionFlagBits(u32)
);
/// Khronos: [VkAttachmentDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionFlagBits.html) (bitmask)
pub type VkAttachmentDescriptionFlags = VkAttachmentDescriptionFlagBits;
/// The attachment may alias physical memory of another attachment in the same
/// render pass
pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlagBits =
  VkAttachmentDescriptionFlagBits(1 << 0);
impl core::fmt::Debug for VkAttachmentDescriptionFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "may_alias")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkAttachmentDescriptionFlagBits {
  #[inline]
  #[must_use]
  pub const fn may_alias(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkBufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html) (bitmask)
  VkBufferCreateFlagBits(u32)
);
/// Khronos: [VkBufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html) (bitmask)
pub type VkBufferCreateFlags = VkBufferCreateFlagBits;
pub const VK_BUFFER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT:
  VkBufferCreateFlagBits = VkBufferCreateFlagBits(1 << 5);
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkBufferCreateFlagBits =
  VkBufferCreateFlagBits(1 << 4);
/// Buffer requires protected memory
pub const VK_BUFFER_CREATE_PROTECTED_BIT: VkBufferCreateFlagBits =
  VkBufferCreateFlagBits(1 << 3);
/// Buffer should support constant data access to physical memory ranges mapped
/// into multiple locations of sparse buffers
pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkBufferCreateFlagBits =
  VkBufferCreateFlagBits(1 << 2);
/// Buffer should support sparse backing
pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkBufferCreateFlagBits =
  VkBufferCreateFlagBits(1 << 0);
/// Buffer should support sparse backing with partial residency
pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkBufferCreateFlagBits =
  VkBufferCreateFlagBits(1 << 1);
/// Alias of [`VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`]
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: VkBufferCreateFlagBits =
  VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
/// Alias of [`VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`]
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkBufferCreateFlagBits =
  VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
impl core::fmt::Debug for VkBufferCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (32, "descriptor_buffer_capture_replay"),
      (16, "device_address_capture_replay"),
      (8, "protected"),
      (4, "sparse_aliased"),
      (1, "sparse_binding"),
      (2, "sparse_residency"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkBufferCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn descriptor_buffer_capture_replay(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn device_address_capture_replay(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn protected(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sparse_aliased(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sparse_binding(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sparse_residency(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html) (bitmask)
  VkBufferUsageFlagBits(u32)
);
/// Khronos: [VkBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html) (bitmask)
pub type VkBufferUsageFlags = VkBufferUsageFlagBits;
pub const VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR:
  VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 19);
pub const VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 20);
/// Specifies the buffer can be used as predicate in conditional rendering
pub const VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 9);
/// Can be used as source of fixed-function index fetch (index buffer)
pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 6);
/// Can be the source of indirect parameters (e.g. indirect buffer, parameter
/// buffer)
pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 8);
pub const VK_BUFFER_USAGE_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 23);
pub const VK_BUFFER_USAGE_MICROMAP_STORAGE_BIT_EXT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 24);
pub const VK_BUFFER_USAGE_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT:
  VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 26);
pub const VK_BUFFER_USAGE_RESERVED_18_BIT_QCOM: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 18);
pub const VK_BUFFER_USAGE_RESERVED_25_BIT_AMD: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 25);
pub const VK_BUFFER_USAGE_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 22);
pub const VK_BUFFER_USAGE_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 21);
pub const VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 10);
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 17);
/// Can be used as SSBO
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 5);
/// Can be used as IBO
pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 3);
/// Can be used as a destination of transfer operations
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 1);
/// Can be used as a source of transfer operations
pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 0);
pub const VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 11);
pub const VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT:
  VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 12);
/// Can be used as UBO
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 4);
/// Can be used as TBO
pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 2);
/// Can be used as source of fixed-function vertex fetch (VBO)
pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 7);
pub const VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 14);
pub const VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1 << 13);
/// Alias of [`VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR`]
pub const VK_BUFFER_USAGE_RAY_TRACING_BIT_NV: VkBufferUsageFlagBits =
  VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR;
/// Alias of [`VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT`]
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT: VkBufferUsageFlagBits =
  VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT;
/// Alias of [`VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT`]
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR: VkBufferUsageFlagBits =
  VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT;
impl core::fmt::Debug for VkBufferUsageFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (524288, "acceleration_structure_build_input_read_only"),
      (1048576, "acceleration_structure_storage"),
      (512, "conditional_rendering"),
      (64, "index_buffer"),
      (256, "indirect_buffer"),
      (8388608, "micromap_build_input_read_only"),
      (16777216, "micromap_storage"),
      (67108864, "push_descriptors_descriptor_buffer"),
      (262144, "reserved_18"),
      (33554432, "reserved_25"),
      (4194304, "resource_descriptor_buffer"),
      (2097152, "sampler_descriptor_buffer"),
      (1024, "shader_binding_table"),
      (131072, "shader_device_address"),
      (32, "storage_buffer"),
      (8, "storage_texel_buffer"),
      (2, "transfer_dst"),
      (1, "transfer_src"),
      (2048, "transform_feedback_buffer"),
      (4096, "transform_feedback_counter_buffer"),
      (16, "uniform_buffer"),
      (4, "uniform_texel_buffer"),
      (128, "vertex_buffer"),
      (16384, "video_decode_dst"),
      (8192, "video_decode_src"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkBufferUsageFlagBits {
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_build_input_read_only(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_storage(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn conditional_rendering(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn index_buffer(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn indirect_buffer(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn micromap_build_input_read_only(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn micromap_storage(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn push_descriptors_descriptor_buffer(self) -> bool {
    (self.0 & 67108864) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_18(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_25(self) -> bool {
    (self.0 & 33554432) != 0
  }
  #[inline]
  #[must_use]
  pub const fn resource_descriptor_buffer(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampler_descriptor_buffer(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_binding_table(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_device_address(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_buffer(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_texel_buffer(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_dst(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_src(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback_buffer(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback_counter_buffer(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn uniform_buffer(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn uniform_texel_buffer(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_buffer(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_dst(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_src(self) -> bool {
    (self.0 & 8192) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkBufferViewCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateFlags.html) (bitmask, no bits defined)
  VkBufferViewCreateFlagBits(u32)
);
/// Khronos: [VkBufferViewCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateFlags.html) (bitmask, no bits defined)
pub type VkBufferViewCreateFlags = VkBufferViewCreateFlagBits;
impl core::fmt::Debug for VkBufferViewCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkBuildAccelerationStructureFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html) (bitmask)
  VkBuildAccelerationStructureFlagBitsKHR(u32)
);
/// Khronos: [VkBuildAccelerationStructureFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html) (bitmask)
pub type VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagBitsKHR;
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 1);
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_EXT:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 7);
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 8);
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_EXT:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 6);
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 0);
pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 4);
pub const VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 5);
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 3);
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 2);
pub const VK_BUILD_ACCELERATION_STRUCTURE_RESERVED_BIT_10_NV:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 10);
pub const VK_BUILD_ACCELERATION_STRUCTURE_RESERVED_BIT_9_NV:
  VkBuildAccelerationStructureFlagBitsKHR =
  VkBuildAccelerationStructureFlagBitsKHR(1 << 9);
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV:
  VkBuildAccelerationStructureFlagBitsKHR =
  VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR;
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV:
  VkBuildAccelerationStructureFlagBitsKHR =
  VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR;
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV:
  VkBuildAccelerationStructureFlagBitsKHR =
  VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR;
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV:
  VkBuildAccelerationStructureFlagBitsKHR =
  VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR;
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV:
  VkBuildAccelerationStructureFlagBitsKHR =
  VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR;
impl core::fmt::Debug for VkBuildAccelerationStructureFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "allow_compaction"),
      (128, "allow_disable_opacity_micromaps"),
      (256, "allow_opacity_micromap_data_update"),
      (64, "allow_opacity_micromap_update"),
      (1, "allow_update"),
      (16, "low_memory"),
      (32, "motion"),
      (8, "prefer_fast_build"),
      (4, "prefer_fast_trace"),
      (1024, "reserved_bit_10"),
      (512, "reserved_bit_9"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkBuildAccelerationStructureFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn allow_compaction(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn allow_disable_opacity_micromaps(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn allow_opacity_micromap_data_update(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn allow_opacity_micromap_update(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn allow_update(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn low_memory(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn motion(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn prefer_fast_build(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn prefer_fast_trace(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_bit_10(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_bit_9(self) -> bool {
    (self.0 & 512) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkBuildMicromapFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapFlagBitsEXT.html) (bitmask)
  VkBuildMicromapFlagBitsEXT(u32)
);
/// Khronos: [VkBuildMicromapFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapFlagBitsEXT.html) (bitmask)
pub type VkBuildMicromapFlagsEXT = VkBuildMicromapFlagBitsEXT;
pub const VK_BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT: VkBuildMicromapFlagBitsEXT =
  VkBuildMicromapFlagBitsEXT(1 << 2);
pub const VK_BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT: VkBuildMicromapFlagBitsEXT =
  VkBuildMicromapFlagBitsEXT(1 << 1);
pub const VK_BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT: VkBuildMicromapFlagBitsEXT =
  VkBuildMicromapFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkBuildMicromapFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(4, "allow_compaction"), (2, "prefer_fast_build"), (1, "prefer_fast_trace")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkBuildMicromapFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn allow_compaction(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn prefer_fast_build(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn prefer_fast_trace(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkColorComponentFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html) (bitmask)
  VkColorComponentFlagBits(u32)
);
/// Khronos: [VkColorComponentFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html) (bitmask)
pub type VkColorComponentFlags = VkColorComponentFlagBits;
pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlagBits =
  VkColorComponentFlagBits(1 << 3);
pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlagBits =
  VkColorComponentFlagBits(1 << 2);
pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlagBits =
  VkColorComponentFlagBits(1 << 1);
pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlagBits =
  VkColorComponentFlagBits(1 << 0);
impl core::fmt::Debug for VkColorComponentFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(8, "a"), (4, "b"), (2, "g"), (1, "r")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkColorComponentFlagBits {
  #[inline]
  #[must_use]
  pub const fn a(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn b(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn g(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn r(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkCommandBufferResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html) (bitmask)
  VkCommandBufferResetFlagBits(u32)
);
/// Khronos: [VkCommandBufferResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html) (bitmask)
pub type VkCommandBufferResetFlags = VkCommandBufferResetFlagBits;
/// Release resources owned by the buffer
pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: VkCommandBufferResetFlagBits =
  VkCommandBufferResetFlagBits(1 << 0);
impl core::fmt::Debug for VkCommandBufferResetFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "release_resources")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkCommandBufferResetFlagBits {
  #[inline]
  #[must_use]
  pub const fn release_resources(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkCommandBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html) (bitmask)
  VkCommandBufferUsageFlagBits(u32)
);
/// Khronos: [VkCommandBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html) (bitmask)
pub type VkCommandBufferUsageFlags = VkCommandBufferUsageFlagBits;
pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlagBits =
  VkCommandBufferUsageFlagBits(1 << 0);
pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlagBits =
  VkCommandBufferUsageFlagBits(1 << 1);
/// Command buffer may be submitted/executed more than once simultaneously
pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlagBits =
  VkCommandBufferUsageFlagBits(1 << 2);
impl core::fmt::Debug for VkCommandBufferUsageFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "one_time_submit"), (2, "render_pass_continue"), (4, "simultaneous_use")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkCommandBufferUsageFlagBits {
  #[inline]
  #[must_use]
  pub const fn one_time_submit(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn render_pass_continue(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn simultaneous_use(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkCommandPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html) (bitmask)
  VkCommandPoolCreateFlagBits(u32)
);
/// Khronos: [VkCommandPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html) (bitmask)
pub type VkCommandPoolCreateFlags = VkCommandPoolCreateFlagBits;
/// Command buffers allocated from pool are protected command buffers
pub const VK_COMMAND_POOL_CREATE_PROTECTED_BIT: VkCommandPoolCreateFlagBits =
  VkCommandPoolCreateFlagBits(1 << 2);
/// Command buffers may release their memory individually
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlagBits =
  VkCommandPoolCreateFlagBits(1 << 1);
/// Command buffers have a short lifetime
pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkCommandPoolCreateFlagBits =
  VkCommandPoolCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkCommandPoolCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(4, "protected"), (2, "reset_command_buffer"), (1, "transient")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkCommandPoolCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn protected(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reset_command_buffer(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transient(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkCommandPoolResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html) (bitmask)
  VkCommandPoolResetFlagBits(u32)
);
/// Khronos: [VkCommandPoolResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html) (bitmask)
pub type VkCommandPoolResetFlags = VkCommandPoolResetFlagBits;
/// Release resources owned by the pool
pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: VkCommandPoolResetFlagBits =
  VkCommandPoolResetFlagBits(1 << 0);
pub const VK_COMMAND_POOL_RESET_RESERVED_1_BIT_COREAVI: VkCommandPoolResetFlagBits =
  VkCommandPoolResetFlagBits(1 << 1);
impl core::fmt::Debug for VkCommandPoolResetFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "release_resources"), (2, "reserved_1_bit_coreavi")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkCommandPoolResetFlagBits {
  #[inline]
  #[must_use]
  pub const fn release_resources(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_1_bit_coreavi(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkCommandPoolTrimFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolTrimFlags.html) (bitmask, no bits defined)
  VkCommandPoolTrimFlagBits(u32)
);
/// Khronos: [VkCommandPoolTrimFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolTrimFlags.html) (bitmask, no bits defined)
pub type VkCommandPoolTrimFlags = VkCommandPoolTrimFlagBits;
impl core::fmt::Debug for VkCommandPoolTrimFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkCompositeAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html) (bitmask)
  VkCompositeAlphaFlagBitsKHR(u32)
);
/// Khronos: [VkCompositeAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html) (bitmask)
pub type VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagBitsKHR;
pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: VkCompositeAlphaFlagBitsKHR =
  VkCompositeAlphaFlagBitsKHR(1 << 3);
pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagBitsKHR =
  VkCompositeAlphaFlagBitsKHR(1 << 0);
pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR =
  VkCompositeAlphaFlagBitsKHR(1 << 2);
pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR =
  VkCompositeAlphaFlagBitsKHR(1 << 1);
impl core::fmt::Debug for VkCompositeAlphaFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(8, "inherit"), (1, "opaque"), (4, "post_multiplied"), (2, "pre_multiplied")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkCompositeAlphaFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn inherit(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn post_multiplied(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn pre_multiplied(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkConditionalRenderingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html) (bitmask)
  VkConditionalRenderingFlagBitsEXT(u32)
);
/// Khronos: [VkConditionalRenderingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html) (bitmask)
pub type VkConditionalRenderingFlagsEXT = VkConditionalRenderingFlagBitsEXT;
pub const VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT: VkConditionalRenderingFlagBitsEXT =
  VkConditionalRenderingFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkConditionalRenderingFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "inverted")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkConditionalRenderingFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn inverted(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkCullModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html) (bitmask)
  VkCullModeFlagBits(u32)
);
/// Khronos: [VkCullModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html) (bitmask)
pub type VkCullModeFlags = VkCullModeFlagBits;
pub const VK_CULL_MODE_BACK_BIT: VkCullModeFlagBits = VkCullModeFlagBits(1 << 1);
pub const VK_CULL_MODE_FRONT_BIT: VkCullModeFlagBits = VkCullModeFlagBits(1 << 0);
pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlagBits = VkCullModeFlagBits(3);
pub const VK_CULL_MODE_NONE: VkCullModeFlagBits = VkCullModeFlagBits(0);
impl core::fmt::Debug for VkCullModeFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(2, "back"), (1, "front")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkCullModeFlagBits {
  #[inline]
  #[must_use]
  pub const fn back(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn front(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDebugReportFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html) (bitmask)
  VkDebugReportFlagBitsEXT(u32)
);
/// Khronos: [VkDebugReportFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html) (bitmask)
pub type VkDebugReportFlagsEXT = VkDebugReportFlagBitsEXT;
pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT: VkDebugReportFlagBitsEXT =
  VkDebugReportFlagBitsEXT(1 << 4);
pub const VK_DEBUG_REPORT_ERROR_BIT_EXT: VkDebugReportFlagBitsEXT =
  VkDebugReportFlagBitsEXT(1 << 3);
pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkDebugReportFlagBitsEXT =
  VkDebugReportFlagBitsEXT(1 << 0);
pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT =
  VkDebugReportFlagBitsEXT(1 << 2);
pub const VK_DEBUG_REPORT_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT =
  VkDebugReportFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkDebugReportFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (16, "debug"),
      (8, "error"),
      (1, "information"),
      (4, "performance_warning"),
      (2, "warning"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDebugReportFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn debug(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn error(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn information(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn performance_warning(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn warning(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDebugUtilsMessageSeverityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html) (bitmask)
  VkDebugUtilsMessageSeverityFlagBitsEXT(u32)
);
/// Khronos: [VkDebugUtilsMessageSeverityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html) (bitmask)
pub type VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT:
  VkDebugUtilsMessageSeverityFlagBitsEXT =
  VkDebugUtilsMessageSeverityFlagBitsEXT(1 << 12);
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT:
  VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(1 << 4);
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT:
  VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(1 << 0);
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT:
  VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(1 << 8);
impl core::fmt::Debug for VkDebugUtilsMessageSeverityFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(4096, "error"), (16, "info"), (1, "verbose"), (256, "warning")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDebugUtilsMessageSeverityFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn error(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn info(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn verbose(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn warning(self) -> bool {
    (self.0 & 256) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDebugUtilsMessageTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html) (bitmask)
  VkDebugUtilsMessageTypeFlagBitsEXT(u32)
);
/// Khronos: [VkDebugUtilsMessageTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html) (bitmask)
pub type VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagBitsEXT;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_DEVICE_ADDRESS_BINDING_BIT_EXT:
  VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(1 << 3);
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT:
  VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(1 << 0);
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT:
  VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(1 << 2);
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT:
  VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkDebugUtilsMessageTypeFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "device_address_binding"),
      (1, "general"),
      (4, "performance"),
      (2, "validation"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDebugUtilsMessageTypeFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn device_address_binding(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn general(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn performance(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn validation(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDebugUtilsMessengerCallbackDataFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html) (bitmask, no bits defined)
  VkDebugUtilsMessengerCallbackDataFlagBitsEXT(u32)
);
/// Khronos: [VkDebugUtilsMessengerCallbackDataFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html) (bitmask, no bits defined)
pub type VkDebugUtilsMessengerCallbackDataFlagsEXT =
  VkDebugUtilsMessengerCallbackDataFlagBitsEXT;
impl core::fmt::Debug for VkDebugUtilsMessengerCallbackDataFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDebugUtilsMessengerCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateFlagsEXT.html) (bitmask, no bits defined)
  VkDebugUtilsMessengerCreateFlagBitsEXT(u32)
);
/// Khronos: [VkDebugUtilsMessengerCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkDebugUtilsMessengerCreateFlagsEXT = VkDebugUtilsMessengerCreateFlagBitsEXT;
impl core::fmt::Debug for VkDebugUtilsMessengerCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDependencyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html) (bitmask)
  VkDependencyFlagBits(u32)
);
/// Khronos: [VkDependencyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html) (bitmask)
pub type VkDependencyFlags = VkDependencyFlagBits;
/// Dependency is per pixel region
pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlagBits =
  VkDependencyFlagBits(1 << 0);
/// Dependency is across devices
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT: VkDependencyFlagBits =
  VkDependencyFlagBits(1 << 2);
/// Dependency may be a feedback loop
pub const VK_DEPENDENCY_FEEDBACK_LOOP_BIT_EXT: VkDependencyFlagBits =
  VkDependencyFlagBits(1 << 3);
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT: VkDependencyFlagBits =
  VkDependencyFlagBits(1 << 1);
/// Alias of [`VK_DEPENDENCY_DEVICE_GROUP_BIT`]
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR: VkDependencyFlagBits =
  VK_DEPENDENCY_DEVICE_GROUP_BIT;
/// Alias of [`VK_DEPENDENCY_VIEW_LOCAL_BIT`]
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR: VkDependencyFlagBits =
  VK_DEPENDENCY_VIEW_LOCAL_BIT;
impl core::fmt::Debug for VkDependencyFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "by_region"), (4, "device_group"), (8, "feedback_loop"), (2, "view_local")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDependencyFlagBits {
  #[inline]
  #[must_use]
  pub const fn by_region(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn device_group(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn feedback_loop(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn view_local(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDescriptorBindingFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html) (bitmask)
  VkDescriptorBindingFlagBits(u32)
);
/// Khronos: [VkDescriptorBindingFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html) (bitmask)
pub type VkDescriptorBindingFlags = VkDescriptorBindingFlagBits;
pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT: VkDescriptorBindingFlagBits =
  VkDescriptorBindingFlagBits(1 << 2);
pub const VK_DESCRIPTOR_BINDING_RESERVED_4_BIT_QCOM: VkDescriptorBindingFlagBits =
  VkDescriptorBindingFlagBits(1 << 4);
pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT: VkDescriptorBindingFlagBits =
  VkDescriptorBindingFlagBits(1 << 0);
pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT:
  VkDescriptorBindingFlagBits = VkDescriptorBindingFlagBits(1 << 1);
pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT:
  VkDescriptorBindingFlagBits = VkDescriptorBindingFlagBits(1 << 3);
/// Alias of [`VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT`]
pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT: VkDescriptorBindingFlagBits =
  VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT;
/// Alias of [`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`]
pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT: VkDescriptorBindingFlagBits =
  VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT;
/// Alias of [`VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`]
pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT:
  VkDescriptorBindingFlagBits = VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT;
/// Alias of [`VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`]
pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT:
  VkDescriptorBindingFlagBits = VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT;
impl core::fmt::Debug for VkDescriptorBindingFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "partially_bound"),
      (16, "reserved_4"),
      (1, "update_after_bind"),
      (2, "update_unused_while_pending"),
      (8, "variable_descriptor_count"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDescriptorBindingFlagBits {
  #[inline]
  #[must_use]
  pub const fn partially_bound(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_4(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn update_after_bind(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn update_unused_while_pending(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn variable_descriptor_count(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDescriptorPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html) (bitmask)
  VkDescriptorPoolCreateFlagBits(u32)
);
/// Khronos: [VkDescriptorPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html) (bitmask)
pub type VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlagBits;
/// Descriptor sets may be freed individually
pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT:
  VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlagBits(1 << 0);
pub const VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT: VkDescriptorPoolCreateFlagBits =
  VkDescriptorPoolCreateFlagBits(1 << 2);
pub const VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT:
  VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlagBits(1 << 1);
/// Alias of [`VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT`]
pub const VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE: VkDescriptorPoolCreateFlagBits =
  VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT;
/// Alias of [`VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT`]
pub const VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT:
  VkDescriptorPoolCreateFlagBits = VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT;
impl core::fmt::Debug for VkDescriptorPoolCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "free_descriptor_set"), (4, "host_only"), (2, "update_after_bind")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDescriptorPoolCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn free_descriptor_set(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_only(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn update_after_bind(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDescriptorPoolResetFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolResetFlags.html) (bitmask, no bits defined)
  VkDescriptorPoolResetFlagBits(u32)
);
/// Khronos: [VkDescriptorPoolResetFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolResetFlags.html) (bitmask, no bits defined)
pub type VkDescriptorPoolResetFlags = VkDescriptorPoolResetFlagBits;
impl core::fmt::Debug for VkDescriptorPoolResetFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDescriptorSetLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html) (bitmask)
  VkDescriptorSetLayoutCreateFlagBits(u32)
);
/// Khronos: [VkDescriptorSetLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html) (bitmask)
pub type VkDescriptorSetLayoutCreateFlags = VkDescriptorSetLayoutCreateFlagBits;
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_DESCRIPTOR_BUFFER_BIT_EXT:
  VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 4);
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT:
  VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 5);
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT:
  VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 2);
/// Descriptors are pushed via flink:vkCmdPushDescriptorSetKHR
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR:
  VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 0);
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_RESERVED_3_BIT_AMD:
  VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 3);
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_RESERVED_6_BIT_EXT:
  VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 6);
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT:
  VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 1);
/// Alias of [`VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT`]
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE:
  VkDescriptorSetLayoutCreateFlagBits =
  VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT;
/// Alias of [`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`]
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT:
  VkDescriptorSetLayoutCreateFlagBits =
  VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT;
impl core::fmt::Debug for VkDescriptorSetLayoutCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (16, "descriptor_buffer"),
      (32, "embedded_immutable_samplers"),
      (4, "host_only_pool"),
      (1, "push_descriptor"),
      (8, "reserved_3"),
      (64, "reserved_6"),
      (2, "update_after_bind_pool"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDescriptorSetLayoutCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn descriptor_buffer(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn embedded_immutable_samplers(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_only_pool(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn push_descriptor(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_3(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_6(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn update_after_bind_pool(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDescriptorUpdateTemplateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html) (bitmask, no bits defined)
  VkDescriptorUpdateTemplateCreateFlagBits(u32)
);
/// Khronos: [VkDescriptorUpdateTemplateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html) (bitmask, no bits defined)
pub type VkDescriptorUpdateTemplateCreateFlags = VkDescriptorUpdateTemplateCreateFlagBits;
impl core::fmt::Debug for VkDescriptorUpdateTemplateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDeviceAddressBindingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingFlagBitsEXT.html) (bitmask)
  VkDeviceAddressBindingFlagBitsEXT(u32)
);
/// Khronos: [VkDeviceAddressBindingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingFlagBitsEXT.html) (bitmask)
pub type VkDeviceAddressBindingFlagsEXT = VkDeviceAddressBindingFlagBitsEXT;
pub const VK_DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT:
  VkDeviceAddressBindingFlagBitsEXT = VkDeviceAddressBindingFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkDeviceAddressBindingFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "internal_object")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDeviceAddressBindingFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn internal_object(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDeviceCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlags.html) (bitmask, no bits defined)
  VkDeviceCreateFlagBits(u32)
);
/// Khronos: [VkDeviceCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlags.html) (bitmask, no bits defined)
pub type VkDeviceCreateFlags = VkDeviceCreateFlagBits;
impl core::fmt::Debug for VkDeviceCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDeviceDiagnosticsConfigFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html) (bitmask)
  VkDeviceDiagnosticsConfigFlagBitsNV(u32)
);
/// Khronos: [VkDeviceDiagnosticsConfigFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html) (bitmask)
pub type VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagBitsNV;
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV:
  VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagBitsNV(1 << 2);
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV:
  VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagBitsNV(1 << 1);
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV:
  VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagBitsNV(1 << 0);
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV:
  VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagBitsNV(1 << 3);
impl core::fmt::Debug for VkDeviceDiagnosticsConfigFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "enable_automatic_checkpoints"),
      (2, "enable_resource_tracking"),
      (1, "enable_shader_debug_info"),
      (8, "enable_shader_error_reporting"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDeviceDiagnosticsConfigFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn enable_automatic_checkpoints(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn enable_resource_tracking(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn enable_shader_debug_info(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn enable_shader_error_reporting(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDeviceGroupPresentModeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) (bitmask)
  VkDeviceGroupPresentModeFlagBitsKHR(u32)
);
/// Khronos: [VkDeviceGroupPresentModeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) (bitmask)
pub type VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagBitsKHR;
/// Present from local memory
pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR:
  VkDeviceGroupPresentModeFlagBitsKHR = VkDeviceGroupPresentModeFlagBitsKHR(1 << 0);
/// Each physical device presents from local memory
pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR:
  VkDeviceGroupPresentModeFlagBitsKHR = VkDeviceGroupPresentModeFlagBitsKHR(1 << 3);
/// Present from remote memory
pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR:
  VkDeviceGroupPresentModeFlagBitsKHR = VkDeviceGroupPresentModeFlagBitsKHR(1 << 1);
/// Present sum of local and/or remote memory
pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR: VkDeviceGroupPresentModeFlagBitsKHR =
  VkDeviceGroupPresentModeFlagBitsKHR(1 << 2);
impl core::fmt::Debug for VkDeviceGroupPresentModeFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "local"), (8, "local_multi_device"), (2, "remote"), (4, "sum")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDeviceGroupPresentModeFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn local(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn local_multi_device(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn remote(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sum(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDeviceMemoryReportFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportFlagsEXT.html) (bitmask, no bits defined)
  VkDeviceMemoryReportFlagBitsEXT(u32)
);
/// Khronos: [VkDeviceMemoryReportFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportFlagsEXT.html) (bitmask, no bits defined)
pub type VkDeviceMemoryReportFlagsEXT = VkDeviceMemoryReportFlagBitsEXT;
impl core::fmt::Debug for VkDeviceMemoryReportFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDeviceQueueCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html) (bitmask)
  VkDeviceQueueCreateFlagBits(u32)
);
/// Khronos: [VkDeviceQueueCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html) (bitmask)
pub type VkDeviceQueueCreateFlags = VkDeviceQueueCreateFlagBits;
/// Queue is a protected-capable device queue
pub const VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT: VkDeviceQueueCreateFlagBits =
  VkDeviceQueueCreateFlagBits(1 << 0);
pub const VK_DEVICE_QUEUE_CREATE_RESERVED_1_BIT_QCOM: VkDeviceQueueCreateFlagBits =
  VkDeviceQueueCreateFlagBits(1 << 1);
impl core::fmt::Debug for VkDeviceQueueCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "protected"), (2, "reserved_1")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDeviceQueueCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn protected(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_1(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDirectDriverLoadingFlagsLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingFlagsLUNARG.html) (bitmask, no bits defined)
  VkDirectDriverLoadingFlagBitsLUNARG(u32)
);
/// Khronos: [VkDirectDriverLoadingFlagsLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingFlagsLUNARG.html) (bitmask, no bits defined)
pub type VkDirectDriverLoadingFlagsLUNARG = VkDirectDriverLoadingFlagBitsLUNARG;
impl core::fmt::Debug for VkDirectDriverLoadingFlagBitsLUNARG {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDirectFBSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
  VkDirectFBSurfaceCreateFlagBitsEXT(u32)
);
/// Khronos: [VkDirectFBSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkDirectFBSurfaceCreateFlagsEXT = VkDirectFBSurfaceCreateFlagBitsEXT;
impl core::fmt::Debug for VkDirectFBSurfaceCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDisplayModeCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateFlagsKHR.html) (bitmask, no bits defined)
  VkDisplayModeCreateFlagBitsKHR(u32)
);
/// Khronos: [VkDisplayModeCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkDisplayModeCreateFlagsKHR = VkDisplayModeCreateFlagBitsKHR;
impl core::fmt::Debug for VkDisplayModeCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDisplayPlaneAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html) (bitmask)
  VkDisplayPlaneAlphaFlagBitsKHR(u32)
);
/// Khronos: [VkDisplayPlaneAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html) (bitmask)
pub type VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagBitsKHR;
pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR =
  VkDisplayPlaneAlphaFlagBitsKHR(1 << 1);
pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR =
  VkDisplayPlaneAlphaFlagBitsKHR(1 << 0);
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR =
  VkDisplayPlaneAlphaFlagBitsKHR(1 << 2);
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR:
  VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagBitsKHR(1 << 3);
impl core::fmt::Debug for VkDisplayPlaneAlphaFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(2, "global"), (1, "opaque"), (4, "per_pixel"), (8, "per_pixel_premultiplied")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkDisplayPlaneAlphaFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn global(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn per_pixel(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn per_pixel_premultiplied(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkDisplaySurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
  VkDisplaySurfaceCreateFlagBitsKHR(u32)
);
/// Khronos: [VkDisplaySurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkDisplaySurfaceCreateFlagsKHR = VkDisplaySurfaceCreateFlagBitsKHR;
impl core::fmt::Debug for VkDisplaySurfaceCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkEventCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlagBits.html) (bitmask)
  VkEventCreateFlagBits(u32)
);
/// Khronos: [VkEventCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlagBits.html) (bitmask)
pub type VkEventCreateFlags = VkEventCreateFlagBits;
pub const VK_EVENT_CREATE_DEVICE_ONLY_BIT: VkEventCreateFlagBits =
  VkEventCreateFlagBits(1 << 0);
/// Alias of [`VK_EVENT_CREATE_DEVICE_ONLY_BIT`]
pub const VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR: VkEventCreateFlagBits =
  VK_EVENT_CREATE_DEVICE_ONLY_BIT;
impl core::fmt::Debug for VkEventCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "device_only")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkEventCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn device_only(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkExportMetalObjectTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectTypeFlagBitsEXT.html) (bitmask)
  VkExportMetalObjectTypeFlagBitsEXT(u32)
);
/// Khronos: [VkExportMetalObjectTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectTypeFlagBitsEXT.html) (bitmask)
pub type VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagBitsEXT;
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT:
  VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 2);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT:
  VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 1);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT:
  VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 0);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT:
  VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 4);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT:
  VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 5);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT:
  VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 3);
impl core::fmt::Debug for VkExportMetalObjectTypeFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "metal_buffer"),
      (2, "metal_command_queue"),
      (1, "metal_device"),
      (16, "metal_iosurface"),
      (32, "metal_shared_event"),
      (8, "metal_texture"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkExportMetalObjectTypeFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn metal_buffer(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn metal_command_queue(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn metal_device(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn metal_iosurface(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn metal_shared_event(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn metal_texture(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkExternalFenceFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html) (bitmask)
  VkExternalFenceFeatureFlagBits(u32)
);
/// Khronos: [VkExternalFenceFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html) (bitmask)
pub type VkExternalFenceFeatureFlags = VkExternalFenceFeatureFlagBits;
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT: VkExternalFenceFeatureFlagBits =
  VkExternalFenceFeatureFlagBits(1 << 0);
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT: VkExternalFenceFeatureFlagBits =
  VkExternalFenceFeatureFlagBits(1 << 1);
/// Alias of [`VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT`]
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagBits =
  VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT;
/// Alias of [`VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT`]
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagBits =
  VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT;
impl core::fmt::Debug for VkExternalFenceFeatureFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "exportable"), (2, "importable")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkExternalFenceFeatureFlagBits {
  #[inline]
  #[must_use]
  pub const fn exportable(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn importable(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkExternalFenceHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html) (bitmask)
  VkExternalFenceHandleTypeFlagBits(u32)
);
/// Khronos: [VkExternalFenceHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html) (bitmask)
pub type VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlagBits;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalFenceHandleTypeFlagBits =
  VkExternalFenceHandleTypeFlagBits(1 << 0);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT:
  VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 1);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT:
  VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 2);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SCI_SYNC_FENCE_BIT_NV:
  VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 5);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SCI_SYNC_OBJ_BIT_NV:
  VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 4);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalFenceHandleTypeFlagBits =
  VkExternalFenceHandleTypeFlagBits(1 << 3);
/// Alias of [`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT`]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR:
  VkExternalFenceHandleTypeFlagBits = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT;
/// Alias of [`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR:
  VkExternalFenceHandleTypeFlagBits = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT;
/// Alias of [`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR:
  VkExternalFenceHandleTypeFlagBits = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT;
/// Alias of [`VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT`]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR:
  VkExternalFenceHandleTypeFlagBits = VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT;
impl core::fmt::Debug for VkExternalFenceHandleTypeFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "opaque_fd"),
      (2, "opaque_win_32"),
      (4, "opaque_win_32_kmt"),
      (32, "sci_sync_fence"),
      (16, "sci_sync_obj"),
      (8, "sync_fd"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkExternalFenceHandleTypeFlagBits {
  #[inline]
  #[must_use]
  pub const fn opaque_fd(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_win_32(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_win_32_kmt(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sci_sync_fence(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sci_sync_obj(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sync_fd(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkExternalMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html) (bitmask)
  VkExternalMemoryFeatureFlagBits(u32)
);
/// Khronos: [VkExternalMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html) (bitmask)
pub type VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlagBits;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT: VkExternalMemoryFeatureFlagBits =
  VkExternalMemoryFeatureFlagBits(1 << 0);
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT: VkExternalMemoryFeatureFlagBits =
  VkExternalMemoryFeatureFlagBits(1 << 1);
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT: VkExternalMemoryFeatureFlagBits =
  VkExternalMemoryFeatureFlagBits(1 << 2);
/// Alias of [`VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT`]
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR:
  VkExternalMemoryFeatureFlagBits = VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT`]
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagBits =
  VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT`]
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagBits =
  VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT;
impl core::fmt::Debug for VkExternalMemoryFeatureFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "dedicated_only"), (2, "exportable"), (4, "importable")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkExternalMemoryFeatureFlagBits {
  #[inline]
  #[must_use]
  pub const fn dedicated_only(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn exportable(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn importable(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkExternalMemoryFeatureFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html) (bitmask)
  VkExternalMemoryFeatureFlagBitsNV(u32)
);
/// Khronos: [VkExternalMemoryFeatureFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html) (bitmask)
pub type VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagBitsNV;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV:
  VkExternalMemoryFeatureFlagBitsNV = VkExternalMemoryFeatureFlagBitsNV(1 << 0);
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV:
  VkExternalMemoryFeatureFlagBitsNV = VkExternalMemoryFeatureFlagBitsNV(1 << 1);
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV:
  VkExternalMemoryFeatureFlagBitsNV = VkExternalMemoryFeatureFlagBitsNV(1 << 2);
impl core::fmt::Debug for VkExternalMemoryFeatureFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "dedicated_only"), (2, "exportable"), (4, "importable")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkExternalMemoryFeatureFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn dedicated_only(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn exportable(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn importable(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkExternalMemoryHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html) (bitmask)
  VkExternalMemoryHandleTypeFlagBits(u32)
);
/// Khronos: [VkExternalMemoryHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html) (bitmask)
pub type VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlagBits;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 10);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 3);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 4);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 5);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 6);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 9);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 7);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 8);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 0);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 1);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 2);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 12);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_SCI_BUF_BIT_NV:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 13);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA:
  VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 11);
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR:
  VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR:
  VkExternalMemoryHandleTypeFlagBits =
  VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR:
  VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR:
  VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR:
  VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR:
  VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR:
  VkExternalMemoryHandleTypeFlagBits =
  VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT;
impl core::fmt::Debug for VkExternalMemoryHandleTypeFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1024, "android_hardware_buffer"),
      (8, "d_3_d_11_texture"),
      (16, "d_3_d_11_texture_kmt"),
      (32, "d_3_d_12_heap"),
      (64, "d_3_d_12_resource"),
      (512, "dma_buf"),
      (128, "host_allocation"),
      (256, "host_mapped_foreign_memory"),
      (1, "opaque_fd"),
      (2, "opaque_win_32"),
      (4, "opaque_win_32_kmt"),
      (4096, "rdma_address"),
      (8192, "sci_buf"),
      (2048, "zircon_vmo"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkExternalMemoryHandleTypeFlagBits {
  #[inline]
  #[must_use]
  pub const fn android_hardware_buffer(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn d_3_d_11_texture(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn d_3_d_11_texture_kmt(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn d_3_d_12_heap(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn d_3_d_12_resource(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn dma_buf(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_allocation(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_mapped_foreign_memory(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_fd(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_win_32(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_win_32_kmt(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rdma_address(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sci_buf(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn zircon_vmo(self) -> bool {
    (self.0 & 2048) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkExternalMemoryHandleTypeFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html) (bitmask)
  VkExternalMemoryHandleTypeFlagBitsNV(u32)
);
/// Khronos: [VkExternalMemoryHandleTypeFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html) (bitmask)
pub type VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagBitsNV;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV:
  VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagBitsNV(1 << 2);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV:
  VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagBitsNV(1 << 3);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV:
  VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagBitsNV(1 << 0);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV:
  VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagBitsNV(1 << 1);
impl core::fmt::Debug for VkExternalMemoryHandleTypeFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "d_3_d_11_image"),
      (8, "d_3_d_11_image_kmt"),
      (1, "opaque_win_32"),
      (2, "opaque_win_32_kmt"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkExternalMemoryHandleTypeFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn d_3_d_11_image(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn d_3_d_11_image_kmt(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_win_32(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_win_32_kmt(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkExternalSemaphoreFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html) (bitmask)
  VkExternalSemaphoreFeatureFlagBits(u32)
);
/// Khronos: [VkExternalSemaphoreFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html) (bitmask)
pub type VkExternalSemaphoreFeatureFlags = VkExternalSemaphoreFeatureFlagBits;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT:
  VkExternalSemaphoreFeatureFlagBits = VkExternalSemaphoreFeatureFlagBits(1 << 0);
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT:
  VkExternalSemaphoreFeatureFlagBits = VkExternalSemaphoreFeatureFlagBits(1 << 1);
/// Alias of [`VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR:
  VkExternalSemaphoreFeatureFlagBits = VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR:
  VkExternalSemaphoreFeatureFlagBits = VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT;
impl core::fmt::Debug for VkExternalSemaphoreFeatureFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "exportable"), (2, "importable")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkExternalSemaphoreFeatureFlagBits {
  #[inline]
  #[must_use]
  pub const fn exportable(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn importable(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkExternalSemaphoreHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html) (bitmask)
  VkExternalSemaphoreHandleTypeFlagBits(u32)
);
/// Khronos: [VkExternalSemaphoreHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html) (bitmask)
pub type VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlagBits;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT:
  VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 3);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT:
  VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 0);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT:
  VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 1);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT:
  VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 2);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SCI_SYNC_OBJ_BIT_NV:
  VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 5);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT:
  VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 4);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA:
  VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 7);
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT:
  VkExternalSemaphoreHandleTypeFlagBits =
  VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR:
  VkExternalSemaphoreHandleTypeFlagBits =
  VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR:
  VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR:
  VkExternalSemaphoreHandleTypeFlagBits =
  VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR:
  VkExternalSemaphoreHandleTypeFlagBits =
  VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR:
  VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT;
impl core::fmt::Debug for VkExternalSemaphoreHandleTypeFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "d_3_d_12_fence"),
      (1, "opaque_fd"),
      (2, "opaque_win_32"),
      (4, "opaque_win_32_kmt"),
      (32, "sci_sync_obj"),
      (16, "sync_fd"),
      (128, "zircon_event"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkExternalSemaphoreHandleTypeFlagBits {
  #[inline]
  #[must_use]
  pub const fn d_3_d_12_fence(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_fd(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_win_32(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque_win_32_kmt(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sci_sync_obj(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sync_fd(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn zircon_event(self) -> bool {
    (self.0 & 128) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkFenceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html) (bitmask)
  VkFenceCreateFlagBits(u32)
);
/// Khronos: [VkFenceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html) (bitmask)
pub type VkFenceCreateFlags = VkFenceCreateFlagBits;
pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlagBits =
  VkFenceCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkFenceCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "signaled")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkFenceCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn signaled(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkFenceImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html) (bitmask)
  VkFenceImportFlagBits(u32)
);
/// Khronos: [VkFenceImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html) (bitmask)
pub type VkFenceImportFlags = VkFenceImportFlagBits;
pub const VK_FENCE_IMPORT_TEMPORARY_BIT: VkFenceImportFlagBits =
  VkFenceImportFlagBits(1 << 0);
/// Alias of [`VK_FENCE_IMPORT_TEMPORARY_BIT`]
pub const VK_FENCE_IMPORT_TEMPORARY_BIT_KHR: VkFenceImportFlagBits =
  VK_FENCE_IMPORT_TEMPORARY_BIT;
impl core::fmt::Debug for VkFenceImportFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "temporary")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkFenceImportFlagBits {
  #[inline]
  #[must_use]
  pub const fn temporary(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkFormatFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html) (bitmask)
  VkFormatFeatureFlagBits(u32)
);
/// Khronos: [VkFormatFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html) (bitmask)
pub type VkFormatFeatureFlags = VkFormatFeatureFlagBits;
pub const VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR:
  VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 29);
/// Format can be used as the destination image of blits with vkCmdBlitImage
pub const VK_FORMAT_FEATURE_BLIT_DST_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 11);
/// Format can be used as the source image of blits with vkCmdBlitImage
pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 10);
/// Format can be used for color attachment images
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 7);
/// Format supports blending in case it is used for color attachment images
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 8);
/// Format can have cosited rather than midpoint chroma samples
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 23);
/// Format can be used for depth/stencil attachment images
pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 9);
/// Format supports disjoint planes
pub const VK_FORMAT_FEATURE_DISJOINT_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 22);
pub const VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 24);
pub const VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR:
  VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 30);
/// Format can have midpoint rather than cosited chroma samples
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 17);
/// Format can be used for sampled images (SAMPLED_IMAGE and
/// COMBINED_IMAGE_SAMPLER descriptor types)
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 0);
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 13);
/// Format can be filtered with VK_FILTER_LINEAR when being sampled
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 12);
/// Format can be used with min/max reduction filtering
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 16);
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 20);
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 21);
/// Format can be used with linear filtering whilst color conversion is enabled
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT:
  VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 18);
/// Format can have different chroma, min and mag filters
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 19);
/// Format supports atomic operations in case it is used for storage images
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 2);
/// Format can be used for storage images (STORAGE_IMAGE descriptor type)
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 1);
/// Format supports atomic operations in case it is used for storage texel
/// buffers
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 5);
/// Format can be used for storage texel buffers (IBOs)
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 4);
/// Format can be used as the destination image of image transfer commands
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 15);
/// Format can be used as the source image of image transfer commands
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 14);
/// Format can be used for uniform texel buffers (TBOs)
pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 3);
/// Format can be used for vertex buffers (VBOs)
pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 6);
pub const VK_FORMAT_FEATURE_VIDEO_DECODE_DPB_BIT_KHR: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 26);
pub const VK_FORMAT_FEATURE_VIDEO_DECODE_OUTPUT_BIT_KHR: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1 << 25);
/// Alias of [`VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`]
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT;
/// Alias of [`VK_FORMAT_FEATURE_DISJOINT_BIT`]
pub const VK_FORMAT_FEATURE_DISJOINT_BIT_KHR: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_DISJOINT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT`]
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`]
///
/// Format can be filtered with VK_FILTER_CUBIC_IMG when being sampled
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR:
  VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_TRANSFER_DST_BIT`]
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_TRANSFER_DST_BIT;
/// Alias of [`VK_FORMAT_FEATURE_TRANSFER_SRC_BIT`]
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR: VkFormatFeatureFlagBits =
  VK_FORMAT_FEATURE_TRANSFER_SRC_BIT;
impl core::fmt::Debug for VkFormatFeatureFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (536870912, "acceleration_structure_vertex_buffer"),
      (2048, "blit_dst"),
      (1024, "blit_src"),
      (128, "color_attachment"),
      (256, "color_attachment_blend"),
      (8388608, "cosited_chroma_samples"),
      (512, "depth_stencil_attachment"),
      (4194304, "disjoint"),
      (16777216, "fragment_density_map"),
      (1073741824, "fragment_shading_rate_attachment"),
      (131072, "midpoint_chroma_samples"),
      (1, "sampled_image"),
      (8192, "sampled_image_filter_cubic"),
      (4096, "sampled_image_filter_linear"),
      (65536, "sampled_image_filter_minmax"),
      (1048576, "sampled_image_ycbcr_conversion_chroma_reconstruction_explicit"),
      (
        2097152,
        "sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable",
      ),
      (262144, "sampled_image_ycbcr_conversion_linear_filter"),
      (524288, "sampled_image_ycbcr_conversion_separate_reconstruction_filter"),
      (4, "storage_image_atomic"),
      (2, "storage_image"),
      (32, "storage_texel_buffer_atomic"),
      (16, "storage_texel_buffer"),
      (32768, "transfer_dst"),
      (16384, "transfer_src"),
      (8, "uniform_texel_buffer"),
      (64, "vertex_buffer"),
      (67108864, "video_decode_dpb"),
      (33554432, "video_decode_output"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkFormatFeatureFlagBits {
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_vertex_buffer(self) -> bool {
    (self.0 & 536870912) != 0
  }
  #[inline]
  #[must_use]
  pub const fn blit_dst(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn blit_src(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_blend(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cosited_chroma_samples(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn depth_stencil_attachment(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn disjoint(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_map(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shading_rate_attachment(self) -> bool {
    (self.0 & 1073741824) != 0
  }
  #[inline]
  #[must_use]
  pub const fn midpoint_chroma_samples(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_filter_cubic(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_filter_linear(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_filter_minmax(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_ycbcr_conversion_chroma_reconstruction_explicit(
    self,
  ) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable(
    self,
  ) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_ycbcr_conversion_linear_filter(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_ycbcr_conversion_separate_reconstruction_filter(
    self,
  ) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_image_atomic(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_image(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_texel_buffer_atomic(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_texel_buffer(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_dst(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_src(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn uniform_texel_buffer(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_buffer(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_dpb(self) -> bool {
    (self.0 & 67108864) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_output(self) -> bool {
    (self.0 & 33554432) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkFormatFeatureFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits2.html) (bitmask)
  VkFormatFeatureFlagBits2(u64)
);
/// Khronos: [VkFormatFeatureFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits2.html) (bitmask)
pub type VkFormatFeatureFlags2 = VkFormatFeatureFlagBits2;
pub const VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR:
  VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 29);
pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 11);
pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 10);
pub const VK_FORMAT_FEATURE_2_BLOCK_MATCHING_BIT_QCOM: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 36);
pub const VK_FORMAT_FEATURE_2_BOX_FILTER_SAMPLED_BIT_QCOM: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 37);
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 7);
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 8);
pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 23);
pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 9);
pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 22);
pub const VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 24);
pub const VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR:
  VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 30);
/// Format support linear image as render target, it cannot be mixed with non
/// linear attachment
pub const VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 38);
pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 17);
pub const VK_FORMAT_FEATURE_2_OPTICAL_FLOW_COST_BIT_NV: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 42);
pub const VK_FORMAT_FEATURE_2_OPTICAL_FLOW_IMAGE_BIT_NV: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 40);
pub const VK_FORMAT_FEATURE_2_OPTICAL_FLOW_VECTOR_BIT_NV: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 41);
pub const VK_FORMAT_FEATURE_2_RESERVED_39_BIT_EXT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 39);
pub const VK_FORMAT_FEATURE_2_RESERVED_44_BIT_EXT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 44);
pub const VK_FORMAT_FEATURE_2_RESERVED_45_BIT_EXT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 45);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 0);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT:
  VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 33);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 13);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 12);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 16);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 20);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 21);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT:
  VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 18);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 19);
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 2);
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 1);
pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 31);
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 5);
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 4);
pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 32);
pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 15);
pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 14);
pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 3);
pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 6);
pub const VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 26);
pub const VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 25);
pub const VK_FORMAT_FEATURE_2_WEIGHT_IMAGE_BIT_QCOM: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 34);
pub const VK_FORMAT_FEATURE_2_WEIGHT_SAMPLED_IMAGE_BIT_QCOM: VkFormatFeatureFlagBits2 =
  VkFormatFeatureFlagBits2(1 << 35);
/// Alias of [`VK_FORMAT_FEATURE_2_BLIT_DST_BIT`]
pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_BLIT_DST_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_BLIT_SRC_BIT`]
pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_BLIT_SRC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT`]
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT`]
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT`]
pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT`]
pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_DISJOINT_BIT`]
pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_DISJOINT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT`]
pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR:
  VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT:
  VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR:
  VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR:
  VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR:
  VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR:
  VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR:
  VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR:
  VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT`]
pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT`]
pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT`]
pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT`]
pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 =
  VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT;
impl core::fmt::Debug for VkFormatFeatureFlagBits2 {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (536870912, "acceleration_structure_vertex_buffer"),
      (2048, "blit_dst"),
      (1024, "blit_src"),
      (68719476736, "block_matching"),
      (137438953472, "box_filter_sampled"),
      (128, "color_attachment"),
      (256, "color_attachment_blend"),
      (8388608, "cosited_chroma_samples"),
      (512, "depth_stencil_attachment"),
      (4194304, "disjoint"),
      (16777216, "fragment_density_map"),
      (1073741824, "fragment_shading_rate_attachment"),
      (274877906944, "linear_color_attachment"),
      (131072, "midpoint_chroma_samples"),
      (4398046511104, "optical_flow_cost"),
      (1099511627776, "optical_flow_image"),
      (2199023255552, "optical_flow_vector"),
      (549755813888, "reserved_39"),
      (17592186044416, "reserved_44"),
      (35184372088832, "reserved_45"),
      (1, "sampled_image"),
      (8589934592, "sampled_image_depth_comparison"),
      (8192, "sampled_image_filter_cubic"),
      (4096, "sampled_image_filter_linear"),
      (65536, "sampled_image_filter_minmax"),
      (1048576, "sampled_image_ycbcr_conversion_chroma_reconstruction_explicit"),
      (
        2097152,
        "sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable",
      ),
      (262144, "sampled_image_ycbcr_conversion_linear_filter"),
      (524288, "sampled_image_ycbcr_conversion_separate_reconstruction_filter"),
      (4, "storage_image_atomic"),
      (2, "storage_image"),
      (2147483648, "storage_read_without_format"),
      (32, "storage_texel_buffer_atomic"),
      (16, "storage_texel_buffer"),
      (4294967296, "storage_write_without_format"),
      (32768, "transfer_dst"),
      (16384, "transfer_src"),
      (8, "uniform_texel_buffer"),
      (64, "vertex_buffer"),
      (67108864, "video_decode_dpb"),
      (33554432, "video_decode_output"),
      (17179869184, "weight_image"),
      (34359738368, "weight_sampled_image"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkFormatFeatureFlagBits2 {
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_vertex_buffer(self) -> bool {
    (self.0 & 536870912) != 0
  }
  #[inline]
  #[must_use]
  pub const fn blit_dst(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn blit_src(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn block_matching(self) -> bool {
    (self.0 & 68719476736) != 0
  }
  #[inline]
  #[must_use]
  pub const fn box_filter_sampled(self) -> bool {
    (self.0 & 137438953472) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_blend(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cosited_chroma_samples(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn depth_stencil_attachment(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn disjoint(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_map(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shading_rate_attachment(self) -> bool {
    (self.0 & 1073741824) != 0
  }
  #[inline]
  #[must_use]
  pub const fn linear_color_attachment(self) -> bool {
    (self.0 & 274877906944) != 0
  }
  #[inline]
  #[must_use]
  pub const fn midpoint_chroma_samples(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn optical_flow_cost(self) -> bool {
    (self.0 & 4398046511104) != 0
  }
  #[inline]
  #[must_use]
  pub const fn optical_flow_image(self) -> bool {
    (self.0 & 1099511627776) != 0
  }
  #[inline]
  #[must_use]
  pub const fn optical_flow_vector(self) -> bool {
    (self.0 & 2199023255552) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_39(self) -> bool {
    (self.0 & 549755813888) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_44(self) -> bool {
    (self.0 & 17592186044416) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_45(self) -> bool {
    (self.0 & 35184372088832) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_depth_comparison(self) -> bool {
    (self.0 & 8589934592) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_filter_cubic(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_filter_linear(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_filter_minmax(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_ycbcr_conversion_chroma_reconstruction_explicit(
    self,
  ) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable(
    self,
  ) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_ycbcr_conversion_linear_filter(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled_image_ycbcr_conversion_separate_reconstruction_filter(
    self,
  ) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_image_atomic(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_image(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_read_without_format(self) -> bool {
    (self.0 & 2147483648) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_texel_buffer_atomic(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_texel_buffer(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage_write_without_format(self) -> bool {
    (self.0 & 4294967296) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_dst(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_src(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn uniform_texel_buffer(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_buffer(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_dpb(self) -> bool {
    (self.0 & 67108864) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_output(self) -> bool {
    (self.0 & 33554432) != 0
  }
  #[inline]
  #[must_use]
  pub const fn weight_image(self) -> bool {
    (self.0 & 17179869184) != 0
  }
  #[inline]
  #[must_use]
  pub const fn weight_sampled_image(self) -> bool {
    (self.0 & 34359738368) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkFramebufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlagBits.html) (bitmask)
  VkFramebufferCreateFlagBits(u32)
);
/// Khronos: [VkFramebufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlagBits.html) (bitmask)
pub type VkFramebufferCreateFlags = VkFramebufferCreateFlagBits;
pub const VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT: VkFramebufferCreateFlagBits =
  VkFramebufferCreateFlagBits(1 << 0);
/// Alias of [`VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`]
pub const VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR: VkFramebufferCreateFlagBits =
  VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT;
impl core::fmt::Debug for VkFramebufferCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "imageless")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkFramebufferCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn imageless(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkGeometryFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html) (bitmask)
  VkGeometryFlagBitsKHR(u32)
);
/// Khronos: [VkGeometryFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html) (bitmask)
pub type VkGeometryFlagsKHR = VkGeometryFlagBitsKHR;
pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR: VkGeometryFlagBitsKHR =
  VkGeometryFlagBitsKHR(1 << 1);
pub const VK_GEOMETRY_OPAQUE_BIT_KHR: VkGeometryFlagBitsKHR =
  VkGeometryFlagBitsKHR(1 << 0);
/// Alias of [`VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR`]
pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV: VkGeometryFlagBitsKHR =
  VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR;
/// Alias of [`VK_GEOMETRY_OPAQUE_BIT_KHR`]
pub const VK_GEOMETRY_OPAQUE_BIT_NV: VkGeometryFlagBitsKHR = VK_GEOMETRY_OPAQUE_BIT_KHR;
impl core::fmt::Debug for VkGeometryFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(2, "no_duplicate_any_hit_invocation"), (1, "opaque")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkGeometryFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn no_duplicate_any_hit_invocation(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn opaque(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkGeometryInstanceFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html) (bitmask)
  VkGeometryInstanceFlagBitsKHR(u32)
);
/// Khronos: [VkGeometryInstanceFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html) (bitmask)
pub type VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagBitsKHR;
pub const VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_EXT:
  VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 5);
pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR: VkGeometryInstanceFlagBitsKHR =
  VkGeometryInstanceFlagBitsKHR(1 << 3);
pub const VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_EXT:
  VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 4);
pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR: VkGeometryInstanceFlagBitsKHR =
  VkGeometryInstanceFlagBitsKHR(1 << 2);
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR:
  VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 0);
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR:
  VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 1);
/// Alias of [`VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV: VkGeometryInstanceFlagBitsKHR =
  VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR;
/// Alias of [`VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV: VkGeometryInstanceFlagBitsKHR =
  VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR;
/// Alias of [`VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV:
  VkGeometryInstanceFlagBitsKHR =
  VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR;
/// Alias of [`VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR:
  VkGeometryInstanceFlagBitsKHR = VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR;
/// Alias of [`VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV:
  VkGeometryInstanceFlagBitsKHR =
  VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR;
impl core::fmt::Debug for VkGeometryInstanceFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (32, "disable_opacity_micromaps"),
      (8, "force_no_opaque"),
      (16, "force_opacity_micromap_2_state"),
      (4, "force_opaque"),
      (1, "triangle_facing_cull_disable"),
      (2, "triangle_flip_facing"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkGeometryInstanceFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn disable_opacity_micromaps(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn force_no_opaque(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn force_opacity_micromap_2_state(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn force_opaque(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn triangle_facing_cull_disable(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn triangle_flip_facing(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkGraphicsPipelineLibraryFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryFlagBitsEXT.html) (bitmask)
  VkGraphicsPipelineLibraryFlagBitsEXT(u32)
);
/// Khronos: [VkGraphicsPipelineLibraryFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryFlagBitsEXT.html) (bitmask)
pub type VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagBitsEXT;
pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT:
  VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagBitsEXT(1 << 3);
pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT:
  VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagBitsEXT(1 << 2);
pub const VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT:
  VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagBitsEXT(1 << 1);
pub const VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT:
  VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkGraphicsPipelineLibraryFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "fragment_output_interface"),
      (4, "fragment_shader"),
      (2, "pre_rasterization_shaders"),
      (1, "vertex_input_interface"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkGraphicsPipelineLibraryFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn fragment_output_interface(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shader(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn pre_rasterization_shaders(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_input_interface(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkHeadlessSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
  VkHeadlessSurfaceCreateFlagBitsEXT(u32)
);
/// Khronos: [VkHeadlessSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkHeadlessSurfaceCreateFlagsEXT = VkHeadlessSurfaceCreateFlagBitsEXT;
impl core::fmt::Debug for VkHeadlessSurfaceCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkIOSSurfaceCreateFlagsMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateFlagsMVK.html) (bitmask, no bits defined)
  VkIOSSurfaceCreateFlagBitsMVK(u32)
);
/// Khronos: [VkIOSSurfaceCreateFlagsMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateFlagsMVK.html) (bitmask, no bits defined)
pub type VkIOSSurfaceCreateFlagsMVK = VkIOSSurfaceCreateFlagBitsMVK;
impl core::fmt::Debug for VkIOSSurfaceCreateFlagBitsMVK {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkImageAspectFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html) (bitmask)
  VkImageAspectFlagBits(u32)
);
/// Khronos: [VkImageAspectFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html) (bitmask)
pub type VkImageAspectFlags = VkImageAspectFlagBits;
pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 0);
pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 1);
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 7);
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 8);
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 9);
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 10);
pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 3);
pub const VK_IMAGE_ASPECT_PLANE_0_BIT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 4);
pub const VK_IMAGE_ASPECT_PLANE_1_BIT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 5);
pub const VK_IMAGE_ASPECT_PLANE_2_BIT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 6);
pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlagBits =
  VkImageAspectFlagBits(1 << 2);
pub const VK_IMAGE_ASPECT_NONE: VkImageAspectFlagBits = VkImageAspectFlagBits(0);
/// Alias of [`VK_IMAGE_ASPECT_NONE`]
pub const VK_IMAGE_ASPECT_NONE_KHR: VkImageAspectFlagBits = VK_IMAGE_ASPECT_NONE;
/// Alias of [`VK_IMAGE_ASPECT_PLANE_0_BIT`]
pub const VK_IMAGE_ASPECT_PLANE_0_BIT_KHR: VkImageAspectFlagBits =
  VK_IMAGE_ASPECT_PLANE_0_BIT;
/// Alias of [`VK_IMAGE_ASPECT_PLANE_1_BIT`]
pub const VK_IMAGE_ASPECT_PLANE_1_BIT_KHR: VkImageAspectFlagBits =
  VK_IMAGE_ASPECT_PLANE_1_BIT;
/// Alias of [`VK_IMAGE_ASPECT_PLANE_2_BIT`]
pub const VK_IMAGE_ASPECT_PLANE_2_BIT_KHR: VkImageAspectFlagBits =
  VK_IMAGE_ASPECT_PLANE_2_BIT;
impl core::fmt::Debug for VkImageAspectFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "color"),
      (2, "depth"),
      (128, "memory_plane_0"),
      (256, "memory_plane_1"),
      (512, "memory_plane_2"),
      (1024, "memory_plane_3"),
      (8, "metadata"),
      (16, "plane_0"),
      (32, "plane_1"),
      (64, "plane_2"),
      (4, "stencil"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkImageAspectFlagBits {
  #[inline]
  #[must_use]
  pub const fn color(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn depth(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn memory_plane_0(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn memory_plane_1(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn memory_plane_2(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn memory_plane_3(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn metadata(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn plane_0(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn plane_1(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn plane_2(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn stencil(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkImageCompressionFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFlagBitsEXT.html) (bitmask)
  VkImageCompressionFlagBitsEXT(u32)
);
/// Khronos: [VkImageCompressionFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFlagBitsEXT.html) (bitmask)
pub type VkImageCompressionFlagsEXT = VkImageCompressionFlagBitsEXT;
pub const VK_IMAGE_COMPRESSION_DISABLED_EXT: VkImageCompressionFlagBitsEXT =
  VkImageCompressionFlagBitsEXT(1 << 2);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT: VkImageCompressionFlagBitsEXT =
  VkImageCompressionFlagBitsEXT(1 << 0);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT: VkImageCompressionFlagBitsEXT =
  VkImageCompressionFlagBitsEXT(1 << 1);
pub const VK_IMAGE_COMPRESSION_DEFAULT_EXT: VkImageCompressionFlagBitsEXT =
  VkImageCompressionFlagBitsEXT(0);
impl core::fmt::Debug for VkImageCompressionFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(4, "disabled"), (1, "fixed_rate_default"), (2, "fixed_rate_explicit")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkImageCompressionFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn disabled(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fixed_rate_default(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fixed_rate_explicit(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkImageCompressionFixedRateFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFixedRateFlagBitsEXT.html) (bitmask)
  VkImageCompressionFixedRateFlagBitsEXT(u32)
);
/// Khronos: [VkImageCompressionFixedRateFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFixedRateFlagBitsEXT.html) (bitmask)
pub type VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagBitsEXT;
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 9);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 10);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 11);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 12);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 13);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 14);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 15);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 16);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 17);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 18);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 0);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 19);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 20);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 21);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 22);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT =
  VkImageCompressionFixedRateFlagBitsEXT(1 << 23);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 1);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 2);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 3);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 4);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 5);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 6);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 7);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 8);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_NONE_EXT:
  VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(0);
impl core::fmt::Debug for VkImageCompressionFixedRateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (512, "10_bpc"),
      (1024, "11_bpc"),
      (2048, "12_bpc"),
      (4096, "13_bpc"),
      (8192, "14_bpc"),
      (16384, "15_bpc"),
      (32768, "16_bpc"),
      (65536, "17_bpc"),
      (131072, "18_bpc"),
      (262144, "19_bpc"),
      (1, "1_bpc"),
      (524288, "20_bpc"),
      (1048576, "21_bpc"),
      (2097152, "22_bpc"),
      (4194304, "23_bpc"),
      (8388608, "24_bpc"),
      (2, "2_bpc"),
      (4, "3_bpc"),
      (8, "4_bpc"),
      (16, "5_bpc"),
      (32, "6_bpc"),
      (64, "7_bpc"),
      (128, "8_bpc"),
      (256, "9_bpc"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkImageCompressionFixedRateFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn _10_bpc(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _11_bpc(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _12_bpc(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _13_bpc(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _14_bpc(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _15_bpc(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _16_bpc(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _17_bpc(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _18_bpc(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _19_bpc(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _1_bpc(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _20_bpc(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _21_bpc(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _22_bpc(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _23_bpc(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _24_bpc(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _2_bpc(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _3_bpc(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _4_bpc(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _5_bpc(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _6_bpc(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _7_bpc(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _8_bpc(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _9_bpc(self) -> bool {
    (self.0 & 256) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkImageConstraintsInfoFlagBitsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html) (bitmask)
  VkImageConstraintsInfoFlagBitsFUCHSIA(u32)
);
/// Khronos: [VkImageConstraintsInfoFlagBitsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html) (bitmask)
pub type VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA;
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA:
  VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 1);
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA:
  VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 0);
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA:
  VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 3);
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA:
  VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 2);
pub const VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA:
  VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 4);
impl core::fmt::Debug for VkImageConstraintsInfoFlagBitsFUCHSIA {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "cpu_read_often"),
      (1, "cpu_read_rarely"),
      (8, "cpu_write_often"),
      (4, "cpu_write_rarely"),
      (16, "protected_optional"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkImageConstraintsInfoFlagBitsFUCHSIA {
  #[inline]
  #[must_use]
  pub const fn cpu_read_often(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cpu_read_rarely(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cpu_write_often(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cpu_write_rarely(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn protected_optional(self) -> bool {
    (self.0 & 16) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkImageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html) (bitmask)
  VkImageCreateFlagBits(u32)
);
/// Khronos: [VkImageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html) (bitmask)
pub type VkImageCreateFlags = VkImageCreateFlagBits;
/// The 3D image can be viewed as a 2D or 2D array image
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 5);
/// Image is created with a layout where individual slices are capable of being
/// used as 2D images
pub const VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 17);
pub const VK_IMAGE_CREATE_ALIAS_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 10);
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 7);
pub const VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 13);
/// Allows creating image views with cube type from the created image
pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 4);
pub const VK_IMAGE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT:
  VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 16);
pub const VK_IMAGE_CREATE_DISJOINT_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 9);
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 8);
pub const VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 15);
pub const VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT:
  VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 18);
/// Allows image views to have different format than the base image
pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 3);
/// Image requires protected memory
pub const VK_IMAGE_CREATE_PROTECTED_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 11);
pub const VK_IMAGE_CREATE_RESERVED_19_BIT_EXT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 19);
pub const VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT:
  VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 12);
/// Image should support constant data access to physical memory ranges mapped
/// into multiple locations of sparse images
pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 2);
/// Image should support sparse backing
pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 0);
/// Image should support sparse backing with partial residency
pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 1);
/// Allows using VkBindImageMemoryDeviceGroupInfo::pSplitInstanceBindRegions
/// when binding memory to the image
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 6);
pub const VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1 << 14);
/// Alias of [`VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT`]
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits =
  VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT;
/// Alias of [`VK_IMAGE_CREATE_ALIAS_BIT`]
pub const VK_IMAGE_CREATE_ALIAS_BIT_KHR: VkImageCreateFlagBits =
  VK_IMAGE_CREATE_ALIAS_BIT;
/// Alias of [`VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT`]
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits =
  VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT;
/// Alias of [`VK_IMAGE_CREATE_DISJOINT_BIT`]
pub const VK_IMAGE_CREATE_DISJOINT_BIT_KHR: VkImageCreateFlagBits =
  VK_IMAGE_CREATE_DISJOINT_BIT;
/// Alias of [`VK_IMAGE_CREATE_EXTENDED_USAGE_BIT`]
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR: VkImageCreateFlagBits =
  VK_IMAGE_CREATE_EXTENDED_USAGE_BIT;
/// Alias of [`VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT`]
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: VkImageCreateFlagBits =
  VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT;
impl core::fmt::Debug for VkImageCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (32, "2_d_array_compatible"),
      (131072, "2_d_view_compatible"),
      (1024, "alias"),
      (128, "block_texel_view_compatible"),
      (8192, "corner_sampled"),
      (16, "cube_compatible"),
      (65536, "descriptor_buffer_capture_replay"),
      (512, "disjoint"),
      (256, "extended_usage"),
      (32768, "fragment_density_map_offset"),
      (262144, "multisampled_render_to_single_sampled"),
      (8, "mutable_format"),
      (2048, "protected"),
      (524288, "reserved_19"),
      (4096, "sample_locations_compatible_depth"),
      (4, "sparse_aliased"),
      (1, "sparse_binding"),
      (2, "sparse_residency"),
      (64, "split_instance_bind_regions"),
      (16384, "subsampled"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkImageCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn _2_d_array_compatible(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _2_d_view_compatible(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn alias(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn block_texel_view_compatible(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn corner_sampled(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cube_compatible(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn descriptor_buffer_capture_replay(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn disjoint(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn extended_usage(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_map_offset(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn multisampled_render_to_single_sampled(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn mutable_format(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn protected(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_19(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sample_locations_compatible_depth(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sparse_aliased(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sparse_binding(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sparse_residency(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn split_instance_bind_regions(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn subsampled(self) -> bool {
    (self.0 & 16384) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkImageFormatConstraintsFlagsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsFlagsFUCHSIA.html) (bitmask, no bits defined)
  VkImageFormatConstraintsFlagBitsFUCHSIA(u32)
);
/// Khronos: [VkImageFormatConstraintsFlagsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsFlagsFUCHSIA.html) (bitmask, no bits defined)
pub type VkImageFormatConstraintsFlagsFUCHSIA = VkImageFormatConstraintsFlagBitsFUCHSIA;
impl core::fmt::Debug for VkImageFormatConstraintsFlagBitsFUCHSIA {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkImagePipeSurfaceCreateFlagsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateFlagsFUCHSIA.html) (bitmask, no bits defined)
  VkImagePipeSurfaceCreateFlagBitsFUCHSIA(u32)
);
/// Khronos: [VkImagePipeSurfaceCreateFlagsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateFlagsFUCHSIA.html) (bitmask, no bits defined)
pub type VkImagePipeSurfaceCreateFlagsFUCHSIA = VkImagePipeSurfaceCreateFlagBitsFUCHSIA;
impl core::fmt::Debug for VkImagePipeSurfaceCreateFlagBitsFUCHSIA {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkImageUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html) (bitmask)
  VkImageUsageFlagBits(u32)
);
/// Khronos: [VkImageUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html) (bitmask)
pub type VkImageUsageFlags = VkImageUsageFlagBits;
pub const VK_IMAGE_USAGE_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 19);
/// Can be used as framebuffer color attachment
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 4);
/// Can be used as framebuffer depth/stencil attachment
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 5);
pub const VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 9);
pub const VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 8);
/// Can be used as framebuffer input attachment
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 7);
pub const VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 18);
pub const VK_IMAGE_USAGE_RESERVED_16_BIT_QCOM: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 16);
pub const VK_IMAGE_USAGE_RESERVED_17_BIT_QCOM: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 17);
pub const VK_IMAGE_USAGE_RESERVED_22_BIT_EXT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 22);
/// Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor
/// types)
pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 2);
pub const VK_IMAGE_USAGE_SAMPLE_BLOCK_MATCH_BIT_QCOM: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 21);
pub const VK_IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 20);
/// Can be used as storage image (STORAGE_IMAGE descriptor type)
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 3);
/// Can be used as a destination of transfer operations
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 1);
/// Can be used as a source of transfer operations
pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 0);
/// Image data not needed outside of rendering
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 6);
pub const VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 12);
pub const VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 10);
pub const VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1 << 11);
/// Alias of [`VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
pub const VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV: VkImageUsageFlagBits =
  VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
impl core::fmt::Debug for VkImageUsageFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (524288, "attachment_feedback_loop"),
      (16, "color_attachment"),
      (32, "depth_stencil_attachment"),
      (512, "fragment_density_map"),
      (256, "fragment_shading_rate_attachment"),
      (128, "input_attachment"),
      (262144, "invocation_mask"),
      (65536, "reserved_16"),
      (131072, "reserved_17"),
      (4194304, "reserved_22"),
      (4, "sampled"),
      (2097152, "sample_block_match"),
      (1048576, "sample_weight"),
      (8, "storage"),
      (2, "transfer_dst"),
      (1, "transfer_src"),
      (64, "transient_attachment"),
      (4096, "video_decode_dpb"),
      (1024, "video_decode_dst"),
      (2048, "video_decode_src"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkImageUsageFlagBits {
  #[inline]
  #[must_use]
  pub const fn attachment_feedback_loop(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn depth_stencil_attachment(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_map(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shading_rate_attachment(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn input_attachment(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn invocation_mask(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_16(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_17(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_22(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sampled(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sample_block_match(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sample_weight(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn storage(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_dst(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer_src(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transient_attachment(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_dpb(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_dst(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode_src(self) -> bool {
    (self.0 & 2048) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkImageViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html) (bitmask)
  VkImageViewCreateFlagBits(u32)
);
/// Khronos: [VkImageViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html) (bitmask)
pub type VkImageViewCreateFlags = VkImageViewCreateFlagBits;
pub const VK_IMAGE_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT:
  VkImageViewCreateFlagBits = VkImageViewCreateFlagBits(1 << 2);
pub const VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT:
  VkImageViewCreateFlagBits = VkImageViewCreateFlagBits(1 << 1);
pub const VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT:
  VkImageViewCreateFlagBits = VkImageViewCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkImageViewCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "descriptor_buffer_capture_replay"),
      (2, "fragment_density_map_deferred"),
      (1, "fragment_density_map_dynamic"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkImageViewCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn descriptor_buffer_capture_replay(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_map_deferred(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_map_dynamic(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkIndirectCommandsLayoutUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html) (bitmask)
  VkIndirectCommandsLayoutUsageFlagBitsNV(u32)
);
/// Khronos: [VkIndirectCommandsLayoutUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html) (bitmask)
pub type VkIndirectCommandsLayoutUsageFlagsNV = VkIndirectCommandsLayoutUsageFlagBitsNV;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV:
  VkIndirectCommandsLayoutUsageFlagBitsNV =
  VkIndirectCommandsLayoutUsageFlagBitsNV(1 << 0);
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV:
  VkIndirectCommandsLayoutUsageFlagBitsNV =
  VkIndirectCommandsLayoutUsageFlagBitsNV(1 << 1);
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV:
  VkIndirectCommandsLayoutUsageFlagBitsNV =
  VkIndirectCommandsLayoutUsageFlagBitsNV(1 << 2);
impl core::fmt::Debug for VkIndirectCommandsLayoutUsageFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "explicit_preprocess"), (2, "indexed_sequences"), (4, "unordered_sequences")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkIndirectCommandsLayoutUsageFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn explicit_preprocess(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn indexed_sequences(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn unordered_sequences(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkIndirectStateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html) (bitmask)
  VkIndirectStateFlagBitsNV(u32)
);
/// Khronos: [VkIndirectStateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html) (bitmask)
pub type VkIndirectStateFlagsNV = VkIndirectStateFlagBitsNV;
pub const VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV: VkIndirectStateFlagBitsNV =
  VkIndirectStateFlagBitsNV(1 << 0);
impl core::fmt::Debug for VkIndirectStateFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "flag_frontface")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkIndirectStateFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn flag_frontface(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html) (bitmask)
  VkInstanceCreateFlagBits(u32)
);
/// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html) (bitmask)
pub type VkInstanceCreateFlags = VkInstanceCreateFlagBits;
pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR: VkInstanceCreateFlagBits =
  VkInstanceCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkInstanceCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "enumerate_portability")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkInstanceCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn enumerate_portability(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkMacOSSurfaceCreateFlagsMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html) (bitmask, no bits defined)
  VkMacOSSurfaceCreateFlagBitsMVK(u32)
);
/// Khronos: [VkMacOSSurfaceCreateFlagsMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html) (bitmask, no bits defined)
pub type VkMacOSSurfaceCreateFlagsMVK = VkMacOSSurfaceCreateFlagBitsMVK;
impl core::fmt::Debug for VkMacOSSurfaceCreateFlagBitsMVK {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkMemoryAllocateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html) (bitmask)
  VkMemoryAllocateFlagBits(u32)
);
/// Khronos: [VkMemoryAllocateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html) (bitmask)
pub type VkMemoryAllocateFlags = VkMemoryAllocateFlagBits;
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT: VkMemoryAllocateFlagBits =
  VkMemoryAllocateFlagBits(1 << 1);
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkMemoryAllocateFlagBits =
  VkMemoryAllocateFlagBits(1 << 2);
/// Force allocation on specific devices
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT: VkMemoryAllocateFlagBits =
  VkMemoryAllocateFlagBits(1 << 0);
/// Alias of [`VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT`]
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR: VkMemoryAllocateFlagBits =
  VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT;
/// Alias of [`VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`]
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR:
  VkMemoryAllocateFlagBits = VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
/// Alias of [`VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT`]
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR: VkMemoryAllocateFlagBits =
  VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT;
impl core::fmt::Debug for VkMemoryAllocateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(2, "device_address"), (4, "device_address_capture_replay"), (1, "device_mask")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkMemoryAllocateFlagBits {
  #[inline]
  #[must_use]
  pub const fn device_address(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn device_address_capture_replay(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn device_mask(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkMemoryDecompressionMethodFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDecompressionMethodFlagBitsNV.html) (bitmask)
  VkMemoryDecompressionMethodFlagBitsNV(u64)
);
/// Khronos: [VkMemoryDecompressionMethodFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDecompressionMethodFlagBitsNV.html) (bitmask)
pub type VkMemoryDecompressionMethodFlagsNV = VkMemoryDecompressionMethodFlagBitsNV;
pub const VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV:
  VkMemoryDecompressionMethodFlagBitsNV = VkMemoryDecompressionMethodFlagBitsNV(1 << 0);
impl core::fmt::Debug for VkMemoryDecompressionMethodFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "gdeflate_1_0")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkMemoryDecompressionMethodFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn gdeflate_1_0(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkMemoryHeapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html) (bitmask)
  VkMemoryHeapFlagBits(u32)
);
/// Khronos: [VkMemoryHeapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html) (bitmask)
pub type VkMemoryHeapFlags = VkMemoryHeapFlagBits;
/// If set, heap represents device memory
pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT: VkMemoryHeapFlagBits =
  VkMemoryHeapFlagBits(1 << 0);
/// If set, heap allocations allocate multiple instances by default
pub const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT: VkMemoryHeapFlagBits =
  VkMemoryHeapFlagBits(1 << 1);
/// Alias of [`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT`]
pub const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR: VkMemoryHeapFlagBits =
  VK_MEMORY_HEAP_MULTI_INSTANCE_BIT;
impl core::fmt::Debug for VkMemoryHeapFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "device_local"), (2, "multi_instance")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkMemoryHeapFlagBits {
  #[inline]
  #[must_use]
  pub const fn device_local(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn multi_instance(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkMemoryMapFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlags.html) (bitmask, no bits defined)
  VkMemoryMapFlagBits(u32)
);
/// Khronos: [VkMemoryMapFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlags.html) (bitmask, no bits defined)
pub type VkMemoryMapFlags = VkMemoryMapFlagBits;
impl core::fmt::Debug for VkMemoryMapFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkMemoryPropertyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html) (bitmask)
  VkMemoryPropertyFlagBits(u32)
);
/// Khronos: [VkMemoryPropertyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html) (bitmask)
pub type VkMemoryPropertyFlags = VkMemoryPropertyFlagBits;
pub const VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1 << 6);
/// If otherwise stated, then allocate memory on device
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1 << 0);
pub const VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1 << 7);
/// Memory will be cached by the host
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1 << 3);
/// Memory will have i/o coherency. If not set, application may need to use
/// vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to
/// flush/invalidate host cache
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1 << 2);
/// Memory is mappable by host
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1 << 1);
/// Memory may be allocated by the driver when it is required
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1 << 4);
/// Memory is protected
pub const VK_MEMORY_PROPERTY_PROTECTED_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1 << 5);
pub const VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1 << 8);
impl core::fmt::Debug for VkMemoryPropertyFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (64, "device_coherent"),
      (1, "device_local"),
      (128, "device_uncached"),
      (8, "host_cached"),
      (4, "host_coherent"),
      (2, "host_visible"),
      (16, "lazily_allocated"),
      (32, "protected"),
      (256, "rdma_capable"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkMemoryPropertyFlagBits {
  #[inline]
  #[must_use]
  pub const fn device_coherent(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn device_local(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn device_uncached(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_cached(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_coherent(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host_visible(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn lazily_allocated(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn protected(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rdma_capable(self) -> bool {
    (self.0 & 256) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkMetalSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
  VkMetalSurfaceCreateFlagBitsEXT(u32)
);
/// Khronos: [VkMetalSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkMetalSurfaceCreateFlagsEXT = VkMetalSurfaceCreateFlagBitsEXT;
impl core::fmt::Debug for VkMetalSurfaceCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkMicromapCreateFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapCreateFlagBitsEXT.html) (bitmask)
  VkMicromapCreateFlagBitsEXT(u32)
);
/// Khronos: [VkMicromapCreateFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapCreateFlagBitsEXT.html) (bitmask)
pub type VkMicromapCreateFlagsEXT = VkMicromapCreateFlagBitsEXT;
pub const VK_MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT:
  VkMicromapCreateFlagBitsEXT = VkMicromapCreateFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkMicromapCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "device_address_capture_replay")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkMicromapCreateFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn device_address_capture_replay(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkOpticalFlowExecuteFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteFlagBitsNV.html) (bitmask)
  VkOpticalFlowExecuteFlagBitsNV(u32)
);
/// Khronos: [VkOpticalFlowExecuteFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteFlagBitsNV.html) (bitmask)
pub type VkOpticalFlowExecuteFlagsNV = VkOpticalFlowExecuteFlagBitsNV;
pub const VK_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV:
  VkOpticalFlowExecuteFlagBitsNV = VkOpticalFlowExecuteFlagBitsNV(1 << 0);
impl core::fmt::Debug for VkOpticalFlowExecuteFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "disable_temporal_hints")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkOpticalFlowExecuteFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn disable_temporal_hints(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkOpticalFlowGridSizeFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowGridSizeFlagBitsNV.html) (bitmask)
  VkOpticalFlowGridSizeFlagBitsNV(u32)
);
/// Khronos: [VkOpticalFlowGridSizeFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowGridSizeFlagBitsNV.html) (bitmask)
pub type VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagBitsNV;
pub const VK_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV: VkOpticalFlowGridSizeFlagBitsNV =
  VkOpticalFlowGridSizeFlagBitsNV(1 << 0);
pub const VK_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV: VkOpticalFlowGridSizeFlagBitsNV =
  VkOpticalFlowGridSizeFlagBitsNV(1 << 1);
pub const VK_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV: VkOpticalFlowGridSizeFlagBitsNV =
  VkOpticalFlowGridSizeFlagBitsNV(1 << 2);
pub const VK_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV: VkOpticalFlowGridSizeFlagBitsNV =
  VkOpticalFlowGridSizeFlagBitsNV(1 << 3);
pub const VK_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV: VkOpticalFlowGridSizeFlagBitsNV =
  VkOpticalFlowGridSizeFlagBitsNV(0);
impl core::fmt::Debug for VkOpticalFlowGridSizeFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "1_x_1"), (2, "2_x_2"), (4, "4_x_4"), (8, "8_x_8")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkOpticalFlowGridSizeFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn _1_x_1(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _2_x_2(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _4_x_4(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _8_x_8(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkOpticalFlowSessionCreateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateFlagBitsNV.html) (bitmask)
  VkOpticalFlowSessionCreateFlagBitsNV(u32)
);
/// Khronos: [VkOpticalFlowSessionCreateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateFlagBitsNV.html) (bitmask)
pub type VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagBitsNV;
pub const VK_OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV:
  VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 3);
pub const VK_OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV:
  VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 4);
pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV:
  VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 1);
pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV:
  VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 2);
pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV:
  VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 0);
impl core::fmt::Debug for VkOpticalFlowSessionCreateFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "allow_regions"),
      (16, "both_directions"),
      (2, "enable_cost"),
      (4, "enable_global_flow"),
      (1, "enable_hint"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkOpticalFlowSessionCreateFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn allow_regions(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn both_directions(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn enable_cost(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn enable_global_flow(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn enable_hint(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkOpticalFlowUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowUsageFlagBitsNV.html) (bitmask)
  VkOpticalFlowUsageFlagBitsNV(u32)
);
/// Khronos: [VkOpticalFlowUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowUsageFlagBitsNV.html) (bitmask)
pub type VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagBitsNV;
pub const VK_OPTICAL_FLOW_USAGE_COST_BIT_NV: VkOpticalFlowUsageFlagBitsNV =
  VkOpticalFlowUsageFlagBitsNV(1 << 3);
pub const VK_OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV: VkOpticalFlowUsageFlagBitsNV =
  VkOpticalFlowUsageFlagBitsNV(1 << 4);
pub const VK_OPTICAL_FLOW_USAGE_HINT_BIT_NV: VkOpticalFlowUsageFlagBitsNV =
  VkOpticalFlowUsageFlagBitsNV(1 << 2);
pub const VK_OPTICAL_FLOW_USAGE_INPUT_BIT_NV: VkOpticalFlowUsageFlagBitsNV =
  VkOpticalFlowUsageFlagBitsNV(1 << 0);
pub const VK_OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV: VkOpticalFlowUsageFlagBitsNV =
  VkOpticalFlowUsageFlagBitsNV(1 << 1);
pub const VK_OPTICAL_FLOW_USAGE_UNKNOWN_NV: VkOpticalFlowUsageFlagBitsNV =
  VkOpticalFlowUsageFlagBitsNV(0);
impl core::fmt::Debug for VkOpticalFlowUsageFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(8, "cost"), (16, "global_flow"), (4, "hint"), (1, "input"), (2, "output")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkOpticalFlowUsageFlagBitsNV {
  #[inline]
  #[must_use]
  pub const fn cost(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn global_flow(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn hint(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn input(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn output(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPeerMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html) (bitmask)
  VkPeerMemoryFeatureFlagBits(u32)
);
/// Khronos: [VkPeerMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html) (bitmask)
pub type VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlagBits;
/// Can write with vkCmdCopy commands
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT: VkPeerMemoryFeatureFlagBits =
  VkPeerMemoryFeatureFlagBits(1 << 1);
/// Can read with vkCmdCopy commands
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT: VkPeerMemoryFeatureFlagBits =
  VkPeerMemoryFeatureFlagBits(1 << 0);
/// Can write with and access type/command
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT: VkPeerMemoryFeatureFlagBits =
  VkPeerMemoryFeatureFlagBits(1 << 3);
/// Can read with any access type/command
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT: VkPeerMemoryFeatureFlagBits =
  VkPeerMemoryFeatureFlagBits(1 << 2);
/// Alias of [`VK_PEER_MEMORY_FEATURE_COPY_DST_BIT`]
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR: VkPeerMemoryFeatureFlagBits =
  VK_PEER_MEMORY_FEATURE_COPY_DST_BIT;
/// Alias of [`VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT`]
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBits =
  VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT;
/// Alias of [`VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT`]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR: VkPeerMemoryFeatureFlagBits =
  VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT;
/// Alias of [`VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT`]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBits =
  VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT;
impl core::fmt::Debug for VkPeerMemoryFeatureFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(2, "copy_dst"), (1, "copy_src"), (8, "generic_dst"), (4, "generic_src")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPeerMemoryFeatureFlagBits {
  #[inline]
  #[must_use]
  pub const fn copy_dst(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn copy_src(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn generic_dst(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn generic_src(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPerformanceCounterDescriptionFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html) (bitmask)
  VkPerformanceCounterDescriptionFlagBitsKHR(u32)
);
/// Khronos: [VkPerformanceCounterDescriptionFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html) (bitmask)
pub type VkPerformanceCounterDescriptionFlagsKHR =
  VkPerformanceCounterDescriptionFlagBitsKHR;
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR:
  VkPerformanceCounterDescriptionFlagBitsKHR =
  VkPerformanceCounterDescriptionFlagBitsKHR(1 << 1);
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR:
  VkPerformanceCounterDescriptionFlagBitsKHR =
  VkPerformanceCounterDescriptionFlagBitsKHR(1 << 0);
/// Alias of [`VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR`]
#[deprecated = "aliased"]
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR:
  VkPerformanceCounterDescriptionFlagBitsKHR =
  VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR;
/// Alias of [`VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR`]
#[deprecated = "aliased"]
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR:
  VkPerformanceCounterDescriptionFlagBitsKHR =
  VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR;
impl core::fmt::Debug for VkPerformanceCounterDescriptionFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(2, "concurrently_impacted"), (1, "performance_impacting")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPerformanceCounterDescriptionFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn concurrently_impacted(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn performance_impacting(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCacheCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html) (bitmask)
  VkPipelineCacheCreateFlagBits(u32)
);
/// Khronos: [VkPipelineCacheCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html) (bitmask)
pub type VkPipelineCacheCreateFlags = VkPipelineCacheCreateFlagBits;
pub const VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT:
  VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlagBits(1 << 0);
/// Alias of [`VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`]
pub const VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT:
  VkPipelineCacheCreateFlagBits = VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT;
impl core::fmt::Debug for VkPipelineCacheCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "externally_synchronized")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPipelineCacheCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn externally_synchronized(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineColorBlendStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html) (bitmask)
  VkPipelineColorBlendStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineColorBlendStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html) (bitmask)
pub type VkPipelineColorBlendStateCreateFlags = VkPipelineColorBlendStateCreateFlagBits;
pub const VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT: VkPipelineColorBlendStateCreateFlagBits = VkPipelineColorBlendStateCreateFlagBits(1 << 0);
/// Alias of [`VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT`]
pub const VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM: VkPipelineColorBlendStateCreateFlagBits = VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT;
impl core::fmt::Debug for VkPipelineColorBlendStateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "rasterization_order_attachment_access")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPipelineColorBlendStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn rasterization_order_attachment_access(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCompilerControlFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html) (bitmask)
  VkPipelineCompilerControlFlagBitsAMD(u32)
);
/// Khronos: [VkPipelineCompilerControlFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html) (bitmask)
pub type VkPipelineCompilerControlFlagsAMD = VkPipelineCompilerControlFlagBitsAMD;
impl core::fmt::Debug for VkPipelineCompilerControlFlagBitsAMD {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCoverageModulationStateCreateFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateFlagsNV.html) (bitmask, no bits defined)
  VkPipelineCoverageModulationStateCreateFlagBitsNV(u32)
);
/// Khronos: [VkPipelineCoverageModulationStateCreateFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateFlagsNV.html) (bitmask, no bits defined)
pub type VkPipelineCoverageModulationStateCreateFlagsNV =
  VkPipelineCoverageModulationStateCreateFlagBitsNV;
impl core::fmt::Debug for VkPipelineCoverageModulationStateCreateFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCoverageReductionStateCreateFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageReductionStateCreateFlagsNV.html) (bitmask, no bits defined)
  VkPipelineCoverageReductionStateCreateFlagBitsNV(u32)
);
/// Khronos: [VkPipelineCoverageReductionStateCreateFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageReductionStateCreateFlagsNV.html) (bitmask, no bits defined)
pub type VkPipelineCoverageReductionStateCreateFlagsNV =
  VkPipelineCoverageReductionStateCreateFlagBitsNV;
impl core::fmt::Debug for VkPipelineCoverageReductionStateCreateFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCoverageToColorStateCreateFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateFlagsNV.html) (bitmask, no bits defined)
  VkPipelineCoverageToColorStateCreateFlagBitsNV(u32)
);
/// Khronos: [VkPipelineCoverageToColorStateCreateFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateFlagsNV.html) (bitmask, no bits defined)
pub type VkPipelineCoverageToColorStateCreateFlagsNV =
  VkPipelineCoverageToColorStateCreateFlagBitsNV;
impl core::fmt::Debug for VkPipelineCoverageToColorStateCreateFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits.html) (bitmask)
  ///
  /// Note that the gap at bitpos 10 is unused, and can be reserved
  VkPipelineCreateFlagBits(u32)
);
/// Khronos: [VkPipelineCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits.html) (bitmask)
pub type VkPipelineCreateFlags = VkPipelineCreateFlagBits;
pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 1);
pub const VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 7);
pub const VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 6);
pub const VK_PIPELINE_CREATE_COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 25);
pub const VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 5);
pub const VK_PIPELINE_CREATE_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 26);
pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 2);
pub const VK_PIPELINE_CREATE_DESCRIPTOR_BUFFER_BIT_EXT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 29);
pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 0);
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_BIT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 4);
pub const VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 9);
pub const VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 8);
pub const VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 18);
pub const VK_PIPELINE_CREATE_LIBRARY_BIT_KHR: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 11);
pub const VK_PIPELINE_CREATE_LINK_TIME_OPTIMIZATION_BIT_EXT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 10);
pub const VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT_EXT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 27);
pub const VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT_EXT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 30);
pub const VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 20);
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 14);
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 15);
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 17);
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 16);
pub const VK_PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 24);
pub const VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 19);
pub const VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 13);
pub const VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 12);
pub const VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 22);
pub const VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 21);
pub const VK_PIPELINE_CREATE_RESERVED_BIT_28_NV: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 28);
pub const VK_PIPELINE_CREATE_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT:
  VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 23);
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1 << 3);
/// Alias of [`VK_PIPELINE_CREATE_DISPATCH_BASE_BIT`]
pub const VK_PIPELINE_CREATE_DISPATCH_BASE: VkPipelineCreateFlagBits =
  VK_PIPELINE_CREATE_DISPATCH_BASE_BIT;
/// Alias of [`VK_PIPELINE_CREATE_DISPATCH_BASE`]
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_KHR: VkPipelineCreateFlagBits =
  VK_PIPELINE_CREATE_DISPATCH_BASE;
/// Alias of [`VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`]
pub const VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT: VkPipelineCreateFlagBits =
  VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT;
/// Alias of [`VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT`]
pub const VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT:
  VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT;
/// Alias of [`VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT`]
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR:
  VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT;
/// Alias of [`VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`]
#[deprecated = "aliased"]
pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT;
/// Alias of [`VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
#[deprecated = "aliased"]
pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
impl core::fmt::Debug for VkPipelineCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "allow_derivatives"),
      (128, "capture_internal_representations"),
      (64, "capture_statistics"),
      (33554432, "color_attachment_feedback_loop"),
      (32, "defer_compile"),
      (67108864, "depth_stencil_attachment_feedback_loop"),
      (4, "derivative"),
      (536870912, "descriptor_buffer"),
      (1, "disable_optimization"),
      (16, "dispatch_base"),
      (512, "early_return_on_failure"),
      (256, "fail_on_pipeline_compile_required"),
      (262144, "indirect_bindable"),
      (2048, "library"),
      (1024, "link_time_optimization"),
      (134217728, "no_protected_access"),
      (1073741824, "protected_access_only"),
      (1048576, "ray_tracing_allow_motion"),
      (16384, "ray_tracing_no_null_any_hit_shaders"),
      (32768, "ray_tracing_no_null_closest_hit_shaders"),
      (131072, "ray_tracing_no_null_intersection_shaders"),
      (65536, "ray_tracing_no_null_miss_shaders"),
      (16777216, "ray_tracing_opacity_micromap"),
      (524288, "ray_tracing_shader_group_handle_capture_replay"),
      (8192, "ray_tracing_skip_aabbs"),
      (4096, "ray_tracing_skip_triangles"),
      (4194304, "rendering_fragment_density_map_attachment"),
      (2097152, "rendering_fragment_shading_rate_attachment"),
      (268435456, "reserved_bit_28"),
      (8388608, "retain_link_time_optimization_info"),
      (8, "view_index_from_device_index"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPipelineCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn allow_derivatives(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn capture_internal_representations(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn capture_statistics(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_feedback_loop(self) -> bool {
    (self.0 & 33554432) != 0
  }
  #[inline]
  #[must_use]
  pub const fn defer_compile(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn depth_stencil_attachment_feedback_loop(self) -> bool {
    (self.0 & 67108864) != 0
  }
  #[inline]
  #[must_use]
  pub const fn derivative(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn descriptor_buffer(self) -> bool {
    (self.0 & 536870912) != 0
  }
  #[inline]
  #[must_use]
  pub const fn disable_optimization(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn dispatch_base(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn early_return_on_failure(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fail_on_pipeline_compile_required(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn indirect_bindable(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn library(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn link_time_optimization(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn no_protected_access(self) -> bool {
    (self.0 & 134217728) != 0
  }
  #[inline]
  #[must_use]
  pub const fn protected_access_only(self) -> bool {
    (self.0 & 1073741824) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_allow_motion(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_no_null_any_hit_shaders(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_no_null_closest_hit_shaders(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_no_null_intersection_shaders(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_no_null_miss_shaders(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_opacity_micromap(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_shader_group_handle_capture_replay(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_skip_aabbs(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_skip_triangles(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rendering_fragment_density_map_attachment(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rendering_fragment_shading_rate_attachment(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_bit_28(self) -> bool {
    (self.0 & 268435456) != 0
  }
  #[inline]
  #[must_use]
  pub const fn retain_link_time_optimization_info(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn view_index_from_device_index(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCreationFeedbackFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackFlagBits.html) (bitmask)
  VkPipelineCreationFeedbackFlagBits(u32)
);
/// Khronos: [VkPipelineCreationFeedbackFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackFlagBits.html) (bitmask)
pub type VkPipelineCreationFeedbackFlags = VkPipelineCreationFeedbackFlagBits;
pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT:
  VkPipelineCreationFeedbackFlagBits = VkPipelineCreationFeedbackFlagBits(1 << 1);
pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT:
  VkPipelineCreationFeedbackFlagBits = VkPipelineCreationFeedbackFlagBits(1 << 2);
pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT: VkPipelineCreationFeedbackFlagBits =
  VkPipelineCreationFeedbackFlagBits(1 << 0);
/// Alias of [`VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT`]
pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT:
  VkPipelineCreationFeedbackFlagBits =
  VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT;
/// Alias of [`VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT`]
pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT:
  VkPipelineCreationFeedbackFlagBits =
  VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT;
/// Alias of [`VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT`]
pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT:
  VkPipelineCreationFeedbackFlagBits = VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT;
impl core::fmt::Debug for VkPipelineCreationFeedbackFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "application_pipeline_cache_hit"),
      (4, "base_pipeline_acceleration"),
      (1, "valid"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPipelineCreationFeedbackFlagBits {
  #[inline]
  #[must_use]
  pub const fn application_pipeline_cache_hit(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn base_pipeline_acceleration(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn valid(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineDepthStencilStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html) (bitmask)
  VkPipelineDepthStencilStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineDepthStencilStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html) (bitmask)
pub type VkPipelineDepthStencilStateCreateFlags =
  VkPipelineDepthStencilStateCreateFlagBits;
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT: VkPipelineDepthStencilStateCreateFlagBits = VkPipelineDepthStencilStateCreateFlagBits(1 << 0);
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT: VkPipelineDepthStencilStateCreateFlagBits = VkPipelineDepthStencilStateCreateFlagBits(1 << 1);
/// Alias of [`VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT`]
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: VkPipelineDepthStencilStateCreateFlagBits = VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT;
/// Alias of [`VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT`]
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: VkPipelineDepthStencilStateCreateFlagBits = VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT;
impl core::fmt::Debug for VkPipelineDepthStencilStateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "rasterization_order_attachment_depth_access"),
      (2, "rasterization_order_attachment_stencil_access"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPipelineDepthStencilStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn rasterization_order_attachment_depth_access(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rasterization_order_attachment_stencil_access(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineDiscardRectangleStateCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html) (bitmask, no bits defined)
  VkPipelineDiscardRectangleStateCreateFlagBitsEXT(u32)
);
/// Khronos: [VkPipelineDiscardRectangleStateCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkPipelineDiscardRectangleStateCreateFlagsEXT =
  VkPipelineDiscardRectangleStateCreateFlagBitsEXT;
impl core::fmt::Debug for VkPipelineDiscardRectangleStateCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineDynamicStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateFlags.html) (bitmask, no bits defined)
  VkPipelineDynamicStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineDynamicStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateFlags.html) (bitmask, no bits defined)
pub type VkPipelineDynamicStateCreateFlags = VkPipelineDynamicStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineDynamicStateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineInputAssemblyStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateFlags.html) (bitmask, no bits defined)
  VkPipelineInputAssemblyStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineInputAssemblyStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateFlags.html) (bitmask, no bits defined)
pub type VkPipelineInputAssemblyStateCreateFlags =
  VkPipelineInputAssemblyStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineInputAssemblyStateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html) (bitmask)
  VkPipelineLayoutCreateFlagBits(u32)
);
/// Khronos: [VkPipelineLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html) (bitmask)
pub type VkPipelineLayoutCreateFlags = VkPipelineLayoutCreateFlagBits;
pub const VK_PIPELINE_LAYOUT_CREATE_INDEPENDENT_SETS_BIT_EXT:
  VkPipelineLayoutCreateFlagBits = VkPipelineLayoutCreateFlagBits(1 << 1);
pub const VK_PIPELINE_LAYOUT_CREATE_RESERVED_0_BIT_AMD: VkPipelineLayoutCreateFlagBits =
  VkPipelineLayoutCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkPipelineLayoutCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(2, "independent_sets"), (1, "reserved_0")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPipelineLayoutCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn independent_sets(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_0(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineMultisampleStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateFlags.html) (bitmask, no bits defined)
  VkPipelineMultisampleStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineMultisampleStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateFlags.html) (bitmask, no bits defined)
pub type VkPipelineMultisampleStateCreateFlags = VkPipelineMultisampleStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineMultisampleStateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineRasterizationConservativeStateCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html) (bitmask, no bits defined)
  VkPipelineRasterizationConservativeStateCreateFlagBitsEXT(u32)
);
/// Khronos: [VkPipelineRasterizationConservativeStateCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkPipelineRasterizationConservativeStateCreateFlagsEXT =
  VkPipelineRasterizationConservativeStateCreateFlagBitsEXT;
impl core::fmt::Debug for VkPipelineRasterizationConservativeStateCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineRasterizationDepthClipStateCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html) (bitmask, no bits defined)
  VkPipelineRasterizationDepthClipStateCreateFlagBitsEXT(u32)
);
/// Khronos: [VkPipelineRasterizationDepthClipStateCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkPipelineRasterizationDepthClipStateCreateFlagsEXT =
  VkPipelineRasterizationDepthClipStateCreateFlagBitsEXT;
impl core::fmt::Debug for VkPipelineRasterizationDepthClipStateCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineRasterizationStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateFlags.html) (bitmask, no bits defined)
  VkPipelineRasterizationStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineRasterizationStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateFlags.html) (bitmask, no bits defined)
pub type VkPipelineRasterizationStateCreateFlags =
  VkPipelineRasterizationStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineRasterizationStateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineRasterizationStateStreamCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateFlagsEXT.html) (bitmask, no bits defined)
  VkPipelineRasterizationStateStreamCreateFlagBitsEXT(u32)
);
/// Khronos: [VkPipelineRasterizationStateStreamCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkPipelineRasterizationStateStreamCreateFlagsEXT =
  VkPipelineRasterizationStateStreamCreateFlagBitsEXT;
impl core::fmt::Debug for VkPipelineRasterizationStateStreamCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineShaderStageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html) (bitmask)
  VkPipelineShaderStageCreateFlagBits(u32)
);
/// Khronos: [VkPipelineShaderStageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html) (bitmask)
pub type VkPipelineShaderStageCreateFlags = VkPipelineShaderStageCreateFlagBits;
pub const VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT:
  VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlagBits(1 << 0);
pub const VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT:
  VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlagBits(1 << 1);
pub const VK_PIPELINE_SHADER_STAGE_CREATE_RESERVED_3_BIT_KHR:
  VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlagBits(1 << 3);
/// Alias of [`VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT`]
pub const VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT:
  VkPipelineShaderStageCreateFlagBits =
  VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT;
/// Alias of [`VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT`]
pub const VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT:
  VkPipelineShaderStageCreateFlagBits =
  VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT;
impl core::fmt::Debug for VkPipelineShaderStageCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "allow_varying_subgroup_size"),
      (2, "require_full_subgroups"),
      (8, "reserved_3"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPipelineShaderStageCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn allow_varying_subgroup_size(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn require_full_subgroups(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_3(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html) (bitmask)
  VkPipelineStageFlagBits(u32)
);
/// Khronos: [VkPipelineStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html) (bitmask)
pub type VkPipelineStageFlags = VkPipelineStageFlagBits;
pub const VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR:
  VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 25);
/// All stages supported on the queue
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 16);
/// All stages of the graphics pipeline
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 15);
/// After previous commands have completed
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 13);
/// Color attachment writes
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 10);
pub const VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 17);
/// Compute shading
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 11);
/// A pipeline stage for conditional rendering predicate fetch
pub const VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 18);
/// Draw/DispatchIndirect command fetch
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 1);
/// Early fragment (depth and stencil) tests
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 8);
pub const VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 23);
/// Fragment shading
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 7);
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR:
  VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 22);
/// Geometry shading
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 6);
/// Indicates host (CPU) is a source/sink of the dependency
pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 14);
/// Late fragment (depth and stencil) tests
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 9);
pub const VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 20);
pub const VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 21);
pub const VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 19);
/// Tessellation control shading
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 4);
/// Tessellation evaluation shading
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 5);
/// Before subsequent commands are processed
pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 0);
/// Transfer/copy operations
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 12);
pub const VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 24);
/// Vertex/index fetch
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 2);
/// Vertex shading
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1 << 3);
pub const VK_PIPELINE_STAGE_NONE: VkPipelineStageFlagBits = VkPipelineStageFlagBits(0);
/// Alias of [`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`]
pub const VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV: VkPipelineStageFlagBits =
  VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT`]
pub const VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV: VkPipelineStageFlagBits =
  VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT;
/// Alias of [`VK_PIPELINE_STAGE_NONE`]
pub const VK_PIPELINE_STAGE_NONE_KHR: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_NONE;
/// Alias of [`VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR`]
pub const VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV: VkPipelineStageFlagBits =
  VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
pub const VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV: VkPipelineStageFlagBits =
  VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT`]
pub const VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV: VkPipelineStageFlagBits =
  VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT;
impl core::fmt::Debug for VkPipelineStageFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (33554432, "acceleration_structure_build"),
      (65536, "all_commands"),
      (32768, "all_graphics"),
      (8192, "bottom_of_pipe"),
      (1024, "color_attachment_output"),
      (131072, "command_preprocess"),
      (2048, "compute_shader"),
      (262144, "conditional_rendering"),
      (2, "draw_indirect"),
      (256, "early_fragment_tests"),
      (8388608, "fragment_density_process"),
      (128, "fragment_shader"),
      (4194304, "fragment_shading_rate_attachment"),
      (64, "geometry_shader"),
      (16384, "host"),
      (512, "late_fragment_tests"),
      (1048576, "mesh_shader"),
      (2097152, "ray_tracing_shader"),
      (524288, "task_shader"),
      (16, "tessellation_control_shader"),
      (32, "tessellation_evaluation_shader"),
      (1, "top_of_pipe"),
      (4096, "transfer"),
      (16777216, "transform_feedback"),
      (4, "vertex_input"),
      (8, "vertex_shader"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPipelineStageFlagBits {
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_build(self) -> bool {
    (self.0 & 33554432) != 0
  }
  #[inline]
  #[must_use]
  pub const fn all_commands(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn all_graphics(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn bottom_of_pipe(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_output(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn command_preprocess(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn compute_shader(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn conditional_rendering(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn draw_indirect(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn early_fragment_tests(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_process(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shader(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shading_rate_attachment(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn geometry_shader(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn late_fragment_tests(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn mesh_shader(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_shader(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn task_shader(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn tessellation_control_shader(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn tessellation_evaluation_shader(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn top_of_pipe(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_input(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_shader(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineStageFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html) (bitmask)
  VkPipelineStageFlagBits2(u64)
);
/// Khronos: [VkPipelineStageFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html) (bitmask)
pub type VkPipelineStageFlags2 = VkPipelineStageFlagBits2;
pub const VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR:
  VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 25);
pub const VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_COPY_BIT_KHR:
  VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 28);
pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 16);
pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 15);
pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 12);
pub const VK_PIPELINE_STAGE_2_BLIT_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 34);
pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 13);
pub const VK_PIPELINE_STAGE_2_CLEAR_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 35);
pub const VK_PIPELINE_STAGE_2_CLUSTER_CULLING_SHADER_BIT_HUAWEI:
  VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 41);
pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 10);
pub const VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 17);
pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 11);
/// A pipeline stage for conditional rendering predicate fetch
pub const VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 18);
pub const VK_PIPELINE_STAGE_2_COPY_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 32);
pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 1);
pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 8);
pub const VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 23);
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 7);
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR:
  VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 22);
pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 6);
pub const VK_PIPELINE_STAGE_2_HOST_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 14);
pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 36);
pub const VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 40);
pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 9);
pub const VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 20);
pub const VK_PIPELINE_STAGE_2_MICROMAP_BUILD_BIT_EXT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 30);
pub const VK_PIPELINE_STAGE_2_OPTICAL_FLOW_BIT_NV: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 29);
pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 38);
pub const VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 21);
pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 33);
pub const VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 39);
pub const VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 19);
pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 4);
pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT:
  VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 5);
pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 0);
pub const VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 24);
pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 37);
pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 2);
pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 3);
pub const VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(1 << 26);
pub const VK_PIPELINE_STAGE_2_NONE: VkPipelineStageFlagBits2 =
  VkPipelineStageFlagBits2(0);
/// Alias of [`VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`]
pub const VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV:
  VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`]
pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`]
pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`]
pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_BLIT_BIT`]
pub const VK_PIPELINE_STAGE_2_BLIT_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_BLIT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT`]
pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_CLEAR_BIT`]
pub const VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_CLEAR_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT`]
pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_COPY_BIT`]
pub const VK_PIPELINE_STAGE_2_COPY_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_COPY_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`]
pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`]
pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_HOST_BIT`]
pub const VK_PIPELINE_STAGE_2_HOST_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_HOST_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`]
pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`]
pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT`]
pub const VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT;
/// Alias of [`VK_PIPELINE_STAGE_2_NONE`]
pub const VK_PIPELINE_STAGE_2_NONE_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_NONE;
/// Alias of [`VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT`]
pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR:
  VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`]
pub const VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_2_RESOLVE_BIT`]
pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_RESOLVE_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
pub const VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT`]
pub const VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT;
/// Alias of [`VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR:
  VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR:
  VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT`]
pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR`]
pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`]
pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`]
pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`]
pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR: VkPipelineStageFlagBits2 =
  VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT;
impl core::fmt::Debug for VkPipelineStageFlagBits2 {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (33554432, "acceleration_structure_build"),
      (268435456, "acceleration_structure_copy"),
      (65536, "all_commands"),
      (32768, "all_graphics"),
      (4096, "all_transfer"),
      (17179869184, "blit"),
      (8192, "bottom_of_pipe"),
      (34359738368, "clear"),
      (2199023255552, "cluster_culling_shader"),
      (1024, "color_attachment_output"),
      (131072, "command_preprocess"),
      (2048, "compute_shader"),
      (262144, "conditional_rendering"),
      (4294967296, "copy"),
      (2, "draw_indirect"),
      (256, "early_fragment_tests"),
      (8388608, "fragment_density_process"),
      (128, "fragment_shader"),
      (4194304, "fragment_shading_rate_attachment"),
      (64, "geometry_shader"),
      (16384, "host"),
      (68719476736, "index_input"),
      (1099511627776, "invocation_mask"),
      (512, "late_fragment_tests"),
      (1048576, "mesh_shader"),
      (1073741824, "micromap_build"),
      (536870912, "optical_flow"),
      (274877906944, "pre_rasterization_shaders"),
      (2097152, "ray_tracing_shader"),
      (8589934592, "resolve"),
      (549755813888, "subpass_shading"),
      (524288, "task_shader"),
      (16, "tessellation_control_shader"),
      (32, "tessellation_evaluation_shader"),
      (1, "top_of_pipe"),
      (16777216, "transform_feedback"),
      (137438953472, "vertex_attribute_input"),
      (4, "vertex_input"),
      (8, "vertex_shader"),
      (67108864, "video_decode"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPipelineStageFlagBits2 {
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_build(self) -> bool {
    (self.0 & 33554432) != 0
  }
  #[inline]
  #[must_use]
  pub const fn acceleration_structure_copy(self) -> bool {
    (self.0 & 268435456) != 0
  }
  #[inline]
  #[must_use]
  pub const fn all_commands(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn all_graphics(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn all_transfer(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn blit(self) -> bool {
    (self.0 & 17179869184) != 0
  }
  #[inline]
  #[must_use]
  pub const fn bottom_of_pipe(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn clear(self) -> bool {
    (self.0 & 34359738368) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cluster_culling_shader(self) -> bool {
    (self.0 & 2199023255552) != 0
  }
  #[inline]
  #[must_use]
  pub const fn color_attachment_output(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn command_preprocess(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn compute_shader(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn conditional_rendering(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn copy(self) -> bool {
    (self.0 & 4294967296) != 0
  }
  #[inline]
  #[must_use]
  pub const fn draw_indirect(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn early_fragment_tests(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_density_process(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shader(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shading_rate_attachment(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn geometry_shader(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn host(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn index_input(self) -> bool {
    (self.0 & 68719476736) != 0
  }
  #[inline]
  #[must_use]
  pub const fn invocation_mask(self) -> bool {
    (self.0 & 1099511627776) != 0
  }
  #[inline]
  #[must_use]
  pub const fn late_fragment_tests(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn mesh_shader(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn micromap_build(self) -> bool {
    (self.0 & 1073741824) != 0
  }
  #[inline]
  #[must_use]
  pub const fn optical_flow(self) -> bool {
    (self.0 & 536870912) != 0
  }
  #[inline]
  #[must_use]
  pub const fn pre_rasterization_shaders(self) -> bool {
    (self.0 & 274877906944) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ray_tracing_shader(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn resolve(self) -> bool {
    (self.0 & 8589934592) != 0
  }
  #[inline]
  #[must_use]
  pub const fn subpass_shading(self) -> bool {
    (self.0 & 549755813888) != 0
  }
  #[inline]
  #[must_use]
  pub const fn task_shader(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn tessellation_control_shader(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn tessellation_evaluation_shader(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn top_of_pipe(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_feedback(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_attribute_input(self) -> bool {
    (self.0 & 137438953472) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_input(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_shader(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode(self) -> bool {
    (self.0 & 67108864) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineTessellationStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateFlags.html) (bitmask, no bits defined)
  VkPipelineTessellationStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineTessellationStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateFlags.html) (bitmask, no bits defined)
pub type VkPipelineTessellationStateCreateFlags =
  VkPipelineTessellationStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineTessellationStateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineVertexInputStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateFlags.html) (bitmask, no bits defined)
  VkPipelineVertexInputStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineVertexInputStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateFlags.html) (bitmask, no bits defined)
pub type VkPipelineVertexInputStateCreateFlags = VkPipelineVertexInputStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineVertexInputStateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineViewportStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateFlags.html) (bitmask, no bits defined)
  VkPipelineViewportStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineViewportStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateFlags.html) (bitmask, no bits defined)
pub type VkPipelineViewportStateCreateFlags = VkPipelineViewportStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineViewportStateCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineViewportSwizzleStateCreateFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateFlagsNV.html) (bitmask, no bits defined)
  VkPipelineViewportSwizzleStateCreateFlagBitsNV(u32)
);
/// Khronos: [VkPipelineViewportSwizzleStateCreateFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateFlagsNV.html) (bitmask, no bits defined)
pub type VkPipelineViewportSwizzleStateCreateFlagsNV =
  VkPipelineViewportSwizzleStateCreateFlagBitsNV;
impl core::fmt::Debug for VkPipelineViewportSwizzleStateCreateFlagBitsNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPresentGravityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentGravityFlagBitsEXT.html) (bitmask)
  VkPresentGravityFlagBitsEXT(u32)
);
/// Khronos: [VkPresentGravityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentGravityFlagBitsEXT.html) (bitmask)
pub type VkPresentGravityFlagsEXT = VkPresentGravityFlagBitsEXT;
pub const VK_PRESENT_GRAVITY_CENTERED_BIT_EXT: VkPresentGravityFlagBitsEXT =
  VkPresentGravityFlagBitsEXT(1 << 2);
pub const VK_PRESENT_GRAVITY_MAX_BIT_EXT: VkPresentGravityFlagBitsEXT =
  VkPresentGravityFlagBitsEXT(1 << 1);
pub const VK_PRESENT_GRAVITY_MIN_BIT_EXT: VkPresentGravityFlagBitsEXT =
  VkPresentGravityFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkPresentGravityFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(4, "centered"), (2, "max"), (1, "min")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPresentGravityFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn centered(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn max(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn min(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPresentScalingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentScalingFlagBitsEXT.html) (bitmask)
  VkPresentScalingFlagBitsEXT(u32)
);
/// Khronos: [VkPresentScalingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentScalingFlagBitsEXT.html) (bitmask)
pub type VkPresentScalingFlagsEXT = VkPresentScalingFlagBitsEXT;
pub const VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT: VkPresentScalingFlagBitsEXT =
  VkPresentScalingFlagBitsEXT(1 << 1);
pub const VK_PRESENT_SCALING_ONE_TO_ONE_BIT_EXT: VkPresentScalingFlagBitsEXT =
  VkPresentScalingFlagBitsEXT(1 << 0);
pub const VK_PRESENT_SCALING_STRETCH_BIT_EXT: VkPresentScalingFlagBitsEXT =
  VkPresentScalingFlagBitsEXT(1 << 2);
impl core::fmt::Debug for VkPresentScalingFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(2, "aspect_ratio_stretch"), (1, "one_to_one"), (4, "stretch")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPresentScalingFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn aspect_ratio_stretch(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn one_to_one(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn stretch(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkPrivateDataSlotCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateFlags.html) (bitmask, no bits defined)
  VkPrivateDataSlotCreateFlagBits(u32)
);
/// Khronos: [VkPrivateDataSlotCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateFlags.html) (bitmask, no bits defined)
pub type VkPrivateDataSlotCreateFlags = VkPrivateDataSlotCreateFlagBits;
pub const VK_PRIVATE_DATA_SLOT_CREATE_RESERVED_0_BIT_NV: VkPrivateDataSlotCreateFlagBits =
  VkPrivateDataSlotCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkPrivateDataSlotCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "reserved_0")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkPrivateDataSlotCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn reserved_0(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkQueryControlFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html) (bitmask)
  VkQueryControlFlagBits(u32)
);
/// Khronos: [VkQueryControlFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html) (bitmask)
pub type VkQueryControlFlags = VkQueryControlFlagBits;
/// Require precise results to be collected by the query
pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlagBits =
  VkQueryControlFlagBits(1 << 0);
impl core::fmt::Debug for VkQueryControlFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "precise")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkQueryControlFlagBits {
  #[inline]
  #[must_use]
  pub const fn precise(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkQueryPipelineStatisticFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html) (bitmask)
  VkQueryPipelineStatisticFlagBits(u32)
);
/// Khronos: [VkQueryPipelineStatisticFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html) (bitmask)
pub type VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlagBits;
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 5);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 6);
pub const VK_QUERY_PIPELINE_STATISTIC_CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 13);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 10);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 7);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 3);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 4);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 1);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 0);
pub const VK_QUERY_PIPELINE_STATISTIC_MESH_SHADER_INVOCATIONS_BIT_EXT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 12);
pub const VK_QUERY_PIPELINE_STATISTIC_TASK_SHADER_INVOCATIONS_BIT_EXT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 11);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 8);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 9);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 2);
impl core::fmt::Debug for VkQueryPipelineStatisticFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (32, "clipping_invocations"),
      (64, "clipping_primitives"),
      (8192, "cluster_culling_shader_invocations"),
      (1024, "compute_shader_invocations"),
      (128, "fragment_shader_invocations"),
      (8, "geometry_shader_invocations"),
      (16, "geometry_shader_primitives"),
      (2, "input_assembly_primitives"),
      (1, "input_assembly_vertices"),
      (4096, "mesh_shader_invocations"),
      (2048, "task_shader_invocations"),
      (256, "tessellation_control_shader_patches"),
      (512, "tessellation_evaluation_shader_invocations"),
      (4, "vertex_shader_invocations"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkQueryPipelineStatisticFlagBits {
  #[inline]
  #[must_use]
  pub const fn clipping_invocations(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn clipping_primitives(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cluster_culling_shader_invocations(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn compute_shader_invocations(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_shader_invocations(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn geometry_shader_invocations(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn geometry_shader_primitives(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn input_assembly_primitives(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn input_assembly_vertices(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn mesh_shader_invocations(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn task_shader_invocations(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn tessellation_control_shader_patches(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn tessellation_evaluation_shader_invocations(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex_shader_invocations(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkQueryPoolCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlags.html) (bitmask, no bits defined)
  VkQueryPoolCreateFlagBits(u32)
);
/// Khronos: [VkQueryPoolCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlags.html) (bitmask, no bits defined)
pub type VkQueryPoolCreateFlags = VkQueryPoolCreateFlagBits;
impl core::fmt::Debug for VkQueryPoolCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkQueryResultFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html) (bitmask)
  VkQueryResultFlagBits(u32)
);
/// Khronos: [VkQueryResultFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html) (bitmask)
pub type VkQueryResultFlags = VkQueryResultFlagBits;
/// Results of the queries are written to the destination buffer as 64-bit
/// values
pub const VK_QUERY_RESULT_64_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1 << 0);
/// Copy the partial results of the query even if the final results are not
/// available
pub const VK_QUERY_RESULT_PARTIAL_BIT: VkQueryResultFlagBits =
  VkQueryResultFlagBits(1 << 3);
/// Results of the queries are waited on before proceeding with the result copy
pub const VK_QUERY_RESULT_WAIT_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1 << 1);
/// Besides the results of the query, the availability of the results is also
/// written
pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT: VkQueryResultFlagBits =
  VkQueryResultFlagBits(1 << 2);
pub const VK_QUERY_RESULT_WITH_STATUS_BIT_KHR: VkQueryResultFlagBits =
  VkQueryResultFlagBits(1 << 4);
impl core::fmt::Debug for VkQueryResultFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "64"),
      (8, "partial"),
      (2, "wait"),
      (4, "with_availability"),
      (16, "with_status"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkQueryResultFlagBits {
  #[inline]
  #[must_use]
  pub const fn _64(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn partial(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn wait(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn with_availability(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn with_status(self) -> bool {
    (self.0 & 16) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkQueueFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html) (bitmask)
  VkQueueFlagBits(u32)
);
/// Khronos: [VkQueueFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html) (bitmask)
pub type VkQueueFlags = VkQueueFlagBits;
/// Queue supports compute operations
pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlagBits = VkQueueFlagBits(1 << 1);
/// Queue supports graphics operations
pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlagBits = VkQueueFlagBits(1 << 0);
pub const VK_QUEUE_OPTICAL_FLOW_BIT_NV: VkQueueFlagBits = VkQueueFlagBits(1 << 8);
/// Queues may support protected operations
pub const VK_QUEUE_PROTECTED_BIT: VkQueueFlagBits = VkQueueFlagBits(1 << 4);
pub const VK_QUEUE_RESERVED_7_BIT_QCOM: VkQueueFlagBits = VkQueueFlagBits(1 << 7);
pub const VK_QUEUE_RESERVED_9_BIT_EXT: VkQueueFlagBits = VkQueueFlagBits(1 << 9);
/// Queue supports sparse resource memory management operations
pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlagBits = VkQueueFlagBits(1 << 3);
/// Queue supports transfer operations
pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlagBits = VkQueueFlagBits(1 << 2);
pub const VK_QUEUE_VIDEO_DECODE_BIT_KHR: VkQueueFlagBits = VkQueueFlagBits(1 << 5);
impl core::fmt::Debug for VkQueueFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "compute"),
      (1, "graphics"),
      (256, "optical_flow"),
      (16, "protected"),
      (128, "reserved_7"),
      (512, "reserved_9"),
      (8, "sparse_binding"),
      (4, "transfer"),
      (32, "video_decode"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkQueueFlagBits {
  #[inline]
  #[must_use]
  pub const fn compute(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn graphics(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn optical_flow(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn protected(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_7(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_9(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sparse_binding(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transfer(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn video_decode(self) -> bool {
    (self.0 & 32) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkRefreshObjectFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshObjectFlagBitsKHR.html) (bitmask)
  VkRefreshObjectFlagBitsKHR(u32)
);
/// Khronos: [VkRefreshObjectFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshObjectFlagBitsKHR.html) (bitmask)
pub type VkRefreshObjectFlagsKHR = VkRefreshObjectFlagBitsKHR;
impl core::fmt::Debug for VkRefreshObjectFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkRenderPassCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateFlagBits.html) (bitmask)
  VkRenderPassCreateFlagBits(u32)
);
/// Khronos: [VkRenderPassCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateFlagBits.html) (bitmask)
pub type VkRenderPassCreateFlags = VkRenderPassCreateFlagBits;
pub const VK_RENDER_PASS_CREATE_RESERVED_0_BIT_KHR: VkRenderPassCreateFlagBits =
  VkRenderPassCreateFlagBits(1 << 0);
pub const VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM: VkRenderPassCreateFlagBits =
  VkRenderPassCreateFlagBits(1 << 1);
impl core::fmt::Debug for VkRenderPassCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "reserved_0"), (2, "transform")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkRenderPassCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn reserved_0(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkRenderingFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html) (bitmask)
  VkRenderingFlagBits(u32)
);
/// Khronos: [VkRenderingFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html) (bitmask)
pub type VkRenderingFlags = VkRenderingFlagBits;
pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: VkRenderingFlagBits =
  VkRenderingFlagBits(1 << 0);
pub const VK_RENDERING_ENABLE_LEGACY_DITHERING_BIT_EXT: VkRenderingFlagBits =
  VkRenderingFlagBits(1 << 3);
pub const VK_RENDERING_RESUMING_BIT: VkRenderingFlagBits = VkRenderingFlagBits(1 << 2);
pub const VK_RENDERING_SUSPENDING_BIT: VkRenderingFlagBits = VkRenderingFlagBits(1 << 1);
/// Alias of [`VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT`]
pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR: VkRenderingFlagBits =
  VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT;
/// Alias of [`VK_RENDERING_RESUMING_BIT`]
pub const VK_RENDERING_RESUMING_BIT_KHR: VkRenderingFlagBits = VK_RENDERING_RESUMING_BIT;
/// Alias of [`VK_RENDERING_SUSPENDING_BIT`]
pub const VK_RENDERING_SUSPENDING_BIT_KHR: VkRenderingFlagBits =
  VK_RENDERING_SUSPENDING_BIT;
impl core::fmt::Debug for VkRenderingFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "contents_secondary_command_buffers"),
      (8, "enable_legacy_dithering"),
      (4, "resuming"),
      (2, "suspending"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkRenderingFlagBits {
  #[inline]
  #[must_use]
  pub const fn contents_secondary_command_buffers(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn enable_legacy_dithering(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn resuming(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn suspending(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkResolveModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html) (bitmask)
  VkResolveModeFlagBits(u32)
);
/// Khronos: [VkResolveModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html) (bitmask)
pub type VkResolveModeFlags = VkResolveModeFlagBits;
pub const VK_RESOLVE_MODE_AVERAGE_BIT: VkResolveModeFlagBits =
  VkResolveModeFlagBits(1 << 1);
pub const VK_RESOLVE_MODE_MAX_BIT: VkResolveModeFlagBits = VkResolveModeFlagBits(1 << 3);
pub const VK_RESOLVE_MODE_MIN_BIT: VkResolveModeFlagBits = VkResolveModeFlagBits(1 << 2);
pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT: VkResolveModeFlagBits =
  VkResolveModeFlagBits(1 << 0);
pub const VK_RESOLVE_MODE_NONE: VkResolveModeFlagBits = VkResolveModeFlagBits(0);
/// Alias of [`VK_RESOLVE_MODE_AVERAGE_BIT`]
pub const VK_RESOLVE_MODE_AVERAGE_BIT_KHR: VkResolveModeFlagBits =
  VK_RESOLVE_MODE_AVERAGE_BIT;
/// Alias of [`VK_RESOLVE_MODE_MAX_BIT`]
pub const VK_RESOLVE_MODE_MAX_BIT_KHR: VkResolveModeFlagBits = VK_RESOLVE_MODE_MAX_BIT;
/// Alias of [`VK_RESOLVE_MODE_MIN_BIT`]
pub const VK_RESOLVE_MODE_MIN_BIT_KHR: VkResolveModeFlagBits = VK_RESOLVE_MODE_MIN_BIT;
/// Alias of [`VK_RESOLVE_MODE_NONE`]
pub const VK_RESOLVE_MODE_NONE_KHR: VkResolveModeFlagBits = VK_RESOLVE_MODE_NONE;
/// Alias of [`VK_RESOLVE_MODE_SAMPLE_ZERO_BIT`]
pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR: VkResolveModeFlagBits =
  VK_RESOLVE_MODE_SAMPLE_ZERO_BIT;
impl core::fmt::Debug for VkResolveModeFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(2, "average"), (8, "max"), (4, "min"), (1, "sample_zero")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkResolveModeFlagBits {
  #[inline]
  #[must_use]
  pub const fn average(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn max(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn min(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sample_zero(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSampleCountFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html) (bitmask)
  VkSampleCountFlagBits(u32)
);
/// Khronos: [VkSampleCountFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html) (bitmask)
pub type VkSampleCountFlags = VkSampleCountFlagBits;
/// Sample count 16 supported
pub const VK_SAMPLE_COUNT_16_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1 << 4);
/// Sample count 1 supported
pub const VK_SAMPLE_COUNT_1_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1 << 0);
/// Sample count 2 supported
pub const VK_SAMPLE_COUNT_2_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1 << 1);
/// Sample count 32 supported
pub const VK_SAMPLE_COUNT_32_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1 << 5);
/// Sample count 4 supported
pub const VK_SAMPLE_COUNT_4_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1 << 2);
/// Sample count 64 supported
pub const VK_SAMPLE_COUNT_64_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1 << 6);
/// Sample count 8 supported
pub const VK_SAMPLE_COUNT_8_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1 << 3);
impl core::fmt::Debug for VkSampleCountFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(16, "16"), (1, "1"), (2, "2"), (32, "32"), (4, "4"), (64, "64"), (8, "8")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSampleCountFlagBits {
  #[inline]
  #[must_use]
  pub const fn _16(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _1(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _2(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _32(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _4(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _64(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _8(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSamplerCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html) (bitmask)
  VkSamplerCreateFlagBits(u32)
);
/// Khronos: [VkSamplerCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html) (bitmask)
pub type VkSamplerCreateFlags = VkSamplerCreateFlagBits;
pub const VK_SAMPLER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT:
  VkSamplerCreateFlagBits = VkSamplerCreateFlagBits(1 << 3);
pub const VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM: VkSamplerCreateFlagBits =
  VkSamplerCreateFlagBits(1 << 4);
pub const VK_SAMPLER_CREATE_NON_SEAMLESS_CUBE_MAP_BIT_EXT: VkSamplerCreateFlagBits =
  VkSamplerCreateFlagBits(1 << 2);
pub const VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT: VkSamplerCreateFlagBits =
  VkSamplerCreateFlagBits(1 << 0);
pub const VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT:
  VkSamplerCreateFlagBits = VkSamplerCreateFlagBits(1 << 1);
impl core::fmt::Debug for VkSamplerCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "descriptor_buffer_capture_replay"),
      (16, "image_processing"),
      (4, "non_seamless_cube_map"),
      (1, "subsampled"),
      (2, "subsampled_coarse_reconstruction"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSamplerCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn descriptor_buffer_capture_replay(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn image_processing(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn non_seamless_cube_map(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn subsampled(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn subsampled_coarse_reconstruction(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkScreenSurfaceCreateFlagsQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateFlagsQNX.html) (bitmask, no bits defined)
  VkScreenSurfaceCreateFlagBitsQNX(u32)
);
/// Khronos: [VkScreenSurfaceCreateFlagsQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateFlagsQNX.html) (bitmask, no bits defined)
pub type VkScreenSurfaceCreateFlagsQNX = VkScreenSurfaceCreateFlagBitsQNX;
impl core::fmt::Debug for VkScreenSurfaceCreateFlagBitsQNX {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSemaphoreCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateFlags.html) (bitmask, no bits defined)
  VkSemaphoreCreateFlagBits(u32)
);
/// Khronos: [VkSemaphoreCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateFlags.html) (bitmask, no bits defined)
pub type VkSemaphoreCreateFlags = VkSemaphoreCreateFlagBits;
impl core::fmt::Debug for VkSemaphoreCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSemaphoreImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBits.html) (bitmask)
  VkSemaphoreImportFlagBits(u32)
);
/// Khronos: [VkSemaphoreImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBits.html) (bitmask)
pub type VkSemaphoreImportFlags = VkSemaphoreImportFlagBits;
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT: VkSemaphoreImportFlagBits =
  VkSemaphoreImportFlagBits(1 << 0);
/// Alias of [`VK_SEMAPHORE_IMPORT_TEMPORARY_BIT`]
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR: VkSemaphoreImportFlagBits =
  VK_SEMAPHORE_IMPORT_TEMPORARY_BIT;
impl core::fmt::Debug for VkSemaphoreImportFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "temporary")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSemaphoreImportFlagBits {
  #[inline]
  #[must_use]
  pub const fn temporary(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSemaphoreWaitFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html) (bitmask)
  VkSemaphoreWaitFlagBits(u32)
);
/// Khronos: [VkSemaphoreWaitFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html) (bitmask)
pub type VkSemaphoreWaitFlags = VkSemaphoreWaitFlagBits;
pub const VK_SEMAPHORE_WAIT_ANY_BIT: VkSemaphoreWaitFlagBits =
  VkSemaphoreWaitFlagBits(1 << 0);
/// Alias of [`VK_SEMAPHORE_WAIT_ANY_BIT`]
pub const VK_SEMAPHORE_WAIT_ANY_BIT_KHR: VkSemaphoreWaitFlagBits =
  VK_SEMAPHORE_WAIT_ANY_BIT;
impl core::fmt::Debug for VkSemaphoreWaitFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "any")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSemaphoreWaitFlagBits {
  #[inline]
  #[must_use]
  pub const fn any(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkShaderCorePropertiesFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html) (bitmask)
  VkShaderCorePropertiesFlagBitsAMD(u32)
);
/// Khronos: [VkShaderCorePropertiesFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html) (bitmask)
pub type VkShaderCorePropertiesFlagsAMD = VkShaderCorePropertiesFlagBitsAMD;
impl core::fmt::Debug for VkShaderCorePropertiesFlagBitsAMD {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkShaderModuleCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateFlags.html) (bitmask, no bits defined)
  VkShaderModuleCreateFlagBits(u32)
);
/// Khronos: [VkShaderModuleCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateFlags.html) (bitmask, no bits defined)
pub type VkShaderModuleCreateFlags = VkShaderModuleCreateFlagBits;
impl core::fmt::Debug for VkShaderModuleCreateFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkShaderStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html) (bitmask)
  VkShaderStageFlagBits(u32)
);
/// Khronos: [VkShaderStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html) (bitmask)
pub type VkShaderStageFlags = VkShaderStageFlagBits;
pub const VK_SHADER_STAGE_ANY_HIT_BIT_KHR: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 9);
pub const VK_SHADER_STAGE_CALLABLE_BIT_KHR: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 13);
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 10);
pub const VK_SHADER_STAGE_CLUSTER_CULLING_BIT_HUAWEI: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 19);
pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 5);
pub const VK_SHADER_STAGE_EXT_483_RESERVE_15: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 15);
pub const VK_SHADER_STAGE_EXT_483_RESERVE_16: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 16);
pub const VK_SHADER_STAGE_EXT_483_RESERVE_17: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 17);
pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 4);
pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 3);
pub const VK_SHADER_STAGE_INTERSECTION_BIT_KHR: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 12);
pub const VK_SHADER_STAGE_MESH_BIT_EXT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 7);
pub const VK_SHADER_STAGE_MISS_BIT_KHR: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 11);
pub const VK_SHADER_STAGE_RAYGEN_BIT_KHR: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 8);
pub const VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 14);
pub const VK_SHADER_STAGE_TASK_BIT_EXT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 6);
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 1);
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 2);
pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1 << 0);
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlagBits = VkShaderStageFlagBits(2147483647);
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlagBits = VkShaderStageFlagBits(31);
/// Alias of [`VK_SHADER_STAGE_ANY_HIT_BIT_KHR`]
pub const VK_SHADER_STAGE_ANY_HIT_BIT_NV: VkShaderStageFlagBits =
  VK_SHADER_STAGE_ANY_HIT_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_CALLABLE_BIT_KHR`]
pub const VK_SHADER_STAGE_CALLABLE_BIT_NV: VkShaderStageFlagBits =
  VK_SHADER_STAGE_CALLABLE_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR`]
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV: VkShaderStageFlagBits =
  VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_INTERSECTION_BIT_KHR`]
pub const VK_SHADER_STAGE_INTERSECTION_BIT_NV: VkShaderStageFlagBits =
  VK_SHADER_STAGE_INTERSECTION_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_MESH_BIT_EXT`]
pub const VK_SHADER_STAGE_MESH_BIT_NV: VkShaderStageFlagBits =
  VK_SHADER_STAGE_MESH_BIT_EXT;
/// Alias of [`VK_SHADER_STAGE_MISS_BIT_KHR`]
pub const VK_SHADER_STAGE_MISS_BIT_NV: VkShaderStageFlagBits =
  VK_SHADER_STAGE_MISS_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_RAYGEN_BIT_KHR`]
pub const VK_SHADER_STAGE_RAYGEN_BIT_NV: VkShaderStageFlagBits =
  VK_SHADER_STAGE_RAYGEN_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_TASK_BIT_EXT`]
pub const VK_SHADER_STAGE_TASK_BIT_NV: VkShaderStageFlagBits =
  VK_SHADER_STAGE_TASK_BIT_EXT;
impl core::fmt::Debug for VkShaderStageFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (512, "any_hit"),
      (8192, "callable"),
      (1024, "closest_hit"),
      (524288, "cluster_culling"),
      (32, "compute"),
      (32768, "ext_483_reserve_15"),
      (65536, "ext_483_reserve_16"),
      (131072, "ext_483_reserve_17"),
      (16, "fragment"),
      (8, "geometry"),
      (4096, "intersection"),
      (128, "mesh"),
      (2048, "miss"),
      (256, "raygen"),
      (16384, "subpass_shading"),
      (64, "task"),
      (2, "tessellation_control"),
      (4, "tessellation_evaluation"),
      (1, "vertex"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkShaderStageFlagBits {
  #[inline]
  #[must_use]
  pub const fn any_hit(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn callable(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn closest_hit(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cluster_culling(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn compute(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ext_483_reserve_15(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ext_483_reserve_16(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ext_483_reserve_17(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn geometry(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn intersection(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn mesh(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn miss(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn raygen(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn subpass_shading(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn task(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn tessellation_control(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn tessellation_evaluation(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vertex(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSparseImageFormatFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html) (bitmask)
  VkSparseImageFormatFlagBits(u32)
);
/// Khronos: [VkSparseImageFormatFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html) (bitmask)
pub type VkSparseImageFormatFlags = VkSparseImageFormatFlagBits;
/// Image requires mip level dimensions to be an integer multiple of the sparse
/// image block dimensions for non-tail mip levels.
pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlagBits =
  VkSparseImageFormatFlagBits(1 << 1);
/// Image uses a non-standard sparse image block dimensions
pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlagBits =
  VkSparseImageFormatFlagBits(1 << 2);
/// Image uses a single mip tail region for all array layers
pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlagBits =
  VkSparseImageFormatFlagBits(1 << 0);
impl core::fmt::Debug for VkSparseImageFormatFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(2, "aligned_mip_size"), (4, "nonstandard_block_size"), (1, "single_miptail")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSparseImageFormatFlagBits {
  #[inline]
  #[must_use]
  pub const fn aligned_mip_size(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn nonstandard_block_size(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn single_miptail(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSparseMemoryBindFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html) (bitmask)
  VkSparseMemoryBindFlagBits(u32)
);
/// Khronos: [VkSparseMemoryBindFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html) (bitmask)
pub type VkSparseMemoryBindFlags = VkSparseMemoryBindFlagBits;
/// Operation binds resource metadata to memory
pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT: VkSparseMemoryBindFlagBits =
  VkSparseMemoryBindFlagBits(1 << 0);
impl core::fmt::Debug for VkSparseMemoryBindFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "metadata")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSparseMemoryBindFlagBits {
  #[inline]
  #[must_use]
  pub const fn metadata(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkStencilFaceFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilFaceFlagBits.html) (bitmask)
  VkStencilFaceFlagBits(u32)
);
/// Khronos: [VkStencilFaceFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilFaceFlagBits.html) (bitmask)
pub type VkStencilFaceFlags = VkStencilFaceFlagBits;
/// Back face
pub const VK_STENCIL_FACE_BACK_BIT: VkStencilFaceFlagBits = VkStencilFaceFlagBits(1 << 1);
/// Front face
pub const VK_STENCIL_FACE_FRONT_BIT: VkStencilFaceFlagBits =
  VkStencilFaceFlagBits(1 << 0);
/// Front and back faces
pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlagBits =
  VkStencilFaceFlagBits(3);
/// Alias of [`VK_STENCIL_FACE_FRONT_AND_BACK`]
#[deprecated = "aliased"]
pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlagBits =
  VK_STENCIL_FACE_FRONT_AND_BACK;
impl core::fmt::Debug for VkStencilFaceFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(2, "back"), (1, "front")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkStencilFaceFlagBits {
  #[inline]
  #[must_use]
  pub const fn back(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn front(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkStreamDescriptorSurfaceCreateFlagsGGP](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html) (bitmask, no bits defined)
  VkStreamDescriptorSurfaceCreateFlagBitsGGP(u32)
);
/// Khronos: [VkStreamDescriptorSurfaceCreateFlagsGGP](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html) (bitmask, no bits defined)
pub type VkStreamDescriptorSurfaceCreateFlagsGGP =
  VkStreamDescriptorSurfaceCreateFlagBitsGGP;
impl core::fmt::Debug for VkStreamDescriptorSurfaceCreateFlagBitsGGP {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSubgroupFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlagBits.html) (bitmask)
  VkSubgroupFeatureFlagBits(u32)
);
/// Khronos: [VkSubgroupFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlagBits.html) (bitmask)
pub type VkSubgroupFeatureFlags = VkSubgroupFeatureFlagBits;
/// Arithmetic subgroup operations
pub const VK_SUBGROUP_FEATURE_ARITHMETIC_BIT: VkSubgroupFeatureFlagBits =
  VkSubgroupFeatureFlagBits(1 << 2);
/// Ballot subgroup operations
pub const VK_SUBGROUP_FEATURE_BALLOT_BIT: VkSubgroupFeatureFlagBits =
  VkSubgroupFeatureFlagBits(1 << 3);
/// Basic subgroup operations
pub const VK_SUBGROUP_FEATURE_BASIC_BIT: VkSubgroupFeatureFlagBits =
  VkSubgroupFeatureFlagBits(1 << 0);
/// Clustered subgroup operations
pub const VK_SUBGROUP_FEATURE_CLUSTERED_BIT: VkSubgroupFeatureFlagBits =
  VkSubgroupFeatureFlagBits(1 << 6);
pub const VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV: VkSubgroupFeatureFlagBits =
  VkSubgroupFeatureFlagBits(1 << 8);
/// Quad subgroup operations
pub const VK_SUBGROUP_FEATURE_QUAD_BIT: VkSubgroupFeatureFlagBits =
  VkSubgroupFeatureFlagBits(1 << 7);
/// Shuffle subgroup operations
pub const VK_SUBGROUP_FEATURE_SHUFFLE_BIT: VkSubgroupFeatureFlagBits =
  VkSubgroupFeatureFlagBits(1 << 4);
/// Shuffle relative subgroup operations
pub const VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT: VkSubgroupFeatureFlagBits =
  VkSubgroupFeatureFlagBits(1 << 5);
/// Vote subgroup operations
pub const VK_SUBGROUP_FEATURE_VOTE_BIT: VkSubgroupFeatureFlagBits =
  VkSubgroupFeatureFlagBits(1 << 1);
impl core::fmt::Debug for VkSubgroupFeatureFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "arithmetic"),
      (8, "ballot"),
      (1, "basic"),
      (64, "clustered"),
      (256, "partitioned"),
      (128, "quad"),
      (16, "shuffle"),
      (32, "shuffle_relative"),
      (2, "vote"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSubgroupFeatureFlagBits {
  #[inline]
  #[must_use]
  pub const fn arithmetic(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn ballot(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn basic(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn clustered(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn partitioned(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn quad(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shuffle(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shuffle_relative(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vote(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSubmitFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagBits.html) (bitmask)
  VkSubmitFlagBits(u32)
);
/// Khronos: [VkSubmitFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagBits.html) (bitmask)
pub type VkSubmitFlags = VkSubmitFlagBits;
pub const VK_SUBMIT_PROTECTED_BIT: VkSubmitFlagBits = VkSubmitFlagBits(1 << 0);
/// Alias of [`VK_SUBMIT_PROTECTED_BIT`]
pub const VK_SUBMIT_PROTECTED_BIT_KHR: VkSubmitFlagBits = VK_SUBMIT_PROTECTED_BIT;
impl core::fmt::Debug for VkSubmitFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "protected")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSubmitFlagBits {
  #[inline]
  #[must_use]
  pub const fn protected(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSubpassDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlagBits.html) (bitmask)
  VkSubpassDescriptionFlagBits(u32)
);
/// Khronos: [VkSubpassDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlagBits.html) (bitmask)
pub type VkSubpassDescriptionFlags = VkSubpassDescriptionFlagBits;
pub const VK_SUBPASS_DESCRIPTION_ENABLE_LEGACY_DITHERING_BIT_EXT:
  VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 7);
pub const VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM: VkSubpassDescriptionFlagBits =
  VkSubpassDescriptionFlagBits(1 << 2);
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX:
  VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 0);
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX:
  VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 1);
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT:
  VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 4);
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT:
  VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 5);
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT:
  VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 6);
pub const VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM: VkSubpassDescriptionFlagBits =
  VkSubpassDescriptionFlagBits(1 << 3);
/// Alias of [`VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT`]
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM:
  VkSubpassDescriptionFlagBits =
  VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT;
/// Alias of [`VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT`]
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM:
  VkSubpassDescriptionFlagBits =
  VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT;
/// Alias of [`VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT`]
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM:
  VkSubpassDescriptionFlagBits =
  VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT;
impl core::fmt::Debug for VkSubpassDescriptionFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (128, "enable_legacy_dithering"),
      (4, "fragment_region"),
      (1, "per_view_attributes"),
      (2, "per_view_position_x_only"),
      (16, "rasterization_order_attachment_color_access"),
      (32, "rasterization_order_attachment_depth_access"),
      (64, "rasterization_order_attachment_stencil_access"),
      (8, "shader_resolve"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSubpassDescriptionFlagBits {
  #[inline]
  #[must_use]
  pub const fn enable_legacy_dithering(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn fragment_region(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn per_view_attributes(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn per_view_position_x_only(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rasterization_order_attachment_color_access(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rasterization_order_attachment_depth_access(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rasterization_order_attachment_stencil_access(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn shader_resolve(self) -> bool {
    (self.0 & 8) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSurfaceCounterFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html) (bitmask)
  VkSurfaceCounterFlagBitsEXT(u32)
);
/// Khronos: [VkSurfaceCounterFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html) (bitmask)
pub type VkSurfaceCounterFlagsEXT = VkSurfaceCounterFlagBitsEXT;
pub const VK_SURFACE_COUNTER_VBLANK_BIT_EXT: VkSurfaceCounterFlagBitsEXT =
  VkSurfaceCounterFlagBitsEXT(1 << 0);
/// Alias of [`VK_SURFACE_COUNTER_VBLANK_BIT_EXT`]
#[deprecated = "aliased"]
pub const VK_SURFACE_COUNTER_VBLANK_EXT: VkSurfaceCounterFlagBitsEXT =
  VK_SURFACE_COUNTER_VBLANK_BIT_EXT;
impl core::fmt::Debug for VkSurfaceCounterFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "vblank")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSurfaceCounterFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn vblank(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSurfaceTransformFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) (bitmask)
  VkSurfaceTransformFlagBitsKHR(u32)
);
/// Khronos: [VkSurfaceTransformFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) (bitmask)
pub type VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagBitsKHR;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1 << 4);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR:
  VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 6);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR:
  VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 7);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR:
  VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 5);
pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1 << 0);
pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1 << 8);
pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1 << 2);
pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1 << 3);
pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR =
  VkSurfaceTransformFlagBitsKHR(1 << 1);
impl core::fmt::Debug for VkSurfaceTransformFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (16, "horizontal_mirror"),
      (64, "horizontal_mirror_rotate_180"),
      (128, "horizontal_mirror_rotate_270"),
      (32, "horizontal_mirror_rotate_90"),
      (1, "identity"),
      (256, "inherit"),
      (4, "rotate_180"),
      (8, "rotate_270"),
      (2, "rotate_90"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSurfaceTransformFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn horizontal_mirror(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn horizontal_mirror_rotate_180(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn horizontal_mirror_rotate_270(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn horizontal_mirror_rotate_90(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn identity(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn inherit(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rotate_180(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rotate_270(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rotate_90(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSwapchainCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) (bitmask)
  VkSwapchainCreateFlagBitsKHR(u32)
);
/// Khronos: [VkSwapchainCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) (bitmask)
pub type VkSwapchainCreateFlagsKHR = VkSwapchainCreateFlagBitsKHR;
pub const VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT:
  VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagBitsKHR(1 << 3);
pub const VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR: VkSwapchainCreateFlagBitsKHR =
  VkSwapchainCreateFlagBitsKHR(1 << 2);
/// Swapchain is protected
pub const VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR: VkSwapchainCreateFlagBitsKHR =
  VkSwapchainCreateFlagBitsKHR(1 << 1);
pub const VK_SWAPCHAIN_CREATE_RESERVED_4_BIT_EXT: VkSwapchainCreateFlagBitsKHR =
  VkSwapchainCreateFlagBitsKHR(1 << 4);
/// Allow images with VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT
pub const VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR:
  VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagBitsKHR(1 << 0);
impl core::fmt::Debug for VkSwapchainCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "deferred_memory_allocation"),
      (4, "mutable_format"),
      (2, "protected"),
      (16, "reserved_4"),
      (1, "split_instance_bind_regions"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSwapchainCreateFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn deferred_memory_allocation(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn mutable_format(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn protected(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn reserved_4(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn split_instance_bind_regions(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkSwapchainImageUsageFlagBitsANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainImageUsageFlagBitsANDROID.html) (bitmask)
  VkSwapchainImageUsageFlagBitsANDROID(u32)
);
/// Khronos: [VkSwapchainImageUsageFlagBitsANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainImageUsageFlagBitsANDROID.html) (bitmask)
pub type VkSwapchainImageUsageFlagsANDROID = VkSwapchainImageUsageFlagBitsANDROID;
pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID:
  VkSwapchainImageUsageFlagBitsANDROID = VkSwapchainImageUsageFlagBitsANDROID(1 << 0);
impl core::fmt::Debug for VkSwapchainImageUsageFlagBitsANDROID {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "shared")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkSwapchainImageUsageFlagBitsANDROID {
  #[inline]
  #[must_use]
  pub const fn shared(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkToolPurposeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagBits.html) (bitmask)
  VkToolPurposeFlagBits(u32)
);
/// Khronos: [VkToolPurposeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagBits.html) (bitmask)
pub type VkToolPurposeFlags = VkToolPurposeFlagBits;
pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT: VkToolPurposeFlagBits =
  VkToolPurposeFlagBits(1 << 3);
pub const VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT: VkToolPurposeFlagBits =
  VkToolPurposeFlagBits(1 << 6);
pub const VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT: VkToolPurposeFlagBits =
  VkToolPurposeFlagBits(1 << 5);
pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT: VkToolPurposeFlagBits =
  VkToolPurposeFlagBits(1 << 4);
pub const VK_TOOL_PURPOSE_PROFILING_BIT: VkToolPurposeFlagBits =
  VkToolPurposeFlagBits(1 << 1);
pub const VK_TOOL_PURPOSE_TRACING_BIT: VkToolPurposeFlagBits =
  VkToolPurposeFlagBits(1 << 2);
pub const VK_TOOL_PURPOSE_VALIDATION_BIT: VkToolPurposeFlagBits =
  VkToolPurposeFlagBits(1 << 0);
/// Alias of [`VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT`]
pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT: VkToolPurposeFlagBits =
  VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT;
/// Alias of [`VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT`]
pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT: VkToolPurposeFlagBits =
  VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT;
/// Alias of [`VK_TOOL_PURPOSE_PROFILING_BIT`]
pub const VK_TOOL_PURPOSE_PROFILING_BIT_EXT: VkToolPurposeFlagBits =
  VK_TOOL_PURPOSE_PROFILING_BIT;
/// Alias of [`VK_TOOL_PURPOSE_TRACING_BIT`]
pub const VK_TOOL_PURPOSE_TRACING_BIT_EXT: VkToolPurposeFlagBits =
  VK_TOOL_PURPOSE_TRACING_BIT;
/// Alias of [`VK_TOOL_PURPOSE_VALIDATION_BIT`]
pub const VK_TOOL_PURPOSE_VALIDATION_BIT_EXT: VkToolPurposeFlagBits =
  VK_TOOL_PURPOSE_VALIDATION_BIT;
impl core::fmt::Debug for VkToolPurposeFlagBits {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "additional_features"),
      (64, "debug_markers"),
      (32, "debug_reporting"),
      (16, "modifying_features"),
      (2, "profiling"),
      (4, "tracing"),
      (1, "validation"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkToolPurposeFlagBits {
  #[inline]
  #[must_use]
  pub const fn additional_features(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn debug_markers(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn debug_reporting(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn modifying_features(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn profiling(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn tracing(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn validation(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkValidationCacheCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateFlagsEXT.html) (bitmask, no bits defined)
  VkValidationCacheCreateFlagBitsEXT(u32)
);
/// Khronos: [VkValidationCacheCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkValidationCacheCreateFlagsEXT = VkValidationCacheCreateFlagBitsEXT;
impl core::fmt::Debug for VkValidationCacheCreateFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkViSurfaceCreateFlagsNN](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViSurfaceCreateFlagsNN.html) (bitmask, no bits defined)
  VkViSurfaceCreateFlagBitsNN(u32)
);
/// Khronos: [VkViSurfaceCreateFlagsNN](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViSurfaceCreateFlagsNN.html) (bitmask, no bits defined)
pub type VkViSurfaceCreateFlagsNN = VkViSurfaceCreateFlagBitsNN;
impl core::fmt::Debug for VkViSurfaceCreateFlagBitsNN {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoBeginCodingFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingFlagsKHR.html) (bitmask, no bits defined)
  VkVideoBeginCodingFlagBitsKHR(u32)
);
/// Khronos: [VkVideoBeginCodingFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoBeginCodingFlagsKHR = VkVideoBeginCodingFlagBitsKHR;
impl core::fmt::Debug for VkVideoBeginCodingFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoCapabilityFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilityFlagBitsKHR.html) (bitmask)
  VkVideoCapabilityFlagBitsKHR(u32)
);
/// Khronos: [VkVideoCapabilityFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilityFlagBitsKHR.html) (bitmask)
pub type VkVideoCapabilityFlagsKHR = VkVideoCapabilityFlagBitsKHR;
pub const VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR: VkVideoCapabilityFlagBitsKHR =
  VkVideoCapabilityFlagBitsKHR(1 << 0);
pub const VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR:
  VkVideoCapabilityFlagBitsKHR = VkVideoCapabilityFlagBitsKHR(1 << 1);
impl core::fmt::Debug for VkVideoCapabilityFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "protected_content"), (2, "separate_reference_images")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoCapabilityFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn protected_content(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn separate_reference_images(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoChromaSubsamplingFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoChromaSubsamplingFlagBitsKHR.html) (bitmask)
  ///
  /// Vulkan video chroma subsampling definitions
  VkVideoChromaSubsamplingFlagBitsKHR(u32)
);
/// Khronos: [VkVideoChromaSubsamplingFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoChromaSubsamplingFlagBitsKHR.html) (bitmask)
pub type VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagBitsKHR;
pub const VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR: VkVideoChromaSubsamplingFlagBitsKHR =
  VkVideoChromaSubsamplingFlagBitsKHR(1 << 1);
pub const VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR: VkVideoChromaSubsamplingFlagBitsKHR =
  VkVideoChromaSubsamplingFlagBitsKHR(1 << 2);
pub const VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR: VkVideoChromaSubsamplingFlagBitsKHR =
  VkVideoChromaSubsamplingFlagBitsKHR(1 << 3);
pub const VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR:
  VkVideoChromaSubsamplingFlagBitsKHR = VkVideoChromaSubsamplingFlagBitsKHR(1 << 0);
pub const VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_KHR: VkVideoChromaSubsamplingFlagBitsKHR =
  VkVideoChromaSubsamplingFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoChromaSubsamplingFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(2, "420"), (4, "422"), (8, "444"), (1, "monochrome")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoChromaSubsamplingFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn _420(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _422(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _444(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn monochrome(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoCodecOperationFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html) (bitmask)
  VkVideoCodecOperationFlagBitsKHR(u32)
);
/// Khronos: [VkVideoCodecOperationFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html) (bitmask)
pub type VkVideoCodecOperationFlagsKHR = VkVideoCodecOperationFlagBitsKHR;
pub const VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_KHR: VkVideoCodecOperationFlagBitsKHR =
  VkVideoCodecOperationFlagBitsKHR(1 << 0);
pub const VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_KHR: VkVideoCodecOperationFlagBitsKHR =
  VkVideoCodecOperationFlagBitsKHR(1 << 1);
pub const VK_VIDEO_CODEC_OPERATION_NONE_KHR: VkVideoCodecOperationFlagBitsKHR =
  VkVideoCodecOperationFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoCodecOperationFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "decode_h_264"), (2, "decode_h_265")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoCodecOperationFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn decode_h_264(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn decode_h_265(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoCodingControlFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html) (bitmask)
  VkVideoCodingControlFlagBitsKHR(u32)
);
/// Khronos: [VkVideoCodingControlFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html) (bitmask)
pub type VkVideoCodingControlFlagsKHR = VkVideoCodingControlFlagBitsKHR;
pub const VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR: VkVideoCodingControlFlagBitsKHR =
  VkVideoCodingControlFlagBitsKHR(1 << 0);
impl core::fmt::Debug for VkVideoCodingControlFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "reset")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoCodingControlFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn reset(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoComponentBitDepthFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoComponentBitDepthFlagBitsKHR.html) (bitmask)
  ///
  /// Vulkan video component bit depth definitions
  VkVideoComponentBitDepthFlagBitsKHR(u32)
);
/// Khronos: [VkVideoComponentBitDepthFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoComponentBitDepthFlagBitsKHR.html) (bitmask)
pub type VkVideoComponentBitDepthFlagsKHR = VkVideoComponentBitDepthFlagBitsKHR;
pub const VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR: VkVideoComponentBitDepthFlagBitsKHR =
  VkVideoComponentBitDepthFlagBitsKHR(1 << 2);
pub const VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR: VkVideoComponentBitDepthFlagBitsKHR =
  VkVideoComponentBitDepthFlagBitsKHR(1 << 4);
pub const VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR: VkVideoComponentBitDepthFlagBitsKHR =
  VkVideoComponentBitDepthFlagBitsKHR(1 << 0);
pub const VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR: VkVideoComponentBitDepthFlagBitsKHR =
  VkVideoComponentBitDepthFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoComponentBitDepthFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(4, "10"), (16, "12"), (1, "8")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoComponentBitDepthFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn _10(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _12(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _8(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoDecodeFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagsKHR.html) (bitmask, no bits defined)
  VkVideoDecodeFlagBitsKHR(u32)
);
/// Khronos: [VkVideoDecodeFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoDecodeFlagsKHR = VkVideoDecodeFlagBitsKHR;
impl core::fmt::Debug for VkVideoDecodeFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoDecodeCapabilityFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html) (bitmask)
  VkVideoDecodeCapabilityFlagBitsKHR(u32)
);
/// Khronos: [VkVideoDecodeCapabilityFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html) (bitmask)
pub type VkVideoDecodeCapabilityFlagsKHR = VkVideoDecodeCapabilityFlagBitsKHR;
pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR:
  VkVideoDecodeCapabilityFlagBitsKHR = VkVideoDecodeCapabilityFlagBitsKHR(1 << 0);
pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR:
  VkVideoDecodeCapabilityFlagBitsKHR = VkVideoDecodeCapabilityFlagBitsKHR(1 << 1);
impl core::fmt::Debug for VkVideoDecodeCapabilityFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "dpb_and_output_coincide"), (2, "dpb_and_output_distinct")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoDecodeCapabilityFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn dpb_and_output_coincide(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn dpb_and_output_distinct(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoDecodeH264PictureLayoutFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html) (bitmask)
  VkVideoDecodeH264PictureLayoutFlagBitsKHR(u32)
);
/// Khronos: [VkVideoDecodeH264PictureLayoutFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html) (bitmask)
pub type VkVideoDecodeH264PictureLayoutFlagsKHR =
  VkVideoDecodeH264PictureLayoutFlagBitsKHR;
pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR:
  VkVideoDecodeH264PictureLayoutFlagBitsKHR =
  VkVideoDecodeH264PictureLayoutFlagBitsKHR(1 << 0);
pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR:
  VkVideoDecodeH264PictureLayoutFlagBitsKHR =
  VkVideoDecodeH264PictureLayoutFlagBitsKHR(1 << 1);
pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR:
  VkVideoDecodeH264PictureLayoutFlagBitsKHR =
  VkVideoDecodeH264PictureLayoutFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoDecodeH264PictureLayoutFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "interlaced_interleaved_lines"), (2, "interlaced_separate_planes")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoDecodeH264PictureLayoutFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn interlaced_interleaved_lines(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn interlaced_separate_planes(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoDecodeUsageFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeUsageFlagBitsKHR.html) (bitmask)
  VkVideoDecodeUsageFlagBitsKHR(u32)
);
/// Khronos: [VkVideoDecodeUsageFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeUsageFlagBitsKHR.html) (bitmask)
pub type VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagBitsKHR;
pub const VK_VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR: VkVideoDecodeUsageFlagBitsKHR =
  VkVideoDecodeUsageFlagBitsKHR(1 << 1);
pub const VK_VIDEO_DECODE_USAGE_STREAMING_BIT_KHR: VkVideoDecodeUsageFlagBitsKHR =
  VkVideoDecodeUsageFlagBitsKHR(1 << 2);
pub const VK_VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR: VkVideoDecodeUsageFlagBitsKHR =
  VkVideoDecodeUsageFlagBitsKHR(1 << 0);
pub const VK_VIDEO_DECODE_USAGE_DEFAULT_KHR: VkVideoDecodeUsageFlagBitsKHR =
  VkVideoDecodeUsageFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoDecodeUsageFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(2, "offline"), (4, "streaming"), (1, "transcoding")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoDecodeUsageFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn offline(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn streaming(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transcoding(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFlagsKHR.html) (bitmask, no bits defined)
  VkVideoEncodeFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoEncodeFlagsKHR = VkVideoEncodeFlagBitsKHR;
impl core::fmt::Debug for VkVideoEncodeFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeCapabilityFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilityFlagBitsKHR.html) (bitmask)
  VkVideoEncodeCapabilityFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeCapabilityFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilityFlagBitsKHR.html) (bitmask)
pub type VkVideoEncodeCapabilityFlagsKHR = VkVideoEncodeCapabilityFlagBitsKHR;
pub const VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR:
  VkVideoEncodeCapabilityFlagBitsKHR = VkVideoEncodeCapabilityFlagBitsKHR(1 << 0);
impl core::fmt::Debug for VkVideoEncodeCapabilityFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "preceding_externally_encoded_bytes")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoEncodeCapabilityFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn preceding_externally_encoded_bytes(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeContentFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeContentFlagBitsKHR.html) (bitmask)
  VkVideoEncodeContentFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeContentFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeContentFlagBitsKHR.html) (bitmask)
pub type VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagBitsKHR;
pub const VK_VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR: VkVideoEncodeContentFlagBitsKHR =
  VkVideoEncodeContentFlagBitsKHR(1 << 0);
pub const VK_VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR: VkVideoEncodeContentFlagBitsKHR =
  VkVideoEncodeContentFlagBitsKHR(1 << 1);
pub const VK_VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR: VkVideoEncodeContentFlagBitsKHR =
  VkVideoEncodeContentFlagBitsKHR(1 << 2);
pub const VK_VIDEO_ENCODE_CONTENT_DEFAULT_KHR: VkVideoEncodeContentFlagBitsKHR =
  VkVideoEncodeContentFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoEncodeContentFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "camera"), (2, "desktop"), (4, "rendered")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoEncodeContentFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn camera(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn desktop(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn rendered(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeFeedbackFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFeedbackFlagBitsKHR.html) (bitmask)
  VkVideoEncodeFeedbackFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeFeedbackFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFeedbackFlagBitsKHR.html) (bitmask)
pub type VkVideoEncodeFeedbackFlagsKHR = VkVideoEncodeFeedbackFlagBitsKHR;
pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR:
  VkVideoEncodeFeedbackFlagBitsKHR = VkVideoEncodeFeedbackFlagBitsKHR(1 << 0);
pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR:
  VkVideoEncodeFeedbackFlagBitsKHR = VkVideoEncodeFeedbackFlagBitsKHR(1 << 1);
impl core::fmt::Debug for VkVideoEncodeFeedbackFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(1, "bitstream_buffer_offset"), (2, "bitstream_bytes_written")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoEncodeFeedbackFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn bitstream_buffer_offset(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn bitstream_bytes_written(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH264CapabilityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilityFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH264CapabilityFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH264CapabilityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilityFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH264CapabilityFlagsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 24);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_CABAC_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 14);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_CAVLC_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 15);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_CHROMA_QP_OFFSET_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 6);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 16);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 17);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 18);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_REFERENCE_FINAL_LISTS_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 25);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 23);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIRECT_8X8_INFERENCE_DISABLED_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 1);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIRECT_8X8_INFERENCE_ENABLED_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DISABLE_DIRECT_SPATIAL_MV_PRED_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 19);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 5);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 20);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PIC_INIT_QP_MINUS26_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 8);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 3);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 22);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_SCALING_LISTS_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 4);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 7);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_SLICE_MB_COUNT_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 21);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_TRANSFORM_8X8_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 13);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BIPRED_EXPLICIT_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 10);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BIPRED_IMPLICIT_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 11);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_PRED_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 9);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_PRED_NO_TABLE_BIT_EXT:
  VkVideoEncodeH264CapabilityFlagBitsEXT =
  VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 12);
impl core::fmt::Debug for VkVideoEncodeH264CapabilityFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (16777216, "b_frame_in_l_1_list"),
      (16384, "cabac"),
      (32768, "cavlc"),
      (64, "chroma_qp_offset"),
      (65536, "deblocking_filter_disabled"),
      (131072, "deblocking_filter_enabled"),
      (262144, "deblocking_filter_partial"),
      (33554432, "different_reference_final_lists"),
      (8388608, "different_slice_type"),
      (2, "direct_8_x_8_inference_disabled"),
      (1, "direct_8_x_8_inference_enabled"),
      (524288, "disable_direct_spatial_mv_pred"),
      (32, "hrd_compliance"),
      (1048576, "multiple_slice_per_frame"),
      (256, "pic_init_qp_minus_26"),
      (8, "qpprime_y_zero_transform_bypass"),
      (4194304, "row_unaligned_slice"),
      (16, "scaling_lists"),
      (128, "second_chroma_qp_offset"),
      (4, "separate_colour_plane"),
      (2097152, "slice_mb_count"),
      (8192, "transform_8_x_8"),
      (1024, "weighted_bipred_explicit"),
      (2048, "weighted_bipred_implicit"),
      (512, "weighted_pred"),
      (4096, "weighted_pred_no_table"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoEncodeH264CapabilityFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn b_frame_in_l_1_list(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cabac(self) -> bool {
    (self.0 & 16384) != 0
  }
  #[inline]
  #[must_use]
  pub const fn cavlc(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn chroma_qp_offset(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn deblocking_filter_disabled(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn deblocking_filter_enabled(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn deblocking_filter_partial(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn different_reference_final_lists(self) -> bool {
    (self.0 & 33554432) != 0
  }
  #[inline]
  #[must_use]
  pub const fn different_slice_type(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn direct_8_x_8_inference_disabled(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn direct_8_x_8_inference_enabled(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn disable_direct_spatial_mv_pred(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn hrd_compliance(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn multiple_slice_per_frame(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn pic_init_qp_minus_26(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn qpprime_y_zero_transform_bypass(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn row_unaligned_slice(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn scaling_lists(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn second_chroma_qp_offset(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn separate_colour_plane(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn slice_mb_count(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_8_x_8(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn weighted_bipred_explicit(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn weighted_bipred_implicit(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn weighted_pred(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn weighted_pred_no_table(self) -> bool {
    (self.0 & 4096) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH265CapabilityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilityFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH265CapabilityFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH265CapabilityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilityFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH265CapabilityFlagsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 25);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DEBLOCKING_FILTER_OVERRIDE_ENABLED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 17);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DEPENDENT_SLICE_SEGMENT_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 23);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_REFERENCE_FINAL_LISTS_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 26);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 24);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_ENTROPY_CODING_SYNC_ENABLED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 16);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 5);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_INIT_QP_MINUS26_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 6);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 7);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_PER_TILE_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 19);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILE_PER_FRAME_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 18);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILE_PER_SLICE_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 20);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PCM_ENABLE_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 3);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 11);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 22);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SAMPLE_ADAPTIVE_OFFSET_ENABLED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SCALING_LISTS_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 1);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SIGN_DATA_HIDING_ENABLED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 8);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SLICE_SEGMENT_CTB_COUNT_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 21);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SPS_TEMPORAL_MVP_ENABLED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 4);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSFORM_SKIP_DISABLED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 10);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSFORM_SKIP_ENABLED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 9);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSQUANT_BYPASS_ENABLED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 15);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_BIPRED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 13);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_PRED_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 12);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_PRED_NO_TABLE_BIT_EXT:
  VkVideoEncodeH265CapabilityFlagBitsEXT =
  VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 14);
impl core::fmt::Debug for VkVideoEncodeH265CapabilityFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (33554432, "b_frame_in_l_1_list"),
      (131072, "deblocking_filter_override_enabled"),
      (8388608, "dependent_slice_segment"),
      (67108864, "different_reference_final_lists"),
      (16777216, "different_slice_type"),
      (65536, "entropy_coding_sync_enabled"),
      (32, "hrd_compliance"),
      (64, "init_qp_minus_26"),
      (128, "log_2_parallel_merge_level_minus_2"),
      (524288, "multiple_slice_per_tile"),
      (262144, "multiple_tile_per_frame"),
      (1048576, "multiple_tile_per_slice"),
      (8, "pcm_enable"),
      (2048, "pps_slice_chroma_qp_offsets_present"),
      (4194304, "row_unaligned_slice_segment"),
      (4, "sample_adaptive_offset_enabled"),
      (2, "scaling_lists"),
      (1, "separate_colour_plane"),
      (256, "sign_data_hiding_enabled"),
      (2097152, "slice_segment_ctb_count"),
      (16, "sps_temporal_mvp_enabled"),
      (1024, "transform_skip_disabled"),
      (512, "transform_skip_enabled"),
      (32768, "transquant_bypass_enabled"),
      (8192, "weighted_bipred"),
      (4096, "weighted_pred"),
      (16384, "weighted_pred_no_table"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoEncodeH265CapabilityFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn b_frame_in_l_1_list(self) -> bool {
    (self.0 & 33554432) != 0
  }
  #[inline]
  #[must_use]
  pub const fn deblocking_filter_override_enabled(self) -> bool {
    (self.0 & 131072) != 0
  }
  #[inline]
  #[must_use]
  pub const fn dependent_slice_segment(self) -> bool {
    (self.0 & 8388608) != 0
  }
  #[inline]
  #[must_use]
  pub const fn different_reference_final_lists(self) -> bool {
    (self.0 & 67108864) != 0
  }
  #[inline]
  #[must_use]
  pub const fn different_slice_type(self) -> bool {
    (self.0 & 16777216) != 0
  }
  #[inline]
  #[must_use]
  pub const fn entropy_coding_sync_enabled(self) -> bool {
    (self.0 & 65536) != 0
  }
  #[inline]
  #[must_use]
  pub const fn hrd_compliance(self) -> bool {
    (self.0 & 32) != 0
  }
  #[inline]
  #[must_use]
  pub const fn init_qp_minus_26(self) -> bool {
    (self.0 & 64) != 0
  }
  #[inline]
  #[must_use]
  pub const fn log_2_parallel_merge_level_minus_2(self) -> bool {
    (self.0 & 128) != 0
  }
  #[inline]
  #[must_use]
  pub const fn multiple_slice_per_tile(self) -> bool {
    (self.0 & 524288) != 0
  }
  #[inline]
  #[must_use]
  pub const fn multiple_tile_per_frame(self) -> bool {
    (self.0 & 262144) != 0
  }
  #[inline]
  #[must_use]
  pub const fn multiple_tile_per_slice(self) -> bool {
    (self.0 & 1048576) != 0
  }
  #[inline]
  #[must_use]
  pub const fn pcm_enable(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn pps_slice_chroma_qp_offsets_present(self) -> bool {
    (self.0 & 2048) != 0
  }
  #[inline]
  #[must_use]
  pub const fn row_unaligned_slice_segment(self) -> bool {
    (self.0 & 4194304) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sample_adaptive_offset_enabled(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn scaling_lists(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn separate_colour_plane(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sign_data_hiding_enabled(self) -> bool {
    (self.0 & 256) != 0
  }
  #[inline]
  #[must_use]
  pub const fn slice_segment_ctb_count(self) -> bool {
    (self.0 & 2097152) != 0
  }
  #[inline]
  #[must_use]
  pub const fn sps_temporal_mvp_enabled(self) -> bool {
    (self.0 & 16) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_skip_disabled(self) -> bool {
    (self.0 & 1024) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transform_skip_enabled(self) -> bool {
    (self.0 & 512) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transquant_bypass_enabled(self) -> bool {
    (self.0 & 32768) != 0
  }
  #[inline]
  #[must_use]
  pub const fn weighted_bipred(self) -> bool {
    (self.0 & 8192) != 0
  }
  #[inline]
  #[must_use]
  pub const fn weighted_pred(self) -> bool {
    (self.0 & 4096) != 0
  }
  #[inline]
  #[must_use]
  pub const fn weighted_pred_no_table(self) -> bool {
    (self.0 & 16384) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH265CtbSizeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CtbSizeFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH265CtbSizeFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH265CtbSizeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CtbSizeFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH265CtbSizeFlagsEXT = VkVideoEncodeH265CtbSizeFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_EXT: VkVideoEncodeH265CtbSizeFlagBitsEXT =
  VkVideoEncodeH265CtbSizeFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_EXT: VkVideoEncodeH265CtbSizeFlagBitsEXT =
  VkVideoEncodeH265CtbSizeFlagBitsEXT(1 << 1);
pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_EXT: VkVideoEncodeH265CtbSizeFlagBitsEXT =
  VkVideoEncodeH265CtbSizeFlagBitsEXT(1 << 2);
impl core::fmt::Debug for VkVideoEncodeH265CtbSizeFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "16"), (2, "32"), (4, "64")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoEncodeH265CtbSizeFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn _16(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _32(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _64(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH265TransformBlockSizeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265TransformBlockSizeFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH265TransformBlockSizeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265TransformBlockSizeFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH265TransformBlockSizeFlagsEXT =
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_EXT:
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT =
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_EXT:
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT =
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(1 << 3);
pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_EXT:
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT =
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_EXT:
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT =
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkVideoEncodeH265TransformBlockSizeFlagBitsEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(4, "16"), (8, "32"), (1, "4"), (2, "8")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoEncodeH265TransformBlockSizeFlagBitsEXT {
  #[inline]
  #[must_use]
  pub const fn _16(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _32(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _4(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn _8(self) -> bool {
    (self.0 & 2) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeRateControlFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlFlagsKHR.html) (bitmask, no bits defined)
  VkVideoEncodeRateControlFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeRateControlFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoEncodeRateControlFlagsKHR = VkVideoEncodeRateControlFlagBitsKHR;
impl core::fmt::Debug for VkVideoEncodeRateControlFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeRateControlModeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlModeFlagBitsKHR.html) (bitmask)
  VkVideoEncodeRateControlModeFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeRateControlModeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlModeFlagBitsKHR.html) (bitmask)
pub type VkVideoEncodeRateControlModeFlagsKHR = VkVideoEncodeRateControlModeFlagBitsKHR;
pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR:
  VkVideoEncodeRateControlModeFlagBitsKHR =
  VkVideoEncodeRateControlModeFlagBitsKHR(1 << 1);
pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DISABLED_BIT_KHR:
  VkVideoEncodeRateControlModeFlagBitsKHR =
  VkVideoEncodeRateControlModeFlagBitsKHR(1 << 0);
pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR:
  VkVideoEncodeRateControlModeFlagBitsKHR =
  VkVideoEncodeRateControlModeFlagBitsKHR(1 << 2);
pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DEFAULT_KHR:
  VkVideoEncodeRateControlModeFlagBitsKHR = VkVideoEncodeRateControlModeFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoEncodeRateControlModeFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(2, "cbr"), (1, "disabled"), (4, "vbr")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoEncodeRateControlModeFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn cbr(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn disabled(self) -> bool {
    (self.0 & 1) != 0
  }
  #[inline]
  #[must_use]
  pub const fn vbr(self) -> bool {
    (self.0 & 4) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeUsageFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeUsageFlagBitsKHR.html) (bitmask)
  VkVideoEncodeUsageFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeUsageFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeUsageFlagBitsKHR.html) (bitmask)
pub type VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagBitsKHR;
pub const VK_VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR: VkVideoEncodeUsageFlagBitsKHR =
  VkVideoEncodeUsageFlagBitsKHR(1 << 3);
pub const VK_VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR: VkVideoEncodeUsageFlagBitsKHR =
  VkVideoEncodeUsageFlagBitsKHR(1 << 2);
pub const VK_VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR: VkVideoEncodeUsageFlagBitsKHR =
  VkVideoEncodeUsageFlagBitsKHR(1 << 1);
pub const VK_VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR: VkVideoEncodeUsageFlagBitsKHR =
  VkVideoEncodeUsageFlagBitsKHR(1 << 0);
pub const VK_VIDEO_ENCODE_USAGE_DEFAULT_KHR: VkVideoEncodeUsageFlagBitsKHR =
  VkVideoEncodeUsageFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoEncodeUsageFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in
      [(8, "conferencing"), (4, "recording"), (2, "streaming"), (1, "transcoding")]
    {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoEncodeUsageFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn conferencing(self) -> bool {
    (self.0 & 8) != 0
  }
  #[inline]
  #[must_use]
  pub const fn recording(self) -> bool {
    (self.0 & 4) != 0
  }
  #[inline]
  #[must_use]
  pub const fn streaming(self) -> bool {
    (self.0 & 2) != 0
  }
  #[inline]
  #[must_use]
  pub const fn transcoding(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEndCodingFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingFlagsKHR.html) (bitmask, no bits defined)
  VkVideoEndCodingFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEndCodingFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoEndCodingFlagsKHR = VkVideoEndCodingFlagBitsKHR;
impl core::fmt::Debug for VkVideoEndCodingFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoSessionCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateFlagBitsKHR.html) (bitmask)
  VkVideoSessionCreateFlagBitsKHR(u32)
);
/// Khronos: [VkVideoSessionCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateFlagBitsKHR.html) (bitmask)
pub type VkVideoSessionCreateFlagsKHR = VkVideoSessionCreateFlagBitsKHR;
pub const VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR:
  VkVideoSessionCreateFlagBitsKHR = VkVideoSessionCreateFlagBitsKHR(1 << 0);
impl core::fmt::Debug for VkVideoSessionCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [(1, "protected_content")] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}
impl VkVideoSessionCreateFlagBitsKHR {
  #[inline]
  #[must_use]
  pub const fn protected_content(self) -> bool {
    (self.0 & 1) != 0
  }
}

define_bitmask!(
  /// Khronos: [VkVideoSessionParametersCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateFlagsKHR.html) (bitmask, no bits defined)
  VkVideoSessionParametersCreateFlagBitsKHR(u32)
);
/// Khronos: [VkVideoSessionParametersCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoSessionParametersCreateFlagsKHR =
  VkVideoSessionParametersCreateFlagBitsKHR;
impl core::fmt::Debug for VkVideoSessionParametersCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkWaylandSurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
  VkWaylandSurfaceCreateFlagBitsKHR(u32)
);
/// Khronos: [VkWaylandSurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkWaylandSurfaceCreateFlagsKHR = VkWaylandSurfaceCreateFlagBitsKHR;
impl core::fmt::Debug for VkWaylandSurfaceCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkWin32SurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
  VkWin32SurfaceCreateFlagBitsKHR(u32)
);
/// Khronos: [VkWin32SurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkWin32SurfaceCreateFlagsKHR = VkWin32SurfaceCreateFlagBitsKHR;
impl core::fmt::Debug for VkWin32SurfaceCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkXcbSurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
  VkXcbSurfaceCreateFlagBitsKHR(u32)
);
/// Khronos: [VkXcbSurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkXcbSurfaceCreateFlagsKHR = VkXcbSurfaceCreateFlagBitsKHR;
impl core::fmt::Debug for VkXcbSurfaceCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkXlibSurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
  VkXlibSurfaceCreateFlagBitsKHR(u32)
);
/// Khronos: [VkXlibSurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkXlibSurfaceCreateFlagsKHR = VkXlibSurfaceCreateFlagBitsKHR;
impl core::fmt::Debug for VkXlibSurfaceCreateFlagBitsKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}
