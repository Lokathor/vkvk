#![allow(non_upper_case_globals)]

use crate::prelude::*;

/// Khronos: [VkAccelerationStructureGeometryDataKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkAccelerationStructureGeometryDataKHR {
  /// * Selection: [`VK_GEOMETRY_TYPE_TRIANGLES_KHR`]
  pub triangles: VkAccelerationStructureGeometryTrianglesDataKHR,
  /// * Selection: [`VK_GEOMETRY_TYPE_AABBS_KHR`]
  pub aabbs: VkAccelerationStructureGeometryAabbsDataKHR,
  /// * Selection: [`VK_GEOMETRY_TYPE_INSTANCES_KHR`]
  pub instances: VkAccelerationStructureGeometryInstancesDataKHR,
}

/// Khronos: [VkClearColorValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearColorValue.html)
///
/// Union allowing specification of floating point, integer, or unsigned integer color data. Actual value selected is based on image/attachment being cleared.
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearColorValue {
  pub float32: [c_float; 4],
  pub int32: [i32; 4],
  pub uint32: [u32; 4],
}

/// Khronos: [VkClearValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearValue.html)
///
/// Union allowing specification of color or depth and stencil values. Actual value selected is based on attachment being cleared.
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkClearValue {
  /// * No Auto Validity
  pub color: VkClearColorValue,
  pub depthStencil: VkClearDepthStencilValue,
}

/// Khronos: [VkDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorDataEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDescriptorDataEXT {
  /// * Selection: [`VK_DESCRIPTOR_TYPE_SAMPLER`]
  pub pSampler: *const VkSampler,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`]
  pub pCombinedImageSampler: *const VkDescriptorImageInfo,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`]
  pub pInputAttachmentImage: *const VkDescriptorImageInfo,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`]
  /// * Optional: true
  pub pSampledImage: *const VkDescriptorImageInfo,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`]
  /// * Optional: true
  pub pStorageImage: *const VkDescriptorImageInfo,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`]
  /// * Optional: true
  pub pUniformTexelBuffer: *const VkDescriptorAddressInfoEXT,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`]
  /// * Optional: true
  pub pStorageTexelBuffer: *const VkDescriptorAddressInfoEXT,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`]
  /// * Optional: true
  pub pUniformBuffer: *const VkDescriptorAddressInfoEXT,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`]
  /// * Optional: true
  pub pStorageBuffer: *const VkDescriptorAddressInfoEXT,
  /// * Selection: [`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`]
  /// * Selection: [`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV`]
  pub accelerationStructure: VkDeviceAddress,
}

/// Khronos: [VkDeviceOrHostAddressConstKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressConstKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDeviceOrHostAddressConstKHR {
  /// * No Auto Validity
  pub deviceAddress: VkDeviceAddress,
  /// * No Auto Validity
  pub hostAddress: *const c_void,
}

/// Khronos: [VkDeviceOrHostAddressKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkDeviceOrHostAddressKHR {
  /// * No Auto Validity
  pub deviceAddress: VkDeviceAddress,
  /// * No Auto Validity
  pub hostAddress: *mut c_void,
}

/// Khronos: [VkPerformanceCounterResultKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterResultKHR.html)
///
/// Union of all the possible return types a counter result could return
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkPerformanceCounterResultKHR {
  pub int32: i32,
  pub int64: i64,
  pub uint32: u32,
  pub uint64: u64,
  pub float32: c_float,
  pub float64: c_double,
}

/// Khronos: [VkPerformanceValueDataINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueDataINTEL.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkPerformanceValueDataINTEL {
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL`]
  pub value32: u32,
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL`]
  pub value64: u64,
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL`]
  pub valueFloat: c_float,
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL`]
  pub valueBool: VkBool32,
  /// * Selection: [`VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL`]
  /// * Len: null-terminated
  pub valueString: *const u8,
}

/// Khronos: [VkPipelineExecutableStatisticValueKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub union VkPipelineExecutableStatisticValueKHR {
  /// * Selection: [`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR`]
  pub b32: VkBool32,
  /// * Selection: [`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR`]
  pub i64: i64,
  /// * Selection: [`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR`]
  pub u64: u64,
  /// * Selection: [`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR`]
  pub f64: c_double,
}

