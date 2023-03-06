#![allow(non_upper_case_globals)]

define_bitmask!(
  /// Khronos: [VkAccelerationStructureCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html) (bitmask)
  VkAccelerationStructureCreateFlagBitsKHR(u32)
);
/// Khronos: [VkAccelerationStructureCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html) (bitmask)
pub type VkAccelerationStructureCreateFlagsKHR = VkAccelerationStructureCreateFlagBitsKHR;
pub const VK_ACCELERATION_STRUCTURE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: VkAccelerationStructureCreateFlagBitsKHR = VkAccelerationStructureCreateFlagBitsKHR(1 << 3);
pub const VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkAccelerationStructureCreateFlagBitsKHR = VkAccelerationStructureCreateFlagBitsKHR(1 << 0);
pub const VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV: VkAccelerationStructureCreateFlagBitsKHR = VkAccelerationStructureCreateFlagBitsKHR(1 << 2);
impl core::fmt::Debug for VkAccelerationStructureCreateFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT"),
      (1, "DEVICE_ADDRESS_CAPTURE_REPLAY"),
      (4, "MOTION_BIT_NV"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkAccelerationStructureMotionInfoFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoFlagsNV.html) (bitmask, no bits defined)
  VkAccelerationStructureMotionInfoFlagBitsNV(u32)
);
/// Khronos: [VkAccelerationStructureMotionInfoFlagsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoFlagsNV.html) (bitmask, no bits defined)
pub type VkAccelerationStructureMotionInfoFlagsNV = VkAccelerationStructureMotionInfoFlagBitsNV;
impl core::fmt::Debug for VkAccelerationStructureMotionInfoFlagBitsNV {
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
pub type VkAccelerationStructureMotionInstanceFlagsNV = VkAccelerationStructureMotionInstanceFlagBitsNV;
impl core::fmt::Debug for VkAccelerationStructureMotionInstanceFlagBitsNV {
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
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR: VkAccessFlagBits = VkAccessFlagBits(1 << 21);
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR: VkAccessFlagBits = VkAccessFlagBits(1 << 22);
/// Controls coherency of color attachment reads
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 7);
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: VkAccessFlagBits = VkAccessFlagBits(1 << 19);
/// Controls coherency of color attachment writes
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 8);
pub const VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV: VkAccessFlagBits = VkAccessFlagBits(1 << 17);
pub const VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV: VkAccessFlagBits = VkAccessFlagBits(1 << 18);
/// read access flag for reading conditional rendering predicate
pub const VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT: VkAccessFlagBits = VkAccessFlagBits(1 << 20);
/// Controls coherency of depth/stencil attachment reads
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 9);
/// Controls coherency of depth/stencil attachment writes
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 10);
pub const VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT: VkAccessFlagBits = VkAccessFlagBits(1 << 24);
pub const VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits = VkAccessFlagBits(1 << 23);
/// Controls coherency of host reads
pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 13);
/// Controls coherency of host writes
pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 14);
/// Controls coherency of index reads
pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 1);
/// Controls coherency of indirect command reads
pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 0);
/// Controls coherency of input attachment reads
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 4);
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
pub const VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: VkAccessFlagBits = VkAccessFlagBits(1 << 26);
pub const VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: VkAccessFlagBits = VkAccessFlagBits(1 << 27);
pub const VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT: VkAccessFlagBits = VkAccessFlagBits(1 << 25);
/// Controls coherency of uniform buffer reads
pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 3);
/// Controls coherency of vertex attribute reads
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 2);
pub const VK_ACCESS_NONE: VkAccessFlagBits = VkAccessFlagBits(0);
/// Alias of [`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR`]
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV: VkAccessFlagBits = VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR;
/// Alias of [`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`]
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV: VkAccessFlagBits = VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR;
/// Alias of [`VK_ACCESS_NONE`]
pub const VK_ACCESS_NONE_KHR: VkAccessFlagBits = VK_ACCESS_NONE;
/// Alias of [`VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR`]
pub const VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV: VkAccessFlagBits = VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR;
impl core::fmt::Debug for VkAccessFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2097152, "ACCELERATION_STRUCTURE_READ_BIT_KHR"),
      (4194304, "ACCELERATION_STRUCTURE_WRITE_BIT_KHR"),
      (128, "COLOR_ATTACHMENT_READ"),
      (524288, "COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT"),
      (256, "COLOR_ATTACHMENT_WRITE"),
      (131072, "COMMAND_PREPROCESS_READ_BIT_NV"),
      (262144, "COMMAND_PREPROCESS_WRITE_BIT_NV"),
      (1048576, "CONDITIONAL_RENDERING_READ_BIT_EXT"),
      (512, "DEPTH_STENCIL_ATTACHMENT_READ"),
      (1024, "DEPTH_STENCIL_ATTACHMENT_WRITE"),
      (16777216, "FRAGMENT_DENSITY_MAP_READ_BIT_EXT"),
      (8388608, "FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR"),
      (8192, "HOST_READ"),
      (16384, "HOST_WRITE"),
      (2, "INDEX_READ"),
      (1, "INDIRECT_COMMAND_READ"),
      (16, "INPUT_ATTACHMENT_READ"),
      (32768, "MEMORY_READ"),
      (65536, "MEMORY_WRITE"),
      (32, "SHADER_READ"),
      (64, "SHADER_WRITE"),
      (2048, "TRANSFER_READ"),
      (4096, "TRANSFER_WRITE"),
      (67108864, "TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT"),
      (134217728, "TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT"),
      (33554432, "TRANSFORM_FEEDBACK_WRITE_BIT_EXT"),
      (8, "UNIFORM_READ"),
      (4, "VERTEX_ATTRIBUTE_READ"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkAccessFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html) (bitmask)
  VkAccessFlagBits2(u64)
);
/// Khronos: [VkAccessFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html) (bitmask)
pub type VkAccessFlags2 = VkAccessFlagBits2;
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 21);
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 22);
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 7);
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 19);
pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 8);
pub const VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 17);
pub const VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 18);
/// read access flag for reading conditional rendering predicate
pub const VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 20);
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 9);
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 10);
pub const VK_ACCESS_2_DESCRIPTOR_BUFFER_READ_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 41);
pub const VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 24);
pub const VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 23);
pub const VK_ACCESS_2_HOST_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 13);
pub const VK_ACCESS_2_HOST_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 14);
pub const VK_ACCESS_2_INDEX_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 1);
pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 0);
pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 4);
pub const VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 39);
pub const VK_ACCESS_2_MEMORY_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 15);
pub const VK_ACCESS_2_MEMORY_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 16);
pub const VK_ACCESS_2_MICROMAP_READ_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 44);
pub const VK_ACCESS_2_MICROMAP_WRITE_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 45);
pub const VK_ACCESS_2_OPTICAL_FLOW_READ_BIT_NV: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 42);
pub const VK_ACCESS_2_OPTICAL_FLOW_WRITE_BIT_NV: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 43);
pub const VK_ACCESS_2_RESERVED_46_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 46);
pub const VK_ACCESS_2_SHADER_BINDING_TABLE_READ_BIT_KHR: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 40);
pub const VK_ACCESS_2_SHADER_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 5);
pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 32);
pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 33);
pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 34);
pub const VK_ACCESS_2_SHADER_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 6);
pub const VK_ACCESS_2_TRANSFER_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 11);
pub const VK_ACCESS_2_TRANSFER_WRITE_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 12);
pub const VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 26);
pub const VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 27);
pub const VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 25);
pub const VK_ACCESS_2_UNIFORM_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 3);
pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 2);
pub const VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 35);
pub const VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR: VkAccessFlagBits2 = VkAccessFlagBits2(1 << 36);
pub const VK_ACCESS_2_NONE: VkAccessFlagBits2 = VkAccessFlagBits2(0);
/// Alias of [`VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`]
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV: VkAccessFlagBits2 = VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR;
/// Alias of [`VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`]
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV: VkAccessFlagBits2 = VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR;
/// Alias of [`VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`]
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT;
/// Alias of [`VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`]
pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`]
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT;
/// Alias of [`VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`]
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_HOST_READ_BIT`]
pub const VK_ACCESS_2_HOST_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_HOST_READ_BIT;
/// Alias of [`VK_ACCESS_2_HOST_WRITE_BIT`]
pub const VK_ACCESS_2_HOST_WRITE_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_HOST_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_INDEX_READ_BIT`]
pub const VK_ACCESS_2_INDEX_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_INDEX_READ_BIT;
/// Alias of [`VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`]
pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT;
/// Alias of [`VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`]
pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT;
/// Alias of [`VK_ACCESS_2_MEMORY_READ_BIT`]
pub const VK_ACCESS_2_MEMORY_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_MEMORY_READ_BIT;
/// Alias of [`VK_ACCESS_2_MEMORY_WRITE_BIT`]
pub const VK_ACCESS_2_MEMORY_WRITE_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_MEMORY_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_NONE`]
pub const VK_ACCESS_2_NONE_KHR: VkAccessFlagBits2 = VK_ACCESS_2_NONE;
/// Alias of [`VK_ACCESS_2_SHADER_READ_BIT`]
pub const VK_ACCESS_2_SHADER_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_SHADER_READ_BIT;
/// Alias of [`VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`]
pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_SHADER_SAMPLED_READ_BIT;
/// Alias of [`VK_ACCESS_2_SHADER_STORAGE_READ_BIT`]
pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_SHADER_STORAGE_READ_BIT;
/// Alias of [`VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`]
pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_SHADER_WRITE_BIT`]
pub const VK_ACCESS_2_SHADER_WRITE_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_SHADER_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR`]
pub const VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV: VkAccessFlagBits2 = VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR;
/// Alias of [`VK_ACCESS_2_TRANSFER_READ_BIT`]
pub const VK_ACCESS_2_TRANSFER_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_TRANSFER_READ_BIT;
/// Alias of [`VK_ACCESS_2_TRANSFER_WRITE_BIT`]
pub const VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_TRANSFER_WRITE_BIT;
/// Alias of [`VK_ACCESS_2_UNIFORM_READ_BIT`]
pub const VK_ACCESS_2_UNIFORM_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_UNIFORM_READ_BIT;
/// Alias of [`VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`]
pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR: VkAccessFlagBits2 = VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT;
impl core::fmt::Debug for VkAccessFlagBits2 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2097152, "ACCELERATION_STRUCTURE_READ_BIT_KHR"),
      (4194304, "ACCELERATION_STRUCTURE_WRITE_BIT_KHR"),
      (128, "COLOR_ATTACHMENT_READ"),
      (524288, "COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT"),
      (256, "COLOR_ATTACHMENT_WRITE"),
      (131072, "COMMAND_PREPROCESS_READ_BIT_NV"),
      (262144, "COMMAND_PREPROCESS_WRITE_BIT_NV"),
      (1048576, "CONDITIONAL_RENDERING_READ_BIT_EXT"),
      (512, "DEPTH_STENCIL_ATTACHMENT_READ"),
      (1024, "DEPTH_STENCIL_ATTACHMENT_WRITE"),
      (2199023255552, "DESCRIPTOR_BUFFER_READ_BIT_EXT"),
      (16777216, "FRAGMENT_DENSITY_MAP_READ_BIT_EXT"),
      (8388608, "FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR"),
      (8192, "HOST_READ"),
      (16384, "HOST_WRITE"),
      (2, "INDEX_READ"),
      (1, "INDIRECT_COMMAND_READ"),
      (16, "INPUT_ATTACHMENT_READ"),
      (549755813888, "INVOCATION_MASK_READ_BIT_HUAWEI"),
      (32768, "MEMORY_READ"),
      (65536, "MEMORY_WRITE"),
      (17592186044416, "MICROMAP_READ_BIT_EXT"),
      (35184372088832, "MICROMAP_WRITE_BIT_EXT"),
      (4398046511104, "OPTICAL_FLOW_READ_BIT_NV"),
      (8796093022208, "OPTICAL_FLOW_WRITE_BIT_NV"),
      (70368744177664, "RESERVED_46_BIT_EXT"),
      (1099511627776, "SHADER_BINDING_TABLE_READ_BIT_KHR"),
      (32, "SHADER_READ"),
      (4294967296, "SHADER_SAMPLED_READ"),
      (8589934592, "SHADER_STORAGE_READ"),
      (17179869184, "SHADER_STORAGE_WRITE"),
      (64, "SHADER_WRITE"),
      (2048, "TRANSFER_READ"),
      (4096, "TRANSFER_WRITE"),
      (67108864, "TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT"),
      (134217728, "TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT"),
      (33554432, "TRANSFORM_FEEDBACK_WRITE_BIT_EXT"),
      (8, "UNIFORM_READ"),
      (4, "VERTEX_ATTRIBUTE_READ"),
      (34359738368, "VIDEO_DECODE_READ_BIT_KHR"),
      (68719476736, "VIDEO_DECODE_WRITE_BIT_KHR"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkAcquireProfilingLockFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html) (bitmask)
  VkAcquireProfilingLockFlagBitsKHR(u32)
);
/// Khronos: [VkAcquireProfilingLockFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html) (bitmask)
pub type VkAcquireProfilingLockFlagsKHR = VkAcquireProfilingLockFlagBitsKHR;
impl core::fmt::Debug for VkAcquireProfilingLockFlagBitsKHR {
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
/// The attachment may alias physical memory of another attachment in the same render pass
pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlagBits(1 << 0);
impl core::fmt::Debug for VkAttachmentDescriptionFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "MAY_ALIAS"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkBufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html) (bitmask)
  VkBufferCreateFlagBits(u32)
);
/// Khronos: [VkBufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html) (bitmask)
pub type VkBufferCreateFlags = VkBufferCreateFlagBits;
pub const VK_BUFFER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: VkBufferCreateFlagBits = VkBufferCreateFlagBits(1 << 5);
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkBufferCreateFlagBits = VkBufferCreateFlagBits(1 << 4);
/// Buffer requires protected memory
pub const VK_BUFFER_CREATE_PROTECTED_BIT: VkBufferCreateFlagBits = VkBufferCreateFlagBits(1 << 3);
/// Buffer should support constant data access to physical memory ranges mapped into multiple locations of sparse buffers
pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkBufferCreateFlagBits = VkBufferCreateFlagBits(1 << 2);
/// Buffer should support sparse backing
pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkBufferCreateFlagBits = VkBufferCreateFlagBits(1 << 0);
/// Buffer should support sparse backing with partial residency
pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkBufferCreateFlagBits = VkBufferCreateFlagBits(1 << 1);
/// Alias of [`VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`]
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: VkBufferCreateFlagBits = VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
/// Alias of [`VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`]
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkBufferCreateFlagBits = VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
impl core::fmt::Debug for VkBufferCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (32, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT"),
      (16, "DEVICE_ADDRESS_CAPTURE_REPLAY"),
      (8, "PROTECTED"),
      (4, "SPARSE_ALIASED"),
      (1, "SPARSE_BINDING"),
      (2, "SPARSE_RESIDENCY"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html) (bitmask)
  VkBufferUsageFlagBits(u32)
);
/// Khronos: [VkBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html) (bitmask)
pub type VkBufferUsageFlags = VkBufferUsageFlagBits;
pub const VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 19);
pub const VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 20);
/// Specifies the buffer can be used as predicate in conditional rendering
pub const VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 9);
/// Can be used as source of fixed-function index fetch (index buffer)
pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 6);
/// Can be the source of indirect parameters (e.g. indirect buffer, parameter buffer)
pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 8);
pub const VK_BUFFER_USAGE_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 23);
pub const VK_BUFFER_USAGE_MICROMAP_STORAGE_BIT_EXT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 24);
pub const VK_BUFFER_USAGE_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 26);
pub const VK_BUFFER_USAGE_RESERVED_18_BIT_QCOM: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 18);
pub const VK_BUFFER_USAGE_RESERVED_25_BIT_AMD: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 25);
pub const VK_BUFFER_USAGE_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 22);
pub const VK_BUFFER_USAGE_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 21);
pub const VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 10);
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 17);
/// Can be used as SSBO
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 5);
/// Can be used as IBO
pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 3);
/// Can be used as a destination of transfer operations
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 1);
/// Can be used as a source of transfer operations
pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 0);
pub const VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 11);
pub const VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 12);
/// Can be used as UBO
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 4);
/// Can be used as TBO
pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 2);
/// Can be used as source of fixed-function vertex fetch (VBO)
pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 7);
pub const VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 14);
pub const VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1 << 13);
/// Alias of [`VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR`]
pub const VK_BUFFER_USAGE_RAY_TRACING_BIT_NV: VkBufferUsageFlagBits = VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR;
/// Alias of [`VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT`]
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT: VkBufferUsageFlagBits = VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT;
/// Alias of [`VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT`]
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR: VkBufferUsageFlagBits = VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT;
impl core::fmt::Debug for VkBufferUsageFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (524288, "ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR"),
      (1048576, "ACCELERATION_STRUCTURE_STORAGE_BIT_KHR"),
      (512, "CONDITIONAL_RENDERING_BIT_EXT"),
      (64, "INDEX_BUFFER"),
      (256, "INDIRECT_BUFFER"),
      (8388608, "MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT"),
      (16777216, "MICROMAP_STORAGE_BIT_EXT"),
      (67108864, "PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT"),
      (262144, "RESERVED_18_BIT_QCOM"),
      (33554432, "RESERVED_25_BIT_AMD"),
      (4194304, "RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT"),
      (2097152, "SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT"),
      (1024, "SHADER_BINDING_TABLE_BIT_KHR"),
      (131072, "SHADER_DEVICE_ADDRESS"),
      (32, "STORAGE_BUFFER"),
      (8, "STORAGE_TEXEL_BUFFER"),
      (2, "TRANSFER_DST"),
      (1, "TRANSFER_SRC"),
      (2048, "TRANSFORM_FEEDBACK_BUFFER_BIT_EXT"),
      (4096, "TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT"),
      (16, "UNIFORM_BUFFER"),
      (4, "UNIFORM_TEXEL_BUFFER"),
      (128, "VERTEX_BUFFER"),
      (16384, "VIDEO_DECODE_DST_BIT_KHR"),
      (8192, "VIDEO_DECODE_SRC_BIT_KHR"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkBufferViewCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateFlags.html) (bitmask, no bits defined)
  VkBufferViewCreateFlagBits(u32)
);
/// Khronos: [VkBufferViewCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateFlags.html) (bitmask, no bits defined)
pub type VkBufferViewCreateFlags = VkBufferViewCreateFlagBits;
impl core::fmt::Debug for VkBufferViewCreateFlagBits {
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
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 1);
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 7);
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 8);
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_EXT: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 6);
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 0);
pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 4);
pub const VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 5);
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 3);
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 2);
pub const VK_BUILD_ACCELERATION_STRUCTURE_RESERVED_BIT_10_NV: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 10);
pub const VK_BUILD_ACCELERATION_STRUCTURE_RESERVED_BIT_9_NV: VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagBitsKHR(1 << 9);
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR;
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR;
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR;
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR;
/// Alias of [`VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR`]
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR;
impl core::fmt::Debug for VkBuildAccelerationStructureFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "ALLOW_COMPACTION"),
      (128, "ALLOW_DISABLE_OPACITY_MICROMAPS_EXT"),
      (256, "ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT"),
      (64, "ALLOW_OPACITY_MICROMAP_UPDATE_EXT"),
      (1, "ALLOW_UPDATE"),
      (16, "LOW_MEMORY"),
      (32, "MOTION_BIT_NV"),
      (8, "PREFER_FAST_BUILD"),
      (4, "PREFER_FAST_TRACE"),
      (1024, "RESERVED_BIT_10_NV"),
      (512, "RESERVED_BIT_9_NV"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkBuildMicromapFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapFlagBitsEXT.html) (bitmask)
  VkBuildMicromapFlagBitsEXT(u32)
);
/// Khronos: [VkBuildMicromapFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapFlagBitsEXT.html) (bitmask)
pub type VkBuildMicromapFlagsEXT = VkBuildMicromapFlagBitsEXT;
pub const VK_BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT: VkBuildMicromapFlagBitsEXT = VkBuildMicromapFlagBitsEXT(1 << 2);
pub const VK_BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT: VkBuildMicromapFlagBitsEXT = VkBuildMicromapFlagBitsEXT(1 << 1);
pub const VK_BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT: VkBuildMicromapFlagBitsEXT = VkBuildMicromapFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkBuildMicromapFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "ALLOW_COMPACTION"),
      (2, "PREFER_FAST_BUILD"),
      (1, "PREFER_FAST_TRACE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkColorComponentFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html) (bitmask)
  VkColorComponentFlagBits(u32)
);
/// Khronos: [VkColorComponentFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html) (bitmask)
pub type VkColorComponentFlags = VkColorComponentFlagBits;
pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1 << 3);
pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1 << 2);
pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1 << 1);
pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1 << 0);
impl core::fmt::Debug for VkColorComponentFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "A"),
      (4, "B"),
      (2, "G"),
      (1, "R"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkCommandBufferResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html) (bitmask)
  VkCommandBufferResetFlagBits(u32)
);
/// Khronos: [VkCommandBufferResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html) (bitmask)
pub type VkCommandBufferResetFlags = VkCommandBufferResetFlagBits;
/// Release resources owned by the buffer
pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: VkCommandBufferResetFlagBits = VkCommandBufferResetFlagBits(1 << 0);
impl core::fmt::Debug for VkCommandBufferResetFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "RELEASE_RESOURCES"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkCommandBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html) (bitmask)
  VkCommandBufferUsageFlagBits(u32)
);
/// Khronos: [VkCommandBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html) (bitmask)
pub type VkCommandBufferUsageFlags = VkCommandBufferUsageFlagBits;
pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlagBits(1 << 0);
pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlagBits(1 << 1);
/// Command buffer may be submitted/executed more than once simultaneously
pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlagBits(1 << 2);
impl core::fmt::Debug for VkCommandBufferUsageFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "ONE_TIME_SUBMIT"),
      (2, "RENDER_PASS_CONTINUE"),
      (4, "SIMULTANEOUS_USE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkCommandPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html) (bitmask)
  VkCommandPoolCreateFlagBits(u32)
);
/// Khronos: [VkCommandPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html) (bitmask)
pub type VkCommandPoolCreateFlags = VkCommandPoolCreateFlagBits;
/// Command buffers allocated from pool are protected command buffers
pub const VK_COMMAND_POOL_CREATE_PROTECTED_BIT: VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlagBits(1 << 2);
/// Command buffers may release their memory individually
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlagBits(1 << 1);
/// Command buffers have a short lifetime
pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkCommandPoolCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "PROTECTED"),
      (2, "RESET_COMMAND_BUFFER"),
      (1, "TRANSIENT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkCommandPoolResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html) (bitmask)
  VkCommandPoolResetFlagBits(u32)
);
/// Khronos: [VkCommandPoolResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html) (bitmask)
pub type VkCommandPoolResetFlags = VkCommandPoolResetFlagBits;
/// Release resources owned by the pool
pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: VkCommandPoolResetFlagBits = VkCommandPoolResetFlagBits(1 << 0);
pub const VK_COMMAND_POOL_RESET_RESERVED_1_BIT_COREAVI: VkCommandPoolResetFlagBits = VkCommandPoolResetFlagBits(1 << 1);
impl core::fmt::Debug for VkCommandPoolResetFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "RELEASE_RESOURCES"),
      (2, "RESERVED_1_BIT_COREAVI"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkCommandPoolTrimFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolTrimFlags.html) (bitmask, no bits defined)
  VkCommandPoolTrimFlagBits(u32)
);
/// Khronos: [VkCommandPoolTrimFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolTrimFlags.html) (bitmask, no bits defined)
pub type VkCommandPoolTrimFlags = VkCommandPoolTrimFlagBits;
impl core::fmt::Debug for VkCommandPoolTrimFlagBits {
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
pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(1 << 3);
pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(1 << 0);
pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(1 << 2);
pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(1 << 1);
impl core::fmt::Debug for VkCompositeAlphaFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "INHERIT"),
      (1, "OPAQUE"),
      (4, "POST_MULTIPLIED"),
      (2, "PRE_MULTIPLIED"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkConditionalRenderingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html) (bitmask)
  VkConditionalRenderingFlagBitsEXT(u32)
);
/// Khronos: [VkConditionalRenderingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html) (bitmask)
pub type VkConditionalRenderingFlagsEXT = VkConditionalRenderingFlagBitsEXT;
pub const VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT: VkConditionalRenderingFlagBitsEXT = VkConditionalRenderingFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkConditionalRenderingFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "INVERTED"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
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
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "BACK"),
      (1, "FRONT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDebugReportFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html) (bitmask)
  VkDebugReportFlagBitsEXT(u32)
);
/// Khronos: [VkDebugReportFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html) (bitmask)
pub type VkDebugReportFlagsEXT = VkDebugReportFlagBitsEXT;
pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 4);
pub const VK_DEBUG_REPORT_ERROR_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 3);
pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 0);
pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 2);
pub const VK_DEBUG_REPORT_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkDebugReportFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (16, "DEBUG"),
      (8, "ERROR"),
      (1, "INFORMATION"),
      (4, "PERFORMANCE_WARNING"),
      (2, "WARNING"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDebugUtilsMessageSeverityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html) (bitmask)
  VkDebugUtilsMessageSeverityFlagBitsEXT(u32)
);
/// Khronos: [VkDebugUtilsMessageSeverityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html) (bitmask)
pub type VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(1 << 12);
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(1 << 4);
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(1 << 0);
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(1 << 8);
impl core::fmt::Debug for VkDebugUtilsMessageSeverityFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4096, "ERROR"),
      (16, "INFO"),
      (1, "VERBOSE"),
      (256, "WARNING"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDebugUtilsMessageTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html) (bitmask)
  VkDebugUtilsMessageTypeFlagBitsEXT(u32)
);
/// Khronos: [VkDebugUtilsMessageTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html) (bitmask)
pub type VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagBitsEXT;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_DEVICE_ADDRESS_BINDING_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(1 << 3);
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(1 << 0);
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(1 << 2);
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkDebugUtilsMessageTypeFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "DEVICE_ADDRESS_BINDING"),
      (1, "GENERAL"),
      (4, "PERFORMANCE"),
      (2, "VALIDATION"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDebugUtilsMessengerCallbackDataFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html) (bitmask, no bits defined)
  VkDebugUtilsMessengerCallbackDataFlagBitsEXT(u32)
);
/// Khronos: [VkDebugUtilsMessengerCallbackDataFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html) (bitmask, no bits defined)
pub type VkDebugUtilsMessengerCallbackDataFlagsEXT = VkDebugUtilsMessengerCallbackDataFlagBitsEXT;
impl core::fmt::Debug for VkDebugUtilsMessengerCallbackDataFlagBitsEXT {
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
pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlagBits = VkDependencyFlagBits(1 << 0);
/// Dependency is across devices
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT: VkDependencyFlagBits = VkDependencyFlagBits(1 << 2);
/// Dependency may be a feedback loop
pub const VK_DEPENDENCY_FEEDBACK_LOOP_BIT_EXT: VkDependencyFlagBits = VkDependencyFlagBits(1 << 3);
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT: VkDependencyFlagBits = VkDependencyFlagBits(1 << 1);
/// Alias of [`VK_DEPENDENCY_DEVICE_GROUP_BIT`]
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR: VkDependencyFlagBits = VK_DEPENDENCY_DEVICE_GROUP_BIT;
/// Alias of [`VK_DEPENDENCY_VIEW_LOCAL_BIT`]
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR: VkDependencyFlagBits = VK_DEPENDENCY_VIEW_LOCAL_BIT;
impl core::fmt::Debug for VkDependencyFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "BY_REGION"),
      (4, "DEVICE_GROUP"),
      (8, "FEEDBACK_LOOP_BIT_EXT"),
      (2, "VIEW_LOCAL"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDescriptorBindingFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html) (bitmask)
  VkDescriptorBindingFlagBits(u32)
);
/// Khronos: [VkDescriptorBindingFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html) (bitmask)
pub type VkDescriptorBindingFlags = VkDescriptorBindingFlagBits;
pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT: VkDescriptorBindingFlagBits = VkDescriptorBindingFlagBits(1 << 2);
pub const VK_DESCRIPTOR_BINDING_RESERVED_4_BIT_QCOM: VkDescriptorBindingFlagBits = VkDescriptorBindingFlagBits(1 << 4);
pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT: VkDescriptorBindingFlagBits = VkDescriptorBindingFlagBits(1 << 0);
pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT: VkDescriptorBindingFlagBits = VkDescriptorBindingFlagBits(1 << 1);
pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT: VkDescriptorBindingFlagBits = VkDescriptorBindingFlagBits(1 << 3);
/// Alias of [`VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT`]
pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT: VkDescriptorBindingFlagBits = VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT;
/// Alias of [`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`]
pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT: VkDescriptorBindingFlagBits = VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT;
/// Alias of [`VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`]
pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT: VkDescriptorBindingFlagBits = VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT;
/// Alias of [`VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`]
pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT: VkDescriptorBindingFlagBits = VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT;
impl core::fmt::Debug for VkDescriptorBindingFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "PARTIALLY_BOUND"),
      (16, "RESERVED_4_BIT_QCOM"),
      (1, "UPDATE_AFTER_BIND"),
      (2, "UPDATE_UNUSED_WHILE_PENDING"),
      (8, "VARIABLE_DESCRIPTOR_COUNT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDescriptorPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html) (bitmask)
  VkDescriptorPoolCreateFlagBits(u32)
);
/// Khronos: [VkDescriptorPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html) (bitmask)
pub type VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlagBits;
/// Descriptor sets may be freed individually
pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlagBits(1 << 0);
pub const VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT: VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlagBits(1 << 2);
pub const VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT: VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlagBits(1 << 1);
/// Alias of [`VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT`]
pub const VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE: VkDescriptorPoolCreateFlagBits = VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT;
/// Alias of [`VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT`]
pub const VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT: VkDescriptorPoolCreateFlagBits = VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT;
impl core::fmt::Debug for VkDescriptorPoolCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "FREE_DESCRIPTOR_SET"),
      (4, "HOST_ONLY_BIT_EXT"),
      (2, "UPDATE_AFTER_BIND"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDescriptorPoolResetFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolResetFlags.html) (bitmask, no bits defined)
  VkDescriptorPoolResetFlagBits(u32)
);
/// Khronos: [VkDescriptorPoolResetFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolResetFlags.html) (bitmask, no bits defined)
pub type VkDescriptorPoolResetFlags = VkDescriptorPoolResetFlagBits;
impl core::fmt::Debug for VkDescriptorPoolResetFlagBits {
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
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_DESCRIPTOR_BUFFER_BIT_EXT: VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 4);
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT: VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 5);
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT: VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 2);
/// Descriptors are pushed via flink:vkCmdPushDescriptorSetKHR
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR: VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 0);
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_RESERVED_3_BIT_AMD: VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 3);
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT: VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlagBits(1 << 1);
/// Alias of [`VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT`]
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE: VkDescriptorSetLayoutCreateFlagBits = VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT;
/// Alias of [`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`]
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT: VkDescriptorSetLayoutCreateFlagBits = VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT;
impl core::fmt::Debug for VkDescriptorSetLayoutCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (16, "DESCRIPTOR_BUFFER_BIT_EXT"),
      (32, "EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT"),
      (4, "HOST_ONLY_POOL_BIT_EXT"),
      (1, "PUSH_DESCRIPTOR_BIT_KHR"),
      (8, "RESERVED_3_BIT_AMD"),
      (2, "UPDATE_AFTER_BIND_POOL"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDescriptorUpdateTemplateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html) (bitmask, no bits defined)
  VkDescriptorUpdateTemplateCreateFlagBits(u32)
);
/// Khronos: [VkDescriptorUpdateTemplateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html) (bitmask, no bits defined)
pub type VkDescriptorUpdateTemplateCreateFlags = VkDescriptorUpdateTemplateCreateFlagBits;
impl core::fmt::Debug for VkDescriptorUpdateTemplateCreateFlagBits {
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
pub const VK_DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT: VkDeviceAddressBindingFlagBitsEXT = VkDeviceAddressBindingFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkDeviceAddressBindingFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "INTERNAL_OBJECT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDeviceCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlags.html) (bitmask, no bits defined)
  VkDeviceCreateFlagBits(u32)
);
/// Khronos: [VkDeviceCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlags.html) (bitmask, no bits defined)
pub type VkDeviceCreateFlags = VkDeviceCreateFlagBits;
impl core::fmt::Debug for VkDeviceCreateFlagBits {
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
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV: VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagBitsNV(1 << 2);
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV: VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagBitsNV(1 << 1);
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV: VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagBitsNV(1 << 0);
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV: VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagBitsNV(1 << 3);
impl core::fmt::Debug for VkDeviceDiagnosticsConfigFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "ENABLE_AUTOMATIC_CHECKPOINTS"),
      (2, "ENABLE_RESOURCE_TRACKING"),
      (1, "ENABLE_SHADER_DEBUG_INFO"),
      (8, "ENABLE_SHADER_ERROR_REPORTING"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDeviceGroupPresentModeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) (bitmask)
  VkDeviceGroupPresentModeFlagBitsKHR(u32)
);
/// Khronos: [VkDeviceGroupPresentModeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) (bitmask)
pub type VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagBitsKHR;
/// Present from local memory
pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR: VkDeviceGroupPresentModeFlagBitsKHR = VkDeviceGroupPresentModeFlagBitsKHR(1 << 0);
/// Each physical device presents from local memory
pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR: VkDeviceGroupPresentModeFlagBitsKHR = VkDeviceGroupPresentModeFlagBitsKHR(1 << 3);
/// Present from remote memory
pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR: VkDeviceGroupPresentModeFlagBitsKHR = VkDeviceGroupPresentModeFlagBitsKHR(1 << 1);
/// Present sum of local and/or remote memory
pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR: VkDeviceGroupPresentModeFlagBitsKHR = VkDeviceGroupPresentModeFlagBitsKHR(1 << 2);
impl core::fmt::Debug for VkDeviceGroupPresentModeFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "LOCAL"),
      (8, "LOCAL_MULTI_DEVICE"),
      (2, "REMOTE"),
      (4, "SUM"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDeviceMemoryReportFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportFlagsEXT.html) (bitmask, no bits defined)
  VkDeviceMemoryReportFlagBitsEXT(u32)
);
/// Khronos: [VkDeviceMemoryReportFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportFlagsEXT.html) (bitmask, no bits defined)
pub type VkDeviceMemoryReportFlagsEXT = VkDeviceMemoryReportFlagBitsEXT;
impl core::fmt::Debug for VkDeviceMemoryReportFlagBitsEXT {
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
pub const VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT: VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlagBits(1 << 0);
pub const VK_DEVICE_QUEUE_CREATE_RESERVED_1_BIT_QCOM: VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlagBits(1 << 1);
impl core::fmt::Debug for VkDeviceQueueCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "PROTECTED"),
      (2, "RESERVED_1_BIT_QCOM"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDirectDriverLoadingFlagsLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingFlagsLUNARG.html) (bitmask, no bits defined)
  VkDirectDriverLoadingFlagBitsLUNARG(u32)
);
/// Khronos: [VkDirectDriverLoadingFlagsLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingFlagsLUNARG.html) (bitmask, no bits defined)
pub type VkDirectDriverLoadingFlagsLUNARG = VkDirectDriverLoadingFlagBitsLUNARG;
impl core::fmt::Debug for VkDirectDriverLoadingFlagBitsLUNARG {
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
pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagBitsKHR(1 << 1);
pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagBitsKHR(1 << 0);
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagBitsKHR(1 << 2);
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagBitsKHR(1 << 3);
impl core::fmt::Debug for VkDisplayPlaneAlphaFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "GLOBAL"),
      (1, "OPAQUE"),
      (4, "PER_PIXEL"),
      (8, "PER_PIXEL_PREMULTIPLIED"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkDisplaySurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
  VkDisplaySurfaceCreateFlagBitsKHR(u32)
);
/// Khronos: [VkDisplaySurfaceCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkDisplaySurfaceCreateFlagsKHR = VkDisplaySurfaceCreateFlagBitsKHR;
impl core::fmt::Debug for VkDisplaySurfaceCreateFlagBitsKHR {
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
pub const VK_EVENT_CREATE_DEVICE_ONLY_BIT: VkEventCreateFlagBits = VkEventCreateFlagBits(1 << 0);
/// Alias of [`VK_EVENT_CREATE_DEVICE_ONLY_BIT`]
pub const VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR: VkEventCreateFlagBits = VK_EVENT_CREATE_DEVICE_ONLY_BIT;
impl core::fmt::Debug for VkEventCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "DEVICE_ONLY"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkExportMetalObjectTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectTypeFlagBitsEXT.html) (bitmask)
  VkExportMetalObjectTypeFlagBitsEXT(u32)
);
/// Khronos: [VkExportMetalObjectTypeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectTypeFlagBitsEXT.html) (bitmask)
pub type VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagBitsEXT;
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 2);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 1);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 0);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 4);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 5);
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT: VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagBitsEXT(1 << 3);
impl core::fmt::Debug for VkExportMetalObjectTypeFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "METAL_BUFFER"),
      (2, "METAL_COMMAND_QUEUE"),
      (1, "METAL_DEVICE"),
      (16, "METAL_IOSURFACE"),
      (32, "METAL_SHARED_EVENT"),
      (8, "METAL_TEXTURE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkExternalFenceFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html) (bitmask)
  VkExternalFenceFeatureFlagBits(u32)
);
/// Khronos: [VkExternalFenceFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html) (bitmask)
pub type VkExternalFenceFeatureFlags = VkExternalFenceFeatureFlagBits;
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT: VkExternalFenceFeatureFlagBits = VkExternalFenceFeatureFlagBits(1 << 0);
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT: VkExternalFenceFeatureFlagBits = VkExternalFenceFeatureFlagBits(1 << 1);
/// Alias of [`VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT`]
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagBits = VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT;
/// Alias of [`VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT`]
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagBits = VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT;
impl core::fmt::Debug for VkExternalFenceFeatureFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "EXPORTABLE"),
      (2, "IMPORTABLE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkExternalFenceHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html) (bitmask)
  VkExternalFenceHandleTypeFlagBits(u32)
);
/// Khronos: [VkExternalFenceHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html) (bitmask)
pub type VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlagBits;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 0);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 1);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 2);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SCI_SYNC_FENCE_BIT_NV: VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 5);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SCI_SYNC_OBJ_BIT_NV: VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 4);
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBits(1 << 3);
/// Alias of [`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT`]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalFenceHandleTypeFlagBits = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT;
/// Alias of [`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalFenceHandleTypeFlagBits = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT;
/// Alias of [`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalFenceHandleTypeFlagBits = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT;
/// Alias of [`VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT`]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR: VkExternalFenceHandleTypeFlagBits = VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT;
impl core::fmt::Debug for VkExternalFenceHandleTypeFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "OPAQUE_FD"),
      (2, "OPAQUE_WIN32"),
      (4, "OPAQUE_WIN32_KMT"),
      (32, "SCI_SYNC_FENCE_BIT_NV"),
      (16, "SCI_SYNC_OBJ_BIT_NV"),
      (8, "SYNC_FD"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkExternalMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html) (bitmask)
  VkExternalMemoryFeatureFlagBits(u32)
);
/// Khronos: [VkExternalMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html) (bitmask)
pub type VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlagBits;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT: VkExternalMemoryFeatureFlagBits = VkExternalMemoryFeatureFlagBits(1 << 0);
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT: VkExternalMemoryFeatureFlagBits = VkExternalMemoryFeatureFlagBits(1 << 1);
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT: VkExternalMemoryFeatureFlagBits = VkExternalMemoryFeatureFlagBits(1 << 2);
/// Alias of [`VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT`]
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR: VkExternalMemoryFeatureFlagBits = VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT`]
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagBits = VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT`]
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagBits = VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT;
impl core::fmt::Debug for VkExternalMemoryFeatureFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "DEDICATED_ONLY"),
      (2, "EXPORTABLE"),
      (4, "IMPORTABLE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkExternalMemoryFeatureFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html) (bitmask)
  VkExternalMemoryFeatureFlagBitsNV(u32)
);
/// Khronos: [VkExternalMemoryFeatureFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html) (bitmask)
pub type VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagBitsNV;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV: VkExternalMemoryFeatureFlagBitsNV = VkExternalMemoryFeatureFlagBitsNV(1 << 0);
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagBitsNV = VkExternalMemoryFeatureFlagBitsNV(1 << 1);
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagBitsNV = VkExternalMemoryFeatureFlagBitsNV(1 << 2);
impl core::fmt::Debug for VkExternalMemoryFeatureFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "DEDICATED_ONLY"),
      (2, "EXPORTABLE"),
      (4, "IMPORTABLE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkExternalMemoryHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html) (bitmask)
  VkExternalMemoryHandleTypeFlagBits(u32)
);
/// Khronos: [VkExternalMemoryHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html) (bitmask)
pub type VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlagBits;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 10);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 3);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 4);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 5);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 6);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 9);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 7);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 8);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 0);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 1);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 2);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 12);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_SCI_BUF_BIT_NV: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 13);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA: VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBits(1 << 11);
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT;
/// Alias of [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT;
impl core::fmt::Debug for VkExternalMemoryHandleTypeFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1024, "ANDROID_HARDWARE_BUFFER_BIT_ANDROID"),
      (8, "D3D11_TEXTURE"),
      (16, "D3D11_TEXTURE_KMT"),
      (32, "D3D12_HEAP"),
      (64, "D3D12_RESOURCE"),
      (512, "DMA_BUF_BIT_EXT"),
      (128, "HOST_ALLOCATION_BIT_EXT"),
      (256, "HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT"),
      (1, "OPAQUE_FD"),
      (2, "OPAQUE_WIN32"),
      (4, "OPAQUE_WIN32_KMT"),
      (4096, "RDMA_ADDRESS_BIT_NV"),
      (8192, "SCI_BUF_BIT_NV"),
      (2048, "ZIRCON_VMO_BIT_FUCHSIA"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkExternalMemoryHandleTypeFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html) (bitmask)
  VkExternalMemoryHandleTypeFlagBitsNV(u32)
);
/// Khronos: [VkExternalMemoryHandleTypeFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html) (bitmask)
pub type VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagBitsNV;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV: VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagBitsNV(1 << 2);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagBitsNV(1 << 3);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV: VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagBitsNV(1 << 0);
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagBitsNV(1 << 1);
impl core::fmt::Debug for VkExternalMemoryHandleTypeFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "D3D11_IMAGE"),
      (8, "D3D11_IMAGE_KMT"),
      (1, "OPAQUE_WIN32"),
      (2, "OPAQUE_WIN32_KMT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkExternalSemaphoreFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html) (bitmask)
  VkExternalSemaphoreFeatureFlagBits(u32)
);
/// Khronos: [VkExternalSemaphoreFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html) (bitmask)
pub type VkExternalSemaphoreFeatureFlags = VkExternalSemaphoreFeatureFlagBits;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT: VkExternalSemaphoreFeatureFlagBits = VkExternalSemaphoreFeatureFlagBits(1 << 0);
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT: VkExternalSemaphoreFeatureFlagBits = VkExternalSemaphoreFeatureFlagBits(1 << 1);
/// Alias of [`VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR: VkExternalSemaphoreFeatureFlagBits = VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR: VkExternalSemaphoreFeatureFlagBits = VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT;
impl core::fmt::Debug for VkExternalSemaphoreFeatureFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "EXPORTABLE"),
      (2, "IMPORTABLE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkExternalSemaphoreHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html) (bitmask)
  VkExternalSemaphoreHandleTypeFlagBits(u32)
);
/// Khronos: [VkExternalSemaphoreHandleTypeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html) (bitmask)
pub type VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlagBits;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 3);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 0);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 1);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 2);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SCI_SYNC_OBJ_BIT_NV: VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 5);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 4);
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA: VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBits(1 << 7);
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT;
/// Alias of [`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT`]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT;
impl core::fmt::Debug for VkExternalSemaphoreHandleTypeFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "D3D12_FENCE"),
      (1, "OPAQUE_FD"),
      (2, "OPAQUE_WIN32"),
      (4, "OPAQUE_WIN32_KMT"),
      (32, "SCI_SYNC_OBJ_BIT_NV"),
      (16, "SYNC_FD"),
      (128, "ZIRCON_EVENT_BIT_FUCHSIA"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkFenceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html) (bitmask)
  VkFenceCreateFlagBits(u32)
);
/// Khronos: [VkFenceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html) (bitmask)
pub type VkFenceCreateFlags = VkFenceCreateFlagBits;
pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlagBits = VkFenceCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkFenceCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "SIGNALED"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkFenceImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html) (bitmask)
  VkFenceImportFlagBits(u32)
);
/// Khronos: [VkFenceImportFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html) (bitmask)
pub type VkFenceImportFlags = VkFenceImportFlagBits;
pub const VK_FENCE_IMPORT_TEMPORARY_BIT: VkFenceImportFlagBits = VkFenceImportFlagBits(1 << 0);
/// Alias of [`VK_FENCE_IMPORT_TEMPORARY_BIT`]
pub const VK_FENCE_IMPORT_TEMPORARY_BIT_KHR: VkFenceImportFlagBits = VK_FENCE_IMPORT_TEMPORARY_BIT;
impl core::fmt::Debug for VkFenceImportFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "TEMPORARY"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkFormatFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html) (bitmask)
  VkFormatFeatureFlagBits(u32)
);
/// Khronos: [VkFormatFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html) (bitmask)
pub type VkFormatFeatureFlags = VkFormatFeatureFlagBits;
pub const VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 29);
/// Format can be used as the destination image of blits with vkCmdBlitImage
pub const VK_FORMAT_FEATURE_BLIT_DST_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 11);
/// Format can be used as the source image of blits with vkCmdBlitImage
pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 10);
/// Format can be used for color attachment images
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 7);
/// Format supports blending in case it is used for color attachment images
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 8);
/// Format can have cosited rather than midpoint chroma samples
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 23);
/// Format can be used for depth/stencil attachment images
pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 9);
/// Format supports disjoint planes
pub const VK_FORMAT_FEATURE_DISJOINT_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 22);
pub const VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 24);
pub const VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 30);
/// Format can have midpoint rather than cosited chroma samples
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 17);
/// Format can be used for sampled images (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 0);
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 13);
/// Format can be filtered with VK_FILTER_LINEAR when being sampled
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 12);
/// Format can be used with min/max reduction filtering
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 16);
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 20);
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 21);
/// Format can be used with linear filtering whilst color conversion is enabled
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 18);
/// Format can have different chroma, min and mag filters
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 19);
/// Format supports atomic operations in case it is used for storage images
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 2);
/// Format can be used for storage images (STORAGE_IMAGE descriptor type)
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 1);
/// Format supports atomic operations in case it is used for storage texel buffers
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 5);
/// Format can be used for storage texel buffers (IBOs)
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 4);
/// Format can be used as the destination image of image transfer commands
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 15);
/// Format can be used as the source image of image transfer commands
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 14);
/// Format can be used for uniform texel buffers (TBOs)
pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 3);
/// Format can be used for vertex buffers (VBOs)
pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 6);
pub const VK_FORMAT_FEATURE_VIDEO_DECODE_DPB_BIT_KHR: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 26);
pub const VK_FORMAT_FEATURE_VIDEO_DECODE_OUTPUT_BIT_KHR: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1 << 25);
/// Alias of [`VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`]
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT;
/// Alias of [`VK_FORMAT_FEATURE_DISJOINT_BIT`]
pub const VK_FORMAT_FEATURE_DISJOINT_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_DISJOINT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT`]
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`]
///
/// Format can be filtered with VK_FILTER_CUBIC_IMG when being sampled
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT`]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_TRANSFER_DST_BIT`]
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_TRANSFER_DST_BIT;
/// Alias of [`VK_FORMAT_FEATURE_TRANSFER_SRC_BIT`]
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_TRANSFER_SRC_BIT;
impl core::fmt::Debug for VkFormatFeatureFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (536870912, "ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR"),
      (2048, "BLIT_DST"),
      (1024, "BLIT_SRC"),
      (128, "COLOR_ATTACHMENT"),
      (256, "COLOR_ATTACHMENT_BLEND"),
      (8388608, "COSITED_CHROMA_SAMPLES"),
      (512, "DEPTH_STENCIL_ATTACHMENT"),
      (4194304, "DISJOINT"),
      (16777216, "FRAGMENT_DENSITY_MAP_BIT_EXT"),
      (1073741824, "FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR"),
      (131072, "MIDPOINT_CHROMA_SAMPLES"),
      (1, "SAMPLED_IMAGE"),
      (8192, "SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT"),
      (4096, "SAMPLED_IMAGE_FILTER_LINEAR"),
      (65536, "SAMPLED_IMAGE_FILTER_MINMAX"),
      (1048576, "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT"),
      (2097152, "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE"),
      (262144, "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER"),
      (524288, "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER"),
      (4, "STORAGE_IMAGE_ATOMIC"),
      (2, "STORAGE_IMAGE"),
      (32, "STORAGE_TEXEL_BUFFER_ATOMIC"),
      (16, "STORAGE_TEXEL_BUFFER"),
      (32768, "TRANSFER_DST"),
      (16384, "TRANSFER_SRC"),
      (8, "UNIFORM_TEXEL_BUFFER"),
      (64, "VERTEX_BUFFER"),
      (67108864, "VIDEO_DECODE_DPB_BIT_KHR"),
      (33554432, "VIDEO_DECODE_OUTPUT_BIT_KHR"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkFormatFeatureFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits2.html) (bitmask)
  VkFormatFeatureFlagBits2(u64)
);
/// Khronos: [VkFormatFeatureFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits2.html) (bitmask)
pub type VkFormatFeatureFlags2 = VkFormatFeatureFlagBits2;
pub const VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 29);
pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 11);
pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 10);
pub const VK_FORMAT_FEATURE_2_BLOCK_MATCHING_BIT_QCOM: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 36);
pub const VK_FORMAT_FEATURE_2_BOX_FILTER_SAMPLED_BIT_QCOM: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 37);
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 7);
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 8);
pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 23);
pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 9);
pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 22);
pub const VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 24);
pub const VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 30);
/// Format support linear image as render target, it cannot be mixed with non linear attachment
pub const VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 38);
pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 17);
pub const VK_FORMAT_FEATURE_2_OPTICAL_FLOW_COST_BIT_NV: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 42);
pub const VK_FORMAT_FEATURE_2_OPTICAL_FLOW_IMAGE_BIT_NV: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 40);
pub const VK_FORMAT_FEATURE_2_OPTICAL_FLOW_VECTOR_BIT_NV: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 41);
pub const VK_FORMAT_FEATURE_2_RESERVED_39_BIT_EXT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 39);
pub const VK_FORMAT_FEATURE_2_RESERVED_44_BIT_EXT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 44);
pub const VK_FORMAT_FEATURE_2_RESERVED_45_BIT_EXT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 45);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 0);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 33);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 13);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 12);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 16);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 20);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 21);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 18);
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 19);
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 2);
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 1);
pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 31);
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 5);
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 4);
pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 32);
pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 15);
pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 14);
pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 3);
pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 6);
pub const VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 26);
pub const VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 25);
pub const VK_FORMAT_FEATURE_2_WEIGHT_IMAGE_BIT_QCOM: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 34);
pub const VK_FORMAT_FEATURE_2_WEIGHT_SAMPLED_IMAGE_BIT_QCOM: VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2(1 << 35);
/// Alias of [`VK_FORMAT_FEATURE_2_BLIT_DST_BIT`]
pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_BLIT_DST_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_BLIT_SRC_BIT`]
pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_BLIT_SRC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT`]
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT`]
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT`]
pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT`]
pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_DISJOINT_BIT`]
pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_DISJOINT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT`]
pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT`]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`]
pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT`]
pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT`]
pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT`]
pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT;
/// Alias of [`VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT`]
pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 = VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT;
impl core::fmt::Debug for VkFormatFeatureFlagBits2 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (536870912, "ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR"),
      (2048, "BLIT_DST"),
      (1024, "BLIT_SRC"),
      (68719476736, "BLOCK_MATCHING_BIT_QCOM"),
      (137438953472, "BOX_FILTER_SAMPLED_BIT_QCOM"),
      (128, "COLOR_ATTACHMENT"),
      (256, "COLOR_ATTACHMENT_BLEND"),
      (8388608, "COSITED_CHROMA_SAMPLES"),
      (512, "DEPTH_STENCIL_ATTACHMENT"),
      (4194304, "DISJOINT"),
      (16777216, "FRAGMENT_DENSITY_MAP_BIT_EXT"),
      (1073741824, "FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR"),
      (274877906944, "LINEAR_COLOR_ATTACHMENT_BIT_NV"),
      (131072, "MIDPOINT_CHROMA_SAMPLES"),
      (4398046511104, "OPTICAL_FLOW_COST_BIT_NV"),
      (1099511627776, "OPTICAL_FLOW_IMAGE_BIT_NV"),
      (2199023255552, "OPTICAL_FLOW_VECTOR_BIT_NV"),
      (549755813888, "RESERVED_39_BIT_EXT"),
      (17592186044416, "RESERVED_44_BIT_EXT"),
      (35184372088832, "RESERVED_45_BIT_EXT"),
      (1, "SAMPLED_IMAGE"),
      (8589934592, "SAMPLED_IMAGE_DEPTH_COMPARISON"),
      (8192, "SAMPLED_IMAGE_FILTER_CUBIC"),
      (4096, "SAMPLED_IMAGE_FILTER_LINEAR"),
      (65536, "SAMPLED_IMAGE_FILTER_MINMAX"),
      (1048576, "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT"),
      (2097152, "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE"),
      (262144, "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER"),
      (524288, "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER"),
      (4, "STORAGE_IMAGE_ATOMIC"),
      (2, "STORAGE_IMAGE"),
      (2147483648, "STORAGE_READ_WITHOUT_FORMAT"),
      (32, "STORAGE_TEXEL_BUFFER_ATOMIC"),
      (16, "STORAGE_TEXEL_BUFFER"),
      (4294967296, "STORAGE_WRITE_WITHOUT_FORMAT"),
      (32768, "TRANSFER_DST"),
      (16384, "TRANSFER_SRC"),
      (8, "UNIFORM_TEXEL_BUFFER"),
      (64, "VERTEX_BUFFER"),
      (67108864, "VIDEO_DECODE_DPB_BIT_KHR"),
      (33554432, "VIDEO_DECODE_OUTPUT_BIT_KHR"),
      (17179869184, "WEIGHT_IMAGE_BIT_QCOM"),
      (34359738368, "WEIGHT_SAMPLED_IMAGE_BIT_QCOM"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkFramebufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlagBits.html) (bitmask)
  VkFramebufferCreateFlagBits(u32)
);
/// Khronos: [VkFramebufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlagBits.html) (bitmask)
pub type VkFramebufferCreateFlags = VkFramebufferCreateFlagBits;
pub const VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT: VkFramebufferCreateFlagBits = VkFramebufferCreateFlagBits(1 << 0);
/// Alias of [`VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`]
pub const VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR: VkFramebufferCreateFlagBits = VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT;
impl core::fmt::Debug for VkFramebufferCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "IMAGELESS"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkGeometryFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html) (bitmask)
  VkGeometryFlagBitsKHR(u32)
);
/// Khronos: [VkGeometryFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html) (bitmask)
pub type VkGeometryFlagsKHR = VkGeometryFlagBitsKHR;
pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR: VkGeometryFlagBitsKHR = VkGeometryFlagBitsKHR(1 << 1);
pub const VK_GEOMETRY_OPAQUE_BIT_KHR: VkGeometryFlagBitsKHR = VkGeometryFlagBitsKHR(1 << 0);
/// Alias of [`VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR`]
pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV: VkGeometryFlagBitsKHR = VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR;
/// Alias of [`VK_GEOMETRY_OPAQUE_BIT_KHR`]
pub const VK_GEOMETRY_OPAQUE_BIT_NV: VkGeometryFlagBitsKHR = VK_GEOMETRY_OPAQUE_BIT_KHR;
impl core::fmt::Debug for VkGeometryFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "NO_DUPLICATE_ANY_HIT_INVOCATION"),
      (1, "OPAQUE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkGeometryInstanceFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html) (bitmask)
  VkGeometryInstanceFlagBitsKHR(u32)
);
/// Khronos: [VkGeometryInstanceFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html) (bitmask)
pub type VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagBitsKHR;
pub const VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_EXT: VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 5);
pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR: VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 3);
pub const VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_EXT: VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 4);
pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR: VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 2);
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR: VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 0);
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR: VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagBitsKHR(1 << 1);
/// Alias of [`VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV: VkGeometryInstanceFlagBitsKHR = VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR;
/// Alias of [`VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV: VkGeometryInstanceFlagBitsKHR = VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR;
/// Alias of [`VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV: VkGeometryInstanceFlagBitsKHR = VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR;
/// Alias of [`VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR: VkGeometryInstanceFlagBitsKHR = VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR;
/// Alias of [`VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR`]
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV: VkGeometryInstanceFlagBitsKHR = VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR;
impl core::fmt::Debug for VkGeometryInstanceFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (32, "DISABLE_OPACITY_MICROMAPS_EXT"),
      (8, "FORCE_NO_OPAQUE"),
      (16, "FORCE_OPACITY_MICROMAP_2_STATE_EXT"),
      (4, "FORCE_OPAQUE"),
      (1, "TRIANGLE_FACING_CULL_DISABLE"),
      (2, "TRIANGLE_FLIP_FACING"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkGraphicsPipelineLibraryFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryFlagBitsEXT.html) (bitmask)
  VkGraphicsPipelineLibraryFlagBitsEXT(u32)
);
/// Khronos: [VkGraphicsPipelineLibraryFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryFlagBitsEXT.html) (bitmask)
pub type VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagBitsEXT;
pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT: VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagBitsEXT(1 << 3);
pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT: VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagBitsEXT(1 << 2);
pub const VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT: VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagBitsEXT(1 << 1);
pub const VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT: VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkGraphicsPipelineLibraryFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "FRAGMENT_OUTPUT_INTERFACE"),
      (4, "FRAGMENT_SHADER"),
      (2, "PRE_RASTERIZATION_SHADERS"),
      (1, "VERTEX_INPUT_INTERFACE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkHeadlessSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
  VkHeadlessSurfaceCreateFlagBitsEXT(u32)
);
/// Khronos: [VkHeadlessSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkHeadlessSurfaceCreateFlagsEXT = VkHeadlessSurfaceCreateFlagBitsEXT;
impl core::fmt::Debug for VkHeadlessSurfaceCreateFlagBitsEXT {
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
pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 0);
pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 1);
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 7);
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 8);
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 9);
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 10);
pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 3);
pub const VK_IMAGE_ASPECT_PLANE_0_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 4);
pub const VK_IMAGE_ASPECT_PLANE_1_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 5);
pub const VK_IMAGE_ASPECT_PLANE_2_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 6);
pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 2);
pub const VK_IMAGE_ASPECT_NONE: VkImageAspectFlagBits = VkImageAspectFlagBits(0);
/// Alias of [`VK_IMAGE_ASPECT_NONE`]
pub const VK_IMAGE_ASPECT_NONE_KHR: VkImageAspectFlagBits = VK_IMAGE_ASPECT_NONE;
/// Alias of [`VK_IMAGE_ASPECT_PLANE_0_BIT`]
pub const VK_IMAGE_ASPECT_PLANE_0_BIT_KHR: VkImageAspectFlagBits = VK_IMAGE_ASPECT_PLANE_0_BIT;
/// Alias of [`VK_IMAGE_ASPECT_PLANE_1_BIT`]
pub const VK_IMAGE_ASPECT_PLANE_1_BIT_KHR: VkImageAspectFlagBits = VK_IMAGE_ASPECT_PLANE_1_BIT;
/// Alias of [`VK_IMAGE_ASPECT_PLANE_2_BIT`]
pub const VK_IMAGE_ASPECT_PLANE_2_BIT_KHR: VkImageAspectFlagBits = VK_IMAGE_ASPECT_PLANE_2_BIT;
impl core::fmt::Debug for VkImageAspectFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "COLOR"),
      (2, "DEPTH"),
      (128, "MEMORY_PLANE_0_BIT_EXT"),
      (256, "MEMORY_PLANE_1_BIT_EXT"),
      (512, "MEMORY_PLANE_2_BIT_EXT"),
      (1024, "MEMORY_PLANE_3_BIT_EXT"),
      (8, "METADATA"),
      (16, "PLANE_0"),
      (32, "PLANE_1"),
      (64, "PLANE_2"),
      (4, "STENCIL"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkImageCompressionFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFlagBitsEXT.html) (bitmask)
  VkImageCompressionFlagBitsEXT(u32)
);
/// Khronos: [VkImageCompressionFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFlagBitsEXT.html) (bitmask)
pub type VkImageCompressionFlagsEXT = VkImageCompressionFlagBitsEXT;
pub const VK_IMAGE_COMPRESSION_DISABLED_EXT: VkImageCompressionFlagBitsEXT = VkImageCompressionFlagBitsEXT(1 << 2);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT: VkImageCompressionFlagBitsEXT = VkImageCompressionFlagBitsEXT(1 << 0);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT: VkImageCompressionFlagBitsEXT = VkImageCompressionFlagBitsEXT(1 << 1);
pub const VK_IMAGE_COMPRESSION_DEFAULT_EXT: VkImageCompressionFlagBitsEXT = VkImageCompressionFlagBitsEXT(0);
impl core::fmt::Debug for VkImageCompressionFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "DISABLED"),
      (1, "FIXED_RATE_DEFAULT"),
      (2, "FIXED_RATE_EXPLICIT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkImageCompressionFixedRateFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFixedRateFlagBitsEXT.html) (bitmask)
  VkImageCompressionFixedRateFlagBitsEXT(u32)
);
/// Khronos: [VkImageCompressionFixedRateFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFixedRateFlagBitsEXT.html) (bitmask)
pub type VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagBitsEXT;
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 9);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 10);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 11);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 12);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 13);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 14);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 15);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 16);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 17);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 18);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 0);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 19);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 20);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 21);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 22);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 23);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 1);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 2);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 3);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 4);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 5);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 6);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 7);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(1 << 8);
pub const VK_IMAGE_COMPRESSION_FIXED_RATE_NONE_EXT: VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagBitsEXT(0);
impl core::fmt::Debug for VkImageCompressionFixedRateFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (512, "10BPC"),
      (1024, "11BPC"),
      (2048, "12BPC"),
      (4096, "13BPC"),
      (8192, "14BPC"),
      (16384, "15BPC"),
      (32768, "16BPC"),
      (65536, "17BPC"),
      (131072, "18BPC"),
      (262144, "19BPC"),
      (1, "1BPC"),
      (524288, "20BPC"),
      (1048576, "21BPC"),
      (2097152, "22BPC"),
      (4194304, "23BPC"),
      (8388608, "24BPC"),
      (2, "2BPC"),
      (4, "3BPC"),
      (8, "4BPC"),
      (16, "5BPC"),
      (32, "6BPC"),
      (64, "7BPC"),
      (128, "8BPC"),
      (256, "9BPC"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkImageConstraintsInfoFlagBitsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html) (bitmask)
  VkImageConstraintsInfoFlagBitsFUCHSIA(u32)
);
/// Khronos: [VkImageConstraintsInfoFlagBitsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html) (bitmask)
pub type VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA;
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 1);
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 0);
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 3);
pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 2);
pub const VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA: VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagBitsFUCHSIA(1 << 4);
impl core::fmt::Debug for VkImageConstraintsInfoFlagBitsFUCHSIA {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "CPU_READ_OFTEN"),
      (1, "CPU_READ_RARELY"),
      (8, "CPU_WRITE_OFTEN"),
      (4, "CPU_WRITE_RARELY"),
      (16, "PROTECTED_OPTIONAL"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkImageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html) (bitmask)
  VkImageCreateFlagBits(u32)
);
/// Khronos: [VkImageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html) (bitmask)
pub type VkImageCreateFlags = VkImageCreateFlagBits;
/// The 3D image can be viewed as a 2D or 2D array image
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 5);
/// Image is created with a layout where individual slices are capable of being used as 2D images
pub const VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 17);
pub const VK_IMAGE_CREATE_ALIAS_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 10);
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 7);
pub const VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 13);
/// Allows creating image views with cube type from the created image
pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 4);
pub const VK_IMAGE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 16);
pub const VK_IMAGE_CREATE_DISJOINT_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 9);
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 8);
pub const VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 15);
pub const VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 18);
/// Allows image views to have different format than the base image
pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 3);
/// Image requires protected memory
pub const VK_IMAGE_CREATE_PROTECTED_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 11);
pub const VK_IMAGE_CREATE_RESERVED_19_BIT_EXT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 19);
pub const VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 12);
/// Image should support constant data access to physical memory ranges mapped into multiple locations of sparse images
pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 2);
/// Image should support sparse backing
pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 0);
/// Image should support sparse backing with partial residency
pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 1);
/// Allows using VkBindImageMemoryDeviceGroupInfo::pSplitInstanceBindRegions when binding memory to the image
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 6);
pub const VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 14);
/// Alias of [`VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT`]
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT;
/// Alias of [`VK_IMAGE_CREATE_ALIAS_BIT`]
pub const VK_IMAGE_CREATE_ALIAS_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_ALIAS_BIT;
/// Alias of [`VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT`]
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT;
/// Alias of [`VK_IMAGE_CREATE_DISJOINT_BIT`]
pub const VK_IMAGE_CREATE_DISJOINT_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_DISJOINT_BIT;
/// Alias of [`VK_IMAGE_CREATE_EXTENDED_USAGE_BIT`]
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_EXTENDED_USAGE_BIT;
/// Alias of [`VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT`]
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: VkImageCreateFlagBits = VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT;
impl core::fmt::Debug for VkImageCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (32, "2D_ARRAY_COMPATIBLE"),
      (131072, "2D_VIEW_COMPATIBLE_BIT_EXT"),
      (1024, "ALIAS"),
      (128, "BLOCK_TEXEL_VIEW_COMPATIBLE"),
      (8192, "CORNER_SAMPLED_BIT_NV"),
      (16, "CUBE_COMPATIBLE"),
      (65536, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT"),
      (512, "DISJOINT"),
      (256, "EXTENDED_USAGE"),
      (32768, "FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM"),
      (262144, "MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT"),
      (8, "MUTABLE_FORMAT"),
      (2048, "PROTECTED"),
      (524288, "RESERVED_19_BIT_EXT"),
      (4096, "SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT"),
      (4, "SPARSE_ALIASED"),
      (1, "SPARSE_BINDING"),
      (2, "SPARSE_RESIDENCY"),
      (64, "SPLIT_INSTANCE_BIND_REGIONS"),
      (16384, "SUBSAMPLED_BIT_EXT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkImageFormatConstraintsFlagsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsFlagsFUCHSIA.html) (bitmask, no bits defined)
  VkImageFormatConstraintsFlagBitsFUCHSIA(u32)
);
/// Khronos: [VkImageFormatConstraintsFlagsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsFlagsFUCHSIA.html) (bitmask, no bits defined)
pub type VkImageFormatConstraintsFlagsFUCHSIA = VkImageFormatConstraintsFlagBitsFUCHSIA;
impl core::fmt::Debug for VkImageFormatConstraintsFlagBitsFUCHSIA {
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
pub const VK_IMAGE_USAGE_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 19);
/// Can be used as framebuffer color attachment
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 4);
/// Can be used as framebuffer depth/stencil attachment
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 5);
pub const VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 9);
pub const VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 8);
/// Can be used as framebuffer input attachment
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 7);
pub const VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 18);
pub const VK_IMAGE_USAGE_RESERVED_16_BIT_QCOM: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 16);
pub const VK_IMAGE_USAGE_RESERVED_17_BIT_QCOM: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 17);
pub const VK_IMAGE_USAGE_RESERVED_22_BIT_EXT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 22);
/// Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)
pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 2);
pub const VK_IMAGE_USAGE_SAMPLE_BLOCK_MATCH_BIT_QCOM: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 21);
pub const VK_IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 20);
/// Can be used as storage image (STORAGE_IMAGE descriptor type)
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 3);
/// Can be used as a destination of transfer operations
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 1);
/// Can be used as a source of transfer operations
pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 0);
/// Image data not needed outside of rendering
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 6);
pub const VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 12);
pub const VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 10);
pub const VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 11);
/// Alias of [`VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
pub const VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV: VkImageUsageFlagBits = VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
impl core::fmt::Debug for VkImageUsageFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (524288, "ATTACHMENT_FEEDBACK_LOOP_BIT_EXT"),
      (16, "COLOR_ATTACHMENT"),
      (32, "DEPTH_STENCIL_ATTACHMENT"),
      (512, "FRAGMENT_DENSITY_MAP_BIT_EXT"),
      (256, "FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR"),
      (128, "INPUT_ATTACHMENT"),
      (262144, "INVOCATION_MASK_BIT_HUAWEI"),
      (65536, "RESERVED_16_BIT_QCOM"),
      (131072, "RESERVED_17_BIT_QCOM"),
      (4194304, "RESERVED_22_BIT_EXT"),
      (4, "SAMPLED"),
      (2097152, "SAMPLE_BLOCK_MATCH_BIT_QCOM"),
      (1048576, "SAMPLE_WEIGHT_BIT_QCOM"),
      (8, "STORAGE"),
      (2, "TRANSFER_DST"),
      (1, "TRANSFER_SRC"),
      (64, "TRANSIENT_ATTACHMENT"),
      (4096, "VIDEO_DECODE_DPB_BIT_KHR"),
      (1024, "VIDEO_DECODE_DST_BIT_KHR"),
      (2048, "VIDEO_DECODE_SRC_BIT_KHR"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkImageViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html) (bitmask)
  VkImageViewCreateFlagBits(u32)
);
/// Khronos: [VkImageViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html) (bitmask)
pub type VkImageViewCreateFlags = VkImageViewCreateFlagBits;
pub const VK_IMAGE_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: VkImageViewCreateFlagBits = VkImageViewCreateFlagBits(1 << 2);
pub const VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT: VkImageViewCreateFlagBits = VkImageViewCreateFlagBits(1 << 1);
pub const VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT: VkImageViewCreateFlagBits = VkImageViewCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkImageViewCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT"),
      (2, "FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT"),
      (1, "FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkIndirectCommandsLayoutUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html) (bitmask)
  VkIndirectCommandsLayoutUsageFlagBitsNV(u32)
);
/// Khronos: [VkIndirectCommandsLayoutUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html) (bitmask)
pub type VkIndirectCommandsLayoutUsageFlagsNV = VkIndirectCommandsLayoutUsageFlagBitsNV;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV: VkIndirectCommandsLayoutUsageFlagBitsNV = VkIndirectCommandsLayoutUsageFlagBitsNV(1 << 0);
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV: VkIndirectCommandsLayoutUsageFlagBitsNV = VkIndirectCommandsLayoutUsageFlagBitsNV(1 << 1);
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV: VkIndirectCommandsLayoutUsageFlagBitsNV = VkIndirectCommandsLayoutUsageFlagBitsNV(1 << 2);
impl core::fmt::Debug for VkIndirectCommandsLayoutUsageFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "EXPLICIT_PREPROCESS"),
      (2, "INDEXED_SEQUENCES"),
      (4, "UNORDERED_SEQUENCES"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkIndirectStateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html) (bitmask)
  VkIndirectStateFlagBitsNV(u32)
);
/// Khronos: [VkIndirectStateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html) (bitmask)
pub type VkIndirectStateFlagsNV = VkIndirectStateFlagBitsNV;
pub const VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV: VkIndirectStateFlagBitsNV = VkIndirectStateFlagBitsNV(1 << 0);
impl core::fmt::Debug for VkIndirectStateFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "FLAG_FRONTFACE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html) (bitmask)
  VkInstanceCreateFlagBits(u32)
);
/// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html) (bitmask)
pub type VkInstanceCreateFlags = VkInstanceCreateFlagBits;
pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR: VkInstanceCreateFlagBits = VkInstanceCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkInstanceCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "ENUMERATE_PORTABILITY_BIT_KHR"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkMacOSSurfaceCreateFlagsMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html) (bitmask, no bits defined)
  VkMacOSSurfaceCreateFlagBitsMVK(u32)
);
/// Khronos: [VkMacOSSurfaceCreateFlagsMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateFlagsMVK.html) (bitmask, no bits defined)
pub type VkMacOSSurfaceCreateFlagsMVK = VkMacOSSurfaceCreateFlagBitsMVK;
impl core::fmt::Debug for VkMacOSSurfaceCreateFlagBitsMVK {
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
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT: VkMemoryAllocateFlagBits = VkMemoryAllocateFlagBits(1 << 1);
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkMemoryAllocateFlagBits = VkMemoryAllocateFlagBits(1 << 2);
/// Force allocation on specific devices
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT: VkMemoryAllocateFlagBits = VkMemoryAllocateFlagBits(1 << 0);
/// Alias of [`VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT`]
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR: VkMemoryAllocateFlagBits = VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT;
/// Alias of [`VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`]
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkMemoryAllocateFlagBits = VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT;
/// Alias of [`VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT`]
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR: VkMemoryAllocateFlagBits = VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT;
impl core::fmt::Debug for VkMemoryAllocateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "DEVICE_ADDRESS"),
      (4, "DEVICE_ADDRESS_CAPTURE_REPLAY"),
      (1, "DEVICE_MASK"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkMemoryDecompressionMethodFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDecompressionMethodFlagBitsNV.html) (bitmask)
  VkMemoryDecompressionMethodFlagBitsNV(u64)
);
/// Khronos: [VkMemoryDecompressionMethodFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDecompressionMethodFlagBitsNV.html) (bitmask)
pub type VkMemoryDecompressionMethodFlagsNV = VkMemoryDecompressionMethodFlagBitsNV;
pub const VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV: VkMemoryDecompressionMethodFlagBitsNV = VkMemoryDecompressionMethodFlagBitsNV(1 << 0);
impl core::fmt::Debug for VkMemoryDecompressionMethodFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "GDEFLATE_1_0"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkMemoryHeapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html) (bitmask)
  VkMemoryHeapFlagBits(u32)
);
/// Khronos: [VkMemoryHeapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html) (bitmask)
pub type VkMemoryHeapFlags = VkMemoryHeapFlagBits;
/// If set, heap represents device memory
pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT: VkMemoryHeapFlagBits = VkMemoryHeapFlagBits(1 << 0);
/// If set, heap allocations allocate multiple instances by default
pub const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT: VkMemoryHeapFlagBits = VkMemoryHeapFlagBits(1 << 1);
/// Alias of [`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT`]
pub const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR: VkMemoryHeapFlagBits = VK_MEMORY_HEAP_MULTI_INSTANCE_BIT;
impl core::fmt::Debug for VkMemoryHeapFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "DEVICE_LOCAL"),
      (2, "MULTI_INSTANCE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkMemoryMapFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlags.html) (bitmask, no bits defined)
  VkMemoryMapFlagBits(u32)
);
/// Khronos: [VkMemoryMapFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlags.html) (bitmask, no bits defined)
pub type VkMemoryMapFlags = VkMemoryMapFlagBits;
impl core::fmt::Debug for VkMemoryMapFlagBits {
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
pub const VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 6);
/// If otherwise stated, then allocate memory on device
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 0);
pub const VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 7);
/// Memory will be cached by the host
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 3);
/// Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 2);
/// Memory is mappable by host
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 1);
/// Memory may be allocated by the driver when it is required
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 4);
/// Memory is protected
pub const VK_MEMORY_PROPERTY_PROTECTED_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 5);
pub const VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 8);
impl core::fmt::Debug for VkMemoryPropertyFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (64, "DEVICE_COHERENT_BIT_AMD"),
      (1, "DEVICE_LOCAL"),
      (128, "DEVICE_UNCACHED_BIT_AMD"),
      (8, "HOST_CACHED"),
      (4, "HOST_COHERENT"),
      (2, "HOST_VISIBLE"),
      (16, "LAZILY_ALLOCATED"),
      (32, "PROTECTED"),
      (256, "RDMA_CAPABLE_BIT_NV"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkMetalSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
  VkMetalSurfaceCreateFlagBitsEXT(u32)
);
/// Khronos: [VkMetalSurfaceCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkMetalSurfaceCreateFlagsEXT = VkMetalSurfaceCreateFlagBitsEXT;
impl core::fmt::Debug for VkMetalSurfaceCreateFlagBitsEXT {
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
pub const VK_MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: VkMicromapCreateFlagBitsEXT = VkMicromapCreateFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkMicromapCreateFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "DEVICE_ADDRESS_CAPTURE_REPLAY"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkOpticalFlowExecuteFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteFlagBitsNV.html) (bitmask)
  VkOpticalFlowExecuteFlagBitsNV(u32)
);
/// Khronos: [VkOpticalFlowExecuteFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteFlagBitsNV.html) (bitmask)
pub type VkOpticalFlowExecuteFlagsNV = VkOpticalFlowExecuteFlagBitsNV;
pub const VK_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV: VkOpticalFlowExecuteFlagBitsNV = VkOpticalFlowExecuteFlagBitsNV(1 << 0);
impl core::fmt::Debug for VkOpticalFlowExecuteFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "DISABLE_TEMPORAL_HINTS"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkOpticalFlowGridSizeFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowGridSizeFlagBitsNV.html) (bitmask)
  VkOpticalFlowGridSizeFlagBitsNV(u32)
);
/// Khronos: [VkOpticalFlowGridSizeFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowGridSizeFlagBitsNV.html) (bitmask)
pub type VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagBitsNV;
pub const VK_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV: VkOpticalFlowGridSizeFlagBitsNV = VkOpticalFlowGridSizeFlagBitsNV(1 << 0);
pub const VK_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV: VkOpticalFlowGridSizeFlagBitsNV = VkOpticalFlowGridSizeFlagBitsNV(1 << 1);
pub const VK_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV: VkOpticalFlowGridSizeFlagBitsNV = VkOpticalFlowGridSizeFlagBitsNV(1 << 2);
pub const VK_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV: VkOpticalFlowGridSizeFlagBitsNV = VkOpticalFlowGridSizeFlagBitsNV(1 << 3);
pub const VK_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV: VkOpticalFlowGridSizeFlagBitsNV = VkOpticalFlowGridSizeFlagBitsNV(0);
impl core::fmt::Debug for VkOpticalFlowGridSizeFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "1X1"),
      (2, "2X2"),
      (4, "4X4"),
      (8, "8X8"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkOpticalFlowSessionCreateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateFlagBitsNV.html) (bitmask)
  VkOpticalFlowSessionCreateFlagBitsNV(u32)
);
/// Khronos: [VkOpticalFlowSessionCreateFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateFlagBitsNV.html) (bitmask)
pub type VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagBitsNV;
pub const VK_OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV: VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 3);
pub const VK_OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV: VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 4);
pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV: VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 1);
pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV: VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 2);
pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV: VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagBitsNV(1 << 0);
impl core::fmt::Debug for VkOpticalFlowSessionCreateFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "ALLOW_REGIONS"),
      (16, "BOTH_DIRECTIONS"),
      (2, "ENABLE_COST"),
      (4, "ENABLE_GLOBAL_FLOW"),
      (1, "ENABLE_HINT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkOpticalFlowUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowUsageFlagBitsNV.html) (bitmask)
  VkOpticalFlowUsageFlagBitsNV(u32)
);
/// Khronos: [VkOpticalFlowUsageFlagBitsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowUsageFlagBitsNV.html) (bitmask)
pub type VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagBitsNV;
pub const VK_OPTICAL_FLOW_USAGE_COST_BIT_NV: VkOpticalFlowUsageFlagBitsNV = VkOpticalFlowUsageFlagBitsNV(1 << 3);
pub const VK_OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV: VkOpticalFlowUsageFlagBitsNV = VkOpticalFlowUsageFlagBitsNV(1 << 4);
pub const VK_OPTICAL_FLOW_USAGE_HINT_BIT_NV: VkOpticalFlowUsageFlagBitsNV = VkOpticalFlowUsageFlagBitsNV(1 << 2);
pub const VK_OPTICAL_FLOW_USAGE_INPUT_BIT_NV: VkOpticalFlowUsageFlagBitsNV = VkOpticalFlowUsageFlagBitsNV(1 << 0);
pub const VK_OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV: VkOpticalFlowUsageFlagBitsNV = VkOpticalFlowUsageFlagBitsNV(1 << 1);
pub const VK_OPTICAL_FLOW_USAGE_UNKNOWN_NV: VkOpticalFlowUsageFlagBitsNV = VkOpticalFlowUsageFlagBitsNV(0);
impl core::fmt::Debug for VkOpticalFlowUsageFlagBitsNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "COST"),
      (16, "GLOBAL_FLOW"),
      (4, "HINT"),
      (1, "INPUT"),
      (2, "OUTPUT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPeerMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html) (bitmask)
  VkPeerMemoryFeatureFlagBits(u32)
);
/// Khronos: [VkPeerMemoryFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html) (bitmask)
pub type VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlagBits;
/// Can write with vkCmdCopy commands
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT: VkPeerMemoryFeatureFlagBits = VkPeerMemoryFeatureFlagBits(1 << 1);
/// Can read with vkCmdCopy commands
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT: VkPeerMemoryFeatureFlagBits = VkPeerMemoryFeatureFlagBits(1 << 0);
/// Can write with and access type/command
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT: VkPeerMemoryFeatureFlagBits = VkPeerMemoryFeatureFlagBits(1 << 3);
/// Can read with any access type/command
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT: VkPeerMemoryFeatureFlagBits = VkPeerMemoryFeatureFlagBits(1 << 2);
/// Alias of [`VK_PEER_MEMORY_FEATURE_COPY_DST_BIT`]
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR: VkPeerMemoryFeatureFlagBits = VK_PEER_MEMORY_FEATURE_COPY_DST_BIT;
/// Alias of [`VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT`]
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBits = VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT;
/// Alias of [`VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT`]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR: VkPeerMemoryFeatureFlagBits = VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT;
/// Alias of [`VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT`]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBits = VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT;
impl core::fmt::Debug for VkPeerMemoryFeatureFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "COPY_DST"),
      (1, "COPY_SRC"),
      (8, "GENERIC_DST"),
      (4, "GENERIC_SRC"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPerformanceCounterDescriptionFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html) (bitmask)
  VkPerformanceCounterDescriptionFlagBitsKHR(u32)
);
/// Khronos: [VkPerformanceCounterDescriptionFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html) (bitmask)
pub type VkPerformanceCounterDescriptionFlagsKHR = VkPerformanceCounterDescriptionFlagBitsKHR;
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = VkPerformanceCounterDescriptionFlagBitsKHR(1 << 1);
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = VkPerformanceCounterDescriptionFlagBitsKHR(1 << 0);
/// Alias of [`VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR`]
#[deprecated = "aliased"]
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR;
/// Alias of [`VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR`]
#[deprecated = "aliased"]
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR;
impl core::fmt::Debug for VkPerformanceCounterDescriptionFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "CONCURRENTLY_IMPACTED"),
      (1, "PERFORMANCE_IMPACTING"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCacheCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html) (bitmask)
  VkPipelineCacheCreateFlagBits(u32)
);
/// Khronos: [VkPipelineCacheCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html) (bitmask)
pub type VkPipelineCacheCreateFlags = VkPipelineCacheCreateFlagBits;
pub const VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT: VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlagBits(1 << 0);
pub const VK_PIPELINE_CACHE_CREATE_RESERVED_1_BIT_EXT: VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlagBits(1 << 1);
/// Alias of [`VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`]
pub const VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT: VkPipelineCacheCreateFlagBits = VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT;
impl core::fmt::Debug for VkPipelineCacheCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "EXTERNALLY_SYNCHRONIZED"),
      (2, "RESERVED_1_BIT_EXT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
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
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCompilerControlFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html) (bitmask)
  VkPipelineCompilerControlFlagBitsAMD(u32)
);
/// Khronos: [VkPipelineCompilerControlFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html) (bitmask)
pub type VkPipelineCompilerControlFlagsAMD = VkPipelineCompilerControlFlagBitsAMD;
impl core::fmt::Debug for VkPipelineCompilerControlFlagBitsAMD {
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
pub type VkPipelineCoverageModulationStateCreateFlagsNV = VkPipelineCoverageModulationStateCreateFlagBitsNV;
impl core::fmt::Debug for VkPipelineCoverageModulationStateCreateFlagBitsNV {
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
pub type VkPipelineCoverageReductionStateCreateFlagsNV = VkPipelineCoverageReductionStateCreateFlagBitsNV;
impl core::fmt::Debug for VkPipelineCoverageReductionStateCreateFlagBitsNV {
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
pub type VkPipelineCoverageToColorStateCreateFlagsNV = VkPipelineCoverageToColorStateCreateFlagBitsNV;
impl core::fmt::Debug for VkPipelineCoverageToColorStateCreateFlagBitsNV {
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
pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 1);
pub const VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 7);
pub const VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 6);
pub const VK_PIPELINE_CREATE_COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 25);
pub const VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 5);
pub const VK_PIPELINE_CREATE_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 26);
pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 2);
pub const VK_PIPELINE_CREATE_DESCRIPTOR_BUFFER_BIT_EXT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 29);
pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 0);
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 4);
pub const VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 9);
pub const VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 8);
pub const VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 18);
pub const VK_PIPELINE_CREATE_LIBRARY_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 11);
pub const VK_PIPELINE_CREATE_LINK_TIME_OPTIMIZATION_BIT_EXT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 10);
pub const VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT_EXT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 27);
pub const VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT_EXT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 30);
pub const VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 20);
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 14);
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 15);
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 17);
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 16);
pub const VK_PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 24);
pub const VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 19);
pub const VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 13);
pub const VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 12);
pub const VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 22);
pub const VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 21);
pub const VK_PIPELINE_CREATE_RESERVED_BIT_28_NV: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 28);
pub const VK_PIPELINE_CREATE_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 23);
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1 << 3);
/// Alias of [`VK_PIPELINE_CREATE_DISPATCH_BASE_BIT`]
pub const VK_PIPELINE_CREATE_DISPATCH_BASE: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_DISPATCH_BASE_BIT;
/// Alias of [`VK_PIPELINE_CREATE_DISPATCH_BASE`]
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_KHR: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_DISPATCH_BASE;
/// Alias of [`VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`]
pub const VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT;
/// Alias of [`VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT`]
pub const VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT;
/// Alias of [`VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT`]
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT;
/// Alias of [`VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`]
#[deprecated = "aliased"]
pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT;
/// Alias of [`VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
#[deprecated = "aliased"]
pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineCreateFlagBits = VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
impl core::fmt::Debug for VkPipelineCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "ALLOW_DERIVATIVES"),
      (128, "CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR"),
      (64, "CAPTURE_STATISTICS_BIT_KHR"),
      (33554432, "COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT"),
      (32, "DEFER_COMPILE_BIT_NV"),
      (67108864, "DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT"),
      (4, "DERIVATIVE"),
      (536870912, "DESCRIPTOR_BUFFER_BIT_EXT"),
      (1, "DISABLE_OPTIMIZATION"),
      (16, "DISPATCH_BASE"),
      (512, "EARLY_RETURN_ON_FAILURE"),
      (256, "FAIL_ON_PIPELINE_COMPILE_REQUIRED"),
      (262144, "INDIRECT_BINDABLE_BIT_NV"),
      (2048, "LIBRARY_BIT_KHR"),
      (1024, "LINK_TIME_OPTIMIZATION_BIT_EXT"),
      (134217728, "NO_PROTECTED_ACCESS_BIT_EXT"),
      (1073741824, "PROTECTED_ACCESS_ONLY_BIT_EXT"),
      (1048576, "RAY_TRACING_ALLOW_MOTION_BIT_NV"),
      (16384, "RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR"),
      (32768, "RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR"),
      (131072, "RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR"),
      (65536, "RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR"),
      (16777216, "RAY_TRACING_OPACITY_MICROMAP_BIT_EXT"),
      (524288, "RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR"),
      (8192, "RAY_TRACING_SKIP_AABBS_BIT_KHR"),
      (4096, "RAY_TRACING_SKIP_TRIANGLES_BIT_KHR"),
      (4194304, "RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT"),
      (2097152, "RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR"),
      (268435456, "RESERVED_BIT_28_NV"),
      (8388608, "RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT"),
      (8, "VIEW_INDEX_FROM_DEVICE_INDEX"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineCreationFeedbackFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackFlagBits.html) (bitmask)
  VkPipelineCreationFeedbackFlagBits(u32)
);
/// Khronos: [VkPipelineCreationFeedbackFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackFlagBits.html) (bitmask)
pub type VkPipelineCreationFeedbackFlags = VkPipelineCreationFeedbackFlagBits;
pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT: VkPipelineCreationFeedbackFlagBits = VkPipelineCreationFeedbackFlagBits(1 << 1);
pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT: VkPipelineCreationFeedbackFlagBits = VkPipelineCreationFeedbackFlagBits(1 << 2);
pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT: VkPipelineCreationFeedbackFlagBits = VkPipelineCreationFeedbackFlagBits(1 << 0);
/// Alias of [`VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT`]
pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT: VkPipelineCreationFeedbackFlagBits = VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT;
/// Alias of [`VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT`]
pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT: VkPipelineCreationFeedbackFlagBits = VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT;
/// Alias of [`VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT`]
pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT: VkPipelineCreationFeedbackFlagBits = VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT;
impl core::fmt::Debug for VkPipelineCreationFeedbackFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "APPLICATION_PIPELINE_CACHE_HIT"),
      (4, "BASE_PIPELINE_ACCELERATION"),
      (1, "VALID"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineDepthStencilStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html) (bitmask)
  VkPipelineDepthStencilStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineDepthStencilStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html) (bitmask)
