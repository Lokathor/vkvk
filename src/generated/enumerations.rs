#![allow(non_upper_case_globals)]

define_enumeration!(
  /// Khronos: [VkAccelerationStructureBuildTypeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html) (enumeration)
  VkAccelerationStructureBuildTypeKHR(u32)
);
pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR:
  VkAccelerationStructureBuildTypeKHR = VkAccelerationStructureBuildTypeKHR(1);
pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR:
  VkAccelerationStructureBuildTypeKHR = VkAccelerationStructureBuildTypeKHR(0);
pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR:
  VkAccelerationStructureBuildTypeKHR = VkAccelerationStructureBuildTypeKHR(2);
impl core::fmt::Debug for VkAccelerationStructureBuildTypeKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR => {
        "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR"
      }
      VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR => {
        "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR"
      }
      VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR => {
        "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR"
      }
      other => return write!(f, "VkAccelerationStructureBuildTypeKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkAccelerationStructureCompatibilityKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCompatibilityKHR.html) (enumeration)
  VkAccelerationStructureCompatibilityKHR(u32)
);
pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR:
  VkAccelerationStructureCompatibilityKHR = VkAccelerationStructureCompatibilityKHR(0);
pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR:
  VkAccelerationStructureCompatibilityKHR = VkAccelerationStructureCompatibilityKHR(1);
impl core::fmt::Debug for VkAccelerationStructureCompatibilityKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR => {
        "VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR"
      }
      VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR => {
        "VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR"
      }
      other => return write!(f, "VkAccelerationStructureCompatibilityKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkAccelerationStructureMemoryRequirementsTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html) (enumeration)
  VkAccelerationStructureMemoryRequirementsTypeNV(u32)
);
pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV:
  VkAccelerationStructureMemoryRequirementsTypeNV =
  VkAccelerationStructureMemoryRequirementsTypeNV(1);
pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV:
  VkAccelerationStructureMemoryRequirementsTypeNV =
  VkAccelerationStructureMemoryRequirementsTypeNV(0);
pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV:
  VkAccelerationStructureMemoryRequirementsTypeNV =
  VkAccelerationStructureMemoryRequirementsTypeNV(2);
impl core::fmt::Debug for VkAccelerationStructureMemoryRequirementsTypeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV => {
        "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV"
      }
      VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV => {
        "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV"
      }
      VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV => {
        "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV"
      }
      other => {
        return write!(f, "VkAccelerationStructureMemoryRequirementsTypeNV({})", other.0)
      }
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkAccelerationStructureMotionInstanceTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceTypeNV.html) (enumeration)
  VkAccelerationStructureMotionInstanceTypeNV(u32)
);
pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV:
  VkAccelerationStructureMotionInstanceTypeNV =
  VkAccelerationStructureMotionInstanceTypeNV(1);
pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV:
  VkAccelerationStructureMotionInstanceTypeNV =
  VkAccelerationStructureMotionInstanceTypeNV(2);
pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV:
  VkAccelerationStructureMotionInstanceTypeNV =
  VkAccelerationStructureMotionInstanceTypeNV(0);
impl core::fmt::Debug for VkAccelerationStructureMotionInstanceTypeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV => {
        "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV"
      }
      VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV => {
        "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV"
      }
      VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV => {
        "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV"
      }
      other => {
        return write!(f, "VkAccelerationStructureMotionInstanceTypeNV({})", other.0)
      }
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkAccelerationStructureTypeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTypeKHR.html) (enumeration)
  VkAccelerationStructureTypeKHR(u32)
);
pub const VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR:
  VkAccelerationStructureTypeKHR = VkAccelerationStructureTypeKHR(1);
pub const VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR: VkAccelerationStructureTypeKHR =
  VkAccelerationStructureTypeKHR(2);
pub const VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR: VkAccelerationStructureTypeKHR =
  VkAccelerationStructureTypeKHR(0);
/// Alias of [`VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR`]
pub const VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV: VkAccelerationStructureTypeKHR =
  VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR;
/// Alias of [`VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`]
pub const VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV: VkAccelerationStructureTypeKHR =
  VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR;
impl core::fmt::Debug for VkAccelerationStructureTypeKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR => {
        "VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR"
      }
      VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR => {
        "VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR"
      }
      VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR => {
        "VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR"
      }
      other => return write!(f, "VkAccelerationStructureTypeKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkAttachmentLoadOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentLoadOp.html) (enumeration)
  VkAttachmentLoadOp(u32)
);
pub const VK_ATTACHMENT_LOAD_OP_CLEAR: VkAttachmentLoadOp = VkAttachmentLoadOp(1);
pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: VkAttachmentLoadOp = VkAttachmentLoadOp(2);
pub const VK_ATTACHMENT_LOAD_OP_LOAD: VkAttachmentLoadOp = VkAttachmentLoadOp(0);
pub const VK_ATTACHMENT_LOAD_OP_NONE_EXT: VkAttachmentLoadOp =
  VkAttachmentLoadOp(1000400000);
impl core::fmt::Debug for VkAttachmentLoadOp {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_ATTACHMENT_LOAD_OP_CLEAR => "VK_ATTACHMENT_LOAD_OP_CLEAR",
      VK_ATTACHMENT_LOAD_OP_DONT_CARE => "VK_ATTACHMENT_LOAD_OP_DONT_CARE",
      VK_ATTACHMENT_LOAD_OP_LOAD => "VK_ATTACHMENT_LOAD_OP_LOAD",
      VK_ATTACHMENT_LOAD_OP_NONE_EXT => "VK_ATTACHMENT_LOAD_OP_NONE_EXT",
      other => return write!(f, "VkAttachmentLoadOp({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkAttachmentStoreOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentStoreOp.html) (enumeration)
  VkAttachmentStoreOp(u32)
);
pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: VkAttachmentStoreOp = VkAttachmentStoreOp(1);
pub const VK_ATTACHMENT_STORE_OP_NONE: VkAttachmentStoreOp =
  VkAttachmentStoreOp(1000301000);
pub const VK_ATTACHMENT_STORE_OP_STORE: VkAttachmentStoreOp = VkAttachmentStoreOp(0);
/// Alias of [`VK_ATTACHMENT_STORE_OP_NONE`]
pub const VK_ATTACHMENT_STORE_OP_NONE_EXT: VkAttachmentStoreOp =
  VK_ATTACHMENT_STORE_OP_NONE;
/// Alias of [`VK_ATTACHMENT_STORE_OP_NONE`]
pub const VK_ATTACHMENT_STORE_OP_NONE_KHR: VkAttachmentStoreOp =
  VK_ATTACHMENT_STORE_OP_NONE;
/// Alias of [`VK_ATTACHMENT_STORE_OP_NONE`]
pub const VK_ATTACHMENT_STORE_OP_NONE_QCOM: VkAttachmentStoreOp =
  VK_ATTACHMENT_STORE_OP_NONE;
impl core::fmt::Debug for VkAttachmentStoreOp {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_ATTACHMENT_STORE_OP_DONT_CARE => "VK_ATTACHMENT_STORE_OP_DONT_CARE",
      VK_ATTACHMENT_STORE_OP_NONE => "VK_ATTACHMENT_STORE_OP_NONE",
      VK_ATTACHMENT_STORE_OP_STORE => "VK_ATTACHMENT_STORE_OP_STORE",
      other => return write!(f, "VkAttachmentStoreOp({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkBlendFactor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlendFactor.html) (enumeration)
  VkBlendFactor(u32)
);
pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: VkBlendFactor = VkBlendFactor(12);
pub const VK_BLEND_FACTOR_CONSTANT_COLOR: VkBlendFactor = VkBlendFactor(10);
pub const VK_BLEND_FACTOR_DST_ALPHA: VkBlendFactor = VkBlendFactor(8);
pub const VK_BLEND_FACTOR_DST_COLOR: VkBlendFactor = VkBlendFactor(4);
pub const VK_BLEND_FACTOR_ONE: VkBlendFactor = VkBlendFactor(1);
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: VkBlendFactor = VkBlendFactor(13);
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: VkBlendFactor = VkBlendFactor(11);
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: VkBlendFactor = VkBlendFactor(9);
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: VkBlendFactor = VkBlendFactor(5);
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: VkBlendFactor = VkBlendFactor(18);
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: VkBlendFactor = VkBlendFactor(16);
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: VkBlendFactor = VkBlendFactor(7);
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: VkBlendFactor = VkBlendFactor(3);
pub const VK_BLEND_FACTOR_SRC1_ALPHA: VkBlendFactor = VkBlendFactor(17);
pub const VK_BLEND_FACTOR_SRC1_COLOR: VkBlendFactor = VkBlendFactor(15);
pub const VK_BLEND_FACTOR_SRC_ALPHA: VkBlendFactor = VkBlendFactor(6);
pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: VkBlendFactor = VkBlendFactor(14);
pub const VK_BLEND_FACTOR_SRC_COLOR: VkBlendFactor = VkBlendFactor(2);
pub const VK_BLEND_FACTOR_ZERO: VkBlendFactor = VkBlendFactor(0);
impl core::fmt::Debug for VkBlendFactor {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_BLEND_FACTOR_CONSTANT_ALPHA => "VK_BLEND_FACTOR_CONSTANT_ALPHA",
      VK_BLEND_FACTOR_CONSTANT_COLOR => "VK_BLEND_FACTOR_CONSTANT_COLOR",
      VK_BLEND_FACTOR_DST_ALPHA => "VK_BLEND_FACTOR_DST_ALPHA",
      VK_BLEND_FACTOR_DST_COLOR => "VK_BLEND_FACTOR_DST_COLOR",
      VK_BLEND_FACTOR_ONE => "VK_BLEND_FACTOR_ONE",
      VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA => {
        "VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA"
      }
      VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR => {
        "VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR"
      }
      VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA => "VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA",
      VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR => "VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR",
      VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA => "VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA",
      VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR => "VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR",
      VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA => "VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA",
      VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR => "VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR",
      VK_BLEND_FACTOR_SRC1_ALPHA => "VK_BLEND_FACTOR_SRC1_ALPHA",
      VK_BLEND_FACTOR_SRC1_COLOR => "VK_BLEND_FACTOR_SRC1_COLOR",
      VK_BLEND_FACTOR_SRC_ALPHA => "VK_BLEND_FACTOR_SRC_ALPHA",
      VK_BLEND_FACTOR_SRC_ALPHA_SATURATE => "VK_BLEND_FACTOR_SRC_ALPHA_SATURATE",
      VK_BLEND_FACTOR_SRC_COLOR => "VK_BLEND_FACTOR_SRC_COLOR",
      VK_BLEND_FACTOR_ZERO => "VK_BLEND_FACTOR_ZERO",
      other => return write!(f, "VkBlendFactor({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkBlendOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlendOp.html) (enumeration)
  VkBlendOp(u32)
);
pub const VK_BLEND_OP_ADD: VkBlendOp = VkBlendOp(0);
pub const VK_BLEND_OP_BLUE_EXT: VkBlendOp = VkBlendOp(1000148045);
pub const VK_BLEND_OP_COLORBURN_EXT: VkBlendOp = VkBlendOp(1000148018);
pub const VK_BLEND_OP_COLORDODGE_EXT: VkBlendOp = VkBlendOp(1000148017);
pub const VK_BLEND_OP_CONTRAST_EXT: VkBlendOp = VkBlendOp(1000148041);
pub const VK_BLEND_OP_DARKEN_EXT: VkBlendOp = VkBlendOp(1000148015);
pub const VK_BLEND_OP_DIFFERENCE_EXT: VkBlendOp = VkBlendOp(1000148021);
pub const VK_BLEND_OP_DST_ATOP_EXT: VkBlendOp = VkBlendOp(1000148010);
pub const VK_BLEND_OP_DST_EXT: VkBlendOp = VkBlendOp(1000148002);
pub const VK_BLEND_OP_DST_IN_EXT: VkBlendOp = VkBlendOp(1000148006);
pub const VK_BLEND_OP_DST_OUT_EXT: VkBlendOp = VkBlendOp(1000148008);
pub const VK_BLEND_OP_DST_OVER_EXT: VkBlendOp = VkBlendOp(1000148004);
pub const VK_BLEND_OP_EXCLUSION_EXT: VkBlendOp = VkBlendOp(1000148022);
pub const VK_BLEND_OP_GREEN_EXT: VkBlendOp = VkBlendOp(1000148044);
pub const VK_BLEND_OP_HARDLIGHT_EXT: VkBlendOp = VkBlendOp(1000148019);
pub const VK_BLEND_OP_HARDMIX_EXT: VkBlendOp = VkBlendOp(1000148030);
pub const VK_BLEND_OP_HSL_COLOR_EXT: VkBlendOp = VkBlendOp(1000148033);
pub const VK_BLEND_OP_HSL_HUE_EXT: VkBlendOp = VkBlendOp(1000148031);
pub const VK_BLEND_OP_HSL_LUMINOSITY_EXT: VkBlendOp = VkBlendOp(1000148034);
pub const VK_BLEND_OP_HSL_SATURATION_EXT: VkBlendOp = VkBlendOp(1000148032);
pub const VK_BLEND_OP_INVERT_EXT: VkBlendOp = VkBlendOp(1000148023);
pub const VK_BLEND_OP_INVERT_OVG_EXT: VkBlendOp = VkBlendOp(1000148042);
pub const VK_BLEND_OP_INVERT_RGB_EXT: VkBlendOp = VkBlendOp(1000148024);
pub const VK_BLEND_OP_LIGHTEN_EXT: VkBlendOp = VkBlendOp(1000148016);
pub const VK_BLEND_OP_LINEARBURN_EXT: VkBlendOp = VkBlendOp(1000148026);
pub const VK_BLEND_OP_LINEARDODGE_EXT: VkBlendOp = VkBlendOp(1000148025);
pub const VK_BLEND_OP_LINEARLIGHT_EXT: VkBlendOp = VkBlendOp(1000148028);
pub const VK_BLEND_OP_MAX: VkBlendOp = VkBlendOp(4);
pub const VK_BLEND_OP_MIN: VkBlendOp = VkBlendOp(3);
pub const VK_BLEND_OP_MINUS_CLAMPED_EXT: VkBlendOp = VkBlendOp(1000148040);
pub const VK_BLEND_OP_MINUS_EXT: VkBlendOp = VkBlendOp(1000148039);
pub const VK_BLEND_OP_MULTIPLY_EXT: VkBlendOp = VkBlendOp(1000148012);
pub const VK_BLEND_OP_OVERLAY_EXT: VkBlendOp = VkBlendOp(1000148014);
pub const VK_BLEND_OP_PINLIGHT_EXT: VkBlendOp = VkBlendOp(1000148029);
pub const VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT: VkBlendOp = VkBlendOp(1000148037);
pub const VK_BLEND_OP_PLUS_CLAMPED_EXT: VkBlendOp = VkBlendOp(1000148036);
pub const VK_BLEND_OP_PLUS_DARKER_EXT: VkBlendOp = VkBlendOp(1000148038);
pub const VK_BLEND_OP_PLUS_EXT: VkBlendOp = VkBlendOp(1000148035);
pub const VK_BLEND_OP_RED_EXT: VkBlendOp = VkBlendOp(1000148043);
pub const VK_BLEND_OP_REVERSE_SUBTRACT: VkBlendOp = VkBlendOp(2);
pub const VK_BLEND_OP_SCREEN_EXT: VkBlendOp = VkBlendOp(1000148013);
pub const VK_BLEND_OP_SOFTLIGHT_EXT: VkBlendOp = VkBlendOp(1000148020);
pub const VK_BLEND_OP_SRC_ATOP_EXT: VkBlendOp = VkBlendOp(1000148009);
pub const VK_BLEND_OP_SRC_EXT: VkBlendOp = VkBlendOp(1000148001);
pub const VK_BLEND_OP_SRC_IN_EXT: VkBlendOp = VkBlendOp(1000148005);
pub const VK_BLEND_OP_SRC_OUT_EXT: VkBlendOp = VkBlendOp(1000148007);
pub const VK_BLEND_OP_SRC_OVER_EXT: VkBlendOp = VkBlendOp(1000148003);
pub const VK_BLEND_OP_SUBTRACT: VkBlendOp = VkBlendOp(1);
pub const VK_BLEND_OP_VIVIDLIGHT_EXT: VkBlendOp = VkBlendOp(1000148027);
pub const VK_BLEND_OP_XOR_EXT: VkBlendOp = VkBlendOp(1000148011);
pub const VK_BLEND_OP_ZERO_EXT: VkBlendOp = VkBlendOp(1000148000);
impl core::fmt::Debug for VkBlendOp {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_BLEND_OP_ADD => "VK_BLEND_OP_ADD",
      VK_BLEND_OP_BLUE_EXT => "VK_BLEND_OP_BLUE_EXT",
      VK_BLEND_OP_COLORBURN_EXT => "VK_BLEND_OP_COLORBURN_EXT",
      VK_BLEND_OP_COLORDODGE_EXT => "VK_BLEND_OP_COLORDODGE_EXT",
      VK_BLEND_OP_CONTRAST_EXT => "VK_BLEND_OP_CONTRAST_EXT",
      VK_BLEND_OP_DARKEN_EXT => "VK_BLEND_OP_DARKEN_EXT",
      VK_BLEND_OP_DIFFERENCE_EXT => "VK_BLEND_OP_DIFFERENCE_EXT",
      VK_BLEND_OP_DST_ATOP_EXT => "VK_BLEND_OP_DST_ATOP_EXT",
      VK_BLEND_OP_DST_EXT => "VK_BLEND_OP_DST_EXT",
      VK_BLEND_OP_DST_IN_EXT => "VK_BLEND_OP_DST_IN_EXT",
      VK_BLEND_OP_DST_OUT_EXT => "VK_BLEND_OP_DST_OUT_EXT",
      VK_BLEND_OP_DST_OVER_EXT => "VK_BLEND_OP_DST_OVER_EXT",
      VK_BLEND_OP_EXCLUSION_EXT => "VK_BLEND_OP_EXCLUSION_EXT",
      VK_BLEND_OP_GREEN_EXT => "VK_BLEND_OP_GREEN_EXT",
      VK_BLEND_OP_HARDLIGHT_EXT => "VK_BLEND_OP_HARDLIGHT_EXT",
      VK_BLEND_OP_HARDMIX_EXT => "VK_BLEND_OP_HARDMIX_EXT",
      VK_BLEND_OP_HSL_COLOR_EXT => "VK_BLEND_OP_HSL_COLOR_EXT",
      VK_BLEND_OP_HSL_HUE_EXT => "VK_BLEND_OP_HSL_HUE_EXT",
      VK_BLEND_OP_HSL_LUMINOSITY_EXT => "VK_BLEND_OP_HSL_LUMINOSITY_EXT",
      VK_BLEND_OP_HSL_SATURATION_EXT => "VK_BLEND_OP_HSL_SATURATION_EXT",
      VK_BLEND_OP_INVERT_EXT => "VK_BLEND_OP_INVERT_EXT",
      VK_BLEND_OP_INVERT_OVG_EXT => "VK_BLEND_OP_INVERT_OVG_EXT",
      VK_BLEND_OP_INVERT_RGB_EXT => "VK_BLEND_OP_INVERT_RGB_EXT",
      VK_BLEND_OP_LIGHTEN_EXT => "VK_BLEND_OP_LIGHTEN_EXT",
      VK_BLEND_OP_LINEARBURN_EXT => "VK_BLEND_OP_LINEARBURN_EXT",
      VK_BLEND_OP_LINEARDODGE_EXT => "VK_BLEND_OP_LINEARDODGE_EXT",
      VK_BLEND_OP_LINEARLIGHT_EXT => "VK_BLEND_OP_LINEARLIGHT_EXT",
      VK_BLEND_OP_MAX => "VK_BLEND_OP_MAX",
      VK_BLEND_OP_MIN => "VK_BLEND_OP_MIN",
      VK_BLEND_OP_MINUS_CLAMPED_EXT => "VK_BLEND_OP_MINUS_CLAMPED_EXT",
      VK_BLEND_OP_MINUS_EXT => "VK_BLEND_OP_MINUS_EXT",
      VK_BLEND_OP_MULTIPLY_EXT => "VK_BLEND_OP_MULTIPLY_EXT",
      VK_BLEND_OP_OVERLAY_EXT => "VK_BLEND_OP_OVERLAY_EXT",
      VK_BLEND_OP_PINLIGHT_EXT => "VK_BLEND_OP_PINLIGHT_EXT",
      VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT => "VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT",
      VK_BLEND_OP_PLUS_CLAMPED_EXT => "VK_BLEND_OP_PLUS_CLAMPED_EXT",
      VK_BLEND_OP_PLUS_DARKER_EXT => "VK_BLEND_OP_PLUS_DARKER_EXT",
      VK_BLEND_OP_PLUS_EXT => "VK_BLEND_OP_PLUS_EXT",
      VK_BLEND_OP_RED_EXT => "VK_BLEND_OP_RED_EXT",
      VK_BLEND_OP_REVERSE_SUBTRACT => "VK_BLEND_OP_REVERSE_SUBTRACT",
      VK_BLEND_OP_SCREEN_EXT => "VK_BLEND_OP_SCREEN_EXT",
      VK_BLEND_OP_SOFTLIGHT_EXT => "VK_BLEND_OP_SOFTLIGHT_EXT",
      VK_BLEND_OP_SRC_ATOP_EXT => "VK_BLEND_OP_SRC_ATOP_EXT",
      VK_BLEND_OP_SRC_EXT => "VK_BLEND_OP_SRC_EXT",
      VK_BLEND_OP_SRC_IN_EXT => "VK_BLEND_OP_SRC_IN_EXT",
      VK_BLEND_OP_SRC_OUT_EXT => "VK_BLEND_OP_SRC_OUT_EXT",
      VK_BLEND_OP_SRC_OVER_EXT => "VK_BLEND_OP_SRC_OVER_EXT",
      VK_BLEND_OP_SUBTRACT => "VK_BLEND_OP_SUBTRACT",
      VK_BLEND_OP_VIVIDLIGHT_EXT => "VK_BLEND_OP_VIVIDLIGHT_EXT",
      VK_BLEND_OP_XOR_EXT => "VK_BLEND_OP_XOR_EXT",
      VK_BLEND_OP_ZERO_EXT => "VK_BLEND_OP_ZERO_EXT",
      other => return write!(f, "VkBlendOp({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkBlendOverlapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlendOverlapEXT.html) (enumeration)
  VkBlendOverlapEXT(u32)
);
pub const VK_BLEND_OVERLAP_CONJOINT_EXT: VkBlendOverlapEXT = VkBlendOverlapEXT(2);
pub const VK_BLEND_OVERLAP_DISJOINT_EXT: VkBlendOverlapEXT = VkBlendOverlapEXT(1);
pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT: VkBlendOverlapEXT = VkBlendOverlapEXT(0);
impl core::fmt::Debug for VkBlendOverlapEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_BLEND_OVERLAP_CONJOINT_EXT => "VK_BLEND_OVERLAP_CONJOINT_EXT",
      VK_BLEND_OVERLAP_DISJOINT_EXT => "VK_BLEND_OVERLAP_DISJOINT_EXT",
      VK_BLEND_OVERLAP_UNCORRELATED_EXT => "VK_BLEND_OVERLAP_UNCORRELATED_EXT",
      other => return write!(f, "VkBlendOverlapEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkBorderColor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBorderColor.html) (enumeration)
  VkBorderColor(u32)
);
pub const VK_BORDER_COLOR_FLOAT_CUSTOM_EXT: VkBorderColor = VkBorderColor(1000287003);
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: VkBorderColor = VkBorderColor(2);
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: VkBorderColor = VkBorderColor(4);
pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: VkBorderColor = VkBorderColor(0);
pub const VK_BORDER_COLOR_INT_CUSTOM_EXT: VkBorderColor = VkBorderColor(1000287004);
pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: VkBorderColor = VkBorderColor(3);
pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: VkBorderColor = VkBorderColor(5);
pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: VkBorderColor = VkBorderColor(1);
impl core::fmt::Debug for VkBorderColor {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_BORDER_COLOR_FLOAT_CUSTOM_EXT => "VK_BORDER_COLOR_FLOAT_CUSTOM_EXT",
      VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK => "VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK",
      VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE => "VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE",
      VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK => {
        "VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK"
      }
      VK_BORDER_COLOR_INT_CUSTOM_EXT => "VK_BORDER_COLOR_INT_CUSTOM_EXT",
      VK_BORDER_COLOR_INT_OPAQUE_BLACK => "VK_BORDER_COLOR_INT_OPAQUE_BLACK",
      VK_BORDER_COLOR_INT_OPAQUE_WHITE => "VK_BORDER_COLOR_INT_OPAQUE_WHITE",
      VK_BORDER_COLOR_INT_TRANSPARENT_BLACK => "VK_BORDER_COLOR_INT_TRANSPARENT_BLACK",
      other => return write!(f, "VkBorderColor({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkBuildAccelerationStructureModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureModeKHR.html) (enumeration)
  VkBuildAccelerationStructureModeKHR(u32)
);
pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR:
  VkBuildAccelerationStructureModeKHR = VkBuildAccelerationStructureModeKHR(0);
pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR:
  VkBuildAccelerationStructureModeKHR = VkBuildAccelerationStructureModeKHR(1);
impl core::fmt::Debug for VkBuildAccelerationStructureModeKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR => {
        "VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR"
      }
      VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR => {
        "VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR"
      }
      other => return write!(f, "VkBuildAccelerationStructureModeKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkBuildMicromapModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapModeEXT.html) (enumeration)
  VkBuildMicromapModeEXT(u32)
);
pub const VK_BUILD_MICROMAP_MODE_BUILD_EXT: VkBuildMicromapModeEXT =
  VkBuildMicromapModeEXT(0);
impl core::fmt::Debug for VkBuildMicromapModeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_BUILD_MICROMAP_MODE_BUILD_EXT => "VK_BUILD_MICROMAP_MODE_BUILD_EXT",
      other => return write!(f, "VkBuildMicromapModeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkChromaLocation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkChromaLocation.html) (enumeration)
  VkChromaLocation(u32)
);
pub const VK_CHROMA_LOCATION_COSITED_EVEN: VkChromaLocation = VkChromaLocation(0);
pub const VK_CHROMA_LOCATION_MIDPOINT: VkChromaLocation = VkChromaLocation(1);
/// Alias of [`VK_CHROMA_LOCATION_COSITED_EVEN`]
pub const VK_CHROMA_LOCATION_COSITED_EVEN_KHR: VkChromaLocation =
  VK_CHROMA_LOCATION_COSITED_EVEN;
/// Alias of [`VK_CHROMA_LOCATION_MIDPOINT`]
pub const VK_CHROMA_LOCATION_MIDPOINT_KHR: VkChromaLocation = VK_CHROMA_LOCATION_MIDPOINT;
impl core::fmt::Debug for VkChromaLocation {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_CHROMA_LOCATION_COSITED_EVEN => "VK_CHROMA_LOCATION_COSITED_EVEN",
      VK_CHROMA_LOCATION_MIDPOINT => "VK_CHROMA_LOCATION_MIDPOINT",
      other => return write!(f, "VkChromaLocation({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkCoarseSampleOrderTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderTypeNV.html) (enumeration)
  VkCoarseSampleOrderTypeNV(u32)
);
pub const VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV: VkCoarseSampleOrderTypeNV =
  VkCoarseSampleOrderTypeNV(1);
pub const VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV: VkCoarseSampleOrderTypeNV =
  VkCoarseSampleOrderTypeNV(0);
pub const VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV: VkCoarseSampleOrderTypeNV =
  VkCoarseSampleOrderTypeNV(2);
pub const VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV: VkCoarseSampleOrderTypeNV =
  VkCoarseSampleOrderTypeNV(3);
impl core::fmt::Debug for VkCoarseSampleOrderTypeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV => "VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV",
      VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV => "VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV",
      VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV => {
        "VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV"
      }
      VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV => {
        "VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV"
      }
      other => return write!(f, "VkCoarseSampleOrderTypeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkColorSpaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorSpaceKHR.html) (enumeration)
  VkColorSpaceKHR(u32)
);
pub const VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT: VkColorSpaceKHR =
  VkColorSpaceKHR(1000104011);
pub const VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: VkColorSpaceKHR =
  VkColorSpaceKHR(1000104012);
pub const VK_COLOR_SPACE_BT2020_LINEAR_EXT: VkColorSpaceKHR = VkColorSpaceKHR(1000104007);
pub const VK_COLOR_SPACE_BT709_LINEAR_EXT: VkColorSpaceKHR = VkColorSpaceKHR(1000104005);
pub const VK_COLOR_SPACE_BT709_NONLINEAR_EXT: VkColorSpaceKHR =
  VkColorSpaceKHR(1000104006);
pub const VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT: VkColorSpaceKHR =
  VkColorSpaceKHR(1000104004);
pub const VK_COLOR_SPACE_DISPLAY_NATIVE_AMD: VkColorSpaceKHR =
  VkColorSpaceKHR(1000213000);
pub const VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT: VkColorSpaceKHR =
  VkColorSpaceKHR(1000104003);
pub const VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: VkColorSpaceKHR =
  VkColorSpaceKHR(1000104001);
pub const VK_COLOR_SPACE_DOLBYVISION_EXT: VkColorSpaceKHR = VkColorSpaceKHR(1000104009);
pub const VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: VkColorSpaceKHR =
  VkColorSpaceKHR(1000104002);
pub const VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT: VkColorSpaceKHR =
  VkColorSpaceKHR(1000104014);
pub const VK_COLOR_SPACE_HDR10_HLG_EXT: VkColorSpaceKHR = VkColorSpaceKHR(1000104010);
pub const VK_COLOR_SPACE_HDR10_ST2084_EXT: VkColorSpaceKHR = VkColorSpaceKHR(1000104008);
pub const VK_COLOR_SPACE_PASS_THROUGH_EXT: VkColorSpaceKHR = VkColorSpaceKHR(1000104013);
pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VkColorSpaceKHR(0);
/// Alias of [`VK_COLOR_SPACE_SRGB_NONLINEAR_KHR`]
#[deprecated = "aliased"]
pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR =
  VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
/// Alias of [`VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT`]
#[deprecated = "aliased"]
pub const VK_COLOR_SPACE_DCI_P3_LINEAR_EXT: VkColorSpaceKHR =
  VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT;
impl core::fmt::Debug for VkColorSpaceKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT => "VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT",
      VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT => "VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT",
      VK_COLOR_SPACE_BT2020_LINEAR_EXT => "VK_COLOR_SPACE_BT2020_LINEAR_EXT",
      VK_COLOR_SPACE_BT709_LINEAR_EXT => "VK_COLOR_SPACE_BT709_LINEAR_EXT",
      VK_COLOR_SPACE_BT709_NONLINEAR_EXT => "VK_COLOR_SPACE_BT709_NONLINEAR_EXT",
      VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT => "VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT",
      VK_COLOR_SPACE_DISPLAY_NATIVE_AMD => "VK_COLOR_SPACE_DISPLAY_NATIVE_AMD",
      VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT => "VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT",
      VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT => {
        "VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT"
      }
      VK_COLOR_SPACE_DOLBYVISION_EXT => "VK_COLOR_SPACE_DOLBYVISION_EXT",
      VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT => {
        "VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT"
      }
      VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT => {
        "VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT"
      }
      VK_COLOR_SPACE_HDR10_HLG_EXT => "VK_COLOR_SPACE_HDR10_HLG_EXT",
      VK_COLOR_SPACE_HDR10_ST2084_EXT => "VK_COLOR_SPACE_HDR10_ST2084_EXT",
      VK_COLOR_SPACE_PASS_THROUGH_EXT => "VK_COLOR_SPACE_PASS_THROUGH_EXT",
      VK_COLOR_SPACE_SRGB_NONLINEAR_KHR => "VK_COLOR_SPACE_SRGB_NONLINEAR_KHR",
      other => return write!(f, "VkColorSpaceKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkCommandBufferLevel](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferLevel.html) (enumeration)
  VkCommandBufferLevel(u32)
);
pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = VkCommandBufferLevel(0);
pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: VkCommandBufferLevel =
  VkCommandBufferLevel(1);
impl core::fmt::Debug for VkCommandBufferLevel {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COMMAND_BUFFER_LEVEL_PRIMARY => "VK_COMMAND_BUFFER_LEVEL_PRIMARY",
      VK_COMMAND_BUFFER_LEVEL_SECONDARY => "VK_COMMAND_BUFFER_LEVEL_SECONDARY",
      other => return write!(f, "VkCommandBufferLevel({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkCompareOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompareOp.html) (enumeration)
  VkCompareOp(u32)
);
pub const VK_COMPARE_OP_ALWAYS: VkCompareOp = VkCompareOp(7);
pub const VK_COMPARE_OP_EQUAL: VkCompareOp = VkCompareOp(2);
pub const VK_COMPARE_OP_GREATER: VkCompareOp = VkCompareOp(4);
pub const VK_COMPARE_OP_GREATER_OR_EQUAL: VkCompareOp = VkCompareOp(6);
pub const VK_COMPARE_OP_LESS: VkCompareOp = VkCompareOp(1);
pub const VK_COMPARE_OP_LESS_OR_EQUAL: VkCompareOp = VkCompareOp(3);
pub const VK_COMPARE_OP_NEVER: VkCompareOp = VkCompareOp(0);
pub const VK_COMPARE_OP_NOT_EQUAL: VkCompareOp = VkCompareOp(5);
impl core::fmt::Debug for VkCompareOp {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COMPARE_OP_ALWAYS => "VK_COMPARE_OP_ALWAYS",
      VK_COMPARE_OP_EQUAL => "VK_COMPARE_OP_EQUAL",
      VK_COMPARE_OP_GREATER => "VK_COMPARE_OP_GREATER",
      VK_COMPARE_OP_GREATER_OR_EQUAL => "VK_COMPARE_OP_GREATER_OR_EQUAL",
      VK_COMPARE_OP_LESS => "VK_COMPARE_OP_LESS",
      VK_COMPARE_OP_LESS_OR_EQUAL => "VK_COMPARE_OP_LESS_OR_EQUAL",
      VK_COMPARE_OP_NEVER => "VK_COMPARE_OP_NEVER",
      VK_COMPARE_OP_NOT_EQUAL => "VK_COMPARE_OP_NOT_EQUAL",
      other => return write!(f, "VkCompareOp({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkComponentSwizzle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentSwizzle.html) (enumeration)
  VkComponentSwizzle(u32)
);
pub const VK_COMPONENT_SWIZZLE_A: VkComponentSwizzle = VkComponentSwizzle(6);
pub const VK_COMPONENT_SWIZZLE_B: VkComponentSwizzle = VkComponentSwizzle(5);
pub const VK_COMPONENT_SWIZZLE_G: VkComponentSwizzle = VkComponentSwizzle(4);
pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = VkComponentSwizzle(0);
pub const VK_COMPONENT_SWIZZLE_ONE: VkComponentSwizzle = VkComponentSwizzle(2);
pub const VK_COMPONENT_SWIZZLE_R: VkComponentSwizzle = VkComponentSwizzle(3);
pub const VK_COMPONENT_SWIZZLE_ZERO: VkComponentSwizzle = VkComponentSwizzle(1);
impl core::fmt::Debug for VkComponentSwizzle {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COMPONENT_SWIZZLE_A => "VK_COMPONENT_SWIZZLE_A",
      VK_COMPONENT_SWIZZLE_B => "VK_COMPONENT_SWIZZLE_B",
      VK_COMPONENT_SWIZZLE_G => "VK_COMPONENT_SWIZZLE_G",
      VK_COMPONENT_SWIZZLE_IDENTITY => "VK_COMPONENT_SWIZZLE_IDENTITY",
      VK_COMPONENT_SWIZZLE_ONE => "VK_COMPONENT_SWIZZLE_ONE",
      VK_COMPONENT_SWIZZLE_R => "VK_COMPONENT_SWIZZLE_R",
      VK_COMPONENT_SWIZZLE_ZERO => "VK_COMPONENT_SWIZZLE_ZERO",
      other => return write!(f, "VkComponentSwizzle({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkComponentTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentTypeNV.html) (enumeration)
  VkComponentTypeNV(u32)
);
pub const VK_COMPONENT_TYPE_FLOAT16_NV: VkComponentTypeNV = VkComponentTypeNV(0);
pub const VK_COMPONENT_TYPE_FLOAT32_NV: VkComponentTypeNV = VkComponentTypeNV(1);
pub const VK_COMPONENT_TYPE_FLOAT64_NV: VkComponentTypeNV = VkComponentTypeNV(2);
pub const VK_COMPONENT_TYPE_SINT16_NV: VkComponentTypeNV = VkComponentTypeNV(4);
pub const VK_COMPONENT_TYPE_SINT32_NV: VkComponentTypeNV = VkComponentTypeNV(5);
pub const VK_COMPONENT_TYPE_SINT64_NV: VkComponentTypeNV = VkComponentTypeNV(6);
pub const VK_COMPONENT_TYPE_SINT8_NV: VkComponentTypeNV = VkComponentTypeNV(3);
pub const VK_COMPONENT_TYPE_UINT16_NV: VkComponentTypeNV = VkComponentTypeNV(8);
pub const VK_COMPONENT_TYPE_UINT32_NV: VkComponentTypeNV = VkComponentTypeNV(9);
pub const VK_COMPONENT_TYPE_UINT64_NV: VkComponentTypeNV = VkComponentTypeNV(10);
pub const VK_COMPONENT_TYPE_UINT8_NV: VkComponentTypeNV = VkComponentTypeNV(7);
impl core::fmt::Debug for VkComponentTypeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COMPONENT_TYPE_FLOAT16_NV => "VK_COMPONENT_TYPE_FLOAT16_NV",
      VK_COMPONENT_TYPE_FLOAT32_NV => "VK_COMPONENT_TYPE_FLOAT32_NV",
      VK_COMPONENT_TYPE_FLOAT64_NV => "VK_COMPONENT_TYPE_FLOAT64_NV",
      VK_COMPONENT_TYPE_SINT16_NV => "VK_COMPONENT_TYPE_SINT16_NV",
      VK_COMPONENT_TYPE_SINT32_NV => "VK_COMPONENT_TYPE_SINT32_NV",
      VK_COMPONENT_TYPE_SINT64_NV => "VK_COMPONENT_TYPE_SINT64_NV",
      VK_COMPONENT_TYPE_SINT8_NV => "VK_COMPONENT_TYPE_SINT8_NV",
      VK_COMPONENT_TYPE_UINT16_NV => "VK_COMPONENT_TYPE_UINT16_NV",
      VK_COMPONENT_TYPE_UINT32_NV => "VK_COMPONENT_TYPE_UINT32_NV",
      VK_COMPONENT_TYPE_UINT64_NV => "VK_COMPONENT_TYPE_UINT64_NV",
      VK_COMPONENT_TYPE_UINT8_NV => "VK_COMPONENT_TYPE_UINT8_NV",
      other => return write!(f, "VkComponentTypeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkConservativeRasterizationModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConservativeRasterizationModeEXT.html) (enumeration)
  VkConservativeRasterizationModeEXT(u32)
);
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT:
  VkConservativeRasterizationModeEXT = VkConservativeRasterizationModeEXT(0);
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT:
  VkConservativeRasterizationModeEXT = VkConservativeRasterizationModeEXT(1);
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT:
  VkConservativeRasterizationModeEXT = VkConservativeRasterizationModeEXT(2);
impl core::fmt::Debug for VkConservativeRasterizationModeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT => {
        "VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT"
      }
      VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT => {
        "VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT"
      }
      VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT => {
        "VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT"
      }
      other => return write!(f, "VkConservativeRasterizationModeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkCopyAccelerationStructureModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureModeKHR.html) (enumeration)
  VkCopyAccelerationStructureModeKHR(u32)
);
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR:
  VkCopyAccelerationStructureModeKHR = VkCopyAccelerationStructureModeKHR(0);
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR:
  VkCopyAccelerationStructureModeKHR = VkCopyAccelerationStructureModeKHR(1);
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR:
  VkCopyAccelerationStructureModeKHR = VkCopyAccelerationStructureModeKHR(3);
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR:
  VkCopyAccelerationStructureModeKHR = VkCopyAccelerationStructureModeKHR(2);
/// Alias of [`VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR`]
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV:
  VkCopyAccelerationStructureModeKHR = VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR;
/// Alias of [`VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR`]
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV:
  VkCopyAccelerationStructureModeKHR = VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR;
impl core::fmt::Debug for VkCopyAccelerationStructureModeKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR => {
        "VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR"
      }
      VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR => {
        "VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR"
      }
      VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR => {
        "VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR"
      }
      VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR => {
        "VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR"
      }
      other => return write!(f, "VkCopyAccelerationStructureModeKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkCopyMicromapModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMicromapModeEXT.html) (enumeration)
  VkCopyMicromapModeEXT(u32)
);
pub const VK_COPY_MICROMAP_MODE_CLONE_EXT: VkCopyMicromapModeEXT =
  VkCopyMicromapModeEXT(0);
pub const VK_COPY_MICROMAP_MODE_COMPACT_EXT: VkCopyMicromapModeEXT =
  VkCopyMicromapModeEXT(3);
pub const VK_COPY_MICROMAP_MODE_DESERIALIZE_EXT: VkCopyMicromapModeEXT =
  VkCopyMicromapModeEXT(2);
pub const VK_COPY_MICROMAP_MODE_SERIALIZE_EXT: VkCopyMicromapModeEXT =
  VkCopyMicromapModeEXT(1);
impl core::fmt::Debug for VkCopyMicromapModeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COPY_MICROMAP_MODE_CLONE_EXT => "VK_COPY_MICROMAP_MODE_CLONE_EXT",
      VK_COPY_MICROMAP_MODE_COMPACT_EXT => "VK_COPY_MICROMAP_MODE_COMPACT_EXT",
      VK_COPY_MICROMAP_MODE_DESERIALIZE_EXT => "VK_COPY_MICROMAP_MODE_DESERIALIZE_EXT",
      VK_COPY_MICROMAP_MODE_SERIALIZE_EXT => "VK_COPY_MICROMAP_MODE_SERIALIZE_EXT",
      other => return write!(f, "VkCopyMicromapModeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkCoverageModulationModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCoverageModulationModeNV.html) (enumeration)
  VkCoverageModulationModeNV(u32)
);
pub const VK_COVERAGE_MODULATION_MODE_ALPHA_NV: VkCoverageModulationModeNV =
  VkCoverageModulationModeNV(2);
pub const VK_COVERAGE_MODULATION_MODE_NONE_NV: VkCoverageModulationModeNV =
  VkCoverageModulationModeNV(0);
pub const VK_COVERAGE_MODULATION_MODE_RGBA_NV: VkCoverageModulationModeNV =
  VkCoverageModulationModeNV(3);
pub const VK_COVERAGE_MODULATION_MODE_RGB_NV: VkCoverageModulationModeNV =
  VkCoverageModulationModeNV(1);
impl core::fmt::Debug for VkCoverageModulationModeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COVERAGE_MODULATION_MODE_ALPHA_NV => "VK_COVERAGE_MODULATION_MODE_ALPHA_NV",
      VK_COVERAGE_MODULATION_MODE_NONE_NV => "VK_COVERAGE_MODULATION_MODE_NONE_NV",
      VK_COVERAGE_MODULATION_MODE_RGBA_NV => "VK_COVERAGE_MODULATION_MODE_RGBA_NV",
      VK_COVERAGE_MODULATION_MODE_RGB_NV => "VK_COVERAGE_MODULATION_MODE_RGB_NV",
      other => return write!(f, "VkCoverageModulationModeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkCoverageReductionModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCoverageReductionModeNV.html) (enumeration)
  VkCoverageReductionModeNV(u32)
);
pub const VK_COVERAGE_REDUCTION_MODE_MERGE_NV: VkCoverageReductionModeNV =
  VkCoverageReductionModeNV(0);
pub const VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV: VkCoverageReductionModeNV =
  VkCoverageReductionModeNV(1);
impl core::fmt::Debug for VkCoverageReductionModeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_COVERAGE_REDUCTION_MODE_MERGE_NV => "VK_COVERAGE_REDUCTION_MODE_MERGE_NV",
      VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV => "VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV",
      other => return write!(f, "VkCoverageReductionModeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDebugReportObjectTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportObjectTypeEXT.html) (enumeration)
  VkDebugReportObjectTypeEXT(u32)
);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT:
  VkDebugReportObjectTypeEXT = VkDebugReportObjectTypeEXT(1000150000);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT:
  VkDebugReportObjectTypeEXT = VkDebugReportObjectTypeEXT(1000165000);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT:
  VkDebugReportObjectTypeEXT = VkDebugReportObjectTypeEXT(1000366000);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(9);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(13);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(6);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(25);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(1000029001);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(1000029000);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT:
  VkDebugReportObjectTypeEXT = VkDebugReportObjectTypeEXT(28);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(22);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(23);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT:
  VkDebugReportObjectTypeEXT = VkDebugReportObjectTypeEXT(20);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT:
  VkDebugReportObjectTypeEXT = VkDebugReportObjectTypeEXT(1000085000);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(3);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(8);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(29);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(30);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(11);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(7);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(24);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(10);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(14);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(1);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(2);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(16);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(19);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(17);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(12);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(4);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(18);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(21);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT:
  VkDebugReportObjectTypeEXT = VkDebugReportObjectTypeEXT(1000156000);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(5);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(15);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(26);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(27);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT(0);
pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT:
  VkDebugReportObjectTypeEXT = VkDebugReportObjectTypeEXT(33);
/// Alias of [`VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT`]
#[deprecated = "aliased"]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT: VkDebugReportObjectTypeEXT =
  VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT;
/// Alias of [`VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT`]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT:
  VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT;
/// Alias of [`VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT`]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT:
  VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT;
/// Alias of [`VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT`]
#[deprecated = "aliased"]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkDebugReportObjectTypeEXT =
  VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT;
impl core::fmt::Debug for VkDebugReportObjectTypeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT => "VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT",
      VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT => "VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT",
      VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT => "VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT",
      VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT => "VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT",
      VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT => "VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT",
      VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT => "VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT",
      VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT"
      }
      VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT => {
        "VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT"
      }
      other => return write!(f, "VkDebugReportObjectTypeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDescriptorType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorType.html) (enumeration)
  VkDescriptorType(u32)
);
pub const VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR: VkDescriptorType =
  VkDescriptorType(1000150000);
pub const VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV: VkDescriptorType =
  VkDescriptorType(1000165000);
pub const VK_DESCRIPTOR_TYPE_BLOCK_MATCH_IMAGE_QCOM: VkDescriptorType =
  VkDescriptorType(1000440001);
pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: VkDescriptorType =
  VkDescriptorType(1);
pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK: VkDescriptorType =
  VkDescriptorType(1000138000);
pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: VkDescriptorType = VkDescriptorType(10);
pub const VK_DESCRIPTOR_TYPE_MUTABLE_EXT: VkDescriptorType = VkDescriptorType(1000351000);
pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: VkDescriptorType = VkDescriptorType(2);
pub const VK_DESCRIPTOR_TYPE_SAMPLER: VkDescriptorType = VkDescriptorType(0);
pub const VK_DESCRIPTOR_TYPE_SAMPLE_WEIGHT_IMAGE_QCOM: VkDescriptorType =
  VkDescriptorType(1000440000);
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: VkDescriptorType = VkDescriptorType(7);
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: VkDescriptorType =
  VkDescriptorType(9);
pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: VkDescriptorType = VkDescriptorType(3);
pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: VkDescriptorType = VkDescriptorType(5);
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: VkDescriptorType = VkDescriptorType(6);
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: VkDescriptorType =
  VkDescriptorType(8);
pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: VkDescriptorType = VkDescriptorType(4);
/// Alias of [`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`]
pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT: VkDescriptorType =
  VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK;
/// Alias of [`VK_DESCRIPTOR_TYPE_MUTABLE_EXT`]
pub const VK_DESCRIPTOR_TYPE_MUTABLE_VALVE: VkDescriptorType =
  VK_DESCRIPTOR_TYPE_MUTABLE_EXT;
impl core::fmt::Debug for VkDescriptorType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR => {
        "VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR"
      }
      VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV => {
        "VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV"
      }
      VK_DESCRIPTOR_TYPE_BLOCK_MATCH_IMAGE_QCOM => {
        "VK_DESCRIPTOR_TYPE_BLOCK_MATCH_IMAGE_QCOM"
      }
      VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER => {
        "VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER"
      }
      VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK => {
        "VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK"
      }
      VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT => "VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT",
      VK_DESCRIPTOR_TYPE_MUTABLE_EXT => "VK_DESCRIPTOR_TYPE_MUTABLE_EXT",
      VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE => "VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE",
      VK_DESCRIPTOR_TYPE_SAMPLER => "VK_DESCRIPTOR_TYPE_SAMPLER",
      VK_DESCRIPTOR_TYPE_SAMPLE_WEIGHT_IMAGE_QCOM => {
        "VK_DESCRIPTOR_TYPE_SAMPLE_WEIGHT_IMAGE_QCOM"
      }
      VK_DESCRIPTOR_TYPE_STORAGE_BUFFER => "VK_DESCRIPTOR_TYPE_STORAGE_BUFFER",
      VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC => {
        "VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC"
      }
      VK_DESCRIPTOR_TYPE_STORAGE_IMAGE => "VK_DESCRIPTOR_TYPE_STORAGE_IMAGE",
      VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER => {
        "VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER"
      }
      VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER => "VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER",
      VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC => {
        "VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC"
      }
      VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER => {
        "VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER"
      }
      other => return write!(f, "VkDescriptorType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDescriptorUpdateTemplateType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateType.html) (enumeration)
  VkDescriptorUpdateTemplateType(u32)
);
/// Create descriptor update template for descriptor set updates
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET:
  VkDescriptorUpdateTemplateType = VkDescriptorUpdateTemplateType(0);
/// Create descriptor update template for pushed descriptor updates
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR:
  VkDescriptorUpdateTemplateType = VkDescriptorUpdateTemplateType(1);
/// Alias of [`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`]
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR:
  VkDescriptorUpdateTemplateType = VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET;
impl core::fmt::Debug for VkDescriptorUpdateTemplateType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET => {
        "VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET"
      }
      VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR => {
        "VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR"
      }
      other => return write!(f, "VkDescriptorUpdateTemplateType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDeviceAddressBindingTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingTypeEXT.html) (enumeration)
  VkDeviceAddressBindingTypeEXT(u32)
);
pub const VK_DEVICE_ADDRESS_BINDING_TYPE_BIND_EXT: VkDeviceAddressBindingTypeEXT =
  VkDeviceAddressBindingTypeEXT(0);
pub const VK_DEVICE_ADDRESS_BINDING_TYPE_UNBIND_EXT: VkDeviceAddressBindingTypeEXT =
  VkDeviceAddressBindingTypeEXT(1);
impl core::fmt::Debug for VkDeviceAddressBindingTypeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DEVICE_ADDRESS_BINDING_TYPE_BIND_EXT => {
        "VK_DEVICE_ADDRESS_BINDING_TYPE_BIND_EXT"
      }
      VK_DEVICE_ADDRESS_BINDING_TYPE_UNBIND_EXT => {
        "VK_DEVICE_ADDRESS_BINDING_TYPE_UNBIND_EXT"
      }
      other => return write!(f, "VkDeviceAddressBindingTypeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDeviceEventTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceEventTypeEXT.html) (enumeration)
  VkDeviceEventTypeEXT(u32)
);
pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: VkDeviceEventTypeEXT =
  VkDeviceEventTypeEXT(0);
impl core::fmt::Debug for VkDeviceEventTypeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT => {
        "VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT"
      }
      other => return write!(f, "VkDeviceEventTypeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDeviceFaultAddressTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultAddressTypeEXT.html) (enumeration)
  VkDeviceFaultAddressTypeEXT(u32)
);
pub const VK_DEVICE_FAULT_ADDRESS_TYPE_EXECUTE_INVALID_EXT: VkDeviceFaultAddressTypeEXT =
  VkDeviceFaultAddressTypeEXT(3);
pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_FAULT_EXT:
  VkDeviceFaultAddressTypeEXT = VkDeviceFaultAddressTypeEXT(6);
pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_INVALID_EXT:
  VkDeviceFaultAddressTypeEXT = VkDeviceFaultAddressTypeEXT(5);
pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_UNKNOWN_EXT:
  VkDeviceFaultAddressTypeEXT = VkDeviceFaultAddressTypeEXT(4);
/// Currently unused
pub const VK_DEVICE_FAULT_ADDRESS_TYPE_NONE_EXT: VkDeviceFaultAddressTypeEXT =
  VkDeviceFaultAddressTypeEXT(0);
pub const VK_DEVICE_FAULT_ADDRESS_TYPE_READ_INVALID_EXT: VkDeviceFaultAddressTypeEXT =
  VkDeviceFaultAddressTypeEXT(1);
pub const VK_DEVICE_FAULT_ADDRESS_TYPE_WRITE_INVALID_EXT: VkDeviceFaultAddressTypeEXT =
  VkDeviceFaultAddressTypeEXT(2);
impl core::fmt::Debug for VkDeviceFaultAddressTypeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DEVICE_FAULT_ADDRESS_TYPE_EXECUTE_INVALID_EXT => {
        "VK_DEVICE_FAULT_ADDRESS_TYPE_EXECUTE_INVALID_EXT"
      }
      VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_FAULT_EXT => {
        "VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_FAULT_EXT"
      }
      VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_INVALID_EXT => {
        "VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_INVALID_EXT"
      }
      VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_UNKNOWN_EXT => {
        "VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_UNKNOWN_EXT"
      }
      VK_DEVICE_FAULT_ADDRESS_TYPE_NONE_EXT => "VK_DEVICE_FAULT_ADDRESS_TYPE_NONE_EXT",
      VK_DEVICE_FAULT_ADDRESS_TYPE_READ_INVALID_EXT => {
        "VK_DEVICE_FAULT_ADDRESS_TYPE_READ_INVALID_EXT"
      }
      VK_DEVICE_FAULT_ADDRESS_TYPE_WRITE_INVALID_EXT => {
        "VK_DEVICE_FAULT_ADDRESS_TYPE_WRITE_INVALID_EXT"
      }
      other => return write!(f, "VkDeviceFaultAddressTypeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDeviceFaultVendorBinaryHeaderVersionEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultVendorBinaryHeaderVersionEXT.html) (enumeration)
  VkDeviceFaultVendorBinaryHeaderVersionEXT(u32)
);
pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_EXT:
  VkDeviceFaultVendorBinaryHeaderVersionEXT =
  VkDeviceFaultVendorBinaryHeaderVersionEXT(1);
impl core::fmt::Debug for VkDeviceFaultVendorBinaryHeaderVersionEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_EXT => {
        "VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_EXT"
      }
      other => {
        return write!(f, "VkDeviceFaultVendorBinaryHeaderVersionEXT({})", other.0)
      }
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDeviceMemoryReportEventTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportEventTypeEXT.html) (enumeration)
  VkDeviceMemoryReportEventTypeEXT(u32)
);
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT:
  VkDeviceMemoryReportEventTypeEXT = VkDeviceMemoryReportEventTypeEXT(0);
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT:
  VkDeviceMemoryReportEventTypeEXT = VkDeviceMemoryReportEventTypeEXT(4);
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT: VkDeviceMemoryReportEventTypeEXT =
  VkDeviceMemoryReportEventTypeEXT(1);
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT:
  VkDeviceMemoryReportEventTypeEXT = VkDeviceMemoryReportEventTypeEXT(2);
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT:
  VkDeviceMemoryReportEventTypeEXT = VkDeviceMemoryReportEventTypeEXT(3);
impl core::fmt::Debug for VkDeviceMemoryReportEventTypeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT => {
        "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT"
      }
      VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT => {
        "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT"
      }
      VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT => {
        "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT"
      }
      VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT => {
        "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT"
      }
      VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT => {
        "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT"
      }
      other => return write!(f, "VkDeviceMemoryReportEventTypeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDirectDriverLoadingModeLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingModeLUNARG.html) (enumeration)
  VkDirectDriverLoadingModeLUNARG(u32)
);
pub const VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG:
  VkDirectDriverLoadingModeLUNARG = VkDirectDriverLoadingModeLUNARG(0);
pub const VK_DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG:
  VkDirectDriverLoadingModeLUNARG = VkDirectDriverLoadingModeLUNARG(1);
impl core::fmt::Debug for VkDirectDriverLoadingModeLUNARG {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG => {
        "VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG"
      }
      VK_DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG => {
        "VK_DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG"
      }
      other => return write!(f, "VkDirectDriverLoadingModeLUNARG({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDiscardRectangleModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDiscardRectangleModeEXT.html) (enumeration)
  VkDiscardRectangleModeEXT(u32)
);
pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: VkDiscardRectangleModeEXT =
  VkDiscardRectangleModeEXT(1);
pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: VkDiscardRectangleModeEXT =
  VkDiscardRectangleModeEXT(0);
impl core::fmt::Debug for VkDiscardRectangleModeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT => {
        "VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT"
      }
      VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT => {
        "VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT"
      }
      other => return write!(f, "VkDiscardRectangleModeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDisplayEventTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayEventTypeEXT.html) (enumeration)
  VkDisplayEventTypeEXT(u32)
);
pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: VkDisplayEventTypeEXT =
  VkDisplayEventTypeEXT(0);
impl core::fmt::Debug for VkDisplayEventTypeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT => {
        "VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT"
      }
      other => return write!(f, "VkDisplayEventTypeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDisplayPowerStateEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerStateEXT.html) (enumeration)
  VkDisplayPowerStateEXT(u32)
);
pub const VK_DISPLAY_POWER_STATE_OFF_EXT: VkDisplayPowerStateEXT =
  VkDisplayPowerStateEXT(0);
pub const VK_DISPLAY_POWER_STATE_ON_EXT: VkDisplayPowerStateEXT =
  VkDisplayPowerStateEXT(2);
pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT: VkDisplayPowerStateEXT =
  VkDisplayPowerStateEXT(1);
impl core::fmt::Debug for VkDisplayPowerStateEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DISPLAY_POWER_STATE_OFF_EXT => "VK_DISPLAY_POWER_STATE_OFF_EXT",
      VK_DISPLAY_POWER_STATE_ON_EXT => "VK_DISPLAY_POWER_STATE_ON_EXT",
      VK_DISPLAY_POWER_STATE_SUSPEND_EXT => "VK_DISPLAY_POWER_STATE_SUSPEND_EXT",
      other => return write!(f, "VkDisplayPowerStateEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDriverId](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDriverId.html) (enumeration)
  VkDriverId(u32)
);
/// Advanced Micro Devices, Inc.
pub const VK_DRIVER_ID_AMD_OPEN_SOURCE: VkDriverId = VkDriverId(2);
/// Advanced Micro Devices, Inc.
pub const VK_DRIVER_ID_AMD_PROPRIETARY: VkDriverId = VkDriverId(1);
/// Arm Limited
pub const VK_DRIVER_ID_ARM_PROPRIETARY: VkDriverId = VkDriverId(9);
/// Broadcom Inc.
pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY: VkDriverId = VkDriverId(12);
/// Core Avionics &amp; Industrial Inc.
pub const VK_DRIVER_ID_COREAVI_PROPRIETARY: VkDriverId = VkDriverId(15);
/// Google LLC
pub const VK_DRIVER_ID_GGP_PROPRIETARY: VkDriverId = VkDriverId(11);
/// Google LLC
pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER: VkDriverId = VkDriverId(10);
/// Imagination Technologies
pub const VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA: VkDriverId = VkDriverId(25);
/// Imagination Technologies
pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY: VkDriverId = VkDriverId(7);
/// Intel Corporation
pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA: VkDriverId = VkDriverId(6);
/// Intel Corporation
pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: VkDriverId = VkDriverId(5);
/// Juice Technologies, Inc.
pub const VK_DRIVER_ID_JUICE_PROPRIETARY: VkDriverId = VkDriverId(16);
/// Mesa open source project
pub const VK_DRIVER_ID_MESA_DOZEN: VkDriverId = VkDriverId(23);
/// Mesa
pub const VK_DRIVER_ID_MESA_LLVMPIPE: VkDriverId = VkDriverId(13);
/// Mesa open source project
pub const VK_DRIVER_ID_MESA_NVK: VkDriverId = VkDriverId(24);
/// Mesa open source project
pub const VK_DRIVER_ID_MESA_PANVK: VkDriverId = VkDriverId(20);
/// Mesa open source project
pub const VK_DRIVER_ID_MESA_RADV: VkDriverId = VkDriverId(3);
/// Mesa open source project
pub const VK_DRIVER_ID_MESA_TURNIP: VkDriverId = VkDriverId(18);
/// Mesa open source project
pub const VK_DRIVER_ID_MESA_V3DV: VkDriverId = VkDriverId(19);
/// Mesa open source project
pub const VK_DRIVER_ID_MESA_VENUS: VkDriverId = VkDriverId(22);
/// MoltenVK
pub const VK_DRIVER_ID_MOLTENVK: VkDriverId = VkDriverId(14);
/// NVIDIA Corporation
pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY: VkDriverId = VkDriverId(4);
/// Qualcomm Technologies, Inc.
pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY: VkDriverId = VkDriverId(8);
/// Samsung Electronics Co., Ltd.
pub const VK_DRIVER_ID_SAMSUNG_PROPRIETARY: VkDriverId = VkDriverId(21);
/// Verisilicon, Inc.
pub const VK_DRIVER_ID_VERISILICON_PROPRIETARY: VkDriverId = VkDriverId(17);
/// Alias of [`VK_DRIVER_ID_AMD_OPEN_SOURCE`]
pub const VK_DRIVER_ID_AMD_OPEN_SOURCE_KHR: VkDriverId = VK_DRIVER_ID_AMD_OPEN_SOURCE;
/// Alias of [`VK_DRIVER_ID_AMD_PROPRIETARY`]
pub const VK_DRIVER_ID_AMD_PROPRIETARY_KHR: VkDriverId = VK_DRIVER_ID_AMD_PROPRIETARY;
/// Alias of [`VK_DRIVER_ID_ARM_PROPRIETARY`]
pub const VK_DRIVER_ID_ARM_PROPRIETARY_KHR: VkDriverId = VK_DRIVER_ID_ARM_PROPRIETARY;
/// Alias of [`VK_DRIVER_ID_BROADCOM_PROPRIETARY`]
pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY_KHR: VkDriverId =
  VK_DRIVER_ID_BROADCOM_PROPRIETARY;
/// Alias of [`VK_DRIVER_ID_GGP_PROPRIETARY`]
pub const VK_DRIVER_ID_GGP_PROPRIETARY_KHR: VkDriverId = VK_DRIVER_ID_GGP_PROPRIETARY;
/// Alias of [`VK_DRIVER_ID_GOOGLE_SWIFTSHADER`]
pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER_KHR: VkDriverId =
  VK_DRIVER_ID_GOOGLE_SWIFTSHADER;
/// Alias of [`VK_DRIVER_ID_IMAGINATION_PROPRIETARY`]
pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY_KHR: VkDriverId =
  VK_DRIVER_ID_IMAGINATION_PROPRIETARY;
/// Alias of [`VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA`]
pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA_KHR: VkDriverId =
  VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA;
/// Alias of [`VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS`]
pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS_KHR: VkDriverId =
  VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS;
/// Alias of [`VK_DRIVER_ID_MESA_RADV`]
pub const VK_DRIVER_ID_MESA_RADV_KHR: VkDriverId = VK_DRIVER_ID_MESA_RADV;
/// Alias of [`VK_DRIVER_ID_NVIDIA_PROPRIETARY`]
pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY_KHR: VkDriverId =
  VK_DRIVER_ID_NVIDIA_PROPRIETARY;
/// Alias of [`VK_DRIVER_ID_QUALCOMM_PROPRIETARY`]
pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY_KHR: VkDriverId =
  VK_DRIVER_ID_QUALCOMM_PROPRIETARY;
impl core::fmt::Debug for VkDriverId {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DRIVER_ID_AMD_OPEN_SOURCE => "VK_DRIVER_ID_AMD_OPEN_SOURCE",
      VK_DRIVER_ID_AMD_PROPRIETARY => "VK_DRIVER_ID_AMD_PROPRIETARY",
      VK_DRIVER_ID_ARM_PROPRIETARY => "VK_DRIVER_ID_ARM_PROPRIETARY",
      VK_DRIVER_ID_BROADCOM_PROPRIETARY => "VK_DRIVER_ID_BROADCOM_PROPRIETARY",
      VK_DRIVER_ID_COREAVI_PROPRIETARY => "VK_DRIVER_ID_COREAVI_PROPRIETARY",
      VK_DRIVER_ID_GGP_PROPRIETARY => "VK_DRIVER_ID_GGP_PROPRIETARY",
      VK_DRIVER_ID_GOOGLE_SWIFTSHADER => "VK_DRIVER_ID_GOOGLE_SWIFTSHADER",
      VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA => {
        "VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA"
      }
      VK_DRIVER_ID_IMAGINATION_PROPRIETARY => "VK_DRIVER_ID_IMAGINATION_PROPRIETARY",
      VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA => "VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA",
      VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS => "VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS",
      VK_DRIVER_ID_JUICE_PROPRIETARY => "VK_DRIVER_ID_JUICE_PROPRIETARY",
      VK_DRIVER_ID_MESA_DOZEN => "VK_DRIVER_ID_MESA_DOZEN",
      VK_DRIVER_ID_MESA_LLVMPIPE => "VK_DRIVER_ID_MESA_LLVMPIPE",
      VK_DRIVER_ID_MESA_NVK => "VK_DRIVER_ID_MESA_NVK",
      VK_DRIVER_ID_MESA_PANVK => "VK_DRIVER_ID_MESA_PANVK",
      VK_DRIVER_ID_MESA_RADV => "VK_DRIVER_ID_MESA_RADV",
      VK_DRIVER_ID_MESA_TURNIP => "VK_DRIVER_ID_MESA_TURNIP",
      VK_DRIVER_ID_MESA_V3DV => "VK_DRIVER_ID_MESA_V3DV",
      VK_DRIVER_ID_MESA_VENUS => "VK_DRIVER_ID_MESA_VENUS",
      VK_DRIVER_ID_MOLTENVK => "VK_DRIVER_ID_MOLTENVK",
      VK_DRIVER_ID_NVIDIA_PROPRIETARY => "VK_DRIVER_ID_NVIDIA_PROPRIETARY",
      VK_DRIVER_ID_QUALCOMM_PROPRIETARY => "VK_DRIVER_ID_QUALCOMM_PROPRIETARY",
      VK_DRIVER_ID_SAMSUNG_PROPRIETARY => "VK_DRIVER_ID_SAMSUNG_PROPRIETARY",
      VK_DRIVER_ID_VERISILICON_PROPRIETARY => "VK_DRIVER_ID_VERISILICON_PROPRIETARY",
      other => return write!(f, "VkDriverId({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkDynamicState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDynamicState.html) (enumeration)
  VkDynamicState(u32)
);
pub const VK_DYNAMIC_STATE_ALPHA_TO_COVERAGE_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000455007);
pub const VK_DYNAMIC_STATE_ALPHA_TO_ONE_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000455008);
pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: VkDynamicState = VkDynamicState(4);
pub const VK_DYNAMIC_STATE_COLOR_BLEND_ADVANCED_EXT: VkDynamicState =
  VkDynamicState(1000455018);
pub const VK_DYNAMIC_STATE_COLOR_BLEND_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000455010);
pub const VK_DYNAMIC_STATE_COLOR_BLEND_EQUATION_EXT: VkDynamicState =
  VkDynamicState(1000455011);
pub const VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000381000);
pub const VK_DYNAMIC_STATE_COLOR_WRITE_MASK_EXT: VkDynamicState =
  VkDynamicState(1000455012);
pub const VK_DYNAMIC_STATE_CONSERVATIVE_RASTERIZATION_MODE_EXT: VkDynamicState =
  VkDynamicState(1000455014);
pub const VK_DYNAMIC_STATE_COVERAGE_MODULATION_MODE_NV: VkDynamicState =
  VkDynamicState(1000455027);
pub const VK_DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_ENABLE_NV: VkDynamicState =
  VkDynamicState(1000455028);
pub const VK_DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_NV: VkDynamicState =
  VkDynamicState(1000455029);
pub const VK_DYNAMIC_STATE_COVERAGE_REDUCTION_MODE_NV: VkDynamicState =
  VkDynamicState(1000455032);
pub const VK_DYNAMIC_STATE_COVERAGE_TO_COLOR_ENABLE_NV: VkDynamicState =
  VkDynamicState(1000455025);
pub const VK_DYNAMIC_STATE_COVERAGE_TO_COLOR_LOCATION_NV: VkDynamicState =
  VkDynamicState(1000455026);
pub const VK_DYNAMIC_STATE_CULL_MODE: VkDynamicState = VkDynamicState(1000267000);
pub const VK_DYNAMIC_STATE_DEPTH_BIAS: VkDynamicState = VkDynamicState(3);
pub const VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE: VkDynamicState = VkDynamicState(1000377002);
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: VkDynamicState = VkDynamicState(5);
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE: VkDynamicState =
  VkDynamicState(1000267009);
pub const VK_DYNAMIC_STATE_DEPTH_CLAMP_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000455003);
pub const VK_DYNAMIC_STATE_DEPTH_CLIP_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000455016);
pub const VK_DYNAMIC_STATE_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: VkDynamicState =
  VkDynamicState(1000455022);
pub const VK_DYNAMIC_STATE_DEPTH_COMPARE_OP: VkDynamicState = VkDynamicState(1000267008);
pub const VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE: VkDynamicState = VkDynamicState(1000267006);
pub const VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE: VkDynamicState =
  VkDynamicState(1000267007);
pub const VK_DYNAMIC_STATE_DISCARD_RECTANGLE_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000099001);
pub const VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT: VkDynamicState =
  VkDynamicState(1000099000);
pub const VK_DYNAMIC_STATE_DISCARD_RECTANGLE_MODE_EXT: VkDynamicState =
  VkDynamicState(1000099002);
pub const VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_ENABLE_NV: VkDynamicState =
  VkDynamicState(1000205000);
pub const VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV: VkDynamicState =
  VkDynamicState(1000205001);
pub const VK_DYNAMIC_STATE_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: VkDynamicState =
  VkDynamicState(1000455015);
pub const VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR: VkDynamicState =
  VkDynamicState(1000226000);
pub const VK_DYNAMIC_STATE_FRONT_FACE: VkDynamicState = VkDynamicState(1000267001);
pub const VK_DYNAMIC_STATE_LINE_RASTERIZATION_MODE_EXT: VkDynamicState =
  VkDynamicState(1000455020);
pub const VK_DYNAMIC_STATE_LINE_STIPPLE_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000455021);
pub const VK_DYNAMIC_STATE_LINE_STIPPLE_EXT: VkDynamicState = VkDynamicState(1000259000);
pub const VK_DYNAMIC_STATE_LINE_WIDTH: VkDynamicState = VkDynamicState(2);
pub const VK_DYNAMIC_STATE_LOGIC_OP_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000455009);
/// Not promoted to 1.3
pub const VK_DYNAMIC_STATE_LOGIC_OP_EXT: VkDynamicState = VkDynamicState(1000377003);
/// Not promoted to 1.3
pub const VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT: VkDynamicState =
  VkDynamicState(1000377000);
pub const VK_DYNAMIC_STATE_POLYGON_MODE_EXT: VkDynamicState = VkDynamicState(1000455004);
pub const VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE: VkDynamicState =
  VkDynamicState(1000377004);
pub const VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY: VkDynamicState =
  VkDynamicState(1000267002);
pub const VK_DYNAMIC_STATE_PROVOKING_VERTEX_MODE_EXT: VkDynamicState =
  VkDynamicState(1000455019);
pub const VK_DYNAMIC_STATE_RASTERIZATION_SAMPLES_EXT: VkDynamicState =
  VkDynamicState(1000455005);
pub const VK_DYNAMIC_STATE_RASTERIZATION_STREAM_EXT: VkDynamicState =
  VkDynamicState(1000455013);
pub const VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE: VkDynamicState =
  VkDynamicState(1000377001);
pub const VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR: VkDynamicState =
  VkDynamicState(1000347000);
pub const VK_DYNAMIC_STATE_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: VkDynamicState =
  VkDynamicState(1000455031);
pub const VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_ENABLE_EXT: VkDynamicState =
  VkDynamicState(1000455017);
pub const VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT: VkDynamicState =
  VkDynamicState(1000143000);
pub const VK_DYNAMIC_STATE_SAMPLE_MASK_EXT: VkDynamicState = VkDynamicState(1000455006);
pub const VK_DYNAMIC_STATE_SCISSOR: VkDynamicState = VkDynamicState(1);
pub const VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT: VkDynamicState =
  VkDynamicState(1000267004);
pub const VK_DYNAMIC_STATE_SHADING_RATE_IMAGE_ENABLE_NV: VkDynamicState =
  VkDynamicState(1000455030);
pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: VkDynamicState = VkDynamicState(6);
pub const VK_DYNAMIC_STATE_STENCIL_OP: VkDynamicState = VkDynamicState(1000267011);
pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: VkDynamicState = VkDynamicState(8);
pub const VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE: VkDynamicState =
  VkDynamicState(1000267010);
pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: VkDynamicState = VkDynamicState(7);
pub const VK_DYNAMIC_STATE_TESSELLATION_DOMAIN_ORIGIN_EXT: VkDynamicState =
  VkDynamicState(1000455002);
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE: VkDynamicState =
  VkDynamicState(1000267005);
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_EXT: VkDynamicState = VkDynamicState(1000352000);
pub const VK_DYNAMIC_STATE_VIEWPORT: VkDynamicState = VkDynamicState(0);
pub const VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV: VkDynamicState =
  VkDynamicState(1000164006);
pub const VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV: VkDynamicState =
  VkDynamicState(1000164004);
pub const VK_DYNAMIC_STATE_VIEWPORT_SWIZZLE_NV: VkDynamicState =
  VkDynamicState(1000455024);
pub const VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT: VkDynamicState =
  VkDynamicState(1000267003);
pub const VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_ENABLE_NV: VkDynamicState =
  VkDynamicState(1000455023);
pub const VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV: VkDynamicState =
  VkDynamicState(1000087000);
/// Alias of [`VK_DYNAMIC_STATE_CULL_MODE`]
pub const VK_DYNAMIC_STATE_CULL_MODE_EXT: VkDynamicState = VK_DYNAMIC_STATE_CULL_MODE;
/// Alias of [`VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`]
pub const VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE;
/// Alias of [`VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE`]
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE;
/// Alias of [`VK_DYNAMIC_STATE_DEPTH_COMPARE_OP`]
pub const VK_DYNAMIC_STATE_DEPTH_COMPARE_OP_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_DEPTH_COMPARE_OP;
/// Alias of [`VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE`]
pub const VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE;
/// Alias of [`VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE`]
pub const VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE;
/// Alias of [`VK_DYNAMIC_STATE_FRONT_FACE`]
pub const VK_DYNAMIC_STATE_FRONT_FACE_EXT: VkDynamicState = VK_DYNAMIC_STATE_FRONT_FACE;
/// Alias of [`VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE`]
pub const VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE;
/// Alias of [`VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY`]
pub const VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY;
/// Alias of [`VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE`]
pub const VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE;
/// Alias of [`VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT`]
pub const VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT;
/// Alias of [`VK_DYNAMIC_STATE_STENCIL_OP`]
pub const VK_DYNAMIC_STATE_STENCIL_OP_EXT: VkDynamicState = VK_DYNAMIC_STATE_STENCIL_OP;
/// Alias of [`VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE`]
pub const VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE;
/// Alias of [`VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE`]
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE;
/// Alias of [`VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`]
pub const VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT_EXT: VkDynamicState =
  VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT;
impl core::fmt::Debug for VkDynamicState {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_DYNAMIC_STATE_ALPHA_TO_COVERAGE_ENABLE_EXT => {
        "VK_DYNAMIC_STATE_ALPHA_TO_COVERAGE_ENABLE_EXT"
      }
      VK_DYNAMIC_STATE_ALPHA_TO_ONE_ENABLE_EXT => {
        "VK_DYNAMIC_STATE_ALPHA_TO_ONE_ENABLE_EXT"
      }
      VK_DYNAMIC_STATE_BLEND_CONSTANTS => "VK_DYNAMIC_STATE_BLEND_CONSTANTS",
      VK_DYNAMIC_STATE_COLOR_BLEND_ADVANCED_EXT => {
        "VK_DYNAMIC_STATE_COLOR_BLEND_ADVANCED_EXT"
      }
      VK_DYNAMIC_STATE_COLOR_BLEND_ENABLE_EXT => {
        "VK_DYNAMIC_STATE_COLOR_BLEND_ENABLE_EXT"
      }
      VK_DYNAMIC_STATE_COLOR_BLEND_EQUATION_EXT => {
        "VK_DYNAMIC_STATE_COLOR_BLEND_EQUATION_EXT"
      }
      VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT => {
        "VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT"
      }
      VK_DYNAMIC_STATE_COLOR_WRITE_MASK_EXT => "VK_DYNAMIC_STATE_COLOR_WRITE_MASK_EXT",
      VK_DYNAMIC_STATE_CONSERVATIVE_RASTERIZATION_MODE_EXT => {
        "VK_DYNAMIC_STATE_CONSERVATIVE_RASTERIZATION_MODE_EXT"
      }
      VK_DYNAMIC_STATE_COVERAGE_MODULATION_MODE_NV => {
        "VK_DYNAMIC_STATE_COVERAGE_MODULATION_MODE_NV"
      }
      VK_DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_ENABLE_NV => {
        "VK_DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_ENABLE_NV"
      }
      VK_DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_NV => {
        "VK_DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_NV"
      }
      VK_DYNAMIC_STATE_COVERAGE_REDUCTION_MODE_NV => {
        "VK_DYNAMIC_STATE_COVERAGE_REDUCTION_MODE_NV"
      }
      VK_DYNAMIC_STATE_COVERAGE_TO_COLOR_ENABLE_NV => {
        "VK_DYNAMIC_STATE_COVERAGE_TO_COLOR_ENABLE_NV"
      }
      VK_DYNAMIC_STATE_COVERAGE_TO_COLOR_LOCATION_NV => {
        "VK_DYNAMIC_STATE_COVERAGE_TO_COLOR_LOCATION_NV"
      }
      VK_DYNAMIC_STATE_CULL_MODE => "VK_DYNAMIC_STATE_CULL_MODE",
      VK_DYNAMIC_STATE_DEPTH_BIAS => "VK_DYNAMIC_STATE_DEPTH_BIAS",
      VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE => "VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE",
      VK_DYNAMIC_STATE_DEPTH_BOUNDS => "VK_DYNAMIC_STATE_DEPTH_BOUNDS",
      VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE => {
        "VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE"
      }
      VK_DYNAMIC_STATE_DEPTH_CLAMP_ENABLE_EXT => {
        "VK_DYNAMIC_STATE_DEPTH_CLAMP_ENABLE_EXT"
      }
      VK_DYNAMIC_STATE_DEPTH_CLIP_ENABLE_EXT => "VK_DYNAMIC_STATE_DEPTH_CLIP_ENABLE_EXT",
      VK_DYNAMIC_STATE_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT => {
        "VK_DYNAMIC_STATE_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT"
      }
      VK_DYNAMIC_STATE_DEPTH_COMPARE_OP => "VK_DYNAMIC_STATE_DEPTH_COMPARE_OP",
      VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE => "VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE",
      VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE => "VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE",
      VK_DYNAMIC_STATE_DISCARD_RECTANGLE_ENABLE_EXT => {
        "VK_DYNAMIC_STATE_DISCARD_RECTANGLE_ENABLE_EXT"
      }
      VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT => "VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT",
      VK_DYNAMIC_STATE_DISCARD_RECTANGLE_MODE_EXT => {
        "VK_DYNAMIC_STATE_DISCARD_RECTANGLE_MODE_EXT"
      }
      VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_ENABLE_NV => {
        "VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_ENABLE_NV"
      }
      VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV => "VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV",
      VK_DYNAMIC_STATE_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT => {
        "VK_DYNAMIC_STATE_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT"
      }
      VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR => {
        "VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR"
      }
      VK_DYNAMIC_STATE_FRONT_FACE => "VK_DYNAMIC_STATE_FRONT_FACE",
      VK_DYNAMIC_STATE_LINE_RASTERIZATION_MODE_EXT => {
        "VK_DYNAMIC_STATE_LINE_RASTERIZATION_MODE_EXT"
      }
      VK_DYNAMIC_STATE_LINE_STIPPLE_ENABLE_EXT => {
        "VK_DYNAMIC_STATE_LINE_STIPPLE_ENABLE_EXT"
      }
      VK_DYNAMIC_STATE_LINE_STIPPLE_EXT => "VK_DYNAMIC_STATE_LINE_STIPPLE_EXT",
      VK_DYNAMIC_STATE_LINE_WIDTH => "VK_DYNAMIC_STATE_LINE_WIDTH",
      VK_DYNAMIC_STATE_LOGIC_OP_ENABLE_EXT => "VK_DYNAMIC_STATE_LOGIC_OP_ENABLE_EXT",
      VK_DYNAMIC_STATE_LOGIC_OP_EXT => "VK_DYNAMIC_STATE_LOGIC_OP_EXT",
      VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT => {
        "VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT"
      }
      VK_DYNAMIC_STATE_POLYGON_MODE_EXT => "VK_DYNAMIC_STATE_POLYGON_MODE_EXT",
      VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE => {
        "VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE"
      }
      VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY => "VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY",
      VK_DYNAMIC_STATE_PROVOKING_VERTEX_MODE_EXT => {
        "VK_DYNAMIC_STATE_PROVOKING_VERTEX_MODE_EXT"
      }
      VK_DYNAMIC_STATE_RASTERIZATION_SAMPLES_EXT => {
        "VK_DYNAMIC_STATE_RASTERIZATION_SAMPLES_EXT"
      }
      VK_DYNAMIC_STATE_RASTERIZATION_STREAM_EXT => {
        "VK_DYNAMIC_STATE_RASTERIZATION_STREAM_EXT"
      }
      VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE => {
        "VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE"
      }
      VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR => {
        "VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR"
      }
      VK_DYNAMIC_STATE_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV => {
        "VK_DYNAMIC_STATE_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV"
      }
      VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_ENABLE_EXT => {
        "VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_ENABLE_EXT"
      }
      VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT => "VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT",
      VK_DYNAMIC_STATE_SAMPLE_MASK_EXT => "VK_DYNAMIC_STATE_SAMPLE_MASK_EXT",
      VK_DYNAMIC_STATE_SCISSOR => "VK_DYNAMIC_STATE_SCISSOR",
      VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT => "VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT",
      VK_DYNAMIC_STATE_SHADING_RATE_IMAGE_ENABLE_NV => {
        "VK_DYNAMIC_STATE_SHADING_RATE_IMAGE_ENABLE_NV"
      }
      VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK => "VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK",
      VK_DYNAMIC_STATE_STENCIL_OP => "VK_DYNAMIC_STATE_STENCIL_OP",
      VK_DYNAMIC_STATE_STENCIL_REFERENCE => "VK_DYNAMIC_STATE_STENCIL_REFERENCE",
      VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE => "VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE",
      VK_DYNAMIC_STATE_STENCIL_WRITE_MASK => "VK_DYNAMIC_STATE_STENCIL_WRITE_MASK",
      VK_DYNAMIC_STATE_TESSELLATION_DOMAIN_ORIGIN_EXT => {
        "VK_DYNAMIC_STATE_TESSELLATION_DOMAIN_ORIGIN_EXT"
      }
      VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE => {
        "VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE"
      }
      VK_DYNAMIC_STATE_VERTEX_INPUT_EXT => "VK_DYNAMIC_STATE_VERTEX_INPUT_EXT",
      VK_DYNAMIC_STATE_VIEWPORT => "VK_DYNAMIC_STATE_VIEWPORT",
      VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV => {
        "VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV"
      }
      VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV => {
        "VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV"
      }
      VK_DYNAMIC_STATE_VIEWPORT_SWIZZLE_NV => "VK_DYNAMIC_STATE_VIEWPORT_SWIZZLE_NV",
      VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT => "VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT",
      VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_ENABLE_NV => {
        "VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_ENABLE_NV"
      }
      VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV => "VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV",
      other => return write!(f, "VkDynamicState({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFaultLevel](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFaultLevel.html) (enumeration)
  VkFaultLevel(u32)
);
pub const VK_FAULT_LEVEL_CRITICAL: VkFaultLevel = VkFaultLevel(1);
pub const VK_FAULT_LEVEL_RECOVERABLE: VkFaultLevel = VkFaultLevel(2);
pub const VK_FAULT_LEVEL_UNASSIGNED: VkFaultLevel = VkFaultLevel(0);
pub const VK_FAULT_LEVEL_WARNING: VkFaultLevel = VkFaultLevel(3);
impl core::fmt::Debug for VkFaultLevel {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FAULT_LEVEL_CRITICAL => "VK_FAULT_LEVEL_CRITICAL",
      VK_FAULT_LEVEL_RECOVERABLE => "VK_FAULT_LEVEL_RECOVERABLE",
      VK_FAULT_LEVEL_UNASSIGNED => "VK_FAULT_LEVEL_UNASSIGNED",
      VK_FAULT_LEVEL_WARNING => "VK_FAULT_LEVEL_WARNING",
      other => return write!(f, "VkFaultLevel({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFaultQueryBehavior](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFaultQueryBehavior.html) (enumeration)
  VkFaultQueryBehavior(u32)
);
pub const VK_FAULT_QUERY_BEHAVIOR_GET_AND_CLEAR_ALL_FAULTS: VkFaultQueryBehavior =
  VkFaultQueryBehavior(0);
impl core::fmt::Debug for VkFaultQueryBehavior {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FAULT_QUERY_BEHAVIOR_GET_AND_CLEAR_ALL_FAULTS => {
        "VK_FAULT_QUERY_BEHAVIOR_GET_AND_CLEAR_ALL_FAULTS"
      }
      other => return write!(f, "VkFaultQueryBehavior({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFaultType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFaultType.html) (enumeration)
  VkFaultType(u32)
);
pub const VK_FAULT_TYPE_COMMAND_BUFFER_FULL: VkFaultType = VkFaultType(5);
pub const VK_FAULT_TYPE_IMPLEMENTATION: VkFaultType = VkFaultType(2);
pub const VK_FAULT_TYPE_INVALID: VkFaultType = VkFaultType(0);
pub const VK_FAULT_TYPE_INVALID_API_USAGE: VkFaultType = VkFaultType(6);
pub const VK_FAULT_TYPE_PHYSICAL_DEVICE: VkFaultType = VkFaultType(4);
pub const VK_FAULT_TYPE_SYSTEM: VkFaultType = VkFaultType(3);
pub const VK_FAULT_TYPE_UNASSIGNED: VkFaultType = VkFaultType(1);
impl core::fmt::Debug for VkFaultType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FAULT_TYPE_COMMAND_BUFFER_FULL => "VK_FAULT_TYPE_COMMAND_BUFFER_FULL",
      VK_FAULT_TYPE_IMPLEMENTATION => "VK_FAULT_TYPE_IMPLEMENTATION",
      VK_FAULT_TYPE_INVALID => "VK_FAULT_TYPE_INVALID",
      VK_FAULT_TYPE_INVALID_API_USAGE => "VK_FAULT_TYPE_INVALID_API_USAGE",
      VK_FAULT_TYPE_PHYSICAL_DEVICE => "VK_FAULT_TYPE_PHYSICAL_DEVICE",
      VK_FAULT_TYPE_SYSTEM => "VK_FAULT_TYPE_SYSTEM",
      VK_FAULT_TYPE_UNASSIGNED => "VK_FAULT_TYPE_UNASSIGNED",
      other => return write!(f, "VkFaultType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFilter](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFilter.html) (enumeration)
  VkFilter(u32)
);
pub const VK_FILTER_CUBIC_EXT: VkFilter = VkFilter(1000015000);
pub const VK_FILTER_LINEAR: VkFilter = VkFilter(1);
pub const VK_FILTER_NEAREST: VkFilter = VkFilter(0);
/// Alias of [`VK_FILTER_CUBIC_EXT`]
pub const VK_FILTER_CUBIC_IMG: VkFilter = VK_FILTER_CUBIC_EXT;
impl core::fmt::Debug for VkFilter {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FILTER_CUBIC_EXT => "VK_FILTER_CUBIC_EXT",
      VK_FILTER_LINEAR => "VK_FILTER_LINEAR",
      VK_FILTER_NEAREST => "VK_FILTER_NEAREST",
      other => return write!(f, "VkFilter({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFormat](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormat.html) (enumeration)
  VkFormat(u32)
);
pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: VkFormat = VkFormat(8);
pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: VkFormat = VkFormat(69);
pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: VkFormat = VkFormat(65);
pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: VkFormat = VkFormat(67);
pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: VkFormat = VkFormat(68);
pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: VkFormat = VkFormat(64);
pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: VkFormat = VkFormat(66);
pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: VkFormat = VkFormat(63);
pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: VkFormat = VkFormat(59);
pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: VkFormat = VkFormat(61);
pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: VkFormat = VkFormat(62);
pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: VkFormat = VkFormat(58);
pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: VkFormat = VkFormat(60);
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16: VkFormat = VkFormat(1000340001);
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16: VkFormat = VkFormat(1000340000);
pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: VkFormat = VkFormat(56);
pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: VkFormat = VkFormat(52);
pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: VkFormat = VkFormat(57);
pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: VkFormat = VkFormat(54);
pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: VkFormat = VkFormat(55);
pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: VkFormat = VkFormat(51);
pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: VkFormat = VkFormat(53);
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK: VkFormat = VkFormat(1000066011);
pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: VkFormat = VkFormat(180);
pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: VkFormat = VkFormat(179);
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK: VkFormat = VkFormat(1000066008);
pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: VkFormat = VkFormat(174);
pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: VkFormat = VkFormat(173);
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK: VkFormat = VkFormat(1000066009);
pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: VkFormat = VkFormat(176);
pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: VkFormat = VkFormat(175);
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK: VkFormat = VkFormat(1000066010);
pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: VkFormat = VkFormat(178);
pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: VkFormat = VkFormat(177);
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK: VkFormat = VkFormat(1000066012);
pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: VkFormat = VkFormat(182);
pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: VkFormat = VkFormat(181);
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK: VkFormat = VkFormat(1000066013);
pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: VkFormat = VkFormat(184);
pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: VkFormat = VkFormat(183);
pub const VK_FORMAT_ASTC_3x3x3_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288002);
pub const VK_FORMAT_ASTC_3x3x3_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288001);
pub const VK_FORMAT_ASTC_3x3x3_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288000);
pub const VK_FORMAT_ASTC_4x3x3_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288005);
pub const VK_FORMAT_ASTC_4x3x3_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288004);
pub const VK_FORMAT_ASTC_4x3x3_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288003);
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK: VkFormat = VkFormat(1000066000);
pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: VkFormat = VkFormat(158);
pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: VkFormat = VkFormat(157);
pub const VK_FORMAT_ASTC_4x4x3_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288008);
pub const VK_FORMAT_ASTC_4x4x3_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288007);
pub const VK_FORMAT_ASTC_4x4x3_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288006);
pub const VK_FORMAT_ASTC_4x4x4_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288011);
pub const VK_FORMAT_ASTC_4x4x4_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288010);
pub const VK_FORMAT_ASTC_4x4x4_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288009);
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK: VkFormat = VkFormat(1000066001);
pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: VkFormat = VkFormat(160);
pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: VkFormat = VkFormat(159);
pub const VK_FORMAT_ASTC_5x4x4_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288014);
pub const VK_FORMAT_ASTC_5x4x4_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288013);
pub const VK_FORMAT_ASTC_5x4x4_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288012);
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK: VkFormat = VkFormat(1000066002);
pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: VkFormat = VkFormat(162);
pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: VkFormat = VkFormat(161);
pub const VK_FORMAT_ASTC_5x5x4_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288017);
pub const VK_FORMAT_ASTC_5x5x4_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288016);
pub const VK_FORMAT_ASTC_5x5x4_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288015);
pub const VK_FORMAT_ASTC_5x5x5_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288020);
pub const VK_FORMAT_ASTC_5x5x5_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288019);
pub const VK_FORMAT_ASTC_5x5x5_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288018);
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK: VkFormat = VkFormat(1000066003);
pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: VkFormat = VkFormat(164);
pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: VkFormat = VkFormat(163);
pub const VK_FORMAT_ASTC_6x5x5_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288023);
pub const VK_FORMAT_ASTC_6x5x5_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288022);
pub const VK_FORMAT_ASTC_6x5x5_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288021);
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK: VkFormat = VkFormat(1000066004);
pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: VkFormat = VkFormat(166);
pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: VkFormat = VkFormat(165);
pub const VK_FORMAT_ASTC_6x6x5_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288026);
pub const VK_FORMAT_ASTC_6x6x5_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288025);
pub const VK_FORMAT_ASTC_6x6x5_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288024);
pub const VK_FORMAT_ASTC_6x6x6_SFLOAT_BLOCK_EXT: VkFormat = VkFormat(1000288029);
pub const VK_FORMAT_ASTC_6x6x6_SRGB_BLOCK_EXT: VkFormat = VkFormat(1000288028);
pub const VK_FORMAT_ASTC_6x6x6_UNORM_BLOCK_EXT: VkFormat = VkFormat(1000288027);
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK: VkFormat = VkFormat(1000066005);
pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: VkFormat = VkFormat(168);
pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: VkFormat = VkFormat(167);
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK: VkFormat = VkFormat(1000066006);
pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: VkFormat = VkFormat(170);
pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: VkFormat = VkFormat(169);
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK: VkFormat = VkFormat(1000066007);
pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: VkFormat = VkFormat(172);
pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: VkFormat = VkFormat(171);
pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: VkFormat = VkFormat(122);
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: VkFormat =
  VkFormat(1000156011);
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: VkFormat =
  VkFormat(1000156021);
pub const VK_FORMAT_B16G16R16G16_422_UNORM: VkFormat = VkFormat(1000156028);
pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: VkFormat = VkFormat(3);
pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: VkFormat = VkFormat(7);
pub const VK_FORMAT_B5G6R5_UNORM_PACK16: VkFormat = VkFormat(5);
pub const VK_FORMAT_B8G8R8A8_SINT: VkFormat = VkFormat(49);
pub const VK_FORMAT_B8G8R8A8_SNORM: VkFormat = VkFormat(45);
pub const VK_FORMAT_B8G8R8A8_SRGB: VkFormat = VkFormat(50);
pub const VK_FORMAT_B8G8R8A8_SSCALED: VkFormat = VkFormat(47);
pub const VK_FORMAT_B8G8R8A8_UINT: VkFormat = VkFormat(48);
pub const VK_FORMAT_B8G8R8A8_UNORM: VkFormat = VkFormat(44);
pub const VK_FORMAT_B8G8R8A8_USCALED: VkFormat = VkFormat(46);
pub const VK_FORMAT_B8G8R8G8_422_UNORM: VkFormat = VkFormat(1000156001);
pub const VK_FORMAT_B8G8R8_SINT: VkFormat = VkFormat(35);
pub const VK_FORMAT_B8G8R8_SNORM: VkFormat = VkFormat(31);
pub const VK_FORMAT_B8G8R8_SRGB: VkFormat = VkFormat(36);
pub const VK_FORMAT_B8G8R8_SSCALED: VkFormat = VkFormat(33);
pub const VK_FORMAT_B8G8R8_UINT: VkFormat = VkFormat(34);
pub const VK_FORMAT_B8G8R8_UNORM: VkFormat = VkFormat(30);
pub const VK_FORMAT_B8G8R8_USCALED: VkFormat = VkFormat(32);
pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: VkFormat = VkFormat(134);
pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: VkFormat = VkFormat(133);
pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: VkFormat = VkFormat(132);
pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: VkFormat = VkFormat(131);
pub const VK_FORMAT_BC2_SRGB_BLOCK: VkFormat = VkFormat(136);
pub const VK_FORMAT_BC2_UNORM_BLOCK: VkFormat = VkFormat(135);
pub const VK_FORMAT_BC3_SRGB_BLOCK: VkFormat = VkFormat(138);
pub const VK_FORMAT_BC3_UNORM_BLOCK: VkFormat = VkFormat(137);
pub const VK_FORMAT_BC4_SNORM_BLOCK: VkFormat = VkFormat(140);
pub const VK_FORMAT_BC4_UNORM_BLOCK: VkFormat = VkFormat(139);
pub const VK_FORMAT_BC5_SNORM_BLOCK: VkFormat = VkFormat(142);
pub const VK_FORMAT_BC5_UNORM_BLOCK: VkFormat = VkFormat(141);
pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: VkFormat = VkFormat(144);
pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: VkFormat = VkFormat(143);
pub const VK_FORMAT_BC7_SRGB_BLOCK: VkFormat = VkFormat(146);
pub const VK_FORMAT_BC7_UNORM_BLOCK: VkFormat = VkFormat(145);
pub const VK_FORMAT_D16_UNORM: VkFormat = VkFormat(124);
pub const VK_FORMAT_D16_UNORM_S8_UINT: VkFormat = VkFormat(128);
pub const VK_FORMAT_D24_UNORM_S8_UINT: VkFormat = VkFormat(129);
pub const VK_FORMAT_D32_SFLOAT: VkFormat = VkFormat(126);
pub const VK_FORMAT_D32_SFLOAT_S8_UINT: VkFormat = VkFormat(130);
pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: VkFormat = VkFormat(123);
pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: VkFormat = VkFormat(156);
pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: VkFormat = VkFormat(155);
pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: VkFormat = VkFormat(154);
pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: VkFormat = VkFormat(153);
pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: VkFormat = VkFormat(150);
pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: VkFormat = VkFormat(149);
pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: VkFormat = VkFormat(152);
pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: VkFormat = VkFormat(151);
pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: VkFormat = VkFormat(148);
pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: VkFormat = VkFormat(147);
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: VkFormat =
  VkFormat(1000156010);
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: VkFormat =
  VkFormat(1000156013);
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: VkFormat =
  VkFormat(1000156015);
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: VkFormat =
  VkFormat(1000330001);
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: VkFormat =
  VkFormat(1000156012);
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: VkFormat =
  VkFormat(1000156014);
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: VkFormat =
  VkFormat(1000156016);
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: VkFormat =
  VkFormat(1000156020);
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: VkFormat =
  VkFormat(1000156023);
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: VkFormat =
  VkFormat(1000156025);
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: VkFormat =
  VkFormat(1000330002);
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: VkFormat =
  VkFormat(1000156022);
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: VkFormat =
  VkFormat(1000156024);
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: VkFormat =
  VkFormat(1000156026);
pub const VK_FORMAT_G16B16G16R16_422_UNORM: VkFormat = VkFormat(1000156027);
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: VkFormat = VkFormat(1000156030);
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: VkFormat = VkFormat(1000156032);
pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM: VkFormat = VkFormat(1000330003);
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: VkFormat = VkFormat(1000156029);
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: VkFormat = VkFormat(1000156031);
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: VkFormat = VkFormat(1000156033);
pub const VK_FORMAT_G8B8G8R8_422_UNORM: VkFormat = VkFormat(1000156000);
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: VkFormat = VkFormat(1000156003);
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: VkFormat = VkFormat(1000156005);
pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM: VkFormat = VkFormat(1000330000);
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: VkFormat = VkFormat(1000156002);
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: VkFormat = VkFormat(1000156004);
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: VkFormat = VkFormat(1000156006);
pub const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG: VkFormat = VkFormat(1000054004);
pub const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG: VkFormat = VkFormat(1000054000);
pub const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG: VkFormat = VkFormat(1000054005);
pub const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG: VkFormat = VkFormat(1000054001);
pub const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG: VkFormat = VkFormat(1000054006);
pub const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG: VkFormat = VkFormat(1000054002);
pub const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG: VkFormat = VkFormat(1000054007);
pub const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG: VkFormat = VkFormat(1000054003);
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: VkFormat = VkFormat(1000156009);
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: VkFormat = VkFormat(1000156008);
pub const VK_FORMAT_R10X6_UNORM_PACK16: VkFormat = VkFormat(1000156007);
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: VkFormat = VkFormat(1000156019);
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: VkFormat = VkFormat(1000156018);
pub const VK_FORMAT_R12X4_UNORM_PACK16: VkFormat = VkFormat(1000156017);
pub const VK_FORMAT_R16G16B16A16_SFLOAT: VkFormat = VkFormat(97);
pub const VK_FORMAT_R16G16B16A16_SINT: VkFormat = VkFormat(96);
pub const VK_FORMAT_R16G16B16A16_SNORM: VkFormat = VkFormat(92);
pub const VK_FORMAT_R16G16B16A16_SSCALED: VkFormat = VkFormat(94);
pub const VK_FORMAT_R16G16B16A16_UINT: VkFormat = VkFormat(95);
pub const VK_FORMAT_R16G16B16A16_UNORM: VkFormat = VkFormat(91);
pub const VK_FORMAT_R16G16B16A16_USCALED: VkFormat = VkFormat(93);
pub const VK_FORMAT_R16G16B16_SFLOAT: VkFormat = VkFormat(90);
pub const VK_FORMAT_R16G16B16_SINT: VkFormat = VkFormat(89);
pub const VK_FORMAT_R16G16B16_SNORM: VkFormat = VkFormat(85);
pub const VK_FORMAT_R16G16B16_SSCALED: VkFormat = VkFormat(87);
pub const VK_FORMAT_R16G16B16_UINT: VkFormat = VkFormat(88);
pub const VK_FORMAT_R16G16B16_UNORM: VkFormat = VkFormat(84);
pub const VK_FORMAT_R16G16B16_USCALED: VkFormat = VkFormat(86);
pub const VK_FORMAT_R16G16_S10_5_NV: VkFormat = VkFormat(1000464000);
pub const VK_FORMAT_R16G16_SFLOAT: VkFormat = VkFormat(83);
pub const VK_FORMAT_R16G16_SINT: VkFormat = VkFormat(82);
pub const VK_FORMAT_R16G16_SNORM: VkFormat = VkFormat(78);
pub const VK_FORMAT_R16G16_SSCALED: VkFormat = VkFormat(80);
pub const VK_FORMAT_R16G16_UINT: VkFormat = VkFormat(81);
pub const VK_FORMAT_R16G16_UNORM: VkFormat = VkFormat(77);
pub const VK_FORMAT_R16G16_USCALED: VkFormat = VkFormat(79);
pub const VK_FORMAT_R16_SFLOAT: VkFormat = VkFormat(76);
pub const VK_FORMAT_R16_SINT: VkFormat = VkFormat(75);
pub const VK_FORMAT_R16_SNORM: VkFormat = VkFormat(71);
pub const VK_FORMAT_R16_SSCALED: VkFormat = VkFormat(73);
pub const VK_FORMAT_R16_UINT: VkFormat = VkFormat(74);
pub const VK_FORMAT_R16_UNORM: VkFormat = VkFormat(70);
pub const VK_FORMAT_R16_USCALED: VkFormat = VkFormat(72);
pub const VK_FORMAT_R32G32B32A32_SFLOAT: VkFormat = VkFormat(109);
pub const VK_FORMAT_R32G32B32A32_SINT: VkFormat = VkFormat(108);
pub const VK_FORMAT_R32G32B32A32_UINT: VkFormat = VkFormat(107);
pub const VK_FORMAT_R32G32B32_SFLOAT: VkFormat = VkFormat(106);
pub const VK_FORMAT_R32G32B32_SINT: VkFormat = VkFormat(105);
pub const VK_FORMAT_R32G32B32_UINT: VkFormat = VkFormat(104);
pub const VK_FORMAT_R32G32_SFLOAT: VkFormat = VkFormat(103);
pub const VK_FORMAT_R32G32_SINT: VkFormat = VkFormat(102);
pub const VK_FORMAT_R32G32_UINT: VkFormat = VkFormat(101);
pub const VK_FORMAT_R32_SFLOAT: VkFormat = VkFormat(100);
pub const VK_FORMAT_R32_SINT: VkFormat = VkFormat(99);
pub const VK_FORMAT_R32_UINT: VkFormat = VkFormat(98);
pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: VkFormat = VkFormat(2);
pub const VK_FORMAT_R4G4_UNORM_PACK8: VkFormat = VkFormat(1);
pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: VkFormat = VkFormat(6);
pub const VK_FORMAT_R5G6B5_UNORM_PACK16: VkFormat = VkFormat(4);
pub const VK_FORMAT_R64G64B64A64_SFLOAT: VkFormat = VkFormat(121);
pub const VK_FORMAT_R64G64B64A64_SINT: VkFormat = VkFormat(120);
pub const VK_FORMAT_R64G64B64A64_UINT: VkFormat = VkFormat(119);
pub const VK_FORMAT_R64G64B64_SFLOAT: VkFormat = VkFormat(118);
pub const VK_FORMAT_R64G64B64_SINT: VkFormat = VkFormat(117);
pub const VK_FORMAT_R64G64B64_UINT: VkFormat = VkFormat(116);
pub const VK_FORMAT_R64G64_SFLOAT: VkFormat = VkFormat(115);
pub const VK_FORMAT_R64G64_SINT: VkFormat = VkFormat(114);
pub const VK_FORMAT_R64G64_UINT: VkFormat = VkFormat(113);
pub const VK_FORMAT_R64_SFLOAT: VkFormat = VkFormat(112);
pub const VK_FORMAT_R64_SINT: VkFormat = VkFormat(111);
pub const VK_FORMAT_R64_UINT: VkFormat = VkFormat(110);
pub const VK_FORMAT_R8G8B8A8_SINT: VkFormat = VkFormat(42);
pub const VK_FORMAT_R8G8B8A8_SNORM: VkFormat = VkFormat(38);
pub const VK_FORMAT_R8G8B8A8_SRGB: VkFormat = VkFormat(43);
pub const VK_FORMAT_R8G8B8A8_SSCALED: VkFormat = VkFormat(40);
pub const VK_FORMAT_R8G8B8A8_UINT: VkFormat = VkFormat(41);
pub const VK_FORMAT_R8G8B8A8_UNORM: VkFormat = VkFormat(37);
pub const VK_FORMAT_R8G8B8A8_USCALED: VkFormat = VkFormat(39);
pub const VK_FORMAT_R8G8B8_SINT: VkFormat = VkFormat(28);
pub const VK_FORMAT_R8G8B8_SNORM: VkFormat = VkFormat(24);
pub const VK_FORMAT_R8G8B8_SRGB: VkFormat = VkFormat(29);
pub const VK_FORMAT_R8G8B8_SSCALED: VkFormat = VkFormat(26);
pub const VK_FORMAT_R8G8B8_UINT: VkFormat = VkFormat(27);
pub const VK_FORMAT_R8G8B8_UNORM: VkFormat = VkFormat(23);
pub const VK_FORMAT_R8G8B8_USCALED: VkFormat = VkFormat(25);
pub const VK_FORMAT_R8G8_SINT: VkFormat = VkFormat(21);
pub const VK_FORMAT_R8G8_SNORM: VkFormat = VkFormat(17);
pub const VK_FORMAT_R8G8_SRGB: VkFormat = VkFormat(22);
pub const VK_FORMAT_R8G8_SSCALED: VkFormat = VkFormat(19);
pub const VK_FORMAT_R8G8_UINT: VkFormat = VkFormat(20);
pub const VK_FORMAT_R8G8_UNORM: VkFormat = VkFormat(16);
pub const VK_FORMAT_R8G8_USCALED: VkFormat = VkFormat(18);
pub const VK_FORMAT_R8_SINT: VkFormat = VkFormat(14);
pub const VK_FORMAT_R8_SNORM: VkFormat = VkFormat(10);
pub const VK_FORMAT_R8_SRGB: VkFormat = VkFormat(15);
pub const VK_FORMAT_R8_SSCALED: VkFormat = VkFormat(12);
pub const VK_FORMAT_R8_UINT: VkFormat = VkFormat(13);
pub const VK_FORMAT_R8_UNORM: VkFormat = VkFormat(9);
pub const VK_FORMAT_R8_USCALED: VkFormat = VkFormat(11);
pub const VK_FORMAT_S8_UINT: VkFormat = VkFormat(127);
pub const VK_FORMAT_UNDEFINED: VkFormat = VkFormat(0);
pub const VK_FORMAT_X8_D24_UNORM_PACK32: VkFormat = VkFormat(125);
/// Alias of [`VK_FORMAT_A4B4G4R4_UNORM_PACK16`]
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT: VkFormat = VK_FORMAT_A4B4G4R4_UNORM_PACK16;
/// Alias of [`VK_FORMAT_A4R4G4B4_UNORM_PACK16`]
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT: VkFormat = VK_FORMAT_A4R4G4B4_UNORM_PACK16;
/// Alias of [`VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK_EXT: VkFormat =
  VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK_EXT: VkFormat =
  VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK_EXT: VkFormat =
  VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK_EXT: VkFormat =
  VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK_EXT: VkFormat =
  VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK_EXT: VkFormat =
  VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT: VkFormat = VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK_EXT: VkFormat = VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK_EXT: VkFormat = VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK_EXT: VkFormat = VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK_EXT: VkFormat = VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK_EXT: VkFormat = VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK_EXT: VkFormat = VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`]
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK_EXT: VkFormat = VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK;
/// Alias of [`VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16`]
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: VkFormat =
  VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
/// Alias of [`VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16`]
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: VkFormat =
  VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
/// Alias of [`VK_FORMAT_B16G16R16G16_422_UNORM`]
pub const VK_FORMAT_B16G16R16G16_422_UNORM_KHR: VkFormat =
  VK_FORMAT_B16G16R16G16_422_UNORM;
/// Alias of [`VK_FORMAT_B8G8R8G8_422_UNORM`]
pub const VK_FORMAT_B8G8R8G8_422_UNORM_KHR: VkFormat = VK_FORMAT_B8G8R8G8_422_UNORM;
/// Alias of [`VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16`]
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: VkFormat =
  VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
/// Alias of [`VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16`]
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16`]
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16`]
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: VkFormat =
  VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16`]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16`]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16`]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16`]
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: VkFormat =
  VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
/// Alias of [`VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16`]
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16`]
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16`]
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: VkFormat =
  VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16`]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16`]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16`]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: VkFormat =
  VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
/// Alias of [`VK_FORMAT_G16B16G16R16_422_UNORM`]
pub const VK_FORMAT_G16B16G16R16_422_UNORM_KHR: VkFormat =
  VK_FORMAT_G16B16G16R16_422_UNORM;
/// Alias of [`VK_FORMAT_G16_B16R16_2PLANE_420_UNORM`]
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR: VkFormat =
  VK_FORMAT_G16_B16R16_2PLANE_420_UNORM;
/// Alias of [`VK_FORMAT_G16_B16R16_2PLANE_422_UNORM`]
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR: VkFormat =
  VK_FORMAT_G16_B16R16_2PLANE_422_UNORM;
/// Alias of [`VK_FORMAT_G16_B16R16_2PLANE_444_UNORM`]
pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM_EXT: VkFormat =
  VK_FORMAT_G16_B16R16_2PLANE_444_UNORM;
/// Alias of [`VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM`]
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR: VkFormat =
  VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM;
/// Alias of [`VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM`]
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR: VkFormat =
  VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM;
/// Alias of [`VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM`]
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR: VkFormat =
  VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM;
/// Alias of [`VK_FORMAT_G8B8G8R8_422_UNORM`]
pub const VK_FORMAT_G8B8G8R8_422_UNORM_KHR: VkFormat = VK_FORMAT_G8B8G8R8_422_UNORM;
/// Alias of [`VK_FORMAT_G8_B8R8_2PLANE_420_UNORM`]
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR: VkFormat =
  VK_FORMAT_G8_B8R8_2PLANE_420_UNORM;
/// Alias of [`VK_FORMAT_G8_B8R8_2PLANE_422_UNORM`]
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR: VkFormat =
  VK_FORMAT_G8_B8R8_2PLANE_422_UNORM;
/// Alias of [`VK_FORMAT_G8_B8R8_2PLANE_444_UNORM`]
pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM_EXT: VkFormat =
  VK_FORMAT_G8_B8R8_2PLANE_444_UNORM;
/// Alias of [`VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM`]
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR: VkFormat =
  VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM;
/// Alias of [`VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM`]
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR: VkFormat =
  VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM;
/// Alias of [`VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM`]
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR: VkFormat =
  VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM;
/// Alias of [`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16`]
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: VkFormat =
  VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16;
/// Alias of [`VK_FORMAT_R10X6G10X6_UNORM_2PACK16`]
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR: VkFormat =
  VK_FORMAT_R10X6G10X6_UNORM_2PACK16;
/// Alias of [`VK_FORMAT_R10X6_UNORM_PACK16`]
pub const VK_FORMAT_R10X6_UNORM_PACK16_KHR: VkFormat = VK_FORMAT_R10X6_UNORM_PACK16;
/// Alias of [`VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16`]
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: VkFormat =
  VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16;
/// Alias of [`VK_FORMAT_R12X4G12X4_UNORM_2PACK16`]
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR: VkFormat =
  VK_FORMAT_R12X4G12X4_UNORM_2PACK16;
/// Alias of [`VK_FORMAT_R12X4_UNORM_PACK16`]
pub const VK_FORMAT_R12X4_UNORM_PACK16_KHR: VkFormat = VK_FORMAT_R12X4_UNORM_PACK16;
impl core::fmt::Debug for VkFormat {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FORMAT_A1R5G5B5_UNORM_PACK16 => "VK_FORMAT_A1R5G5B5_UNORM_PACK16",
      VK_FORMAT_A2B10G10R10_SINT_PACK32 => "VK_FORMAT_A2B10G10R10_SINT_PACK32",
      VK_FORMAT_A2B10G10R10_SNORM_PACK32 => "VK_FORMAT_A2B10G10R10_SNORM_PACK32",
      VK_FORMAT_A2B10G10R10_SSCALED_PACK32 => "VK_FORMAT_A2B10G10R10_SSCALED_PACK32",
      VK_FORMAT_A2B10G10R10_UINT_PACK32 => "VK_FORMAT_A2B10G10R10_UINT_PACK32",
      VK_FORMAT_A2B10G10R10_UNORM_PACK32 => "VK_FORMAT_A2B10G10R10_UNORM_PACK32",
      VK_FORMAT_A2B10G10R10_USCALED_PACK32 => "VK_FORMAT_A2B10G10R10_USCALED_PACK32",
      VK_FORMAT_A2R10G10B10_SINT_PACK32 => "VK_FORMAT_A2R10G10B10_SINT_PACK32",
      VK_FORMAT_A2R10G10B10_SNORM_PACK32 => "VK_FORMAT_A2R10G10B10_SNORM_PACK32",
      VK_FORMAT_A2R10G10B10_SSCALED_PACK32 => "VK_FORMAT_A2R10G10B10_SSCALED_PACK32",
      VK_FORMAT_A2R10G10B10_UINT_PACK32 => "VK_FORMAT_A2R10G10B10_UINT_PACK32",
      VK_FORMAT_A2R10G10B10_UNORM_PACK32 => "VK_FORMAT_A2R10G10B10_UNORM_PACK32",
      VK_FORMAT_A2R10G10B10_USCALED_PACK32 => "VK_FORMAT_A2R10G10B10_USCALED_PACK32",
      VK_FORMAT_A4B4G4R4_UNORM_PACK16 => "VK_FORMAT_A4B4G4R4_UNORM_PACK16",
      VK_FORMAT_A4R4G4B4_UNORM_PACK16 => "VK_FORMAT_A4R4G4B4_UNORM_PACK16",
      VK_FORMAT_A8B8G8R8_SINT_PACK32 => "VK_FORMAT_A8B8G8R8_SINT_PACK32",
      VK_FORMAT_A8B8G8R8_SNORM_PACK32 => "VK_FORMAT_A8B8G8R8_SNORM_PACK32",
      VK_FORMAT_A8B8G8R8_SRGB_PACK32 => "VK_FORMAT_A8B8G8R8_SRGB_PACK32",
      VK_FORMAT_A8B8G8R8_SSCALED_PACK32 => "VK_FORMAT_A8B8G8R8_SSCALED_PACK32",
      VK_FORMAT_A8B8G8R8_UINT_PACK32 => "VK_FORMAT_A8B8G8R8_UINT_PACK32",
      VK_FORMAT_A8B8G8R8_UNORM_PACK32 => "VK_FORMAT_A8B8G8R8_UNORM_PACK32",
      VK_FORMAT_A8B8G8R8_USCALED_PACK32 => "VK_FORMAT_A8B8G8R8_USCALED_PACK32",
      VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK => "VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_10x10_SRGB_BLOCK => "VK_FORMAT_ASTC_10x10_SRGB_BLOCK",
      VK_FORMAT_ASTC_10x10_UNORM_BLOCK => "VK_FORMAT_ASTC_10x10_UNORM_BLOCK",
      VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK => "VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_10x5_SRGB_BLOCK => "VK_FORMAT_ASTC_10x5_SRGB_BLOCK",
      VK_FORMAT_ASTC_10x5_UNORM_BLOCK => "VK_FORMAT_ASTC_10x5_UNORM_BLOCK",
      VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK => "VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_10x6_SRGB_BLOCK => "VK_FORMAT_ASTC_10x6_SRGB_BLOCK",
      VK_FORMAT_ASTC_10x6_UNORM_BLOCK => "VK_FORMAT_ASTC_10x6_UNORM_BLOCK",
      VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK => "VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_10x8_SRGB_BLOCK => "VK_FORMAT_ASTC_10x8_SRGB_BLOCK",
      VK_FORMAT_ASTC_10x8_UNORM_BLOCK => "VK_FORMAT_ASTC_10x8_UNORM_BLOCK",
      VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK => "VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_12x10_SRGB_BLOCK => "VK_FORMAT_ASTC_12x10_SRGB_BLOCK",
      VK_FORMAT_ASTC_12x10_UNORM_BLOCK => "VK_FORMAT_ASTC_12x10_UNORM_BLOCK",
      VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK => "VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_12x12_SRGB_BLOCK => "VK_FORMAT_ASTC_12x12_SRGB_BLOCK",
      VK_FORMAT_ASTC_12x12_UNORM_BLOCK => "VK_FORMAT_ASTC_12x12_UNORM_BLOCK",
      VK_FORMAT_ASTC_3x3x3_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_3x3x3_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_3x3x3_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_3x3x3_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_3x3x3_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_3x3x3_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_4x3x3_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_4x3x3_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_4x3x3_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_4x3x3_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_4x3x3_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_4x3x3_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK => "VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_4x4_SRGB_BLOCK => "VK_FORMAT_ASTC_4x4_SRGB_BLOCK",
      VK_FORMAT_ASTC_4x4_UNORM_BLOCK => "VK_FORMAT_ASTC_4x4_UNORM_BLOCK",
      VK_FORMAT_ASTC_4x4x3_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_4x4x3_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_4x4x3_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_4x4x3_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_4x4x3_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_4x4x3_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_4x4x4_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_4x4x4_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_4x4x4_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_4x4x4_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_4x4x4_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_4x4x4_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK => "VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_5x4_SRGB_BLOCK => "VK_FORMAT_ASTC_5x4_SRGB_BLOCK",
      VK_FORMAT_ASTC_5x4_UNORM_BLOCK => "VK_FORMAT_ASTC_5x4_UNORM_BLOCK",
      VK_FORMAT_ASTC_5x4x4_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_5x4x4_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_5x4x4_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_5x4x4_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_5x4x4_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_5x4x4_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK => "VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_5x5_SRGB_BLOCK => "VK_FORMAT_ASTC_5x5_SRGB_BLOCK",
      VK_FORMAT_ASTC_5x5_UNORM_BLOCK => "VK_FORMAT_ASTC_5x5_UNORM_BLOCK",
      VK_FORMAT_ASTC_5x5x4_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_5x5x4_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_5x5x4_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_5x5x4_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_5x5x4_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_5x5x4_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_5x5x5_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_5x5x5_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_5x5x5_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_5x5x5_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_5x5x5_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_5x5x5_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK => "VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_6x5_SRGB_BLOCK => "VK_FORMAT_ASTC_6x5_SRGB_BLOCK",
      VK_FORMAT_ASTC_6x5_UNORM_BLOCK => "VK_FORMAT_ASTC_6x5_UNORM_BLOCK",
      VK_FORMAT_ASTC_6x5x5_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_6x5x5_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_6x5x5_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_6x5x5_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_6x5x5_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_6x5x5_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK => "VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_6x6_SRGB_BLOCK => "VK_FORMAT_ASTC_6x6_SRGB_BLOCK",
      VK_FORMAT_ASTC_6x6_UNORM_BLOCK => "VK_FORMAT_ASTC_6x6_UNORM_BLOCK",
      VK_FORMAT_ASTC_6x6x5_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_6x6x5_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_6x6x5_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_6x6x5_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_6x6x5_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_6x6x5_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_6x6x6_SFLOAT_BLOCK_EXT => "VK_FORMAT_ASTC_6x6x6_SFLOAT_BLOCK_EXT",
      VK_FORMAT_ASTC_6x6x6_SRGB_BLOCK_EXT => "VK_FORMAT_ASTC_6x6x6_SRGB_BLOCK_EXT",
      VK_FORMAT_ASTC_6x6x6_UNORM_BLOCK_EXT => "VK_FORMAT_ASTC_6x6x6_UNORM_BLOCK_EXT",
      VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK => "VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_8x5_SRGB_BLOCK => "VK_FORMAT_ASTC_8x5_SRGB_BLOCK",
      VK_FORMAT_ASTC_8x5_UNORM_BLOCK => "VK_FORMAT_ASTC_8x5_UNORM_BLOCK",
      VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK => "VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_8x6_SRGB_BLOCK => "VK_FORMAT_ASTC_8x6_SRGB_BLOCK",
      VK_FORMAT_ASTC_8x6_UNORM_BLOCK => "VK_FORMAT_ASTC_8x6_UNORM_BLOCK",
      VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK => "VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK",
      VK_FORMAT_ASTC_8x8_SRGB_BLOCK => "VK_FORMAT_ASTC_8x8_SRGB_BLOCK",
      VK_FORMAT_ASTC_8x8_UNORM_BLOCK => "VK_FORMAT_ASTC_8x8_UNORM_BLOCK",
      VK_FORMAT_B10G11R11_UFLOAT_PACK32 => "VK_FORMAT_B10G11R11_UFLOAT_PACK32",
      VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
        "VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16"
      }
      VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
        "VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16"
      }
      VK_FORMAT_B16G16R16G16_422_UNORM => "VK_FORMAT_B16G16R16G16_422_UNORM",
      VK_FORMAT_B4G4R4A4_UNORM_PACK16 => "VK_FORMAT_B4G4R4A4_UNORM_PACK16",
      VK_FORMAT_B5G5R5A1_UNORM_PACK16 => "VK_FORMAT_B5G5R5A1_UNORM_PACK16",
      VK_FORMAT_B5G6R5_UNORM_PACK16 => "VK_FORMAT_B5G6R5_UNORM_PACK16",
      VK_FORMAT_B8G8R8A8_SINT => "VK_FORMAT_B8G8R8A8_SINT",
      VK_FORMAT_B8G8R8A8_SNORM => "VK_FORMAT_B8G8R8A8_SNORM",
      VK_FORMAT_B8G8R8A8_SRGB => "VK_FORMAT_B8G8R8A8_SRGB",
      VK_FORMAT_B8G8R8A8_SSCALED => "VK_FORMAT_B8G8R8A8_SSCALED",
      VK_FORMAT_B8G8R8A8_UINT => "VK_FORMAT_B8G8R8A8_UINT",
      VK_FORMAT_B8G8R8A8_UNORM => "VK_FORMAT_B8G8R8A8_UNORM",
      VK_FORMAT_B8G8R8A8_USCALED => "VK_FORMAT_B8G8R8A8_USCALED",
      VK_FORMAT_B8G8R8G8_422_UNORM => "VK_FORMAT_B8G8R8G8_422_UNORM",
      VK_FORMAT_B8G8R8_SINT => "VK_FORMAT_B8G8R8_SINT",
      VK_FORMAT_B8G8R8_SNORM => "VK_FORMAT_B8G8R8_SNORM",
      VK_FORMAT_B8G8R8_SRGB => "VK_FORMAT_B8G8R8_SRGB",
      VK_FORMAT_B8G8R8_SSCALED => "VK_FORMAT_B8G8R8_SSCALED",
      VK_FORMAT_B8G8R8_UINT => "VK_FORMAT_B8G8R8_UINT",
      VK_FORMAT_B8G8R8_UNORM => "VK_FORMAT_B8G8R8_UNORM",
      VK_FORMAT_B8G8R8_USCALED => "VK_FORMAT_B8G8R8_USCALED",
      VK_FORMAT_BC1_RGBA_SRGB_BLOCK => "VK_FORMAT_BC1_RGBA_SRGB_BLOCK",
      VK_FORMAT_BC1_RGBA_UNORM_BLOCK => "VK_FORMAT_BC1_RGBA_UNORM_BLOCK",
      VK_FORMAT_BC1_RGB_SRGB_BLOCK => "VK_FORMAT_BC1_RGB_SRGB_BLOCK",
      VK_FORMAT_BC1_RGB_UNORM_BLOCK => "VK_FORMAT_BC1_RGB_UNORM_BLOCK",
      VK_FORMAT_BC2_SRGB_BLOCK => "VK_FORMAT_BC2_SRGB_BLOCK",
      VK_FORMAT_BC2_UNORM_BLOCK => "VK_FORMAT_BC2_UNORM_BLOCK",
      VK_FORMAT_BC3_SRGB_BLOCK => "VK_FORMAT_BC3_SRGB_BLOCK",
      VK_FORMAT_BC3_UNORM_BLOCK => "VK_FORMAT_BC3_UNORM_BLOCK",
      VK_FORMAT_BC4_SNORM_BLOCK => "VK_FORMAT_BC4_SNORM_BLOCK",
      VK_FORMAT_BC4_UNORM_BLOCK => "VK_FORMAT_BC4_UNORM_BLOCK",
      VK_FORMAT_BC5_SNORM_BLOCK => "VK_FORMAT_BC5_SNORM_BLOCK",
      VK_FORMAT_BC5_UNORM_BLOCK => "VK_FORMAT_BC5_UNORM_BLOCK",
      VK_FORMAT_BC6H_SFLOAT_BLOCK => "VK_FORMAT_BC6H_SFLOAT_BLOCK",
      VK_FORMAT_BC6H_UFLOAT_BLOCK => "VK_FORMAT_BC6H_UFLOAT_BLOCK",
      VK_FORMAT_BC7_SRGB_BLOCK => "VK_FORMAT_BC7_SRGB_BLOCK",
      VK_FORMAT_BC7_UNORM_BLOCK => "VK_FORMAT_BC7_UNORM_BLOCK",
      VK_FORMAT_D16_UNORM => "VK_FORMAT_D16_UNORM",
      VK_FORMAT_D16_UNORM_S8_UINT => "VK_FORMAT_D16_UNORM_S8_UINT",
      VK_FORMAT_D24_UNORM_S8_UINT => "VK_FORMAT_D24_UNORM_S8_UINT",
      VK_FORMAT_D32_SFLOAT => "VK_FORMAT_D32_SFLOAT",
      VK_FORMAT_D32_SFLOAT_S8_UINT => "VK_FORMAT_D32_SFLOAT_S8_UINT",
      VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => "VK_FORMAT_E5B9G9R9_UFLOAT_PACK32",
      VK_FORMAT_EAC_R11G11_SNORM_BLOCK => "VK_FORMAT_EAC_R11G11_SNORM_BLOCK",
      VK_FORMAT_EAC_R11G11_UNORM_BLOCK => "VK_FORMAT_EAC_R11G11_UNORM_BLOCK",
      VK_FORMAT_EAC_R11_SNORM_BLOCK => "VK_FORMAT_EAC_R11_SNORM_BLOCK",
      VK_FORMAT_EAC_R11_UNORM_BLOCK => "VK_FORMAT_EAC_R11_UNORM_BLOCK",
      VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK => "VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK",
      VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK => "VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK",
      VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK => "VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK",
      VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK => "VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK",
      VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK => "VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK",
      VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK => "VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK",
      VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
        "VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16"
      }
      VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
        "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16"
      }
      VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
        "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16"
      }
      VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => {
        "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16"
      }
      VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
        "VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16"
      }
      VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
        "VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16"
      }
      VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
        "VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16"
      }
      VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
        "VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16"
      }
      VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
        "VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16"
      }
      VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
        "VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16"
      }
      VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => {
        "VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16"
      }
      VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
        "VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16"
      }
      VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
        "VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16"
      }
      VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
        "VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16"
      }
      VK_FORMAT_G16B16G16R16_422_UNORM => "VK_FORMAT_G16B16G16R16_422_UNORM",
      VK_FORMAT_G16_B16R16_2PLANE_420_UNORM => "VK_FORMAT_G16_B16R16_2PLANE_420_UNORM",
      VK_FORMAT_G16_B16R16_2PLANE_422_UNORM => "VK_FORMAT_G16_B16R16_2PLANE_422_UNORM",
      VK_FORMAT_G16_B16R16_2PLANE_444_UNORM => "VK_FORMAT_G16_B16R16_2PLANE_444_UNORM",
      VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM => "VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM",
      VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM => "VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM",
      VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM => "VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM",
      VK_FORMAT_G8B8G8R8_422_UNORM => "VK_FORMAT_G8B8G8R8_422_UNORM",
      VK_FORMAT_G8_B8R8_2PLANE_420_UNORM => "VK_FORMAT_G8_B8R8_2PLANE_420_UNORM",
      VK_FORMAT_G8_B8R8_2PLANE_422_UNORM => "VK_FORMAT_G8_B8R8_2PLANE_422_UNORM",
      VK_FORMAT_G8_B8R8_2PLANE_444_UNORM => "VK_FORMAT_G8_B8R8_2PLANE_444_UNORM",
      VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM => "VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM",
      VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM => "VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM",
      VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM => "VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM",
      VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG => "VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG",
      VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG => "VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG",
      VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG => "VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG",
      VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG => "VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG",
      VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG => "VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG",
      VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG => "VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG",
      VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG => "VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG",
      VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG => "VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG",
      VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16 => {
        "VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16"
      }
      VK_FORMAT_R10X6G10X6_UNORM_2PACK16 => "VK_FORMAT_R10X6G10X6_UNORM_2PACK16",
      VK_FORMAT_R10X6_UNORM_PACK16 => "VK_FORMAT_R10X6_UNORM_PACK16",
      VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16 => {
        "VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16"
      }
      VK_FORMAT_R12X4G12X4_UNORM_2PACK16 => "VK_FORMAT_R12X4G12X4_UNORM_2PACK16",
      VK_FORMAT_R12X4_UNORM_PACK16 => "VK_FORMAT_R12X4_UNORM_PACK16",
      VK_FORMAT_R16G16B16A16_SFLOAT => "VK_FORMAT_R16G16B16A16_SFLOAT",
      VK_FORMAT_R16G16B16A16_SINT => "VK_FORMAT_R16G16B16A16_SINT",
      VK_FORMAT_R16G16B16A16_SNORM => "VK_FORMAT_R16G16B16A16_SNORM",
      VK_FORMAT_R16G16B16A16_SSCALED => "VK_FORMAT_R16G16B16A16_SSCALED",
      VK_FORMAT_R16G16B16A16_UINT => "VK_FORMAT_R16G16B16A16_UINT",
      VK_FORMAT_R16G16B16A16_UNORM => "VK_FORMAT_R16G16B16A16_UNORM",
      VK_FORMAT_R16G16B16A16_USCALED => "VK_FORMAT_R16G16B16A16_USCALED",
      VK_FORMAT_R16G16B16_SFLOAT => "VK_FORMAT_R16G16B16_SFLOAT",
      VK_FORMAT_R16G16B16_SINT => "VK_FORMAT_R16G16B16_SINT",
      VK_FORMAT_R16G16B16_SNORM => "VK_FORMAT_R16G16B16_SNORM",
      VK_FORMAT_R16G16B16_SSCALED => "VK_FORMAT_R16G16B16_SSCALED",
      VK_FORMAT_R16G16B16_UINT => "VK_FORMAT_R16G16B16_UINT",
      VK_FORMAT_R16G16B16_UNORM => "VK_FORMAT_R16G16B16_UNORM",
      VK_FORMAT_R16G16B16_USCALED => "VK_FORMAT_R16G16B16_USCALED",
      VK_FORMAT_R16G16_S10_5_NV => "VK_FORMAT_R16G16_S10_5_NV",
      VK_FORMAT_R16G16_SFLOAT => "VK_FORMAT_R16G16_SFLOAT",
      VK_FORMAT_R16G16_SINT => "VK_FORMAT_R16G16_SINT",
      VK_FORMAT_R16G16_SNORM => "VK_FORMAT_R16G16_SNORM",
      VK_FORMAT_R16G16_SSCALED => "VK_FORMAT_R16G16_SSCALED",
      VK_FORMAT_R16G16_UINT => "VK_FORMAT_R16G16_UINT",
      VK_FORMAT_R16G16_UNORM => "VK_FORMAT_R16G16_UNORM",
      VK_FORMAT_R16G16_USCALED => "VK_FORMAT_R16G16_USCALED",
      VK_FORMAT_R16_SFLOAT => "VK_FORMAT_R16_SFLOAT",
      VK_FORMAT_R16_SINT => "VK_FORMAT_R16_SINT",
      VK_FORMAT_R16_SNORM => "VK_FORMAT_R16_SNORM",
      VK_FORMAT_R16_SSCALED => "VK_FORMAT_R16_SSCALED",
      VK_FORMAT_R16_UINT => "VK_FORMAT_R16_UINT",
      VK_FORMAT_R16_UNORM => "VK_FORMAT_R16_UNORM",
      VK_FORMAT_R16_USCALED => "VK_FORMAT_R16_USCALED",
      VK_FORMAT_R32G32B32A32_SFLOAT => "VK_FORMAT_R32G32B32A32_SFLOAT",
      VK_FORMAT_R32G32B32A32_SINT => "VK_FORMAT_R32G32B32A32_SINT",
      VK_FORMAT_R32G32B32A32_UINT => "VK_FORMAT_R32G32B32A32_UINT",
      VK_FORMAT_R32G32B32_SFLOAT => "VK_FORMAT_R32G32B32_SFLOAT",
      VK_FORMAT_R32G32B32_SINT => "VK_FORMAT_R32G32B32_SINT",
      VK_FORMAT_R32G32B32_UINT => "VK_FORMAT_R32G32B32_UINT",
      VK_FORMAT_R32G32_SFLOAT => "VK_FORMAT_R32G32_SFLOAT",
      VK_FORMAT_R32G32_SINT => "VK_FORMAT_R32G32_SINT",
      VK_FORMAT_R32G32_UINT => "VK_FORMAT_R32G32_UINT",
      VK_FORMAT_R32_SFLOAT => "VK_FORMAT_R32_SFLOAT",
      VK_FORMAT_R32_SINT => "VK_FORMAT_R32_SINT",
      VK_FORMAT_R32_UINT => "VK_FORMAT_R32_UINT",
      VK_FORMAT_R4G4B4A4_UNORM_PACK16 => "VK_FORMAT_R4G4B4A4_UNORM_PACK16",
      VK_FORMAT_R4G4_UNORM_PACK8 => "VK_FORMAT_R4G4_UNORM_PACK8",
      VK_FORMAT_R5G5B5A1_UNORM_PACK16 => "VK_FORMAT_R5G5B5A1_UNORM_PACK16",
      VK_FORMAT_R5G6B5_UNORM_PACK16 => "VK_FORMAT_R5G6B5_UNORM_PACK16",
      VK_FORMAT_R64G64B64A64_SFLOAT => "VK_FORMAT_R64G64B64A64_SFLOAT",
      VK_FORMAT_R64G64B64A64_SINT => "VK_FORMAT_R64G64B64A64_SINT",
      VK_FORMAT_R64G64B64A64_UINT => "VK_FORMAT_R64G64B64A64_UINT",
      VK_FORMAT_R64G64B64_SFLOAT => "VK_FORMAT_R64G64B64_SFLOAT",
      VK_FORMAT_R64G64B64_SINT => "VK_FORMAT_R64G64B64_SINT",
      VK_FORMAT_R64G64B64_UINT => "VK_FORMAT_R64G64B64_UINT",
      VK_FORMAT_R64G64_SFLOAT => "VK_FORMAT_R64G64_SFLOAT",
      VK_FORMAT_R64G64_SINT => "VK_FORMAT_R64G64_SINT",
      VK_FORMAT_R64G64_UINT => "VK_FORMAT_R64G64_UINT",
      VK_FORMAT_R64_SFLOAT => "VK_FORMAT_R64_SFLOAT",
      VK_FORMAT_R64_SINT => "VK_FORMAT_R64_SINT",
      VK_FORMAT_R64_UINT => "VK_FORMAT_R64_UINT",
      VK_FORMAT_R8G8B8A8_SINT => "VK_FORMAT_R8G8B8A8_SINT",
      VK_FORMAT_R8G8B8A8_SNORM => "VK_FORMAT_R8G8B8A8_SNORM",
      VK_FORMAT_R8G8B8A8_SRGB => "VK_FORMAT_R8G8B8A8_SRGB",
      VK_FORMAT_R8G8B8A8_SSCALED => "VK_FORMAT_R8G8B8A8_SSCALED",
      VK_FORMAT_R8G8B8A8_UINT => "VK_FORMAT_R8G8B8A8_UINT",
      VK_FORMAT_R8G8B8A8_UNORM => "VK_FORMAT_R8G8B8A8_UNORM",
      VK_FORMAT_R8G8B8A8_USCALED => "VK_FORMAT_R8G8B8A8_USCALED",
      VK_FORMAT_R8G8B8_SINT => "VK_FORMAT_R8G8B8_SINT",
      VK_FORMAT_R8G8B8_SNORM => "VK_FORMAT_R8G8B8_SNORM",
      VK_FORMAT_R8G8B8_SRGB => "VK_FORMAT_R8G8B8_SRGB",
      VK_FORMAT_R8G8B8_SSCALED => "VK_FORMAT_R8G8B8_SSCALED",
      VK_FORMAT_R8G8B8_UINT => "VK_FORMAT_R8G8B8_UINT",
      VK_FORMAT_R8G8B8_UNORM => "VK_FORMAT_R8G8B8_UNORM",
      VK_FORMAT_R8G8B8_USCALED => "VK_FORMAT_R8G8B8_USCALED",
      VK_FORMAT_R8G8_SINT => "VK_FORMAT_R8G8_SINT",
      VK_FORMAT_R8G8_SNORM => "VK_FORMAT_R8G8_SNORM",
      VK_FORMAT_R8G8_SRGB => "VK_FORMAT_R8G8_SRGB",
      VK_FORMAT_R8G8_SSCALED => "VK_FORMAT_R8G8_SSCALED",
      VK_FORMAT_R8G8_UINT => "VK_FORMAT_R8G8_UINT",
      VK_FORMAT_R8G8_UNORM => "VK_FORMAT_R8G8_UNORM",
      VK_FORMAT_R8G8_USCALED => "VK_FORMAT_R8G8_USCALED",
      VK_FORMAT_R8_SINT => "VK_FORMAT_R8_SINT",
      VK_FORMAT_R8_SNORM => "VK_FORMAT_R8_SNORM",
      VK_FORMAT_R8_SRGB => "VK_FORMAT_R8_SRGB",
      VK_FORMAT_R8_SSCALED => "VK_FORMAT_R8_SSCALED",
      VK_FORMAT_R8_UINT => "VK_FORMAT_R8_UINT",
      VK_FORMAT_R8_UNORM => "VK_FORMAT_R8_UNORM",
      VK_FORMAT_R8_USCALED => "VK_FORMAT_R8_USCALED",
      VK_FORMAT_S8_UINT => "VK_FORMAT_S8_UINT",
      VK_FORMAT_UNDEFINED => "VK_FORMAT_UNDEFINED",
      VK_FORMAT_X8_D24_UNORM_PACK32 => "VK_FORMAT_X8_D24_UNORM_PACK32",
      other => return write!(f, "VkFormat({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFragmentShadingRateCombinerOpKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateCombinerOpKHR.html) (enumeration)
  VkFragmentShadingRateCombinerOpKHR(u32)
);
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR:
  VkFragmentShadingRateCombinerOpKHR = VkFragmentShadingRateCombinerOpKHR(0);
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR:
  VkFragmentShadingRateCombinerOpKHR = VkFragmentShadingRateCombinerOpKHR(3);
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR:
  VkFragmentShadingRateCombinerOpKHR = VkFragmentShadingRateCombinerOpKHR(2);
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR:
  VkFragmentShadingRateCombinerOpKHR = VkFragmentShadingRateCombinerOpKHR(4);
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR:
  VkFragmentShadingRateCombinerOpKHR = VkFragmentShadingRateCombinerOpKHR(1);
impl core::fmt::Debug for VkFragmentShadingRateCombinerOpKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR => {
        "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR"
      }
      VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR => {
        "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR"
      }
      VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR => {
        "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR"
      }
      VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR => {
        "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR"
      }
      VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR => {
        "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR"
      }
      other => return write!(f, "VkFragmentShadingRateCombinerOpKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFragmentShadingRateNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateNV.html) (enumeration)
  VkFragmentShadingRateNV(u32)
);
pub const VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV =
  VkFragmentShadingRateNV(14);
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV:
  VkFragmentShadingRateNV = VkFragmentShadingRateNV(1);
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV:
  VkFragmentShadingRateNV = VkFragmentShadingRateNV(4);
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV:
  VkFragmentShadingRateNV = VkFragmentShadingRateNV(5);
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV:
  VkFragmentShadingRateNV = VkFragmentShadingRateNV(6);
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV:
  VkFragmentShadingRateNV = VkFragmentShadingRateNV(9);
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV:
  VkFragmentShadingRateNV = VkFragmentShadingRateNV(10);
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: VkFragmentShadingRateNV =
  VkFragmentShadingRateNV(0);
pub const VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV =
  VkFragmentShadingRateNV(11);
pub const VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV =
  VkFragmentShadingRateNV(12);
pub const VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV =
  VkFragmentShadingRateNV(13);
pub const VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV: VkFragmentShadingRateNV =
  VkFragmentShadingRateNV(15);
impl core::fmt::Debug for VkFragmentShadingRateNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV => {
        "VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV"
      }
      VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV => {
        "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV"
      }
      VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV => {
        "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV"
      }
      VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV => {
        "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV"
      }
      VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV => {
        "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV"
      }
      VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV => {
        "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV"
      }
      VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV => {
        "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV"
      }
      VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV => {
        "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV"
      }
      VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV => {
        "VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV"
      }
      VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV => {
        "VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV"
      }
      VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV => {
        "VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV"
      }
      VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV => {
        "VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV"
      }
      other => return write!(f, "VkFragmentShadingRateNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFragmentShadingRateTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateTypeNV.html) (enumeration)
  VkFragmentShadingRateTypeNV(u32)
);
pub const VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV: VkFragmentShadingRateTypeNV =
  VkFragmentShadingRateTypeNV(1);
pub const VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV: VkFragmentShadingRateTypeNV =
  VkFragmentShadingRateTypeNV(0);
impl core::fmt::Debug for VkFragmentShadingRateTypeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV => "VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV",
      VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV => {
        "VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV"
      }
      other => return write!(f, "VkFragmentShadingRateTypeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFrontFace](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFrontFace.html) (enumeration)
  VkFrontFace(u32)
);
pub const VK_FRONT_FACE_CLOCKWISE: VkFrontFace = VkFrontFace(1);
pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: VkFrontFace = VkFrontFace(0);
impl core::fmt::Debug for VkFrontFace {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FRONT_FACE_CLOCKWISE => "VK_FRONT_FACE_CLOCKWISE",
      VK_FRONT_FACE_COUNTER_CLOCKWISE => "VK_FRONT_FACE_COUNTER_CLOCKWISE",
      other => return write!(f, "VkFrontFace({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkFullScreenExclusiveEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFullScreenExclusiveEXT.html) (enumeration)
  VkFullScreenExclusiveEXT(u32)
);
pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: VkFullScreenExclusiveEXT =
  VkFullScreenExclusiveEXT(1);
pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: VkFullScreenExclusiveEXT =
  VkFullScreenExclusiveEXT(3);
pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: VkFullScreenExclusiveEXT =
  VkFullScreenExclusiveEXT(0);
pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: VkFullScreenExclusiveEXT =
  VkFullScreenExclusiveEXT(2);
impl core::fmt::Debug for VkFullScreenExclusiveEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT => "VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT",
      VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT => {
        "VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT"
      }
      VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT => "VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT",
      VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT => {
        "VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT"
      }
      other => return write!(f, "VkFullScreenExclusiveEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkGeometryTypeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryTypeKHR.html) (enumeration)
  VkGeometryTypeKHR(u32)
);
pub const VK_GEOMETRY_TYPE_AABBS_KHR: VkGeometryTypeKHR = VkGeometryTypeKHR(1);
pub const VK_GEOMETRY_TYPE_INSTANCES_KHR: VkGeometryTypeKHR = VkGeometryTypeKHR(2);
pub const VK_GEOMETRY_TYPE_TRIANGLES_KHR: VkGeometryTypeKHR = VkGeometryTypeKHR(0);
/// Alias of [`VK_GEOMETRY_TYPE_AABBS_KHR`]
pub const VK_GEOMETRY_TYPE_AABBS_NV: VkGeometryTypeKHR = VK_GEOMETRY_TYPE_AABBS_KHR;
/// Alias of [`VK_GEOMETRY_TYPE_TRIANGLES_KHR`]
pub const VK_GEOMETRY_TYPE_TRIANGLES_NV: VkGeometryTypeKHR =
  VK_GEOMETRY_TYPE_TRIANGLES_KHR;
impl core::fmt::Debug for VkGeometryTypeKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_GEOMETRY_TYPE_AABBS_KHR => "VK_GEOMETRY_TYPE_AABBS_KHR",
      VK_GEOMETRY_TYPE_INSTANCES_KHR => "VK_GEOMETRY_TYPE_INSTANCES_KHR",
      VK_GEOMETRY_TYPE_TRIANGLES_KHR => "VK_GEOMETRY_TYPE_TRIANGLES_KHR",
      other => return write!(f, "VkGeometryTypeKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkImageLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageLayout.html) (enumeration)
  VkImageLayout(u32)
);
pub const VK_IMAGE_LAYOUT_ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT: VkImageLayout =
  VkImageLayout(1000339000);
pub const VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL: VkImageLayout = VkImageLayout(1000314001);
/// Optimal layout when image is only used for color attachment read/write
pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: VkImageLayout = VkImageLayout(2);
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL: VkImageLayout =
  VkImageLayout(1000241000);
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout =
  VkImageLayout(1000117001);
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL: VkImageLayout =
  VkImageLayout(1000241001);
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout =
  VkImageLayout(1000117000);
/// Optimal layout when image is only used for depth/stencil attachment
/// read/write
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout =
  VkImageLayout(3);
/// Optimal layout when image is used for read only depth/stencil attachment and
/// shader access
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout =
  VkImageLayout(4);
pub const VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: VkImageLayout =
  VkImageLayout(1000218000);
pub const VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: VkImageLayout =
  VkImageLayout(1000164003);
/// General layout when image can be used for any kind of access
pub const VK_IMAGE_LAYOUT_GENERAL: VkImageLayout = VkImageLayout(1);
/// Initial layout used when the data is populated by the CPU
pub const VK_IMAGE_LAYOUT_PREINITIALIZED: VkImageLayout = VkImageLayout(8);
pub const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR: VkImageLayout = VkImageLayout(1000001002);
pub const VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL: VkImageLayout = VkImageLayout(1000314000);
/// Optimal layout when image is used for read only shader access
pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: VkImageLayout = VkImageLayout(5);
pub const VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR: VkImageLayout = VkImageLayout(1000111000);
pub const VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout =
  VkImageLayout(1000241002);
pub const VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout =
  VkImageLayout(1000241003);
/// Optimal layout when image is used only as destination of transfer operations
pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: VkImageLayout = VkImageLayout(7);
/// Optimal layout when image is used only as source of transfer operations
pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: VkImageLayout = VkImageLayout(6);
/// Implicit layout an image is when its contents are undefined due to various
/// reasons (e.g. right after creation)
pub const VK_IMAGE_LAYOUT_UNDEFINED: VkImageLayout = VkImageLayout(0);
pub const VK_IMAGE_LAYOUT_VIDEO_DECODE_DPB_KHR: VkImageLayout = VkImageLayout(1000024002);
pub const VK_IMAGE_LAYOUT_VIDEO_DECODE_DST_KHR: VkImageLayout = VkImageLayout(1000024000);
pub const VK_IMAGE_LAYOUT_VIDEO_DECODE_SRC_KHR: VkImageLayout = VkImageLayout(1000024001);
pub const VK_IMAGE_LAYOUT_VIDEO_ENCODE_DPB_KHR: VkImageLayout = VkImageLayout(1000299002);
pub const VK_IMAGE_LAYOUT_VIDEO_ENCODE_DST_KHR: VkImageLayout = VkImageLayout(1000299000);
pub const VK_IMAGE_LAYOUT_VIDEO_ENCODE_SRC_KHR: VkImageLayout = VkImageLayout(1000299001);
/// Alias of [`VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL`]
pub const VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR: VkImageLayout =
  VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL;
/// Alias of [`VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`]
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR: VkImageLayout =
  VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL;
/// Alias of [`VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`]
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: VkImageLayout =
  VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL;
/// Alias of [`VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`]
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR: VkImageLayout =
  VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL;
/// Alias of [`VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`]
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: VkImageLayout =
  VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL;
/// Alias of [`VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL`]
pub const VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR: VkImageLayout =
  VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL;
/// Alias of [`VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`]
pub const VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV: VkImageLayout =
  VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR;
/// Alias of [`VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`]
pub const VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL_KHR: VkImageLayout =
  VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL;
/// Alias of [`VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`]
pub const VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL_KHR: VkImageLayout =
  VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL;
impl core::fmt::Debug for VkImageLayout {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_IMAGE_LAYOUT_ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT => {
        "VK_IMAGE_LAYOUT_ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT"
      }
      VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL => "VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL",
      VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => {
        "VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL => {
        "VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL => {
        "VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL => {
        "VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL => {
        "VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
        "VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL => {
        "VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT => {
        "VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT"
      }
      VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR => {
        "VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR"
      }
      VK_IMAGE_LAYOUT_GENERAL => "VK_IMAGE_LAYOUT_GENERAL",
      VK_IMAGE_LAYOUT_PREINITIALIZED => "VK_IMAGE_LAYOUT_PREINITIALIZED",
      VK_IMAGE_LAYOUT_PRESENT_SRC_KHR => "VK_IMAGE_LAYOUT_PRESENT_SRC_KHR",
      VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL => "VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL",
      VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => {
        "VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR => "VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR",
      VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL => {
        "VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL => {
        "VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL"
      }
      VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => "VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL",
      VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => "VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL",
      VK_IMAGE_LAYOUT_UNDEFINED => "VK_IMAGE_LAYOUT_UNDEFINED",
      VK_IMAGE_LAYOUT_VIDEO_DECODE_DPB_KHR => "VK_IMAGE_LAYOUT_VIDEO_DECODE_DPB_KHR",
      VK_IMAGE_LAYOUT_VIDEO_DECODE_DST_KHR => "VK_IMAGE_LAYOUT_VIDEO_DECODE_DST_KHR",
      VK_IMAGE_LAYOUT_VIDEO_DECODE_SRC_KHR => "VK_IMAGE_LAYOUT_VIDEO_DECODE_SRC_KHR",
      VK_IMAGE_LAYOUT_VIDEO_ENCODE_DPB_KHR => "VK_IMAGE_LAYOUT_VIDEO_ENCODE_DPB_KHR",
      VK_IMAGE_LAYOUT_VIDEO_ENCODE_DST_KHR => "VK_IMAGE_LAYOUT_VIDEO_ENCODE_DST_KHR",
      VK_IMAGE_LAYOUT_VIDEO_ENCODE_SRC_KHR => "VK_IMAGE_LAYOUT_VIDEO_ENCODE_SRC_KHR",
      other => return write!(f, "VkImageLayout({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkImageTiling](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageTiling.html) (enumeration)
  VkImageTiling(u32)
);
pub const VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT: VkImageTiling =
  VkImageTiling(1000158000);
pub const VK_IMAGE_TILING_LINEAR: VkImageTiling = VkImageTiling(1);
pub const VK_IMAGE_TILING_OPTIMAL: VkImageTiling = VkImageTiling(0);
impl core::fmt::Debug for VkImageTiling {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT => {
        "VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT"
      }
      VK_IMAGE_TILING_LINEAR => "VK_IMAGE_TILING_LINEAR",
      VK_IMAGE_TILING_OPTIMAL => "VK_IMAGE_TILING_OPTIMAL",
      other => return write!(f, "VkImageTiling({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkImageType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageType.html) (enumeration)
  VkImageType(u32)
);
pub const VK_IMAGE_TYPE_1D: VkImageType = VkImageType(0);
pub const VK_IMAGE_TYPE_2D: VkImageType = VkImageType(1);
pub const VK_IMAGE_TYPE_3D: VkImageType = VkImageType(2);
impl core::fmt::Debug for VkImageType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_IMAGE_TYPE_1D => "VK_IMAGE_TYPE_1D",
      VK_IMAGE_TYPE_2D => "VK_IMAGE_TYPE_2D",
      VK_IMAGE_TYPE_3D => "VK_IMAGE_TYPE_3D",
      other => return write!(f, "VkImageType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkImageViewType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewType.html) (enumeration)
  VkImageViewType(u32)
);
pub const VK_IMAGE_VIEW_TYPE_1D: VkImageViewType = VkImageViewType(0);
pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: VkImageViewType = VkImageViewType(4);
pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = VkImageViewType(1);
pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: VkImageViewType = VkImageViewType(5);
pub const VK_IMAGE_VIEW_TYPE_3D: VkImageViewType = VkImageViewType(2);
pub const VK_IMAGE_VIEW_TYPE_CUBE: VkImageViewType = VkImageViewType(3);
pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: VkImageViewType = VkImageViewType(6);
impl core::fmt::Debug for VkImageViewType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_IMAGE_VIEW_TYPE_1D => "VK_IMAGE_VIEW_TYPE_1D",
      VK_IMAGE_VIEW_TYPE_1D_ARRAY => "VK_IMAGE_VIEW_TYPE_1D_ARRAY",
      VK_IMAGE_VIEW_TYPE_2D => "VK_IMAGE_VIEW_TYPE_2D",
      VK_IMAGE_VIEW_TYPE_2D_ARRAY => "VK_IMAGE_VIEW_TYPE_2D_ARRAY",
      VK_IMAGE_VIEW_TYPE_3D => "VK_IMAGE_VIEW_TYPE_3D",
      VK_IMAGE_VIEW_TYPE_CUBE => "VK_IMAGE_VIEW_TYPE_CUBE",
      VK_IMAGE_VIEW_TYPE_CUBE_ARRAY => "VK_IMAGE_VIEW_TYPE_CUBE_ARRAY",
      other => return write!(f, "VkImageViewType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkIndexType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndexType.html) (enumeration)
  VkIndexType(u32)
);
pub const VK_INDEX_TYPE_NONE_KHR: VkIndexType = VkIndexType(1000165000);
pub const VK_INDEX_TYPE_UINT16: VkIndexType = VkIndexType(0);
pub const VK_INDEX_TYPE_UINT32: VkIndexType = VkIndexType(1);
pub const VK_INDEX_TYPE_UINT8_EXT: VkIndexType = VkIndexType(1000265000);
/// Alias of [`VK_INDEX_TYPE_NONE_KHR`]
pub const VK_INDEX_TYPE_NONE_NV: VkIndexType = VK_INDEX_TYPE_NONE_KHR;
impl core::fmt::Debug for VkIndexType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_INDEX_TYPE_NONE_KHR => "VK_INDEX_TYPE_NONE_KHR",
      VK_INDEX_TYPE_UINT16 => "VK_INDEX_TYPE_UINT16",
      VK_INDEX_TYPE_UINT32 => "VK_INDEX_TYPE_UINT32",
      VK_INDEX_TYPE_UINT8_EXT => "VK_INDEX_TYPE_UINT8_EXT",
      other => return write!(f, "VkIndexType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkIndirectCommandsTokenTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsTokenTypeNV.html) (enumeration)
  VkIndirectCommandsTokenTypeNV(u32)
);
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV: VkIndirectCommandsTokenTypeNV =
  VkIndirectCommandsTokenTypeNV(5);
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_MESH_TASKS_NV:
  VkIndirectCommandsTokenTypeNV = VkIndirectCommandsTokenTypeNV(1000328000);
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV: VkIndirectCommandsTokenTypeNV =
  VkIndirectCommandsTokenTypeNV(6);
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV: VkIndirectCommandsTokenTypeNV =
  VkIndirectCommandsTokenTypeNV(7);
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV: VkIndirectCommandsTokenTypeNV =
  VkIndirectCommandsTokenTypeNV(2);
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV:
  VkIndirectCommandsTokenTypeNV = VkIndirectCommandsTokenTypeNV(4);
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV: VkIndirectCommandsTokenTypeNV =
  VkIndirectCommandsTokenTypeNV(0);
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV: VkIndirectCommandsTokenTypeNV =
  VkIndirectCommandsTokenTypeNV(1);
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV:
  VkIndirectCommandsTokenTypeNV = VkIndirectCommandsTokenTypeNV(3);
impl core::fmt::Debug for VkIndirectCommandsTokenTypeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV => {
        "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV"
      }
      VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_MESH_TASKS_NV => {
        "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_MESH_TASKS_NV"
      }
      VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV => {
        "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV"
      }
      VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV => {
        "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV"
      }
      VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV => {
        "VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV"
      }
      VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV => {
        "VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV"
      }
      VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV => {
        "VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV"
      }
      VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV => {
        "VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV"
      }
      VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV => {
        "VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV"
      }
      other => return write!(f, "VkIndirectCommandsTokenTypeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkInternalAllocationType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInternalAllocationType.html) (enumeration)
  VkInternalAllocationType(u32)
);
pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: VkInternalAllocationType =
  VkInternalAllocationType(0);
impl core::fmt::Debug for VkInternalAllocationType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE => "VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE",
      other => return write!(f, "VkInternalAllocationType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkLineRasterizationModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLineRasterizationModeEXT.html) (enumeration)
  VkLineRasterizationModeEXT(u32)
);
pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT: VkLineRasterizationModeEXT =
  VkLineRasterizationModeEXT(2);
pub const VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT: VkLineRasterizationModeEXT =
  VkLineRasterizationModeEXT(0);
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT: VkLineRasterizationModeEXT =
  VkLineRasterizationModeEXT(1);
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT: VkLineRasterizationModeEXT =
  VkLineRasterizationModeEXT(3);
impl core::fmt::Debug for VkLineRasterizationModeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT => {
        "VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT"
      }
      VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT => "VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT",
      VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT => {
        "VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT"
      }
      VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT => {
        "VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT"
      }
      other => return write!(f, "VkLineRasterizationModeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkLogicOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLogicOp.html) (enumeration)
  VkLogicOp(u32)
);
pub const VK_LOGIC_OP_AND: VkLogicOp = VkLogicOp(1);
pub const VK_LOGIC_OP_AND_INVERTED: VkLogicOp = VkLogicOp(4);
pub const VK_LOGIC_OP_AND_REVERSE: VkLogicOp = VkLogicOp(2);
pub const VK_LOGIC_OP_CLEAR: VkLogicOp = VkLogicOp(0);
pub const VK_LOGIC_OP_COPY: VkLogicOp = VkLogicOp(3);
pub const VK_LOGIC_OP_COPY_INVERTED: VkLogicOp = VkLogicOp(12);
pub const VK_LOGIC_OP_EQUIVALENT: VkLogicOp = VkLogicOp(9);
pub const VK_LOGIC_OP_INVERT: VkLogicOp = VkLogicOp(10);
pub const VK_LOGIC_OP_NAND: VkLogicOp = VkLogicOp(14);
pub const VK_LOGIC_OP_NOR: VkLogicOp = VkLogicOp(8);
pub const VK_LOGIC_OP_NO_OP: VkLogicOp = VkLogicOp(5);
pub const VK_LOGIC_OP_OR: VkLogicOp = VkLogicOp(7);
pub const VK_LOGIC_OP_OR_INVERTED: VkLogicOp = VkLogicOp(13);
pub const VK_LOGIC_OP_OR_REVERSE: VkLogicOp = VkLogicOp(11);
pub const VK_LOGIC_OP_SET: VkLogicOp = VkLogicOp(15);
pub const VK_LOGIC_OP_XOR: VkLogicOp = VkLogicOp(6);
impl core::fmt::Debug for VkLogicOp {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_LOGIC_OP_AND => "VK_LOGIC_OP_AND",
      VK_LOGIC_OP_AND_INVERTED => "VK_LOGIC_OP_AND_INVERTED",
      VK_LOGIC_OP_AND_REVERSE => "VK_LOGIC_OP_AND_REVERSE",
      VK_LOGIC_OP_CLEAR => "VK_LOGIC_OP_CLEAR",
      VK_LOGIC_OP_COPY => "VK_LOGIC_OP_COPY",
      VK_LOGIC_OP_COPY_INVERTED => "VK_LOGIC_OP_COPY_INVERTED",
      VK_LOGIC_OP_EQUIVALENT => "VK_LOGIC_OP_EQUIVALENT",
      VK_LOGIC_OP_INVERT => "VK_LOGIC_OP_INVERT",
      VK_LOGIC_OP_NAND => "VK_LOGIC_OP_NAND",
      VK_LOGIC_OP_NOR => "VK_LOGIC_OP_NOR",
      VK_LOGIC_OP_NO_OP => "VK_LOGIC_OP_NO_OP",
      VK_LOGIC_OP_OR => "VK_LOGIC_OP_OR",
      VK_LOGIC_OP_OR_INVERTED => "VK_LOGIC_OP_OR_INVERTED",
      VK_LOGIC_OP_OR_REVERSE => "VK_LOGIC_OP_OR_REVERSE",
      VK_LOGIC_OP_SET => "VK_LOGIC_OP_SET",
      VK_LOGIC_OP_XOR => "VK_LOGIC_OP_XOR",
      other => return write!(f, "VkLogicOp({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkMemoryOverallocationBehaviorAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html) (enumeration)
  VkMemoryOverallocationBehaviorAMD(u32)
);
pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD:
  VkMemoryOverallocationBehaviorAMD = VkMemoryOverallocationBehaviorAMD(1);
pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD:
  VkMemoryOverallocationBehaviorAMD = VkMemoryOverallocationBehaviorAMD(0);
pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD:
  VkMemoryOverallocationBehaviorAMD = VkMemoryOverallocationBehaviorAMD(2);
impl core::fmt::Debug for VkMemoryOverallocationBehaviorAMD {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD => {
        "VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD"
      }
      VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD => {
        "VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD"
      }
      VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD => {
        "VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD"
      }
      other => return write!(f, "VkMemoryOverallocationBehaviorAMD({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkMicromapTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapTypeEXT.html) (enumeration)
  VkMicromapTypeEXT(u32)
);
pub const VK_MICROMAP_TYPE_OPACITY_MICROMAP_EXT: VkMicromapTypeEXT = VkMicromapTypeEXT(0);
impl core::fmt::Debug for VkMicromapTypeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_MICROMAP_TYPE_OPACITY_MICROMAP_EXT => "VK_MICROMAP_TYPE_OPACITY_MICROMAP_EXT",
      other => return write!(f, "VkMicromapTypeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkObjectType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkObjectType.html) (enumeration)
  VkObjectType(u32)
);
pub const VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR: VkObjectType =
  VkObjectType(1000150000);
pub const VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV: VkObjectType =
  VkObjectType(1000165000);
pub const VK_OBJECT_TYPE_BUFFER: VkObjectType = VkObjectType(9);
/// VkBufferCollectionFUCHSIA
pub const VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA: VkObjectType =
  VkObjectType(1000366000);
pub const VK_OBJECT_TYPE_BUFFER_VIEW: VkObjectType = VkObjectType(13);
pub const VK_OBJECT_TYPE_COMMAND_BUFFER: VkObjectType = VkObjectType(6);
pub const VK_OBJECT_TYPE_COMMAND_POOL: VkObjectType = VkObjectType(25);
pub const VK_OBJECT_TYPE_CU_FUNCTION_NVX: VkObjectType = VkObjectType(1000029001);
pub const VK_OBJECT_TYPE_CU_MODULE_NVX: VkObjectType = VkObjectType(1000029000);
pub const VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT: VkObjectType =
  VkObjectType(1000011000);
pub const VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT: VkObjectType =
  VkObjectType(1000128000);
pub const VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR: VkObjectType = VkObjectType(1000268000);
pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: VkObjectType = VkObjectType(22);
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: VkObjectType = VkObjectType(23);
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: VkObjectType = VkObjectType(20);
pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE: VkObjectType =
  VkObjectType(1000085000);
pub const VK_OBJECT_TYPE_DEVICE: VkObjectType = VkObjectType(3);
pub const VK_OBJECT_TYPE_DEVICE_MEMORY: VkObjectType = VkObjectType(8);
pub const VK_OBJECT_TYPE_DISPLAY_KHR: VkObjectType = VkObjectType(1000002000);
pub const VK_OBJECT_TYPE_DISPLAY_MODE_KHR: VkObjectType = VkObjectType(1000002001);
pub const VK_OBJECT_TYPE_EVENT: VkObjectType = VkObjectType(11);
pub const VK_OBJECT_TYPE_FENCE: VkObjectType = VkObjectType(7);
pub const VK_OBJECT_TYPE_FRAMEBUFFER: VkObjectType = VkObjectType(24);
pub const VK_OBJECT_TYPE_IMAGE: VkObjectType = VkObjectType(10);
pub const VK_OBJECT_TYPE_IMAGE_VIEW: VkObjectType = VkObjectType(14);
pub const VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV: VkObjectType =
  VkObjectType(1000277000);
pub const VK_OBJECT_TYPE_INSTANCE: VkObjectType = VkObjectType(1);
pub const VK_OBJECT_TYPE_MICROMAP_EXT: VkObjectType = VkObjectType(1000396000);
pub const VK_OBJECT_TYPE_OPTICAL_FLOW_SESSION_NV: VkObjectType = VkObjectType(1000464000);
pub const VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL: VkObjectType =
  VkObjectType(1000210000);
pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: VkObjectType = VkObjectType(2);
pub const VK_OBJECT_TYPE_PIPELINE: VkObjectType = VkObjectType(19);
pub const VK_OBJECT_TYPE_PIPELINE_CACHE: VkObjectType = VkObjectType(16);
pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: VkObjectType = VkObjectType(17);
pub const VK_OBJECT_TYPE_PRIVATE_DATA_SLOT: VkObjectType = VkObjectType(1000295000);
pub const VK_OBJECT_TYPE_QUERY_POOL: VkObjectType = VkObjectType(12);
pub const VK_OBJECT_TYPE_QUEUE: VkObjectType = VkObjectType(4);
pub const VK_OBJECT_TYPE_RENDER_PASS: VkObjectType = VkObjectType(18);
pub const VK_OBJECT_TYPE_SAMPLER: VkObjectType = VkObjectType(21);
pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION: VkObjectType =
  VkObjectType(1000156000);
pub const VK_OBJECT_TYPE_SEMAPHORE: VkObjectType = VkObjectType(5);
/// VkSemaphoreSciSyncPoolNV
pub const VK_OBJECT_TYPE_SEMAPHORE_SCI_SYNC_POOL_NV: VkObjectType =
  VkObjectType(1000489000);
pub const VK_OBJECT_TYPE_SHADER_MODULE: VkObjectType = VkObjectType(15);
pub const VK_OBJECT_TYPE_SURFACE_KHR: VkObjectType = VkObjectType(1000000000);
pub const VK_OBJECT_TYPE_SWAPCHAIN_KHR: VkObjectType = VkObjectType(1000001000);
pub const VK_OBJECT_TYPE_UNKNOWN: VkObjectType = VkObjectType(0);
pub const VK_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkObjectType = VkObjectType(1000160000);
/// VkVideoSessionKHR
pub const VK_OBJECT_TYPE_VIDEO_SESSION_KHR: VkObjectType = VkObjectType(1000023000);
/// VkVideoSessionParametersKHR
pub const VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR: VkObjectType =
  VkObjectType(1000023001);
/// Alias of [`VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE`]
pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: VkObjectType =
  VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE;
/// Alias of [`VK_OBJECT_TYPE_PRIVATE_DATA_SLOT`]
pub const VK_OBJECT_TYPE_PRIVATE_DATA_SLOT_EXT: VkObjectType =
  VK_OBJECT_TYPE_PRIVATE_DATA_SLOT;
/// Alias of [`VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION`]
pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR: VkObjectType =
  VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION;
impl core::fmt::Debug for VkObjectType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR => {
        "VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR"
      }
      VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV => {
        "VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV"
      }
      VK_OBJECT_TYPE_BUFFER => "VK_OBJECT_TYPE_BUFFER",
      VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA => {
        "VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA"
      }
      VK_OBJECT_TYPE_BUFFER_VIEW => "VK_OBJECT_TYPE_BUFFER_VIEW",
      VK_OBJECT_TYPE_COMMAND_BUFFER => "VK_OBJECT_TYPE_COMMAND_BUFFER",
      VK_OBJECT_TYPE_COMMAND_POOL => "VK_OBJECT_TYPE_COMMAND_POOL",
      VK_OBJECT_TYPE_CU_FUNCTION_NVX => "VK_OBJECT_TYPE_CU_FUNCTION_NVX",
      VK_OBJECT_TYPE_CU_MODULE_NVX => "VK_OBJECT_TYPE_CU_MODULE_NVX",
      VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT => {
        "VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT"
      }
      VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT => {
        "VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT"
      }
      VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR => "VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR",
      VK_OBJECT_TYPE_DESCRIPTOR_POOL => "VK_OBJECT_TYPE_DESCRIPTOR_POOL",
      VK_OBJECT_TYPE_DESCRIPTOR_SET => "VK_OBJECT_TYPE_DESCRIPTOR_SET",
      VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT => "VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT",
      VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE => {
        "VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE"
      }
      VK_OBJECT_TYPE_DEVICE => "VK_OBJECT_TYPE_DEVICE",
      VK_OBJECT_TYPE_DEVICE_MEMORY => "VK_OBJECT_TYPE_DEVICE_MEMORY",
      VK_OBJECT_TYPE_DISPLAY_KHR => "VK_OBJECT_TYPE_DISPLAY_KHR",
      VK_OBJECT_TYPE_DISPLAY_MODE_KHR => "VK_OBJECT_TYPE_DISPLAY_MODE_KHR",
      VK_OBJECT_TYPE_EVENT => "VK_OBJECT_TYPE_EVENT",
      VK_OBJECT_TYPE_FENCE => "VK_OBJECT_TYPE_FENCE",
      VK_OBJECT_TYPE_FRAMEBUFFER => "VK_OBJECT_TYPE_FRAMEBUFFER",
      VK_OBJECT_TYPE_IMAGE => "VK_OBJECT_TYPE_IMAGE",
      VK_OBJECT_TYPE_IMAGE_VIEW => "VK_OBJECT_TYPE_IMAGE_VIEW",
      VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV => {
        "VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV"
      }
      VK_OBJECT_TYPE_INSTANCE => "VK_OBJECT_TYPE_INSTANCE",
      VK_OBJECT_TYPE_MICROMAP_EXT => "VK_OBJECT_TYPE_MICROMAP_EXT",
      VK_OBJECT_TYPE_OPTICAL_FLOW_SESSION_NV => "VK_OBJECT_TYPE_OPTICAL_FLOW_SESSION_NV",
      VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL => {
        "VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL"
      }
      VK_OBJECT_TYPE_PHYSICAL_DEVICE => "VK_OBJECT_TYPE_PHYSICAL_DEVICE",
      VK_OBJECT_TYPE_PIPELINE => "VK_OBJECT_TYPE_PIPELINE",
      VK_OBJECT_TYPE_PIPELINE_CACHE => "VK_OBJECT_TYPE_PIPELINE_CACHE",
      VK_OBJECT_TYPE_PIPELINE_LAYOUT => "VK_OBJECT_TYPE_PIPELINE_LAYOUT",
      VK_OBJECT_TYPE_PRIVATE_DATA_SLOT => "VK_OBJECT_TYPE_PRIVATE_DATA_SLOT",
      VK_OBJECT_TYPE_QUERY_POOL => "VK_OBJECT_TYPE_QUERY_POOL",
      VK_OBJECT_TYPE_QUEUE => "VK_OBJECT_TYPE_QUEUE",
      VK_OBJECT_TYPE_RENDER_PASS => "VK_OBJECT_TYPE_RENDER_PASS",
      VK_OBJECT_TYPE_SAMPLER => "VK_OBJECT_TYPE_SAMPLER",
      VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION => {
        "VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION"
      }
      VK_OBJECT_TYPE_SEMAPHORE => "VK_OBJECT_TYPE_SEMAPHORE",
      VK_OBJECT_TYPE_SEMAPHORE_SCI_SYNC_POOL_NV => {
        "VK_OBJECT_TYPE_SEMAPHORE_SCI_SYNC_POOL_NV"
      }
      VK_OBJECT_TYPE_SHADER_MODULE => "VK_OBJECT_TYPE_SHADER_MODULE",
      VK_OBJECT_TYPE_SURFACE_KHR => "VK_OBJECT_TYPE_SURFACE_KHR",
      VK_OBJECT_TYPE_SWAPCHAIN_KHR => "VK_OBJECT_TYPE_SWAPCHAIN_KHR",
      VK_OBJECT_TYPE_UNKNOWN => "VK_OBJECT_TYPE_UNKNOWN",
      VK_OBJECT_TYPE_VALIDATION_CACHE_EXT => "VK_OBJECT_TYPE_VALIDATION_CACHE_EXT",
      VK_OBJECT_TYPE_VIDEO_SESSION_KHR => "VK_OBJECT_TYPE_VIDEO_SESSION_KHR",
      VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR => {
        "VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR"
      }
      other => return write!(f, "VkObjectType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkOpacityMicromapFormatEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpacityMicromapFormatEXT.html) (enumeration)
  VkOpacityMicromapFormatEXT(u32)
);
pub const VK_OPACITY_MICROMAP_FORMAT_2_STATE_EXT: VkOpacityMicromapFormatEXT =
  VkOpacityMicromapFormatEXT(1);
pub const VK_OPACITY_MICROMAP_FORMAT_4_STATE_EXT: VkOpacityMicromapFormatEXT =
  VkOpacityMicromapFormatEXT(2);
impl core::fmt::Debug for VkOpacityMicromapFormatEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_OPACITY_MICROMAP_FORMAT_2_STATE_EXT => "VK_OPACITY_MICROMAP_FORMAT_2_STATE_EXT",
      VK_OPACITY_MICROMAP_FORMAT_4_STATE_EXT => "VK_OPACITY_MICROMAP_FORMAT_4_STATE_EXT",
      other => return write!(f, "VkOpacityMicromapFormatEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkOpacityMicromapSpecialIndexEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpacityMicromapSpecialIndexEXT.html) (enumeration)
  VkOpacityMicromapSpecialIndexEXT(i32)
);
pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_OPAQUE_EXT:
  VkOpacityMicromapSpecialIndexEXT = VkOpacityMicromapSpecialIndexEXT(-2);
pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_TRANSPARENT_EXT:
  VkOpacityMicromapSpecialIndexEXT = VkOpacityMicromapSpecialIndexEXT(-1);
pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_OPAQUE_EXT:
  VkOpacityMicromapSpecialIndexEXT = VkOpacityMicromapSpecialIndexEXT(-4);
pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_TRANSPARENT_EXT:
  VkOpacityMicromapSpecialIndexEXT = VkOpacityMicromapSpecialIndexEXT(-3);
impl core::fmt::Debug for VkOpacityMicromapSpecialIndexEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_OPAQUE_EXT => {
        "VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_OPAQUE_EXT"
      }
      VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_TRANSPARENT_EXT => {
        "VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_TRANSPARENT_EXT"
      }
      VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_OPAQUE_EXT => {
        "VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_OPAQUE_EXT"
      }
      VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_TRANSPARENT_EXT => {
        "VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_TRANSPARENT_EXT"
      }
      other => return write!(f, "VkOpacityMicromapSpecialIndexEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkOpticalFlowPerformanceLevelNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowPerformanceLevelNV.html) (enumeration)
  VkOpticalFlowPerformanceLevelNV(u32)
);
pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_NV: VkOpticalFlowPerformanceLevelNV =
  VkOpticalFlowPerformanceLevelNV(3);
pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_NV: VkOpticalFlowPerformanceLevelNV =
  VkOpticalFlowPerformanceLevelNV(2);
pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_NV: VkOpticalFlowPerformanceLevelNV =
  VkOpticalFlowPerformanceLevelNV(1);
pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_NV: VkOpticalFlowPerformanceLevelNV =
  VkOpticalFlowPerformanceLevelNV(0);
impl core::fmt::Debug for VkOpticalFlowPerformanceLevelNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_NV => {
        "VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_NV"
      }
      VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_NV => {
        "VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_NV"
      }
      VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_NV => {
        "VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_NV"
      }
      VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_NV => {
        "VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_NV"
      }
      other => return write!(f, "VkOpticalFlowPerformanceLevelNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkOpticalFlowSessionBindingPointNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionBindingPointNV.html) (enumeration)
  VkOpticalFlowSessionBindingPointNV(u32)
);
pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_COST_NV:
  VkOpticalFlowSessionBindingPointNV = VkOpticalFlowSessionBindingPointNV(7);
pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_FLOW_VECTOR_NV:
  VkOpticalFlowSessionBindingPointNV = VkOpticalFlowSessionBindingPointNV(5);
pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_COST_NV:
  VkOpticalFlowSessionBindingPointNV = VkOpticalFlowSessionBindingPointNV(6);
pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_FLOW_VECTOR_NV:
  VkOpticalFlowSessionBindingPointNV = VkOpticalFlowSessionBindingPointNV(4);
pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_GLOBAL_FLOW_NV:
  VkOpticalFlowSessionBindingPointNV = VkOpticalFlowSessionBindingPointNV(8);
pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_HINT_NV:
  VkOpticalFlowSessionBindingPointNV = VkOpticalFlowSessionBindingPointNV(3);
pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_INPUT_NV:
  VkOpticalFlowSessionBindingPointNV = VkOpticalFlowSessionBindingPointNV(1);
pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_REFERENCE_NV:
  VkOpticalFlowSessionBindingPointNV = VkOpticalFlowSessionBindingPointNV(2);
pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_UNKNOWN_NV:
  VkOpticalFlowSessionBindingPointNV = VkOpticalFlowSessionBindingPointNV(0);
impl core::fmt::Debug for VkOpticalFlowSessionBindingPointNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_COST_NV => {
        "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_COST_NV"
      }
      VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_FLOW_VECTOR_NV => {
        "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_FLOW_VECTOR_NV"
      }
      VK_OPTICAL_FLOW_SESSION_BINDING_POINT_COST_NV => {
        "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_COST_NV"
      }
      VK_OPTICAL_FLOW_SESSION_BINDING_POINT_FLOW_VECTOR_NV => {
        "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_FLOW_VECTOR_NV"
      }
      VK_OPTICAL_FLOW_SESSION_BINDING_POINT_GLOBAL_FLOW_NV => {
        "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_GLOBAL_FLOW_NV"
      }
      VK_OPTICAL_FLOW_SESSION_BINDING_POINT_HINT_NV => {
        "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_HINT_NV"
      }
      VK_OPTICAL_FLOW_SESSION_BINDING_POINT_INPUT_NV => {
        "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_INPUT_NV"
      }
      VK_OPTICAL_FLOW_SESSION_BINDING_POINT_REFERENCE_NV => {
        "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_REFERENCE_NV"
      }
      VK_OPTICAL_FLOW_SESSION_BINDING_POINT_UNKNOWN_NV => {
        "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_UNKNOWN_NV"
      }
      other => return write!(f, "VkOpticalFlowSessionBindingPointNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPerformanceConfigurationTypeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html) (enumeration)
  VkPerformanceConfigurationTypeINTEL(u32)
);
pub const VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: VkPerformanceConfigurationTypeINTEL = VkPerformanceConfigurationTypeINTEL(0);
impl core::fmt::Debug for VkPerformanceConfigurationTypeINTEL {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL => "VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL",
      other => return write!(f, "VkPerformanceConfigurationTypeINTEL({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPerformanceCounterScopeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterScopeKHR.html) (enumeration)
  VkPerformanceCounterScopeKHR(u32)
);
pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR: VkPerformanceCounterScopeKHR =
  VkPerformanceCounterScopeKHR(0);
pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR: VkPerformanceCounterScopeKHR =
  VkPerformanceCounterScopeKHR(2);
pub const VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR: VkPerformanceCounterScopeKHR =
  VkPerformanceCounterScopeKHR(1);
/// Alias of [`VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR`]
#[deprecated = "aliased"]
pub const VK_QUERY_SCOPE_COMMAND_BUFFER_KHR: VkPerformanceCounterScopeKHR =
  VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR;
/// Alias of [`VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR`]
#[deprecated = "aliased"]
pub const VK_QUERY_SCOPE_COMMAND_KHR: VkPerformanceCounterScopeKHR =
  VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR;
/// Alias of [`VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR`]
#[deprecated = "aliased"]
pub const VK_QUERY_SCOPE_RENDER_PASS_KHR: VkPerformanceCounterScopeKHR =
  VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR;
impl core::fmt::Debug for VkPerformanceCounterScopeKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR => {
        "VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR"
      }
      VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR => {
        "VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR"
      }
      VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR => {
        "VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR"
      }
      other => return write!(f, "VkPerformanceCounterScopeKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPerformanceCounterStorageKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterStorageKHR.html) (enumeration)
  VkPerformanceCounterStorageKHR(u32)
);
pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR: VkPerformanceCounterStorageKHR =
  VkPerformanceCounterStorageKHR(4);
pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR: VkPerformanceCounterStorageKHR =
  VkPerformanceCounterStorageKHR(5);
pub const VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR: VkPerformanceCounterStorageKHR =
  VkPerformanceCounterStorageKHR(0);
pub const VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR: VkPerformanceCounterStorageKHR =
  VkPerformanceCounterStorageKHR(1);
pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR: VkPerformanceCounterStorageKHR =
  VkPerformanceCounterStorageKHR(2);
pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR: VkPerformanceCounterStorageKHR =
  VkPerformanceCounterStorageKHR(3);
impl core::fmt::Debug for VkPerformanceCounterStorageKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR => {
        "VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR"
      }
      VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR => {
        "VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR"
      }
      VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR => {
        "VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR"
      }
      VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR => {
        "VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR"
      }
      VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR => {
        "VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR"
      }
      VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR => {
        "VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR"
      }
      other => return write!(f, "VkPerformanceCounterStorageKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPerformanceCounterUnitKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterUnitKHR.html) (enumeration)
  VkPerformanceCounterUnitKHR(u32)
);
pub const VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(8);
pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(3);
pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(4);
pub const VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(10);
pub const VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(0);
pub const VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(9);
pub const VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(5);
pub const VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(2);
pub const VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(1);
pub const VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(7);
pub const VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR: VkPerformanceCounterUnitKHR =
  VkPerformanceCounterUnitKHR(6);
impl core::fmt::Debug for VkPerformanceCounterUnitKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR => "VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR",
      VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR => "VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR",
      VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR => {
        "VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR"
      }
      VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR => "VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR",
      VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR => {
        "VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR"
      }
      VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR => "VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR",
      VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR => "VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR",
      VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR => {
        "VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR"
      }
      VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR => {
        "VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR"
      }
      VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR => "VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR",
      VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR => "VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR",
      other => return write!(f, "VkPerformanceCounterUnitKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPerformanceOverrideTypeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideTypeINTEL.html) (enumeration)
  VkPerformanceOverrideTypeINTEL(u32)
);
pub const VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL:
  VkPerformanceOverrideTypeINTEL = VkPerformanceOverrideTypeINTEL(1);
pub const VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL:
  VkPerformanceOverrideTypeINTEL = VkPerformanceOverrideTypeINTEL(0);
impl core::fmt::Debug for VkPerformanceOverrideTypeINTEL {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL => {
        "VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL"
      }
      VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL => {
        "VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL"
      }
      other => return write!(f, "VkPerformanceOverrideTypeINTEL({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPerformanceParameterTypeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceParameterTypeINTEL.html) (enumeration)
  VkPerformanceParameterTypeINTEL(u32)
);
pub const VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL:
  VkPerformanceParameterTypeINTEL = VkPerformanceParameterTypeINTEL(0);
pub const VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL:
  VkPerformanceParameterTypeINTEL = VkPerformanceParameterTypeINTEL(1);
impl core::fmt::Debug for VkPerformanceParameterTypeINTEL {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL => {
        "VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL"
      }
      VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL => {
        "VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL"
      }
      other => return write!(f, "VkPerformanceParameterTypeINTEL({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPerformanceValueTypeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueTypeINTEL.html) (enumeration)
  VkPerformanceValueTypeINTEL(u32)
);
pub const VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL: VkPerformanceValueTypeINTEL =
  VkPerformanceValueTypeINTEL(3);
pub const VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL: VkPerformanceValueTypeINTEL =
  VkPerformanceValueTypeINTEL(2);
pub const VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL: VkPerformanceValueTypeINTEL =
  VkPerformanceValueTypeINTEL(4);
pub const VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL: VkPerformanceValueTypeINTEL =
  VkPerformanceValueTypeINTEL(0);
pub const VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL: VkPerformanceValueTypeINTEL =
  VkPerformanceValueTypeINTEL(1);
impl core::fmt::Debug for VkPerformanceValueTypeINTEL {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL => "VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL",
      VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL => "VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL",
      VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL => "VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL",
      VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL => "VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL",
      VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL => "VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL",
      other => return write!(f, "VkPerformanceValueTypeINTEL({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPhysicalDeviceType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceType.html) (enumeration)
  VkPhysicalDeviceType(u32)
);
pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = VkPhysicalDeviceType(4);
pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType =
  VkPhysicalDeviceType(2);
pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType =
  VkPhysicalDeviceType(1);
pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = VkPhysicalDeviceType(0);
pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType =
  VkPhysicalDeviceType(3);
impl core::fmt::Debug for VkPhysicalDeviceType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PHYSICAL_DEVICE_TYPE_CPU => "VK_PHYSICAL_DEVICE_TYPE_CPU",
      VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => "VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU",
      VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => "VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU",
      VK_PHYSICAL_DEVICE_TYPE_OTHER => "VK_PHYSICAL_DEVICE_TYPE_OTHER",
      VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => "VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU",
      other => return write!(f, "VkPhysicalDeviceType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPipelineBindPoint](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineBindPoint.html) (enumeration)
  VkPipelineBindPoint(u32)
);
pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint = VkPipelineBindPoint(1);
pub const VK_PIPELINE_BIND_POINT_GRAPHICS: VkPipelineBindPoint = VkPipelineBindPoint(0);
pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR: VkPipelineBindPoint =
  VkPipelineBindPoint(1000165000);
pub const VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI: VkPipelineBindPoint =
  VkPipelineBindPoint(1000369003);
/// Alias of [`VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR`]
pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_NV: VkPipelineBindPoint =
  VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR;
impl core::fmt::Debug for VkPipelineBindPoint {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PIPELINE_BIND_POINT_COMPUTE => "VK_PIPELINE_BIND_POINT_COMPUTE",
      VK_PIPELINE_BIND_POINT_GRAPHICS => "VK_PIPELINE_BIND_POINT_GRAPHICS",
      VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR => "VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR",
      VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI => {
        "VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI"
      }
      other => return write!(f, "VkPipelineBindPoint({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPipelineCacheHeaderVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersion.html) (enumeration)
  VkPipelineCacheHeaderVersion(u32)
);
pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE: VkPipelineCacheHeaderVersion =
  VkPipelineCacheHeaderVersion(1);
impl core::fmt::Debug for VkPipelineCacheHeaderVersion {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PIPELINE_CACHE_HEADER_VERSION_ONE => "VK_PIPELINE_CACHE_HEADER_VERSION_ONE",
      other => return write!(f, "VkPipelineCacheHeaderVersion({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPipelineCacheValidationVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheValidationVersion.html) (enumeration)
  VkPipelineCacheValidationVersion(u32)
);
pub const VK_PIPELINE_CACHE_VALIDATION_VERSION_SAFETY_CRITICAL_ONE:
  VkPipelineCacheValidationVersion = VkPipelineCacheValidationVersion(1);
impl core::fmt::Debug for VkPipelineCacheValidationVersion {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PIPELINE_CACHE_VALIDATION_VERSION_SAFETY_CRITICAL_ONE => {
        "VK_PIPELINE_CACHE_VALIDATION_VERSION_SAFETY_CRITICAL_ONE"
      }
      other => return write!(f, "VkPipelineCacheValidationVersion({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPipelineExecutableStatisticFormatKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html) (enumeration)
  VkPipelineExecutableStatisticFormatKHR(u32)
);
pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR:
  VkPipelineExecutableStatisticFormatKHR = VkPipelineExecutableStatisticFormatKHR(0);
pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR:
  VkPipelineExecutableStatisticFormatKHR = VkPipelineExecutableStatisticFormatKHR(3);
pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR:
  VkPipelineExecutableStatisticFormatKHR = VkPipelineExecutableStatisticFormatKHR(1);
pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR:
  VkPipelineExecutableStatisticFormatKHR = VkPipelineExecutableStatisticFormatKHR(2);
impl core::fmt::Debug for VkPipelineExecutableStatisticFormatKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR => {
        "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR"
      }
      VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR => {
        "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR"
      }
      VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR => {
        "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR"
      }
      VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR => {
        "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR"
      }
      other => return write!(f, "VkPipelineExecutableStatisticFormatKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPipelineMatchControl](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMatchControl.html) (enumeration)
  VkPipelineMatchControl(u32)
);
pub const VK_PIPELINE_MATCH_CONTROL_APPLICATION_UUID_EXACT_MATCH: VkPipelineMatchControl =
  VkPipelineMatchControl(0);
impl core::fmt::Debug for VkPipelineMatchControl {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PIPELINE_MATCH_CONTROL_APPLICATION_UUID_EXACT_MATCH => {
        "VK_PIPELINE_MATCH_CONTROL_APPLICATION_UUID_EXACT_MATCH"
      }
      other => return write!(f, "VkPipelineMatchControl({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPipelineRobustnessBufferBehaviorEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRobustnessBufferBehaviorEXT.html) (enumeration)
  VkPipelineRobustnessBufferBehaviorEXT(u32)
);
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT_EXT:
  VkPipelineRobustnessBufferBehaviorEXT = VkPipelineRobustnessBufferBehaviorEXT(0);
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED_EXT:
  VkPipelineRobustnessBufferBehaviorEXT = VkPipelineRobustnessBufferBehaviorEXT(1);
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2_EXT:
  VkPipelineRobustnessBufferBehaviorEXT = VkPipelineRobustnessBufferBehaviorEXT(3);
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_EXT:
  VkPipelineRobustnessBufferBehaviorEXT = VkPipelineRobustnessBufferBehaviorEXT(2);
impl core::fmt::Debug for VkPipelineRobustnessBufferBehaviorEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT_EXT => {
        "VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT_EXT"
      }
      VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED_EXT => {
        "VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED_EXT"
      }
      VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2_EXT => {
        "VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2_EXT"
      }
      VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_EXT => {
        "VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_EXT"
      }
      other => return write!(f, "VkPipelineRobustnessBufferBehaviorEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPipelineRobustnessImageBehaviorEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRobustnessImageBehaviorEXT.html) (enumeration)
  VkPipelineRobustnessImageBehaviorEXT(u32)
);
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT_EXT:
  VkPipelineRobustnessImageBehaviorEXT = VkPipelineRobustnessImageBehaviorEXT(0);
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED_EXT:
  VkPipelineRobustnessImageBehaviorEXT = VkPipelineRobustnessImageBehaviorEXT(1);
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2_EXT:
  VkPipelineRobustnessImageBehaviorEXT = VkPipelineRobustnessImageBehaviorEXT(3);
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_EXT:
  VkPipelineRobustnessImageBehaviorEXT = VkPipelineRobustnessImageBehaviorEXT(2);
impl core::fmt::Debug for VkPipelineRobustnessImageBehaviorEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT_EXT => {
        "VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT_EXT"
      }
      VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED_EXT => {
        "VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED_EXT"
      }
      VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2_EXT => {
        "VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2_EXT"
      }
      VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_EXT => {
        "VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_EXT"
      }
      other => return write!(f, "VkPipelineRobustnessImageBehaviorEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPointClippingBehavior](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPointClippingBehavior.html) (enumeration)
  VkPointClippingBehavior(u32)
);
pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: VkPointClippingBehavior =
  VkPointClippingBehavior(0);
pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: VkPointClippingBehavior =
  VkPointClippingBehavior(1);
/// Alias of [`VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES`]
pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR: VkPointClippingBehavior =
  VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES;
/// Alias of [`VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY`]
pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR: VkPointClippingBehavior =
  VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY;
impl core::fmt::Debug for VkPointClippingBehavior {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES => {
        "VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES"
      }
      VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY => {
        "VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY"
      }
      other => return write!(f, "VkPointClippingBehavior({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPolygonMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPolygonMode.html) (enumeration)
  VkPolygonMode(u32)
);
pub const VK_POLYGON_MODE_FILL: VkPolygonMode = VkPolygonMode(0);
pub const VK_POLYGON_MODE_FILL_RECTANGLE_NV: VkPolygonMode = VkPolygonMode(1000153000);
pub const VK_POLYGON_MODE_LINE: VkPolygonMode = VkPolygonMode(1);
pub const VK_POLYGON_MODE_POINT: VkPolygonMode = VkPolygonMode(2);
impl core::fmt::Debug for VkPolygonMode {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_POLYGON_MODE_FILL => "VK_POLYGON_MODE_FILL",
      VK_POLYGON_MODE_FILL_RECTANGLE_NV => "VK_POLYGON_MODE_FILL_RECTANGLE_NV",
      VK_POLYGON_MODE_LINE => "VK_POLYGON_MODE_LINE",
      VK_POLYGON_MODE_POINT => "VK_POLYGON_MODE_POINT",
      other => return write!(f, "VkPolygonMode({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPresentModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html) (enumeration)
  VkPresentModeKHR(u32)
);
pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = VkPresentModeKHR(2);
pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: VkPresentModeKHR = VkPresentModeKHR(3);
pub const VK_PRESENT_MODE_IMMEDIATE_KHR: VkPresentModeKHR = VkPresentModeKHR(0);
pub const VK_PRESENT_MODE_MAILBOX_KHR: VkPresentModeKHR = VkPresentModeKHR(1);
pub const VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR: VkPresentModeKHR =
  VkPresentModeKHR(1000111001);
pub const VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR: VkPresentModeKHR =
  VkPresentModeKHR(1000111000);
impl core::fmt::Debug for VkPresentModeKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PRESENT_MODE_FIFO_KHR => "VK_PRESENT_MODE_FIFO_KHR",
      VK_PRESENT_MODE_FIFO_RELAXED_KHR => "VK_PRESENT_MODE_FIFO_RELAXED_KHR",
      VK_PRESENT_MODE_IMMEDIATE_KHR => "VK_PRESENT_MODE_IMMEDIATE_KHR",
      VK_PRESENT_MODE_MAILBOX_KHR => "VK_PRESENT_MODE_MAILBOX_KHR",
      VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR => {
        "VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR"
      }
      VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR => {
        "VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR"
      }
      other => return write!(f, "VkPresentModeKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkPrimitiveTopology](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html) (enumeration)
  VkPrimitiveTopology(u32)
);
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: VkPrimitiveTopology = VkPrimitiveTopology(1);
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: VkPrimitiveTopology =
  VkPrimitiveTopology(6);
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: VkPrimitiveTopology = VkPrimitiveTopology(2);
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology =
  VkPrimitiveTopology(7);
pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: VkPrimitiveTopology = VkPrimitiveTopology(10);
pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: VkPrimitiveTopology = VkPrimitiveTopology(0);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: VkPrimitiveTopology =
  VkPrimitiveTopology(5);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: VkPrimitiveTopology =
  VkPrimitiveTopology(3);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: VkPrimitiveTopology =
  VkPrimitiveTopology(8);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: VkPrimitiveTopology =
  VkPrimitiveTopology(4);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology =
  VkPrimitiveTopology(9);
impl core::fmt::Debug for VkPrimitiveTopology {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PRIMITIVE_TOPOLOGY_LINE_LIST => "VK_PRIMITIVE_TOPOLOGY_LINE_LIST",
      VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY => {
        "VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY"
      }
      VK_PRIMITIVE_TOPOLOGY_LINE_STRIP => "VK_PRIMITIVE_TOPOLOGY_LINE_STRIP",
      VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY => {
        "VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY"
      }
      VK_PRIMITIVE_TOPOLOGY_PATCH_LIST => "VK_PRIMITIVE_TOPOLOGY_PATCH_LIST",
      VK_PRIMITIVE_TOPOLOGY_POINT_LIST => "VK_PRIMITIVE_TOPOLOGY_POINT_LIST",
      VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN => "VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN",
      VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST => "VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST",
      VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY => {
        "VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY"
      }
      VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP => "VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP",
      VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY => {
        "VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY"
      }
      other => return write!(f, "VkPrimitiveTopology({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkProvokingVertexModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkProvokingVertexModeEXT.html) (enumeration)
  VkProvokingVertexModeEXT(u32)
);
pub const VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT: VkProvokingVertexModeEXT =
  VkProvokingVertexModeEXT(0);
pub const VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT: VkProvokingVertexModeEXT =
  VkProvokingVertexModeEXT(1);
impl core::fmt::Debug for VkProvokingVertexModeEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT => {
        "VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT"
      }
      VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT => {
        "VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT"
      }
      other => return write!(f, "VkProvokingVertexModeEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkQueryPoolSamplingModeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolSamplingModeINTEL.html) (enumeration)
  VkQueryPoolSamplingModeINTEL(u32)
);
pub const VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL: VkQueryPoolSamplingModeINTEL =
  VkQueryPoolSamplingModeINTEL(0);
impl core::fmt::Debug for VkQueryPoolSamplingModeINTEL {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL => {
        "VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL"
      }
      other => return write!(f, "VkQueryPoolSamplingModeINTEL({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkQueryResultStatusKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryResultStatusKHR.html) (enumeration)
  VkQueryResultStatusKHR(i32)
);
pub const VK_QUERY_RESULT_STATUS_COMPLETE_KHR: VkQueryResultStatusKHR =
  VkQueryResultStatusKHR(1);
pub const VK_QUERY_RESULT_STATUS_ERROR_KHR: VkQueryResultStatusKHR =
  VkQueryResultStatusKHR(-1);
pub const VK_QUERY_RESULT_STATUS_NOT_READY_KHR: VkQueryResultStatusKHR =
  VkQueryResultStatusKHR(0);
impl core::fmt::Debug for VkQueryResultStatusKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_QUERY_RESULT_STATUS_COMPLETE_KHR => "VK_QUERY_RESULT_STATUS_COMPLETE_KHR",
      VK_QUERY_RESULT_STATUS_ERROR_KHR => "VK_QUERY_RESULT_STATUS_ERROR_KHR",
      VK_QUERY_RESULT_STATUS_NOT_READY_KHR => "VK_QUERY_RESULT_STATUS_NOT_READY_KHR",
      other => return write!(f, "VkQueryResultStatusKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkQueryType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryType.html) (enumeration)
  VkQueryType(u32)
);
pub const VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: VkQueryType =
  VkQueryType(1000150000);
pub const VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: VkQueryType =
  VkQueryType(1000165000);
pub const VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR:
  VkQueryType = VkQueryType(1000386000);
pub const VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: VkQueryType =
  VkQueryType(1000150001);
pub const VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SIZE_KHR: VkQueryType =
  VkQueryType(1000386001);
pub const VK_QUERY_TYPE_MESH_PRIMITIVES_GENERATED_EXT: VkQueryType =
  VkQueryType(1000328000);
pub const VK_QUERY_TYPE_MICROMAP_COMPACTED_SIZE_EXT: VkQueryType =
  VkQueryType(1000396001);
pub const VK_QUERY_TYPE_MICROMAP_SERIALIZATION_SIZE_EXT: VkQueryType =
  VkQueryType(1000396000);
pub const VK_QUERY_TYPE_OCCLUSION: VkQueryType = VkQueryType(0);
pub const VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL: VkQueryType = VkQueryType(1000210000);
pub const VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR: VkQueryType = VkQueryType(1000116000);
/// Optional
pub const VK_QUERY_TYPE_PIPELINE_STATISTICS: VkQueryType = VkQueryType(1);
pub const VK_QUERY_TYPE_PRIMITIVES_GENERATED_EXT: VkQueryType = VkQueryType(1000382000);
pub const VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR: VkQueryType = VkQueryType(1000023000);
pub const VK_QUERY_TYPE_TIMESTAMP: VkQueryType = VkQueryType(2);
pub const VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT: VkQueryType =
  VkQueryType(1000028004);
pub const VK_QUERY_TYPE_VIDEO_ENCODE_FEEDBACK_KHR: VkQueryType = VkQueryType(1000299000);
impl core::fmt::Debug for VkQueryType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR => {
        "VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR"
      }
      VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV => {
        "VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV"
      }
      VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR => {
        "VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR"
      }
      VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR => {
        "VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR"
      }
      VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SIZE_KHR => {
        "VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SIZE_KHR"
      }
      VK_QUERY_TYPE_MESH_PRIMITIVES_GENERATED_EXT => {
        "VK_QUERY_TYPE_MESH_PRIMITIVES_GENERATED_EXT"
      }
      VK_QUERY_TYPE_MICROMAP_COMPACTED_SIZE_EXT => {
        "VK_QUERY_TYPE_MICROMAP_COMPACTED_SIZE_EXT"
      }
      VK_QUERY_TYPE_MICROMAP_SERIALIZATION_SIZE_EXT => {
        "VK_QUERY_TYPE_MICROMAP_SERIALIZATION_SIZE_EXT"
      }
      VK_QUERY_TYPE_OCCLUSION => "VK_QUERY_TYPE_OCCLUSION",
      VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL => "VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL",
      VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR => "VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR",
      VK_QUERY_TYPE_PIPELINE_STATISTICS => "VK_QUERY_TYPE_PIPELINE_STATISTICS",
      VK_QUERY_TYPE_PRIMITIVES_GENERATED_EXT => "VK_QUERY_TYPE_PRIMITIVES_GENERATED_EXT",
      VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR => "VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR",
      VK_QUERY_TYPE_TIMESTAMP => "VK_QUERY_TYPE_TIMESTAMP",
      VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT => {
        "VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT"
      }
      VK_QUERY_TYPE_VIDEO_ENCODE_FEEDBACK_KHR => {
        "VK_QUERY_TYPE_VIDEO_ENCODE_FEEDBACK_KHR"
      }
      other => return write!(f, "VkQueryType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkQueueGlobalPriorityKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueGlobalPriorityKHR.html) (enumeration)
  VkQueueGlobalPriorityKHR(u32)
);
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR: VkQueueGlobalPriorityKHR =
  VkQueueGlobalPriorityKHR(512);
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR: VkQueueGlobalPriorityKHR =
  VkQueueGlobalPriorityKHR(128);
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR: VkQueueGlobalPriorityKHR =
  VkQueueGlobalPriorityKHR(256);
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR: VkQueueGlobalPriorityKHR =
  VkQueueGlobalPriorityKHR(1024);
/// Alias of [`VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR`]
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT: VkQueueGlobalPriorityKHR =
  VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR;
/// Alias of [`VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR`]
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT: VkQueueGlobalPriorityKHR =
  VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR;
/// Alias of [`VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR`]
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT: VkQueueGlobalPriorityKHR =
  VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR;
/// Alias of [`VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR`]
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT: VkQueueGlobalPriorityKHR =
  VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR;
impl core::fmt::Debug for VkQueueGlobalPriorityKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR => "VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR",
      VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR => "VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR",
      VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR => "VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR",
      VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR => "VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR",
      other => return write!(f, "VkQueueGlobalPriorityKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkRasterizationOrderAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRasterizationOrderAMD.html) (enumeration)
  VkRasterizationOrderAMD(u32)
);
pub const VK_RASTERIZATION_ORDER_RELAXED_AMD: VkRasterizationOrderAMD =
  VkRasterizationOrderAMD(1);
pub const VK_RASTERIZATION_ORDER_STRICT_AMD: VkRasterizationOrderAMD =
  VkRasterizationOrderAMD(0);
impl core::fmt::Debug for VkRasterizationOrderAMD {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_RASTERIZATION_ORDER_RELAXED_AMD => "VK_RASTERIZATION_ORDER_RELAXED_AMD",
      VK_RASTERIZATION_ORDER_STRICT_AMD => "VK_RASTERIZATION_ORDER_STRICT_AMD",
      other => return write!(f, "VkRasterizationOrderAMD({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkRayTracingInvocationReorderModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingInvocationReorderModeNV.html) (enumeration)
  VkRayTracingInvocationReorderModeNV(u32)
);
pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_NONE_NV:
  VkRayTracingInvocationReorderModeNV = VkRayTracingInvocationReorderModeNV(0);
pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_REORDER_NV:
  VkRayTracingInvocationReorderModeNV = VkRayTracingInvocationReorderModeNV(1);
impl core::fmt::Debug for VkRayTracingInvocationReorderModeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_RAY_TRACING_INVOCATION_REORDER_MODE_NONE_NV => {
        "VK_RAY_TRACING_INVOCATION_REORDER_MODE_NONE_NV"
      }
      VK_RAY_TRACING_INVOCATION_REORDER_MODE_REORDER_NV => {
        "VK_RAY_TRACING_INVOCATION_REORDER_MODE_REORDER_NV"
      }
      other => return write!(f, "VkRayTracingInvocationReorderModeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkRayTracingShaderGroupTypeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html) (enumeration)
  VkRayTracingShaderGroupTypeKHR(u32)
);
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR: VkRayTracingShaderGroupTypeKHR =
  VkRayTracingShaderGroupTypeKHR(0);
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR:
  VkRayTracingShaderGroupTypeKHR = VkRayTracingShaderGroupTypeKHR(2);
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR:
  VkRayTracingShaderGroupTypeKHR = VkRayTracingShaderGroupTypeKHR(1);
/// Alias of [`VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR`]
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV: VkRayTracingShaderGroupTypeKHR =
  VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR;
/// Alias of [`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`]
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV:
  VkRayTracingShaderGroupTypeKHR =
  VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR;
/// Alias of [`VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR`]
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV:
  VkRayTracingShaderGroupTypeKHR =
  VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR;
impl core::fmt::Debug for VkRayTracingShaderGroupTypeKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR => {
        "VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR"
      }
      VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR => {
        "VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR"
      }
      VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR => {
        "VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR"
      }
      other => return write!(f, "VkRayTracingShaderGroupTypeKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkResult](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResult.html) (enumeration)
  VkResult(i32)
);
pub const VK_ERROR_COMPRESSION_EXHAUSTED_EXT: VkResult = VkResult(-1000338000);
/// The logical device has been lost. Spec: [5.2.3. Lost Device](https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device).
pub const VK_ERROR_DEVICE_LOST: VkResult = VkResult(-4);
/// Extension specified does not exist
pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = VkResult(-7);
/// Requested feature is not available on this device
pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = VkResult(-8);
/// Requested format is not supported on this device
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = VkResult(-11);
pub const VK_ERROR_FRAGMENTATION: VkResult = VkResult(-1000161000);
/// A requested pool allocation has failed due to fragmentation of the pool's
/// memory
pub const VK_ERROR_FRAGMENTED_POOL: VkResult = VkResult(-12);
pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult = VkResult(-1000255000);
pub const VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: VkResult = VkResult(-1000023000);
pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: VkResult = VkResult(-1000003001);
/// Unable to find a Vulkan driver
pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = VkResult(-9);
/// Initialization of an object has failed
pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = VkResult(-3);
pub const VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: VkResult =
  VkResult(-1000158000);
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: VkResult = VkResult(-1000072003);
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: VkResult = VkResult(-1000257000);
pub const VK_ERROR_INVALID_SHADER_NV: VkResult = VkResult(-1000012000);
pub const VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: VkResult = VkResult(-1000299000);
/// Layer specified does not exist
pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = VkResult(-6);
/// Mapping of a memory object has failed
pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = VkResult(-5);
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = VkResult(-1000000001);
pub const VK_ERROR_NOT_PERMITTED_KHR: VkResult = VkResult(-1000174001);
pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult = VkResult(-1000001004);
/// A device memory allocation has failed
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = VkResult(-2);
/// A host memory allocation has failed
pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = VkResult(-1);
pub const VK_ERROR_OUT_OF_POOL_MEMORY: VkResult = VkResult(-1000069000);
pub const VK_ERROR_SURFACE_LOST_KHR: VkResult = VkResult(-1000000000);
/// Too many objects of the type have already been created
pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = VkResult(-10);
/// An unknown error has occurred, due to an implementation or application bug
pub const VK_ERROR_UNKNOWN: VkResult = VkResult(-13);
pub const VK_ERROR_VALIDATION_FAILED_EXT: VkResult = VkResult(-1000011001);
pub const VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: VkResult =
  VkResult(-1000023001);
pub const VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: VkResult =
  VkResult(-1000023004);
pub const VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: VkResult =
  VkResult(-1000023003);
pub const VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: VkResult =
  VkResult(-1000023002);
pub const VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: VkResult = VkResult(-1000023005);
/// An event is unsignaled
pub const VK_EVENT_RESET: VkResult = VkResult(4);
/// An event is signaled
pub const VK_EVENT_SET: VkResult = VkResult(3);
/// A return array was too small for the result
pub const VK_INCOMPLETE: VkResult = VkResult(5);
/// A fence or query has not yet completed
pub const VK_NOT_READY: VkResult = VkResult(1);
pub const VK_OPERATION_DEFERRED_KHR: VkResult = VkResult(1000268002);
pub const VK_OPERATION_NOT_DEFERRED_KHR: VkResult = VkResult(1000268003);
pub const VK_PIPELINE_COMPILE_REQUIRED: VkResult = VkResult(1000297000);
pub const VK_SUBOPTIMAL_KHR: VkResult = VkResult(1000001003);
/// Command completed successfully
pub const VK_SUCCESS: VkResult = VkResult(0);
pub const VK_THREAD_DONE_KHR: VkResult = VkResult(1000268001);
pub const VK_THREAD_IDLE_KHR: VkResult = VkResult(1000268000);
/// A wait operation has not completed in the specified time
pub const VK_TIMEOUT: VkResult = VkResult(2);
/// Alias of [`VK_ERROR_FRAGMENTATION`]
pub const VK_ERROR_FRAGMENTATION_EXT: VkResult = VK_ERROR_FRAGMENTATION;
/// Alias of [`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`]
pub const VK_ERROR_INVALID_DEVICE_ADDRESS_EXT: VkResult =
  VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
/// Alias of [`VK_ERROR_INVALID_EXTERNAL_HANDLE`]
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR: VkResult =
  VK_ERROR_INVALID_EXTERNAL_HANDLE;
/// Alias of [`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`]
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: VkResult =
  VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
/// Alias of [`VK_ERROR_NOT_PERMITTED_KHR`]
pub const VK_ERROR_NOT_PERMITTED_EXT: VkResult = VK_ERROR_NOT_PERMITTED_KHR;
/// Alias of [`VK_ERROR_OUT_OF_POOL_MEMORY`]
pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR: VkResult = VK_ERROR_OUT_OF_POOL_MEMORY;
/// Alias of [`VK_PIPELINE_COMPILE_REQUIRED`]
pub const VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT: VkResult = VK_PIPELINE_COMPILE_REQUIRED;
/// Alias of [`VK_PIPELINE_COMPILE_REQUIRED`]
pub const VK_PIPELINE_COMPILE_REQUIRED_EXT: VkResult = VK_PIPELINE_COMPILE_REQUIRED;
impl core::fmt::Debug for VkResult {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_ERROR_COMPRESSION_EXHAUSTED_EXT => "VK_ERROR_COMPRESSION_EXHAUSTED_EXT",
      VK_ERROR_DEVICE_LOST => "VK_ERROR_DEVICE_LOST",
      VK_ERROR_EXTENSION_NOT_PRESENT => "VK_ERROR_EXTENSION_NOT_PRESENT",
      VK_ERROR_FEATURE_NOT_PRESENT => "VK_ERROR_FEATURE_NOT_PRESENT",
      VK_ERROR_FORMAT_NOT_SUPPORTED => "VK_ERROR_FORMAT_NOT_SUPPORTED",
      VK_ERROR_FRAGMENTATION => "VK_ERROR_FRAGMENTATION",
      VK_ERROR_FRAGMENTED_POOL => "VK_ERROR_FRAGMENTED_POOL",
      VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => {
        "VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT"
      }
      VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR => "VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR",
      VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => "VK_ERROR_INCOMPATIBLE_DISPLAY_KHR",
      VK_ERROR_INCOMPATIBLE_DRIVER => "VK_ERROR_INCOMPATIBLE_DRIVER",
      VK_ERROR_INITIALIZATION_FAILED => "VK_ERROR_INITIALIZATION_FAILED",
      VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => {
        "VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT"
      }
      VK_ERROR_INVALID_EXTERNAL_HANDLE => "VK_ERROR_INVALID_EXTERNAL_HANDLE",
      VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => {
        "VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS"
      }
      VK_ERROR_INVALID_SHADER_NV => "VK_ERROR_INVALID_SHADER_NV",
      VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR => {
        "VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR"
      }
      VK_ERROR_LAYER_NOT_PRESENT => "VK_ERROR_LAYER_NOT_PRESENT",
      VK_ERROR_MEMORY_MAP_FAILED => "VK_ERROR_MEMORY_MAP_FAILED",
      VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => "VK_ERROR_NATIVE_WINDOW_IN_USE_KHR",
      VK_ERROR_NOT_PERMITTED_KHR => "VK_ERROR_NOT_PERMITTED_KHR",
      VK_ERROR_OUT_OF_DATE_KHR => "VK_ERROR_OUT_OF_DATE_KHR",
      VK_ERROR_OUT_OF_DEVICE_MEMORY => "VK_ERROR_OUT_OF_DEVICE_MEMORY",
      VK_ERROR_OUT_OF_HOST_MEMORY => "VK_ERROR_OUT_OF_HOST_MEMORY",
      VK_ERROR_OUT_OF_POOL_MEMORY => "VK_ERROR_OUT_OF_POOL_MEMORY",
      VK_ERROR_SURFACE_LOST_KHR => "VK_ERROR_SURFACE_LOST_KHR",
      VK_ERROR_TOO_MANY_OBJECTS => "VK_ERROR_TOO_MANY_OBJECTS",
      VK_ERROR_UNKNOWN => "VK_ERROR_UNKNOWN",
      VK_ERROR_VALIDATION_FAILED_EXT => "VK_ERROR_VALIDATION_FAILED_EXT",
      VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR => {
        "VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR"
      }
      VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR => {
        "VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR"
      }
      VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR => {
        "VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR"
      }
      VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR => {
        "VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR"
      }
      VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR => {
        "VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR"
      }
      VK_EVENT_RESET => "VK_EVENT_RESET",
      VK_EVENT_SET => "VK_EVENT_SET",
      VK_INCOMPLETE => "VK_INCOMPLETE",
      VK_NOT_READY => "VK_NOT_READY",
      VK_OPERATION_DEFERRED_KHR => "VK_OPERATION_DEFERRED_KHR",
      VK_OPERATION_NOT_DEFERRED_KHR => "VK_OPERATION_NOT_DEFERRED_KHR",
      VK_PIPELINE_COMPILE_REQUIRED => "VK_PIPELINE_COMPILE_REQUIRED",
      VK_SUBOPTIMAL_KHR => "VK_SUBOPTIMAL_KHR",
      VK_SUCCESS => "VK_SUCCESS",
      VK_THREAD_DONE_KHR => "VK_THREAD_DONE_KHR",
      VK_THREAD_IDLE_KHR => "VK_THREAD_IDLE_KHR",
      VK_TIMEOUT => "VK_TIMEOUT",
      other => return write!(f, "VkResult({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSamplerAddressMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerAddressMode.html) (enumeration)
  VkSamplerAddressMode(u32)
);
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: VkSamplerAddressMode =
  VkSamplerAddressMode(3);
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: VkSamplerAddressMode =
  VkSamplerAddressMode(2);
pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: VkSamplerAddressMode =
  VkSamplerAddressMode(1);
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE: VkSamplerAddressMode =
  VkSamplerAddressMode(4);
pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: VkSamplerAddressMode = VkSamplerAddressMode(0);
/// Alias of [`VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE`]
///
/// Introduced for consistency with extension suffixing rules
#[deprecated = "aliased"]
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE_KHR: VkSamplerAddressMode =
  VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE;
impl core::fmt::Debug for VkSamplerAddressMode {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER => {
        "VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER"
      }
      VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE => "VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE",
      VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT => {
        "VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT"
      }
      VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE => {
        "VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE"
      }
      VK_SAMPLER_ADDRESS_MODE_REPEAT => "VK_SAMPLER_ADDRESS_MODE_REPEAT",
      other => return write!(f, "VkSamplerAddressMode({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSamplerMipmapMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerMipmapMode.html) (enumeration)
  VkSamplerMipmapMode(u32)
);
/// Linear filter between mip levels
pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: VkSamplerMipmapMode = VkSamplerMipmapMode(1);
/// Choose nearest mip level
pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: VkSamplerMipmapMode = VkSamplerMipmapMode(0);
impl core::fmt::Debug for VkSamplerMipmapMode {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SAMPLER_MIPMAP_MODE_LINEAR => "VK_SAMPLER_MIPMAP_MODE_LINEAR",
      VK_SAMPLER_MIPMAP_MODE_NEAREST => "VK_SAMPLER_MIPMAP_MODE_NEAREST",
      other => return write!(f, "VkSamplerMipmapMode({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSamplerReductionMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionMode.html) (enumeration)
  VkSamplerReductionMode(u32)
);
pub const VK_SAMPLER_REDUCTION_MODE_MAX: VkSamplerReductionMode =
  VkSamplerReductionMode(2);
pub const VK_SAMPLER_REDUCTION_MODE_MIN: VkSamplerReductionMode =
  VkSamplerReductionMode(1);
pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: VkSamplerReductionMode =
  VkSamplerReductionMode(0);
/// Alias of [`VK_SAMPLER_REDUCTION_MODE_MAX`]
pub const VK_SAMPLER_REDUCTION_MODE_MAX_EXT: VkSamplerReductionMode =
  VK_SAMPLER_REDUCTION_MODE_MAX;
/// Alias of [`VK_SAMPLER_REDUCTION_MODE_MIN`]
pub const VK_SAMPLER_REDUCTION_MODE_MIN_EXT: VkSamplerReductionMode =
  VK_SAMPLER_REDUCTION_MODE_MIN;
/// Alias of [`VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE`]
pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT: VkSamplerReductionMode =
  VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE;
impl core::fmt::Debug for VkSamplerReductionMode {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SAMPLER_REDUCTION_MODE_MAX => "VK_SAMPLER_REDUCTION_MODE_MAX",
      VK_SAMPLER_REDUCTION_MODE_MIN => "VK_SAMPLER_REDUCTION_MODE_MIN",
      VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE => {
        "VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE"
      }
      other => return write!(f, "VkSamplerReductionMode({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSamplerYcbcrModelConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrModelConversion.html) (enumeration)
  VkSamplerYcbcrModelConversion(u32)
);
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: VkSamplerYcbcrModelConversion =
  VkSamplerYcbcrModelConversion(0);
/// aka UHD YUV
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: VkSamplerYcbcrModelConversion =
  VkSamplerYcbcrModelConversion(4);
/// aka SD YUV
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: VkSamplerYcbcrModelConversion =
  VkSamplerYcbcrModelConversion(3);
/// aka HD YUV
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: VkSamplerYcbcrModelConversion =
  VkSamplerYcbcrModelConversion(2);
/// just range expansion
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY:
  VkSamplerYcbcrModelConversion = VkSamplerYcbcrModelConversion(1);
/// Alias of [`VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY`]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR:
  VkSamplerYcbcrModelConversion = VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY;
/// Alias of [`VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020`]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR:
  VkSamplerYcbcrModelConversion = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020;
/// Alias of [`VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601`]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR: VkSamplerYcbcrModelConversion =
  VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601;
/// Alias of [`VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709`]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR: VkSamplerYcbcrModelConversion =
  VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709;
/// Alias of [`VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY`]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR:
  VkSamplerYcbcrModelConversion = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY;
impl core::fmt::Debug for VkSamplerYcbcrModelConversion {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY => {
        "VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY"
      }
      VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020 => {
        "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020"
      }
      VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601 => {
        "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601"
      }
      VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709 => {
        "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709"
      }
      VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY => {
        "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY"
      }
      other => return write!(f, "VkSamplerYcbcrModelConversion({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSamplerYcbcrRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrRange.html) (enumeration)
  VkSamplerYcbcrRange(u32)
);
/// Luma 0..1 maps to 0..255, chroma -0.5..0.5 to 1..255 (clamped)
pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL: VkSamplerYcbcrRange = VkSamplerYcbcrRange(0);
/// Luma 0..1 maps to 16..235, chroma -0.5..0.5 to 16..240
pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW: VkSamplerYcbcrRange = VkSamplerYcbcrRange(1);
/// Alias of [`VK_SAMPLER_YCBCR_RANGE_ITU_FULL`]
pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR: VkSamplerYcbcrRange =
  VK_SAMPLER_YCBCR_RANGE_ITU_FULL;
/// Alias of [`VK_SAMPLER_YCBCR_RANGE_ITU_NARROW`]
pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR: VkSamplerYcbcrRange =
  VK_SAMPLER_YCBCR_RANGE_ITU_NARROW;
impl core::fmt::Debug for VkSamplerYcbcrRange {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SAMPLER_YCBCR_RANGE_ITU_FULL => "VK_SAMPLER_YCBCR_RANGE_ITU_FULL",
      VK_SAMPLER_YCBCR_RANGE_ITU_NARROW => "VK_SAMPLER_YCBCR_RANGE_ITU_NARROW",
      other => return write!(f, "VkSamplerYcbcrRange({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSciSyncClientTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSciSyncClientTypeNV.html) (enumeration)
  VkSciSyncClientTypeNV(u32)
);
pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_NV: VkSciSyncClientTypeNV =
  VkSciSyncClientTypeNV(0);
pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_WAITER_NV: VkSciSyncClientTypeNV =
  VkSciSyncClientTypeNV(2);
pub const VK_SCI_SYNC_CLIENT_TYPE_WAITER_NV: VkSciSyncClientTypeNV =
  VkSciSyncClientTypeNV(1);
impl core::fmt::Debug for VkSciSyncClientTypeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_NV => "VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_NV",
      VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_WAITER_NV => {
        "VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_WAITER_NV"
      }
      VK_SCI_SYNC_CLIENT_TYPE_WAITER_NV => "VK_SCI_SYNC_CLIENT_TYPE_WAITER_NV",
      other => return write!(f, "VkSciSyncClientTypeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSciSyncPrimitiveTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSciSyncPrimitiveTypeNV.html) (enumeration)
  VkSciSyncPrimitiveTypeNV(u32)
);
pub const VK_SCI_SYNC_PRIMITIVE_TYPE_FENCE_NV: VkSciSyncPrimitiveTypeNV =
  VkSciSyncPrimitiveTypeNV(0);
pub const VK_SCI_SYNC_PRIMITIVE_TYPE_SEMAPHORE_NV: VkSciSyncPrimitiveTypeNV =
  VkSciSyncPrimitiveTypeNV(1);
impl core::fmt::Debug for VkSciSyncPrimitiveTypeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SCI_SYNC_PRIMITIVE_TYPE_FENCE_NV => "VK_SCI_SYNC_PRIMITIVE_TYPE_FENCE_NV",
      VK_SCI_SYNC_PRIMITIVE_TYPE_SEMAPHORE_NV => {
        "VK_SCI_SYNC_PRIMITIVE_TYPE_SEMAPHORE_NV"
      }
      other => return write!(f, "VkSciSyncPrimitiveTypeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkScopeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkScopeNV.html) (enumeration)
  VkScopeNV(u32)
);
pub const VK_SCOPE_DEVICE_NV: VkScopeNV = VkScopeNV(1);
pub const VK_SCOPE_QUEUE_FAMILY_NV: VkScopeNV = VkScopeNV(5);
pub const VK_SCOPE_SUBGROUP_NV: VkScopeNV = VkScopeNV(3);
pub const VK_SCOPE_WORKGROUP_NV: VkScopeNV = VkScopeNV(2);
impl core::fmt::Debug for VkScopeNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SCOPE_DEVICE_NV => "VK_SCOPE_DEVICE_NV",
      VK_SCOPE_QUEUE_FAMILY_NV => "VK_SCOPE_QUEUE_FAMILY_NV",
      VK_SCOPE_SUBGROUP_NV => "VK_SCOPE_SUBGROUP_NV",
      VK_SCOPE_WORKGROUP_NV => "VK_SCOPE_WORKGROUP_NV",
      other => return write!(f, "VkScopeNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSemaphoreType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreType.html) (enumeration)
  VkSemaphoreType(u32)
);
pub const VK_SEMAPHORE_TYPE_BINARY: VkSemaphoreType = VkSemaphoreType(0);
pub const VK_SEMAPHORE_TYPE_TIMELINE: VkSemaphoreType = VkSemaphoreType(1);
/// Alias of [`VK_SEMAPHORE_TYPE_BINARY`]
pub const VK_SEMAPHORE_TYPE_BINARY_KHR: VkSemaphoreType = VK_SEMAPHORE_TYPE_BINARY;
/// Alias of [`VK_SEMAPHORE_TYPE_TIMELINE`]
pub const VK_SEMAPHORE_TYPE_TIMELINE_KHR: VkSemaphoreType = VK_SEMAPHORE_TYPE_TIMELINE;
impl core::fmt::Debug for VkSemaphoreType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SEMAPHORE_TYPE_BINARY => "VK_SEMAPHORE_TYPE_BINARY",
      VK_SEMAPHORE_TYPE_TIMELINE => "VK_SEMAPHORE_TYPE_TIMELINE",
      other => return write!(f, "VkSemaphoreType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkShaderFloatControlsIndependence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderFloatControlsIndependence.html) (enumeration)
  VkShaderFloatControlsIndependence(u32)
);
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY:
  VkShaderFloatControlsIndependence = VkShaderFloatControlsIndependence(0);
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: VkShaderFloatControlsIndependence =
  VkShaderFloatControlsIndependence(1);
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: VkShaderFloatControlsIndependence =
  VkShaderFloatControlsIndependence(2);
/// Alias of [`VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY`]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR:
  VkShaderFloatControlsIndependence = VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY;
/// Alias of [`VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL`]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR:
  VkShaderFloatControlsIndependence = VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL;
/// Alias of [`VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE`]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR:
  VkShaderFloatControlsIndependence = VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE;
impl core::fmt::Debug for VkShaderFloatControlsIndependence {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY => {
        "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY"
      }
      VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL => {
        "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL"
      }
      VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE => {
        "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE"
      }
      other => return write!(f, "VkShaderFloatControlsIndependence({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkShaderGroupShaderKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderGroupShaderKHR.html) (enumeration)
  VkShaderGroupShaderKHR(u32)
);
pub const VK_SHADER_GROUP_SHADER_ANY_HIT_KHR: VkShaderGroupShaderKHR =
  VkShaderGroupShaderKHR(2);
pub const VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR: VkShaderGroupShaderKHR =
  VkShaderGroupShaderKHR(1);
pub const VK_SHADER_GROUP_SHADER_GENERAL_KHR: VkShaderGroupShaderKHR =
  VkShaderGroupShaderKHR(0);
pub const VK_SHADER_GROUP_SHADER_INTERSECTION_KHR: VkShaderGroupShaderKHR =
  VkShaderGroupShaderKHR(3);
impl core::fmt::Debug for VkShaderGroupShaderKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SHADER_GROUP_SHADER_ANY_HIT_KHR => "VK_SHADER_GROUP_SHADER_ANY_HIT_KHR",
      VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR => "VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR",
      VK_SHADER_GROUP_SHADER_GENERAL_KHR => "VK_SHADER_GROUP_SHADER_GENERAL_KHR",
      VK_SHADER_GROUP_SHADER_INTERSECTION_KHR => {
        "VK_SHADER_GROUP_SHADER_INTERSECTION_KHR"
      }
      other => return write!(f, "VkShaderGroupShaderKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkShaderInfoTypeAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderInfoTypeAMD.html) (enumeration)
  VkShaderInfoTypeAMD(u32)
);
pub const VK_SHADER_INFO_TYPE_BINARY_AMD: VkShaderInfoTypeAMD = VkShaderInfoTypeAMD(1);
pub const VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD: VkShaderInfoTypeAMD =
  VkShaderInfoTypeAMD(2);
pub const VK_SHADER_INFO_TYPE_STATISTICS_AMD: VkShaderInfoTypeAMD =
  VkShaderInfoTypeAMD(0);
impl core::fmt::Debug for VkShaderInfoTypeAMD {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SHADER_INFO_TYPE_BINARY_AMD => "VK_SHADER_INFO_TYPE_BINARY_AMD",
      VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD => "VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD",
      VK_SHADER_INFO_TYPE_STATISTICS_AMD => "VK_SHADER_INFO_TYPE_STATISTICS_AMD",
      other => return write!(f, "VkShaderInfoTypeAMD({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkShadingRatePaletteEntryNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteEntryNV.html) (enumeration)
  VkShadingRatePaletteEntryNV(u32)
);
pub const VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(1);
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(7);
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(6);
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(8);
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(10);
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(9);
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(11);
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(5);
pub const VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(4);
pub const VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(3);
pub const VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV:
  VkShadingRatePaletteEntryNV = VkShadingRatePaletteEntryNV(2);
pub const VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV: VkShadingRatePaletteEntryNV =
  VkShadingRatePaletteEntryNV(0);
impl core::fmt::Debug for VkShadingRatePaletteEntryNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV"
      }
      VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV => {
        "VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV"
      }
      other => return write!(f, "VkShadingRatePaletteEntryNV({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSharingMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html) (enumeration)
  VkSharingMode(u32)
);
pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = VkSharingMode(1);
pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = VkSharingMode(0);
impl core::fmt::Debug for VkSharingMode {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SHARING_MODE_CONCURRENT => "VK_SHARING_MODE_CONCURRENT",
      VK_SHARING_MODE_EXCLUSIVE => "VK_SHARING_MODE_EXCLUSIVE",
      other => return write!(f, "VkSharingMode({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkStencilOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilOp.html) (enumeration)
  VkStencilOp(u32)
);
pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: VkStencilOp = VkStencilOp(4);
pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: VkStencilOp = VkStencilOp(7);
pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: VkStencilOp = VkStencilOp(3);
pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: VkStencilOp = VkStencilOp(6);
pub const VK_STENCIL_OP_INVERT: VkStencilOp = VkStencilOp(5);
pub const VK_STENCIL_OP_KEEP: VkStencilOp = VkStencilOp(0);
pub const VK_STENCIL_OP_REPLACE: VkStencilOp = VkStencilOp(2);
pub const VK_STENCIL_OP_ZERO: VkStencilOp = VkStencilOp(1);
impl core::fmt::Debug for VkStencilOp {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_STENCIL_OP_DECREMENT_AND_CLAMP => "VK_STENCIL_OP_DECREMENT_AND_CLAMP",
      VK_STENCIL_OP_DECREMENT_AND_WRAP => "VK_STENCIL_OP_DECREMENT_AND_WRAP",
      VK_STENCIL_OP_INCREMENT_AND_CLAMP => "VK_STENCIL_OP_INCREMENT_AND_CLAMP",
      VK_STENCIL_OP_INCREMENT_AND_WRAP => "VK_STENCIL_OP_INCREMENT_AND_WRAP",
      VK_STENCIL_OP_INVERT => "VK_STENCIL_OP_INVERT",
      VK_STENCIL_OP_KEEP => "VK_STENCIL_OP_KEEP",
      VK_STENCIL_OP_REPLACE => "VK_STENCIL_OP_REPLACE",
      VK_STENCIL_OP_ZERO => "VK_STENCIL_OP_ZERO",
      other => return write!(f, "VkStencilOp({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkStructureType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStructureType.html) (enumeration)
  VkStructureType(u32)
);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR:
  VkStructureType = VkStructureType(1000150000);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: VkStructureType =
  VkStructureType(1000150020);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT:
  VkStructureType = VkStructureType(1000316009);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000150017);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000165001);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR:
  VkStructureType = VkStructureType(1000150002);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR:
  VkStructureType = VkStructureType(1000150003);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR:
  VkStructureType = VkStructureType(1000150004);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR: VkStructureType =
  VkStructureType(1000150006);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV:
  VkStructureType = VkStructureType(1000327000);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR:
  VkStructureType = VkStructureType(1000150005);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV: VkStructureType =
  VkStructureType(1000165012);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV:
  VkStructureType = VkStructureType(1000165008);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV: VkStructureType =
  VkStructureType(1000327002);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT:
  VkStructureType = VkStructureType(1000396009);
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR: VkStructureType =
  VkStructureType(1000150009);
pub const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR: VkStructureType =
  VkStructureType(1000060010);
pub const VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR: VkStructureType =
  VkStructureType(1000116004);
pub const VK_STRUCTURE_TYPE_AMIGO_PROFILING_SUBMIT_INFO_SEC: VkStructureType =
  VkStructureType(1000485001);
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID:
  VkStructureType = VkStructureType(1000129006);
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID:
  VkStructureType = VkStructureType(1000129002);
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: VkStructureType =
  VkStructureType(1000129001);
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: VkStructureType =
  VkStructureType(1000129000);
pub const VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000008000);
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = VkStructureType(0);
pub const VK_STRUCTURE_TYPE_APPLICATION_PARAMETERS_EXT: VkStructureType =
  VkStructureType(1000435000);
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2: VkStructureType =
  VkStructureType(1000109000);
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: VkStructureType =
  VkStructureType(1000241002);
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2: VkStructureType =
  VkStructureType(1000109001);
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT: VkStructureType =
  VkStructureType(1000241001);
pub const VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD: VkStructureType =
  VkStructureType(1000044008);
pub const VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: VkStructureType =
  VkStructureType(1000165006);
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: VkStructureType =
  VkStructureType(1000060013);
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO: VkStructureType =
  VkStructureType(1000157000);
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: VkStructureType =
  VkStructureType(1000060014);
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO: VkStructureType =
  VkStructureType(1000157001);
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: VkStructureType =
  VkStructureType(1000060009);
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO: VkStructureType =
  VkStructureType(1000156002);
pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: VkStructureType = VkStructureType(7);
pub const VK_STRUCTURE_TYPE_BIND_VIDEO_SESSION_MEMORY_INFO_KHR: VkStructureType =
  VkStructureType(1000023004);
pub const VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2: VkStructureType =
  VkStructureType(1000337004);
pub const VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType =
  VkStructureType(1000316005);
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA:
  VkStructureType = VkStructureType(1000366005);
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000366009);
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000366000);
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000366002);
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA: VkStructureType =
  VkStructureType(1000366003);
pub const VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000366004);
pub const VK_STRUCTURE_TYPE_BUFFER_COPY_2: VkStructureType = VkStructureType(1000337006);
pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = VkStructureType(12);
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000244002);
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO: VkStructureType =
  VkStructureType(1000244001);
pub const VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2: VkStructureType =
  VkStructureType(1000337009);
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: VkStructureType = VkStructureType(44);
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2: VkStructureType =
  VkStructureType(1000314001);
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2: VkStructureType =
  VkStructureType(1000146000);
pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: VkStructureType =
  VkStructureType(1000257002);
pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: VkStructureType =
  VkStructureType(13);
pub const VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT: VkStructureType =
  VkStructureType(1000184000);
pub const VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV: VkStructureType =
  VkStructureType(1000314009);
pub const VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV: VkStructureType =
  VkStructureType(1000206000);
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType =
  VkStructureType(40);
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType =
  VkStructureType(42);
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT:
  VkStructureType = VkStructureType(1000081000);
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: VkStructureType =
  VkStructureType(41);
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: VkStructureType =
  VkStructureType(1000044004);
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM:
  VkStructureType = VkStructureType(1000282000);
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV:
  VkStructureType = VkStructureType(1000278001);
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO: VkStructureType =
  VkStructureType(1000314006);
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType =
  VkStructureType(39);
pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType =
  VkStructureType(29);
pub const VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT: VkStructureType =
  VkStructureType(1000081002);
pub const VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV: VkStructureType =
  VkStructureType(1000249001);
pub const VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR: VkStructureType =
  VkStructureType(1000150010);
pub const VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR:
  VkStructureType = VkStructureType(1000150011);
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2: VkStructureType =
  VkStructureType(1000337000);
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2: VkStructureType =
  VkStructureType(1000337002);
pub const VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM: VkStructureType =
  VkStructureType(1000333000);
pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: VkStructureType = VkStructureType(36);
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2: VkStructureType =
  VkStructureType(1000337001);
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2: VkStructureType =
  VkStructureType(1000337003);
pub const VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR:
  VkStructureType = VkStructureType(1000150012);
pub const VK_STRUCTURE_TYPE_COPY_MEMORY_TO_MICROMAP_INFO_EXT: VkStructureType =
  VkStructureType(1000396004);
pub const VK_STRUCTURE_TYPE_COPY_MICROMAP_INFO_EXT: VkStructureType =
  VkStructureType(1000396002);
pub const VK_STRUCTURE_TYPE_COPY_MICROMAP_TO_MEMORY_INFO_EXT: VkStructureType =
  VkStructureType(1000396003);
pub const VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX: VkStructureType =
  VkStructureType(1000029001);
pub const VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX: VkStructureType =
  VkStructureType(1000029002);
pub const VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX: VkStructureType =
  VkStructureType(1000029000);
pub const VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR: VkStructureType =
  VkStructureType(1000078002);
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT: VkStructureType =
  VkStructureType(1000022002);
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT: VkStructureType =
  VkStructureType(1000022000);
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT: VkStructureType =
  VkStructureType(1000022001);
pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000011000);
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT: VkStructureType =
  VkStructureType(1000128002);
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: VkStructureType =
  VkStructureType(1000128003);
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000128004);
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT: VkStructureType =
  VkStructureType(1000128000);
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT: VkStructureType =
  VkStructureType(1000128001);
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000026001);
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000026000);
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV:
  VkStructureType = VkStructureType(1000026002);
pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO: VkStructureType =
  VkStructureType(1000314003);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT: VkStructureType =
  VkStructureType(1000316003);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT: VkStructureType =
  VkStructureType(1000316011);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: VkStructureType = VkStructureType(1000316012);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT: VkStructureType =
  VkStructureType(1000316004);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType =
  VkStructureType(33);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO:
  VkStructureType = VkStructureType(1000138003);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType =
  VkStructureType(34);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: VkStructureType =
  VkStructureType(1000420001);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO:
  VkStructureType = VkStructureType(1000161000);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType =
  VkStructureType(32);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE:
  VkStructureType = VkStructureType(1000420002);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT: VkStructureType =
  VkStructureType(1000168001);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO:
  VkStructureType = VkStructureType(1000161003);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT:
  VkStructureType = VkStructureType(1000161004);
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: VkStructureType =
  VkStructureType(1000085000);
pub const VK_STRUCTURE_TYPE_DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT: VkStructureType =
  VkStructureType(1000354001);
pub const VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS: VkStructureType =
  VkStructureType(1000413002);
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = VkStructureType(3);
pub const VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000284001);
pub const VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000300001);
pub const VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT: VkStructureType =
  VkStructureType(1000091001);
pub const VK_STRUCTURE_TYPE_DEVICE_FAULT_COUNTS_EXT: VkStructureType =
  VkStructureType(1000341001);
pub const VK_STRUCTURE_TYPE_DEVICE_FAULT_INFO_EXT: VkStructureType =
  VkStructureType(1000341002);
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO: VkStructureType =
  VkStructureType(1000060006);
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: VkStructureType =
  VkStructureType(1000060004);
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO: VkStructureType =
  VkStructureType(1000070001);
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: VkStructureType =
  VkStructureType(1000060007);
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR: VkStructureType =
  VkStructureType(1000060011);
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: VkStructureType =
  VkStructureType(1000060003);
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO: VkStructureType =
  VkStructureType(1000060005);
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000060012);
pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS: VkStructureType =
  VkStructureType(1000413003);
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: VkStructureType =
  VkStructureType(1000257004);
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD:
  VkStructureType = VkStructureType(1000189000);
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: VkStructureType =
  VkStructureType(1000284002);
pub const VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO: VkStructureType =
  VkStructureType(1000295001);
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType =
  VkStructureType(2);
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR:
  VkStructureType = VkStructureType(1000174000);
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2: VkStructureType =
  VkStructureType(1000145003);
pub const VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000346000);
pub const VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_INFO_LUNARG: VkStructureType =
  VkStructureType(1000459000);
pub const VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_LIST_LUNARG: VkStructureType =
  VkStructureType(1000459001);
pub const VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT: VkStructureType =
  VkStructureType(1000091002);
pub const VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000002000);
pub const VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR: VkStructureType =
  VkStructureType(1000121002);
pub const VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: VkStructureType =
  VkStructureType(1000213000);
pub const VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR: VkStructureType =
  VkStructureType(1000121004);
pub const VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR: VkStructureType =
  VkStructureType(1000121003);
pub const VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR: VkStructureType =
  VkStructureType(1000121001);
pub const VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT: VkStructureType =
  VkStructureType(1000091000);
pub const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR: VkStructureType =
  VkStructureType(1000003000);
pub const VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR: VkStructureType =
  VkStructureType(1000121000);
pub const VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000002001);
pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: VkStructureType =
  VkStructureType(1000158006);
pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: VkStructureType =
  VkStructureType(1000158000);
pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: VkStructureType = VkStructureType(10);
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO: VkStructureType =
  VkStructureType(1000113000);
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_SCI_SYNC_INFO_NV: VkStructureType =
  VkStructureType(1000373001);
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType =
  VkStructureType(1000114001);
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO: VkStructureType =
  VkStructureType(1000072002);
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV: VkStructureType =
  VkStructureType(1000056001);
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_SCI_BUF_INFO_NV: VkStructureType =
  VkStructureType(1000374001);
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType =
  VkStructureType(1000073001);
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType =
  VkStructureType(1000057001);
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT: VkStructureType =
  VkStructureType(1000311004);
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: VkStructureType =
  VkStructureType(1000311003);
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT: VkStructureType =
  VkStructureType(1000311002);
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT: VkStructureType =
  VkStructureType(1000311008);
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT: VkStructureType =
  VkStructureType(1000311001);
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000311000);
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT: VkStructureType =
  VkStructureType(1000311010);
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT: VkStructureType =
  VkStructureType(1000311006);
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO: VkStructureType =
  VkStructureType(1000077000);
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_SCI_SYNC_INFO_NV: VkStructureType =
  VkStructureType(1000373005);
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType =
  VkStructureType(1000078001);
pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES: VkStructureType =
  VkStructureType(1000071003);
pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES: VkStructureType =
  VkStructureType(1000112001);
pub const VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID: VkStructureType =
  VkStructureType(1000129005);
pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES: VkStructureType =
  VkStructureType(1000071001);
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO: VkStructureType =
  VkStructureType(1000072000);
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO: VkStructureType =
  VkStructureType(1000072001);
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000056000);
pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES: VkStructureType =
  VkStructureType(1000076001);
pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = VkStructureType(8);
pub const VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR: VkStructureType =
  VkStructureType(1000115001);
pub const VK_STRUCTURE_TYPE_FENCE_GET_SCI_SYNC_INFO_NV: VkStructureType =
  VkStructureType(1000373002);
pub const VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType =
  VkStructureType(1000114002);
pub const VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000170001);
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2: VkStructureType =
  VkStructureType(1000059002);
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3: VkStructureType =
  VkStructureType(1000360000);
pub const VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: VkStructureType =
  VkStructureType(1000226000);
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: VkStructureType =
  VkStructureType(1000108001);
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: VkStructureType =
  VkStructureType(1000108002);
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: VkStructureType =
  VkStructureType(37);
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: VkStructureType =
  VkStructureType(1000250002);
pub const VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV: VkStructureType =
  VkStructureType(1000277005);
pub const VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV:
  VkStructureType = VkStructureType(1000277006);
pub const VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV: VkStructureType =
  VkStructureType(1000165005);
pub const VK_STRUCTURE_TYPE_GEOMETRY_NV: VkStructureType = VkStructureType(1000165003);
pub const VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV: VkStructureType =
  VkStructureType(1000165004);
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType =
  VkStructureType(28);
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000320002);
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000277002);
pub const VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000277001);
pub const VK_STRUCTURE_TYPE_HDR_METADATA_EXT: VkStructureType =
  VkStructureType(1000105000);
pub const VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000256000);
pub const VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000214000);
pub const VK_STRUCTURE_TYPE_IMAGE_BLIT_2: VkStructureType = VkStructureType(1000337008);
pub const VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType =
  VkStructureType(1000316006);
pub const VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_CONTROL_EXT: VkStructureType =
  VkStructureType(1000338001);
pub const VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_PROPERTIES_EXT: VkStructureType =
  VkStructureType(1000338004);
pub const VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000366006);
pub const VK_STRUCTURE_TYPE_IMAGE_COPY_2: VkStructureType = VkStructureType(1000337007);
pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: VkStructureType = VkStructureType(14);
pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000158004);
pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000158003);
pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: VkStructureType =
  VkStructureType(1000158005);
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000366007);
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO: VkStructureType =
  VkStructureType(1000147000);
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType =
  VkStructureType(1000059003);
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: VkStructureType = VkStructureType(45);
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2: VkStructureType =
  VkStructureType(1000314002);
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType =
  VkStructureType(1000146001);
pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: VkStructureType =
  VkStructureType(1000156003);
pub const VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2: VkStructureType =
  VkStructureType(1000337010);
pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType =
  VkStructureType(1000146002);
pub const VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO: VkStructureType =
  VkStructureType(1000246000);
pub const VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT: VkStructureType =
  VkStructureType(1000338003);
pub const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000060008);
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: VkStructureType =
  VkStructureType(1000030001);
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT: VkStructureType =
  VkStructureType(1000067000);
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType =
  VkStructureType(1000316007);
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = VkStructureType(15);
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX: VkStructureType =
  VkStructureType(1000030000);
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000391001);
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM: VkStructureType =
  VkStructureType(1000440002);
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_SLICED_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000418001);
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO: VkStructureType =
  VkStructureType(1000117002);
pub const VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: VkStructureType =
  VkStructureType(1000129003);
pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR: VkStructureType =
  VkStructureType(1000115000);
pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_SCI_SYNC_INFO_NV: VkStructureType =
  VkStructureType(1000373000);
pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType =
  VkStructureType(1000114000);
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: VkStructureType =
  VkStructureType(1000366001);
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR: VkStructureType =
  VkStructureType(1000074000);
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT: VkStructureType =
  VkStructureType(1000178000);
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_SCI_BUF_INFO_NV: VkStructureType =
  VkStructureType(1000374000);
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType =
  VkStructureType(1000073000);
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType =
  VkStructureType(1000057000);
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000364000);
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT: VkStructureType =
  VkStructureType(1000311005);
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT: VkStructureType =
  VkStructureType(1000311009);
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT: VkStructureType =
  VkStructureType(1000311011);
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT: VkStructureType =
  VkStructureType(1000311007);
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR: VkStructureType =
  VkStructureType(1000079000);
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV: VkStructureType =
  VkStructureType(1000373004);
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType =
  VkStructureType(1000078000);
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000365000);
pub const VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000277004);
pub const VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: VkStructureType =
  VkStructureType(1000277003);
pub const VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL: VkStructureType =
  VkStructureType(1000210001);
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = VkStructureType(1);
pub const VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK: VkStructureType =
  VkStructureType(1000122000);
/// Reserved for internal use by the loader, layers, and ICDs
pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: VkStructureType =
  VkStructureType(48);
/// Reserved for internal use by the loader, layers, and ICDs
pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: VkStructureType =
  VkStructureType(47);
pub const VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK: VkStructureType =
  VkStructureType(1000123000);
pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: VkStructureType = VkStructureType(6);
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO: VkStructureType =
  VkStructureType(1000060000);
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = VkStructureType(5);
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: VkStructureType = VkStructureType(46);
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2: VkStructureType =
  VkStructureType(1000314000);
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO: VkStructureType =
  VkStructureType(1000127001);
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS: VkStructureType =
  VkStructureType(1000127000);
pub const VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR: VkStructureType =
  VkStructureType(1000074001);
pub const VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID:
  VkStructureType = VkStructureType(1000129004);
pub const VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR: VkStructureType =
  VkStructureType(1000074002);
pub const VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV: VkStructureType =
  VkStructureType(1000371000);
pub const VK_STRUCTURE_TYPE_MEMORY_GET_SCI_BUF_INFO_NV: VkStructureType =
  VkStructureType(1000374002);
pub const VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR: VkStructureType =
  VkStructureType(1000073003);
pub const VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000364002);
pub const VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT: VkStructureType =
  VkStructureType(1000178001);
pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: VkStructureType =
  VkStructureType(1000257003);
pub const VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT: VkStructureType =
  VkStructureType(1000238001);
pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2: VkStructureType =
  VkStructureType(1000146003);
pub const VK_STRUCTURE_TYPE_MEMORY_SCI_BUF_PROPERTIES_NV: VkStructureType =
  VkStructureType(1000374003);
pub const VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR: VkStructureType =
  VkStructureType(1000073002);
pub const VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: VkStructureType =
  VkStructureType(1000364001);
pub const VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000217000);
pub const VK_STRUCTURE_TYPE_MICROMAP_BUILD_INFO_EXT: VkStructureType =
  VkStructureType(1000396000);
pub const VK_STRUCTURE_TYPE_MICROMAP_BUILD_SIZES_INFO_EXT: VkStructureType =
  VkStructureType(1000396008);
pub const VK_STRUCTURE_TYPE_MICROMAP_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000396007);
pub const VK_STRUCTURE_TYPE_MICROMAP_VERSION_INFO_EXT: VkStructureType =
  VkStructureType(1000396001);
pub const VK_STRUCTURE_TYPE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT:
  VkStructureType = VkStructureType(1000376002);
pub const VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT: VkStructureType =
  VkStructureType(1000143004);
pub const VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: VkStructureType =
  VkStructureType(1000044009);
pub const VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM: VkStructureType = VkStructureType(1000510001);
pub const VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000351002);
pub const VK_STRUCTURE_TYPE_NATIVE_BUFFER_ANDROID: VkStructureType =
  VkStructureType(1000010000);
pub const VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000316010);
pub const VK_STRUCTURE_TYPE_OPTICAL_FLOW_EXECUTE_INFO_NV: VkStructureType =
  VkStructureType(1000464005);
pub const VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV: VkStructureType =
  VkStructureType(1000464002);
pub const VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV: VkStructureType =
  VkStructureType(1000464003);
pub const VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000464004);
pub const VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV:
  VkStructureType = VkStructureType(1000464010);
pub const VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL:
  VkStructureType = VkStructureType(1000210005);
pub const VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR: VkStructureType =
  VkStructureType(1000116006);
pub const VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR: VkStructureType =
  VkStructureType(1000116005);
pub const VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL: VkStructureType =
  VkStructureType(1000210002);
pub const VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL: VkStructureType =
  VkStructureType(1000210004);
pub const VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR: VkStructureType =
  VkStructureType(1000116003);
pub const VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL: VkStructureType =
  VkStructureType(1000210003);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: VkStructureType =
  VkStructureType(1000083000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: VkStructureType =
  VkStructureType(1000340000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: VkStructureType =
  VkStructureType(1000177000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR:
  VkStructureType = VkStructureType(1000150013);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR:
  VkStructureType = VkStructureType(1000150014);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT:
  VkStructureType = VkStructureType(1000354000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC:
  VkStructureType = VkStructureType(1000485000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: VkStructureType =
  VkStructureType(1000067001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT: VkStructureType = VkStructureType(1000339000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT:
  VkStructureType = VkStructureType(1000148000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000148001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT:
  VkStructureType = VkStructureType(1000411000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES:
  VkStructureType = VkStructureType(1000257000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT:
  VkStructureType = VkStructureType(1000244000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI:
  VkStructureType = VkStructureType(1000404000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI:
  VkStructureType = VkStructureType(1000404001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD:
  VkStructureType = VkStructureType(1000229000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT:
  VkStructureType = VkStructureType(1000381000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV:
  VkStructureType = VkStructureType(1000201000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT:
  VkStructureType = VkStructureType(1000081001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000101000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV:
  VkStructureType = VkStructureType(1000249000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV:
  VkStructureType = VkStructureType(1000249002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV:
  VkStructureType = VkStructureType(1000426000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV:
  VkStructureType = VkStructureType(1000426001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV:
  VkStructureType = VkStructureType(1000050000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV:
  VkStructureType = VkStructureType(1000250000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT:
  VkStructureType = VkStructureType(1000287002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000287001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: VkStructureType = VkStructureType(1000240000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT:
  VkStructureType = VkStructureType(1000421000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT:
  VkStructureType = VkStructureType(1000355000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT:
  VkStructureType = VkStructureType(1000102000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES:
  VkStructureType = VkStructureType(1000199000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: VkStructureType = VkStructureType(1000316001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT:
  VkStructureType = VkStructureType(1000316002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000316000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES:
  VkStructureType = VkStructureType(1000161001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES:
  VkStructureType = VkStructureType(1000161002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE:
  VkStructureType = VkStructureType(1000420000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV:
  VkStructureType = VkStructureType(1000277007);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV:
  VkStructureType = VkStructureType(1000277000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT:
  VkStructureType = VkStructureType(1000284000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV:
  VkStructureType = VkStructureType(1000300000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000099000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES: VkStructureType =
  VkStructureType(1000196000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: VkStructureType =
  VkStructureType(1000353000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: VkStructureType =
  VkStructureType(1000044003);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV:
  VkStructureType = VkStructureType(1000205002);
/// Not promoted to 1.3
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT:
  VkStructureType = VkStructureType(1000377000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT:
  VkStructureType = VkStructureType(1000455000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000455001);
/// Not promoted to 1.3
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT:
  VkStructureType = VkStructureType(1000267000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: VkStructureType =
  VkStructureType(1000071002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: VkStructureType =
  VkStructureType(1000112000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: VkStructureType =
  VkStructureType(1000071000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000178002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV:
  VkStructureType = VkStructureType(1000371001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV:
  VkStructureType = VkStructureType(1000374004);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV:
  VkStructureType = VkStructureType(1000489002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV:
  VkStructureType = VkStructureType(1000373007);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: VkStructureType =
  VkStructureType(1000076000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FAULT_FEATURES_EXT: VkStructureType =
  VkStructureType(1000341000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2: VkStructureType =
  VkStructureType(1000059000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: VkStructureType =
  VkStructureType(1000197000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT:
  VkStructureType = VkStructureType(1000332000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000332001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT:
  VkStructureType = VkStructureType(1000218000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM:
  VkStructureType = VkStructureType(1000425000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: VkStructureType = VkStructureType(1000425001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000218001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR:
  VkStructureType = VkStructureType(1000203000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR:
  VkStructureType = VkStructureType(1000322000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT:
  VkStructureType = VkStructureType(1000251000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV:
  VkStructureType = VkStructureType(1000326001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV:
  VkStructureType = VkStructureType(1000326000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR:
  VkStructureType = VkStructureType(1000226003);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: VkStructureType =
  VkStructureType(1000226004);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR:
  VkStructureType = VkStructureType(1000226002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR:
  VkStructureType = VkStructureType(1000388000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT:
  VkStructureType = VkStructureType(1000320000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000320001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES: VkStructureType =
  VkStructureType(1000070000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: VkStructureType =
  VkStructureType(1000261000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES: VkStructureType =
  VkStructureType(1000071004);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES:
  VkStructureType = VkStructureType(1000108000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT:
  VkStructureType = VkStructureType(1000393000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT:
  VkStructureType = VkStructureType(1000338000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: VkStructureType = VkStructureType(1000437000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT:
  VkStructureType = VkStructureType(1000158002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: VkStructureType =
  VkStructureType(1000059004);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM:
  VkStructureType = VkStructureType(1000440000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM:
  VkStructureType = VkStructureType(1000440001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: VkStructureType =
  VkStructureType(1000335000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT:
  VkStructureType = VkStructureType(1000418000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT:
  VkStructureType = VkStructureType(1000170000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT:
  VkStructureType = VkStructureType(1000391000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT:
  VkStructureType = VkStructureType(1000265000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV:
  VkStructureType = VkStructureType(1000278000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES:
  VkStructureType = VkStructureType(1000138000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES:
  VkStructureType = VkStructureType(1000138001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI:
  VkStructureType = VkStructureType(1000370000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT:
  VkStructureType = VkStructureType(1000465000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV:
  VkStructureType = VkStructureType(1000430000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT:
  VkStructureType = VkStructureType(1000259000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000259002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: VkStructureType =
  VkStructureType(1000168000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: VkStructureType =
  VkStructureType(1000413000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: VkStructureType =
  VkStructureType(1000413001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000237000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV:
  VkStructureType = VkStructureType(1000427000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV:
  VkStructureType = VkStructureType(1000427001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT:
  VkStructureType = VkStructureType(1000238000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: VkStructureType =
  VkStructureType(1000059006);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT: VkStructureType =
  VkStructureType(1000328000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: VkStructureType =
  VkStructureType(1000202000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT: VkStructureType =
  VkStructureType(1000328001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: VkStructureType =
  VkStructureType(1000202001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: VkStructureType = VkStructureType(1000376000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES: VkStructureType =
  VkStructureType(1000053001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: VkStructureType = VkStructureType(1000097000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM: VkStructureType = VkStructureType(1000510000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM:
  VkStructureType = VkStructureType(1000488000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: VkStructureType =
  VkStructureType(1000053002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: VkStructureType =
  VkStructureType(1000392000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: VkStructureType =
  VkStructureType(1000392001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT:
  VkStructureType = VkStructureType(1000351000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT:
  VkStructureType = VkStructureType(1000422000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT:
  VkStructureType = VkStructureType(1000396005);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000396006);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV: VkStructureType =
  VkStructureType(1000464000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV: VkStructureType =
  VkStructureType(1000464001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT:
  VkStructureType = VkStructureType(1000412000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: VkStructureType =
  VkStructureType(1000212000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR:
  VkStructureType = VkStructureType(1000116000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR:
  VkStructureType = VkStructureType(1000116001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES:
  VkStructureType = VkStructureType(1000297000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: VkStructureType = VkStructureType(1000269000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT: VkStructureType = VkStructureType(1000498000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT:
  VkStructureType = VkStructureType(1000372001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT:
  VkStructureType = VkStructureType(1000466000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT:
  VkStructureType = VkStructureType(1000068001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000068002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: VkStructureType =
  VkStructureType(1000117000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR:
  VkStructureType = VkStructureType(1000163000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR:
  VkStructureType = VkStructureType(1000163001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID:
  VkStructureType = VkStructureType(1000010002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV: VkStructureType =
  VkStructureType(1000292000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: VkStructureType =
  VkStructureType(1000294001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: VkStructureType =
  VkStructureType(1000248000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT:
  VkStructureType = VkStructureType(1000382000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: VkStructureType = VkStructureType(1000356000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: VkStructureType =
  VkStructureType(1000295000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2: VkStructureType =
  VkStructureType(1000059001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: VkStructureType =
  VkStructureType(1000145001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: VkStructureType =
  VkStructureType(1000145002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT:
  VkStructureType = VkStructureType(1000254000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000254002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR:
  VkStructureType = VkStructureType(1000080000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: VkStructureType = VkStructureType(1000342000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: VkStructureType =
  VkStructureType(1000348013);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV:
  VkStructureType = VkStructureType(1000490000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV: VkStructureType = VkStructureType(1000490001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR:
  VkStructureType = VkStructureType(1000386000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV:
  VkStructureType = VkStructureType(1000327001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR:
  VkStructureType = VkStructureType(1000347000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR:
  VkStructureType = VkStructureType(1000347001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: VkStructureType =
  VkStructureType(1000165009);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV:
  VkStructureType = VkStructureType(1000166000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT:
  VkStructureType = VkStructureType(1000344000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: VkStructureType =
  VkStructureType(1000286000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: VkStructureType =
  VkStructureType(1000286001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES:
  VkStructureType = VkStructureType(1000130000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES:
  VkStructureType = VkStructureType(1000156004);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000143003);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES:
  VkStructureType = VkStructureType(1000221000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES:
  VkStructureType = VkStructureType(1000241000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT:
  VkStructureType = VkStructureType(1000273000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT:
  VkStructureType = VkStructureType(1000260000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES:
  VkStructureType = VkStructureType(1000180000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: VkStructureType =
  VkStructureType(1000181000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM:
  VkStructureType = VkStructureType(1000497000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM:
  VkStructureType = VkStructureType(1000497001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD:
  VkStructureType = VkStructureType(1000227000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: VkStructureType =
  VkStructureType(1000185000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM: VkStructureType =
  VkStructureType(1000415000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: VkStructureType = VkStructureType(1000276000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES:
  VkStructureType = VkStructureType(1000063000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: VkStructureType = VkStructureType(1000321000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES:
  VkStructureType = VkStructureType(1000082000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT:
  VkStructureType = VkStructureType(1000234000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV:
  VkStructureType = VkStructureType(1000204000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES:
  VkStructureType = VkStructureType(1000280000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES:
  VkStructureType = VkStructureType(1000280001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL:
  VkStructureType = VkStructureType(1000209000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT:
  VkStructureType = VkStructureType(1000462000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000462001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV:
  VkStructureType = VkStructureType(1000154000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV:
  VkStructureType = VkStructureType(1000154001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES:
  VkStructureType = VkStructureType(1000175000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: VkStructureType = VkStructureType(1000323000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES:
  VkStructureType = VkStructureType(1000215000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV:
  VkStructureType = VkStructureType(1000164001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV:
  VkStructureType = VkStructureType(1000164002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: VkStructureType =
  VkStructureType(1000059008);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: VkStructureType =
  VkStructureType(1000094000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES:
  VkStructureType = VkStructureType(1000225002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES:
  VkStructureType = VkStructureType(1000225000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT:
  VkStructureType = VkStructureType(1000458000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI:
  VkStructureType = VkStructureType(1000369001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI:
  VkStructureType = VkStructureType(1000369002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: VkStructureType =
  VkStructureType(1000119000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT:
  VkStructureType = VkStructureType(1000275000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: VkStructureType =
  VkStructureType(1000314007);
/// Not promoted to 1.3
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT:
  VkStructureType = VkStructureType(1000281000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES:
  VkStructureType = VkStructureType(1000281001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES:
  VkStructureType = VkStructureType(1000066000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM:
  VkStructureType = VkStructureType(1000484000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: VkStructureType =
  VkStructureType(1000207000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES:
  VkStructureType = VkStructureType(1000207001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES: VkStructureType =
  VkStructureType(1000245000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT:
  VkStructureType = VkStructureType(1000028000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000028001);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES:
  VkStructureType = VkStructureType(1000253000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: VkStructureType =
  VkStructureType(1000120000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT:
  VkStructureType = VkStructureType(1000190002);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT:
  VkStructureType = VkStructureType(1000190000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT:
  VkStructureType = VkStructureType(1000352000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: VkStructureType =
  VkStructureType(1000023014);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: VkStructureType =
  VkStructureType(49);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: VkStructureType =
  VkStructureType(50);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: VkStructureType =
  VkStructureType(51);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: VkStructureType =
  VkStructureType(52);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: VkStructureType =
  VkStructureType(53);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: VkStructureType =
  VkStructureType(54);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES:
  VkStructureType = VkStructureType(1000211000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: VkStructureType = VkStructureType(1000336000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT:
  VkStructureType = VkStructureType(1000330000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT:
  VkStructureType = VkStructureType(1000252000);
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES:
  VkStructureType = VkStructureType(1000325000);
pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: VkStructureType =
  VkStructureType(17);
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000148002);
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(26);
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000381001);
pub const VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: VkStructureType =
  VkStructureType(1000183000);
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000152000);
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000250001);
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000149000);
pub const VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO: VkStructureType =
  VkStructureType(1000192000);
pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(25);
pub const VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000099001);
pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(27);
pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR: VkStructureType =
  VkStructureType(1000269003);
pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR:
  VkStructureType = VkStructureType(1000269005);
pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR: VkStructureType =
  VkStructureType(1000269002);
pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR: VkStructureType =
  VkStructureType(1000269004);
pub const VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000326002);
pub const VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR:
  VkStructureType = VkStructureType(1000226001);
pub const VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR: VkStructureType =
  VkStructureType(1000269001);
pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(20);
pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType =
  VkStructureType(30);
pub const VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000290000);
pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(24);
pub const VK_STRUCTURE_TYPE_PIPELINE_PROPERTIES_IDENTIFIER_EXT: VkStructureType =
  VkStructureType(1000372000);
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000101001);
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000102001);
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000259001);
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: VkStructureType = VkStructureType(1000254001);
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(23);
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD:
  VkStructureType = VkStructureType(1000018000);
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000028002);
pub const VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO: VkStructureType =
  VkStructureType(1000044002);
pub const VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000166001);
pub const VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000068000);
pub const VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000143002);
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType =
  VkStructureType(18);
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000462002);
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO:
  VkStructureType = VkStructureType(1000225001);
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO:
  VkStructureType = VkStructureType(1000117003);
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(21);
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000190001);
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(19);
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000164005);
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000355001);
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000205000);
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000164000);
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(22);
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000098000);
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV:
  VkStructureType = VkStructureType(1000087000);
pub const VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP: VkStructureType =
  VkStructureType(1000191000);
pub const VK_STRUCTURE_TYPE_PRESENT_ID_KHR: VkStructureType = VkStructureType(1000294000);
pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR: VkStructureType =
  VkStructureType(1000001001);
pub const VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR: VkStructureType =
  VkStructureType(1000084000);
pub const VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE: VkStructureType =
  VkStructureType(1000092000);
pub const VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO: VkStructureType =
  VkStructureType(1000295002);
pub const VK_STRUCTURE_TYPE_PRIVATE_VENDOR_INFO_RESERVED_OFFSET_0_NV: VkStructureType =
  VkStructureType(1000051000);
pub const VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO: VkStructureType =
  VkStructureType(1000145000);
pub const VK_STRUCTURE_TYPE_QUERY_LOW_LATENCY_SUPPORT_NV: VkStructureType =
  VkStructureType(1000310000);
pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: VkStructureType = VkStructureType(11);
pub const VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000116002);
pub const VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL:
  VkStructureType = VkStructureType(1000210000);
pub const VK_STRUCTURE_TYPE_QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR:
  VkStructureType = VkStructureType(1000299005);
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: VkStructureType =
  VkStructureType(1000314008);
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: VkStructureType =
  VkStructureType(1000206001);
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: VkStructureType =
  VkStructureType(1000388001);
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2: VkStructureType =
  VkStructureType(1000059005);
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR:
  VkStructureType = VkStructureType(1000023016);
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR: VkStructureType =
  VkStructureType(1000023012);
pub const VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000150015);
pub const VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000165000);
pub const VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR:
  VkStructureType = VkStructureType(1000150018);
pub const VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000150016);
pub const VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000165011);
pub const VK_STRUCTURE_TYPE_REFRESH_OBJECT_LIST_KHR: VkStructureType =
  VkStructureType(1000308000);
pub const VK_STRUCTURE_TYPE_RELEASE_SWAPCHAIN_IMAGES_INFO_EXT: VkStructureType =
  VkStructureType(1000275005);
pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO: VkStructureType =
  VkStructureType(1000044001);
pub const VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT:
  VkStructureType = VkStructureType(1000044007);
pub const VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR:
  VkStructureType = VkStructureType(1000044006);
pub const VK_STRUCTURE_TYPE_RENDERING_INFO: VkStructureType = VkStructureType(1000044000);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO: VkStructureType =
  VkStructureType(1000108003);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: VkStructureType = VkStructureType(43);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: VkStructureType =
  VkStructureType(38);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2: VkStructureType =
  VkStructureType(1000109004);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_CONTROL_EXT: VkStructureType =
  VkStructureType(1000458001);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000458002);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000218002);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO:
  VkStructureType = VkStructureType(1000117001);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO: VkStructureType =
  VkStructureType(1000053000);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: VkStructureType =
  VkStructureType(1000143001);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000458003);
pub const VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: VkStructureType =
  VkStructureType(1000282001);
pub const VK_STRUCTURE_TYPE_RESERVED_QCOM: VkStructureType = VkStructureType(1000309000);
pub const VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2: VkStructureType =
  VkStructureType(1000337005);
pub const VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000411001);
pub const VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType =
  VkStructureType(1000316008);
pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: VkStructureType = VkStructureType(31);
pub const VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000287000);
pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO: VkStructureType =
  VkStructureType(1000130001);
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO: VkStructureType =
  VkStructureType(1000156000);
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES:
  VkStructureType = VkStructureType(1000156005);
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO: VkStructureType =
  VkStructureType(1000156001);
pub const VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT: VkStructureType =
  VkStructureType(1000143000);
pub const VK_STRUCTURE_TYPE_SCI_SYNC_ATTRIBUTES_INFO_NV: VkStructureType =
  VkStructureType(1000373003);
pub const VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX: VkStructureType =
  VkStructureType(1000378000);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = VkStructureType(9);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR: VkStructureType =
  VkStructureType(1000079001);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_SCI_SYNC_INFO_NV: VkStructureType =
  VkStructureType(1000373006);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType =
  VkStructureType(1000078003);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType =
  VkStructureType(1000365001);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SCI_SYNC_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000489001);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SCI_SYNC_POOL_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000489000);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO: VkStructureType =
  VkStructureType(1000207005);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO: VkStructureType =
  VkStructureType(1000314005);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO: VkStructureType =
  VkStructureType(1000207002);
pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO: VkStructureType =
  VkStructureType(1000207004);
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType =
  VkStructureType(16);
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_IDENTIFIER_EXT: VkStructureType =
  VkStructureType(1000462003);
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000160001);
pub const VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: VkStructureType =
  VkStructureType(1000111000);
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType =
  VkStructureType(1000059007);
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: VkStructureType =
  VkStructureType(1000146004);
pub const VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: VkStructureType =
  VkStructureType(1000049000);
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = VkStructureType(4);
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2: VkStructureType = VkStructureType(1000314004);
pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO: VkStructureType =
  VkStructureType(1000109005);
pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2: VkStructureType =
  VkStructureType(1000109003);
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2: VkStructureType =
  VkStructureType(1000109002);
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: VkStructureType =
  VkStructureType(1000199001);
pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO: VkStructureType =
  VkStructureType(1000109006);
pub const VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM:
  VkStructureType = VkStructureType(1000425002);
pub const VK_STRUCTURE_TYPE_SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: VkStructureType =
  VkStructureType(1000376001);
pub const VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: VkStructureType =
  VkStructureType(1000369000);
pub const VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT: VkStructureType =
  VkStructureType(1000338002);
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT: VkStructureType =
  VkStructureType(1000090000);
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR: VkStructureType =
  VkStructureType(1000119001);
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT:
  VkStructureType = VkStructureType(1000255002);
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_PRESENT_BARRIER_NV: VkStructureType =
  VkStructureType(1000292001);
pub const VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR: VkStructureType =
  VkStructureType(1000119002);
pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: VkStructureType =
  VkStructureType(1000255000);
pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT:
  VkStructureType = VkStructureType(1000255001);
pub const VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT: VkStructureType =
  VkStructureType(1000274002);
pub const VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT: VkStructureType =
  VkStructureType(1000274000);
pub const VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT: VkStructureType =
  VkStructureType(1000274001);
pub const VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR: VkStructureType =
  VkStructureType(1000239000);
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000091003);
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000001000);
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD:
  VkStructureType = VkStructureType(1000213001);
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID: VkStructureType =
  VkStructureType(1000010001);
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV: VkStructureType =
  VkStructureType(1000292002);
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_FENCE_INFO_EXT: VkStructureType =
  VkStructureType(1000275001);
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000275002);
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODE_INFO_EXT: VkStructureType =
  VkStructureType(1000275003);
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000275004);
pub const VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA: VkStructureType =
  VkStructureType(1000366008);
pub const VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: VkStructureType =
  VkStructureType(1000041000);
pub const VK_STRUCTURE_TYPE_TILE_PROPERTIES_QCOM: VkStructureType =
  VkStructureType(1000484001);
pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO: VkStructureType =
  VkStructureType(1000207003);
pub const VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT: VkStructureType =
  VkStructureType(1000160000);
pub const VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT: VkStructureType =
  VkStructureType(1000247000);
pub const VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT: VkStructureType =
  VkStructureType(1000061000);
pub const VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: VkStructureType =
  VkStructureType(1000352002);
pub const VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: VkStructureType =
  VkStructureType(1000352001);
pub const VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR: VkStructureType =
  VkStructureType(1000023008);
pub const VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR: VkStructureType =
  VkStructureType(1000023001);
pub const VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR: VkStructureType =
  VkStructureType(1000023010);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR: VkStructureType =
  VkStructureType(1000024001);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_CAPABILITIES_KHR: VkStructureType =
  VkStructureType(1000040000);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR: VkStructureType =
  VkStructureType(1000040006);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PICTURE_INFO_KHR: VkStructureType =
  VkStructureType(1000040001);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PROFILE_INFO_KHR: VkStructureType =
  VkStructureType(1000040003);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR:
  VkStructureType = VkStructureType(1000040005);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR:
  VkStructureType = VkStructureType(1000040004);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_CAPABILITIES_KHR: VkStructureType =
  VkStructureType(1000187000);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR: VkStructureType =
  VkStructureType(1000187005);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PICTURE_INFO_KHR: VkStructureType =
  VkStructureType(1000187004);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PROFILE_INFO_KHR: VkStructureType =
  VkStructureType(1000187003);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR:
  VkStructureType = VkStructureType(1000187002);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR:
  VkStructureType = VkStructureType(1000187001);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR: VkStructureType =
  VkStructureType(1000024000);
pub const VK_STRUCTURE_TYPE_VIDEO_DECODE_USAGE_INFO_KHR: VkStructureType =
  VkStructureType(1000024002);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_CAPABILITIES_KHR: VkStructureType =
  VkStructureType(1000299003);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_CAPABILITIES_EXT: VkStructureType =
  VkStructureType(1000038000);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT: VkStructureType =
  VkStructureType(1000038004);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_NALU_SLICE_INFO_EXT: VkStructureType =
  VkStructureType(1000038005);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_PROFILE_INFO_EXT: VkStructureType =
  VkStructureType(1000038007);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT: VkStructureType =
  VkStructureType(1000038008);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT:
  VkStructureType = VkStructureType(1000038009);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT:
  VkStructureType = VkStructureType(1000038002);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000038001);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT: VkStructureType =
  VkStructureType(1000038003);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_CAPABILITIES_EXT: VkStructureType =
  VkStructureType(1000039000);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT: VkStructureType =
  VkStructureType(1000039004);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_EXT:
  VkStructureType = VkStructureType(1000039005);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_PROFILE_INFO_EXT: VkStructureType =
  VkStructureType(1000039007);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT: VkStructureType =
  VkStructureType(1000039009);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT:
  VkStructureType = VkStructureType(1000039010);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT:
  VkStructureType = VkStructureType(1000039002);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT:
  VkStructureType = VkStructureType(1000039001);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT: VkStructureType =
  VkStructureType(1000039003);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_INFO_KHR: VkStructureType =
  VkStructureType(1000299000);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: VkStructureType =
  VkStructureType(1000299001);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: VkStructureType =
  VkStructureType(1000299002);
pub const VK_STRUCTURE_TYPE_VIDEO_ENCODE_USAGE_INFO_KHR: VkStructureType =
  VkStructureType(1000299004);
pub const VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR: VkStructureType =
  VkStructureType(1000023009);
pub const VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR: VkStructureType =
  VkStructureType(1000023015);
pub const VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_INFO_KHR: VkStructureType =
  VkStructureType(1000023002);
pub const VK_STRUCTURE_TYPE_VIDEO_PROFILE_INFO_KHR: VkStructureType =
  VkStructureType(1000023000);
pub const VK_STRUCTURE_TYPE_VIDEO_PROFILE_LIST_INFO_KHR: VkStructureType =
  VkStructureType(1000023013);
pub const VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_INFO_KHR: VkStructureType =
  VkStructureType(1000023011);
pub const VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000023005);
pub const VK_STRUCTURE_TYPE_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: VkStructureType =
  VkStructureType(1000023003);
pub const VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000023006);
pub const VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: VkStructureType =
  VkStructureType(1000023007);
pub const VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN: VkStructureType =
  VkStructureType(1000062000);
pub const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000006000);
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: VkStructureType =
  VkStructureType(1000075000);
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: VkStructureType =
  VkStructureType(1000058000);
pub const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000009000);
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = VkStructureType(35);
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR:
  VkStructureType = VkStructureType(1000150007);
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV:
  VkStructureType = VkStructureType(1000165007);
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: VkStructureType =
  VkStructureType(1000138002);
pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000005000);
pub const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR: VkStructureType =
  VkStructureType(1000004000);
/// Alias of [`VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2`]
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2;
/// Alias of [`VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT`]
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
/// Alias of [`VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2`]
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2;
/// Alias of [`VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT`]
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
/// Alias of [`VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD`]
pub const VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_NV: VkStructureType =
  VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
/// Alias of [`VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO`]
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO`]
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO`]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO`]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO`]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2`]
pub const VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_BUFFER_COPY_2`]
pub const VK_STRUCTURE_TYPE_BUFFER_COPY_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BUFFER_COPY_2;
/// Alias of [`VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`]
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`]
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2`]
pub const VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2;
/// Alias of [`VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2`]
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2;
/// Alias of [`VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2`]
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO`]
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO`]
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2`]
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2`]
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2`]
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2`]
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT`]
#[deprecated = "aliased"]
pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;
/// Alias of [`VK_STRUCTURE_TYPE_DEPENDENCY_INFO`]
pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DEPENDENCY_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT`]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT;
/// Alias of [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO`]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT:
  VkStructureType =
  VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT`]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
/// Alias of [`VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS`]
pub const VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO`]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO`]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO`]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO`]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS`]
pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO`]
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR`]
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR;
/// Alias of [`VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO`]
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2`]
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2;
/// Alias of [`VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3`]
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3;
/// Alias of [`VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO`]
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_BLIT_2`]
pub const VK_STRUCTURE_TYPE_IMAGE_BLIT_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_BLIT_2;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_COPY_2`]
pub const VK_STRUCTURE_TYPE_IMAGE_COPY_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_COPY_2;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2`]
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2`]
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2`]
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO`]
pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2`]
pub const VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2`]
pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO`]
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_MEMORY_BARRIER_2`]
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_MEMORY_BARRIER_2;
/// Alias of [`VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO`]
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS`]
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS;
/// Alias of [`VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO`]
pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2`]
pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2;
/// Alias of [`VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT`]
pub const VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: VkStructureType =
  VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_BUF_FEATURES_NV:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV:
  VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE:
  VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR:
  VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR:
  VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT:
  VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: VkStructureType =
  VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES`]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
/// Alias of [`VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`]
pub const VK_STRUCTURE_TYPE_PIPELINE_INFO_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR;
/// Alias of [`VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR:
  VkStructureType =
  VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL`]
#[deprecated = "aliased"]
pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO_INTEL: VkStructureType =
  VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL;
/// Alias of [`VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR`]
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR;
/// Alias of [`VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2`]
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
/// Alias of [`VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO`]
pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_RENDERING_INFO`]
pub const VK_STRUCTURE_TYPE_RENDERING_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_RENDERING_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO`]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2`]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2`]
pub const VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES`]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
/// Alias of [`VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO`]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO`]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO`]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO`]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2`]
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2;
/// Alias of [`VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2`]
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
/// Alias of [`VK_STRUCTURE_TYPE_SUBMIT_INFO_2`]
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SUBMIT_INFO_2;
/// Alias of [`VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO`]
pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2`]
pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2;
/// Alias of [`VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2`]
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2;
/// Alias of [`VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE`]
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR:
  VkStructureType = VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
/// Alias of [`VK_STRUCTURE_TYPE_SUBPASS_END_INFO`]
pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_SUBPASS_END_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT`]
#[deprecated = "aliased"]
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT: VkStructureType =
  VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT;
/// Alias of [`VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO`]
pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: VkStructureType =
  VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO;
/// Alias of [`VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK`]
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT:
  VkStructureType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
impl core::fmt::Debug for VkStructureType {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT",
      VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR => "VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR",
      VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR => "VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR",
      VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR => "VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR",
      VK_STRUCTURE_TYPE_AMIGO_PROFILING_SUBMIT_INFO_SEC => "VK_STRUCTURE_TYPE_AMIGO_PROFILING_SUBMIT_INFO_SEC",
      VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID => "VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID",
      VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID => "VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID",
      VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID => "VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID",
      VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID => "VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID",
      VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_APPLICATION_INFO => "VK_STRUCTURE_TYPE_APPLICATION_INFO",
      VK_STRUCTURE_TYPE_APPLICATION_PARAMETERS_EXT => "VK_STRUCTURE_TYPE_APPLICATION_PARAMETERS_EXT",
      VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2 => "VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2",
      VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT => "VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT",
      VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2 => "VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2",
      VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT => "VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT",
      VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD => "VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD",
      VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV => "VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV",
      VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO => "VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO",
      VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO => "VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO",
      VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO => "VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO",
      VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO => "VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO",
      VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR => "VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR",
      VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO => "VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO",
      VK_STRUCTURE_TYPE_BIND_SPARSE_INFO => "VK_STRUCTURE_TYPE_BIND_SPARSE_INFO",
      VK_STRUCTURE_TYPE_BIND_VIDEO_SESSION_MEMORY_INFO_KHR => "VK_STRUCTURE_TYPE_BIND_VIDEO_SESSION_MEMORY_INFO_KHR",
      VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2 => "VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2",
      VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => "VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT",
      VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA => "VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA",
      VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_BUFFER_COPY_2 => "VK_STRUCTURE_TYPE_BUFFER_COPY_2",
      VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO => "VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO",
      VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO => "VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO",
      VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2 => "VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2",
      VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER => "VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER",
      VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2 => "VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2",
      VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2 => "VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2",
      VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO => "VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO",
      VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO => "VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO",
      VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT => "VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT",
      VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV => "VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV",
      VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV => "VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV",
      VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO => "VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO",
      VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO => "VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO",
      VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT => "VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT",
      VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO => "VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO",
      VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO => "VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO",
      VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM => "VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM",
      VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV => "VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV",
      VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO => "VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO",
      VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO => "VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO",
      VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO => "VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO",
      VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT => "VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT",
      VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV => "VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR => "VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR",
      VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR => "VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR",
      VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2 => "VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2",
      VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2 => "VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2",
      VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM => "VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM",
      VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET => "VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET",
      VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2 => "VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2",
      VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2 => "VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2",
      VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR => "VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR",
      VK_STRUCTURE_TYPE_COPY_MEMORY_TO_MICROMAP_INFO_EXT => "VK_STRUCTURE_TYPE_COPY_MEMORY_TO_MICROMAP_INFO_EXT",
      VK_STRUCTURE_TYPE_COPY_MICROMAP_INFO_EXT => "VK_STRUCTURE_TYPE_COPY_MICROMAP_INFO_EXT",
      VK_STRUCTURE_TYPE_COPY_MICROMAP_TO_MEMORY_INFO_EXT => "VK_STRUCTURE_TYPE_COPY_MICROMAP_TO_MEMORY_INFO_EXT",
      VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX => "VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX",
      VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX => "VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX",
      VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX => "VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX",
      VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR => "VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR",
      VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT => "VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT",
      VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT => "VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT",
      VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT => "VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT",
      VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT => "VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT",
      VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => "VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT",
      VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT => "VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT",
      VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT => "VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT",
      VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV => "VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV",
      VK_STRUCTURE_TYPE_DEPENDENCY_INFO => "VK_STRUCTURE_TYPE_DEPENDENCY_INFO",
      VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT => "VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT",
      VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT => "VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT",
      VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT => "VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT",
      VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT => "VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT",
      VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO => "VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO",
      VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO => "VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO",
      VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO => "VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO",
      VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE => "VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE",
      VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO => "VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO",
      VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO => "VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO",
      VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE => "VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE",
      VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT => "VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT",
      VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO => "VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO",
      VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT => "VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT",
      VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO => "VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT => "VK_STRUCTURE_TYPE_DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT",
      VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS => "VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS",
      VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO => "VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO",
      VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT => "VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT",
      VK_STRUCTURE_TYPE_DEVICE_FAULT_COUNTS_EXT => "VK_STRUCTURE_TYPE_DEVICE_FAULT_COUNTS_EXT",
      VK_STRUCTURE_TYPE_DEVICE_FAULT_INFO_EXT => "VK_STRUCTURE_TYPE_DEVICE_FAULT_INFO_EXT",
      VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO => "VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO",
      VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO => "VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO",
      VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO => "VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO",
      VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR => "VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR",
      VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR => "VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR",
      VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO => "VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO",
      VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO => "VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO",
      VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS => "VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS",
      VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO => "VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO",
      VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD => "VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD",
      VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT => "VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT",
      VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO => "VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO",
      VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO => "VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO",
      VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2 => "VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2",
      VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_INFO_LUNARG => "VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_INFO_LUNARG",
      VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_LIST_LUNARG => "VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_LIST_LUNARG",
      VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT => "VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT",
      VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR => "VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR",
      VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD => "VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD",
      VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR => "VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR",
      VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR => "VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR",
      VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR => "VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR",
      VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT => "VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT",
      VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR => "VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR",
      VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR => "VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR",
      VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT => "VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT",
      VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT => "VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT",
      VK_STRUCTURE_TYPE_EVENT_CREATE_INFO => "VK_STRUCTURE_TYPE_EVENT_CREATE_INFO",
      VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO => "VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO",
      VK_STRUCTURE_TYPE_EXPORT_FENCE_SCI_SYNC_INFO_NV => "VK_STRUCTURE_TYPE_EXPORT_FENCE_SCI_SYNC_INFO_NV",
      VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR => "VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR",
      VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO => "VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO",
      VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV => "VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV",
      VK_STRUCTURE_TYPE_EXPORT_MEMORY_SCI_BUF_INFO_NV => "VK_STRUCTURE_TYPE_EXPORT_MEMORY_SCI_BUF_INFO_NV",
      VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR => "VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR",
      VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV => "VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV",
      VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT => "VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT",
      VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT => "VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT",
      VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT => "VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT",
      VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT => "VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT",
      VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT => "VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT",
      VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT => "VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT",
      VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT => "VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT",
      VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO => "VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO",
      VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_SCI_SYNC_INFO_NV => "VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_SCI_SYNC_INFO_NV",
      VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => "VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR",
      VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES => "VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES",
      VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES => "VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES",
      VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID => "VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID",
      VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES => "VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES",
      VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO => "VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO",
      VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO => "VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO",
      VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES => "VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES",
      VK_STRUCTURE_TYPE_FENCE_CREATE_INFO => "VK_STRUCTURE_TYPE_FENCE_CREATE_INFO",
      VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR => "VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR",
      VK_STRUCTURE_TYPE_FENCE_GET_SCI_SYNC_INFO_NV => "VK_STRUCTURE_TYPE_FENCE_GET_SCI_SYNC_INFO_NV",
      VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR => "VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR",
      VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2 => "VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2",
      VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3 => "VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3",
      VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => "VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR",
      VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO => "VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO",
      VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO => "VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO",
      VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO => "VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO",
      VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV => "VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV",
      VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV => "VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV",
      VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV => "VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV",
      VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV => "VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV",
      VK_STRUCTURE_TYPE_GEOMETRY_NV => "VK_STRUCTURE_TYPE_GEOMETRY_NV",
      VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV => "VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV",
      VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO => "VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO",
      VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_HDR_METADATA_EXT => "VK_STRUCTURE_TYPE_HDR_METADATA_EXT",
      VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_IMAGE_BLIT_2 => "VK_STRUCTURE_TYPE_IMAGE_BLIT_2",
      VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => "VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT",
      VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_CONTROL_EXT => "VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_CONTROL_EXT",
      VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_IMAGE_COPY_2 => "VK_STRUCTURE_TYPE_IMAGE_COPY_2",
      VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO => "VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO",
      VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO => "VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO",
      VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2 => "VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2",
      VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER => "VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER",
      VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2 => "VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2",
      VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2 => "VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2",
      VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO => "VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO",
      VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2 => "VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2",
      VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 => "VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2",
      VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO => "VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO",
      VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT => "VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT",
      VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX => "VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX",
      VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT => "VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT",
      VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => "VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT",
      VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO => "VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO",
      VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX => "VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX",
      VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM => "VK_STRUCTURE_TYPE_IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM",
      VK_STRUCTURE_TYPE_IMAGE_VIEW_SLICED_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_IMAGE_VIEW_SLICED_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO => "VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO",
      VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => "VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID",
      VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR => "VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR",
      VK_STRUCTURE_TYPE_IMPORT_FENCE_SCI_SYNC_INFO_NV => "VK_STRUCTURE_TYPE_IMPORT_FENCE_SCI_SYNC_INFO_NV",
      VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR => "VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR",
      VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA => "VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA",
      VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR => "VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR",
      VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT => "VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT",
      VK_STRUCTURE_TYPE_IMPORT_MEMORY_SCI_BUF_INFO_NV => "VK_STRUCTURE_TYPE_IMPORT_MEMORY_SCI_BUF_INFO_NV",
      VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR => "VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR",
      VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV => "VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV",
      VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT => "VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT",
      VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT => "VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT",
      VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT => "VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT",
      VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT => "VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT",
      VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR => "VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR",
      VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV => "VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV",
      VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => "VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR",
      VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV => "VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV",
      VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL => "VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL",
      VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO => "VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO",
      VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK => "VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK",
      VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO => "VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO",
      VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO => "VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO",
      VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK => "VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK",
      VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE => "VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE",
      VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO => "VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO",
      VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO => "VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO",
      VK_STRUCTURE_TYPE_MEMORY_BARRIER => "VK_STRUCTURE_TYPE_MEMORY_BARRIER",
      VK_STRUCTURE_TYPE_MEMORY_BARRIER_2 => "VK_STRUCTURE_TYPE_MEMORY_BARRIER_2",
      VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO => "VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO",
      VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS => "VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS",
      VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => "VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID",
      VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR => "VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR",
      VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV => "VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV",
      VK_STRUCTURE_TYPE_MEMORY_GET_SCI_BUF_INFO_NV => "VK_STRUCTURE_TYPE_MEMORY_GET_SCI_BUF_INFO_NV",
      VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR => "VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR",
      VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO => "VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO",
      VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT => "VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT",
      VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2 => "VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2",
      VK_STRUCTURE_TYPE_MEMORY_SCI_BUF_PROPERTIES_NV => "VK_STRUCTURE_TYPE_MEMORY_SCI_BUF_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA => "VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA",
      VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_MICROMAP_BUILD_INFO_EXT => "VK_STRUCTURE_TYPE_MICROMAP_BUILD_INFO_EXT",
      VK_STRUCTURE_TYPE_MICROMAP_BUILD_SIZES_INFO_EXT => "VK_STRUCTURE_TYPE_MICROMAP_BUILD_SIZES_INFO_EXT",
      VK_STRUCTURE_TYPE_MICROMAP_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_MICROMAP_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_MICROMAP_VERSION_INFO_EXT => "VK_STRUCTURE_TYPE_MICROMAP_VERSION_INFO_EXT",
      VK_STRUCTURE_TYPE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT => "VK_STRUCTURE_TYPE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT",
      VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX => "VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX",
      VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM => "VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM",
      VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_NATIVE_BUFFER_ANDROID => "VK_STRUCTURE_TYPE_NATIVE_BUFFER_ANDROID",
      VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_OPTICAL_FLOW_EXECUTE_INFO_NV => "VK_STRUCTURE_TYPE_OPTICAL_FLOW_EXECUTE_INFO_NV",
      VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV => "VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV",
      VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV => "VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV => "VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV",
      VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL => "VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL",
      VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR => "VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR",
      VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR => "VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR",
      VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL => "VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL",
      VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL => "VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL",
      VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR => "VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR",
      VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL => "VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FAULT_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FAULT_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2 => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2 => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT",
      VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES => "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES",
      VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD => "VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD",
      VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR => "VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR",
      VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR => "VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR",
      VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR => "VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR",
      VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR => "VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR",
      VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_PROPERTIES_IDENTIFIER_EXT => "VK_STRUCTURE_TYPE_PIPELINE_PROPERTIES_IDENTIFIER_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD => "VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD",
      VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO => "VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO",
      VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP => "VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP",
      VK_STRUCTURE_TYPE_PRESENT_ID_KHR => "VK_STRUCTURE_TYPE_PRESENT_ID_KHR",
      VK_STRUCTURE_TYPE_PRESENT_INFO_KHR => "VK_STRUCTURE_TYPE_PRESENT_INFO_KHR",
      VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR => "VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR",
      VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE => "VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE",
      VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO => "VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO",
      VK_STRUCTURE_TYPE_PRIVATE_VENDOR_INFO_RESERVED_OFFSET_0_NV => "VK_STRUCTURE_TYPE_PRIVATE_VENDOR_INFO_RESERVED_OFFSET_0_NV",
      VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO => "VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO",
      VK_STRUCTURE_TYPE_QUERY_LOW_LATENCY_SUPPORT_NV => "VK_STRUCTURE_TYPE_QUERY_LOW_LATENCY_SUPPORT_NV",
      VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO => "VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO",
      VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL => "VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL",
      VK_STRUCTURE_TYPE_QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV => "VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV",
      VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV => "VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV",
      VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2 => "VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2",
      VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_REFRESH_OBJECT_LIST_KHR => "VK_STRUCTURE_TYPE_REFRESH_OBJECT_LIST_KHR",
      VK_STRUCTURE_TYPE_RELEASE_SWAPCHAIN_IMAGES_INFO_EXT => "VK_STRUCTURE_TYPE_RELEASE_SWAPCHAIN_IMAGES_INFO_EXT",
      VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO => "VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO",
      VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT => "VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT",
      VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => "VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR",
      VK_STRUCTURE_TYPE_RENDERING_INFO => "VK_STRUCTURE_TYPE_RENDERING_INFO",
      VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO => "VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO",
      VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO => "VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO",
      VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO => "VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO",
      VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2 => "VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2",
      VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_CONTROL_EXT => "VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_CONTROL_EXT",
      VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO => "VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO",
      VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO => "VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO",
      VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT => "VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT",
      VK_STRUCTURE_TYPE_RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM => "VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM",
      VK_STRUCTURE_TYPE_RESERVED_QCOM => "VK_STRUCTURE_TYPE_RESERVED_QCOM",
      VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2 => "VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2",
      VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => "VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT",
      VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO => "VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO",
      VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO => "VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO",
      VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO => "VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO",
      VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES => "VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES",
      VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO => "VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO",
      VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT => "VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT",
      VK_STRUCTURE_TYPE_SCI_SYNC_ATTRIBUTES_INFO_NV => "VK_STRUCTURE_TYPE_SCI_SYNC_ATTRIBUTES_INFO_NV",
      VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX => "VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX",
      VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO => "VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO",
      VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR => "VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR",
      VK_STRUCTURE_TYPE_SEMAPHORE_GET_SCI_SYNC_INFO_NV => "VK_STRUCTURE_TYPE_SEMAPHORE_GET_SCI_SYNC_INFO_NV",
      VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR => "VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR",
      VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA => "VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA",
      VK_STRUCTURE_TYPE_SEMAPHORE_SCI_SYNC_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_SEMAPHORE_SCI_SYNC_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_SEMAPHORE_SCI_SYNC_POOL_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_SEMAPHORE_SCI_SYNC_POOL_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO => "VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO",
      VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO => "VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO",
      VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO => "VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO",
      VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO => "VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO",
      VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO => "VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO",
      VK_STRUCTURE_TYPE_SHADER_MODULE_IDENTIFIER_EXT => "VK_STRUCTURE_TYPE_SHADER_MODULE_IDENTIFIER_EXT",
      VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR => "VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR",
      VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2 => "VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2",
      VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 => "VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2",
      VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP => "VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP",
      VK_STRUCTURE_TYPE_SUBMIT_INFO => "VK_STRUCTURE_TYPE_SUBMIT_INFO",
      VK_STRUCTURE_TYPE_SUBMIT_INFO_2 => "VK_STRUCTURE_TYPE_SUBMIT_INFO_2",
      VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO => "VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO",
      VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2 => "VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2",
      VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2 => "VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2",
      VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE => "VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE",
      VK_STRUCTURE_TYPE_SUBPASS_END_INFO => "VK_STRUCTURE_TYPE_SUBPASS_END_INFO",
      VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM => "VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM",
      VK_STRUCTURE_TYPE_SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT => "VK_STRUCTURE_TYPE_SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT",
      VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI => "VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI",
      VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT => "VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT",
      VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT => "VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT",
      VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR => "VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR",
      VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT => "VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT",
      VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_PRESENT_BARRIER_NV => "VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_PRESENT_BARRIER_NV",
      VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR => "VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR",
      VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT => "VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT",
      VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT => "VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT",
      VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT => "VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT",
      VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT => "VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT",
      VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT => "VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT",
      VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR => "VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR",
      VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD => "VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD",
      VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID => "VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID",
      VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV => "VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV",
      VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_FENCE_INFO_EXT => "VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_FENCE_INFO_EXT",
      VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODE_INFO_EXT => "VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODE_INFO_EXT",
      VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA => "VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA",
      VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD => "VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD",
      VK_STRUCTURE_TYPE_TILE_PROPERTIES_QCOM => "VK_STRUCTURE_TYPE_TILE_PROPERTIES_QCOM",
      VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO => "VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO",
      VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT => "VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT",
      VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT => "VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT",
      VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT => "VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT",
      VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT => "VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT",
      VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR => "VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR",
      VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_CAPABILITIES_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_CAPABILITIES_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PICTURE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PICTURE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PROFILE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PROFILE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_CAPABILITIES_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_CAPABILITIES_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PICTURE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PICTURE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PROFILE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PROFILE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_DECODE_USAGE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_DECODE_USAGE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_CAPABILITIES_KHR => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_CAPABILITIES_KHR",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_CAPABILITIES_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_CAPABILITIES_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_NALU_SLICE_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_NALU_SLICE_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_PROFILE_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_PROFILE_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_CAPABILITIES_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_CAPABILITIES_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_DPB_SLOT_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_PROFILE_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_PROFILE_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_VCL_FRAME_INFO_EXT",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_ENCODE_USAGE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_ENCODE_USAGE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR => "VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR",
      VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_PROFILE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_PROFILE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_PROFILE_LIST_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_PROFILE_LIST_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR => "VK_STRUCTURE_TYPE_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR",
      VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR => "VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR",
      VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN => "VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN",
      VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR => "VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR",
      VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV => "VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV",
      VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET => "VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET",
      VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR => "VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR",
      VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV => "VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV",
      VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK => "VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK",
      VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR",
      VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR => "VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR",
      other => return write!(f, "VkStructureType({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSubpassContents](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassContents.html) (enumeration)
  VkSubpassContents(u32)
);
pub const VK_SUBPASS_CONTENTS_INLINE: VkSubpassContents = VkSubpassContents(0);
pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: VkSubpassContents =
  VkSubpassContents(1);
impl core::fmt::Debug for VkSubpassContents {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SUBPASS_CONTENTS_INLINE => "VK_SUBPASS_CONTENTS_INLINE",
      VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS => {
        "VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS"
      }
      other => return write!(f, "VkSubpassContents({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSubpassMergeStatusEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassMergeStatusEXT.html) (enumeration)
  VkSubpassMergeStatusEXT(u32)
);
pub const VK_SUBPASS_MERGE_STATUS_DISALLOWED_EXT: VkSubpassMergeStatusEXT =
  VkSubpassMergeStatusEXT(1);
pub const VK_SUBPASS_MERGE_STATUS_MERGED_EXT: VkSubpassMergeStatusEXT =
  VkSubpassMergeStatusEXT(0);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_ALIASING_EXT: VkSubpassMergeStatusEXT =
  VkSubpassMergeStatusEXT(5);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPENDENCIES_EXT: VkSubpassMergeStatusEXT =
  VkSubpassMergeStatusEXT(6);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPTH_STENCIL_COUNT_EXT:
  VkSubpassMergeStatusEXT = VkSubpassMergeStatusEXT(10);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT:
  VkSubpassMergeStatusEXT = VkSubpassMergeStatusEXT(7);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INSUFFICIENT_STORAGE_EXT:
  VkSubpassMergeStatusEXT = VkSubpassMergeStatusEXT(9);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT:
  VkSubpassMergeStatusEXT = VkSubpassMergeStatusEXT(11);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SAMPLES_MISMATCH_EXT:
  VkSubpassMergeStatusEXT = VkSubpassMergeStatusEXT(3);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SIDE_EFFECTS_EXT: VkSubpassMergeStatusEXT =
  VkSubpassMergeStatusEXT(2);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SINGLE_SUBPASS_EXT: VkSubpassMergeStatusEXT =
  VkSubpassMergeStatusEXT(12);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT:
  VkSubpassMergeStatusEXT = VkSubpassMergeStatusEXT(8);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_UNSPECIFIED_EXT: VkSubpassMergeStatusEXT =
  VkSubpassMergeStatusEXT(13);
pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_VIEWS_MISMATCH_EXT: VkSubpassMergeStatusEXT =
  VkSubpassMergeStatusEXT(4);
impl core::fmt::Debug for VkSubpassMergeStatusEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SUBPASS_MERGE_STATUS_DISALLOWED_EXT => "VK_SUBPASS_MERGE_STATUS_DISALLOWED_EXT",
      VK_SUBPASS_MERGE_STATUS_MERGED_EXT => "VK_SUBPASS_MERGE_STATUS_MERGED_EXT",
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_ALIASING_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_ALIASING_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPENDENCIES_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPENDENCIES_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPTH_STENCIL_COUNT_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPTH_STENCIL_COUNT_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INSUFFICIENT_STORAGE_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INSUFFICIENT_STORAGE_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SAMPLES_MISMATCH_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SAMPLES_MISMATCH_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SIDE_EFFECTS_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SIDE_EFFECTS_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SINGLE_SUBPASS_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SINGLE_SUBPASS_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_UNSPECIFIED_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_UNSPECIFIED_EXT"
      }
      VK_SUBPASS_MERGE_STATUS_NOT_MERGED_VIEWS_MISMATCH_EXT => {
        "VK_SUBPASS_MERGE_STATUS_NOT_MERGED_VIEWS_MISMATCH_EXT"
      }
      other => return write!(f, "VkSubpassMergeStatusEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkSystemAllocationScope](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSystemAllocationScope.html) (enumeration)
  VkSystemAllocationScope(u32)
);
pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: VkSystemAllocationScope =
  VkSystemAllocationScope(2);
pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: VkSystemAllocationScope =
  VkSystemAllocationScope(0);
pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: VkSystemAllocationScope =
  VkSystemAllocationScope(3);
pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: VkSystemAllocationScope =
  VkSystemAllocationScope(4);
pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: VkSystemAllocationScope =
  VkSystemAllocationScope(1);
impl core::fmt::Debug for VkSystemAllocationScope {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_SYSTEM_ALLOCATION_SCOPE_CACHE => "VK_SYSTEM_ALLOCATION_SCOPE_CACHE",
      VK_SYSTEM_ALLOCATION_SCOPE_COMMAND => "VK_SYSTEM_ALLOCATION_SCOPE_COMMAND",
      VK_SYSTEM_ALLOCATION_SCOPE_DEVICE => "VK_SYSTEM_ALLOCATION_SCOPE_DEVICE",
      VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE => "VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE",
      VK_SYSTEM_ALLOCATION_SCOPE_OBJECT => "VK_SYSTEM_ALLOCATION_SCOPE_OBJECT",
      other => return write!(f, "VkSystemAllocationScope({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkTessellationDomainOrigin](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTessellationDomainOrigin.html) (enumeration)
  VkTessellationDomainOrigin(u32)
);
pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: VkTessellationDomainOrigin =
  VkTessellationDomainOrigin(1);
pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: VkTessellationDomainOrigin =
  VkTessellationDomainOrigin(0);
/// Alias of [`VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT`]
pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR: VkTessellationDomainOrigin =
  VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT;
/// Alias of [`VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT`]
pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR: VkTessellationDomainOrigin =
  VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT;
impl core::fmt::Debug for VkTessellationDomainOrigin {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT => {
        "VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT"
      }
      VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT => {
        "VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT"
      }
      other => return write!(f, "VkTessellationDomainOrigin({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkTimeDomainEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTimeDomainEXT.html) (enumeration)
  VkTimeDomainEXT(u32)
);
pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT: VkTimeDomainEXT = VkTimeDomainEXT(1);
pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT: VkTimeDomainEXT = VkTimeDomainEXT(2);
pub const VK_TIME_DOMAIN_DEVICE_EXT: VkTimeDomainEXT = VkTimeDomainEXT(0);
pub const VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT: VkTimeDomainEXT =
  VkTimeDomainEXT(3);
impl core::fmt::Debug for VkTimeDomainEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT => "VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT",
      VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT => "VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT",
      VK_TIME_DOMAIN_DEVICE_EXT => "VK_TIME_DOMAIN_DEVICE_EXT",
      VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT => {
        "VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT"
      }
      other => return write!(f, "VkTimeDomainEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkValidationCacheHeaderVersionEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheHeaderVersionEXT.html) (enumeration)
  VkValidationCacheHeaderVersionEXT(u32)
);
pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: VkValidationCacheHeaderVersionEXT =
  VkValidationCacheHeaderVersionEXT(1);
impl core::fmt::Debug for VkValidationCacheHeaderVersionEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT => {
        "VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT"
      }
      other => return write!(f, "VkValidationCacheHeaderVersionEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkValidationCheckEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCheckEXT.html) (enumeration)
  VkValidationCheckEXT(u32)
);
pub const VK_VALIDATION_CHECK_ALL_EXT: VkValidationCheckEXT = VkValidationCheckEXT(0);
pub const VK_VALIDATION_CHECK_SHADERS_EXT: VkValidationCheckEXT = VkValidationCheckEXT(1);
impl core::fmt::Debug for VkValidationCheckEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VALIDATION_CHECK_ALL_EXT => "VK_VALIDATION_CHECK_ALL_EXT",
      VK_VALIDATION_CHECK_SHADERS_EXT => "VK_VALIDATION_CHECK_SHADERS_EXT",
      other => return write!(f, "VkValidationCheckEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkValidationFeatureDisableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureDisableEXT.html) (enumeration)
  VkValidationFeatureDisableEXT(u32)
);
pub const VK_VALIDATION_FEATURE_DISABLE_ALL_EXT: VkValidationFeatureDisableEXT =
  VkValidationFeatureDisableEXT(0);
pub const VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT:
  VkValidationFeatureDisableEXT = VkValidationFeatureDisableEXT(3);
pub const VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT: VkValidationFeatureDisableEXT =
  VkValidationFeatureDisableEXT(5);
pub const VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT:
  VkValidationFeatureDisableEXT = VkValidationFeatureDisableEXT(4);
pub const VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT: VkValidationFeatureDisableEXT =
  VkValidationFeatureDisableEXT(1);
pub const VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT:
  VkValidationFeatureDisableEXT = VkValidationFeatureDisableEXT(7);
pub const VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT: VkValidationFeatureDisableEXT =
  VkValidationFeatureDisableEXT(2);
pub const VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT:
  VkValidationFeatureDisableEXT = VkValidationFeatureDisableEXT(6);
impl core::fmt::Debug for VkValidationFeatureDisableEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VALIDATION_FEATURE_DISABLE_ALL_EXT => "VK_VALIDATION_FEATURE_DISABLE_ALL_EXT",
      VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT => {
        "VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT"
      }
      VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT => {
        "VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT"
      }
      VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT => {
        "VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT"
      }
      VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT => {
        "VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT"
      }
      VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT => {
        "VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT"
      }
      VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT => {
        "VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT"
      }
      VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT => {
        "VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT"
      }
      other => return write!(f, "VkValidationFeatureDisableEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkValidationFeatureEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureEnableEXT.html) (enumeration)
  VkValidationFeatureEnableEXT(u32)
);
pub const VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT: VkValidationFeatureEnableEXT =
  VkValidationFeatureEnableEXT(2);
pub const VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT: VkValidationFeatureEnableEXT =
  VkValidationFeatureEnableEXT(3);
pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT: VkValidationFeatureEnableEXT =
  VkValidationFeatureEnableEXT(0);
pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT:
  VkValidationFeatureEnableEXT = VkValidationFeatureEnableEXT(1);
pub const VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT:
  VkValidationFeatureEnableEXT = VkValidationFeatureEnableEXT(4);
impl core::fmt::Debug for VkValidationFeatureEnableEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT => {
        "VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT"
      }
      VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT => {
        "VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT"
      }
      VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT => {
        "VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT"
      }
      VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT => {
        "VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT"
      }
      VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT => {
        "VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT"
      }
      other => return write!(f, "VkValidationFeatureEnableEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkVendorId](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVendorId.html) (enumeration)
  VkVendorId(u32)
);
/// Codeplay Software Ltd. vendor ID
pub const VK_VENDOR_ID_CODEPLAY: VkVendorId = VkVendorId(65540);
/// Kazan Software Renderer
pub const VK_VENDOR_ID_KAZAN: VkVendorId = VkVendorId(65539);
/// Mesa vendor ID
pub const VK_VENDOR_ID_MESA: VkVendorId = VkVendorId(65541);
/// Mobileye vendor ID
pub const VK_VENDOR_ID_MOBILEYE: VkVendorId = VkVendorId(65543);
/// PoCL vendor ID
pub const VK_VENDOR_ID_POCL: VkVendorId = VkVendorId(65542);
/// Vivante vendor ID
pub const VK_VENDOR_ID_VIV: VkVendorId = VkVendorId(65537);
/// VeriSilicon vendor ID
pub const VK_VENDOR_ID_VSI: VkVendorId = VkVendorId(65538);
impl core::fmt::Debug for VkVendorId {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VENDOR_ID_CODEPLAY => "VK_VENDOR_ID_CODEPLAY",
      VK_VENDOR_ID_KAZAN => "VK_VENDOR_ID_KAZAN",
      VK_VENDOR_ID_MESA => "VK_VENDOR_ID_MESA",
      VK_VENDOR_ID_MOBILEYE => "VK_VENDOR_ID_MOBILEYE",
      VK_VENDOR_ID_POCL => "VK_VENDOR_ID_POCL",
      VK_VENDOR_ID_VIV => "VK_VENDOR_ID_VIV",
      VK_VENDOR_ID_VSI => "VK_VENDOR_ID_VSI",
      other => return write!(f, "VkVendorId({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkVertexInputRate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputRate.html) (enumeration)
  VkVertexInputRate(u32)
);
pub const VK_VERTEX_INPUT_RATE_INSTANCE: VkVertexInputRate = VkVertexInputRate(1);
pub const VK_VERTEX_INPUT_RATE_VERTEX: VkVertexInputRate = VkVertexInputRate(0);
impl core::fmt::Debug for VkVertexInputRate {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VERTEX_INPUT_RATE_INSTANCE => "VK_VERTEX_INPUT_RATE_INSTANCE",
      VK_VERTEX_INPUT_RATE_VERTEX => "VK_VERTEX_INPUT_RATE_VERTEX",
      other => return write!(f, "VkVertexInputRate({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkVideoEncodeH264RateControlStructureEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlStructureEXT.html) (enumeration)
  VkVideoEncodeH264RateControlStructureEXT(u32)
);
pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_DYADIC_EXT:
  VkVideoEncodeH264RateControlStructureEXT = VkVideoEncodeH264RateControlStructureEXT(2);
pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_FLAT_EXT:
  VkVideoEncodeH264RateControlStructureEXT = VkVideoEncodeH264RateControlStructureEXT(1);
pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT:
  VkVideoEncodeH264RateControlStructureEXT = VkVideoEncodeH264RateControlStructureEXT(0);
impl core::fmt::Debug for VkVideoEncodeH264RateControlStructureEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_DYADIC_EXT => {
        "VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_DYADIC_EXT"
      }
      VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_FLAT_EXT => {
        "VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_FLAT_EXT"
      }
      VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT => {
        "VK_VIDEO_ENCODE_H264_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT"
      }
      other => return write!(f, "VkVideoEncodeH264RateControlStructureEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkVideoEncodeH265RateControlStructureEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265RateControlStructureEXT.html) (enumeration)
  VkVideoEncodeH265RateControlStructureEXT(u32)
);
pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_DYADIC_EXT:
  VkVideoEncodeH265RateControlStructureEXT = VkVideoEncodeH265RateControlStructureEXT(2);
pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_FLAT_EXT:
  VkVideoEncodeH265RateControlStructureEXT = VkVideoEncodeH265RateControlStructureEXT(1);
pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT:
  VkVideoEncodeH265RateControlStructureEXT = VkVideoEncodeH265RateControlStructureEXT(0);
impl core::fmt::Debug for VkVideoEncodeH265RateControlStructureEXT {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_DYADIC_EXT => {
        "VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_DYADIC_EXT"
      }
      VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_FLAT_EXT => {
        "VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_FLAT_EXT"
      }
      VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT => {
        "VK_VIDEO_ENCODE_H265_RATE_CONTROL_STRUCTURE_UNKNOWN_EXT"
      }
      other => return write!(f, "VkVideoEncodeH265RateControlStructureEXT({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkVideoEncodeTuningModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeTuningModeKHR.html) (enumeration)
  VkVideoEncodeTuningModeKHR(u32)
);
pub const VK_VIDEO_ENCODE_TUNING_MODE_DEFAULT_KHR: VkVideoEncodeTuningModeKHR =
  VkVideoEncodeTuningModeKHR(0);
pub const VK_VIDEO_ENCODE_TUNING_MODE_HIGH_QUALITY_KHR: VkVideoEncodeTuningModeKHR =
  VkVideoEncodeTuningModeKHR(1);
pub const VK_VIDEO_ENCODE_TUNING_MODE_LOSSLESS_KHR: VkVideoEncodeTuningModeKHR =
  VkVideoEncodeTuningModeKHR(4);
pub const VK_VIDEO_ENCODE_TUNING_MODE_LOW_LATENCY_KHR: VkVideoEncodeTuningModeKHR =
  VkVideoEncodeTuningModeKHR(2);
pub const VK_VIDEO_ENCODE_TUNING_MODE_ULTRA_LOW_LATENCY_KHR: VkVideoEncodeTuningModeKHR =
  VkVideoEncodeTuningModeKHR(3);
impl core::fmt::Debug for VkVideoEncodeTuningModeKHR {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VIDEO_ENCODE_TUNING_MODE_DEFAULT_KHR => {
        "VK_VIDEO_ENCODE_TUNING_MODE_DEFAULT_KHR"
      }
      VK_VIDEO_ENCODE_TUNING_MODE_HIGH_QUALITY_KHR => {
        "VK_VIDEO_ENCODE_TUNING_MODE_HIGH_QUALITY_KHR"
      }
      VK_VIDEO_ENCODE_TUNING_MODE_LOSSLESS_KHR => {
        "VK_VIDEO_ENCODE_TUNING_MODE_LOSSLESS_KHR"
      }
      VK_VIDEO_ENCODE_TUNING_MODE_LOW_LATENCY_KHR => {
        "VK_VIDEO_ENCODE_TUNING_MODE_LOW_LATENCY_KHR"
      }
      VK_VIDEO_ENCODE_TUNING_MODE_ULTRA_LOW_LATENCY_KHR => {
        "VK_VIDEO_ENCODE_TUNING_MODE_ULTRA_LOW_LATENCY_KHR"
      }
      other => return write!(f, "VkVideoEncodeTuningModeKHR({})", other.0),
    };
    write!(f, "{s}")
  }
}

define_enumeration!(
  /// Khronos: [VkViewportCoordinateSwizzleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewportCoordinateSwizzleNV.html) (enumeration)
  VkViewportCoordinateSwizzleNV(u32)
);
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: VkViewportCoordinateSwizzleNV =
  VkViewportCoordinateSwizzleNV(7);
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: VkViewportCoordinateSwizzleNV =
  VkViewportCoordinateSwizzleNV(1);
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: VkViewportCoordinateSwizzleNV =
  VkViewportCoordinateSwizzleNV(3);
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: VkViewportCoordinateSwizzleNV =
  VkViewportCoordinateSwizzleNV(5);
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: VkViewportCoordinateSwizzleNV =
  VkViewportCoordinateSwizzleNV(6);
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: VkViewportCoordinateSwizzleNV =
  VkViewportCoordinateSwizzleNV(0);
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: VkViewportCoordinateSwizzleNV =
  VkViewportCoordinateSwizzleNV(2);
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: VkViewportCoordinateSwizzleNV =
  VkViewportCoordinateSwizzleNV(4);
impl core::fmt::Debug for VkViewportCoordinateSwizzleNV {
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let s = match *self {
      VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV => {
        "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV"
      }
      VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV => {
        "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV"
      }
      VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV => {
        "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV"
      }
      VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV => {
        "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV"
      }
      VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV => {
        "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV"
      }
      VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV => {
        "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV"
      }
      VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV => {
        "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV"
      }
      VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV => {
        "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV"
      }
      other => return write!(f, "VkViewportCoordinateSwizzleNV({})", other.0),
    };
    write!(f, "{s}")
  }
}
