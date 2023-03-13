#![allow(non_upper_case_globals)]

use crate::prelude::*;

/// Khronos: [VkClearColorValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearColorValue.html) (union)
///
/// Union allowing specification of floating point, integer, or unsigned integer
/// color data. Actual value selected is based on image/attachment being
/// cleared.
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearColorValue {
  pub float_32: [c_float; 4],
  pub int_32: [i32; 4],
  pub uint_32: [u32; 4],
}
impl Default for VkClearColorValue {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { float_32: Default::default() }
  }
}

/// Khronos: [VkClearValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearValue.html) (union)
///
/// Union allowing specification of color or depth and stencil values. Actual
/// value selected is based on attachment being cleared.
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearValue {
  /// * No Auto Validity
  pub color: VkClearColorValue,
  pub depth_stencil: VkClearDepthStencilValue,
}
impl Default for VkClearValue {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { color: Default::default() }
  }
}

/// Khronos: [VkDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorDataEXT.html) (union)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDescriptorDataEXT {
  /// * Selection: [`VK_DESCRIPTOR_TYPE_SAMPLER`]
  pub sampler: *const VkSampler,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`]
  pub combined_image_sampler: *const VkDescriptorImageInfo,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`]
  pub input_attachment_image: *const VkDescriptorImageInfo,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`]
  /// * Optional: true
  pub sampled_image: *const VkDescriptorImageInfo,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`]
  /// * Optional: true
  pub storage_image: *const VkDescriptorImageInfo,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`]
  /// * Optional: true
  pub uniform_texel_buffer: *const VkDescriptorAddressInfoEXT,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`]
  /// * Optional: true
  pub storage_texel_buffer: *const VkDescriptorAddressInfoEXT,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`]
  /// * Optional: true
  pub uniform_buffer: *const VkDescriptorAddressInfoEXT,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`]
  /// * Optional: true
  pub storage_buffer: *const VkDescriptorAddressInfoEXT,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`]
  /// * Selection: [`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV`]
  pub acceleration_structure: VkDeviceAddress,
}
impl Default for VkDescriptorDataEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { acceleration_structure: Default::default() }
  }
}

/// Khronos: [VkDeviceOrHostAddressConstKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressConstKHR.html) (union)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDeviceOrHostAddressConstKHR {
  /// * No Auto Validity
  pub device_address: VkDeviceAddress,
  /// * No Auto Validity
  pub host_address: *const c_void,
}
impl Default for VkDeviceOrHostAddressConstKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { device_address: Default::default() }
  }
}

/// Khronos: [VkDeviceOrHostAddressKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressKHR.html) (union)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDeviceOrHostAddressKHR {
  /// * No Auto Validity
  pub device_address: VkDeviceAddress,
  /// * No Auto Validity
  pub host_address: *mut c_void,
}
impl Default for VkDeviceOrHostAddressKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { device_address: Default::default() }
  }
}

/// Khronos: [VkPerformanceCounterResultKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterResultKHR.html) (union)
///
/// Union of all the possible return types a counter result could return
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkPerformanceCounterResultKHR {
  pub int_32: i32,
  pub int_64: i64,
  pub uint_32: u32,
  pub uint_64: u64,
  pub float_32: c_float,
  pub float_64: c_double,
}
impl Default for VkPerformanceCounterResultKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { int_32: Default::default() }
  }
}

/// Khronos: [VkPerformanceValueDataINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueDataINTEL.html) (union)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkPerformanceValueDataINTEL {
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL`]
  pub value_32: u32,
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL`]
  pub value_64: u64,
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL`]
  pub value_float: c_float,
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL`]
  pub value_bool: VkBool32,
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL`]
  /// * Len: null-terminated
  pub value_string: *const u8,
}
impl Default for VkPerformanceValueDataINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { value_32: Default::default() }
  }
}

/// Khronos: [VkPipelineExecutableStatisticValueKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html) (union)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkPipelineExecutableStatisticValueKHR {
  /// * Selection: [`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR`]
  pub b_32: VkBool32,
  /// * Selection: [`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR`]
  pub i_64: i64,
  /// * Selection: [`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR`]
  pub u_64: u64,
  /// * Selection: [`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR`]
  pub f_64: c_double,
}
impl Default for VkPipelineExecutableStatisticValueKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { b_32: Default::default() }
  }
}