pub type VkPipelineDepthStencilStateCreateFlags = VkPipelineDepthStencilStateCreateFlagBits;
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT: VkPipelineDepthStencilStateCreateFlagBits = VkPipelineDepthStencilStateCreateFlagBits(1 << 0);
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT: VkPipelineDepthStencilStateCreateFlagBits = VkPipelineDepthStencilStateCreateFlagBits(1 << 1);
/// Alias of [`VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT`]
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: VkPipelineDepthStencilStateCreateFlagBits = VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT;
/// Alias of [`VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT`]
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: VkPipelineDepthStencilStateCreateFlagBits = VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT;
impl core::fmt::Debug for VkPipelineDepthStencilStateCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT"),
      (2, "RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineDiscardRectangleStateCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html) (bitmask, no bits defined)
  VkPipelineDiscardRectangleStateCreateFlagBitsEXT(u32)
);
/// Khronos: [VkPipelineDiscardRectangleStateCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkPipelineDiscardRectangleStateCreateFlagsEXT = VkPipelineDiscardRectangleStateCreateFlagBitsEXT;
impl core::fmt::Debug for VkPipelineDiscardRectangleStateCreateFlagBitsEXT {
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
pub type VkPipelineInputAssemblyStateCreateFlags = VkPipelineInputAssemblyStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineInputAssemblyStateCreateFlagBits {
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
pub const VK_PIPELINE_LAYOUT_CREATE_INDEPENDENT_SETS_BIT_EXT: VkPipelineLayoutCreateFlagBits = VkPipelineLayoutCreateFlagBits(1 << 1);
pub const VK_PIPELINE_LAYOUT_CREATE_RESERVED_0_BIT_AMD: VkPipelineLayoutCreateFlagBits = VkPipelineLayoutCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkPipelineLayoutCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "INDEPENDENT_SETS_BIT_EXT"),
      (1, "RESERVED_0_BIT_AMD"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineMultisampleStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateFlags.html) (bitmask, no bits defined)
  VkPipelineMultisampleStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineMultisampleStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateFlags.html) (bitmask, no bits defined)
pub type VkPipelineMultisampleStateCreateFlags = VkPipelineMultisampleStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineMultisampleStateCreateFlagBits {
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
pub type VkPipelineRasterizationConservativeStateCreateFlagsEXT = VkPipelineRasterizationConservativeStateCreateFlagBitsEXT;
impl core::fmt::Debug for VkPipelineRasterizationConservativeStateCreateFlagBitsEXT {
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
pub type VkPipelineRasterizationDepthClipStateCreateFlagsEXT = VkPipelineRasterizationDepthClipStateCreateFlagBitsEXT;
impl core::fmt::Debug for VkPipelineRasterizationDepthClipStateCreateFlagBitsEXT {
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
pub type VkPipelineRasterizationStateCreateFlags = VkPipelineRasterizationStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineRasterizationStateCreateFlagBits {
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
pub type VkPipelineRasterizationStateStreamCreateFlagsEXT = VkPipelineRasterizationStateStreamCreateFlagBitsEXT;
impl core::fmt::Debug for VkPipelineRasterizationStateStreamCreateFlagBitsEXT {
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
pub const VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT: VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlagBits(1 << 0);
pub const VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT: VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlagBits(1 << 1);
pub const VK_PIPELINE_SHADER_STAGE_CREATE_RESERVED_3_BIT_KHR: VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlagBits(1 << 3);
/// Alias of [`VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT`]
pub const VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT: VkPipelineShaderStageCreateFlagBits = VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT;
/// Alias of [`VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT`]
pub const VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT: VkPipelineShaderStageCreateFlagBits = VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT;
impl core::fmt::Debug for VkPipelineShaderStageCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "ALLOW_VARYING_SUBGROUP_SIZE"),
      (2, "REQUIRE_FULL_SUBGROUPS"),
      (8, "RESERVED_3_BIT_KHR"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html) (bitmask)
  VkPipelineStageFlagBits(u32)
);
/// Khronos: [VkPipelineStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html) (bitmask)
pub type VkPipelineStageFlags = VkPipelineStageFlagBits;
pub const VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 25);
/// All stages supported on the queue
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 16);
/// All stages of the graphics pipeline
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 15);
/// After previous commands have completed
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 13);
/// Color attachment writes
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 10);
pub const VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 17);
/// Compute shading
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 11);
/// A pipeline stage for conditional rendering predicate fetch
pub const VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 18);
/// Draw/DispatchIndirect command fetch
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 1);
/// Early fragment (depth and stencil) tests
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 8);
pub const VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 23);
/// Fragment shading
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 7);
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 22);
/// Geometry shading
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 6);
/// Indicates host (CPU) is a source/sink of the dependency
pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 14);
/// Late fragment (depth and stencil) tests
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 9);
pub const VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 20);
pub const VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 21);
pub const VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 19);
/// Tessellation control shading
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 4);
/// Tessellation evaluation shading
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 5);
/// Before subsequent commands are processed
pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 0);
/// Transfer/copy operations
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 12);
pub const VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 24);
/// Vertex/index fetch
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 2);
/// Vertex shading
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 3);
pub const VK_PIPELINE_STAGE_NONE: VkPipelineStageFlagBits = VkPipelineStageFlagBits(0);
/// Alias of [`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`]
pub const VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT`]
pub const VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT;
/// Alias of [`VK_PIPELINE_STAGE_NONE`]
pub const VK_PIPELINE_STAGE_NONE_KHR: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_NONE;
/// Alias of [`VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR`]
pub const VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
pub const VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT`]
pub const VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV: VkPipelineStageFlagBits = VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT;
impl core::fmt::Debug for VkPipelineStageFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (33554432, "ACCELERATION_STRUCTURE_BUILD_BIT_KHR"),
      (65536, "ALL_COMMANDS"),
      (32768, "ALL_GRAPHICS"),
      (8192, "BOTTOM_OF_PIPE"),
      (1024, "COLOR_ATTACHMENT_OUTPUT"),
      (131072, "COMMAND_PREPROCESS_BIT_NV"),
      (2048, "COMPUTE_SHADER"),
      (262144, "CONDITIONAL_RENDERING_BIT_EXT"),
      (2, "DRAW_INDIRECT"),
      (256, "EARLY_FRAGMENT_TESTS"),
      (8388608, "FRAGMENT_DENSITY_PROCESS_BIT_EXT"),
      (128, "FRAGMENT_SHADER"),
      (4194304, "FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR"),
      (64, "GEOMETRY_SHADER"),
      (16384, "HOST"),
      (512, "LATE_FRAGMENT_TESTS"),
      (1048576, "MESH_SHADER_BIT_EXT"),
      (2097152, "RAY_TRACING_SHADER_BIT_KHR"),
      (524288, "TASK_SHADER_BIT_EXT"),
      (16, "TESSELLATION_CONTROL_SHADER"),
      (32, "TESSELLATION_EVALUATION_SHADER"),
      (1, "TOP_OF_PIPE"),
      (4096, "TRANSFER"),
      (16777216, "TRANSFORM_FEEDBACK_BIT_EXT"),
      (4, "VERTEX_INPUT"),
      (8, "VERTEX_SHADER"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineStageFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html) (bitmask)
  VkPipelineStageFlagBits2(u64)
);
/// Khronos: [VkPipelineStageFlagBits2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html) (bitmask)
pub type VkPipelineStageFlags2 = VkPipelineStageFlagBits2;
pub const VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 25);
pub const VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_COPY_BIT_KHR: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 28);
pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 16);
pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 15);
pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 12);
pub const VK_PIPELINE_STAGE_2_BLIT_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 34);
pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 13);
pub const VK_PIPELINE_STAGE_2_CLEAR_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 35);
pub const VK_PIPELINE_STAGE_2_CLUSTER_CULLING_SHADER_BIT_HUAWEI: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 41);
pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 10);
pub const VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 17);
pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 11);
/// A pipeline stage for conditional rendering predicate fetch
pub const VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 18);
pub const VK_PIPELINE_STAGE_2_COPY_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 32);
pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 1);
pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 8);
pub const VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 23);
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 7);
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 22);
pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 6);
pub const VK_PIPELINE_STAGE_2_HOST_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 14);
pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 36);
pub const VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 40);
pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 9);
pub const VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 20);
pub const VK_PIPELINE_STAGE_2_MICROMAP_BUILD_BIT_EXT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 30);
pub const VK_PIPELINE_STAGE_2_OPTICAL_FLOW_BIT_NV: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 29);
pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 38);
pub const VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 21);
pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 33);
pub const VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 39);
pub const VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 19);
pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 4);
pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 5);
pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 0);
pub const VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 24);
pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 37);
pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 2);
pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 3);
pub const VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(1 << 26);
pub const VK_PIPELINE_STAGE_2_NONE: VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2(0);
/// Alias of [`VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`]
pub const VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`]
pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`]
pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`]
pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_BLIT_BIT`]
pub const VK_PIPELINE_STAGE_2_BLIT_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_BLIT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT`]
pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_CLEAR_BIT`]
pub const VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_CLEAR_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT`]
pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_COPY_BIT`]
pub const VK_PIPELINE_STAGE_2_COPY_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_COPY_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`]
pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`]
pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_HOST_BIT`]
pub const VK_PIPELINE_STAGE_2_HOST_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_HOST_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`]
pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`]
pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT`]
pub const VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT;
/// Alias of [`VK_PIPELINE_STAGE_2_NONE`]
pub const VK_PIPELINE_STAGE_2_NONE_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_NONE;
/// Alias of [`VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT`]
pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`]
pub const VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_2_RESOLVE_BIT`]
pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_RESOLVE_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
pub const VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT`]
pub const VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT;
/// Alias of [`VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT`]
pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR`]
pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR;
/// Alias of [`VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`]
pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`]
pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`]
pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT;
/// Alias of [`VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT`]
pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT;
impl core::fmt::Debug for VkPipelineStageFlagBits2 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (33554432, "ACCELERATION_STRUCTURE_BUILD_BIT_KHR"),
      (268435456, "ACCELERATION_STRUCTURE_COPY_BIT_KHR"),
      (65536, "ALL_COMMANDS"),
      (32768, "ALL_GRAPHICS"),
      (4096, "ALL_TRANSFER"),
      (17179869184, "BLIT"),
      (8192, "BOTTOM_OF_PIPE"),
      (34359738368, "CLEAR"),
      (2199023255552, "CLUSTER_CULLING_SHADER_BIT_HUAWEI"),
      (1024, "COLOR_ATTACHMENT_OUTPUT"),
      (131072, "COMMAND_PREPROCESS_BIT_NV"),
      (2048, "COMPUTE_SHADER"),
      (262144, "CONDITIONAL_RENDERING_BIT_EXT"),
      (4294967296, "COPY"),
      (2, "DRAW_INDIRECT"),
      (256, "EARLY_FRAGMENT_TESTS"),
      (8388608, "FRAGMENT_DENSITY_PROCESS_BIT_EXT"),
      (128, "FRAGMENT_SHADER"),
      (4194304, "FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR"),
      (64, "GEOMETRY_SHADER"),
      (16384, "HOST"),
      (68719476736, "INDEX_INPUT"),
      (1099511627776, "INVOCATION_MASK_BIT_HUAWEI"),
      (512, "LATE_FRAGMENT_TESTS"),
      (1048576, "MESH_SHADER_BIT_EXT"),
      (1073741824, "MICROMAP_BUILD_BIT_EXT"),
      (536870912, "OPTICAL_FLOW_BIT_NV"),
      (274877906944, "PRE_RASTERIZATION_SHADERS"),
      (2097152, "RAY_TRACING_SHADER_BIT_KHR"),
      (8589934592, "RESOLVE"),
      (549755813888, "SUBPASS_SHADING_BIT_HUAWEI"),
      (524288, "TASK_SHADER_BIT_EXT"),
      (16, "TESSELLATION_CONTROL_SHADER"),
      (32, "TESSELLATION_EVALUATION_SHADER"),
      (1, "TOP_OF_PIPE"),
      (16777216, "TRANSFORM_FEEDBACK_BIT_EXT"),
      (137438953472, "VERTEX_ATTRIBUTE_INPUT"),
      (4, "VERTEX_INPUT"),
      (8, "VERTEX_SHADER"),
      (67108864, "VIDEO_DECODE_BIT_KHR"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPipelineTessellationStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateFlags.html) (bitmask, no bits defined)
  VkPipelineTessellationStateCreateFlagBits(u32)
);
/// Khronos: [VkPipelineTessellationStateCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateFlags.html) (bitmask, no bits defined)
pub type VkPipelineTessellationStateCreateFlags = VkPipelineTessellationStateCreateFlagBits;
impl core::fmt::Debug for VkPipelineTessellationStateCreateFlagBits {
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
pub type VkPipelineViewportSwizzleStateCreateFlagsNV = VkPipelineViewportSwizzleStateCreateFlagBitsNV;
impl core::fmt::Debug for VkPipelineViewportSwizzleStateCreateFlagBitsNV {
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
pub const VK_PRESENT_GRAVITY_CENTERED_BIT_EXT: VkPresentGravityFlagBitsEXT = VkPresentGravityFlagBitsEXT(1 << 2);
pub const VK_PRESENT_GRAVITY_MAX_BIT_EXT: VkPresentGravityFlagBitsEXT = VkPresentGravityFlagBitsEXT(1 << 1);
pub const VK_PRESENT_GRAVITY_MIN_BIT_EXT: VkPresentGravityFlagBitsEXT = VkPresentGravityFlagBitsEXT(1 << 0);
impl core::fmt::Debug for VkPresentGravityFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "CENTERED"),
      (2, "MAX"),
      (1, "MIN"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPresentScalingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentScalingFlagBitsEXT.html) (bitmask)
  VkPresentScalingFlagBitsEXT(u32)
);
/// Khronos: [VkPresentScalingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentScalingFlagBitsEXT.html) (bitmask)
pub type VkPresentScalingFlagsEXT = VkPresentScalingFlagBitsEXT;
pub const VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT: VkPresentScalingFlagBitsEXT = VkPresentScalingFlagBitsEXT(1 << 1);
pub const VK_PRESENT_SCALING_ONE_TO_ONE_BIT_EXT: VkPresentScalingFlagBitsEXT = VkPresentScalingFlagBitsEXT(1 << 0);
pub const VK_PRESENT_SCALING_STRETCH_BIT_EXT: VkPresentScalingFlagBitsEXT = VkPresentScalingFlagBitsEXT(1 << 2);
impl core::fmt::Debug for VkPresentScalingFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "ASPECT_RATIO_STRETCH"),
      (1, "ONE_TO_ONE"),
      (4, "STRETCH"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkPrivateDataSlotCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateFlags.html) (bitmask, no bits defined)
  VkPrivateDataSlotCreateFlagBits(u32)
);
/// Khronos: [VkPrivateDataSlotCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateFlags.html) (bitmask, no bits defined)
pub type VkPrivateDataSlotCreateFlags = VkPrivateDataSlotCreateFlagBits;
pub const VK_PRIVATE_DATA_SLOT_CREATE_RESERVED_0_BIT_NV: VkPrivateDataSlotCreateFlagBits = VkPrivateDataSlotCreateFlagBits(1 << 0);
impl core::fmt::Debug for VkPrivateDataSlotCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "RESERVED_0_BIT_NV"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkQueryControlFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html) (bitmask)
  VkQueryControlFlagBits(u32)
);
/// Khronos: [VkQueryControlFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html) (bitmask)
pub type VkQueryControlFlags = VkQueryControlFlagBits;
/// Require precise results to be collected by the query
pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlagBits = VkQueryControlFlagBits(1 << 0);
impl core::fmt::Debug for VkQueryControlFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "PRECISE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkQueryPipelineStatisticFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html) (bitmask)
  VkQueryPipelineStatisticFlagBits(u32)
);
/// Khronos: [VkQueryPipelineStatisticFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html) (bitmask)
pub type VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlagBits;
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 5);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 6);
pub const VK_QUERY_PIPELINE_STATISTIC_CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 13);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 10);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 7);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 3);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 4);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 1);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 0);
pub const VK_QUERY_PIPELINE_STATISTIC_MESH_SHADER_INVOCATIONS_BIT_EXT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 12);
pub const VK_QUERY_PIPELINE_STATISTIC_TASK_SHADER_INVOCATIONS_BIT_EXT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 11);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 8);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 9);
/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1 << 2);
impl core::fmt::Debug for VkQueryPipelineStatisticFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (32, "CLIPPING_INVOCATIONS"),
      (64, "CLIPPING_PRIMITIVES"),
      (8192, "CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI"),
      (1024, "COMPUTE_SHADER_INVOCATIONS"),
      (128, "FRAGMENT_SHADER_INVOCATIONS"),
      (8, "GEOMETRY_SHADER_INVOCATIONS"),
      (16, "GEOMETRY_SHADER_PRIMITIVES"),
      (2, "INPUT_ASSEMBLY_PRIMITIVES"),
      (1, "INPUT_ASSEMBLY_VERTICES"),
      (4096, "MESH_SHADER_INVOCATIONS_BIT_EXT"),
      (2048, "TASK_SHADER_INVOCATIONS_BIT_EXT"),
      (256, "TESSELLATION_CONTROL_SHADER_PATCHES"),
      (512, "TESSELLATION_EVALUATION_SHADER_INVOCATIONS"),
      (4, "VERTEX_SHADER_INVOCATIONS"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkQueryPoolCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlags.html) (bitmask, no bits defined)
  VkQueryPoolCreateFlagBits(u32)
);
/// Khronos: [VkQueryPoolCreateFlags](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlags.html) (bitmask, no bits defined)
pub type VkQueryPoolCreateFlags = VkQueryPoolCreateFlagBits;
impl core::fmt::Debug for VkQueryPoolCreateFlagBits {
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
/// Results of the queries are written to the destination buffer as 64-bit values
pub const VK_QUERY_RESULT_64_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1 << 0);
/// Copy the partial results of the query even if the final results are not available
pub const VK_QUERY_RESULT_PARTIAL_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1 << 3);
/// Results of the queries are waited on before proceeding with the result copy
pub const VK_QUERY_RESULT_WAIT_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1 << 1);
/// Besides the results of the query, the availability of the results is also written
pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1 << 2);
pub const VK_QUERY_RESULT_WITH_STATUS_BIT_KHR: VkQueryResultFlagBits = VkQueryResultFlagBits(1 << 4);
impl core::fmt::Debug for VkQueryResultFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "64"),
      (8, "PARTIAL"),
      (2, "WAIT"),
      (4, "WITH_AVAILABILITY"),
      (16, "WITH_STATUS_BIT_KHR"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
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
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "COMPUTE"),
      (1, "GRAPHICS"),
      (256, "OPTICAL_FLOW_BIT_NV"),
      (16, "PROTECTED"),
      (128, "RESERVED_7_BIT_QCOM"),
      (512, "RESERVED_9_BIT_EXT"),
      (8, "SPARSE_BINDING"),
      (4, "TRANSFER"),
      (32, "VIDEO_DECODE_BIT_KHR"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkRefreshObjectFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshObjectFlagBitsKHR.html) (bitmask)
  VkRefreshObjectFlagBitsKHR(u32)
);
/// Khronos: [VkRefreshObjectFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshObjectFlagBitsKHR.html) (bitmask)
pub type VkRefreshObjectFlagsKHR = VkRefreshObjectFlagBitsKHR;
impl core::fmt::Debug for VkRefreshObjectFlagBitsKHR {
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
pub const VK_RENDER_PASS_CREATE_RESERVED_0_BIT_KHR: VkRenderPassCreateFlagBits = VkRenderPassCreateFlagBits(1 << 0);
pub const VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM: VkRenderPassCreateFlagBits = VkRenderPassCreateFlagBits(1 << 1);
impl core::fmt::Debug for VkRenderPassCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "RESERVED_0_BIT_KHR"),
      (2, "TRANSFORM_BIT_QCOM"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkRenderingFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html) (bitmask)
  VkRenderingFlagBits(u32)
);
/// Khronos: [VkRenderingFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html) (bitmask)
pub type VkRenderingFlags = VkRenderingFlagBits;
pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: VkRenderingFlagBits = VkRenderingFlagBits(1 << 0);
pub const VK_RENDERING_ENABLE_LEGACY_DITHERING_BIT_EXT: VkRenderingFlagBits = VkRenderingFlagBits(1 << 3);
pub const VK_RENDERING_RESUMING_BIT: VkRenderingFlagBits = VkRenderingFlagBits(1 << 2);
pub const VK_RENDERING_SUSPENDING_BIT: VkRenderingFlagBits = VkRenderingFlagBits(1 << 1);
/// Alias of [`VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT`]
pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR: VkRenderingFlagBits = VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT;
/// Alias of [`VK_RENDERING_RESUMING_BIT`]
pub const VK_RENDERING_RESUMING_BIT_KHR: VkRenderingFlagBits = VK_RENDERING_RESUMING_BIT;
/// Alias of [`VK_RENDERING_SUSPENDING_BIT`]
pub const VK_RENDERING_SUSPENDING_BIT_KHR: VkRenderingFlagBits = VK_RENDERING_SUSPENDING_BIT;
impl core::fmt::Debug for VkRenderingFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "CONTENTS_SECONDARY_COMMAND_BUFFERS"),
      (8, "ENABLE_LEGACY_DITHERING_BIT_EXT"),
      (4, "RESUMING"),
      (2, "SUSPENDING"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkResolveModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html) (bitmask)
  VkResolveModeFlagBits(u32)
);
/// Khronos: [VkResolveModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html) (bitmask)
pub type VkResolveModeFlags = VkResolveModeFlagBits;
pub const VK_RESOLVE_MODE_AVERAGE_BIT: VkResolveModeFlagBits = VkResolveModeFlagBits(1 << 1);
pub const VK_RESOLVE_MODE_MAX_BIT: VkResolveModeFlagBits = VkResolveModeFlagBits(1 << 3);
pub const VK_RESOLVE_MODE_MIN_BIT: VkResolveModeFlagBits = VkResolveModeFlagBits(1 << 2);
pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT: VkResolveModeFlagBits = VkResolveModeFlagBits(1 << 0);
pub const VK_RESOLVE_MODE_NONE: VkResolveModeFlagBits = VkResolveModeFlagBits(0);
/// Alias of [`VK_RESOLVE_MODE_AVERAGE_BIT`]
pub const VK_RESOLVE_MODE_AVERAGE_BIT_KHR: VkResolveModeFlagBits = VK_RESOLVE_MODE_AVERAGE_BIT;
/// Alias of [`VK_RESOLVE_MODE_MAX_BIT`]
pub const VK_RESOLVE_MODE_MAX_BIT_KHR: VkResolveModeFlagBits = VK_RESOLVE_MODE_MAX_BIT;
/// Alias of [`VK_RESOLVE_MODE_MIN_BIT`]
pub const VK_RESOLVE_MODE_MIN_BIT_KHR: VkResolveModeFlagBits = VK_RESOLVE_MODE_MIN_BIT;
/// Alias of [`VK_RESOLVE_MODE_NONE`]
pub const VK_RESOLVE_MODE_NONE_KHR: VkResolveModeFlagBits = VK_RESOLVE_MODE_NONE;
/// Alias of [`VK_RESOLVE_MODE_SAMPLE_ZERO_BIT`]
pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR: VkResolveModeFlagBits = VK_RESOLVE_MODE_SAMPLE_ZERO_BIT;
impl core::fmt::Debug for VkResolveModeFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "AVERAGE"),
      (8, "MAX"),
      (4, "MIN"),
      (1, "SAMPLE_ZERO"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
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
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (16, "16"),
      (1, "1"),
      (2, "2"),
      (32, "32"),
      (4, "4"),
      (64, "64"),
      (8, "8"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSamplerCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html) (bitmask)
  VkSamplerCreateFlagBits(u32)
);
/// Khronos: [VkSamplerCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html) (bitmask)
pub type VkSamplerCreateFlags = VkSamplerCreateFlagBits;
pub const VK_SAMPLER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: VkSamplerCreateFlagBits = VkSamplerCreateFlagBits(1 << 3);
pub const VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM: VkSamplerCreateFlagBits = VkSamplerCreateFlagBits(1 << 4);
pub const VK_SAMPLER_CREATE_NON_SEAMLESS_CUBE_MAP_BIT_EXT: VkSamplerCreateFlagBits = VkSamplerCreateFlagBits(1 << 2);
pub const VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT: VkSamplerCreateFlagBits = VkSamplerCreateFlagBits(1 << 0);
pub const VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT: VkSamplerCreateFlagBits = VkSamplerCreateFlagBits(1 << 1);
impl core::fmt::Debug for VkSamplerCreateFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT"),
      (16, "IMAGE_PROCESSING_BIT_QCOM"),
      (4, "NON_SEAMLESS_CUBE_MAP_BIT_EXT"),
      (1, "SUBSAMPLED_BIT_EXT"),
      (2, "SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkScreenSurfaceCreateFlagsQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateFlagsQNX.html) (bitmask, no bits defined)
  VkScreenSurfaceCreateFlagBitsQNX(u32)
);
/// Khronos: [VkScreenSurfaceCreateFlagsQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateFlagsQNX.html) (bitmask, no bits defined)
pub type VkScreenSurfaceCreateFlagsQNX = VkScreenSurfaceCreateFlagBitsQNX;
impl core::fmt::Debug for VkScreenSurfaceCreateFlagBitsQNX {
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
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT: VkSemaphoreImportFlagBits = VkSemaphoreImportFlagBits(1 << 0);
/// Alias of [`VK_SEMAPHORE_IMPORT_TEMPORARY_BIT`]
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR: VkSemaphoreImportFlagBits = VK_SEMAPHORE_IMPORT_TEMPORARY_BIT;
impl core::fmt::Debug for VkSemaphoreImportFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "TEMPORARY"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSemaphoreWaitFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html) (bitmask)
  VkSemaphoreWaitFlagBits(u32)
);
/// Khronos: [VkSemaphoreWaitFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html) (bitmask)
pub type VkSemaphoreWaitFlags = VkSemaphoreWaitFlagBits;
pub const VK_SEMAPHORE_WAIT_ANY_BIT: VkSemaphoreWaitFlagBits = VkSemaphoreWaitFlagBits(1 << 0);
/// Alias of [`VK_SEMAPHORE_WAIT_ANY_BIT`]
pub const VK_SEMAPHORE_WAIT_ANY_BIT_KHR: VkSemaphoreWaitFlagBits = VK_SEMAPHORE_WAIT_ANY_BIT;
impl core::fmt::Debug for VkSemaphoreWaitFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "ANY"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkShaderCorePropertiesFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html) (bitmask)
  VkShaderCorePropertiesFlagBitsAMD(u32)
);
/// Khronos: [VkShaderCorePropertiesFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html) (bitmask)
pub type VkShaderCorePropertiesFlagsAMD = VkShaderCorePropertiesFlagBitsAMD;
impl core::fmt::Debug for VkShaderCorePropertiesFlagBitsAMD {
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
pub const VK_SHADER_STAGE_ANY_HIT_BIT_KHR: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 9);
pub const VK_SHADER_STAGE_CALLABLE_BIT_KHR: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 13);
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 10);
pub const VK_SHADER_STAGE_CLUSTER_CULLING_BIT_HUAWEI: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 19);
pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 5);
pub const VK_SHADER_STAGE_EXT_483_RESERVE_15: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 15);
pub const VK_SHADER_STAGE_EXT_483_RESERVE_16: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 16);
pub const VK_SHADER_STAGE_EXT_483_RESERVE_17: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 17);
pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 4);
pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 3);
pub const VK_SHADER_STAGE_INTERSECTION_BIT_KHR: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 12);
pub const VK_SHADER_STAGE_MESH_BIT_EXT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 7);
pub const VK_SHADER_STAGE_MISS_BIT_KHR: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 11);
pub const VK_SHADER_STAGE_RAYGEN_BIT_KHR: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 8);
pub const VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 14);
pub const VK_SHADER_STAGE_TASK_BIT_EXT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 6);
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 1);
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 2);
pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 0);
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlagBits = VkShaderStageFlagBits(2147483647);
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlagBits = VkShaderStageFlagBits(31);
/// Alias of [`VK_SHADER_STAGE_ANY_HIT_BIT_KHR`]
pub const VK_SHADER_STAGE_ANY_HIT_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_ANY_HIT_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_CALLABLE_BIT_KHR`]
pub const VK_SHADER_STAGE_CALLABLE_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_CALLABLE_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR`]
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_INTERSECTION_BIT_KHR`]
pub const VK_SHADER_STAGE_INTERSECTION_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_INTERSECTION_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_MESH_BIT_EXT`]
pub const VK_SHADER_STAGE_MESH_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_MESH_BIT_EXT;
/// Alias of [`VK_SHADER_STAGE_MISS_BIT_KHR`]
pub const VK_SHADER_STAGE_MISS_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_MISS_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_RAYGEN_BIT_KHR`]
pub const VK_SHADER_STAGE_RAYGEN_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_RAYGEN_BIT_KHR;
/// Alias of [`VK_SHADER_STAGE_TASK_BIT_EXT`]
pub const VK_SHADER_STAGE_TASK_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_TASK_BIT_EXT;
impl core::fmt::Debug for VkShaderStageFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (512, "ANY_HIT_BIT_KHR"),
      (8192, "CALLABLE_BIT_KHR"),
      (1024, "CLOSEST_HIT_BIT_KHR"),
      (524288, "CLUSTER_CULLING_BIT_HUAWEI"),
      (32, "COMPUTE"),
      (32768, "EXT_483_RESERVE_15"),
      (65536, "EXT_483_RESERVE_16"),
      (131072, "EXT_483_RESERVE_17"),
      (16, "FRAGMENT"),
      (8, "GEOMETRY"),
      (4096, "INTERSECTION_BIT_KHR"),
      (128, "MESH_BIT_EXT"),
      (2048, "MISS_BIT_KHR"),
      (256, "RAYGEN_BIT_KHR"),
      (16384, "SUBPASS_SHADING_BIT_HUAWEI"),
      (64, "TASK_BIT_EXT"),
      (2, "TESSELLATION_CONTROL"),
      (4, "TESSELLATION_EVALUATION"),
      (1, "VERTEX"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSparseImageFormatFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html) (bitmask)
  VkSparseImageFormatFlagBits(u32)
);
/// Khronos: [VkSparseImageFormatFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html) (bitmask)
pub type VkSparseImageFormatFlags = VkSparseImageFormatFlagBits;
/// Image requires mip level dimensions to be an integer multiple of the sparse image block dimensions for non-tail mip levels.
pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlagBits = VkSparseImageFormatFlagBits(1 << 1);
/// Image uses a non-standard sparse image block dimensions
pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlagBits = VkSparseImageFormatFlagBits(1 << 2);
/// Image uses a single mip tail region for all array layers
pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlagBits = VkSparseImageFormatFlagBits(1 << 0);
impl core::fmt::Debug for VkSparseImageFormatFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "ALIGNED_MIP_SIZE"),
      (4, "NONSTANDARD_BLOCK_SIZE"),
      (1, "SINGLE_MIPTAIL"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSparseMemoryBindFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html) (bitmask)
  VkSparseMemoryBindFlagBits(u32)
);
/// Khronos: [VkSparseMemoryBindFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html) (bitmask)
pub type VkSparseMemoryBindFlags = VkSparseMemoryBindFlagBits;
/// Operation binds resource metadata to memory
pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT: VkSparseMemoryBindFlagBits = VkSparseMemoryBindFlagBits(1 << 0);
impl core::fmt::Debug for VkSparseMemoryBindFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "METADATA"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
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
pub const VK_STENCIL_FACE_FRONT_BIT: VkStencilFaceFlagBits = VkStencilFaceFlagBits(1 << 0);
/// Front and back faces
pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlagBits = VkStencilFaceFlagBits(3);
/// Alias of [`VK_STENCIL_FACE_FRONT_AND_BACK`]
#[deprecated = "aliased"]
pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlagBits = VK_STENCIL_FACE_FRONT_AND_BACK;
impl core::fmt::Debug for VkStencilFaceFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "BACK"),
      (1, "FRONT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkStreamDescriptorSurfaceCreateFlagsGGP](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html) (bitmask, no bits defined)
  VkStreamDescriptorSurfaceCreateFlagBitsGGP(u32)
);
/// Khronos: [VkStreamDescriptorSurfaceCreateFlagsGGP](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html) (bitmask, no bits defined)
pub type VkStreamDescriptorSurfaceCreateFlagsGGP = VkStreamDescriptorSurfaceCreateFlagBitsGGP;
impl core::fmt::Debug for VkStreamDescriptorSurfaceCreateFlagBitsGGP {
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
pub const VK_SUBGROUP_FEATURE_ARITHMETIC_BIT: VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlagBits(1 << 2);
/// Ballot subgroup operations
pub const VK_SUBGROUP_FEATURE_BALLOT_BIT: VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlagBits(1 << 3);
/// Basic subgroup operations
pub const VK_SUBGROUP_FEATURE_BASIC_BIT: VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlagBits(1 << 0);
/// Clustered subgroup operations
pub const VK_SUBGROUP_FEATURE_CLUSTERED_BIT: VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlagBits(1 << 6);
pub const VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV: VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlagBits(1 << 8);
/// Quad subgroup operations
pub const VK_SUBGROUP_FEATURE_QUAD_BIT: VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlagBits(1 << 7);
/// Shuffle subgroup operations
pub const VK_SUBGROUP_FEATURE_SHUFFLE_BIT: VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlagBits(1 << 4);
/// Shuffle relative subgroup operations
pub const VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT: VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlagBits(1 << 5);
/// Vote subgroup operations
pub const VK_SUBGROUP_FEATURE_VOTE_BIT: VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlagBits(1 << 1);
impl core::fmt::Debug for VkSubgroupFeatureFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "ARITHMETIC"),
      (8, "BALLOT"),
      (1, "BASIC"),
      (64, "CLUSTERED"),
      (256, "PARTITIONED_BIT_NV"),
      (128, "QUAD"),
      (16, "SHUFFLE"),
      (32, "SHUFFLE_RELATIVE"),
      (2, "VOTE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
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
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "PROTECTED"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSubpassDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlagBits.html) (bitmask)
  VkSubpassDescriptionFlagBits(u32)
);
/// Khronos: [VkSubpassDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlagBits.html) (bitmask)
pub type VkSubpassDescriptionFlags = VkSubpassDescriptionFlagBits;
pub const VK_SUBPASS_DESCRIPTION_ENABLE_LEGACY_DITHERING_BIT_EXT: VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 7);
pub const VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM: VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 2);
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX: VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 0);
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX: VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 1);
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT: VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 4);
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT: VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 5);
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT: VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 6);
pub const VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM: VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlagBits(1 << 3);
/// Alias of [`VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT`]
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM: VkSubpassDescriptionFlagBits = VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT;
/// Alias of [`VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT`]
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: VkSubpassDescriptionFlagBits = VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT;
/// Alias of [`VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT`]
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: VkSubpassDescriptionFlagBits = VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT;
impl core::fmt::Debug for VkSubpassDescriptionFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (128, "ENABLE_LEGACY_DITHERING_BIT_EXT"),
      (4, "FRAGMENT_REGION_BIT_QCOM"),
      (1, "PER_VIEW_ATTRIBUTES_BIT_NVX"),
      (2, "PER_VIEW_POSITION_X_ONLY_BIT_NVX"),
      (16, "RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT"),
      (32, "RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT"),
      (64, "RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT"),
      (8, "SHADER_RESOLVE_BIT_QCOM"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSurfaceCounterFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html) (bitmask)
  VkSurfaceCounterFlagBitsEXT(u32)
);
/// Khronos: [VkSurfaceCounterFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html) (bitmask)
pub type VkSurfaceCounterFlagsEXT = VkSurfaceCounterFlagBitsEXT;
pub const VK_SURFACE_COUNTER_VBLANK_BIT_EXT: VkSurfaceCounterFlagBitsEXT = VkSurfaceCounterFlagBitsEXT(1 << 0);
/// Alias of [`VK_SURFACE_COUNTER_VBLANK_BIT_EXT`]
#[deprecated = "aliased"]
pub const VK_SURFACE_COUNTER_VBLANK_EXT: VkSurfaceCounterFlagBitsEXT = VK_SURFACE_COUNTER_VBLANK_BIT_EXT;
impl core::fmt::Debug for VkSurfaceCounterFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "VBLANK"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSurfaceTransformFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) (bitmask)
  VkSurfaceTransformFlagBitsKHR(u32)
);
/// Khronos: [VkSurfaceTransformFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) (bitmask)
pub type VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagBitsKHR;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 4);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 6);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 7);
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 5);
pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 0);
pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 8);
pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 2);
pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 3);
pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1 << 1);
impl core::fmt::Debug for VkSurfaceTransformFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (16, "HORIZONTAL_MIRROR"),
      (64, "HORIZONTAL_MIRROR_ROTATE_180"),
      (128, "HORIZONTAL_MIRROR_ROTATE_270"),
      (32, "HORIZONTAL_MIRROR_ROTATE_90"),
      (1, "IDENTITY"),
      (256, "INHERIT"),
      (4, "ROTATE_180"),
      (8, "ROTATE_270"),
      (2, "ROTATE_90"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSwapchainCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) (bitmask)
  VkSwapchainCreateFlagBitsKHR(u32)
);
/// Khronos: [VkSwapchainCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) (bitmask)
pub type VkSwapchainCreateFlagsKHR = VkSwapchainCreateFlagBitsKHR;
pub const VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT: VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagBitsKHR(1 << 3);
pub const VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR: VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagBitsKHR(1 << 2);
/// Swapchain is protected
pub const VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR: VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagBitsKHR(1 << 1);
pub const VK_SWAPCHAIN_CREATE_RESERVED_4_BIT_EXT: VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagBitsKHR(1 << 4);
/// Allow images with VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT
pub const VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagBitsKHR(1 << 0);
impl core::fmt::Debug for VkSwapchainCreateFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "DEFERRED_MEMORY_ALLOCATION_BIT_EXT"),
      (4, "MUTABLE_FORMAT"),
      (2, "PROTECTED"),
      (16, "RESERVED_4_BIT_EXT"),
      (1, "SPLIT_INSTANCE_BIND_REGIONS"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkSwapchainImageUsageFlagBitsANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainImageUsageFlagBitsANDROID.html) (bitmask)
  VkSwapchainImageUsageFlagBitsANDROID(u32)
);
/// Khronos: [VkSwapchainImageUsageFlagBitsANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainImageUsageFlagBitsANDROID.html) (bitmask)
pub type VkSwapchainImageUsageFlagsANDROID = VkSwapchainImageUsageFlagBitsANDROID;
pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID: VkSwapchainImageUsageFlagBitsANDROID = VkSwapchainImageUsageFlagBitsANDROID(1 << 0);
impl core::fmt::Debug for VkSwapchainImageUsageFlagBitsANDROID {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "SHARED"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkToolPurposeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagBits.html) (bitmask)
  VkToolPurposeFlagBits(u32)
);
/// Khronos: [VkToolPurposeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagBits.html) (bitmask)
pub type VkToolPurposeFlags = VkToolPurposeFlagBits;
pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT: VkToolPurposeFlagBits = VkToolPurposeFlagBits(1 << 3);
pub const VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT: VkToolPurposeFlagBits = VkToolPurposeFlagBits(1 << 6);
pub const VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT: VkToolPurposeFlagBits = VkToolPurposeFlagBits(1 << 5);
pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT: VkToolPurposeFlagBits = VkToolPurposeFlagBits(1 << 4);
pub const VK_TOOL_PURPOSE_PROFILING_BIT: VkToolPurposeFlagBits = VkToolPurposeFlagBits(1 << 1);
pub const VK_TOOL_PURPOSE_TRACING_BIT: VkToolPurposeFlagBits = VkToolPurposeFlagBits(1 << 2);
pub const VK_TOOL_PURPOSE_VALIDATION_BIT: VkToolPurposeFlagBits = VkToolPurposeFlagBits(1 << 0);
/// Alias of [`VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT`]
pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT: VkToolPurposeFlagBits = VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT;
/// Alias of [`VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT`]
pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT: VkToolPurposeFlagBits = VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT;
/// Alias of [`VK_TOOL_PURPOSE_PROFILING_BIT`]
pub const VK_TOOL_PURPOSE_PROFILING_BIT_EXT: VkToolPurposeFlagBits = VK_TOOL_PURPOSE_PROFILING_BIT;
/// Alias of [`VK_TOOL_PURPOSE_TRACING_BIT`]
pub const VK_TOOL_PURPOSE_TRACING_BIT_EXT: VkToolPurposeFlagBits = VK_TOOL_PURPOSE_TRACING_BIT;
/// Alias of [`VK_TOOL_PURPOSE_VALIDATION_BIT`]
pub const VK_TOOL_PURPOSE_VALIDATION_BIT_EXT: VkToolPurposeFlagBits = VK_TOOL_PURPOSE_VALIDATION_BIT;
impl core::fmt::Debug for VkToolPurposeFlagBits {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "ADDITIONAL_FEATURES"),
      (64, "DEBUG_MARKERS_BIT_EXT"),
      (32, "DEBUG_REPORTING_BIT_EXT"),
      (16, "MODIFYING_FEATURES"),
      (2, "PROFILING"),
      (4, "TRACING"),
      (1, "VALIDATION"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkValidationCacheCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateFlagsEXT.html) (bitmask, no bits defined)
  VkValidationCacheCreateFlagBitsEXT(u32)
);
/// Khronos: [VkValidationCacheCreateFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateFlagsEXT.html) (bitmask, no bits defined)
pub type VkValidationCacheCreateFlagsEXT = VkValidationCacheCreateFlagBitsEXT;
impl core::fmt::Debug for VkValidationCacheCreateFlagBitsEXT {
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
pub const VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR: VkVideoCapabilityFlagBitsKHR = VkVideoCapabilityFlagBitsKHR(1 << 0);
pub const VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR: VkVideoCapabilityFlagBitsKHR = VkVideoCapabilityFlagBitsKHR(1 << 1);
impl core::fmt::Debug for VkVideoCapabilityFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "PROTECTED_CONTENT"),
      (2, "SEPARATE_REFERENCE_IMAGES"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
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
pub const VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR: VkVideoChromaSubsamplingFlagBitsKHR = VkVideoChromaSubsamplingFlagBitsKHR(1 << 1);
pub const VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR: VkVideoChromaSubsamplingFlagBitsKHR = VkVideoChromaSubsamplingFlagBitsKHR(1 << 2);
pub const VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR: VkVideoChromaSubsamplingFlagBitsKHR = VkVideoChromaSubsamplingFlagBitsKHR(1 << 3);
pub const VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR: VkVideoChromaSubsamplingFlagBitsKHR = VkVideoChromaSubsamplingFlagBitsKHR(1 << 0);
pub const VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_KHR: VkVideoChromaSubsamplingFlagBitsKHR = VkVideoChromaSubsamplingFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoChromaSubsamplingFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "420"),
      (4, "422"),
      (8, "444"),
      (1, "MONOCHROME"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoCodecOperationFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html) (bitmask)
  VkVideoCodecOperationFlagBitsKHR(u32)
);
/// Khronos: [VkVideoCodecOperationFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodecOperationFlagBitsKHR.html) (bitmask)
pub type VkVideoCodecOperationFlagsKHR = VkVideoCodecOperationFlagBitsKHR;
pub const VK_VIDEO_CODEC_OPERATION_DECODE_H264_BIT_KHR: VkVideoCodecOperationFlagBitsKHR = VkVideoCodecOperationFlagBitsKHR(1 << 0);
pub const VK_VIDEO_CODEC_OPERATION_DECODE_H265_BIT_KHR: VkVideoCodecOperationFlagBitsKHR = VkVideoCodecOperationFlagBitsKHR(1 << 1);
pub const VK_VIDEO_CODEC_OPERATION_NONE_KHR: VkVideoCodecOperationFlagBitsKHR = VkVideoCodecOperationFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoCodecOperationFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "DECODE_H264"),
      (2, "DECODE_H265"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoCodingControlFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html) (bitmask)
  VkVideoCodingControlFlagBitsKHR(u32)
);
/// Khronos: [VkVideoCodingControlFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlFlagBitsKHR.html) (bitmask)
pub type VkVideoCodingControlFlagsKHR = VkVideoCodingControlFlagBitsKHR;
pub const VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR: VkVideoCodingControlFlagBitsKHR = VkVideoCodingControlFlagBitsKHR(1 << 0);
impl core::fmt::Debug for VkVideoCodingControlFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "RESET"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
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
pub const VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR: VkVideoComponentBitDepthFlagBitsKHR = VkVideoComponentBitDepthFlagBitsKHR(1 << 2);
pub const VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR: VkVideoComponentBitDepthFlagBitsKHR = VkVideoComponentBitDepthFlagBitsKHR(1 << 4);
pub const VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR: VkVideoComponentBitDepthFlagBitsKHR = VkVideoComponentBitDepthFlagBitsKHR(1 << 0);
pub const VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR: VkVideoComponentBitDepthFlagBitsKHR = VkVideoComponentBitDepthFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoComponentBitDepthFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "10"),
      (16, "12"),
      (1, "8"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoDecodeFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagsKHR.html) (bitmask, no bits defined)
  VkVideoDecodeFlagBitsKHR(u32)
);
/// Khronos: [VkVideoDecodeFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoDecodeFlagsKHR = VkVideoDecodeFlagBitsKHR;
impl core::fmt::Debug for VkVideoDecodeFlagBitsKHR {
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
pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR: VkVideoDecodeCapabilityFlagBitsKHR = VkVideoDecodeCapabilityFlagBitsKHR(1 << 0);
pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR: VkVideoDecodeCapabilityFlagBitsKHR = VkVideoDecodeCapabilityFlagBitsKHR(1 << 1);
impl core::fmt::Debug for VkVideoDecodeCapabilityFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "DPB_AND_OUTPUT_COINCIDE"),
      (2, "DPB_AND_OUTPUT_DISTINCT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoDecodeH264PictureLayoutFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html) (bitmask)
  VkVideoDecodeH264PictureLayoutFlagBitsKHR(u32)
);
/// Khronos: [VkVideoDecodeH264PictureLayoutFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html) (bitmask)
pub type VkVideoDecodeH264PictureLayoutFlagsKHR = VkVideoDecodeH264PictureLayoutFlagBitsKHR;
pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR: VkVideoDecodeH264PictureLayoutFlagBitsKHR = VkVideoDecodeH264PictureLayoutFlagBitsKHR(1 << 0);
pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR: VkVideoDecodeH264PictureLayoutFlagBitsKHR = VkVideoDecodeH264PictureLayoutFlagBitsKHR(1 << 1);
pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR: VkVideoDecodeH264PictureLayoutFlagBitsKHR = VkVideoDecodeH264PictureLayoutFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoDecodeH264PictureLayoutFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "INTERLACED_INTERLEAVED_LINES"),
      (2, "INTERLACED_SEPARATE_PLANES"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoDecodeUsageFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeUsageFlagBitsKHR.html) (bitmask)
  VkVideoDecodeUsageFlagBitsKHR(u32)
);
/// Khronos: [VkVideoDecodeUsageFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeUsageFlagBitsKHR.html) (bitmask)
pub type VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagBitsKHR;
pub const VK_VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR: VkVideoDecodeUsageFlagBitsKHR = VkVideoDecodeUsageFlagBitsKHR(1 << 1);
pub const VK_VIDEO_DECODE_USAGE_STREAMING_BIT_KHR: VkVideoDecodeUsageFlagBitsKHR = VkVideoDecodeUsageFlagBitsKHR(1 << 2);
pub const VK_VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR: VkVideoDecodeUsageFlagBitsKHR = VkVideoDecodeUsageFlagBitsKHR(1 << 0);
pub const VK_VIDEO_DECODE_USAGE_DEFAULT_KHR: VkVideoDecodeUsageFlagBitsKHR = VkVideoDecodeUsageFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoDecodeUsageFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (2, "OFFLINE"),
      (4, "STREAMING"),
      (1, "TRANSCODING"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFlagsKHR.html) (bitmask, no bits defined)
  VkVideoEncodeFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoEncodeFlagsKHR = VkVideoEncodeFlagBitsKHR;
impl core::fmt::Debug for VkVideoEncodeFlagBitsKHR {
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
pub const VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR: VkVideoEncodeCapabilityFlagBitsKHR = VkVideoEncodeCapabilityFlagBitsKHR(1 << 0);
impl core::fmt::Debug for VkVideoEncodeCapabilityFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "PRECEDING_EXTERNALLY_ENCODED_BYTES"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeContentFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeContentFlagBitsKHR.html) (bitmask)
  VkVideoEncodeContentFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeContentFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeContentFlagBitsKHR.html) (bitmask)
pub type VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagBitsKHR;
pub const VK_VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR: VkVideoEncodeContentFlagBitsKHR = VkVideoEncodeContentFlagBitsKHR(1 << 0);
pub const VK_VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR: VkVideoEncodeContentFlagBitsKHR = VkVideoEncodeContentFlagBitsKHR(1 << 1);
pub const VK_VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR: VkVideoEncodeContentFlagBitsKHR = VkVideoEncodeContentFlagBitsKHR(1 << 2);
pub const VK_VIDEO_ENCODE_CONTENT_DEFAULT_KHR: VkVideoEncodeContentFlagBitsKHR = VkVideoEncodeContentFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoEncodeContentFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "CAMERA"),
      (2, "DESKTOP"),
      (4, "RENDERED"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH264CapabilityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilityFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH264CapabilityFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH264CapabilityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilityFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH264CapabilityFlagsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 24);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_CABAC_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 14);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_CAVLC_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 15);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_CHROMA_QP_OFFSET_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 6);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_DISABLED_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 16);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_ENABLED_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 17);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DEBLOCKING_FILTER_PARTIAL_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 18);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 23);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIRECT_8X8_INFERENCE_DISABLED_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 1);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIRECT_8X8_INFERENCE_ENABLED_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DISABLE_DIRECT_SPATIAL_MV_PRED_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 19);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 5);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_MULTIPLE_SLICE_PER_FRAME_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 20);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PIC_INIT_QP_MINUS26_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 8);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 3);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 22);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_SCALING_LISTS_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 4);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_SECOND_CHROMA_QP_OFFSET_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 7);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_SLICE_MB_COUNT_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 21);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_TRANSFORM_8X8_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 13);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BIPRED_EXPLICIT_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 10);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_BIPRED_IMPLICIT_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 11);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_PRED_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 9);
pub const VK_VIDEO_ENCODE_H264_CAPABILITY_WEIGHTED_PRED_NO_TABLE_BIT_EXT: VkVideoEncodeH264CapabilityFlagBitsEXT = VkVideoEncodeH264CapabilityFlagBitsEXT(1 << 12);
impl core::fmt::Debug for VkVideoEncodeH264CapabilityFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (16777216, "B_FRAME_IN_L1_LIST"),
      (16384, "CABAC"),
      (32768, "CAVLC"),
      (64, "CHROMA_QP_OFFSET"),
      (65536, "DEBLOCKING_FILTER_DISABLED"),
      (131072, "DEBLOCKING_FILTER_ENABLED"),
      (262144, "DEBLOCKING_FILTER_PARTIAL"),
      (8388608, "DIFFERENT_SLICE_TYPE"),
      (2, "DIRECT_8X8_INFERENCE_DISABLED"),
      (1, "DIRECT_8X8_INFERENCE_ENABLED"),
      (524288, "DISABLE_DIRECT_SPATIAL_MV_PRED"),
      (32, "HRD_COMPLIANCE"),
      (1048576, "MULTIPLE_SLICE_PER_FRAME"),
      (256, "PIC_INIT_QP_MINUS26"),
      (8, "QPPRIME_Y_ZERO_TRANSFORM_BYPASS"),
      (4194304, "ROW_UNALIGNED_SLICE"),
      (16, "SCALING_LISTS"),
      (128, "SECOND_CHROMA_QP_OFFSET"),
      (4, "SEPARATE_COLOUR_PLANE"),
      (2097152, "SLICE_MB_COUNT"),
      (8192, "TRANSFORM_8X8"),
      (1024, "WEIGHTED_BIPRED_EXPLICIT"),
      (2048, "WEIGHTED_BIPRED_IMPLICIT"),
      (512, "WEIGHTED_PRED"),
      (4096, "WEIGHTED_PRED_NO_TABLE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH264InputModeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264InputModeFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH264InputModeFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH264InputModeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264InputModeFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH264InputModeFlagsEXT = VkVideoEncodeH264InputModeFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H264_INPUT_MODE_FRAME_BIT_EXT: VkVideoEncodeH264InputModeFlagBitsEXT = VkVideoEncodeH264InputModeFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H264_INPUT_MODE_NON_VCL_BIT_EXT: VkVideoEncodeH264InputModeFlagBitsEXT = VkVideoEncodeH264InputModeFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H264_INPUT_MODE_SLICE_BIT_EXT: VkVideoEncodeH264InputModeFlagBitsEXT = VkVideoEncodeH264InputModeFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkVideoEncodeH264InputModeFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "FRAME"),
      (4, "NON_VCL"),
      (2, "SLICE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH264OutputModeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264OutputModeFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH264OutputModeFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH264OutputModeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264OutputModeFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH264OutputModeFlagsEXT = VkVideoEncodeH264OutputModeFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H264_OUTPUT_MODE_FRAME_BIT_EXT: VkVideoEncodeH264OutputModeFlagBitsEXT = VkVideoEncodeH264OutputModeFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H264_OUTPUT_MODE_NON_VCL_BIT_EXT: VkVideoEncodeH264OutputModeFlagBitsEXT = VkVideoEncodeH264OutputModeFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H264_OUTPUT_MODE_SLICE_BIT_EXT: VkVideoEncodeH264OutputModeFlagBitsEXT = VkVideoEncodeH264OutputModeFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkVideoEncodeH264OutputModeFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "FRAME"),
      (4, "NON_VCL"),
      (2, "SLICE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH265CapabilityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilityFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH265CapabilityFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH265CapabilityFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilityFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH265CapabilityFlagsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 25);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DEBLOCKING_FILTER_OVERRIDE_ENABLED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 17);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DEPENDENT_SLICE_SEGMENT_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 23);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 24);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_ENTROPY_CODING_SYNC_ENABLED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 16);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 5);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_INIT_QP_MINUS26_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 6);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 7);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_PER_TILE_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 19);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILE_PER_FRAME_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 18);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILE_PER_SLICE_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 20);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PCM_ENABLE_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 3);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 11);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 22);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SAMPLE_ADAPTIVE_OFFSET_ENABLED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SCALING_LISTS_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 1);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SEPARATE_COLOUR_PLANE_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SIGN_DATA_HIDING_ENABLED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 8);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SLICE_SEGMENT_CTB_COUNT_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 21);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_SPS_TEMPORAL_MVP_ENABLED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 4);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSFORM_SKIP_DISABLED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 10);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSFORM_SKIP_ENABLED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 9);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_TRANSQUANT_BYPASS_ENABLED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 15);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_BIPRED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 13);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_PRED_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 12);
pub const VK_VIDEO_ENCODE_H265_CAPABILITY_WEIGHTED_PRED_NO_TABLE_BIT_EXT: VkVideoEncodeH265CapabilityFlagBitsEXT = VkVideoEncodeH265CapabilityFlagBitsEXT(1 << 14);
impl core::fmt::Debug for VkVideoEncodeH265CapabilityFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (33554432, "B_FRAME_IN_L1_LIST"),
      (131072, "DEBLOCKING_FILTER_OVERRIDE_ENABLED"),
      (8388608, "DEPENDENT_SLICE_SEGMENT"),
      (16777216, "DIFFERENT_SLICE_TYPE"),
      (65536, "ENTROPY_CODING_SYNC_ENABLED"),
      (32, "HRD_COMPLIANCE"),
      (64, "INIT_QP_MINUS26"),
      (128, "LOG2_PARALLEL_MERGE_LEVEL_MINUS2"),
      (524288, "MULTIPLE_SLICE_PER_TILE"),
      (262144, "MULTIPLE_TILE_PER_FRAME"),
      (1048576, "MULTIPLE_TILE_PER_SLICE"),
      (8, "PCM_ENABLE"),
      (2048, "PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT"),
      (4194304, "ROW_UNALIGNED_SLICE_SEGMENT"),
      (4, "SAMPLE_ADAPTIVE_OFFSET_ENABLED"),
      (2, "SCALING_LISTS"),
      (1, "SEPARATE_COLOUR_PLANE"),
      (256, "SIGN_DATA_HIDING_ENABLED"),
      (2097152, "SLICE_SEGMENT_CTB_COUNT"),
      (16, "SPS_TEMPORAL_MVP_ENABLED"),
      (1024, "TRANSFORM_SKIP_DISABLED"),
      (512, "TRANSFORM_SKIP_ENABLED"),
      (32768, "TRANSQUANT_BYPASS_ENABLED"),
      (8192, "WEIGHTED_BIPRED"),
      (4096, "WEIGHTED_PRED"),
      (16384, "WEIGHTED_PRED_NO_TABLE"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH265CtbSizeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CtbSizeFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH265CtbSizeFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH265CtbSizeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CtbSizeFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH265CtbSizeFlagsEXT = VkVideoEncodeH265CtbSizeFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_EXT: VkVideoEncodeH265CtbSizeFlagBitsEXT = VkVideoEncodeH265CtbSizeFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_EXT: VkVideoEncodeH265CtbSizeFlagBitsEXT = VkVideoEncodeH265CtbSizeFlagBitsEXT(1 << 1);
pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_EXT: VkVideoEncodeH265CtbSizeFlagBitsEXT = VkVideoEncodeH265CtbSizeFlagBitsEXT(1 << 2);
impl core::fmt::Debug for VkVideoEncodeH265CtbSizeFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "16"),
      (2, "32"),
      (4, "64"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH265InputModeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265InputModeFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH265InputModeFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH265InputModeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265InputModeFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH265InputModeFlagsEXT = VkVideoEncodeH265InputModeFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H265_INPUT_MODE_FRAME_BIT_EXT: VkVideoEncodeH265InputModeFlagBitsEXT = VkVideoEncodeH265InputModeFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H265_INPUT_MODE_NON_VCL_BIT_EXT: VkVideoEncodeH265InputModeFlagBitsEXT = VkVideoEncodeH265InputModeFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H265_INPUT_MODE_SLICE_SEGMENT_BIT_EXT: VkVideoEncodeH265InputModeFlagBitsEXT = VkVideoEncodeH265InputModeFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkVideoEncodeH265InputModeFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "FRAME"),
      (4, "NON_VCL"),
      (2, "SLICE_SEGMENT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH265OutputModeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265OutputModeFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH265OutputModeFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH265OutputModeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265OutputModeFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH265OutputModeFlagsEXT = VkVideoEncodeH265OutputModeFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H265_OUTPUT_MODE_FRAME_BIT_EXT: VkVideoEncodeH265OutputModeFlagBitsEXT = VkVideoEncodeH265OutputModeFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H265_OUTPUT_MODE_NON_VCL_BIT_EXT: VkVideoEncodeH265OutputModeFlagBitsEXT = VkVideoEncodeH265OutputModeFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H265_OUTPUT_MODE_SLICE_SEGMENT_BIT_EXT: VkVideoEncodeH265OutputModeFlagBitsEXT = VkVideoEncodeH265OutputModeFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkVideoEncodeH265OutputModeFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "FRAME"),
      (4, "NON_VCL"),
      (2, "SLICE_SEGMENT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeH265TransformBlockSizeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265TransformBlockSizeFlagBitsEXT.html) (bitmask)
  VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(u32)
);
/// Khronos: [VkVideoEncodeH265TransformBlockSizeFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265TransformBlockSizeFlagBitsEXT.html) (bitmask)
pub type VkVideoEncodeH265TransformBlockSizeFlagsEXT = VkVideoEncodeH265TransformBlockSizeFlagBitsEXT;
pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_EXT: VkVideoEncodeH265TransformBlockSizeFlagBitsEXT = VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(1 << 2);
pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_EXT: VkVideoEncodeH265TransformBlockSizeFlagBitsEXT = VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(1 << 3);
pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_EXT: VkVideoEncodeH265TransformBlockSizeFlagBitsEXT = VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(1 << 0);
pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_EXT: VkVideoEncodeH265TransformBlockSizeFlagBitsEXT = VkVideoEncodeH265TransformBlockSizeFlagBitsEXT(1 << 1);
impl core::fmt::Debug for VkVideoEncodeH265TransformBlockSizeFlagBitsEXT {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (4, "16"),
      (8, "32"),
      (1, "4"),
      (2, "8"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeRateControlFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlFlagsKHR.html) (bitmask, no bits defined)
  VkVideoEncodeRateControlFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeRateControlFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoEncodeRateControlFlagsKHR = VkVideoEncodeRateControlFlagBitsKHR;
impl core::fmt::Debug for VkVideoEncodeRateControlFlagBitsKHR {
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
pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR: VkVideoEncodeRateControlModeFlagBitsKHR = VkVideoEncodeRateControlModeFlagBitsKHR(1);
pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR: VkVideoEncodeRateControlModeFlagBitsKHR = VkVideoEncodeRateControlModeFlagBitsKHR(0);
pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR: VkVideoEncodeRateControlModeFlagBitsKHR = VkVideoEncodeRateControlModeFlagBitsKHR(2);
impl core::fmt::Debug for VkVideoEncodeRateControlModeFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEncodeUsageFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeUsageFlagBitsKHR.html) (bitmask)
  VkVideoEncodeUsageFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEncodeUsageFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeUsageFlagBitsKHR.html) (bitmask)
pub type VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagBitsKHR;
pub const VK_VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR: VkVideoEncodeUsageFlagBitsKHR = VkVideoEncodeUsageFlagBitsKHR(1 << 3);
pub const VK_VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR: VkVideoEncodeUsageFlagBitsKHR = VkVideoEncodeUsageFlagBitsKHR(1 << 2);
pub const VK_VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR: VkVideoEncodeUsageFlagBitsKHR = VkVideoEncodeUsageFlagBitsKHR(1 << 1);
pub const VK_VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR: VkVideoEncodeUsageFlagBitsKHR = VkVideoEncodeUsageFlagBitsKHR(1 << 0);
pub const VK_VIDEO_ENCODE_USAGE_DEFAULT_KHR: VkVideoEncodeUsageFlagBitsKHR = VkVideoEncodeUsageFlagBitsKHR(0);
impl core::fmt::Debug for VkVideoEncodeUsageFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (8, "CONFERENCING"),
      (4, "RECORDING"),
      (2, "STREAMING"),
      (1, "TRANSCODING"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoEndCodingFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingFlagsKHR.html) (bitmask, no bits defined)
  VkVideoEndCodingFlagBitsKHR(u32)
);
/// Khronos: [VkVideoEndCodingFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoEndCodingFlagsKHR = VkVideoEndCodingFlagBitsKHR;
impl core::fmt::Debug for VkVideoEndCodingFlagBitsKHR {
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
pub const VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR: VkVideoSessionCreateFlagBitsKHR = VkVideoSessionCreateFlagBitsKHR(1 << 0);
impl core::fmt::Debug for VkVideoSessionCreateFlagBitsKHR {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    for (bit_val, bit_name) in [
      (1, "PROTECTED_CONTENT"),
    ] {
      if (self.0 & bit_val) != 0 {
        x.entry(&bit_name);
      }
    }
    x.finish()
  }
}

define_bitmask!(
  /// Khronos: [VkVideoSessionParametersCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateFlagsKHR.html) (bitmask, no bits defined)
  VkVideoSessionParametersCreateFlagBitsKHR(u32)
);
/// Khronos: [VkVideoSessionParametersCreateFlagsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateFlagsKHR.html) (bitmask, no bits defined)
pub type VkVideoSessionParametersCreateFlagsKHR = VkVideoSessionParametersCreateFlagBitsKHR;
impl core::fmt::Debug for VkVideoSessionParametersCreateFlagBitsKHR {
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
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut x = f.debug_set();
    x.finish()
  }
}

