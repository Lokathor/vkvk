#![allow(non_upper_case_globals)]

use crate::prelude::*;

/// Khronos: [VkAabbPositionsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAabbPositionsKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAabbPositionsKHR {
  pub min_x: c_float,
  pub min_y: c_float,
  pub min_z: c_float,
  pub max_x: c_float,
  pub max_y: c_float,
  pub max_z: c_float,
}
impl Default for VkAabbPositionsKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      min_x: Default::default(),
      min_y: Default::default(),
      min_z: Default::default(),
      max_x: Default::default(),
      max_y: Default::default(),
      max_z: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureBuildRangeInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
  pub primitive_count: u32,
  pub primitive_offset: u32,
  pub first_vertex: u32,
  pub transform_offset: u32,
}
impl Default for VkAccelerationStructureBuildRangeInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      primitive_count: Default::default(),
      primitive_offset: Default::default(),
      first_vertex: Default::default(),
      transform_offset: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureBuildSizesInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureBuildSizesInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub acceleration_structure_size: VkDeviceSize,
  pub update_scratch_size: VkDeviceSize,
  pub build_scratch_size: VkDeviceSize,
}
impl Default for VkAccelerationStructureBuildSizesInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR,
      next: core::ptr::null(),
      acceleration_structure_size: Default::default(),
      update_scratch_size: Default::default(),
      build_scratch_size: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCaptureDescriptorDataInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureCaptureDescriptorDataInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub acceleration_structure: VkAccelerationStructureKHR,
  /// * Optional: true
  pub acceleration_structure_nv: VkAccelerationStructureNV,
}
impl Default for VkAccelerationStructureCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      next: core::ptr::null(),
      acceleration_structure: Default::default(),
      acceleration_structure_nv: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureCreateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub create_flags: VkAccelerationStructureCreateFlagsKHR,
  pub buffer: VkBuffer,
  /// Specified in bytes
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  pub ty: VkAccelerationStructureTypeKHR,
  /// * Optional: true
  pub device_address: VkDeviceAddress,
}
impl Default for VkAccelerationStructureCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      create_flags: Default::default(),
      buffer: Default::default(),
      offset: Default::default(),
      size: Default::default(),
      ty: Default::default(),
      device_address: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub compacted_size: VkDeviceSize,
  pub info: VkAccelerationStructureInfoNV,
}
impl Default for VkAccelerationStructureCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV,
      next: core::ptr::null(),
      compacted_size: Default::default(),
      info: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureDeviceAddressInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub acceleration_structure: VkAccelerationStructureKHR,
}
impl Default for VkAccelerationStructureDeviceAddressInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
      next: core::ptr::null(),
      acceleration_structure: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub ty: VkAccelerationStructureTypeNV,
  /// * Optional: true
  pub flags: VkBuildAccelerationStructureFlagsNV,
  /// * Optional: true
  pub instance_count: u32,
  /// * Optional: true
  pub geometry_count: u32,
  /// * Len: `geometry_count`
  pub geometries: *const VkGeometryNV,
}
impl Default for VkAccelerationStructureInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV,
      next: core::ptr::null(),
      ty: Default::default(),
      flags: Default::default(),
      instance_count: Default::default(),
      geometry_count: Default::default(),
      geometries: core::ptr::null(),
    }
  }
}

/// Khronos: [VkAccelerationStructureMemoryRequirementsInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub ty: VkAccelerationStructureMemoryRequirementsTypeNV,
  pub acceleration_structure: VkAccelerationStructureNV,
}
impl Default for VkAccelerationStructureMemoryRequirementsInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
      next: core::ptr::null(),
      ty: Default::default(),
      acceleration_structure: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureMotionInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoNV.html) (structure)
///
/// * Struct Extends: [`VkAccelerationStructureCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureMotionInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub max_instances: u32,
  /// * Optional: true
  pub flags: VkAccelerationStructureMotionInfoFlagsNV,
}
impl Default for VkAccelerationStructureMotionInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV,
      next: core::ptr::null(),
      max_instances: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureVersionInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAccelerationStructureVersionInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Len: `2*vk_uuid_size`
  pub version_data: *const u8,
}
impl Default for VkAccelerationStructureVersionInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR,
      next: core::ptr::null(),
      version_data: core::ptr::null(),
    }
  }
}

/// Khronos: [VkAcquireNextImageInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireNextImageInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAcquireNextImageInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub swapchain: VkSwapchainKHR,
  pub timeout: u64,
  /// * Optional: true
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  /// * Extern Sync: true
  pub fence: VkFence,
  pub device_mask: u32,
}
impl Default for VkAcquireNextImageInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR,
      next: core::ptr::null(),
      swapchain: Default::default(),
      timeout: Default::default(),
      semaphore: Default::default(),
      fence: Default::default(),
      device_mask: Default::default(),
    }
  }
}

/// Khronos: [VkAcquireProfilingLockInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAcquireProfilingLockInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Acquire profiling lock flags
  /// * Optional: true
  pub flags: VkAcquireProfilingLockFlagsKHR,
  pub timeout: u64,
}
impl Default for VkAcquireProfilingLockInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      timeout: Default::default(),
    }
  }
}

/// Khronos: [VkAllocationCallbacks](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAllocationCallbacks.html) (structure)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAllocationCallbacks {
  /// * Optional: true
  pub user_data: *mut c_void,
  /// * No Auto-Validity
  pub pfn_allocation: PFN_vkAllocationFunction,
  /// * No Auto-Validity
  pub pfn_reallocation: PFN_vkReallocationFunction,
  /// * No Auto-Validity
  pub pfn_free: PFN_vkFreeFunction,
  /// * Optional: true
  /// * No Auto-Validity
  pub pfn_internal_allocation: PFN_vkInternalAllocationNotification,
  /// * Optional: true
  /// * No Auto-Validity
  pub pfn_internal_free: PFN_vkInternalFreeNotification,
}
impl Default for VkAllocationCallbacks {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      user_data: core::ptr::null_mut(),
      pfn_allocation: Default::default(),
      pfn_reallocation: Default::default(),
      pfn_free: Default::default(),
      pfn_internal_allocation: Default::default(),
      pfn_internal_free: Default::default(),
    }
  }
}

/// Khronos: [VkAmigoProfilingSubmitInfoSEC](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAmigoProfilingSubmitInfoSEC.html) (structure)
///
/// * Struct Extends: [`VkSubmitInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAmigoProfilingSubmitInfoSEC {
  /// * Intended Value: `VK_STRUCTURE_TYPE_AMIGO_PROFILING_SUBMIT_INFO_SEC`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub first_draw_timestamp: u64,
  pub swap_buffer_timestamp: u64,
}
impl Default for VkAmigoProfilingSubmitInfoSEC {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_AMIGO_PROFILING_SUBMIT_INFO_SEC,
      next: core::ptr::null(),
      first_draw_timestamp: Default::default(),
      swap_buffer_timestamp: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidHardwareBufferFormatProperties2ANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatProperties2ANDROID.html) (structure)
///
/// * Struct Extends: [`VkAndroidHardwareBufferPropertiesANDROID`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAndroidHardwareBufferFormatProperties2ANDROID {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub format: VkFormat,
  pub external_format: u64,
  pub format_features: VkFormatFeatureFlags2,
  pub sampler_ycbcr_conversion_components: VkComponentMapping,
  pub suggested_ycbcr_model: VkSamplerYcbcrModelConversion,
  pub suggested_ycbcr_range: VkSamplerYcbcrRange,
  pub suggested_x_chroma_offset: VkChromaLocation,
  pub suggested_y_chroma_offset: VkChromaLocation,
}
impl Default for VkAndroidHardwareBufferFormatProperties2ANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID,
      next: core::ptr::null_mut(),
      format: Default::default(),
      external_format: Default::default(),
      format_features: Default::default(),
      sampler_ycbcr_conversion_components: Default::default(),
      suggested_ycbcr_model: Default::default(),
      suggested_ycbcr_range: Default::default(),
      suggested_x_chroma_offset: Default::default(),
      suggested_y_chroma_offset: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidHardwareBufferFormatPropertiesANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html) (structure)
///
/// * Struct Extends: [`VkAndroidHardwareBufferPropertiesANDROID`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAndroidHardwareBufferFormatPropertiesANDROID {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub format: VkFormat,
  pub external_format: u64,
  pub format_features: VkFormatFeatureFlags,
  pub sampler_ycbcr_conversion_components: VkComponentMapping,
  pub suggested_ycbcr_model: VkSamplerYcbcrModelConversion,
  pub suggested_ycbcr_range: VkSamplerYcbcrRange,
  pub suggested_x_chroma_offset: VkChromaLocation,
  pub suggested_y_chroma_offset: VkChromaLocation,
}
impl Default for VkAndroidHardwareBufferFormatPropertiesANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID,
      next: core::ptr::null_mut(),
      format: Default::default(),
      external_format: Default::default(),
      format_features: Default::default(),
      sampler_ycbcr_conversion_components: Default::default(),
      suggested_ycbcr_model: Default::default(),
      suggested_ycbcr_range: Default::default(),
      suggested_x_chroma_offset: Default::default(),
      suggested_y_chroma_offset: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidHardwareBufferPropertiesANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferPropertiesANDROID.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAndroidHardwareBufferPropertiesANDROID {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub allocation_size: VkDeviceSize,
  pub memory_type_bits: u32,
}
impl Default for VkAndroidHardwareBufferPropertiesANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
      next: core::ptr::null_mut(),
      allocation_size: Default::default(),
      memory_type_bits: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidHardwareBufferUsageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html) (structure)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAndroidHardwareBufferUsageANDROID {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub android_hardware_buffer_usage: u64,
}
impl Default for VkAndroidHardwareBufferUsageANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
      next: core::ptr::null_mut(),
      android_hardware_buffer_usage: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidSurfaceCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAndroidSurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkAndroidSurfaceCreateFlagsKHR,
  /// * No Auto-Validity
  pub window: *mut ANativeWindow,
}
impl Default for VkAndroidSurfaceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      window: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkApplicationInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkApplicationInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkApplicationInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_APPLICATION_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  /// * Len: `null_terminated`
  pub application_name: *const u8,
  pub application_version: u32,
  /// * Optional: true
  /// * Len: `null_terminated`
  pub engine_name: *const u8,
  pub engine_version: u32,
  pub api_version: u32,
}
impl Default for VkApplicationInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_APPLICATION_INFO,
      next: core::ptr::null(),
      application_name: core::ptr::null(),
      application_version: Default::default(),
      engine_name: core::ptr::null(),
      engine_version: Default::default(),
      api_version: Default::default(),
    }
  }
}

/// Khronos: [VkApplicationParametersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkApplicationParametersEXT.html) (structure)
///
/// * Struct Extends: [`VkApplicationInfo`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkApplicationParametersEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_APPLICATION_PARAMETERS_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub vendor_id: u32,
  /// * Optional: true
  pub device_id: u32,
  pub key: u32,
  pub value: u64,
}
impl Default for VkApplicationParametersEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_APPLICATION_PARAMETERS_EXT,
      next: core::ptr::null(),
      vendor_id: Default::default(),
      device_id: Default::default(),
      key: Default::default(),
      value: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAttachmentDescription {
  /// * Optional: true
  pub flags: VkAttachmentDescriptionFlags,
  pub format: VkFormat,
  pub samples: VkSampleCountFlagBits,
  /// Load operation for color or depth data
  pub load_op: VkAttachmentLoadOp,
  /// Store operation for color or depth data
  pub store_op: VkAttachmentStoreOp,
  /// Load operation for stencil data
  pub stencil_load_op: VkAttachmentLoadOp,
  /// Store operation for stencil data
  pub stencil_store_op: VkAttachmentStoreOp,
  pub initial_layout: VkImageLayout,
  pub final_layout: VkImageLayout,
}
impl Default for VkAttachmentDescription {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      flags: Default::default(),
      format: Default::default(),
      samples: Default::default(),
      load_op: Default::default(),
      store_op: Default::default(),
      stencil_load_op: Default::default(),
      stencil_store_op: Default::default(),
      initial_layout: Default::default(),
      final_layout: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentDescription2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAttachmentDescription2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkAttachmentDescriptionFlags,
  pub format: VkFormat,
  pub samples: VkSampleCountFlagBits,
  /// Load operation for color or depth data
  pub load_op: VkAttachmentLoadOp,
  /// Store operation for color or depth data
  pub store_op: VkAttachmentStoreOp,
  /// Load operation for stencil data
  pub stencil_load_op: VkAttachmentLoadOp,
  /// Store operation for stencil data
  pub stencil_store_op: VkAttachmentStoreOp,
  pub initial_layout: VkImageLayout,
  pub final_layout: VkImageLayout,
}
impl Default for VkAttachmentDescription2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2,
      next: core::ptr::null(),
      flags: Default::default(),
      format: Default::default(),
      samples: Default::default(),
      load_op: Default::default(),
      store_op: Default::default(),
      stencil_load_op: Default::default(),
      stencil_store_op: Default::default(),
      initial_layout: Default::default(),
      final_layout: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentDescriptionStencilLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionStencilLayout.html) (structure)
///
/// * Struct Extends: [`VkAttachmentDescription2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAttachmentDescriptionStencilLayout {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub stencil_initial_layout: VkImageLayout,
  pub stencil_final_layout: VkImageLayout,
}
impl Default for VkAttachmentDescriptionStencilLayout {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT,
      next: core::ptr::null_mut(),
      stencil_initial_layout: Default::default(),
      stencil_final_layout: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAttachmentReference {
  pub attachment: u32,
  pub layout: VkImageLayout,
}
impl Default for VkAttachmentReference {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { attachment: Default::default(), layout: Default::default() }
  }
}

/// Khronos: [VkAttachmentReference2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAttachmentReference2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub attachment: u32,
  pub layout: VkImageLayout,
  /// * No Auto-Validity
  pub aspect_mask: VkImageAspectFlags,
}
impl Default for VkAttachmentReference2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2,
      next: core::ptr::null(),
      attachment: Default::default(),
      layout: Default::default(),
      aspect_mask: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentReferenceStencilLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentReferenceStencilLayout.html) (structure)
///
/// * Struct Extends: [`VkAttachmentReference2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAttachmentReferenceStencilLayout {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub stencil_layout: VkImageLayout,
}
impl Default for VkAttachmentReferenceStencilLayout {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT,
      next: core::ptr::null_mut(),
      stencil_layout: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentSampleCountInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleCountInfoAMD.html) (structure)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAttachmentSampleCountInfoAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub color_attachment_count: u32,
  /// * Len: `color_attachment_count`
  /// * No Auto-Validity
  pub color_attachment_samples: *const VkSampleCountFlagBits,
  /// * Optional: true
  /// * No Auto-Validity
  pub depth_stencil_attachment_samples: VkSampleCountFlagBits,
}
impl Default for VkAttachmentSampleCountInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD,
      next: core::ptr::null(),
      color_attachment_count: Default::default(),
      color_attachment_samples: core::ptr::null(),
      depth_stencil_attachment_samples: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleLocationsEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkAttachmentSampleLocationsEXT {
  pub attachment_index: u32,
  pub sample_locations_info: VkSampleLocationsInfoEXT,
}
impl Default for VkAttachmentSampleLocationsEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      attachment_index: Default::default(),
      sample_locations_info: Default::default(),
    }
  }
}

/// Khronos: [VkBaseInStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseInStructure.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBaseInStructure {
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const VkBaseInStructure,
}
impl Default for VkBaseInStructure {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { struct_ty: Default::default(), next: core::ptr::null() }
  }
}

/// Khronos: [VkBaseOutStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseOutStructure.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBaseOutStructure {
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut VkBaseOutStructure,
}
impl Default for VkBaseOutStructure {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { struct_ty: Default::default(), next: core::ptr::null_mut() }
  }
}

/// Khronos: [VkBindAccelerationStructureMemoryInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindAccelerationStructureMemoryInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub acceleration_structure: VkAccelerationStructureNV,
  pub memory: VkDeviceMemory,
  pub memory_offset: VkDeviceSize,
  /// * Optional: true
  pub device_index_count: u32,
  /// * Len: `device_index_count`
  pub device_indices: *const u32,
}
impl Default for VkBindAccelerationStructureMemoryInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV,
      next: core::ptr::null(),
      acceleration_structure: Default::default(),
      memory: Default::default(),
      memory_offset: Default::default(),
      device_index_count: Default::default(),
      device_indices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBindBufferMemoryDeviceGroupInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html) (structure)
///
/// * Struct Extends: [`VkBindBufferMemoryInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindBufferMemoryDeviceGroupInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub device_index_count: u32,
  /// * Len: `device_index_count`
  pub device_indices: *const u32,
}
impl Default for VkBindBufferMemoryDeviceGroupInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO,
      next: core::ptr::null(),
      device_index_count: Default::default(),
      device_indices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBindBufferMemoryInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindBufferMemoryInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub buffer: VkBuffer,
  pub memory: VkDeviceMemory,
  pub memory_offset: VkDeviceSize,
}
impl Default for VkBindBufferMemoryInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO,
      next: core::ptr::null(),
      buffer: Default::default(),
      memory: Default::default(),
      memory_offset: Default::default(),
    }
  }
}

/// Khronos: [VkBindImageMemoryDeviceGroupInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html) (structure)
///
/// * Struct Extends: [`VkBindImageMemoryInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindImageMemoryDeviceGroupInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub device_index_count: u32,
  /// * Len: `device_index_count`
  pub device_indices: *const u32,
  /// * Optional: true
  pub split_instance_bind_region_count: u32,
  /// * Len: `split_instance_bind_region_count`
  pub split_instance_bind_regions: *const VkRect2D,
}
impl Default for VkBindImageMemoryDeviceGroupInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO,
      next: core::ptr::null(),
      device_index_count: Default::default(),
      device_indices: core::ptr::null(),
      split_instance_bind_region_count: Default::default(),
      split_instance_bind_regions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBindImageMemoryInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindImageMemoryInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image: VkImage,
  /// * No Auto-Validity
  pub memory: VkDeviceMemory,
  pub memory_offset: VkDeviceSize,
}
impl Default for VkBindImageMemoryInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO,
      next: core::ptr::null(),
      image: Default::default(),
      memory: Default::default(),
      memory_offset: Default::default(),
    }
  }
}

/// Khronos: [VkBindImageMemorySwapchainInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkBindImageMemoryInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindImageMemorySwapchainInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub swapchain: VkSwapchainKHR,
  pub image_index: u32,
}
impl Default for VkBindImageMemorySwapchainInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
      next: core::ptr::null(),
      swapchain: Default::default(),
      image_index: Default::default(),
    }
  }
}

/// Khronos: [VkBindImagePlaneMemoryInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImagePlaneMemoryInfo.html) (structure)
///
/// * Struct Extends: [`VkBindImageMemoryInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindImagePlaneMemoryInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub plane_aspect: VkImageAspectFlagBits,
}
impl Default for VkBindImagePlaneMemoryInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO,
      next: core::ptr::null(),
      plane_aspect: Default::default(),
    }
  }
}

/// Khronos: [VkBindIndexBufferIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindIndexBufferIndirectCommandNV {
  pub buffer_address: VkDeviceAddress,
  pub size: u32,
  pub index_type: VkIndexType,
}
impl Default for VkBindIndexBufferIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      buffer_address: Default::default(),
      size: Default::default(),
      index_type: Default::default(),
    }
  }
}

/// Khronos: [VkBindShaderGroupIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindShaderGroupIndirectCommandNV {
  pub group_index: u32,
}
impl Default for VkBindShaderGroupIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { group_index: Default::default() }
  }
}

/// Khronos: [VkBindSparseInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindSparseInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindSparseInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_SPARSE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub wait_semaphore_count: u32,
  /// * Len: `wait_semaphore_count`
  pub wait_semaphores: *const VkSemaphore,
  /// * Optional: true
  pub buffer_bind_count: u32,
  /// * Len: `buffer_bind_count`
  pub buffer_binds: *const VkSparseBufferMemoryBindInfo,
  /// * Optional: true
  pub image_opaque_bind_count: u32,
  /// * Len: `image_opaque_bind_count`
  pub image_opaque_binds: *const VkSparseImageOpaqueMemoryBindInfo,
  /// * Optional: true
  pub image_bind_count: u32,
  /// * Len: `image_bind_count`
  pub image_binds: *const VkSparseImageMemoryBindInfo,
  /// * Optional: true
  pub signal_semaphore_count: u32,
  /// * Len: `signal_semaphore_count`
  pub signal_semaphores: *const VkSemaphore,
}
impl Default for VkBindSparseInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BIND_SPARSE_INFO,
      next: core::ptr::null(),
      wait_semaphore_count: Default::default(),
      wait_semaphores: core::ptr::null(),
      buffer_bind_count: Default::default(),
      buffer_binds: core::ptr::null(),
      image_opaque_bind_count: Default::default(),
      image_opaque_binds: core::ptr::null(),
      image_bind_count: Default::default(),
      image_binds: core::ptr::null(),
      signal_semaphore_count: Default::default(),
      signal_semaphores: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBindVertexBufferIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindVertexBufferIndirectCommandNV {
  pub buffer_address: VkDeviceAddress,
  pub size: u32,
  pub stride: u32,
}
impl Default for VkBindVertexBufferIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      buffer_address: Default::default(),
      size: Default::default(),
      stride: Default::default(),
    }
  }
}

/// Khronos: [VkBindVideoSessionMemoryInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindVideoSessionMemoryInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBindVideoSessionMemoryInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_VIDEO_SESSION_MEMORY_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory_bind_index: u32,
  pub memory: VkDeviceMemory,
  pub memory_offset: VkDeviceSize,
  pub memory_size: VkDeviceSize,
}
impl Default for VkBindVideoSessionMemoryInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BIND_VIDEO_SESSION_MEMORY_INFO_KHR,
      next: core::ptr::null(),
      memory_bind_index: Default::default(),
      memory: Default::default(),
      memory_offset: Default::default(),
      memory_size: Default::default(),
    }
  }
}

/// Khronos: [VkBlitImageInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlitImageInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBlitImageInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_image: VkImage,
  pub src_image_layout: VkImageLayout,
  pub dst_image: VkImage,
  pub dst_image_layout: VkImageLayout,
  pub region_count: u32,
  /// * Len: `region_count`
  pub regions: *const VkImageBlit2,
  pub filter: VkFilter,
}
impl Default for VkBlitImageInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2,
      next: core::ptr::null(),
      src_image: Default::default(),
      src_image_layout: Default::default(),
      dst_image: Default::default(),
      dst_image_layout: Default::default(),
      region_count: Default::default(),
      regions: core::ptr::null(),
      filter: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCaptureDescriptorDataInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferCaptureDescriptorDataInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub buffer: VkBuffer,
}
impl Default for VkBufferCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      next: core::ptr::null(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionBufferCreateInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionBufferCreateInfoFUCHSIA.html) (structure)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferCollectionBufferCreateInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub collection: VkBufferCollectionFUCHSIA,
  pub index: u32,
}
impl Default for VkBufferCollectionBufferCreateInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA,
      next: core::ptr::null(),
      collection: Default::default(),
      index: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionConstraintsInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionConstraintsInfoFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferCollectionConstraintsInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub min_buffer_count: u32,
  pub max_buffer_count: u32,
  pub min_buffer_count_for_camping: u32,
  pub min_buffer_count_for_dedicated_slack: u32,
  pub min_buffer_count_for_shared_slack: u32,
}
impl Default for VkBufferCollectionConstraintsInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA,
      next: core::ptr::null(),
      min_buffer_count: Default::default(),
      max_buffer_count: Default::default(),
      min_buffer_count_for_camping: Default::default(),
      min_buffer_count_for_dedicated_slack: Default::default(),
      min_buffer_count_for_shared_slack: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionCreateInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionCreateInfoFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferCollectionCreateInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub collection_token: zx_handle_t,
}
impl Default for VkBufferCollectionCreateInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA,
      next: core::ptr::null(),
      collection_token: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionImageCreateInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionImageCreateInfoFUCHSIA.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferCollectionImageCreateInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub collection: VkBufferCollectionFUCHSIA,
  pub index: u32,
}
impl Default for VkBufferCollectionImageCreateInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA,
      next: core::ptr::null(),
      collection: Default::default(),
      index: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionPropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionPropertiesFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferCollectionPropertiesFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_type_bits: u32,
  pub buffer_count: u32,
  pub create_info_index: u32,
  pub sysmem_pixel_format: u64,
  pub format_features: VkFormatFeatureFlags,
  pub sysmem_color_space_index: VkSysmemColorSpaceFUCHSIA,
  pub sampler_ycbcr_conversion_components: VkComponentMapping,
  pub suggested_ycbcr_model: VkSamplerYcbcrModelConversion,
  pub suggested_ycbcr_range: VkSamplerYcbcrRange,
  pub suggested_x_chroma_offset: VkChromaLocation,
  pub suggested_y_chroma_offset: VkChromaLocation,
}
impl Default for VkBufferCollectionPropertiesFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA,
      next: core::ptr::null_mut(),
      memory_type_bits: Default::default(),
      buffer_count: Default::default(),
      create_info_index: Default::default(),
      sysmem_pixel_format: Default::default(),
      format_features: Default::default(),
      sysmem_color_space_index: Default::default(),
      sampler_ycbcr_conversion_components: Default::default(),
      suggested_ycbcr_model: Default::default(),
      suggested_ycbcr_range: Default::default(),
      suggested_x_chroma_offset: Default::default(),
      suggested_y_chroma_offset: Default::default(),
    }
  }
}

/// Khronos: [VkBufferConstraintsInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferConstraintsInfoFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferConstraintsInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub create_info: VkBufferCreateInfo,
  /// * Optional: true
  pub required_format_features: VkFormatFeatureFlags,
  pub buffer_collection_constraints: VkBufferCollectionConstraintsInfoFUCHSIA,
}
impl Default for VkBufferConstraintsInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA,
      next: core::ptr::null(),
      create_info: Default::default(),
      required_format_features: Default::default(),
      buffer_collection_constraints: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCopy](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCopy.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferCopy {
  /// Specified in bytes
  pub src_offset: VkDeviceSize,
  /// Specified in bytes
  pub dst_offset: VkDeviceSize,
  /// Specified in bytes
  /// * No Auto-Validity
  pub size: VkDeviceSize,
}
impl Default for VkBufferCopy {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      src_offset: Default::default(),
      dst_offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCopy2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCopy2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferCopy2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_COPY_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Specified in bytes
  pub src_offset: VkDeviceSize,
  /// Specified in bytes
  pub dst_offset: VkDeviceSize,
  /// Specified in bytes
  /// * No Auto-Validity
  pub size: VkDeviceSize,
}
impl Default for VkBufferCopy2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_COPY_2,
      next: core::ptr::null(),
      src_offset: Default::default(),
      dst_offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Buffer creation flags
  /// * Optional: true
  pub flags: VkBufferCreateFlags,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Buffer usage flags
  pub usage: VkBufferUsageFlags,
  pub sharing_mode: VkSharingMode,
  /// * Optional: true
  pub queue_family_index_count: u32,
  /// * Len: `queue_family_index_count`
  /// * No Auto-Validity
  pub queue_family_indices: *const u32,
}
impl Default for VkBufferCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      size: Default::default(),
      usage: Default::default(),
      sharing_mode: Default::default(),
      queue_family_index_count: Default::default(),
      queue_family_indices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBufferDeviceAddressCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferDeviceAddressCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub device_address: VkDeviceAddress,
}
impl Default for VkBufferDeviceAddressCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT,
      next: core::ptr::null(),
      device_address: Default::default(),
    }
  }
}

/// Khronos: [VkBufferDeviceAddressInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferDeviceAddressInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub buffer: VkBuffer,
}
impl Default for VkBufferDeviceAddressInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO,
      next: core::ptr::null(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkBufferImageCopy](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferImageCopy {
  /// Specified in bytes
  pub buffer_offset: VkDeviceSize,
  /// Specified in texels
  pub buffer_row_length: u32,
  pub buffer_image_height: u32,
  pub image_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub image_offset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub image_extent: VkExtent3D,
}
impl Default for VkBufferImageCopy {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      buffer_offset: Default::default(),
      buffer_row_length: Default::default(),
      buffer_image_height: Default::default(),
      image_subresource: Default::default(),
      image_offset: Default::default(),
      image_extent: Default::default(),
    }
  }
}

/// Khronos: [VkBufferImageCopy2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferImageCopy2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Specified in bytes
  pub buffer_offset: VkDeviceSize,
  /// Specified in texels
  pub buffer_row_length: u32,
  pub buffer_image_height: u32,
  pub image_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub image_offset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub image_extent: VkExtent3D,
}
impl Default for VkBufferImageCopy2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2,
      next: core::ptr::null(),
      buffer_offset: Default::default(),
      buffer_row_length: Default::default(),
      buffer_image_height: Default::default(),
      image_subresource: Default::default(),
      image_offset: Default::default(),
      image_extent: Default::default(),
    }
  }
}

/// Khronos: [VkBufferMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferMemoryBarrier {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * No Auto-Validity
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * No Auto-Validity
  pub dst_access_mask: VkAccessFlags,
  /// Queue family to transition ownership from
  pub src_queue_family_index: u32,
  /// Queue family to transition ownership to
  pub dst_queue_family_index: u32,
  /// Buffer to sync
  pub buffer: VkBuffer,
  /// Offset within the buffer to sync
  pub offset: VkDeviceSize,
  /// Amount of bytes to sync
  pub size: VkDeviceSize,
}
impl Default for VkBufferMemoryBarrier {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
      next: core::ptr::null(),
      src_access_mask: Default::default(),
      dst_access_mask: Default::default(),
      src_queue_family_index: Default::default(),
      dst_queue_family_index: Default::default(),
      buffer: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkBufferMemoryBarrier2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferMemoryBarrier2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub src_stage_mask: VkPipelineStageFlags2,
  /// * Optional: true
  pub src_access_mask: VkAccessFlags2,
  /// * Optional: true
  pub dst_stage_mask: VkPipelineStageFlags2,
  /// * Optional: true
  pub dst_access_mask: VkAccessFlags2,
  pub src_queue_family_index: u32,
  pub dst_queue_family_index: u32,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
}
impl Default for VkBufferMemoryBarrier2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2,
      next: core::ptr::null(),
      src_stage_mask: Default::default(),
      src_access_mask: Default::default(),
      dst_stage_mask: Default::default(),
      dst_access_mask: Default::default(),
      src_queue_family_index: Default::default(),
      dst_queue_family_index: Default::default(),
      buffer: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkBufferMemoryRequirementsInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryRequirementsInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferMemoryRequirementsInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub buffer: VkBuffer,
}
impl Default for VkBufferMemoryRequirementsInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2,
      next: core::ptr::null(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkBufferOpaqueCaptureAddressCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferOpaqueCaptureAddressCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub opaque_capture_address: u64,
}
impl Default for VkBufferOpaqueCaptureAddressCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO,
      next: core::ptr::null(),
      opaque_capture_address: Default::default(),
    }
  }
}

/// Khronos: [VkBufferViewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkBufferViewCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkBufferViewCreateFlags,
  pub buffer: VkBuffer,
  /// Optionally specifies format of elements
  pub format: VkFormat,
  /// Specified in bytes
  pub offset: VkDeviceSize,
  /// View size specified in bytes
  pub range: VkDeviceSize,
}
impl Default for VkBufferViewCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      buffer: Default::default(),
      format: Default::default(),
      offset: Default::default(),
      range: Default::default(),
    }
  }
}

/// Khronos: [VkCalibratedTimestampInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCalibratedTimestampInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCalibratedTimestampInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub time_domain: VkTimeDomainEXT,
}
impl Default for VkCalibratedTimestampInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT,
      next: core::ptr::null(),
      time_domain: Default::default(),
    }
  }
}

/// Khronos: [VkCheckpointData2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCheckpointData2NV.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCheckpointData2NV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub stage: VkPipelineStageFlags2,
  /// * No Auto-Validity
  pub checkpoint_marker: *mut c_void,
}
impl Default for VkCheckpointData2NV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV,
      next: core::ptr::null_mut(),
      stage: Default::default(),
      checkpoint_marker: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkCheckpointDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCheckpointDataNV.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCheckpointDataNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub stage: VkPipelineStageFlagBits,
  /// * No Auto-Validity
  pub checkpoint_marker: *mut c_void,
}
impl Default for VkCheckpointDataNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV,
      next: core::ptr::null_mut(),
      stage: Default::default(),
      checkpoint_marker: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkClearDepthStencilValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearDepthStencilValue.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkClearDepthStencilValue {
  pub depth: c_float,
  pub stencil: u32,
}
impl Default for VkClearDepthStencilValue {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { depth: Default::default(), stencil: Default::default() }
  }
}

/// Khronos: [VkClearRect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearRect.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkClearRect {
  pub rect: VkRect2D,
  pub base_array_layer: u32,
  pub layer_count: u32,
}
impl Default for VkClearRect {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      rect: Default::default(),
      base_array_layer: Default::default(),
      layer_count: Default::default(),
    }
  }
}

/// Khronos: [VkCoarseSampleLocationNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleLocationNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCoarseSampleLocationNV {
  pub pixel_x: u32,
  pub pixel_y: u32,
  pub sample: u32,
}
impl Default for VkCoarseSampleLocationNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      pixel_x: Default::default(),
      pixel_y: Default::default(),
      sample: Default::default(),
    }
  }
}

/// Khronos: [VkCoarseSampleOrderCustomNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderCustomNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCoarseSampleOrderCustomNV {
  pub shading_rate: VkShadingRatePaletteEntryNV,
  pub sample_count: u32,
  pub sample_location_count: u32,
  /// * Len: `sample_location_count`
  pub sample_locations: *const VkCoarseSampleLocationNV,
}
impl Default for VkCoarseSampleOrderCustomNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      shading_rate: Default::default(),
      sample_count: Default::default(),
      sample_location_count: Default::default(),
      sample_locations: core::ptr::null(),
    }
  }
}

/// Khronos: [VkColorBlendAdvancedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorBlendAdvancedEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkColorBlendAdvancedEXT {
  pub advanced_blend_op: VkBlendOp,
  pub src_premultiplied: VkBool32,
  pub dst_premultiplied: VkBool32,
  pub blend_overlap: VkBlendOverlapEXT,
  pub clamp_results: VkBool32,
}
impl Default for VkColorBlendAdvancedEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      advanced_blend_op: Default::default(),
      src_premultiplied: Default::default(),
      dst_premultiplied: Default::default(),
      blend_overlap: Default::default(),
      clamp_results: Default::default(),
    }
  }
}

/// Khronos: [VkColorBlendEquationEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorBlendEquationEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkColorBlendEquationEXT {
  pub src_color_blend_factor: VkBlendFactor,
  pub dst_color_blend_factor: VkBlendFactor,
  pub color_blend_op: VkBlendOp,
  pub src_alpha_blend_factor: VkBlendFactor,
  pub dst_alpha_blend_factor: VkBlendFactor,
  pub alpha_blend_op: VkBlendOp,
}
impl Default for VkColorBlendEquationEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      src_color_blend_factor: Default::default(),
      dst_color_blend_factor: Default::default(),
      color_blend_op: Default::default(),
      src_alpha_blend_factor: Default::default(),
      dst_alpha_blend_factor: Default::default(),
      alpha_blend_op: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferAllocateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub command_pool: VkCommandPool,
  pub level: VkCommandBufferLevel,
  pub command_buffer_count: u32,
}
impl Default for VkCommandBufferAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
      next: core::ptr::null(),
      command_pool: Default::default(),
      level: Default::default(),
      command_buffer_count: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferBeginInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCommandBufferBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Command buffer usage flags
  /// * Optional: true
  pub flags: VkCommandBufferUsageFlags,
  /// Pointer to inheritance info for secondary command buffers
  /// * Optional: true
  /// * No Auto-Validity
  pub inheritance_info: *const VkCommandBufferInheritanceInfo,
}
impl Default for VkCommandBufferBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      inheritance_info: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceConditionalRenderingInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Whether this secondary command buffer may be executed during an active
  /// conditional rendering
  pub conditional_rendering_enable: VkBool32,
}
impl Default for VkCommandBufferInheritanceConditionalRenderingInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT,
      next: core::ptr::null(),
      conditional_rendering_enable: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Render pass for secondary command buffers
  /// * Optional: true
  /// * No Auto-Validity
  pub render_pass: VkRenderPass,
  pub subpass: u32,
  /// Framebuffer for secondary command buffers
  /// * Optional: true
  /// * No Auto-Validity
  pub framebuffer: VkFramebuffer,
  /// Whether this secondary command buffer may be executed during an occlusion
  /// query
  pub occlusion_query_enable: VkBool32,
  /// Query flags used by this secondary command buffer, if executed during an
  /// occlusion query
  /// * Optional: true
  /// * No Auto-Validity
  pub query_flags: VkQueryControlFlags,
  /// Pipeline statistics that may be counted for this secondary command buffer
  /// * Optional: true
  /// * No Auto-Validity
  pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}
impl Default for VkCommandBufferInheritanceInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
      next: core::ptr::null(),
      render_pass: Default::default(),
      subpass: Default::default(),
      framebuffer: Default::default(),
      occlusion_query_enable: Default::default(),
      query_flags: Default::default(),
      pipeline_statistics: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceRenderPassTransformInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html) (structure)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM`
  pub struct_ty: VkStructureType,
  /// Pointer to next structure
  /// * Optional: true
  pub next: *mut c_void,
  /// * No Auto-Validity
  pub transform: VkSurfaceTransformFlagBitsKHR,
  pub render_area: VkRect2D,
}
impl Default for VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM,
      next: core::ptr::null_mut(),
      transform: Default::default(),
      render_area: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceRenderingInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderingInfo.html) (structure)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCommandBufferInheritanceRenderingInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkRenderingFlags,
  pub view_mask: u32,
  /// * Optional: true
  pub color_attachment_count: u32,
  /// * Len: `color_attachment_count`
  pub color_attachment_formats: *const VkFormat,
  pub depth_attachment_format: VkFormat,
  pub stencil_attachment_format: VkFormat,
  /// * Optional: true
  pub rasterization_samples: VkSampleCountFlagBits,
}
impl Default for VkCommandBufferInheritanceRenderingInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      view_mask: Default::default(),
      color_attachment_count: Default::default(),
      color_attachment_formats: core::ptr::null(),
      depth_attachment_format: Default::default(),
      stencil_attachment_format: Default::default(),
      rasterization_samples: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceViewportScissorInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceViewportScissorInfoNV.html) (structure)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCommandBufferInheritanceViewportScissorInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub viewport_scissor_2_d: VkBool32,
  pub viewport_depth_count: u32,
  /// * No Auto-Validity
  pub viewport_depths: *const VkViewport,
}
impl Default for VkCommandBufferInheritanceViewportScissorInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV,
      next: core::ptr::null(),
      viewport_scissor_2_d: Default::default(),
      viewport_depth_count: Default::default(),
      viewport_depths: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCommandBufferSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferSubmitInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCommandBufferSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub command_buffer: VkCommandBuffer,
  pub device_mask: u32,
}
impl Default for VkCommandBufferSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO,
      next: core::ptr::null(),
      command_buffer: Default::default(),
      device_mask: Default::default(),
    }
  }
}

/// Khronos: [VkCommandPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCommandPoolCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Command pool creation flags
  /// * Optional: true
  pub flags: VkCommandPoolCreateFlags,
  pub queue_family_index: u32,
}
impl Default for VkCommandPoolCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      queue_family_index: Default::default(),
    }
  }
}

/// Khronos: [VkComponentMapping](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentMapping.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkComponentMapping {
  pub r: VkComponentSwizzle,
  pub g: VkComponentSwizzle,
  pub b: VkComponentSwizzle,
  pub a: VkComponentSwizzle,
}
impl Default for VkComponentMapping {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      r: Default::default(),
      g: Default::default(),
      b: Default::default(),
      a: Default::default(),
    }
  }
}

/// Khronos: [VkComputePipelineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComputePipelineCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkComputePipelineCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Pipeline creation flags
  /// * Optional: true
  pub flags: VkPipelineCreateFlags,
  pub stage: VkPipelineShaderStageCreateInfo,
  /// Interface layout of the pipeline
  pub layout: VkPipelineLayout,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * Optional: true
  /// * No Auto-Validity
  pub base_pipeline_handle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  pub base_pipeline_index: i32,
}
impl Default for VkComputePipelineCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      stage: Default::default(),
      layout: Default::default(),
      base_pipeline_handle: Default::default(),
      base_pipeline_index: Default::default(),
    }
  }
}

/// Khronos: [VkConditionalRenderingBeginInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkConditionalRenderingBeginInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  /// * Optional: true
  pub flags: VkConditionalRenderingFlagsEXT,
}
impl Default for VkConditionalRenderingBeginInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT,
      next: core::ptr::null(),
      buffer: Default::default(),
      offset: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkConformanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConformanceVersion.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkConformanceVersion {
  pub major: u8,
  pub minor: u8,
  pub subminor: u8,
  pub patch: u8,
}
impl Default for VkConformanceVersion {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      major: Default::default(),
      minor: Default::default(),
      subminor: Default::default(),
      patch: Default::default(),
    }
  }
}

/// Khronos: [VkCooperativeMatrixPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCooperativeMatrixPropertiesNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCooperativeMatrixPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub m_size: u32,
  pub n_size: u32,
  pub k_size: u32,
  pub a_type: VkComponentTypeNV,
  pub b_type: VkComponentTypeNV,
  pub c_type: VkComponentTypeNV,
  pub d_type: VkComponentTypeNV,
  pub scope: VkScopeNV,
}
impl Default for VkCooperativeMatrixPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      m_size: Default::default(),
      n_size: Default::default(),
      k_size: Default::default(),
      a_type: Default::default(),
      b_type: Default::default(),
      c_type: Default::default(),
      d_type: Default::default(),
      scope: Default::default(),
    }
  }
}

/// Khronos: [VkCopyAccelerationStructureInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyAccelerationStructureInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src: VkAccelerationStructureKHR,
  pub dst: VkAccelerationStructureKHR,
  pub mode: VkCopyAccelerationStructureModeKHR,
}
impl Default for VkCopyAccelerationStructureInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR,
      next: core::ptr::null(),
      src: Default::default(),
      dst: Default::default(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkCopyBufferInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyBufferInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyBufferInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_buffer: VkBuffer,
  pub dst_buffer: VkBuffer,
  pub region_count: u32,
  /// * Len: `region_count`
  pub regions: *const VkBufferCopy2,
}
impl Default for VkCopyBufferInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2,
      next: core::ptr::null(),
      src_buffer: Default::default(),
      dst_buffer: Default::default(),
      region_count: Default::default(),
      regions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCopyBufferToImageInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyBufferToImageInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyBufferToImageInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_buffer: VkBuffer,
  pub dst_image: VkImage,
  pub dst_image_layout: VkImageLayout,
  pub region_count: u32,
  /// * Len: `region_count`
  pub regions: *const VkBufferImageCopy2,
}
impl Default for VkCopyBufferToImageInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2,
      next: core::ptr::null(),
      src_buffer: Default::default(),
      dst_image: Default::default(),
      dst_image_layout: Default::default(),
      region_count: Default::default(),
      regions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCopyCommandTransformInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyCommandTransformInfoQCOM.html) (structure)
///
/// * Struct Extends: [`VkBufferImageCopy2`]
/// * Struct Extends: [`VkImageBlit2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyCommandTransformInfoQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *const c_void,
  /// * No Auto-Validity
  pub transform: VkSurfaceTransformFlagBitsKHR,
}
impl Default for VkCopyCommandTransformInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM,
      next: core::ptr::null(),
      transform: Default::default(),
    }
  }
}

/// Khronos: [VkCopyDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyDescriptorSet.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyDescriptorSet {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Source descriptor set
  pub src_set: VkDescriptorSet,
  /// Binding within the source descriptor set to copy from
  pub src_binding: u32,
  /// Array element within the source binding to copy from
  pub src_array_element: u32,
  /// Destination descriptor set
  pub dst_set: VkDescriptorSet,
  /// Binding within the destination descriptor set to copy to
  pub dst_binding: u32,
  /// Array element within the destination binding to copy to
  pub dst_array_element: u32,
  /// Number of descriptors to write (determines the size of the array pointed
  /// by pDescriptors)
  pub descriptor_count: u32,
}
impl Default for VkCopyDescriptorSet {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET,
      next: core::ptr::null(),
      src_set: Default::default(),
      src_binding: Default::default(),
      src_array_element: Default::default(),
      dst_set: Default::default(),
      dst_binding: Default::default(),
      dst_array_element: Default::default(),
      descriptor_count: Default::default(),
    }
  }
}

/// Khronos: [VkCopyImageInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyImageInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyImageInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_image: VkImage,
  pub src_image_layout: VkImageLayout,
  pub dst_image: VkImage,
  pub dst_image_layout: VkImageLayout,
  pub region_count: u32,
  /// * Len: `region_count`
  pub regions: *const VkImageCopy2,
}
impl Default for VkCopyImageInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2,
      next: core::ptr::null(),
      src_image: Default::default(),
      src_image_layout: Default::default(),
      dst_image: Default::default(),
      dst_image_layout: Default::default(),
      region_count: Default::default(),
      regions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCopyImageToBufferInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyImageToBufferInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyImageToBufferInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_image: VkImage,
  pub src_image_layout: VkImageLayout,
  pub dst_buffer: VkBuffer,
  pub region_count: u32,
  /// * Len: `region_count`
  pub regions: *const VkBufferImageCopy2,
}
impl Default for VkCopyImageToBufferInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2,
      next: core::ptr::null(),
      src_image: Default::default(),
      src_image_layout: Default::default(),
      dst_buffer: Default::default(),
      region_count: Default::default(),
      regions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCopyMemoryIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryIndirectCommandNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyMemoryIndirectCommandNV {
  pub src_address: VkDeviceAddress,
  pub dst_address: VkDeviceAddress,
  /// Specified in bytes
  pub size: VkDeviceSize,
}
impl Default for VkCopyMemoryIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      src_address: Default::default(),
      dst_address: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkCopyMemoryToImageIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToImageIndirectCommandNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyMemoryToImageIndirectCommandNV {
  pub src_address: VkDeviceAddress,
  /// Specified in texels
  pub buffer_row_length: u32,
  pub buffer_image_height: u32,
  pub image_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub image_offset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub image_extent: VkExtent3D,
}
impl Default for VkCopyMemoryToImageIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      src_address: Default::default(),
      buffer_row_length: Default::default(),
      buffer_image_height: Default::default(),
      image_subresource: Default::default(),
      image_offset: Default::default(),
      image_extent: Default::default(),
    }
  }
}

/// Khronos: [VkCopyMicromapInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMicromapInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCopyMicromapInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_MICROMAP_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src: VkMicromapEXT,
  pub dst: VkMicromapEXT,
  pub mode: VkCopyMicromapModeEXT,
}
impl Default for VkCopyMicromapInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_COPY_MICROMAP_INFO_EXT,
      next: core::ptr::null(),
      src: Default::default(),
      dst: Default::default(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkCuFunctionCreateInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuFunctionCreateInfoNVX.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCuFunctionCreateInfoNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub module: VkCuModuleNVX,
  /// * Len: `null_terminated`
  pub name: *const u8,
}
impl Default for VkCuFunctionCreateInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX,
      next: core::ptr::null(),
      module: Default::default(),
      name: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCuLaunchInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuLaunchInfoNVX.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCuLaunchInfoNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub function: VkCuFunctionNVX,
  pub grid_dim_x: u32,
  pub grid_dim_y: u32,
  pub grid_dim_z: u32,
  pub block_dim_x: u32,
  pub block_dim_y: u32,
  pub block_dim_z: u32,
  pub shared_mem_bytes: u32,
  /// * Optional: true
  pub param_count: c_size_t,
  /// * Len: `param_count`
  pub params: *const *const c_void,
  /// * Optional: true
  pub extra_count: c_size_t,
  /// * Len: `extra_count`
  pub extras: *const *const c_void,
}
impl Default for VkCuLaunchInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX,
      next: core::ptr::null(),
      function: Default::default(),
      grid_dim_x: Default::default(),
      grid_dim_y: Default::default(),
      grid_dim_z: Default::default(),
      block_dim_x: Default::default(),
      block_dim_y: Default::default(),
      block_dim_z: Default::default(),
      shared_mem_bytes: Default::default(),
      param_count: Default::default(),
      params: core::ptr::null(),
      extra_count: Default::default(),
      extras: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCuModuleCreateInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuModuleCreateInfoNVX.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkCuModuleCreateInfoNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub data_size: c_size_t,
  /// * Len: `data_size`
  pub data: *const c_void,
}
impl Default for VkCuModuleCreateInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX,
      next: core::ptr::null(),
      data_size: Default::default(),
      data: core::ptr::null(),
    }
  }
}

/// Khronos: [VkD3D12FenceSubmitInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkD3D12FenceSubmitInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkSubmitInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkD3D12FenceSubmitInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub wait_semaphore_values_count: u32,
  /// * Optional: true
  /// * Len: `wait_semaphore_values_count`
  pub wait_semaphore_values: *const u64,
  /// * Optional: true
  pub signal_semaphore_values_count: u32,
  /// * Optional: true
  /// * Len: `signal_semaphore_values_count`
  pub signal_semaphore_values: *const u64,
}
impl Default for VkD3D12FenceSubmitInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR,
      next: core::ptr::null(),
      wait_semaphore_values_count: Default::default(),
      wait_semaphore_values: core::ptr::null(),
      signal_semaphore_values_count: Default::default(),
      signal_semaphore_values: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugMarkerMarkerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDebugMarkerMarkerInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Name of the debug marker
  /// * Len: `null_terminated`
  pub marker_name: *const u8,
  /// Optional color for debug marker
  pub color: [c_float; 4],
}
impl Default for VkDebugMarkerMarkerInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT,
      next: core::ptr::null(),
      marker_name: core::ptr::null(),
      color: [Default::default(); 4],
    }
  }
}

/// Khronos: [VkDebugMarkerObjectNameInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDebugMarkerObjectNameInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// The type of the object
  pub object_type: VkDebugReportObjectTypeEXT,
  /// The handle of the object, cast to uint64_t
  /// * Object Type: objectType
  pub object: u64,
  /// Name to apply to the object
  /// * Len: `null_terminated`
  pub object_name: *const u8,
}
impl Default for VkDebugMarkerObjectNameInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
      next: core::ptr::null(),
      object_type: Default::default(),
      object: Default::default(),
      object_name: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugMarkerObjectTagInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDebugMarkerObjectTagInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// The type of the object
  pub object_type: VkDebugReportObjectTypeEXT,
  /// The handle of the object, cast to uint64_t
  /// * Object Type: objectType
  pub object: u64,
  /// The name of the tag to set on the object
  pub tag_name: u64,
  /// The length in bytes of the tag data
  pub tag_size: c_size_t,
  /// Tag data to attach to the object
  /// * Len: `tag_size`
  pub tag: *const c_void,
}
impl Default for VkDebugMarkerObjectTagInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
      next: core::ptr::null(),
      object_type: Default::default(),
      object: Default::default(),
      tag_name: Default::default(),
      tag_size: Default::default(),
      tag: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugReportCallbackCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Indicates which events call this callback
  /// * Optional: true
  pub flags: VkDebugReportFlagsEXT,
  /// Function pointer of a callback function
  pub pfn_callback: PFN_vkDebugReportCallbackEXT,
  /// User data provided to callback function
  /// * Optional: true
  pub user_data: *mut c_void,
}
impl Default for VkDebugReportCallbackCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      pfn_callback: Default::default(),
      user_data: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsLabelEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Len: `null_terminated`
  pub label_name: *const u8,
  pub color: [c_float; 4],
}
impl Default for VkDebugUtilsLabelEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT,
      next: core::ptr::null(),
      label_name: core::ptr::null(),
      color: [Default::default(); 4],
    }
  }
}

/// Khronos: [VkDebugUtilsMessengerCallbackDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
  /// * Optional: true
  /// * Len: `null_terminated`
  pub message_id_name: *const u8,
  pub message_id_number: i32,
  /// * Len: `null_terminated`
  pub message: *const u8,
  /// * Optional: true
  pub queue_label_count: u32,
  /// * Len: `queue_label_count`
  pub queue_labels: *const VkDebugUtilsLabelEXT,
  /// * Optional: true
  pub cmd_buf_label_count: u32,
  /// * Len: `cmd_buf_label_count`
  pub cmd_buf_labels: *const VkDebugUtilsLabelEXT,
  /// * Optional: true
  pub object_count: u32,
  /// * Len: `object_count`
  pub objects: *const VkDebugUtilsObjectNameInfoEXT,
}
impl Default for VkDebugUtilsMessengerCallbackDataEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      message_id_name: core::ptr::null(),
      message_id_number: Default::default(),
      message: core::ptr::null(),
      queue_label_count: Default::default(),
      queue_labels: core::ptr::null(),
      cmd_buf_label_count: Default::default(),
      cmd_buf_labels: core::ptr::null(),
      object_count: Default::default(),
      objects: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugUtilsMessengerCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
  pub message_severity: VkDebugUtilsMessageSeverityFlagsEXT,
  pub message_type: VkDebugUtilsMessageTypeFlagsEXT,
  pub pfn_user_callback: PFN_vkDebugUtilsMessengerCallbackEXT,
  /// * Optional: true
  pub user_data: *mut c_void,
}
impl Default for VkDebugUtilsMessengerCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      message_severity: Default::default(),
      message_type: Default::default(),
      pfn_user_callback: Default::default(),
      user_data: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDebugUtilsObjectNameInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDebugUtilsObjectNameInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub object_type: VkObjectType,
  /// * Object Type: objectType
  pub object_handle: u64,
  /// * Optional: true
  /// * Len: `null_terminated`
  pub object_name: *const u8,
}
impl Default for VkDebugUtilsObjectNameInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
      next: core::ptr::null(),
      object_type: Default::default(),
      object_handle: Default::default(),
      object_name: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugUtilsObjectTagInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDebugUtilsObjectTagInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub object_type: VkObjectType,
  /// * Object Type: objectType
  pub object_handle: u64,
  pub tag_name: u64,
  pub tag_size: c_size_t,
  /// * Len: `tag_size`
  pub tag: *const c_void,
}
impl Default for VkDebugUtilsObjectTagInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
      next: core::ptr::null(),
      object_type: Default::default(),
      object_handle: Default::default(),
      tag_name: Default::default(),
      tag_size: Default::default(),
      tag: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDecompressMemoryRegionNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDecompressMemoryRegionNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDecompressMemoryRegionNV {
  pub src_address: VkDeviceAddress,
  pub dst_address: VkDeviceAddress,
  /// Specified in bytes
  pub compressed_size: VkDeviceSize,
  /// Specified in bytes
  pub decompressed_size: VkDeviceSize,
  pub decompression_method: VkMemoryDecompressionMethodFlagsNV,
}
impl Default for VkDecompressMemoryRegionNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      src_address: Default::default(),
      dst_address: Default::default(),
      compressed_size: Default::default(),
      decompressed_size: Default::default(),
      decompression_method: Default::default(),
    }
  }
}

/// Khronos: [VkDedicatedAllocationBufferCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationBufferCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Whether this buffer uses a dedicated allocation
  pub dedicated_allocation: VkBool32,
}
impl Default for VkDedicatedAllocationBufferCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
      next: core::ptr::null(),
      dedicated_allocation: Default::default(),
    }
  }
}

/// Khronos: [VkDedicatedAllocationImageCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Whether this image uses a dedicated allocation
  pub dedicated_allocation: VkBool32,
}
impl Default for VkDedicatedAllocationImageCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
      next: core::ptr::null(),
      dedicated_allocation: Default::default(),
    }
  }
}

/// Khronos: [VkDedicatedAllocationMemoryAllocateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Image that this allocation will be bound to
  /// * Optional: true
  pub image: VkImage,
  /// Buffer that this allocation will be bound to
  /// * Optional: true
  pub buffer: VkBuffer,
}
impl Default for VkDedicatedAllocationMemoryAllocateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
      next: core::ptr::null(),
      image: Default::default(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkDependencyInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDependencyInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEPENDENCY_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub dependency_flags: VkDependencyFlags,
  /// * Optional: true
  pub memory_barrier_count: u32,
  /// * Len: `memory_barrier_count`
  pub memory_barriers: *const VkMemoryBarrier2,
  /// * Optional: true
  pub buffer_memory_barrier_count: u32,
  /// * Len: `buffer_memory_barrier_count`
  pub buffer_memory_barriers: *const VkBufferMemoryBarrier2,
  /// * Optional: true
  pub image_memory_barrier_count: u32,
  /// * Len: `image_memory_barrier_count`
  pub image_memory_barriers: *const VkImageMemoryBarrier2,
}
impl Default for VkDependencyInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEPENDENCY_INFO,
      next: core::ptr::null(),
      dependency_flags: Default::default(),
      memory_barrier_count: Default::default(),
      memory_barriers: core::ptr::null(),
      buffer_memory_barrier_count: Default::default(),
      buffer_memory_barriers: core::ptr::null(),
      image_memory_barrier_count: Default::default(),
      image_memory_barriers: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorAddressInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorAddressInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorAddressInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub address: VkDeviceAddress,
  pub range: VkDeviceSize,
  pub format: VkFormat,
}
impl Default for VkDescriptorAddressInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT,
      next: core::ptr::null_mut(),
      address: Default::default(),
      range: Default::default(),
      format: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorBufferBindingInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferBindingInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorBufferBindingInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub address: VkDeviceAddress,
  pub usage: VkBufferUsageFlags,
}
impl Default for VkDescriptorBufferBindingInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT,
      next: core::ptr::null_mut(),
      address: Default::default(),
      usage: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorBufferBindingPushDescriptorBufferHandleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferBindingPushDescriptorBufferHandleEXT.html) (structure)
///
/// * Struct Extends: [`VkDescriptorBufferBindingInfoEXT`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub buffer: VkBuffer,
}
impl Default for VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT,
      next: core::ptr::null_mut(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorBufferInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorBufferInfo {
  /// Buffer used for this descriptor slot.
  /// * Optional: true
  pub buffer: VkBuffer,
  /// Base offset from buffer start in bytes to update in the descriptor set.
  pub offset: VkDeviceSize,
  /// Size in bytes of the buffer resource for this descriptor update.
  pub range: VkDeviceSize,
}
impl Default for VkDescriptorBufferInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      buffer: Default::default(),
      offset: Default::default(),
      range: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorImageInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorImageInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorImageInfo {
  /// Sampler to write to the descriptor in case it is a SAMPLER or
  /// COMBINED_IMAGE_SAMPLER descriptor. Ignored otherwise.
  /// * No Auto-Validity
  pub sampler: VkSampler,
  /// Image view to write to the descriptor in case it is a SAMPLED_IMAGE,
  /// STORAGE_IMAGE, COMBINED_IMAGE_SAMPLER, or INPUT_ATTACHMENT descriptor.
  /// Ignored otherwise.
  /// * No Auto-Validity
  pub image_view: VkImageView,
  /// Layout the image is expected to be in when accessed using this descriptor
  /// (only used if imageView is not VK_NULL_HANDLE).
  /// * No Auto-Validity
  pub image_layout: VkImageLayout,
}
impl Default for VkDescriptorImageInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sampler: Default::default(),
      image_view: Default::default(),
      image_layout: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDescriptorPoolCreateFlags,
  pub max_sets: u32,
  /// * Optional: true
  pub pool_size_count: u32,
  /// * Len: `pool_size_count`
  pub pool_sizes: *const VkDescriptorPoolSize,
}
impl Default for VkDescriptorPoolCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      max_sets: Default::default(),
      pool_size_count: Default::default(),
      pool_sizes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorPoolInlineUniformBlockCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolInlineUniformBlockCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkDescriptorPoolCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub max_inline_uniform_block_bindings: u32,
}
impl Default for VkDescriptorPoolInlineUniformBlockCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO,
      next: core::ptr::null(),
      max_inline_uniform_block_bindings: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorPoolSize](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolSize.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorPoolSize {
  pub ty: VkDescriptorType,
  pub descriptor_count: u32,
}
impl Default for VkDescriptorPoolSize {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { ty: Default::default(), descriptor_count: Default::default() }
  }
}

/// Khronos: [VkDescriptorSetAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetAllocateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub descriptor_pool: VkDescriptorPool,
  pub descriptor_set_count: u32,
  /// * Len: `descriptor_set_count`
  pub set_layouts: *const VkDescriptorSetLayout,
}
impl Default for VkDescriptorSetAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
      next: core::ptr::null(),
      descriptor_pool: Default::default(),
      descriptor_set_count: Default::default(),
      set_layouts: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetBindingReferenceVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetBindingReferenceVALVE.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorSetBindingReferenceVALVE {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub descriptor_set_layout: VkDescriptorSetLayout,
  pub binding: u32,
}
impl Default for VkDescriptorSetBindingReferenceVALVE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE,
      next: core::ptr::null(),
      descriptor_set_layout: Default::default(),
      binding: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutBinding](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBinding.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
  /// Binding number for this entry
  pub binding: u32,
  /// Type of the descriptors in this binding
  pub descriptor_type: VkDescriptorType,
  /// Number of descriptors in this binding
  /// * Optional: true
  pub descriptor_count: u32,
  /// Shader stages this binding is visible to
  /// * No Auto-Validity
  pub stage_flags: VkShaderStageFlags,
  /// Immutable samplers (used if descriptor type is SAMPLER or
  /// COMBINED_IMAGE_SAMPLER, is either NULL or contains count number of
  /// elements)
  /// * Optional: true
  /// * Len: `descriptor_count`
  /// * No Auto-Validity
  pub immutable_samplers: *const VkSampler,
}
impl Default for VkDescriptorSetLayoutBinding {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      binding: Default::default(),
      descriptor_type: Default::default(),
      descriptor_count: Default::default(),
      stage_flags: Default::default(),
      immutable_samplers: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutBindingFlagsCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkDescriptorSetLayoutCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub binding_count: u32,
  /// * Optional: false,true
  /// * Len: `binding_count`
  pub binding_flags: *const VkDescriptorBindingFlags,
}
impl Default for VkDescriptorSetLayoutBindingFlagsCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
      next: core::ptr::null(),
      binding_count: Default::default(),
      binding_flags: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDescriptorSetLayoutCreateFlags,
  /// Number of bindings in the descriptor set layout
  /// * Optional: true
  pub binding_count: u32,
  /// Array of descriptor set layout bindings
  /// * Len: `binding_count`
  pub bindings: *const VkDescriptorSetLayoutBinding,
}
impl Default for VkDescriptorSetLayoutCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      binding_count: Default::default(),
      bindings: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutHostMappingInfoVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutHostMappingInfoVALVE.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorSetLayoutHostMappingInfoVALVE {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub descriptor_offset: c_size_t,
  pub descriptor_size: u32,
}
impl Default for VkDescriptorSetLayoutHostMappingInfoVALVE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE,
      next: core::ptr::null_mut(),
      descriptor_offset: Default::default(),
      descriptor_size: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutSupport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutSupport.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorSetLayoutSupport {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub supported: VkBool32,
}
impl Default for VkDescriptorSetLayoutSupport {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT,
      next: core::ptr::null_mut(),
      supported: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorSetVariableDescriptorCountAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfo.html) (structure)
///
/// * Struct Extends: [`VkDescriptorSetAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub descriptor_set_count: u32,
  /// * Len: `descriptor_set_count`
  pub descriptor_counts: *const u32,
}
impl Default for VkDescriptorSetVariableDescriptorCountAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
      next: core::ptr::null(),
      descriptor_set_count: Default::default(),
      descriptor_counts: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetVariableDescriptorCountLayoutSupport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupport.html) (structure)
///
/// * Struct Extends: [`VkDescriptorSetLayoutSupport`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub max_variable_descriptor_count: u32,
}
impl Default for VkDescriptorSetVariableDescriptorCountLayoutSupport {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT,
      next: core::ptr::null_mut(),
      max_variable_descriptor_count: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorUpdateTemplateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorUpdateTemplateCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDescriptorUpdateTemplateCreateFlags,
  /// Number of descriptor update entries to use for the update template
  pub descriptor_update_entry_count: u32,
  /// Descriptor update entries for the template
  /// * Len: `descriptor_update_entry_count`
  pub descriptor_update_entries: *const VkDescriptorUpdateTemplateEntry,
  pub template_type: VkDescriptorUpdateTemplateType,
  /// * No Auto-Validity
  pub descriptor_set_layout: VkDescriptorSetLayout,
  /// * No Auto-Validity
  pub pipeline_bind_point: VkPipelineBindPoint,
  /// If used for push descriptors, this is the only allowed layout
  /// * No Auto-Validity
  pub pipeline_layout: VkPipelineLayout,
  /// * No Auto-Validity
  pub set: u32,
}
impl Default for VkDescriptorUpdateTemplateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      descriptor_update_entry_count: Default::default(),
      descriptor_update_entries: core::ptr::null(),
      template_type: Default::default(),
      descriptor_set_layout: Default::default(),
      pipeline_bind_point: Default::default(),
      pipeline_layout: Default::default(),
      set: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorUpdateTemplateEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateEntry.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDescriptorUpdateTemplateEntry {
  /// Binding within the destination descriptor set to write
  pub dst_binding: u32,
  /// Array element within the destination binding to write
  pub dst_array_element: u32,
  /// Number of descriptors to write
  pub descriptor_count: u32,
  /// Descriptor type to write
  pub descriptor_type: VkDescriptorType,
  /// Offset into pData where the descriptors to update are stored
  pub offset: c_size_t,
  /// Stride between two descriptors in pData when writing more than one
  /// descriptor
  pub stride: c_size_t,
}
impl Default for VkDescriptorUpdateTemplateEntry {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      dst_binding: Default::default(),
      dst_array_element: Default::default(),
      descriptor_count: Default::default(),
      descriptor_type: Default::default(),
      offset: Default::default(),
      stride: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceAddressBindingCallbackDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingCallbackDataEXT.html) (structure)
///
/// * Struct Extends: [`VkDebugUtilsMessengerCallbackDataEXT`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceAddressBindingCallbackDataEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  pub flags: VkDeviceAddressBindingFlagsEXT,
  pub base_address: VkDeviceAddress,
  pub size: VkDeviceSize,
  pub binding_type: VkDeviceAddressBindingTypeEXT,
}
impl Default for VkDeviceAddressBindingCallbackDataEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT,
      next: core::ptr::null_mut(),
      flags: Default::default(),
      base_address: Default::default(),
      size: Default::default(),
      binding_type: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceBufferMemoryRequirements.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceBufferMemoryRequirements {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub create_info: *const VkBufferCreateInfo,
}
impl Default for VkDeviceBufferMemoryRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS,
      next: core::ptr::null(),
      create_info: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDeviceCreateFlags,
  pub queue_create_info_count: u32,
  /// * Len: `queue_create_info_count`
  pub queue_create_infos: *const VkDeviceQueueCreateInfo,
  /// * Optional: true
  #[deprecated = "ignored"]
  pub enabled_layer_count: u32,
  /// Ordered list of layer names to be enabled
  /// * Len: `enabled_layer_count,null_terminated`
  #[deprecated = "ignored"]
  pub enabled_layer_names: *const *const u8,
  /// * Optional: true
  pub enabled_extension_count: u32,
  /// * Len: `enabled_extension_count,null_terminated`
  pub enabled_extension_names: *const *const u8,
  /// * Optional: true
  pub enabled_features: *const VkPhysicalDeviceFeatures,
}
impl Default for VkDeviceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    #[allow(deprecated)]
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      queue_create_info_count: Default::default(),
      queue_create_infos: core::ptr::null(),
      enabled_layer_count: Default::default(),
      enabled_layer_names: core::ptr::null(),
      enabled_extension_count: Default::default(),
      enabled_extension_names: core::ptr::null(),
      enabled_features: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceDeviceMemoryReportCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceDeviceMemoryReportCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub flags: VkDeviceMemoryReportFlagsEXT,
  pub pfn_user_callback: PFN_vkDeviceMemoryReportCallbackEXT,
  pub user_data: *mut c_void,
}
impl Default for VkDeviceDeviceMemoryReportCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      pfn_user_callback: Default::default(),
      user_data: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDeviceDiagnosticsConfigCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceDiagnosticsConfigCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDeviceDiagnosticsConfigFlagsNV,
}
impl Default for VkDeviceDiagnosticsConfigCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV,
      next: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceEventInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceEventInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub device_event: VkDeviceEventTypeEXT,
}
impl Default for VkDeviceEventInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT,
      next: core::ptr::null(),
      device_event: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceFaultAddressInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultAddressInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceFaultAddressInfoEXT {
  pub address_type: VkDeviceFaultAddressTypeEXT,
  pub reported_address: VkDeviceAddress,
  pub address_precision: VkDeviceSize,
}
impl Default for VkDeviceFaultAddressInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      address_type: Default::default(),
      reported_address: Default::default(),
      address_precision: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceFaultCountsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultCountsEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceFaultCountsEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_FAULT_COUNTS_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  pub address_info_count: u32,
  /// * Optional: true
  pub vendor_info_count: u32,
  /// Specified in bytes
  /// * Optional: true
  pub vendor_binary_size: VkDeviceSize,
}
impl Default for VkDeviceFaultCountsEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_FAULT_COUNTS_EXT,
      next: core::ptr::null_mut(),
      address_info_count: Default::default(),
      vendor_info_count: Default::default(),
      vendor_binary_size: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceFaultInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceFaultInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_FAULT_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// Free-form description of the fault
  /// * No Auto-Validity
  pub description: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  /// * Optional: true
  pub address_infos: *mut VkDeviceFaultAddressInfoEXT,
  /// * Optional: true
  pub vendor_infos: *mut VkDeviceFaultVendorInfoEXT,
  /// * Optional: true
  pub vendor_binary_data: *mut c_void,
}
impl Default for VkDeviceFaultInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_FAULT_INFO_EXT,
      next: core::ptr::null_mut(),
      description: Default::default(),
      address_infos: core::ptr::null_mut(),
      vendor_infos: core::ptr::null_mut(),
      vendor_binary_data: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDeviceFaultVendorBinaryHeaderVersionOneEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultVendorBinaryHeaderVersionOneEXT.html) (structure)
///
/// The fields in this structure are non-normative since structure packing is
/// implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceFaultVendorBinaryHeaderVersionOneEXT {
  pub header_size: u32,
  pub header_version: VkDeviceFaultVendorBinaryHeaderVersionEXT,
  pub vendor_id: u32,
  pub device_id: u32,
  pub driver_version: u32,
  pub pipeline_cache_uuid: [u8; VK_UUID_SIZE],
  pub application_name_offset: u32,
  pub application_version: u32,
  pub engine_name_offset: u32,
}
impl Default for VkDeviceFaultVendorBinaryHeaderVersionOneEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      header_size: Default::default(),
      header_version: Default::default(),
      vendor_id: Default::default(),
      device_id: Default::default(),
      driver_version: Default::default(),
      pipeline_cache_uuid: [Default::default(); VK_UUID_SIZE],
      application_name_offset: Default::default(),
      application_version: Default::default(),
      engine_name_offset: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceFaultVendorInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultVendorInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceFaultVendorInfoEXT {
  /// Free-form description of the fault
  /// * No Auto-Validity
  pub description: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  pub vendor_fault_code: u64,
  pub vendor_fault_data: u64,
}
impl Default for VkDeviceFaultVendorInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      description: Default::default(),
      vendor_fault_code: Default::default(),
      vendor_fault_data: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupBindSparseInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupBindSparseInfo.html) (structure)
///
/// * Struct Extends: [`VkBindSparseInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceGroupBindSparseInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub resource_device_index: u32,
  pub memory_device_index: u32,
}
impl Default for VkDeviceGroupBindSparseInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO,
      next: core::ptr::null(),
      resource_device_index: Default::default(),
      memory_device_index: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupCommandBufferBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html) (structure)
///
/// * Struct Extends: [`VkCommandBufferBeginInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceGroupCommandBufferBeginInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub device_mask: u32,
}
impl Default for VkDeviceGroupCommandBufferBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
      next: core::ptr::null(),
      device_mask: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupDeviceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceGroupDeviceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub physical_device_count: u32,
  /// * Len: `physical_device_count`
  pub physical_devices: *const VkPhysicalDevice,
}
impl Default for VkDeviceGroupDeviceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO,
      next: core::ptr::null(),
      physical_device_count: Default::default(),
      physical_devices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceGroupPresentCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub present_mask: [u32; VK_MAX_DEVICE_GROUP_SIZE],
  pub modes: VkDeviceGroupPresentModeFlagsKHR,
}
impl Default for VkDeviceGroupPresentCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
      next: core::ptr::null_mut(),
      present_mask: [Default::default(); VK_MAX_DEVICE_GROUP_SIZE],
      modes: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupPresentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceGroupPresentInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub swapchain_count: u32,
  /// * Len: `swapchain_count`
  pub device_masks: *const u32,
  pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
}
impl Default for VkDeviceGroupPresentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR,
      next: core::ptr::null(),
      swapchain_count: Default::default(),
      device_masks: core::ptr::null(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupRenderPassBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html) (structure)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceGroupRenderPassBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub device_mask: u32,
  /// * Optional: true
  pub device_render_area_count: u32,
  /// * Len: `device_render_area_count`
  pub device_render_areas: *const VkRect2D,
}
impl Default for VkDeviceGroupRenderPassBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO,
      next: core::ptr::null(),
      device_mask: Default::default(),
      device_render_area_count: Default::default(),
      device_render_areas: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceGroupSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSubmitInfo.html) (structure)
///
/// * Struct Extends: [`VkSubmitInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceGroupSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub wait_semaphore_count: u32,
  /// * Len: `wait_semaphore_count`
  pub wait_semaphore_device_indices: *const u32,
  /// * Optional: true
  pub command_buffer_count: u32,
  /// * Len: `command_buffer_count`
  pub command_buffer_device_masks: *const u32,
  /// * Optional: true
  pub signal_semaphore_count: u32,
  /// * Len: `signal_semaphore_count`
  pub signal_semaphore_device_indices: *const u32,
}
impl Default for VkDeviceGroupSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO,
      next: core::ptr::null(),
      wait_semaphore_count: Default::default(),
      wait_semaphore_device_indices: core::ptr::null(),
      command_buffer_count: Default::default(),
      command_buffer_device_masks: core::ptr::null(),
      signal_semaphore_count: Default::default(),
      signal_semaphore_device_indices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceGroupSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub modes: VkDeviceGroupPresentModeFlagsKHR,
}
impl Default for VkDeviceGroupSwapchainCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR,
      next: core::ptr::null(),
      modes: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceImageMemoryRequirements.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceImageMemoryRequirements {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub create_info: *const VkImageCreateInfo,
  /// * Optional: true
  pub plane_aspect: VkImageAspectFlagBits,
}
impl Default for VkDeviceImageMemoryRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS,
      next: core::ptr::null(),
      create_info: core::ptr::null(),
      plane_aspect: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceMemoryOpaqueCaptureAddressInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOpaqueCaptureAddressInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory: VkDeviceMemory,
}
impl Default for VkDeviceMemoryOpaqueCaptureAddressInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO,
      next: core::ptr::null(),
      memory: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceMemoryOverallocationCreateInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html) (structure)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceMemoryOverallocationCreateInfoAMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub overallocation_behavior: VkMemoryOverallocationBehaviorAMD,
}
impl Default for VkDeviceMemoryOverallocationCreateInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD,
      next: core::ptr::null(),
      overallocation_behavior: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceMemoryReportCallbackDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportCallbackDataEXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceMemoryReportCallbackDataEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub flags: VkDeviceMemoryReportFlagsEXT,
  pub ty: VkDeviceMemoryReportEventTypeEXT,
  pub memory_object_id: u64,
  pub size: VkDeviceSize,
  pub object_type: VkObjectType,
  /// * Object Type: objectType
  pub object_handle: u64,
  pub heap_index: u32,
}
impl Default for VkDeviceMemoryReportCallbackDataEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT,
      next: core::ptr::null_mut(),
      flags: Default::default(),
      ty: Default::default(),
      memory_object_id: Default::default(),
      size: Default::default(),
      object_type: Default::default(),
      object_handle: Default::default(),
      heap_index: Default::default(),
    }
  }
}

/// Khronos: [VkDevicePrivateDataCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDevicePrivateDataCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDevicePrivateDataCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub private_data_slot_request_count: u32,
}
impl Default for VkDevicePrivateDataCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO,
      next: core::ptr::null(),
      private_data_slot_request_count: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceQueueCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDeviceQueueCreateFlags,
  pub queue_family_index: u32,
  pub queue_count: u32,
  /// * Len: `queue_count`
  pub queue_priorities: *const c_float,
}
impl Default for VkDeviceQueueCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      queue_family_index: Default::default(),
      queue_count: Default::default(),
      queue_priorities: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceQueueGlobalPriorityCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkDeviceQueueCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub global_priority: VkQueueGlobalPriorityKHR,
}
impl Default for VkDeviceQueueGlobalPriorityCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR,
      next: core::ptr::null(),
      global_priority: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceQueueInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDeviceQueueInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDeviceQueueCreateFlags,
  pub queue_family_index: u32,
  pub queue_index: u32,
}
impl Default for VkDeviceQueueInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2,
      next: core::ptr::null(),
      flags: Default::default(),
      queue_family_index: Default::default(),
      queue_index: Default::default(),
    }
  }
}

/// Khronos: [VkDirectDriverLoadingInfoLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingInfoLUNARG.html) (structure)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDirectDriverLoadingInfoLUNARG {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_INFO_LUNARG`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub flags: VkDirectDriverLoadingFlagsLUNARG,
  /// * No Auto-Validity
  pub pfn_get_instance_proc_addr: PFN_vkGetInstanceProcAddrLUNARG,
}
impl Default for VkDirectDriverLoadingInfoLUNARG {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_INFO_LUNARG,
      next: core::ptr::null_mut(),
      flags: Default::default(),
      pfn_get_instance_proc_addr: Default::default(),
    }
  }
}

/// Khronos: [VkDirectDriverLoadingListLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingListLUNARG.html) (structure)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDirectDriverLoadingListLUNARG {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_LIST_LUNARG`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub mode: VkDirectDriverLoadingModeLUNARG,
  pub driver_count: u32,
  /// * Len: `driver_count`
  pub drivers: *const VkDirectDriverLoadingInfoLUNARG,
}
impl Default for VkDirectDriverLoadingListLUNARG {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_LIST_LUNARG,
      next: core::ptr::null_mut(),
      mode: Default::default(),
      driver_count: Default::default(),
      drivers: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDirectFBSurfaceCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDirectFBSurfaceCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDirectFBSurfaceCreateFlagsEXT,
  /// * No Auto-Validity
  pub dfb: *mut IDirectFB,
  /// * No Auto-Validity
  pub surface: *mut IDirectFBSurface,
}
impl Default for VkDirectFBSurfaceCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      dfb: core::ptr::null_mut(),
      surface: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDispatchIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDispatchIndirectCommand.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDispatchIndirectCommand {
  /// * No Auto-Validity
  pub x: u32,
  /// * No Auto-Validity
  pub y: u32,
  /// * No Auto-Validity
  pub z: u32,
}
impl Default for VkDispatchIndirectCommand {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { x: Default::default(), y: Default::default(), z: Default::default() }
  }
}

/// Khronos: [VkDisplayEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayEventInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayEventInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub display_event: VkDisplayEventTypeEXT,
}
impl Default for VkDisplayEventInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT,
      next: core::ptr::null(),
      display_event: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayModeCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayModeCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDisplayModeCreateFlagsKHR,
  /// The parameters this mode uses.
  pub parameters: VkDisplayModeParametersKHR,
}
impl Default for VkDisplayModeCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      parameters: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayModeParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeParametersKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayModeParametersKHR {
  /// Visible scanout region.
  pub visible_region: VkExtent2D,
  /// Number of times per second the display is updated.
  /// * No Auto-Validity
  pub refresh_rate: u32,
}
impl Default for VkDisplayModeParametersKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { visible_region: Default::default(), refresh_rate: Default::default() }
  }
}

/// Khronos: [VkDisplayModeProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeProperties2KHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayModeProperties2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub display_mode_properties: VkDisplayModePropertiesKHR,
}
impl Default for VkDisplayModeProperties2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR,
      next: core::ptr::null_mut(),
      display_mode_properties: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayModePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModePropertiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayModePropertiesKHR {
  /// Handle of this display mode.
  pub display_mode: VkDisplayModeKHR,
  /// The parameters this mode uses.
  pub parameters: VkDisplayModeParametersKHR,
}
impl Default for VkDisplayModePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { display_mode: Default::default(), parameters: Default::default() }
  }
}

/// Khronos: [VkDisplayNativeHdrSurfaceCapabilitiesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html) (structure)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub local_dimming_support: VkBool32,
}
impl Default for VkDisplayNativeHdrSurfaceCapabilitiesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD,
      next: core::ptr::null_mut(),
      local_dimming_support: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlaneCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayPlaneCapabilities2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub capabilities: VkDisplayPlaneCapabilitiesKHR,
}
impl Default for VkDisplayPlaneCapabilities2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR,
      next: core::ptr::null_mut(),
      capabilities: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlaneCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayPlaneCapabilitiesKHR {
  /// Types of alpha blending supported, if any.
  /// * Optional: true
  pub supported_alpha: VkDisplayPlaneAlphaFlagsKHR,
  /// Does the plane have any position and extent restrictions?
  pub min_src_position: VkOffset2D,
  pub max_src_position: VkOffset2D,
  pub min_src_extent: VkExtent2D,
  pub max_src_extent: VkExtent2D,
  pub min_dst_position: VkOffset2D,
  pub max_dst_position: VkOffset2D,
  pub min_dst_extent: VkExtent2D,
  pub max_dst_extent: VkExtent2D,
}
impl Default for VkDisplayPlaneCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      supported_alpha: Default::default(),
      min_src_position: Default::default(),
      max_src_position: Default::default(),
      min_src_extent: Default::default(),
      max_src_extent: Default::default(),
      min_dst_position: Default::default(),
      max_dst_position: Default::default(),
      min_dst_extent: Default::default(),
      max_dst_extent: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlaneInfo2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneInfo2KHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayPlaneInfo2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub mode: VkDisplayModeKHR,
  pub plane_index: u32,
}
impl Default for VkDisplayPlaneInfo2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR,
      next: core::ptr::null(),
      mode: Default::default(),
      plane_index: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlaneProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneProperties2KHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayPlaneProperties2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub display_plane_properties: VkDisplayPlanePropertiesKHR,
}
impl Default for VkDisplayPlaneProperties2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR,
      next: core::ptr::null_mut(),
      display_plane_properties: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlanePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlanePropertiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayPlanePropertiesKHR {
  /// Display the plane is currently associated with.  Will be VK_NULL_HANDLE if
  /// the plane is not in use.
  pub current_display: VkDisplayKHR,
  /// Current z-order of the plane.
  pub current_stack_index: u32,
}
impl Default for VkDisplayPlanePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { current_display: Default::default(), current_stack_index: Default::default() }
  }
}

/// Khronos: [VkDisplayPowerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayPowerInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub power_state: VkDisplayPowerStateEXT,
}
impl Default for VkDisplayPowerInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT,
      next: core::ptr::null(),
      power_state: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPresentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPresentInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayPresentInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Rectangle within the presentable image to read pixel data from when
  /// presenting to the display.
  pub src_rect: VkRect2D,
  /// Rectangle within the current display mode's visible region to display
  /// srcRectangle in.
  pub dst_rect: VkRect2D,
  /// For smart displays, use buffered mode.  If the display properties member
  /// "persistentMode" is VK_FALSE, this member must always be VK_FALSE.
  pub persistent: VkBool32,
}
impl Default for VkDisplayPresentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR,
      next: core::ptr::null(),
      src_rect: Default::default(),
      dst_rect: Default::default(),
      persistent: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayProperties2KHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayProperties2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub display_properties: VkDisplayPropertiesKHR,
}
impl Default for VkDisplayProperties2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR,
      next: core::ptr::null_mut(),
      display_properties: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPropertiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplayPropertiesKHR {
  /// Handle of the display object
  pub display: VkDisplayKHR,
  /// Name of the display
  /// * Len: `null_terminated`
  pub display_name: *const u8,
  /// In millimeters?
  pub physical_dimensions: VkExtent2D,
  /// Max resolution for CRT?
  pub physical_resolution: VkExtent2D,
  /// one or more bits from VkSurfaceTransformFlagsKHR
  /// * Optional: true
  pub supported_transforms: VkSurfaceTransformFlagsKHR,
  /// VK_TRUE if the overlay plane's z-order can be changed on this display.
  pub plane_reorder_possible: VkBool32,
  /// VK_TRUE if this is a "smart" display that supports self-refresh/internal
  /// buffering.
  pub persistent_content: VkBool32,
}
impl Default for VkDisplayPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      display: Default::default(),
      display_name: core::ptr::null(),
      physical_dimensions: Default::default(),
      physical_resolution: Default::default(),
      supported_transforms: Default::default(),
      plane_reorder_possible: Default::default(),
      persistent_content: Default::default(),
    }
  }
}

/// Khronos: [VkDisplaySurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDisplaySurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkDisplaySurfaceCreateFlagsKHR,
  /// The mode to use when displaying this surface
  pub display_mode: VkDisplayModeKHR,
  /// The plane on which this surface appears.  Must be between 0 and the value
  /// returned by vkGetPhysicalDeviceDisplayPlanePropertiesKHR() in
  /// pPropertyCount.
  pub plane_index: u32,
  /// The z-order of the plane.
  pub plane_stack_index: u32,
  /// Transform to apply to the images as part of the scanout operation
  pub transform: VkSurfaceTransformFlagBitsKHR,
  /// Global alpha value.  Must be between 0 and 1, inclusive.  Ignored if
  /// alphaMode is not VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR
  pub global_alpha: c_float,
  /// What type of alpha blending to use.  Must be a bit from
  /// vkGetDisplayPlanePropertiesKHR::supportedAlpha.
  pub alpha_mode: VkDisplayPlaneAlphaFlagBitsKHR,
  /// size of the images to use with this surface
  pub image_extent: VkExtent2D,
}
impl Default for VkDisplaySurfaceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      display_mode: Default::default(),
      plane_index: Default::default(),
      plane_stack_index: Default::default(),
      transform: Default::default(),
      global_alpha: Default::default(),
      alpha_mode: Default::default(),
      image_extent: Default::default(),
    }
  }
}

/// Khronos: [VkDrawIndexedIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawIndexedIndirectCommand.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
  pub index_count: u32,
  pub instance_count: u32,
  pub first_index: u32,
  pub vertex_offset: i32,
  /// * No Auto-Validity
  pub first_instance: u32,
}
impl Default for VkDrawIndexedIndirectCommand {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      index_count: Default::default(),
      instance_count: Default::default(),
      first_index: Default::default(),
      vertex_offset: Default::default(),
      first_instance: Default::default(),
    }
  }
}

/// Khronos: [VkDrawIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawIndirectCommand.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDrawIndirectCommand {
  pub vertex_count: u32,
  pub instance_count: u32,
  pub first_vertex: u32,
  /// * No Auto-Validity
  pub first_instance: u32,
}
impl Default for VkDrawIndirectCommand {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      vertex_count: Default::default(),
      instance_count: Default::default(),
      first_vertex: Default::default(),
      first_instance: Default::default(),
    }
  }
}

/// Khronos: [VkDrawMeshTasksIndirectCommandEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandEXT {
  /// * No Auto-Validity
  pub group_count_x: u32,
  /// * No Auto-Validity
  pub group_count_y: u32,
  /// * No Auto-Validity
  pub group_count_z: u32,
}
impl Default for VkDrawMeshTasksIndirectCommandEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      group_count_x: Default::default(),
      group_count_y: Default::default(),
      group_count_z: Default::default(),
    }
  }
}

/// Khronos: [VkDrawMeshTasksIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandNV {
  pub task_count: u32,
  pub first_task: u32,
}
impl Default for VkDrawMeshTasksIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { task_count: Default::default(), first_task: Default::default() }
  }
}

/// Khronos: [VkDrmFormatModifierProperties2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierProperties2EXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDrmFormatModifierProperties2EXT {
  pub drm_format_modifier: u64,
  pub drm_format_modifier_plane_count: u32,
  pub drm_format_modifier_tiling_features: VkFormatFeatureFlags2,
}
impl Default for VkDrmFormatModifierProperties2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      drm_format_modifier: Default::default(),
      drm_format_modifier_plane_count: Default::default(),
      drm_format_modifier_tiling_features: Default::default(),
    }
  }
}

/// Khronos: [VkDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDrmFormatModifierPropertiesEXT {
  pub drm_format_modifier: u64,
  pub drm_format_modifier_plane_count: u32,
  pub drm_format_modifier_tiling_features: VkFormatFeatureFlags,
}
impl Default for VkDrmFormatModifierPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      drm_format_modifier: Default::default(),
      drm_format_modifier_plane_count: Default::default(),
      drm_format_modifier_tiling_features: Default::default(),
    }
  }
}

/// Khronos: [VkDrmFormatModifierPropertiesList2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesList2EXT.html) (structure)
///
/// * Struct Extends: [`VkFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDrmFormatModifierPropertiesList2EXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  pub drm_format_modifier_count: u32,
  /// * Optional: true
  /// * Len: `drm_format_modifier_count`
  pub drm_format_modifier_properties: *mut VkDrmFormatModifierProperties2EXT,
}
impl Default for VkDrmFormatModifierPropertiesList2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT,
      next: core::ptr::null_mut(),
      drm_format_modifier_count: Default::default(),
      drm_format_modifier_properties: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDrmFormatModifierPropertiesListEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html) (structure)
///
/// * Struct Extends: [`VkFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkDrmFormatModifierPropertiesListEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  pub drm_format_modifier_count: u32,
  /// * Optional: true
  /// * Len: `drm_format_modifier_count`
  pub drm_format_modifier_properties: *mut VkDrmFormatModifierPropertiesEXT,
}
impl Default for VkDrmFormatModifierPropertiesListEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT,
      next: core::ptr::null_mut(),
      drm_format_modifier_count: Default::default(),
      drm_format_modifier_properties: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkEventCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkEventCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EVENT_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Event creation flags
  /// * Optional: true
  pub flags: VkEventCreateFlags,
}
impl Default for VkEventCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EVENT_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkExportFenceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportFenceCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkFenceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportFenceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_types: VkExternalFenceHandleTypeFlags,
}
impl Default for VkExportFenceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO,
      next: core::ptr::null(),
      handle_types: Default::default(),
    }
  }
}

/// Khronos: [VkExportFenceWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkFenceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportFenceWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub attributes: *const SECURITY_ATTRIBUTES,
  pub dw_access: DWORD,
  pub name: LPCWSTR,
}
impl Default for VkExportFenceWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
      next: core::ptr::null(),
      attributes: core::ptr::null(),
      dw_access: Default::default(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMemoryAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfo.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMemoryAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_types: VkExternalMemoryHandleTypeFlags,
}
impl Default for VkExportMemoryAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO,
      next: core::ptr::null(),
      handle_types: Default::default(),
    }
  }
}

/// Khronos: [VkExportMemoryAllocateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMemoryAllocateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_types: VkExternalMemoryHandleTypeFlagsNV,
}
impl Default for VkExportMemoryAllocateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV,
      next: core::ptr::null(),
      handle_types: Default::default(),
    }
  }
}

/// Khronos: [VkExportMemoryWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub attributes: *const SECURITY_ATTRIBUTES,
  pub dw_access: DWORD,
  pub name: LPCWSTR,
}
impl Default for VkExportMemoryWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
      next: core::ptr::null(),
      attributes: core::ptr::null(),
      dw_access: Default::default(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMemoryWin32HandleInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoNV.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub attributes: *const SECURITY_ATTRIBUTES,
  /// * Optional: true
  pub dw_access: DWORD,
}
impl Default for VkExportMemoryWin32HandleInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
      next: core::ptr::null(),
      attributes: core::ptr::null(),
      dw_access: Default::default(),
    }
  }
}

/// Khronos: [VkExportMetalBufferInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalBufferInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMetalBufferInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory: VkDeviceMemory,
  pub mtl_buffer: MTLBuffer_id,
}
impl Default for VkExportMetalBufferInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT,
      next: core::ptr::null(),
      memory: Default::default(),
      mtl_buffer: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalCommandQueueInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalCommandQueueInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMetalCommandQueueInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub queue: VkQueue,
  pub mtl_command_queue: MTLCommandQueue_id,
}
impl Default for VkExportMetalCommandQueueInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT,
      next: core::ptr::null(),
      queue: Default::default(),
      mtl_command_queue: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalDeviceInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalDeviceInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMetalDeviceInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub mtl_device: MTLDevice_id,
}
impl Default for VkExportMetalDeviceInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT,
      next: core::ptr::null(),
      mtl_device: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalIOSurfaceInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalIOSurfaceInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMetalIOSurfaceInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image: VkImage,
  pub io_surface: IOSurfaceRef,
}
impl Default for VkExportMetalIOSurfaceInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT,
      next: core::ptr::null(),
      image: Default::default(),
      io_surface: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalObjectCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
/// * Struct Extends: [`VkMemoryAllocateInfo`]
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkImageViewCreateInfo`]
/// * Struct Extends: [`VkBufferViewCreateInfo`]
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
/// * Struct Extends: [`VkEventCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMetalObjectCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub export_object_type: VkExportMetalObjectTypeFlagBitsEXT,
}
impl Default for VkExportMetalObjectCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT,
      next: core::ptr::null(),
      export_object_type: Default::default(),
    }
  }
}

/// Khronos: [VkExportMetalObjectsInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectsInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMetalObjectsInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
}
impl Default for VkExportMetalObjectsInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT,
      next: core::ptr::null(),
    }
  }
}

/// Khronos: [VkExportMetalSharedEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalSharedEventInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMetalSharedEventInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  pub event: VkEvent,
  pub mtl_shared_event: MTLSharedEvent_id,
}
impl Default for VkExportMetalSharedEventInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT,
      next: core::ptr::null(),
      semaphore: Default::default(),
      event: Default::default(),
      mtl_shared_event: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalTextureInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalTextureInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportMetalTextureInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub image: VkImage,
  /// * Optional: true
  pub image_view: VkImageView,
  /// * Optional: true
  pub buffer_view: VkBufferView,
  pub plane: VkImageAspectFlagBits,
  pub mtl_texture: MTLTexture_id,
}
impl Default for VkExportMetalTextureInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT,
      next: core::ptr::null(),
      image: Default::default(),
      image_view: Default::default(),
      buffer_view: Default::default(),
      plane: Default::default(),
      mtl_texture: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportSemaphoreCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportSemaphoreCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_types: VkExternalSemaphoreHandleTypeFlags,
}
impl Default for VkExportSemaphoreCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO,
      next: core::ptr::null(),
      handle_types: Default::default(),
    }
  }
}

/// Khronos: [VkExportSemaphoreWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreWin32HandleInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub attributes: *const SECURITY_ATTRIBUTES,
  pub dw_access: DWORD,
  pub name: LPCWSTR,
}
impl Default for VkExportSemaphoreWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
      next: core::ptr::null(),
      attributes: core::ptr::null(),
      dw_access: Default::default(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtensionProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExtensionProperties {
  /// extension name
  pub extension_name: zstring::ArrayZString<VK_MAX_EXTENSION_NAME_SIZE>,
  /// version of the extension specification implemented
  pub spec_version: u32,
}
impl Default for VkExtensionProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { extension_name: Default::default(), spec_version: Default::default() }
  }
}

/// Khronos: [VkExtent2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtent2D.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExtent2D {
  pub width: u32,
  pub height: u32,
}
impl Default for VkExtent2D {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { width: Default::default(), height: Default::default() }
  }
}

/// Khronos: [VkExtent3D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtent3D.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExtent3D {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
}
impl Default for VkExtent3D {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      width: Default::default(),
      height: Default::default(),
      depth: Default::default(),
    }
  }
}

/// Khronos: [VkExternalBufferProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalBufferProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalBufferProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub external_memory_properties: VkExternalMemoryProperties,
}
impl Default for VkExternalBufferProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES,
      next: core::ptr::null_mut(),
      external_memory_properties: Default::default(),
    }
  }
}

/// Khronos: [VkExternalFenceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalFenceProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub export_from_imported_handle_types: VkExternalFenceHandleTypeFlags,
  pub compatible_handle_types: VkExternalFenceHandleTypeFlags,
  /// * Optional: true
  pub external_fence_features: VkExternalFenceFeatureFlags,
}
impl Default for VkExternalFenceProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES,
      next: core::ptr::null_mut(),
      export_from_imported_handle_types: Default::default(),
      compatible_handle_types: Default::default(),
      external_fence_features: Default::default(),
    }
  }
}

/// Khronos: [VkExternalFormatANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFormatANDROID.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkSamplerYcbcrConversionCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalFormatANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub external_format: u64,
}
impl Default for VkExternalFormatANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID,
      next: core::ptr::null_mut(),
      external_format: Default::default(),
    }
  }
}

/// Khronos: [VkExternalImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatProperties.html) (structure)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalImageFormatProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub external_memory_properties: VkExternalMemoryProperties,
}
impl Default for VkExternalImageFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES,
      next: core::ptr::null_mut(),
      external_memory_properties: Default::default(),
    }
  }
}

/// Khronos: [VkExternalImageFormatPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatPropertiesNV.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalImageFormatPropertiesNV {
  pub image_format_properties: VkImageFormatProperties,
  /// * Optional: true
  pub external_memory_features: VkExternalMemoryFeatureFlagsNV,
  /// * Optional: true
  pub export_from_imported_handle_types: VkExternalMemoryHandleTypeFlagsNV,
  /// * Optional: true
  pub compatible_handle_types: VkExternalMemoryHandleTypeFlagsNV,
}
impl Default for VkExternalImageFormatPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      image_format_properties: Default::default(),
      external_memory_features: Default::default(),
      export_from_imported_handle_types: Default::default(),
      compatible_handle_types: Default::default(),
    }
  }
}

/// Khronos: [VkExternalMemoryBufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryBufferCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalMemoryBufferCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_types: VkExternalMemoryHandleTypeFlags,
}
impl Default for VkExternalMemoryBufferCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO,
      next: core::ptr::null(),
      handle_types: Default::default(),
    }
  }
}

/// Khronos: [VkExternalMemoryImageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalMemoryImageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_types: VkExternalMemoryHandleTypeFlags,
}
impl Default for VkExternalMemoryImageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO,
      next: core::ptr::null(),
      handle_types: Default::default(),
    }
  }
}

/// Khronos: [VkExternalMemoryImageCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalMemoryImageCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_types: VkExternalMemoryHandleTypeFlagsNV,
}
impl Default for VkExternalMemoryImageCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
      next: core::ptr::null(),
      handle_types: Default::default(),
    }
  }
}

/// Khronos: [VkExternalMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalMemoryProperties {
  pub external_memory_features: VkExternalMemoryFeatureFlags,
  /// * Optional: true
  pub export_from_imported_handle_types: VkExternalMemoryHandleTypeFlags,
  pub compatible_handle_types: VkExternalMemoryHandleTypeFlags,
}
impl Default for VkExternalMemoryProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      external_memory_features: Default::default(),
      export_from_imported_handle_types: Default::default(),
      compatible_handle_types: Default::default(),
    }
  }
}

/// Khronos: [VkExternalSemaphoreProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkExternalSemaphoreProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub export_from_imported_handle_types: VkExternalSemaphoreHandleTypeFlags,
  pub compatible_handle_types: VkExternalSemaphoreHandleTypeFlags,
  /// * Optional: true
  pub external_semaphore_features: VkExternalSemaphoreFeatureFlags,
}
impl Default for VkExternalSemaphoreProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES,
      next: core::ptr::null_mut(),
      export_from_imported_handle_types: Default::default(),
      compatible_handle_types: Default::default(),
      external_semaphore_features: Default::default(),
    }
  }
}

/// Khronos: [VkFenceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFenceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Fence creation flags
  /// * Optional: true
  pub flags: VkFenceCreateFlags,
}
impl Default for VkFenceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkFenceGetFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceGetFdInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFenceGetFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub fence: VkFence,
  pub handle_type: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkFenceGetFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR,
      next: core::ptr::null(),
      fence: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkFenceGetSciSyncInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceGetSciSyncInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFenceGetSciSyncInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FENCE_GET_SCI_SYNC_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub fence: VkFence,
  pub handle_type: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkFenceGetSciSyncInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FENCE_GET_SCI_SYNC_INFO_NV,
      next: core::ptr::null(),
      fence: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkFenceGetWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFenceGetWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub fence: VkFence,
  pub handle_type: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkFenceGetWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR,
      next: core::ptr::null(),
      fence: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkFilterCubicImageViewImageFormatPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFilterCubicImageViewImageFormatPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// The combinations of format, image type (and image view type if provided)
  /// can be filtered with VK_FILTER_CUBIC_EXT
  pub filter_cubic: VkBool32,
  /// The combination of format, image type (and image view type if provided)
  /// can be filtered with VK_FILTER_CUBIC_EXT and ReductionMode of Min or Max
  pub filter_cubic_minmax: VkBool32,
}
impl Default for VkFilterCubicImageViewImageFormatPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      filter_cubic: Default::default(),
      filter_cubic_minmax: Default::default(),
    }
  }
}

/// Khronos: [VkFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFormatProperties {
  /// Format features in case of linear tiling
  /// * Optional: true
  /// * Limit Type: bitmask
  pub linear_tiling_features: VkFormatFeatureFlags,
  /// Format features in case of optimal tiling
  /// * Optional: true
  /// * Limit Type: bitmask
  pub optimal_tiling_features: VkFormatFeatureFlags,
  /// Format features supported by buffers
  /// * Optional: true
  /// * Limit Type: bitmask
  pub buffer_features: VkFormatFeatureFlags,
}
impl Default for VkFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      linear_tiling_features: Default::default(),
      optimal_tiling_features: Default::default(),
      buffer_features: Default::default(),
    }
  }
}

/// Khronos: [VkFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatProperties2.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFormatProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub format_properties: VkFormatProperties,
}
impl Default for VkFormatProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2,
      next: core::ptr::null_mut(),
      format_properties: Default::default(),
    }
  }
}

/// Khronos: [VkFormatProperties3](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatProperties3.html) (structure)
///
/// * Struct Extends: [`VkFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFormatProperties3 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub linear_tiling_features: VkFormatFeatureFlags2,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub optimal_tiling_features: VkFormatFeatureFlags2,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub buffer_features: VkFormatFeatureFlags2,
}
impl Default for VkFormatProperties3 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3,
      next: core::ptr::null_mut(),
      linear_tiling_features: Default::default(),
      optimal_tiling_features: Default::default(),
      buffer_features: Default::default(),
    }
  }
}

/// Khronos: [VkFragmentShadingRateAttachmentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateAttachmentInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkSubpassDescription2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFragmentShadingRateAttachmentInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub fragment_shading_rate_attachment: *const VkAttachmentReference2,
  pub shading_rate_attachment_texel_size: VkExtent2D,
}
impl Default for VkFragmentShadingRateAttachmentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
      next: core::ptr::null(),
      fragment_shading_rate_attachment: core::ptr::null(),
      shading_rate_attachment_texel_size: Default::default(),
    }
  }
}

/// Khronos: [VkFramebufferAttachmentImageInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentImageInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFramebufferAttachmentImageInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Image creation flags
  /// * Optional: true
  pub flags: VkImageCreateFlags,
  /// Image usage flags
  pub usage: VkImageUsageFlags,
  pub width: u32,
  pub height: u32,
  pub layer_count: u32,
  /// * Optional: true
  pub view_format_count: u32,
  /// * Len: `view_format_count`
  pub view_formats: *const VkFormat,
}
impl Default for VkFramebufferAttachmentImageInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      usage: Default::default(),
      width: Default::default(),
      height: Default::default(),
      layer_count: Default::default(),
      view_format_count: Default::default(),
      view_formats: core::ptr::null(),
    }
  }
}

/// Khronos: [VkFramebufferAttachmentsCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentsCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkFramebufferCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFramebufferAttachmentsCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub attachment_image_info_count: u32,
  /// * Len: `attachment_image_info_count`
  pub attachment_image_infos: *const VkFramebufferAttachmentImageInfo,
}
impl Default for VkFramebufferAttachmentsCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO,
      next: core::ptr::null(),
      attachment_image_info_count: Default::default(),
      attachment_image_infos: core::ptr::null(),
    }
  }
}

/// Khronos: [VkFramebufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFramebufferCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkFramebufferCreateFlags,
  pub render_pass: VkRenderPass,
  /// * Optional: true
  pub attachment_count: u32,
  /// * Len: `attachment_count`
  /// * No Auto-Validity
  pub attachments: *const VkImageView,
  pub width: u32,
  pub height: u32,
  pub layers: u32,
}
impl Default for VkFramebufferCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      render_pass: Default::default(),
      attachment_count: Default::default(),
      attachments: core::ptr::null(),
      width: Default::default(),
      height: Default::default(),
      layers: Default::default(),
    }
  }
}

/// Khronos: [VkFramebufferMixedSamplesCombinationNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferMixedSamplesCombinationNV.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkFramebufferMixedSamplesCombinationNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub coverage_reduction_mode: VkCoverageReductionModeNV,
  pub rasterization_samples: VkSampleCountFlagBits,
  pub depth_stencil_samples: VkSampleCountFlags,
  pub color_samples: VkSampleCountFlags,
}
impl Default for VkFramebufferMixedSamplesCombinationNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV,
      next: core::ptr::null_mut(),
      coverage_reduction_mode: Default::default(),
      rasterization_samples: Default::default(),
      depth_stencil_samples: Default::default(),
      color_samples: Default::default(),
    }
  }
}

/// Khronos: [VkGeneratedCommandsInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGeneratedCommandsInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub pipeline_bind_point: VkPipelineBindPoint,
  pub pipeline: VkPipeline,
  pub indirect_commands_layout: VkIndirectCommandsLayoutNV,
  pub stream_count: u32,
  /// * Len: `stream_count`
  pub streams: *const VkIndirectCommandsStreamNV,
  pub sequences_count: u32,
  pub preprocess_buffer: VkBuffer,
  pub preprocess_offset: VkDeviceSize,
  pub preprocess_size: VkDeviceSize,
  /// * Optional: true
  pub sequences_count_buffer: VkBuffer,
  pub sequences_count_offset: VkDeviceSize,
  /// * Optional: true
  pub sequences_index_buffer: VkBuffer,
  pub sequences_index_offset: VkDeviceSize,
}
impl Default for VkGeneratedCommandsInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV,
      next: core::ptr::null(),
      pipeline_bind_point: Default::default(),
      pipeline: Default::default(),
      indirect_commands_layout: Default::default(),
      stream_count: Default::default(),
      streams: core::ptr::null(),
      sequences_count: Default::default(),
      preprocess_buffer: Default::default(),
      preprocess_offset: Default::default(),
      preprocess_size: Default::default(),
      sequences_count_buffer: Default::default(),
      sequences_count_offset: Default::default(),
      sequences_index_buffer: Default::default(),
      sequences_index_offset: Default::default(),
    }
  }
}

/// Khronos: [VkGeneratedCommandsMemoryRequirementsInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub pipeline_bind_point: VkPipelineBindPoint,
  pub pipeline: VkPipeline,
  pub indirect_commands_layout: VkIndirectCommandsLayoutNV,
  pub max_sequences_count: u32,
}
impl Default for VkGeneratedCommandsMemoryRequirementsInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV,
      next: core::ptr::null(),
      pipeline_bind_point: Default::default(),
      pipeline: Default::default(),
      indirect_commands_layout: Default::default(),
      max_sequences_count: Default::default(),
    }
  }
}

/// Khronos: [VkGeometryAABBNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryAABBNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGeometryAABBNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub aabb_data: VkBuffer,
  pub num_aab_bs: u32,
  /// Stride in bytes between AABBs
  pub stride: u32,
  /// Offset in bytes of the first AABB in aabbData
  pub offset: VkDeviceSize,
}
impl Default for VkGeometryAABBNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV,
      next: core::ptr::null(),
      aabb_data: Default::default(),
      num_aab_bs: Default::default(),
      stride: Default::default(),
      offset: Default::default(),
    }
  }
}

/// Khronos: [VkGeometryDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryDataNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGeometryDataNV {
  pub triangles: VkGeometryTrianglesNV,
  pub aabbs: VkGeometryAABBNV,
}
impl Default for VkGeometryDataNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { triangles: Default::default(), aabbs: Default::default() }
  }
}

/// Khronos: [VkGeometryNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGeometryNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GEOMETRY_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub geometry_type: VkGeometryTypeKHR,
  pub geometry: VkGeometryDataNV,
  /// * Optional: true
  pub flags: VkGeometryFlagsKHR,
}
impl Default for VkGeometryNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GEOMETRY_NV,
      next: core::ptr::null(),
      geometry_type: Default::default(),
      geometry: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkGeometryTrianglesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryTrianglesNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGeometryTrianglesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub vertex_data: VkBuffer,
  pub vertex_offset: VkDeviceSize,
  pub vertex_count: u32,
  pub vertex_stride: VkDeviceSize,
  pub vertex_format: VkFormat,
  /// * Optional: true
  pub index_data: VkBuffer,
  pub index_offset: VkDeviceSize,
  pub index_count: u32,
  pub index_type: VkIndexType,
  /// Optional reference to array of floats representing a 3x4 row major affine
  /// transformation matrix.
  /// * Optional: true
  pub transform_data: VkBuffer,
  pub transform_offset: VkDeviceSize,
}
impl Default for VkGeometryTrianglesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV,
      next: core::ptr::null(),
      vertex_data: Default::default(),
      vertex_offset: Default::default(),
      vertex_count: Default::default(),
      vertex_stride: Default::default(),
      vertex_format: Default::default(),
      index_data: Default::default(),
      index_offset: Default::default(),
      index_count: Default::default(),
      index_type: Default::default(),
      transform_data: Default::default(),
      transform_offset: Default::default(),
    }
  }
}

/// Khronos: [VkGraphicsPipelineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Pipeline creation flags
  /// * Optional: true
  pub flags: VkPipelineCreateFlags,
  /// * Optional: true
  /// * No Auto-Validity
  pub stage_count: u32,
  /// One entry for each active shader stage
  /// * Optional: true
  /// * Len: `stage_count`
  /// * No Auto-Validity
  pub stages: *const VkPipelineShaderStageCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub vertex_input_state: *const VkPipelineVertexInputStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub input_assembly_state: *const VkPipelineInputAssemblyStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub tessellation_state: *const VkPipelineTessellationStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub viewport_state: *const VkPipelineViewportStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub rasterization_state: *const VkPipelineRasterizationStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub multisample_state: *const VkPipelineMultisampleStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub depth_stencil_state: *const VkPipelineDepthStencilStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub color_blend_state: *const VkPipelineColorBlendStateCreateInfo,
  /// * Optional: true
  pub dynamic_state: *const VkPipelineDynamicStateCreateInfo,
  /// Interface layout of the pipeline
  /// * Optional: true
  /// * No Auto-Validity
  pub layout: VkPipelineLayout,
  /// * Optional: true
  /// * No Auto-Validity
  pub render_pass: VkRenderPass,
  /// * No Auto-Validity
  pub subpass: u32,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * Optional: true
  /// * No Auto-Validity
  pub base_pipeline_handle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  pub base_pipeline_index: i32,
}
impl Default for VkGraphicsPipelineCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      stage_count: Default::default(),
      stages: core::ptr::null(),
      vertex_input_state: core::ptr::null(),
      input_assembly_state: core::ptr::null(),
      tessellation_state: core::ptr::null(),
      viewport_state: core::ptr::null(),
      rasterization_state: core::ptr::null(),
      multisample_state: core::ptr::null(),
      depth_stencil_state: core::ptr::null(),
      color_blend_state: core::ptr::null(),
      dynamic_state: core::ptr::null(),
      layout: Default::default(),
      render_pass: Default::default(),
      subpass: Default::default(),
      base_pipeline_handle: Default::default(),
      base_pipeline_index: Default::default(),
    }
  }
}

/// Khronos: [VkGraphicsPipelineLibraryCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGraphicsPipelineLibraryCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub flags: VkGraphicsPipelineLibraryFlagsEXT,
}
impl Default for VkGraphicsPipelineLibraryCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT,
      next: core::ptr::null_mut(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkGraphicsPipelineShaderGroupsCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub group_count: u32,
  /// * Len: `group_count`
  pub groups: *const VkGraphicsShaderGroupCreateInfoNV,
  /// * Optional: true
  pub pipeline_count: u32,
  /// * Len: `pipeline_count`
  pub pipelines: *const VkPipeline,
}
impl Default for VkGraphicsPipelineShaderGroupsCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV,
      next: core::ptr::null(),
      group_count: Default::default(),
      groups: core::ptr::null(),
      pipeline_count: Default::default(),
      pipelines: core::ptr::null(),
    }
  }
}

/// Khronos: [VkGraphicsShaderGroupCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkGraphicsShaderGroupCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub stage_count: u32,
  /// * Len: `stage_count`
  pub stages: *const VkPipelineShaderStageCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub vertex_input_state: *const VkPipelineVertexInputStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub tessellation_state: *const VkPipelineTessellationStateCreateInfo,
}
impl Default for VkGraphicsShaderGroupCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV,
      next: core::ptr::null(),
      stage_count: Default::default(),
      stages: core::ptr::null(),
      vertex_input_state: core::ptr::null(),
      tessellation_state: core::ptr::null(),
    }
  }
}

/// Khronos: [VkHdrMetadataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHdrMetadataEXT.html) (structure)
///
/// From CTA 861.3
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkHdrMetadataEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_HDR_METADATA_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Display primary's Red
  /// * No Auto-Validity
  pub display_primary_red: VkXYColorEXT,
  /// Display primary's Green
  /// * No Auto-Validity
  pub display_primary_green: VkXYColorEXT,
  /// Display primary's Blue
  /// * No Auto-Validity
  pub display_primary_blue: VkXYColorEXT,
  /// Display primary's Blue
  /// * No Auto-Validity
  pub white_point: VkXYColorEXT,
  /// Display maximum luminance
  /// * No Auto-Validity
  pub max_luminance: c_float,
  /// Display minimum luminance
  /// * No Auto-Validity
  pub min_luminance: c_float,
  /// Content maximum luminance
  /// * No Auto-Validity
  pub max_content_light_level: c_float,
  /// * No Auto-Validity
  pub max_frame_average_light_level: c_float,
}
impl Default for VkHdrMetadataEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_HDR_METADATA_EXT,
      next: core::ptr::null(),
      display_primary_red: Default::default(),
      display_primary_green: Default::default(),
      display_primary_blue: Default::default(),
      white_point: Default::default(),
      max_luminance: Default::default(),
      min_luminance: Default::default(),
      max_content_light_level: Default::default(),
      max_frame_average_light_level: Default::default(),
    }
  }
}

/// Khronos: [VkHeadlessSurfaceCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkHeadlessSurfaceCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkHeadlessSurfaceCreateFlagsEXT,
}
impl Default for VkHeadlessSurfaceCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkIOSSurfaceCreateInfoMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkIOSSurfaceCreateInfoMVK {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkIOSSurfaceCreateFlagsMVK,
  /// * No Auto-Validity
  pub view: *const c_void,
}
impl Default for VkIOSSurfaceCreateInfoMVK {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK,
      next: core::ptr::null(),
      flags: Default::default(),
      view: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageBlit](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageBlit.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageBlit {
  pub src_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub src_offsets: [VkOffset3D; 2],
  pub dst_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub dst_offsets: [VkOffset3D; 2],
}
impl Default for VkImageBlit {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      src_subresource: Default::default(),
      src_offsets: [Default::default(); 2],
      dst_subresource: Default::default(),
      dst_offsets: [Default::default(); 2],
    }
  }
}

/// Khronos: [VkImageBlit2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageBlit2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageBlit2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_BLIT_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub src_offsets: [VkOffset3D; 2],
  pub dst_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub dst_offsets: [VkOffset3D; 2],
}
impl Default for VkImageBlit2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_BLIT_2,
      next: core::ptr::null(),
      src_subresource: Default::default(),
      src_offsets: [Default::default(); 2],
      dst_subresource: Default::default(),
      dst_offsets: [Default::default(); 2],
    }
  }
}

/// Khronos: [VkImageCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCaptureDescriptorDataInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageCaptureDescriptorDataInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image: VkImage,
}
impl Default for VkImageCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      next: core::ptr::null(),
      image: Default::default(),
    }
  }
}

/// Khronos: [VkImageCompressionControlEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionControlEXT.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageCompressionControlEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_CONTROL_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * No Auto-Validity
  pub flags: VkImageCompressionFlagsEXT,
  /// * Optional: true
  pub compression_control_plane_count: u32,
  /// * Len: `compression_control_plane_count`
  /// * No Auto-Validity
  pub fixed_rate_flags: *mut VkImageCompressionFixedRateFlagsEXT,
}
impl Default for VkImageCompressionControlEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_CONTROL_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      compression_control_plane_count: Default::default(),
      fixed_rate_flags: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImageCompressionPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Struct Extends: [`VkSurfaceFormat2KHR`]
/// * Struct Extends: [`VkSubresourceLayout2EXT`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageCompressionPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub image_compression_flags: VkImageCompressionFlagsEXT,
  pub image_compression_fixed_rate_flags: VkImageCompressionFixedRateFlagsEXT,
}
impl Default for VkImageCompressionPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      image_compression_flags: Default::default(),
      image_compression_fixed_rate_flags: Default::default(),
    }
  }
}

/// Khronos: [VkImageConstraintsInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageConstraintsInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub format_constraints_count: u32,
  /// * Len: `format_constraints_count`
  pub format_constraints: *const VkImageFormatConstraintsInfoFUCHSIA,
  pub buffer_collection_constraints: VkBufferCollectionConstraintsInfoFUCHSIA,
  /// * Optional: true
  pub flags: VkImageConstraintsInfoFlagsFUCHSIA,
}
impl Default for VkImageConstraintsInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA,
      next: core::ptr::null(),
      format_constraints_count: Default::default(),
      format_constraints: core::ptr::null(),
      buffer_collection_constraints: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkImageCopy](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCopy.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageCopy {
  pub src_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub src_offset: VkOffset3D,
  pub dst_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub dst_offset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub extent: VkExtent3D,
}
impl Default for VkImageCopy {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      src_subresource: Default::default(),
      src_offset: Default::default(),
      dst_subresource: Default::default(),
      dst_offset: Default::default(),
      extent: Default::default(),
    }
  }
}

/// Khronos: [VkImageCopy2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCopy2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageCopy2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_COPY_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub src_offset: VkOffset3D,
  pub dst_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub dst_offset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub extent: VkExtent3D,
}
impl Default for VkImageCopy2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_COPY_2,
      next: core::ptr::null(),
      src_subresource: Default::default(),
      src_offset: Default::default(),
      dst_subresource: Default::default(),
      dst_offset: Default::default(),
      extent: Default::default(),
    }
  }
}

/// Khronos: [VkImageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Image creation flags
  /// * Optional: true
  pub flags: VkImageCreateFlags,
  pub image_type: VkImageType,
  pub format: VkFormat,
  pub extent: VkExtent3D,
  pub mip_levels: u32,
  pub array_layers: u32,
  pub samples: VkSampleCountFlagBits,
  pub tiling: VkImageTiling,
  /// Image usage flags
  pub usage: VkImageUsageFlags,
  /// Cross-queue-family sharing mode
  pub sharing_mode: VkSharingMode,
  /// Number of queue families to share across
  /// * Optional: true
  pub queue_family_index_count: u32,
  /// Array of queue family indices to share across
  /// * Len: `queue_family_index_count`
  /// * No Auto-Validity
  pub queue_family_indices: *const u32,
  /// Initial image layout for all subresources
  pub initial_layout: VkImageLayout,
}
impl Default for VkImageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      image_type: Default::default(),
      format: Default::default(),
      extent: Default::default(),
      mip_levels: Default::default(),
      array_layers: Default::default(),
      samples: Default::default(),
      tiling: Default::default(),
      usage: Default::default(),
      sharing_mode: Default::default(),
      queue_family_index_count: Default::default(),
      queue_family_indices: core::ptr::null(),
      initial_layout: Default::default(),
    }
  }
}

/// Khronos: [VkImageDrmFormatModifierExplicitCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub drm_format_modifier: u64,
  /// * Optional: false
  pub drm_format_modifier_plane_count: u32,
  /// * Len: `drm_format_modifier_plane_count`
  pub plane_layouts: *const VkSubresourceLayout,
}
impl Default for VkImageDrmFormatModifierExplicitCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT,
      next: core::ptr::null(),
      drm_format_modifier: Default::default(),
      drm_format_modifier_plane_count: Default::default(),
      plane_layouts: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageDrmFormatModifierListCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageDrmFormatModifierListCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub drm_format_modifier_count: u32,
  /// * Len: `drm_format_modifier_count`
  pub drm_format_modifiers: *const u64,
}
impl Default for VkImageDrmFormatModifierListCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT,
      next: core::ptr::null(),
      drm_format_modifier_count: Default::default(),
      drm_format_modifiers: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageDrmFormatModifierPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub drm_format_modifier: u64,
}
impl Default for VkImageDrmFormatModifierPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      drm_format_modifier: Default::default(),
    }
  }
}

/// Khronos: [VkImageFormatConstraintsInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsInfoFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageFormatConstraintsInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image_create_info: VkImageCreateInfo,
  pub required_format_features: VkFormatFeatureFlags,
  /// * Optional: true
  pub flags: VkImageFormatConstraintsFlagsFUCHSIA,
  /// * Optional: true
  pub sysmem_pixel_format: u64,
  pub color_space_count: u32,
  /// * Len: `color_space_count`
  pub color_spaces: *const VkSysmemColorSpaceFUCHSIA,
}
impl Default for VkImageFormatConstraintsInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA,
      next: core::ptr::null(),
      image_create_info: Default::default(),
      required_format_features: Default::default(),
      flags: Default::default(),
      sysmem_pixel_format: Default::default(),
      color_space_count: Default::default(),
      color_spaces: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageFormatListCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatListCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageFormatListCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub view_format_count: u32,
  /// * Len: `view_format_count`
  pub view_formats: *const VkFormat,
}
impl Default for VkImageFormatListCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO,
      next: core::ptr::null(),
      view_format_count: Default::default(),
      view_formats: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageFormatProperties {
  /// max image dimensions for this resource type
  pub max_extent: VkExtent3D,
  /// max number of mipmap levels for this resource type
  pub max_mip_levels: u32,
  /// max array size for this resource type
  pub max_array_layers: u32,
  /// supported sample counts for this resource type
  /// * Optional: true
  pub sample_counts: VkSampleCountFlags,
  /// max size (in bytes) of this resource type
  pub max_resource_size: VkDeviceSize,
}
impl Default for VkImageFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      max_extent: Default::default(),
      max_mip_levels: Default::default(),
      max_array_layers: Default::default(),
      sample_counts: Default::default(),
      max_resource_size: Default::default(),
    }
  }
}

/// Khronos: [VkImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties2.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageFormatProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub image_format_properties: VkImageFormatProperties,
}
impl Default for VkImageFormatProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2,
      next: core::ptr::null_mut(),
      image_format_properties: Default::default(),
    }
  }
}

/// Khronos: [VkImageMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageMemoryBarrier {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * No Auto-Validity
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * No Auto-Validity
  pub dst_access_mask: VkAccessFlags,
  /// Current layout of the image
  pub old_layout: VkImageLayout,
  /// New layout to transition the image to
  pub new_layout: VkImageLayout,
  /// Queue family to transition ownership from
  pub src_queue_family_index: u32,
  /// Queue family to transition ownership to
  pub dst_queue_family_index: u32,
  /// Image to sync
  pub image: VkImage,
  /// Subresource range to sync
  pub subresource_range: VkImageSubresourceRange,
}
impl Default for VkImageMemoryBarrier {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
      next: core::ptr::null(),
      src_access_mask: Default::default(),
      dst_access_mask: Default::default(),
      old_layout: Default::default(),
      new_layout: Default::default(),
      src_queue_family_index: Default::default(),
      dst_queue_family_index: Default::default(),
      image: Default::default(),
      subresource_range: Default::default(),
    }
  }
}

/// Khronos: [VkImageMemoryBarrier2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageMemoryBarrier2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub src_stage_mask: VkPipelineStageFlags2,
  /// * Optional: true
  pub src_access_mask: VkAccessFlags2,
  /// * Optional: true
  pub dst_stage_mask: VkPipelineStageFlags2,
  /// * Optional: true
  pub dst_access_mask: VkAccessFlags2,
  pub old_layout: VkImageLayout,
  pub new_layout: VkImageLayout,
  pub src_queue_family_index: u32,
  pub dst_queue_family_index: u32,
  pub image: VkImage,
  pub subresource_range: VkImageSubresourceRange,
}
impl Default for VkImageMemoryBarrier2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2,
      next: core::ptr::null(),
      src_stage_mask: Default::default(),
      src_access_mask: Default::default(),
      dst_stage_mask: Default::default(),
      dst_access_mask: Default::default(),
      old_layout: Default::default(),
      new_layout: Default::default(),
      src_queue_family_index: Default::default(),
      dst_queue_family_index: Default::default(),
      image: Default::default(),
      subresource_range: Default::default(),
    }
  }
}

/// Khronos: [VkImageMemoryRequirementsInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageMemoryRequirementsInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageMemoryRequirementsInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image: VkImage,
}
impl Default for VkImageMemoryRequirementsInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2,
      next: core::ptr::null(),
      image: Default::default(),
    }
  }
}

/// Khronos: [VkImagePipeSurfaceCreateInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkImagePipeSurfaceCreateFlagsFUCHSIA,
  pub image_pipe_handle: zx_handle_t,
}
impl Default for VkImagePipeSurfaceCreateInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
      next: core::ptr::null(),
      flags: Default::default(),
      image_pipe_handle: Default::default(),
    }
  }
}

/// Khronos: [VkImagePlaneMemoryRequirementsInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html) (structure)
///
/// * Struct Extends: [`VkImageMemoryRequirementsInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImagePlaneMemoryRequirementsInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub plane_aspect: VkImageAspectFlagBits,
}
impl Default for VkImagePlaneMemoryRequirementsInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO,
      next: core::ptr::null(),
      plane_aspect: Default::default(),
    }
  }
}

/// Khronos: [VkImageResolve](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageResolve.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageResolve {
  pub src_subresource: VkImageSubresourceLayers,
  pub src_offset: VkOffset3D,
  pub dst_subresource: VkImageSubresourceLayers,
  pub dst_offset: VkOffset3D,
  pub extent: VkExtent3D,
}
impl Default for VkImageResolve {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      src_subresource: Default::default(),
      src_offset: Default::default(),
      dst_subresource: Default::default(),
      dst_offset: Default::default(),
      extent: Default::default(),
    }
  }
}

/// Khronos: [VkImageResolve2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageResolve2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageResolve2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_subresource: VkImageSubresourceLayers,
  pub src_offset: VkOffset3D,
  pub dst_subresource: VkImageSubresourceLayers,
  pub dst_offset: VkOffset3D,
  pub extent: VkExtent3D,
}
impl Default for VkImageResolve2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2,
      next: core::ptr::null(),
      src_subresource: Default::default(),
      src_offset: Default::default(),
      dst_subresource: Default::default(),
      dst_offset: Default::default(),
      extent: Default::default(),
    }
  }
}

/// Khronos: [VkImageSparseMemoryRequirementsInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageSparseMemoryRequirementsInfo2 {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image: VkImage,
}
impl Default for VkImageSparseMemoryRequirementsInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2,
      next: core::ptr::null(),
      image: Default::default(),
    }
  }
}

/// Khronos: [VkImageStencilUsageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageStencilUsageCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageStencilUsageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub stencil_usage: VkImageUsageFlags,
}
impl Default for VkImageStencilUsageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO,
      next: core::ptr::null(),
      stencil_usage: Default::default(),
    }
  }
}

/// Khronos: [VkImageSubresource](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresource.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageSubresource {
  pub aspect_mask: VkImageAspectFlags,
  pub mip_level: u32,
  pub array_layer: u32,
}
impl Default for VkImageSubresource {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      aspect_mask: Default::default(),
      mip_level: Default::default(),
      array_layer: Default::default(),
    }
  }
}

/// Khronos: [VkImageSubresource2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresource2EXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageSubresource2EXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub image_subresource: VkImageSubresource,
}
impl Default for VkImageSubresource2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT,
      next: core::ptr::null_mut(),
      image_subresource: Default::default(),
    }
  }
}

/// Khronos: [VkImageSubresourceLayers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceLayers.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageSubresourceLayers {
  pub aspect_mask: VkImageAspectFlags,
  pub mip_level: u32,
  pub base_array_layer: u32,
  pub layer_count: u32,
}
impl Default for VkImageSubresourceLayers {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      aspect_mask: Default::default(),
      mip_level: Default::default(),
      base_array_layer: Default::default(),
      layer_count: Default::default(),
    }
  }
}

/// Khronos: [VkImageSubresourceRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceRange.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageSubresourceRange {
  pub aspect_mask: VkImageAspectFlags,
  pub base_mip_level: u32,
  pub level_count: u32,
  pub base_array_layer: u32,
  pub layer_count: u32,
}
impl Default for VkImageSubresourceRange {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      aspect_mask: Default::default(),
      base_mip_level: Default::default(),
      level_count: Default::default(),
      base_array_layer: Default::default(),
      layer_count: Default::default(),
    }
  }
}

/// Khronos: [VkImageSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSwapchainCreateInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageSwapchainCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub swapchain: VkSwapchainKHR,
}
impl Default for VkImageSwapchainCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR,
      next: core::ptr::null(),
      swapchain: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewASTCDecodeModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewASTCDecodeModeEXT.html) (structure)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageViewASTCDecodeModeEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub decode_mode: VkFormat,
}
impl Default for VkImageViewASTCDecodeModeEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT,
      next: core::ptr::null(),
      decode_mode: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewAddressPropertiesNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewAddressPropertiesNVX.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageViewAddressPropertiesNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub device_address: VkDeviceAddress,
  pub size: VkDeviceSize,
}
impl Default for VkImageViewAddressPropertiesNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX,
      next: core::ptr::null_mut(),
      device_address: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCaptureDescriptorDataInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageViewCaptureDescriptorDataInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image_view: VkImageView,
}
impl Default for VkImageViewCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      next: core::ptr::null(),
      image_view: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageViewCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
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
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      image: Default::default(),
      view_type: Default::default(),
      format: Default::default(),
      components: Default::default(),
      subresource_range: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewHandleInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewHandleInfoNVX.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageViewHandleInfoNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image_view: VkImageView,
  pub descriptor_type: VkDescriptorType,
  /// * Optional: true
  pub sampler: VkSampler,
}
impl Default for VkImageViewHandleInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX,
      next: core::ptr::null(),
      image_view: Default::default(),
      descriptor_type: Default::default(),
      sampler: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewMinLodCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewMinLodCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageViewMinLodCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub min_lod: c_float,
}
impl Default for VkImageViewMinLodCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT,
      next: core::ptr::null(),
      min_lod: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewSampleWeightCreateInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewSampleWeightCreateInfoQCOM.html) (structure)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageViewSampleWeightCreateInfoQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub filter_center: VkOffset2D,
  pub filter_size: VkExtent2D,
  pub num_phases: u32,
}
impl Default for VkImageViewSampleWeightCreateInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM,
      next: core::ptr::null(),
      filter_center: Default::default(),
      filter_size: Default::default(),
      num_phases: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewSlicedCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewSlicedCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageViewSlicedCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_SLICED_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub slice_offset: u32,
  pub slice_count: u32,
}
impl Default for VkImageViewSlicedCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_SLICED_CREATE_INFO_EXT,
      next: core::ptr::null(),
      slice_offset: Default::default(),
      slice_count: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewUsageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewUsageCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImageViewUsageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub usage: VkImageUsageFlags,
}
impl Default for VkImageViewUsageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO,
      next: core::ptr::null(),
      usage: Default::default(),
    }
  }
}

/// Khronos: [VkImportAndroidHardwareBufferInfoANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportAndroidHardwareBufferInfoANDROID.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportAndroidHardwareBufferInfoANDROID {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub buffer: *mut AHardwareBuffer,
}
impl Default for VkImportAndroidHardwareBufferInfoANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
      next: core::ptr::null(),
      buffer: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportFenceFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportFenceFdInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportFenceFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub fence: VkFence,
  /// * Optional: true
  pub flags: VkFenceImportFlags,
  pub handle_type: VkExternalFenceHandleTypeFlagBits,
  pub fd: c_int,
}
impl Default for VkImportFenceFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR,
      next: core::ptr::null(),
      fence: Default::default(),
      flags: Default::default(),
      handle_type: Default::default(),
      fd: Default::default(),
    }
  }
}

/// Khronos: [VkImportFenceSciSyncInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportFenceSciSyncInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportFenceSciSyncInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_FENCE_SCI_SYNC_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub fence: VkFence,
  pub handle_type: VkExternalFenceHandleTypeFlagBits,
  pub handle: *mut c_void,
}
impl Default for VkImportFenceSciSyncInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_FENCE_SCI_SYNC_INFO_NV,
      next: core::ptr::null(),
      fence: Default::default(),
      handle_type: Default::default(),
      handle: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportFenceWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportFenceWin32HandleInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportFenceWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub fence: VkFence,
  /// * Optional: true
  pub flags: VkFenceImportFlags,
  /// * No Auto-Validity
  pub handle_type: VkExternalFenceHandleTypeFlagBits,
  /// * Optional: true
  pub handle: HANDLE,
  /// * Optional: true
  pub name: LPCWSTR,
}
impl Default for VkImportFenceWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
      next: core::ptr::null(),
      fence: Default::default(),
      flags: Default::default(),
      handle_type: Default::default(),
      handle: core::ptr::null_mut(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMemoryBufferCollectionFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryBufferCollectionFUCHSIA.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMemoryBufferCollectionFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub collection: VkBufferCollectionFUCHSIA,
  pub index: u32,
}
impl Default for VkImportMemoryBufferCollectionFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA,
      next: core::ptr::null(),
      collection: Default::default(),
      index: Default::default(),
    }
  }
}

/// Khronos: [VkImportMemoryFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryFdInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMemoryFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
  pub fd: c_int,
}
impl Default for VkImportMemoryFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR,
      next: core::ptr::null(),
      handle_type: Default::default(),
      fd: Default::default(),
    }
  }
}

/// Khronos: [VkImportMemoryHostPointerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMemoryHostPointerInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
  /// * Optional: false
  pub host_pointer: *mut c_void,
}
impl Default for VkImportMemoryHostPointerInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
      next: core::ptr::null(),
      handle_type: Default::default(),
      host_pointer: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMemoryWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
  /// * Optional: true
  pub handle: HANDLE,
  /// * Optional: true
  pub name: LPCWSTR,
}
impl Default for VkImportMemoryWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
      next: core::ptr::null(),
      handle_type: Default::default(),
      handle: core::ptr::null_mut(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMemoryWin32HandleInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_type: VkExternalMemoryHandleTypeFlagsNV,
  /// * Optional: true
  pub handle: HANDLE,
}
impl Default for VkImportMemoryWin32HandleInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
      next: core::ptr::null(),
      handle_type: Default::default(),
      handle: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMemoryZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryZirconHandleInfoFUCHSIA.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMemoryZirconHandleInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
  /// * Optional: true
  pub handle: zx_handle_t,
}
impl Default for VkImportMemoryZirconHandleInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA,
      next: core::ptr::null(),
      handle_type: Default::default(),
      handle: Default::default(),
    }
  }
}

/// Khronos: [VkImportMetalBufferInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMetalBufferInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMetalBufferInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub mtl_buffer: MTLBuffer_id,
}
impl Default for VkImportMetalBufferInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT,
      next: core::ptr::null(),
      mtl_buffer: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMetalIOSurfaceInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMetalIOSurfaceInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMetalIOSurfaceInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub io_surface: IOSurfaceRef,
}
impl Default for VkImportMetalIOSurfaceInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT,
      next: core::ptr::null(),
      io_surface: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMetalSharedEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMetalSharedEventInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
/// * Struct Extends: [`VkEventCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMetalSharedEventInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub mtl_shared_event: MTLSharedEvent_id,
}
impl Default for VkImportMetalSharedEventInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT,
      next: core::ptr::null(),
      mtl_shared_event: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMetalTextureInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMetalTextureInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportMetalTextureInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub plane: VkImageAspectFlagBits,
  pub mtl_texture: MTLTexture_id,
}
impl Default for VkImportMetalTextureInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT,
      next: core::ptr::null(),
      plane: Default::default(),
      mtl_texture: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportSemaphoreFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreFdInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportSemaphoreFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  pub flags: VkSemaphoreImportFlags,
  pub handle_type: VkExternalSemaphoreHandleTypeFlagBits,
  pub fd: c_int,
}
impl Default for VkImportSemaphoreFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR,
      next: core::ptr::null(),
      semaphore: Default::default(),
      flags: Default::default(),
      handle_type: Default::default(),
      fd: Default::default(),
    }
  }
}

/// Khronos: [VkImportSemaphoreSciSyncInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreSciSyncInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportSemaphoreSciSyncInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  pub handle_type: VkExternalSemaphoreHandleTypeFlagBits,
  pub handle: *mut c_void,
}
impl Default for VkImportSemaphoreSciSyncInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV,
      next: core::ptr::null(),
      semaphore: Default::default(),
      handle_type: Default::default(),
      handle: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportSemaphoreWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreWin32HandleInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  pub flags: VkSemaphoreImportFlags,
  /// * No Auto-Validity
  pub handle_type: VkExternalSemaphoreHandleTypeFlagBits,
  /// * Optional: true
  pub handle: HANDLE,
  /// * Optional: true
  pub name: LPCWSTR,
}
impl Default for VkImportSemaphoreWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
      next: core::ptr::null(),
      semaphore: Default::default(),
      flags: Default::default(),
      handle_type: Default::default(),
      handle: core::ptr::null_mut(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportSemaphoreZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreZirconHandleInfoFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  pub flags: VkSemaphoreImportFlags,
  pub handle_type: VkExternalSemaphoreHandleTypeFlagBits,
  pub zircon_handle: zx_handle_t,
}
impl Default for VkImportSemaphoreZirconHandleInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA,
      next: core::ptr::null(),
      semaphore: Default::default(),
      flags: Default::default(),
      handle_type: Default::default(),
      zircon_handle: Default::default(),
    }
  }
}

/// Khronos: [VkIndirectCommandsLayoutCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkIndirectCommandsLayoutCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkIndirectCommandsLayoutUsageFlagsNV,
  pub pipeline_bind_point: VkPipelineBindPoint,
  pub token_count: u32,
  /// * Len: `token_count`
  pub tokens: *const VkIndirectCommandsLayoutTokenNV,
  pub stream_count: u32,
  /// * Len: `stream_count`
  pub stream_strides: *const u32,
}
impl Default for VkIndirectCommandsLayoutCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV,
      next: core::ptr::null(),
      flags: Default::default(),
      pipeline_bind_point: Default::default(),
      token_count: Default::default(),
      tokens: core::ptr::null(),
      stream_count: Default::default(),
      stream_strides: core::ptr::null(),
    }
  }
}

/// Khronos: [VkIndirectCommandsLayoutTokenNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkIndirectCommandsLayoutTokenNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub token_type: VkIndirectCommandsTokenTypeNV,
  pub stream: u32,
  pub offset: u32,
  pub vertex_binding_unit: u32,
  pub vertex_dynamic_stride: VkBool32,
  /// * Optional: true
  pub pushconstant_pipeline_layout: VkPipelineLayout,
  /// * Optional: true
  pub pushconstant_shader_stage_flags: VkShaderStageFlags,
  pub pushconstant_offset: u32,
  pub pushconstant_size: u32,
  /// * Optional: true
  pub indirect_state_flags: VkIndirectStateFlagsNV,
  /// * Optional: true
  pub index_type_count: u32,
  /// * Len: `index_type_count`
  pub index_types: *const VkIndexType,
  /// * Len: `index_type_count`
  pub index_type_values: *const u32,
}
impl Default for VkIndirectCommandsLayoutTokenNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV,
      next: core::ptr::null(),
      token_type: Default::default(),
      stream: Default::default(),
      offset: Default::default(),
      vertex_binding_unit: Default::default(),
      vertex_dynamic_stride: Default::default(),
      pushconstant_pipeline_layout: Default::default(),
      pushconstant_shader_stage_flags: Default::default(),
      pushconstant_offset: Default::default(),
      pushconstant_size: Default::default(),
      indirect_state_flags: Default::default(),
      index_type_count: Default::default(),
      index_types: core::ptr::null(),
      index_type_values: core::ptr::null(),
    }
  }
}

/// Khronos: [VkIndirectCommandsStreamNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsStreamNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkIndirectCommandsStreamNV {
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
}
impl Default for VkIndirectCommandsStreamNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { buffer: Default::default(), offset: Default::default() }
  }
}

/// Khronos: [VkInitializePerformanceApiInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkInitializePerformanceApiInfoINTEL {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub user_data: *mut c_void,
}
impl Default for VkInitializePerformanceApiInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL,
      next: core::ptr::null(),
      user_data: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkInputAttachmentAspectReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInputAttachmentAspectReference.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkInputAttachmentAspectReference {
  pub subpass: u32,
  pub input_attachment_index: u32,
  pub aspect_mask: VkImageAspectFlags,
}
impl Default for VkInputAttachmentAspectReference {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      subpass: Default::default(),
      input_attachment_index: Default::default(),
      aspect_mask: Default::default(),
    }
  }
}

/// Khronos: [VkInstanceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkInstanceCreateFlags,
  /// * Optional: true
  pub application_info: *const VkApplicationInfo,
  /// * Optional: true
  pub enabled_layer_count: u32,
  /// Ordered list of layer names to be enabled
  /// * Len: `enabled_layer_count,null_terminated`
  pub enabled_layer_names: *const *const u8,
  /// * Optional: true
  pub enabled_extension_count: u32,
  /// Extension names to be enabled
  /// * Len: `enabled_extension_count,null_terminated`
  pub enabled_extension_names: *const *const u8,
}
impl Default for VkInstanceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      application_info: core::ptr::null(),
      enabled_layer_count: Default::default(),
      enabled_layer_names: core::ptr::null(),
      enabled_extension_count: Default::default(),
      enabled_extension_names: core::ptr::null(),
    }
  }
}

/// Khronos: [VkLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLayerProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkLayerProperties {
  /// layer name
  pub layer_name: zstring::ArrayZString<VK_MAX_EXTENSION_NAME_SIZE>,
  /// version of the layer specification implemented
  pub spec_version: VkVersion,
  /// build or release version of the layer's library
  pub implementation_version: u32,
  /// Free-form description of the layer
  pub description: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
}
impl Default for VkLayerProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      layer_name: Default::default(),
      spec_version: Default::default(),
      implementation_version: Default::default(),
      description: Default::default(),
    }
  }
}

/// Khronos: [VkMacOSSurfaceCreateInfoMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMacOSSurfaceCreateInfoMVK {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkMacOSSurfaceCreateFlagsMVK,
  /// * No Auto-Validity
  pub view: *const c_void,
}
impl Default for VkMacOSSurfaceCreateInfoMVK {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK,
      next: core::ptr::null(),
      flags: Default::default(),
      view: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMappedMemoryRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMappedMemoryRange.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMappedMemoryRange {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Mapped memory object
  pub memory: VkDeviceMemory,
  /// Offset within the memory object where the range starts
  pub offset: VkDeviceSize,
  /// Size of the range within the memory object
  pub size: VkDeviceSize,
}
impl Default for VkMappedMemoryRange {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
      next: core::ptr::null(),
      memory: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryAllocateFlagsInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsInfo.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryAllocateFlagsInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkMemoryAllocateFlags,
  pub device_mask: u32,
}
impl Default for VkMemoryAllocateFlagsInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      device_mask: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Size of memory allocation
  pub allocation_size: VkDeviceSize,
  /// Index of the of the memory type to allocate from
  pub memory_type_index: u32,
}
impl Default for VkMemoryAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
      next: core::ptr::null(),
      allocation_size: Default::default(),
      memory_type_index: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryBarrier {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_BARRIER`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * Optional: true
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * Optional: true
  pub dst_access_mask: VkAccessFlags,
}
impl Default for VkMemoryBarrier {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_BARRIER,
      next: core::ptr::null(),
      src_access_mask: Default::default(),
      dst_access_mask: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryBarrier2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier2.html) (structure)
///
/// * Struct Extends: [`VkSubpassDependency2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryBarrier2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_BARRIER_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub src_stage_mask: VkPipelineStageFlags2,
  /// * Optional: true
  pub src_access_mask: VkAccessFlags2,
  /// * Optional: true
  pub dst_stage_mask: VkPipelineStageFlags2,
  /// * Optional: true
  pub dst_access_mask: VkAccessFlags2,
}
impl Default for VkMemoryBarrier2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_BARRIER_2,
      next: core::ptr::null(),
      src_stage_mask: Default::default(),
      src_access_mask: Default::default(),
      dst_stage_mask: Default::default(),
      dst_access_mask: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryDedicatedAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedAllocateInfo.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Image that this allocation will be bound to
  /// * Optional: true
  pub image: VkImage,
  /// Buffer that this allocation will be bound to
  /// * Optional: true
  pub buffer: VkBuffer,
}
impl Default for VkMemoryDedicatedAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO,
      next: core::ptr::null(),
      image: Default::default(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryDedicatedRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedRequirements.html) (structure)
///
/// * Struct Extends: [`VkMemoryRequirements2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryDedicatedRequirements {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub prefers_dedicated_allocation: VkBool32,
  pub requires_dedicated_allocation: VkBool32,
}
impl Default for VkMemoryDedicatedRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS,
      next: core::ptr::null_mut(),
      prefers_dedicated_allocation: Default::default(),
      requires_dedicated_allocation: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryFdPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryFdPropertiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryFdPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_type_bits: u32,
}
impl Default for VkMemoryFdPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      memory_type_bits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetAndroidHardwareBufferInfoANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory: VkDeviceMemory,
}
impl Default for VkMemoryGetAndroidHardwareBufferInfoANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
      next: core::ptr::null(),
      memory: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetFdInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryGetFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory: VkDeviceMemory,
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR,
      next: core::ptr::null(),
      memory: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetRemoteAddressInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetRemoteAddressInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryGetRemoteAddressInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory: VkDeviceMemory,
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetRemoteAddressInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV,
      next: core::ptr::null(),
      memory: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetSciBufInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetSciBufInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryGetSciBufInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_SCI_BUF_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory: VkDeviceMemory,
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetSciBufInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_GET_SCI_BUF_INFO_NV,
      next: core::ptr::null(),
      memory: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetWin32HandleInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryGetWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory: VkDeviceMemory,
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR,
      next: core::ptr::null(),
      memory: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetZirconHandleInfoFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryGetZirconHandleInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory: VkDeviceMemory,
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetZirconHandleInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
      next: core::ptr::null(),
      memory: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryHeap](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeap.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryHeap {
  /// Available memory in the heap
  pub size: VkDeviceSize,
  /// Flags for the heap
  /// * Optional: true
  pub flags: VkMemoryHeapFlags,
}
impl Default for VkMemoryHeap {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { size: Default::default(), flags: Default::default() }
  }
}

/// Khronos: [VkMemoryHostPointerPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryHostPointerPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_type_bits: u32,
}
impl Default for VkMemoryHostPointerPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      memory_type_bits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryOpaqueCaptureAddressAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfo.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub opaque_capture_address: u64,
}
impl Default for VkMemoryOpaqueCaptureAddressAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO,
      next: core::ptr::null(),
      opaque_capture_address: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryPriorityAllocateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryPriorityAllocateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub priority: c_float,
}
impl Default for VkMemoryPriorityAllocateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT,
      next: core::ptr::null(),
      priority: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryRequirements {
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Specified in bytes
  pub alignment: VkDeviceSize,
  /// Bitmask of the allowed memory type indices into memoryTypes\[\] for this
  /// object
  pub memory_type_bits: u32,
}
impl Default for VkMemoryRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      size: Default::default(),
      alignment: Default::default(),
      memory_type_bits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements2.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryRequirements2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_requirements: VkMemoryRequirements,
}
impl Default for VkMemoryRequirements2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2,
      next: core::ptr::null_mut(),
      memory_requirements: Default::default(),
    }
  }
}

/// Khronos: [VkMemorySciBufPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemorySciBufPropertiesNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemorySciBufPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_SCI_BUF_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub memory_type_bits: u32,
}
impl Default for VkMemorySciBufPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_SCI_BUF_PROPERTIES_NV,
      next: core::ptr::null(),
      memory_type_bits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryType.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryType {
  /// Memory properties of this memory type
  /// * Optional: true
  pub property_flags: VkMemoryPropertyFlags,
  /// Index of the memory heap allocations of this memory type are taken from
  pub heap_index: u32,
}
impl Default for VkMemoryType {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { property_flags: Default::default(), heap_index: Default::default() }
  }
}

/// Khronos: [VkMemoryWin32HandlePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryWin32HandlePropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_type_bits: u32,
}
impl Default for VkMemoryWin32HandlePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      memory_type_bits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryZirconHandlePropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryZirconHandlePropertiesFUCHSIA.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMemoryZirconHandlePropertiesFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_type_bits: u32,
}
impl Default for VkMemoryZirconHandlePropertiesFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA,
      next: core::ptr::null_mut(),
      memory_type_bits: Default::default(),
    }
  }
}

/// Khronos: [VkMetalSurfaceCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMetalSurfaceCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkMetalSurfaceCreateFlagsEXT,
  /// * No Auto-Validity
  pub layer: *const CAMetalLayer,
}
impl Default for VkMetalSurfaceCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      layer: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMicromapBuildSizesInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapBuildSizesInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMicromapBuildSizesInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MICROMAP_BUILD_SIZES_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub micromap_size: VkDeviceSize,
  pub build_scratch_size: VkDeviceSize,
  pub discardable: VkBool32,
}
impl Default for VkMicromapBuildSizesInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MICROMAP_BUILD_SIZES_INFO_EXT,
      next: core::ptr::null(),
      micromap_size: Default::default(),
      build_scratch_size: Default::default(),
      discardable: Default::default(),
    }
  }
}

/// Khronos: [VkMicromapCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapCreateInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMicromapCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MICROMAP_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub create_flags: VkMicromapCreateFlagsEXT,
  pub buffer: VkBuffer,
  /// Specified in bytes
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  pub ty: VkMicromapTypeEXT,
  /// * Optional: true
  pub device_address: VkDeviceAddress,
}
impl Default for VkMicromapCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MICROMAP_CREATE_INFO_EXT,
      next: core::ptr::null(),
      create_flags: Default::default(),
      buffer: Default::default(),
      offset: Default::default(),
      size: Default::default(),
      ty: Default::default(),
      device_address: Default::default(),
    }
  }
}

/// Khronos: [VkMicromapTriangleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapTriangleEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMicromapTriangleEXT {
  /// Specified in bytes
  pub data_offset: u32,
  pub subdivision_level: u16,
  pub format: u16,
}
impl Default for VkMicromapTriangleEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      data_offset: Default::default(),
      subdivision_level: Default::default(),
      format: Default::default(),
    }
  }
}

/// Khronos: [VkMicromapUsageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapUsageEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMicromapUsageEXT {
  pub count: u32,
  pub subdivision_level: u32,
  /// Interpretation depends on parent type
  pub format: u32,
}
impl Default for VkMicromapUsageEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      count: Default::default(),
      subdivision_level: Default::default(),
      format: Default::default(),
    }
  }
}

/// Khronos: [VkMicromapVersionInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapVersionInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMicromapVersionInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MICROMAP_VERSION_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Len: `2*vk_uuid_size`
  pub version_data: *const u8,
}
impl Default for VkMicromapVersionInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MICROMAP_VERSION_INFO_EXT,
      next: core::ptr::null(),
      version_data: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMultiDrawIndexedInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiDrawIndexedInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMultiDrawIndexedInfoEXT {
  pub first_index: u32,
  pub index_count: u32,
  pub vertex_offset: i32,
}
impl Default for VkMultiDrawIndexedInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      first_index: Default::default(),
      index_count: Default::default(),
      vertex_offset: Default::default(),
    }
  }
}

/// Khronos: [VkMultiDrawInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiDrawInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMultiDrawInfoEXT {
  pub first_vertex: u32,
  pub vertex_count: u32,
}
impl Default for VkMultiDrawInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { first_vertex: Default::default(), vertex_count: Default::default() }
  }
}

/// Khronos: [VkMultisamplePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultisamplePropertiesEXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMultisamplePropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub max_sample_location_grid_size: VkExtent2D,
}
impl Default for VkMultisamplePropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      max_sample_location_grid_size: Default::default(),
    }
  }
}

/// Khronos: [VkMultisampledRenderToSingleSampledInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultisampledRenderToSingleSampledInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkSubpassDescription2`]
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMultisampledRenderToSingleSampledInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub multisampled_render_to_single_sampled_enable: VkBool32,
  pub rasterization_samples: VkSampleCountFlagBits,
}
impl Default for VkMultisampledRenderToSingleSampledInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT,
      next: core::ptr::null(),
      multisampled_render_to_single_sampled_enable: Default::default(),
      rasterization_samples: Default::default(),
    }
  }
}

/// Khronos: [VkMultiviewPerViewAttributesInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiviewPerViewAttributesInfoNVX.html) (structure)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMultiviewPerViewAttributesInfoNVX {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub per_view_attributes: VkBool32,
  pub per_view_attributes_position_x_only: VkBool32,
}
impl Default for VkMultiviewPerViewAttributesInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX,
      next: core::ptr::null(),
      per_view_attributes: Default::default(),
      per_view_attributes_position_x_only: Default::default(),
    }
  }
}

/// Khronos: [VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM.html) (structure)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub per_view_render_area_count: u32,
  /// * Len: `per_view_render_area_count`
  pub per_view_render_areas: *const VkRect2D,
}
impl Default for VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM,
      next: core::ptr::null(),
      per_view_render_area_count: Default::default(),
      per_view_render_areas: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMutableDescriptorTypeCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkDescriptorSetLayoutCreateInfo`]
/// * Struct Extends: [`VkDescriptorPoolCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMutableDescriptorTypeCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub mutable_descriptor_type_list_count: u32,
  /// * Len: `mutable_descriptor_type_list_count`
  pub mutable_descriptor_type_lists: *const VkMutableDescriptorTypeListEXT,
}
impl Default for VkMutableDescriptorTypeCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      mutable_descriptor_type_list_count: Default::default(),
      mutable_descriptor_type_lists: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMutableDescriptorTypeListEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeListEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkMutableDescriptorTypeListEXT {
  /// * Optional: true
  pub descriptor_type_count: u32,
  /// * Len: `descriptor_type_count`
  pub descriptor_types: *const VkDescriptorType,
}
impl Default for VkMutableDescriptorTypeListEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      descriptor_type_count: Default::default(),
      descriptor_types: core::ptr::null(),
    }
  }
}

/// Khronos: [VkNativeBufferANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkNativeBufferANDROID.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkNativeBufferANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_NATIVE_BUFFER_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub handle: *const c_void,
  pub stride: c_int,
  pub format: c_int,
  pub usage: c_int,
  pub usage_2: VkNativeBufferUsage2ANDROID,
}
impl Default for VkNativeBufferANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_NATIVE_BUFFER_ANDROID,
      next: core::ptr::null(),
      handle: core::ptr::null(),
      stride: Default::default(),
      format: Default::default(),
      usage: Default::default(),
      usage_2: Default::default(),
    }
  }
}

/// Khronos: [VkNativeBufferUsage2ANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkNativeBufferUsage2ANDROID.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkNativeBufferUsage2ANDROID {
  pub consumer: u64,
  pub producer: u64,
}
impl Default for VkNativeBufferUsage2ANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { consumer: Default::default(), producer: Default::default() }
  }
}

/// Khronos: [VkOffset2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOffset2D.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkOffset2D {
  pub x: i32,
  pub y: i32,
}
impl Default for VkOffset2D {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { x: Default::default(), y: Default::default() }
  }
}

/// Khronos: [VkOffset3D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOffset3D.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkOffset3D {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}
impl Default for VkOffset3D {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { x: Default::default(), y: Default::default(), z: Default::default() }
  }
}

/// Khronos: [VkOpaqueCaptureDescriptorDataCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpaqueCaptureDescriptorDataCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkImageViewCreateInfo`]
/// * Struct Extends: [`VkSamplerCreateInfo`]
/// * Struct Extends: [`VkAccelerationStructureCreateInfoKHR`]
/// * Struct Extends: [`VkAccelerationStructureCreateInfoNV`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkOpaqueCaptureDescriptorDataCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub opaque_capture_descriptor_data: *const c_void,
}
impl Default for VkOpaqueCaptureDescriptorDataCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT,
      next: core::ptr::null(),
      opaque_capture_descriptor_data: core::ptr::null(),
    }
  }
}

/// Khronos: [VkOpticalFlowExecuteInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkOpticalFlowExecuteInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_OPTICAL_FLOW_EXECUTE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  pub flags: VkOpticalFlowExecuteFlagsNV,
  /// * Optional: true
  pub region_count: u32,
  /// * Len: `region_count`
  pub regions: *const VkRect2D,
}
impl Default for VkOpticalFlowExecuteInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_OPTICAL_FLOW_EXECUTE_INFO_NV,
      next: core::ptr::null_mut(),
      flags: Default::default(),
      region_count: Default::default(),
      regions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkOpticalFlowImageFormatInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowImageFormatInfoNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkOpticalFlowImageFormatInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub usage: VkOpticalFlowUsageFlagsNV,
}
impl Default for VkOpticalFlowImageFormatInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV,
      next: core::ptr::null(),
      usage: Default::default(),
    }
  }
}

/// Khronos: [VkOpticalFlowImageFormatPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowImageFormatPropertiesNV.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkOpticalFlowImageFormatPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub format: VkFormat,
}
impl Default for VkOpticalFlowImageFormatPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV,
      next: core::ptr::null(),
      format: Default::default(),
    }
  }
}

/// Khronos: [VkOpticalFlowSessionCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkOpticalFlowSessionCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub width: u32,
  pub height: u32,
  pub image_format: VkFormat,
  pub flow_vector_format: VkFormat,
  /// * Optional: true
  pub cost_format: VkFormat,
  pub output_grid_size: VkOpticalFlowGridSizeFlagsNV,
  /// * Optional: true
  pub hint_grid_size: VkOpticalFlowGridSizeFlagsNV,
  /// * Optional: true
  pub performance_level: VkOpticalFlowPerformanceLevelNV,
  /// * Optional: true
  pub flags: VkOpticalFlowSessionCreateFlagsNV,
}
impl Default for VkOpticalFlowSessionCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_INFO_NV,
      next: core::ptr::null_mut(),
      width: Default::default(),
      height: Default::default(),
      image_format: Default::default(),
      flow_vector_format: Default::default(),
      cost_format: Default::default(),
      output_grid_size: Default::default(),
      hint_grid_size: Default::default(),
      performance_level: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkOpticalFlowSessionCreatePrivateDataInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreatePrivateDataInfoNV.html) (structure)
///
/// NV internal use only
/// * Struct Extends: [`VkOpticalFlowSessionCreateInfoNV`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkOpticalFlowSessionCreatePrivateDataInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub id: u32,
  pub size: u32,
  pub private_data: *const c_void,
}
impl Default for VkOpticalFlowSessionCreatePrivateDataInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV,
      next: core::ptr::null_mut(),
      id: Default::default(),
      size: Default::default(),
      private_data: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPastPresentationTimingGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPastPresentationTimingGOOGLE.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPastPresentationTimingGOOGLE {
  /// Application-provided identifier, previously given to vkQueuePresentKHR
  pub present_id: u32,
  /// Earliest time an image should have been presented, previously given to
  /// vkQueuePresentKHR
  pub desired_present_time: u64,
  /// Time the image was actually displayed
  pub actual_present_time: u64,
  /// Earliest time the image could have been displayed
  pub earliest_present_time: u64,
  /// How early vkQueuePresentKHR was processed vs. how soon it needed to be and
  /// make earliestPresentTime
  pub present_margin: u64,
}
impl Default for VkPastPresentationTimingGOOGLE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      present_id: Default::default(),
      desired_present_time: Default::default(),
      actual_present_time: Default::default(),
      earliest_present_time: Default::default(),
      present_margin: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceConfigurationAcquireInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPerformanceConfigurationAcquireInfoINTEL {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub ty: VkPerformanceConfigurationTypeINTEL,
}
impl Default for VkPerformanceConfigurationAcquireInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL,
      next: core::ptr::null(),
      ty: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceCounterDescriptionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPerformanceCounterDescriptionKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  pub flags: VkPerformanceCounterDescriptionFlagsKHR,
  pub name: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  pub category: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  pub description: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
}
impl Default for VkPerformanceCounterDescriptionKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR,
      next: core::ptr::null_mut(),
      flags: Default::default(),
      name: Default::default(),
      category: Default::default(),
      description: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceCounterKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPerformanceCounterKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub unit: VkPerformanceCounterUnitKHR,
  pub scope: VkPerformanceCounterScopeKHR,
  pub storage: VkPerformanceCounterStorageKHR,
  pub uuid: [u8; VK_UUID_SIZE],
}
impl Default for VkPerformanceCounterKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR,
      next: core::ptr::null_mut(),
      unit: Default::default(),
      scope: Default::default(),
      storage: Default::default(),
      uuid: [Default::default(); VK_UUID_SIZE],
    }
  }
}

/// Khronos: [VkPerformanceMarkerInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceMarkerInfoINTEL.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPerformanceMarkerInfoINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub marker: u64,
}
impl Default for VkPerformanceMarkerInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL,
      next: core::ptr::null(),
      marker: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceOverrideInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideInfoINTEL.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPerformanceOverrideInfoINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub ty: VkPerformanceOverrideTypeINTEL,
  pub enable: VkBool32,
  pub parameter: u64,
}
impl Default for VkPerformanceOverrideInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL,
      next: core::ptr::null(),
      ty: Default::default(),
      enable: Default::default(),
      parameter: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceQuerySubmitInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkSubmitInfo`]
/// * Struct Extends: [`VkSubmitInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPerformanceQuerySubmitInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Index for which counter pass to submit
  pub counter_pass_index: u32,
}
impl Default for VkPerformanceQuerySubmitInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR,
      next: core::ptr::null(),
      counter_pass_index: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceStreamMarkerInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPerformanceStreamMarkerInfoINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub marker: u32,
}
impl Default for VkPerformanceStreamMarkerInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL,
      next: core::ptr::null(),
      marker: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevice16BitStorageFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevice16BitStorageFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// 16-bit integer/floating-point variables supported in BufferBlock
  pub storage_buffer_16_bit_access: VkBool32,
  /// 16-bit integer/floating-point variables supported in BufferBlock and Block
  pub uniform_and_storage_buffer_16_bit_access: VkBool32,
  /// 16-bit integer/floating-point variables supported in PushConstant
  pub storage_push_constant_16: VkBool32,
  /// 16-bit integer/floating-point variables supported in shader inputs and
  /// outputs
  pub storage_input_output_16: VkBool32,
}
impl Default for VkPhysicalDevice16BitStorageFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
      next: core::ptr::null_mut(),
      storage_buffer_16_bit_access: Default::default(),
      uniform_and_storage_buffer_16_bit_access: Default::default(),
      storage_push_constant_16: Default::default(),
      storage_input_output_16: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevice4444FormatsFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevice4444FormatsFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub format_a_4_r_4_g_4_b_4: VkBool32,
  pub format_a_4_b_4_g_4_r_4: VkBool32,
}
impl Default for VkPhysicalDevice4444FormatsFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT,
      next: core::ptr::null_mut(),
      format_a_4_r_4_g_4_b_4: Default::default(),
      format_a_4_b_4_g_4_r_4: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevice8BitStorageFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice8BitStorageFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevice8BitStorageFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// 8-bit integer variables supported in StorageBuffer
  pub storage_buffer_8_bit_access: VkBool32,
  /// 8-bit integer variables supported in StorageBuffer and Uniform
  pub uniform_and_storage_buffer_8_bit_access: VkBool32,
  /// 8-bit integer variables supported in PushConstant
  pub storage_push_constant_8: VkBool32,
}
impl Default for VkPhysicalDevice8BitStorageFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES,
      next: core::ptr::null_mut(),
      storage_buffer_8_bit_access: Default::default(),
      uniform_and_storage_buffer_8_bit_access: Default::default(),
      storage_push_constant_8: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceASTCDecodeFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceASTCDecodeFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub decode_mode_shared_exponent: VkBool32,
}
impl Default for VkPhysicalDeviceASTCDecodeFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT,
      next: core::ptr::null_mut(),
      decode_mode_shared_exponent: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAccelerationStructureFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub acceleration_structure: VkBool32,
  pub acceleration_structure_capture_replay: VkBool32,
  pub acceleration_structure_indirect_build: VkBool32,
  pub acceleration_structure_host_commands: VkBool32,
  pub descriptor_binding_acceleration_structure_update_after_bind: VkBool32,
}
impl Default for VkPhysicalDeviceAccelerationStructureFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR,
      next: core::ptr::null_mut(),
      acceleration_structure: Default::default(),
      acceleration_structure_capture_replay: Default::default(),
      acceleration_structure_indirect_build: Default::default(),
      acceleration_structure_host_commands: Default::default(),
      descriptor_binding_acceleration_structure_update_after_bind: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAccelerationStructurePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_geometry_count: u64,
  /// * Limit Type: max
  pub max_instance_count: u64,
  /// * Limit Type: max
  pub max_primitive_count: u64,
  /// * Limit Type: max
  pub max_per_stage_descriptor_acceleration_structures: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
  /// * Limit Type: max
  pub max_descriptor_set_acceleration_structures: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_acceleration_structures: u32,
  /// * Limit Type: min
  pub min_acceleration_structure_scratch_offset_alignment: u32,
}
impl Default for VkPhysicalDeviceAccelerationStructurePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      max_geometry_count: Default::default(),
      max_instance_count: Default::default(),
      max_primitive_count: Default::default(),
      max_per_stage_descriptor_acceleration_structures: Default::default(),
      max_per_stage_descriptor_update_after_bind_acceleration_structures:
        Default::default(),
      max_descriptor_set_acceleration_structures: Default::default(),
      max_descriptor_set_update_after_bind_acceleration_structures: Default::default(),
      min_acceleration_structure_scratch_offset_alignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAddressBindingReportFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAddressBindingReportFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceAddressBindingReportFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub report_address_binding: VkBool32,
}
impl Default for VkPhysicalDeviceAddressBindingReportFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT,
      next: core::ptr::null_mut(),
      report_address_binding: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAmigoProfilingFeaturesSEC](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAmigoProfilingFeaturesSEC.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceAmigoProfilingFeaturesSEC {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub amigo_profiling: VkBool32,
}
impl Default for VkPhysicalDeviceAmigoProfilingFeaturesSEC {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC,
      next: core::ptr::null_mut(),
      amigo_profiling: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub attachment_feedback_loop_layout: VkBool32,
}
impl Default for VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT,
      next: core::ptr::null_mut(),
      attachment_feedback_loop_layout: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub advanced_blend_coherent_operations: VkBool32,
}
impl Default for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
      next: core::ptr::null_mut(),
      advanced_blend_coherent_operations: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub advanced_blend_max_color_attachments: u32,
  /// * Limit Type: bitmask
  pub advanced_blend_independent_blend: VkBool32,
  /// * Limit Type: bitmask
  pub advanced_blend_non_premultiplied_src_color: VkBool32,
  /// * Limit Type: bitmask
  pub advanced_blend_non_premultiplied_dst_color: VkBool32,
  /// * Limit Type: bitmask
  pub advanced_blend_correlated_overlap: VkBool32,
  /// * Limit Type: bitmask
  pub advanced_blend_all_operations: VkBool32,
}
impl Default for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      advanced_blend_max_color_attachments: Default::default(),
      advanced_blend_independent_blend: Default::default(),
      advanced_blend_non_premultiplied_src_color: Default::default(),
      advanced_blend_non_premultiplied_dst_color: Default::default(),
      advanced_blend_correlated_overlap: Default::default(),
      advanced_blend_all_operations: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBorderColorSwizzleFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub border_color_swizzle: VkBool32,
  pub border_color_swizzle_from_image: VkBool32,
}
impl Default for VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT,
      next: core::ptr::null_mut(),
      border_color_swizzle: Default::default(),
      border_color_swizzle_from_image: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBufferDeviceAddressFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub buffer_device_address: VkBool32,
  pub buffer_device_address_capture_replay: VkBool32,
  pub buffer_device_address_multi_device: VkBool32,
}
impl Default for VkPhysicalDeviceBufferDeviceAddressFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES,
      next: core::ptr::null_mut(),
      buffer_device_address: Default::default(),
      buffer_device_address_capture_replay: Default::default(),
      buffer_device_address_multi_device: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBufferDeviceAddressFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub buffer_device_address: VkBool32,
  pub buffer_device_address_capture_replay: VkBool32,
  pub buffer_device_address_multi_device: VkBool32,
}
impl Default for VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT,
      next: core::ptr::null_mut(),
      buffer_device_address: Default::default(),
      buffer_device_address_capture_replay: Default::default(),
      buffer_device_address_multi_device: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub clusterculling_shader: VkBool32,
  pub multiview_cluster_culling_shader: VkBool32,
}
impl Default for VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI,
      next: core::ptr::null_mut(),
      clusterculling_shader: Default::default(),
      multiview_cluster_culling_shader: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max,pot
  pub max_work_group_count: [u32; 3],
  /// * Limit Type: max,pot
  pub max_work_group_size: [u32; 3],
  /// * Limit Type: max
  pub max_output_cluster_count: u32,
  /// * Limit Type: exact
  pub indirect_buffer_offset_alignment: VkDeviceSize,
}
impl Default for VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI,
      next: core::ptr::null_mut(),
      max_work_group_count: [Default::default(); 3],
      max_work_group_size: [Default::default(); 3],
      max_output_cluster_count: Default::default(),
      indirect_buffer_offset_alignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCoherentMemoryFeaturesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub device_coherent_memory: VkBool32,
}
impl Default for VkPhysicalDeviceCoherentMemoryFeaturesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
      next: core::ptr::null_mut(),
      device_coherent_memory: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceColorWriteEnableFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub color_write_enable: VkBool32,
}
impl Default for VkPhysicalDeviceColorWriteEnableFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT,
      next: core::ptr::null_mut(),
      color_write_enable: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceComputeShaderDerivativesFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub compute_derivative_group_quads: VkBool32,
  pub compute_derivative_group_linear: VkBool32,
}
impl Default for VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV,
      next: core::ptr::null_mut(),
      compute_derivative_group_quads: Default::default(),
      compute_derivative_group_linear: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceConditionalRenderingFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub conditional_rendering: VkBool32,
  pub inherited_conditional_rendering: VkBool32,
}
impl Default for VkPhysicalDeviceConditionalRenderingFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT,
      next: core::ptr::null_mut(),
      conditional_rendering: Default::default(),
      inherited_conditional_rendering: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceConservativeRasterizationPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// The size in pixels the primitive is enlarged at each edge during
  /// conservative rasterization
  /// * Limit Type: exact
  pub primitive_overestimation_size: c_float,
  /// The maximum additional overestimation the client can specify in the
  /// pipeline state
  /// * Limit Type: max
  pub max_extra_primitive_overestimation_size: c_float,
  /// The granularity of extra overestimation sizes the implementations supports
  /// between 0 and maxExtraOverestimationSize
  /// * Limit Type: min,mul
  pub extra_primitive_overestimation_size_granularity: c_float,
  /// true if the implementation supports conservative rasterization
  /// underestimation mode
  /// * Limit Type: bitmask
  pub primitive_underestimation: VkBool32,
  /// true if conservative rasterization also applies to points and lines
  /// * Limit Type: bitmask
  pub conservative_point_and_line_rasterization: VkBool32,
  /// true if degenerate triangles (those with zero area after snap) are
  /// rasterized
  /// * Limit Type: exact
  pub degenerate_triangles_rasterized: VkBool32,
  /// true if degenerate lines (those with zero length after snap) are
  /// rasterized
  /// * Limit Type: exact
  pub degenerate_lines_rasterized: VkBool32,
  /// true if the implementation supports the FullyCoveredEXT SPIR-V builtin
  /// fragment shader input variable
  /// * Limit Type: bitmask
  pub fully_covered_fragment_shader_input_variable: VkBool32,
  /// true if the implementation supports both conservative rasterization and
  /// post depth coverage sample coverage mask
  /// * Limit Type: bitmask
  pub conservative_rasterization_post_depth_coverage: VkBool32,
}
impl Default for VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      primitive_overestimation_size: Default::default(),
      max_extra_primitive_overestimation_size: Default::default(),
      extra_primitive_overestimation_size_granularity: Default::default(),
      primitive_underestimation: Default::default(),
      conservative_point_and_line_rasterization: Default::default(),
      degenerate_triangles_rasterized: Default::default(),
      degenerate_lines_rasterized: Default::default(),
      fully_covered_fragment_shader_input_variable: Default::default(),
      conservative_rasterization_post_depth_coverage: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCooperativeMatrixFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub cooperative_matrix: VkBool32,
  pub cooperative_matrix_robust_buffer_access: VkBool32,
}
impl Default for VkPhysicalDeviceCooperativeMatrixFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV,
      next: core::ptr::null_mut(),
      cooperative_matrix: Default::default(),
      cooperative_matrix_robust_buffer_access: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCooperativeMatrixPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub cooperative_matrix_supported_stages: VkShaderStageFlags,
}
impl Default for VkPhysicalDeviceCooperativeMatrixPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      cooperative_matrix_supported_stages: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCopyMemoryIndirectFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCopyMemoryIndirectFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub indirect_copy: VkBool32,
}
impl Default for VkPhysicalDeviceCopyMemoryIndirectFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV,
      next: core::ptr::null_mut(),
      indirect_copy: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCopyMemoryIndirectPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCopyMemoryIndirectPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// Bitfield of which queues are supported for indirect copy
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub supported_queues: VkQueueFlags,
}
impl Default for VkPhysicalDeviceCopyMemoryIndirectPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      supported_queues: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCornerSampledImageFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub corner_sampled_image: VkBool32,
}
impl Default for VkPhysicalDeviceCornerSampledImageFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV,
      next: core::ptr::null_mut(),
      corner_sampled_image: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCoverageReductionModeFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub coverage_reduction_mode: VkBool32,
}
impl Default for VkPhysicalDeviceCoverageReductionModeFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV,
      next: core::ptr::null_mut(),
      coverage_reduction_mode: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCustomBorderColorFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub custom_border_colors: VkBool32,
  pub custom_border_color_without_format: VkBool32,
}
impl Default for VkPhysicalDeviceCustomBorderColorFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT,
      next: core::ptr::null_mut(),
      custom_border_colors: Default::default(),
      custom_border_color_without_format: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCustomBorderColorPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_custom_border_color_samplers: u32,
}
impl Default for VkPhysicalDeviceCustomBorderColorPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      max_custom_border_color_samplers: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub dedicated_allocation_image_aliasing: VkBool32,
}
impl Default for VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV,
      next: core::ptr::null_mut(),
      dedicated_allocation_image_aliasing: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDepthClampZeroOneFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClampZeroOneFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClampZeroOneFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub depth_clamp_zero_one: VkBool32,
}
impl Default for VkPhysicalDeviceDepthClampZeroOneFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT,
      next: core::ptr::null_mut(),
      depth_clamp_zero_one: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDepthClipControlFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipControlFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClipControlFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub depth_clip_control: VkBool32,
}
impl Default for VkPhysicalDeviceDepthClipControlFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT,
      next: core::ptr::null_mut(),
      depth_clip_control: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDepthClipEnableFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub depth_clip_enable: VkBool32,
}
impl Default for VkPhysicalDeviceDepthClipEnableFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT,
      next: core::ptr::null_mut(),
      depth_clip_enable: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDepthStencilResolveProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthStencilResolveProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// supported depth resolve modes
  /// * Limit Type: bitmask
  pub supported_depth_resolve_modes: VkResolveModeFlags,
  /// supported stencil resolve modes
  /// * Limit Type: bitmask
  pub supported_stencil_resolve_modes: VkResolveModeFlags,
  /// depth and stencil resolve modes can be set independently if one of them is
  /// none
  /// * Limit Type: bitmask
  pub independent_resolve_none: VkBool32,
  /// depth and stencil resolve modes can be set independently
  /// * Limit Type: bitmask
  pub independent_resolve: VkBool32,
}
impl Default for VkPhysicalDeviceDepthStencilResolveProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES,
      next: core::ptr::null_mut(),
      supported_depth_resolve_modes: Default::default(),
      supported_stencil_resolve_modes: Default::default(),
      independent_resolve_none: Default::default(),
      independent_resolve: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub combined_image_sampler_density_map_descriptor_size: c_size_t,
}
impl Default for VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      combined_image_sampler_density_map_descriptor_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorBufferFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub descriptor_buffer: VkBool32,
  pub descriptor_buffer_capture_replay: VkBool32,
  pub descriptor_buffer_image_layout_ignored: VkBool32,
  pub descriptor_buffer_push_descriptors: VkBool32,
}
impl Default for VkPhysicalDeviceDescriptorBufferFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT,
      next: core::ptr::null_mut(),
      descriptor_buffer: Default::default(),
      descriptor_buffer_capture_replay: Default::default(),
      descriptor_buffer_image_layout_ignored: Default::default(),
      descriptor_buffer_push_descriptors: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorBufferPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: noauto
  pub combined_image_sampler_descriptor_single_array: VkBool32,
  /// * Limit Type: noauto
  pub bufferless_push_descriptors: VkBool32,
  /// * Limit Type: noauto
  pub allow_sampler_image_view_post_submit_creation: VkBool32,
  /// * Limit Type: noauto
  pub descriptor_buffer_offset_alignment: VkDeviceSize,
  /// * Limit Type: max
  pub max_descriptor_buffer_bindings: u32,
  /// * Limit Type: max
  pub max_resource_descriptor_buffer_bindings: u32,
  /// * Limit Type: max
  pub max_sampler_descriptor_buffer_bindings: u32,
  /// * Limit Type: max
  pub max_embedded_immutable_sampler_bindings: u32,
  /// * Limit Type: max
  pub max_embedded_immutable_samplers: u32,
  /// * Limit Type: noauto
  pub buffer_capture_replay_descriptor_data_size: c_size_t,
  /// * Limit Type: noauto
  pub image_capture_replay_descriptor_data_size: c_size_t,
  /// * Limit Type: noauto
  pub image_view_capture_replay_descriptor_data_size: c_size_t,
  /// * Limit Type: noauto
  pub sampler_capture_replay_descriptor_data_size: c_size_t,
  /// * Limit Type: noauto
  pub acceleration_structure_capture_replay_descriptor_data_size: c_size_t,
  /// * Limit Type: max
  pub sampler_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub combined_image_sampler_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub sampled_image_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub storage_image_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub uniform_texel_buffer_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub robust_uniform_texel_buffer_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub storage_texel_buffer_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub robust_storage_texel_buffer_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub uniform_buffer_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub robust_uniform_buffer_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub storage_buffer_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub robust_storage_buffer_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub input_attachment_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub acceleration_structure_descriptor_size: c_size_t,
  /// * Limit Type: max
  pub max_sampler_descriptor_buffer_range: VkDeviceSize,
  /// * Limit Type: max
  pub max_resource_descriptor_buffer_range: VkDeviceSize,
  /// * Limit Type: max
  pub sampler_descriptor_buffer_address_space_size: VkDeviceSize,
  /// * Limit Type: max
  pub resource_descriptor_buffer_address_space_size: VkDeviceSize,
  /// * Limit Type: max
  pub descriptor_buffer_address_space_size: VkDeviceSize,
}
impl Default for VkPhysicalDeviceDescriptorBufferPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      combined_image_sampler_descriptor_single_array: Default::default(),
      bufferless_push_descriptors: Default::default(),
      allow_sampler_image_view_post_submit_creation: Default::default(),
      descriptor_buffer_offset_alignment: Default::default(),
      max_descriptor_buffer_bindings: Default::default(),
      max_resource_descriptor_buffer_bindings: Default::default(),
      max_sampler_descriptor_buffer_bindings: Default::default(),
      max_embedded_immutable_sampler_bindings: Default::default(),
      max_embedded_immutable_samplers: Default::default(),
      buffer_capture_replay_descriptor_data_size: Default::default(),
      image_capture_replay_descriptor_data_size: Default::default(),
      image_view_capture_replay_descriptor_data_size: Default::default(),
      sampler_capture_replay_descriptor_data_size: Default::default(),
      acceleration_structure_capture_replay_descriptor_data_size: Default::default(),
      sampler_descriptor_size: Default::default(),
      combined_image_sampler_descriptor_size: Default::default(),
      sampled_image_descriptor_size: Default::default(),
      storage_image_descriptor_size: Default::default(),
      uniform_texel_buffer_descriptor_size: Default::default(),
      robust_uniform_texel_buffer_descriptor_size: Default::default(),
      storage_texel_buffer_descriptor_size: Default::default(),
      robust_storage_texel_buffer_descriptor_size: Default::default(),
      uniform_buffer_descriptor_size: Default::default(),
      robust_uniform_buffer_descriptor_size: Default::default(),
      storage_buffer_descriptor_size: Default::default(),
      robust_storage_buffer_descriptor_size: Default::default(),
      input_attachment_descriptor_size: Default::default(),
      acceleration_structure_descriptor_size: Default::default(),
      max_sampler_descriptor_buffer_range: Default::default(),
      max_resource_descriptor_buffer_range: Default::default(),
      sampler_descriptor_buffer_address_space_size: Default::default(),
      resource_descriptor_buffer_address_space_size: Default::default(),
      descriptor_buffer_address_space_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorIndexingFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_input_attachment_array_dynamic_indexing: VkBool32,
  pub shader_uniform_texel_buffer_array_dynamic_indexing: VkBool32,
  pub shader_storage_texel_buffer_array_dynamic_indexing: VkBool32,
  pub shader_uniform_buffer_array_non_uniform_indexing: VkBool32,
  pub shader_sampled_image_array_non_uniform_indexing: VkBool32,
  pub shader_storage_buffer_array_non_uniform_indexing: VkBool32,
  pub shader_storage_image_array_non_uniform_indexing: VkBool32,
  pub shader_input_attachment_array_non_uniform_indexing: VkBool32,
  pub shader_uniform_texel_buffer_array_non_uniform_indexing: VkBool32,
  pub shader_storage_texel_buffer_array_non_uniform_indexing: VkBool32,
  pub descriptor_binding_uniform_buffer_update_after_bind: VkBool32,
  pub descriptor_binding_sampled_image_update_after_bind: VkBool32,
  pub descriptor_binding_storage_image_update_after_bind: VkBool32,
  pub descriptor_binding_storage_buffer_update_after_bind: VkBool32,
  pub descriptor_binding_uniform_texel_buffer_update_after_bind: VkBool32,
  pub descriptor_binding_storage_texel_buffer_update_after_bind: VkBool32,
  pub descriptor_binding_update_unused_while_pending: VkBool32,
  pub descriptor_binding_partially_bound: VkBool32,
  pub descriptor_binding_variable_descriptor_count: VkBool32,
  pub runtime_descriptor_array: VkBool32,
}
impl Default for VkPhysicalDeviceDescriptorIndexingFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES,
      next: core::ptr::null_mut(),
      shader_input_attachment_array_dynamic_indexing: Default::default(),
      shader_uniform_texel_buffer_array_dynamic_indexing: Default::default(),
      shader_storage_texel_buffer_array_dynamic_indexing: Default::default(),
      shader_uniform_buffer_array_non_uniform_indexing: Default::default(),
      shader_sampled_image_array_non_uniform_indexing: Default::default(),
      shader_storage_buffer_array_non_uniform_indexing: Default::default(),
      shader_storage_image_array_non_uniform_indexing: Default::default(),
      shader_input_attachment_array_non_uniform_indexing: Default::default(),
      shader_uniform_texel_buffer_array_non_uniform_indexing: Default::default(),
      shader_storage_texel_buffer_array_non_uniform_indexing: Default::default(),
      descriptor_binding_uniform_buffer_update_after_bind: Default::default(),
      descriptor_binding_sampled_image_update_after_bind: Default::default(),
      descriptor_binding_storage_image_update_after_bind: Default::default(),
      descriptor_binding_storage_buffer_update_after_bind: Default::default(),
      descriptor_binding_uniform_texel_buffer_update_after_bind: Default::default(),
      descriptor_binding_storage_texel_buffer_update_after_bind: Default::default(),
      descriptor_binding_update_unused_while_pending: Default::default(),
      descriptor_binding_partially_bound: Default::default(),
      descriptor_binding_variable_descriptor_count: Default::default(),
      runtime_descriptor_array: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorIndexingProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorIndexingProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_update_after_bind_descriptors_in_all_pools: u32,
  /// * Limit Type: bitmask
  pub shader_uniform_buffer_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub shader_sampled_image_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub shader_storage_buffer_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub shader_storage_image_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub shader_input_attachment_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub robust_buffer_access_update_after_bind: VkBool32,
  /// * Limit Type: bitmask
  pub quad_divergent_implicit_lod: VkBool32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_samplers: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
  /// * Limit Type: max
  pub max_per_stage_update_after_bind_resources: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_samplers: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_storage_buffers: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_sampled_images: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_storage_images: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_input_attachments: u32,
}
impl Default for VkPhysicalDeviceDescriptorIndexingProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES,
      next: core::ptr::null_mut(),
      max_update_after_bind_descriptors_in_all_pools: Default::default(),
      shader_uniform_buffer_array_non_uniform_indexing_native: Default::default(),
      shader_sampled_image_array_non_uniform_indexing_native: Default::default(),
      shader_storage_buffer_array_non_uniform_indexing_native: Default::default(),
      shader_storage_image_array_non_uniform_indexing_native: Default::default(),
      shader_input_attachment_array_non_uniform_indexing_native: Default::default(),
      robust_buffer_access_update_after_bind: Default::default(),
      quad_divergent_implicit_lod: Default::default(),
      max_per_stage_descriptor_update_after_bind_samplers: Default::default(),
      max_per_stage_descriptor_update_after_bind_uniform_buffers: Default::default(),
      max_per_stage_descriptor_update_after_bind_storage_buffers: Default::default(),
      max_per_stage_descriptor_update_after_bind_sampled_images: Default::default(),
      max_per_stage_descriptor_update_after_bind_storage_images: Default::default(),
      max_per_stage_descriptor_update_after_bind_input_attachments: Default::default(),
      max_per_stage_update_after_bind_resources: Default::default(),
      max_descriptor_set_update_after_bind_samplers: Default::default(),
      max_descriptor_set_update_after_bind_uniform_buffers: Default::default(),
      max_descriptor_set_update_after_bind_uniform_buffers_dynamic: Default::default(),
      max_descriptor_set_update_after_bind_storage_buffers: Default::default(),
      max_descriptor_set_update_after_bind_storage_buffers_dynamic: Default::default(),
      max_descriptor_set_update_after_bind_sampled_images: Default::default(),
      max_descriptor_set_update_after_bind_storage_images: Default::default(),
      max_descriptor_set_update_after_bind_input_attachments: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub descriptor_set_host_mapping: VkBool32,
}
impl Default for VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE,
      next: core::ptr::null_mut(),
      descriptor_set_host_mapping: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub device_generated_commands: VkBool32,
}
impl Default for VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV,
      next: core::ptr::null_mut(),
      device_generated_commands: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_graphics_shader_group_count: u32,
  /// * Limit Type: max
  pub max_indirect_sequence_count: u32,
  /// * Limit Type: max
  pub max_indirect_commands_token_count: u32,
  /// * Limit Type: max
  pub max_indirect_commands_stream_count: u32,
  /// * Limit Type: max
  pub max_indirect_commands_token_offset: u32,
  /// * Limit Type: max
  pub max_indirect_commands_stream_stride: u32,
  /// * Limit Type: min
  pub min_sequences_count_buffer_offset_alignment: u32,
  /// * Limit Type: min
  pub min_sequences_index_buffer_offset_alignment: u32,
  /// * Limit Type: min
  pub min_indirect_commands_buffer_offset_alignment: u32,
}
impl Default for VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      max_graphics_shader_group_count: Default::default(),
      max_indirect_sequence_count: Default::default(),
      max_indirect_commands_token_count: Default::default(),
      max_indirect_commands_stream_count: Default::default(),
      max_indirect_commands_token_offset: Default::default(),
      max_indirect_commands_stream_stride: Default::default(),
      min_sequences_count_buffer_offset_alignment: Default::default(),
      min_sequences_index_buffer_offset_alignment: Default::default(),
      min_indirect_commands_buffer_offset_alignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDeviceMemoryReportFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub device_memory_report: VkBool32,
}
impl Default for VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT,
      next: core::ptr::null_mut(),
      device_memory_report: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDiagnosticsConfigFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub diagnostics_config: VkBool32,
}
impl Default for VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV,
      next: core::ptr::null_mut(),
      diagnostics_config: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDiscardRectanglePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// max number of active discard rectangles
  /// * Limit Type: max
  pub max_discard_rectangles: u32,
}
impl Default for VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      max_discard_rectangles: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDriverProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDriverProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDriverProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub driver_id: VkDriverId,
  /// * Limit Type: exact
  pub driver_name: zstring::ArrayZString<VK_MAX_DRIVER_NAME_SIZE>,
  /// * Limit Type: exact
  pub driver_info: zstring::ArrayZString<VK_MAX_DRIVER_INFO_SIZE>,
  /// * Limit Type: exact
  pub conformance_version: VkConformanceVersion,
}
impl Default for VkPhysicalDeviceDriverProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES,
      next: core::ptr::null_mut(),
      driver_id: Default::default(),
      driver_name: Default::default(),
      driver_info: Default::default(),
      conformance_version: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDrmPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDrmPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDrmPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub has_primary: VkBool32,
  /// * Limit Type: bitmask
  pub has_render: VkBool32,
  /// * Limit Type: noauto
  pub primary_major: i64,
  /// * Limit Type: noauto
  pub primary_minor: i64,
  /// * Limit Type: noauto
  pub render_major: i64,
  /// * Limit Type: noauto
  pub render_minor: i64,
}
impl Default for VkPhysicalDeviceDrmPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      has_primary: Default::default(),
      has_render: Default::default(),
      primary_major: Default::default(),
      primary_minor: Default::default(),
      render_major: Default::default(),
      render_minor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDynamicRenderingFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDynamicRenderingFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub dynamic_rendering: VkBool32,
}
impl Default for VkPhysicalDeviceDynamicRenderingFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES,
      next: core::ptr::null_mut(),
      dynamic_rendering: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExclusiveScissorFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub exclusive_scissor: VkBool32,
}
impl Default for VkPhysicalDeviceExclusiveScissorFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV,
      next: core::ptr::null_mut(),
      exclusive_scissor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExtendedDynamicState2FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub extended_dynamic_state_2: VkBool32,
  pub extended_dynamic_state_2_logic_op: VkBool32,
  pub extended_dynamic_state_2_patch_control_points: VkBool32,
}
impl Default for VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT,
      next: core::ptr::null_mut(),
      extended_dynamic_state_2: Default::default(),
      extended_dynamic_state_2_logic_op: Default::default(),
      extended_dynamic_state_2_patch_control_points: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExtendedDynamicState3FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3FeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub extended_dynamic_state_3_tessellation_domain_origin: VkBool32,
  pub extended_dynamic_state_3_depth_clamp_enable: VkBool32,
  pub extended_dynamic_state_3_polygon_mode: VkBool32,
  pub extended_dynamic_state_3_rasterization_samples: VkBool32,
  pub extended_dynamic_state_3_sample_mask: VkBool32,
  pub extended_dynamic_state_3_alpha_to_coverage_enable: VkBool32,
  pub extended_dynamic_state_3_alpha_to_one_enable: VkBool32,
  pub extended_dynamic_state_3_logic_op_enable: VkBool32,
  pub extended_dynamic_state_3_color_blend_enable: VkBool32,
  pub extended_dynamic_state_3_color_blend_equation: VkBool32,
  pub extended_dynamic_state_3_color_write_mask: VkBool32,
  pub extended_dynamic_state_3_rasterization_stream: VkBool32,
  pub extended_dynamic_state_3_conservative_rasterization_mode: VkBool32,
  pub extended_dynamic_state_3_extra_primitive_overestimation_size: VkBool32,
  pub extended_dynamic_state_3_depth_clip_enable: VkBool32,
  pub extended_dynamic_state_3_sample_locations_enable: VkBool32,
  pub extended_dynamic_state_3_color_blend_advanced: VkBool32,
  pub extended_dynamic_state_3_provoking_vertex_mode: VkBool32,
  pub extended_dynamic_state_3_line_rasterization_mode: VkBool32,
  pub extended_dynamic_state_3_line_stipple_enable: VkBool32,
  pub extended_dynamic_state_3_depth_clip_negative_one_to_one: VkBool32,
  pub extended_dynamic_state_3_viewport_w_scaling_enable: VkBool32,
  pub extended_dynamic_state_3_viewport_swizzle: VkBool32,
  pub extended_dynamic_state_3_coverage_to_color_enable: VkBool32,
  pub extended_dynamic_state_3_coverage_to_color_location: VkBool32,
  pub extended_dynamic_state_3_coverage_modulation_mode: VkBool32,
  pub extended_dynamic_state_3_coverage_modulation_table_enable: VkBool32,
  pub extended_dynamic_state_3_coverage_modulation_table: VkBool32,
  pub extended_dynamic_state_3_coverage_reduction_mode: VkBool32,
  pub extended_dynamic_state_3_representative_fragment_test_enable: VkBool32,
  pub extended_dynamic_state_3_shading_rate_image_enable: VkBool32,
}
impl Default for VkPhysicalDeviceExtendedDynamicState3FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT,
      next: core::ptr::null_mut(),
      extended_dynamic_state_3_tessellation_domain_origin: Default::default(),
      extended_dynamic_state_3_depth_clamp_enable: Default::default(),
      extended_dynamic_state_3_polygon_mode: Default::default(),
      extended_dynamic_state_3_rasterization_samples: Default::default(),
      extended_dynamic_state_3_sample_mask: Default::default(),
      extended_dynamic_state_3_alpha_to_coverage_enable: Default::default(),
      extended_dynamic_state_3_alpha_to_one_enable: Default::default(),
      extended_dynamic_state_3_logic_op_enable: Default::default(),
      extended_dynamic_state_3_color_blend_enable: Default::default(),
      extended_dynamic_state_3_color_blend_equation: Default::default(),
      extended_dynamic_state_3_color_write_mask: Default::default(),
      extended_dynamic_state_3_rasterization_stream: Default::default(),
      extended_dynamic_state_3_conservative_rasterization_mode: Default::default(),
      extended_dynamic_state_3_extra_primitive_overestimation_size: Default::default(),
      extended_dynamic_state_3_depth_clip_enable: Default::default(),
      extended_dynamic_state_3_sample_locations_enable: Default::default(),
      extended_dynamic_state_3_color_blend_advanced: Default::default(),
      extended_dynamic_state_3_provoking_vertex_mode: Default::default(),
      extended_dynamic_state_3_line_rasterization_mode: Default::default(),
      extended_dynamic_state_3_line_stipple_enable: Default::default(),
      extended_dynamic_state_3_depth_clip_negative_one_to_one: Default::default(),
      extended_dynamic_state_3_viewport_w_scaling_enable: Default::default(),
      extended_dynamic_state_3_viewport_swizzle: Default::default(),
      extended_dynamic_state_3_coverage_to_color_enable: Default::default(),
      extended_dynamic_state_3_coverage_to_color_location: Default::default(),
      extended_dynamic_state_3_coverage_modulation_mode: Default::default(),
      extended_dynamic_state_3_coverage_modulation_table_enable: Default::default(),
      extended_dynamic_state_3_coverage_modulation_table: Default::default(),
      extended_dynamic_state_3_coverage_reduction_mode: Default::default(),
      extended_dynamic_state_3_representative_fragment_test_enable: Default::default(),
      extended_dynamic_state_3_shading_rate_image_enable: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExtendedDynamicState3PropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState3PropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3PropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub dynamic_primitive_topology_unrestricted: VkBool32,
}
impl Default for VkPhysicalDeviceExtendedDynamicState3PropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      dynamic_primitive_topology_unrestricted: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExtendedDynamicStateFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub extended_dynamic_state: VkBool32,
}
impl Default for VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
      next: core::ptr::null_mut(),
      extended_dynamic_state: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalBufferInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalBufferInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkBufferCreateFlags,
  pub usage: VkBufferUsageFlags,
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkPhysicalDeviceExternalBufferInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      usage: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalFenceInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalFenceInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub handle_type: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkPhysicalDeviceExternalFenceInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO,
      next: core::ptr::null(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalImageFormatInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalImageFormatInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub handle_type: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkPhysicalDeviceExternalImageFormatInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO,
      next: core::ptr::null(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalMemoryHostPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: min,pot
  pub min_imported_host_pointer_alignment: VkDeviceSize,
}
impl Default for VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      min_imported_host_pointer_alignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalMemoryRDMAFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryRDMAFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub external_memory_rdma: VkBool32,
}
impl Default for VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV,
      next: core::ptr::null_mut(),
      external_memory_rdma: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalMemorySciBufFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemorySciBufFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalMemorySciBufFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub sci_buf_import: VkBool32,
  pub sci_buf_export: VkBool32,
}
impl Default for VkPhysicalDeviceExternalMemorySciBufFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV,
      next: core::ptr::null_mut(),
      sci_buf_import: Default::default(),
      sci_buf_export: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalSciSync2FeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSciSync2FeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSync2FeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub sci_sync_fence: VkBool32,
  pub sci_sync_semaphore_2: VkBool32,
  pub sci_sync_import: VkBool32,
  pub sci_sync_export: VkBool32,
}
impl Default for VkPhysicalDeviceExternalSciSync2FeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV,
      next: core::ptr::null_mut(),
      sci_sync_fence: Default::default(),
      sci_sync_semaphore_2: Default::default(),
      sci_sync_import: Default::default(),
      sci_sync_export: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalSciSyncFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSciSyncFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSyncFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub sci_sync_fence: VkBool32,
  pub sci_sync_semaphore: VkBool32,
  pub sci_sync_import: VkBool32,
  pub sci_sync_export: VkBool32,
}
impl Default for VkPhysicalDeviceExternalSciSyncFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV,
      next: core::ptr::null_mut(),
      sci_sync_fence: Default::default(),
      sci_sync_semaphore: Default::default(),
      sci_sync_import: Default::default(),
      sci_sync_export: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalSemaphoreInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkPhysicalDeviceExternalSemaphoreInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO,
      next: core::ptr::null(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFaultFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFaultFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFaultFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FAULT_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub device_fault: VkBool32,
  pub device_fault_vendor_binary: VkBool32,
}
impl Default for VkPhysicalDeviceFaultFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FAULT_FEATURES_EXT,
      next: core::ptr::null_mut(),
      device_fault: Default::default(),
      device_fault_vendor_binary: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
  /// out of bounds buffer accesses are well defined
  pub robust_buffer_access: VkBool32,
  /// full 32-bit range of indices for indexed draw calls
  pub full_draw_index_uint_32: VkBool32,
  /// image views which are arrays of cube maps
  pub image_cube_array: VkBool32,
  /// blending operations are controlled per-attachment
  pub independent_blend: VkBool32,
  /// geometry stage
  pub geometry_shader: VkBool32,
  /// tessellation control and evaluation stage
  pub tessellation_shader: VkBool32,
  /// per-sample shading and interpolation
  pub sample_rate_shading: VkBool32,
  /// blend operations which take two sources
  pub dual_src_blend: VkBool32,
  /// logic operations
  pub logic_op: VkBool32,
  /// multi draw indirect
  pub multi_draw_indirect: VkBool32,
  /// indirect drawing can use non-zero firstInstance
  pub draw_indirect_first_instance: VkBool32,
  /// depth clamping
  pub depth_clamp: VkBool32,
  /// depth bias clamping
  pub depth_bias_clamp: VkBool32,
  /// point and wireframe fill modes
  pub fill_mode_non_solid: VkBool32,
  /// depth bounds test
  pub depth_bounds: VkBool32,
  /// lines with width greater than 1
  pub wide_lines: VkBool32,
  /// points with size greater than 1
  pub large_points: VkBool32,
  /// the fragment alpha component can be forced to maximum representable alpha
  /// value
  pub alpha_to_one: VkBool32,
  /// viewport arrays
  pub multi_viewport: VkBool32,
  /// anisotropic sampler filtering
  pub sampler_anisotropy: VkBool32,
  /// ETC texture compression formats
  pub texture_compression_etc_2: VkBool32,
  /// ASTC LDR texture compression formats
  pub texture_compression_astc_ldr: VkBool32,
  /// BC1-7 texture compressed formats
  pub texture_compression_bc: VkBool32,
  /// precise occlusion queries returning actual sample counts
  pub occlusion_query_precise: VkBool32,
  /// pipeline statistics query
  pub pipeline_statistics_query: VkBool32,
  /// stores and atomic ops on storage buffers and images are supported in
  /// vertex, tessellation, and geometry stages
  pub vertex_pipeline_stores_and_atomics: VkBool32,
  /// stores and atomic ops on storage buffers and images are supported in the
  /// fragment stage
  pub fragment_stores_and_atomics: VkBool32,
  /// tessellation and geometry stages can export point size
  pub shader_tessellation_and_geometry_point_size: VkBool32,
  /// image gather with run-time values and independent offsets
  pub shader_image_gather_extended: VkBool32,
  /// the extended set of formats can be used for storage images
  pub shader_storage_image_extended_formats: VkBool32,
  /// multisample images can be used for storage images
  pub shader_storage_image_multisample: VkBool32,
  /// read from storage image does not require format qualifier
  pub shader_storage_image_read_without_format: VkBool32,
  /// write to storage image does not require format qualifier
  pub shader_storage_image_write_without_format: VkBool32,
  /// arrays of uniform buffers can be accessed with dynamically uniform indices
  pub shader_uniform_buffer_array_dynamic_indexing: VkBool32,
  /// arrays of sampled images can be accessed with dynamically uniform indices
  pub shader_sampled_image_array_dynamic_indexing: VkBool32,
  /// arrays of storage buffers can be accessed with dynamically uniform indices
  pub shader_storage_buffer_array_dynamic_indexing: VkBool32,
  /// arrays of storage images can be accessed with dynamically uniform indices
  pub shader_storage_image_array_dynamic_indexing: VkBool32,
  /// clip distance in shaders
  pub shader_clip_distance: VkBool32,
  /// cull distance in shaders
  pub shader_cull_distance: VkBool32,
  /// 64-bit floats (doubles) in shaders
  pub shader_float_64: VkBool32,
  /// 64-bit integers in shaders
  pub shader_int_64: VkBool32,
  /// 16-bit integers in shaders
  pub shader_int_16: VkBool32,
  /// shader can use texture operations that return resource residency
  /// information (requires sparseNonResident support)
  pub shader_resource_residency: VkBool32,
  /// shader can use texture operations that specify minimum resource LOD
  pub shader_resource_min_lod: VkBool32,
  /// Sparse resources support: Resource memory can be managed at opaque page
  /// level rather than object level
  pub sparse_binding: VkBool32,
  /// Sparse resources support: GPU can access partially resident buffers
  pub sparse_residency_buffer: VkBool32,
  /// Sparse resources support: GPU can access partially resident 2D (non-MSAA
  /// non-depth/stencil) images
  pub sparse_residency_image_2_d: VkBool32,
  /// Sparse resources support: GPU can access partially resident 3D images
  pub sparse_residency_image_3_d: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 2 samples
  pub sparse_residency_2_samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 4 samples
  pub sparse_residency_4_samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 8 samples
  pub sparse_residency_8_samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 16 samples
  pub sparse_residency_16_samples: VkBool32,
  /// Sparse resources support: GPU can correctly access data aliased into
  /// multiple locations (opt-in)
  pub sparse_residency_aliased: VkBool32,
  /// multisample rate must be the same for all pipelines in a subpass
  pub variable_multisample_rate: VkBool32,
  /// Queries may be inherited from primary to secondary command buffers
  pub inherited_queries: VkBool32,
}
impl Default for VkPhysicalDeviceFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      robust_buffer_access: Default::default(),
      full_draw_index_uint_32: Default::default(),
      image_cube_array: Default::default(),
      independent_blend: Default::default(),
      geometry_shader: Default::default(),
      tessellation_shader: Default::default(),
      sample_rate_shading: Default::default(),
      dual_src_blend: Default::default(),
      logic_op: Default::default(),
      multi_draw_indirect: Default::default(),
      draw_indirect_first_instance: Default::default(),
      depth_clamp: Default::default(),
      depth_bias_clamp: Default::default(),
      fill_mode_non_solid: Default::default(),
      depth_bounds: Default::default(),
      wide_lines: Default::default(),
      large_points: Default::default(),
      alpha_to_one: Default::default(),
      multi_viewport: Default::default(),
      sampler_anisotropy: Default::default(),
      texture_compression_etc_2: Default::default(),
      texture_compression_astc_ldr: Default::default(),
      texture_compression_bc: Default::default(),
      occlusion_query_precise: Default::default(),
      pipeline_statistics_query: Default::default(),
      vertex_pipeline_stores_and_atomics: Default::default(),
      fragment_stores_and_atomics: Default::default(),
      shader_tessellation_and_geometry_point_size: Default::default(),
      shader_image_gather_extended: Default::default(),
      shader_storage_image_extended_formats: Default::default(),
      shader_storage_image_multisample: Default::default(),
      shader_storage_image_read_without_format: Default::default(),
      shader_storage_image_write_without_format: Default::default(),
      shader_uniform_buffer_array_dynamic_indexing: Default::default(),
      shader_sampled_image_array_dynamic_indexing: Default::default(),
      shader_storage_buffer_array_dynamic_indexing: Default::default(),
      shader_storage_image_array_dynamic_indexing: Default::default(),
      shader_clip_distance: Default::default(),
      shader_cull_distance: Default::default(),
      shader_float_64: Default::default(),
      shader_int_64: Default::default(),
      shader_int_16: Default::default(),
      shader_resource_residency: Default::default(),
      shader_resource_min_lod: Default::default(),
      sparse_binding: Default::default(),
      sparse_residency_buffer: Default::default(),
      sparse_residency_image_2_d: Default::default(),
      sparse_residency_image_3_d: Default::default(),
      sparse_residency_2_samples: Default::default(),
      sparse_residency_4_samples: Default::default(),
      sparse_residency_8_samples: Default::default(),
      sparse_residency_16_samples: Default::default(),
      sparse_residency_aliased: Default::default(),
      variable_multisample_rate: Default::default(),
      inherited_queries: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFeatures2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures2.html) (structure)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub features: VkPhysicalDeviceFeatures,
}
impl Default for VkPhysicalDeviceFeatures2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
      next: core::ptr::null_mut(),
      features: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFloatControlsProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFloatControlsProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFloatControlsProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub denorm_behavior_independence: VkShaderFloatControlsIndependence,
  /// * Limit Type: exact
  pub rounding_mode_independence: VkShaderFloatControlsIndependence,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shader_signed_zero_inf_nan_preserve_float_16: VkBool32,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shader_signed_zero_inf_nan_preserve_float_32: VkBool32,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shader_signed_zero_inf_nan_preserve_float_64: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_preserve_float_16: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_preserve_float_32: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_preserve_float_64: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_flush_to_zero_float_16: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_flush_to_zero_float_32: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_flush_to_zero_float_64: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rte_float_16: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rte_float_32: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rte_float_64: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rtz_float_16: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rtz_float_32: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rtz_float_64: VkBool32,
}
impl Default for VkPhysicalDeviceFloatControlsProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES,
      next: core::ptr::null_mut(),
      denorm_behavior_independence: Default::default(),
      rounding_mode_independence: Default::default(),
      shader_signed_zero_inf_nan_preserve_float_16: Default::default(),
      shader_signed_zero_inf_nan_preserve_float_32: Default::default(),
      shader_signed_zero_inf_nan_preserve_float_64: Default::default(),
      shader_denorm_preserve_float_16: Default::default(),
      shader_denorm_preserve_float_32: Default::default(),
      shader_denorm_preserve_float_64: Default::default(),
      shader_denorm_flush_to_zero_float_16: Default::default(),
      shader_denorm_flush_to_zero_float_32: Default::default(),
      shader_denorm_flush_to_zero_float_64: Default::default(),
      shader_rounding_mode_rte_float_16: Default::default(),
      shader_rounding_mode_rte_float_32: Default::default(),
      shader_rounding_mode_rte_float_64: Default::default(),
      shader_rounding_mode_rtz_float_16: Default::default(),
      shader_rounding_mode_rtz_float_32: Default::default(),
      shader_rounding_mode_rtz_float_64: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMap2FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2FeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub fragment_density_map_deferred: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT,
      next: core::ptr::null_mut(),
      fragment_density_map_deferred: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMap2PropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub subsampled_loads: VkBool32,
  /// * Limit Type: exact
  pub subsampled_coarse_reconstruction_early_access: VkBool32,
  /// * Limit Type: max
  pub max_subsampled_array_layers: u32,
  /// * Limit Type: max
  pub max_descriptor_set_subsampled_samplers: u32,
}
impl Default for VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      subsampled_loads: Default::default(),
      subsampled_coarse_reconstruction_early_access: Default::default(),
      max_subsampled_array_layers: Default::default(),
      max_descriptor_set_subsampled_samplers: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMapFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub fragment_density_map: VkBool32,
  pub fragment_density_map_dynamic: VkBool32,
  pub fragment_density_map_non_subsampled_images: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT,
      next: core::ptr::null_mut(),
      fragment_density_map: Default::default(),
      fragment_density_map_dynamic: Default::default(),
      fragment_density_map_non_subsampled_images: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub fragment_density_map_offset: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM,
      next: core::ptr::null_mut(),
      fragment_density_map_offset: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: min,mul
  pub fragment_density_offset_granularity: VkExtent2D,
}
impl Default for VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM,
      next: core::ptr::null_mut(),
      fragment_density_offset_granularity: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMapPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: min
  pub min_fragment_density_texel_size: VkExtent2D,
  /// * Limit Type: max
  pub max_fragment_density_texel_size: VkExtent2D,
  /// * Limit Type: bitmask
  pub fragment_density_invocations: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      min_fragment_density_texel_size: Default::default(),
      max_fragment_density_texel_size: Default::default(),
      fragment_density_invocations: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub fragment_shader_barycentric: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR,
      next: core::ptr::null_mut(),
      fragment_shader_barycentric: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub tri_strip_vertex_order_independent_of_provoking_vertex: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      tri_strip_vertex_order_independent_of_provoking_vertex: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// Pointer to next structure
  /// * Optional: true
  pub next: *mut c_void,
  pub fragment_shader_sample_interlock: VkBool32,
  pub fragment_shader_pixel_interlock: VkBool32,
  pub fragment_shader_shading_rate_interlock: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT,
      next: core::ptr::null_mut(),
      fragment_shader_sample_interlock: Default::default(),
      fragment_shader_pixel_interlock: Default::default(),
      fragment_shader_shading_rate_interlock: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub fragment_shading_rate_enums: VkBool32,
  pub supersample_fragment_shading_rates: VkBool32,
  pub no_invocation_fragment_shading_rates: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV,
      next: core::ptr::null_mut(),
      fragment_shading_rate_enums: Default::default(),
      supersample_fragment_shading_rates: Default::default(),
      no_invocation_fragment_shading_rates: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_fragment_shading_rate_invocation_count: VkSampleCountFlagBits,
}
impl Default for VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      max_fragment_shading_rate_invocation_count: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRateFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub pipeline_fragment_shading_rate: VkBool32,
  pub primitive_fragment_shading_rate: VkBool32,
  pub attachment_fragment_shading_rate: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR,
      next: core::ptr::null_mut(),
      pipeline_fragment_shading_rate: Default::default(),
      primitive_fragment_shading_rate: Default::default(),
      attachment_fragment_shading_rate: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRateKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub sample_counts: VkSampleCountFlags,
  pub fragment_size: VkExtent2D,
}
impl Default for VkPhysicalDeviceFragmentShadingRateKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR,
      next: core::ptr::null_mut(),
      sample_counts: Default::default(),
      fragment_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRatePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: min
  pub min_fragment_shading_rate_attachment_texel_size: VkExtent2D,
  /// * Limit Type: max
  pub max_fragment_shading_rate_attachment_texel_size: VkExtent2D,
  /// * Limit Type: max,pot
  pub max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
  /// * Limit Type: bitmask
  pub primitive_fragment_shading_rate_with_multiple_viewports: VkBool32,
  /// * Limit Type: bitmask
  pub layered_shading_rate_attachments: VkBool32,
  /// * Limit Type: bitmask
  pub fragment_shading_rate_non_trivial_combiner_ops: VkBool32,
  /// * Limit Type: max
  pub max_fragment_size: VkExtent2D,
  /// * Limit Type: max,pot
  pub max_fragment_size_aspect_ratio: u32,
  /// * Limit Type: max
  pub max_fragment_shading_rate_coverage_samples: u32,
  /// * Limit Type: max
  pub max_fragment_shading_rate_rasterization_samples: VkSampleCountFlagBits,
  /// * Limit Type: bitmask
  pub fragment_shading_rate_with_shader_depth_stencil_writes: VkBool32,
  /// * Limit Type: bitmask
  pub fragment_shading_rate_with_sample_mask: VkBool32,
  /// * Limit Type: bitmask
  pub fragment_shading_rate_with_shader_sample_mask: VkBool32,
  /// * Limit Type: bitmask
  pub fragment_shading_rate_with_conservative_rasterization: VkBool32,
  /// * Limit Type: bitmask
  pub fragment_shading_rate_with_fragment_shader_interlock: VkBool32,
  /// * Limit Type: bitmask
  pub fragment_shading_rate_with_custom_sample_locations: VkBool32,
  /// * Limit Type: bitmask
  pub fragment_shading_rate_strict_multiply_combiner: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      min_fragment_shading_rate_attachment_texel_size: Default::default(),
      max_fragment_shading_rate_attachment_texel_size: Default::default(),
      max_fragment_shading_rate_attachment_texel_size_aspect_ratio: Default::default(),
      primitive_fragment_shading_rate_with_multiple_viewports: Default::default(),
      layered_shading_rate_attachments: Default::default(),
      fragment_shading_rate_non_trivial_combiner_ops: Default::default(),
      max_fragment_size: Default::default(),
      max_fragment_size_aspect_ratio: Default::default(),
      max_fragment_shading_rate_coverage_samples: Default::default(),
      max_fragment_shading_rate_rasterization_samples: Default::default(),
      fragment_shading_rate_with_shader_depth_stencil_writes: Default::default(),
      fragment_shading_rate_with_sample_mask: Default::default(),
      fragment_shading_rate_with_shader_sample_mask: Default::default(),
      fragment_shading_rate_with_conservative_rasterization: Default::default(),
      fragment_shading_rate_with_fragment_shader_interlock: Default::default(),
      fragment_shading_rate_with_custom_sample_locations: Default::default(),
      fragment_shading_rate_strict_multiply_combiner: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub global_priority_query: VkBool32,
}
impl Default for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR,
      next: core::ptr::null_mut(),
      global_priority_query: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub graphics_pipeline_library: VkBool32,
}
impl Default for VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT,
      next: core::ptr::null_mut(),
      graphics_pipeline_library: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub graphics_pipeline_library_fast_linking: VkBool32,
  /// * Limit Type: bitmask
  pub graphics_pipeline_library_independent_interpolation_decoration: VkBool32,
}
impl Default for VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      graphics_pipeline_library_fast_linking: Default::default(),
      graphics_pipeline_library_independent_interpolation_decoration: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceGroupProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGroupProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceGroupProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub physical_device_count: u32,
  pub physical_devices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE],
  pub subset_allocation: VkBool32,
}
impl Default for VkPhysicalDeviceGroupProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES,
      next: core::ptr::null_mut(),
      physical_device_count: Default::default(),
      physical_devices: [Default::default(); VK_MAX_DEVICE_GROUP_SIZE],
      subset_allocation: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceHostQueryResetFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceHostQueryResetFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceHostQueryResetFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub host_query_reset: VkBool32,
}
impl Default for VkPhysicalDeviceHostQueryResetFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES,
      next: core::ptr::null_mut(),
      host_query_reset: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceIDProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIDProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceIDProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: noauto
  pub device_uuid: [u8; VK_UUID_SIZE],
  /// * Limit Type: noauto
  pub driver_uuid: [u8; VK_UUID_SIZE],
  /// * Limit Type: noauto
  pub device_luid: zstring::ArrayZString<VK_LUID_SIZE>,
  /// * Limit Type: noauto
  pub device_node_mask: u32,
  /// * Limit Type: noauto
  pub device_luid_valid: VkBool32,
}
impl Default for VkPhysicalDeviceIDProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES,
      next: core::ptr::null_mut(),
      device_uuid: [Default::default(); VK_UUID_SIZE],
      driver_uuid: [Default::default(); VK_UUID_SIZE],
      device_luid: Default::default(),
      device_node_mask: Default::default(),
      device_luid_valid: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImage2DViewOf3DFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImage2DViewOf3DFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub image_2_d_view_of_3_d: VkBool32,
  pub sampler_2_d_view_of_3_d: VkBool32,
}
impl Default for VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT,
      next: core::ptr::null_mut(),
      image_2_d_view_of_3_d: Default::default(),
      sampler_2_d_view_of_3_d: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageCompressionControlFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageCompressionControlFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub image_compression_control: VkBool32,
}
impl Default for VkPhysicalDeviceImageCompressionControlFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT,
      next: core::ptr::null_mut(),
      image_compression_control: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub image_compression_control_swapchain: VkBool32,
}
impl Default for VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT,
      next: core::ptr::null_mut(),
      image_compression_control_swapchain: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageDrmFormatModifierInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub drm_format_modifier: u64,
  pub sharing_mode: VkSharingMode,
  /// * Optional: true
  pub queue_family_index_count: u32,
  /// * Len: `queue_family_index_count`
  /// * No Auto-Validity
  pub queue_family_indices: *const u32,
}
impl Default for VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT,
      next: core::ptr::null(),
      drm_format_modifier: Default::default(),
      sharing_mode: Default::default(),
      queue_family_index_count: Default::default(),
      queue_family_indices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageFormatInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageFormatInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub format: VkFormat,
  pub ty: VkImageType,
  pub tiling: VkImageTiling,
  pub usage: VkImageUsageFlags,
  /// * Optional: true
  pub flags: VkImageCreateFlags,
}
impl Default for VkPhysicalDeviceImageFormatInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2,
      next: core::ptr::null(),
      format: Default::default(),
      ty: Default::default(),
      tiling: Default::default(),
      usage: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageProcessingFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageProcessingFeaturesQCOM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingFeaturesQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub texture_sample_weighted: VkBool32,
  pub texture_box_filter: VkBool32,
  pub texture_block_match: VkBool32,
}
impl Default for VkPhysicalDeviceImageProcessingFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM,
      next: core::ptr::null_mut(),
      texture_sample_weighted: Default::default(),
      texture_box_filter: Default::default(),
      texture_block_match: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageProcessingPropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageProcessingPropertiesQCOM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingPropertiesQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  /// * Limit Type: max
  pub max_weight_filter_phases: u32,
  /// * Optional: true
  /// * Limit Type: max
  pub max_weight_filter_dimension: VkExtent2D,
  /// * Optional: true
  /// * Limit Type: max
  pub max_block_match_region: VkExtent2D,
  /// * Optional: true
  /// * Limit Type: max
  pub max_box_filter_block_size: VkExtent2D,
}
impl Default for VkPhysicalDeviceImageProcessingPropertiesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM,
      next: core::ptr::null_mut(),
      max_weight_filter_phases: Default::default(),
      max_weight_filter_dimension: Default::default(),
      max_block_match_region: Default::default(),
      max_box_filter_block_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageRobustnessFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageRobustnessFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageRobustnessFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub robust_image_access: VkBool32,
}
impl Default for VkPhysicalDeviceImageRobustnessFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES,
      next: core::ptr::null_mut(),
      robust_image_access: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub image_sliced_view_of_3_d: VkBool32,
}
impl Default for VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT,
      next: core::ptr::null_mut(),
      image_sliced_view_of_3_d: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageViewImageFormatInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub image_view_type: VkImageViewType,
}
impl Default for VkPhysicalDeviceImageViewImageFormatInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT,
      next: core::ptr::null_mut(),
      image_view_type: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageViewMinLodFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub min_lod: VkBool32,
}
impl Default for VkPhysicalDeviceImageViewMinLodFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT,
      next: core::ptr::null_mut(),
      min_lod: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImagelessFramebufferFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub imageless_framebuffer: VkBool32,
}
impl Default for VkPhysicalDeviceImagelessFramebufferFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES,
      next: core::ptr::null_mut(),
      imageless_framebuffer: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceIndexTypeUint8FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub index_type_uint_8: VkBool32,
}
impl Default for VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT,
      next: core::ptr::null_mut(),
      index_type_uint_8: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceInheritedViewportScissorFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub inherited_viewport_scissor_2_d: VkBool32,
}
impl Default for VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV,
      next: core::ptr::null_mut(),
      inherited_viewport_scissor_2_d: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceInlineUniformBlockFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInlineUniformBlockFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub inline_uniform_block: VkBool32,
  pub descriptor_binding_inline_uniform_block_update_after_bind: VkBool32,
}
impl Default for VkPhysicalDeviceInlineUniformBlockFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES,
      next: core::ptr::null_mut(),
      inline_uniform_block: Default::default(),
      descriptor_binding_inline_uniform_block_update_after_bind: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceInlineUniformBlockProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInlineUniformBlockProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_inline_uniform_block_size: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_inline_uniform_blocks: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
  /// * Limit Type: max
  pub max_descriptor_set_inline_uniform_blocks: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}
impl Default for VkPhysicalDeviceInlineUniformBlockProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES,
      next: core::ptr::null_mut(),
      max_inline_uniform_block_size: Default::default(),
      max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
      max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(
      ),
      max_descriptor_set_inline_uniform_blocks: Default::default(),
      max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceInvocationMaskFeaturesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub invocation_mask: VkBool32,
}
impl Default for VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI,
      next: core::ptr::null_mut(),
      invocation_mask: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLegacyDitheringFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLegacyDitheringFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceLegacyDitheringFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub legacy_dithering: VkBool32,
}
impl Default for VkPhysicalDeviceLegacyDitheringFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT,
      next: core::ptr::null_mut(),
      legacy_dithering: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLimits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLimits.html) (structure)
///
/// compute stage limits
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
  /// max 1D image dimension
  /// * Limit Type: max
  pub max_image_dimension_1_d: u32,
  /// max 2D image dimension
  /// * Limit Type: max
  pub max_image_dimension_2_d: u32,
  /// max 3D image dimension
  /// * Limit Type: max
  pub max_image_dimension_3_d: u32,
  /// max cubemap image dimension
  /// * Limit Type: max
  pub max_image_dimension_cube: u32,
  /// max layers for image arrays
  /// * Limit Type: max
  pub max_image_array_layers: u32,
  /// max texel buffer size (fstexels)
  /// * Limit Type: max
  pub max_texel_buffer_elements: u32,
  /// max uniform buffer range (bytes)
  /// * Limit Type: max
  pub max_uniform_buffer_range: u32,
  /// max storage buffer range (bytes)
  /// * Limit Type: max
  pub max_storage_buffer_range: u32,
  /// max size of the push constants pool (bytes)
  /// * Limit Type: max
  pub max_push_constants_size: u32,
  /// max number of device memory allocations supported
  /// * Limit Type: max
  pub max_memory_allocation_count: u32,
  /// max number of samplers that can be allocated on a device
  /// * Limit Type: max
  pub max_sampler_allocation_count: u32,
  /// Granularity (in bytes) at which buffers and images can be bound to
  /// adjacent memory for simultaneous usage
  /// * Limit Type: min,mul
  pub buffer_image_granularity: VkDeviceSize,
  /// Total address space available for sparse allocations (bytes)
  /// * Limit Type: max
  pub sparse_address_space_size: VkDeviceSize,
  /// max number of descriptors sets that can be bound to a pipeline
  /// * Limit Type: max
  pub max_bound_descriptor_sets: u32,
  /// max number of samplers allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_samplers: u32,
  /// max number of uniform buffers allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_uniform_buffers: u32,
  /// max number of storage buffers allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_storage_buffers: u32,
  /// max number of sampled images allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_sampled_images: u32,
  /// max number of storage images allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_storage_images: u32,
  /// max number of input attachments allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_input_attachments: u32,
  /// max number of resources allowed by a single stage
  /// * Limit Type: max
  pub max_per_stage_resources: u32,
  /// max number of samplers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_samplers: u32,
  /// max number of uniform buffers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_uniform_buffers: u32,
  /// max number of dynamic uniform buffers allowed in all stages in a
  /// descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_uniform_buffers_dynamic: u32,
  /// max number of storage buffers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_storage_buffers: u32,
  /// max number of dynamic storage buffers allowed in all stages in a
  /// descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_storage_buffers_dynamic: u32,
  /// max number of sampled images allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_sampled_images: u32,
  /// max number of storage images allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_storage_images: u32,
  /// max number of input attachments allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_input_attachments: u32,
  /// max number of vertex input attribute slots
  /// * Limit Type: max
  pub max_vertex_input_attributes: u32,
  /// max number of vertex input binding slots
  /// * Limit Type: max
  pub max_vertex_input_bindings: u32,
  /// max vertex input attribute offset added to vertex buffer offset
  /// * Limit Type: max
  pub max_vertex_input_attribute_offset: u32,
  /// max vertex input binding stride
  /// * Limit Type: max
  pub max_vertex_input_binding_stride: u32,
  /// max number of output components written by vertex shader
  /// * Limit Type: max
  pub max_vertex_output_components: u32,
  /// max level supported by tessellation primitive generator
  /// * Limit Type: max
  pub max_tessellation_generation_level: u32,
  /// max patch size (vertices)
  /// * Limit Type: max
  pub max_tessellation_patch_size: u32,
  /// max number of input components per-vertex in TCS
  /// * Limit Type: max
  pub max_tessellation_control_per_vertex_input_components: u32,
  /// max number of output components per-vertex in TCS
  /// * Limit Type: max
  pub max_tessellation_control_per_vertex_output_components: u32,
  /// max number of output components per-patch in TCS
  /// * Limit Type: max
  pub max_tessellation_control_per_patch_output_components: u32,
  /// max total number of per-vertex and per-patch output components in TCS
  /// * Limit Type: max
  pub max_tessellation_control_total_output_components: u32,
  /// max number of input components per vertex in TES
  /// * Limit Type: max
  pub max_tessellation_evaluation_input_components: u32,
  /// max number of output components per vertex in TES
  /// * Limit Type: max
  pub max_tessellation_evaluation_output_components: u32,
  /// max invocation count supported in geometry shader
  /// * Limit Type: max
  pub max_geometry_shader_invocations: u32,
  /// max number of input components read in geometry stage
  /// * Limit Type: max
  pub max_geometry_input_components: u32,
  /// max number of output components written in geometry stage
  /// * Limit Type: max
  pub max_geometry_output_components: u32,
  /// max number of vertices that can be emitted in geometry stage
  /// * Limit Type: max
  pub max_geometry_output_vertices: u32,
  /// max total number of components (all vertices) written in geometry stage
  /// * Limit Type: max
  pub max_geometry_total_output_components: u32,
  /// max number of input components read in fragment stage
  /// * Limit Type: max
  pub max_fragment_input_components: u32,
  /// max number of output attachments written in fragment stage
  /// * Limit Type: max
  pub max_fragment_output_attachments: u32,
  /// max number of output attachments written when using dual source blending
  /// * Limit Type: max
  pub max_fragment_dual_src_attachments: u32,
  /// max total number of storage buffers, storage images and output buffers
  /// * Limit Type: max
  pub max_fragment_combined_output_resources: u32,
  /// max total storage size of work group local storage (bytes)
  /// * Limit Type: max
  pub max_compute_shared_memory_size: u32,
  /// max num of compute work groups that may be dispatched by a single command
  /// (x,y,z)
  /// * Limit Type: max
  pub max_compute_work_group_count: [u32; 3],
  /// max total compute invocations in a single local work group
  /// * Limit Type: max
  pub max_compute_work_group_invocations: u32,
  /// max local size of a compute work group (x,y,z)
  /// * Limit Type: max
  pub max_compute_work_group_size: [u32; 3],
  /// number bits of subpixel precision in screen x and y
  /// * Limit Type: bits
  pub sub_pixel_precision_bits: u32,
  /// number bits of precision for selecting texel weights
  /// * Limit Type: bits
  pub sub_texel_precision_bits: u32,
  /// number bits of precision for selecting mipmap weights
  /// * Limit Type: bits
  pub mipmap_precision_bits: u32,
  /// max index value for indexed draw calls (for 32-bit indices)
  /// * Limit Type: max
  pub max_draw_indexed_index_value: u32,
  /// max draw count for indirect drawing calls
  /// * Limit Type: max
  pub max_draw_indirect_count: u32,
  /// max absolute sampler LOD bias
  /// * Limit Type: max
  pub max_sampler_lod_bias: c_float,
  /// max degree of sampler anisotropy
  /// * Limit Type: max
  pub max_sampler_anisotropy: c_float,
  /// max number of active viewports
  /// * Limit Type: max
  pub max_viewports: u32,
  /// max viewport dimensions (x,y)
  /// * Limit Type: max
  pub max_viewport_dimensions: [u32; 2],
  /// viewport bounds range (min,max)
  /// * Limit Type: range
  pub viewport_bounds_range: [c_float; 2],
  /// number bits of subpixel precision for viewport
  /// * Limit Type: bits
  pub viewport_sub_pixel_bits: u32,
  /// min required alignment of pointers returned by MapMemory (bytes)
  /// * Limit Type: min,pot
  pub min_memory_map_alignment: c_size_t,
  /// min required alignment for texel buffer offsets (bytes)
  /// * Limit Type: min,pot
  pub min_texel_buffer_offset_alignment: VkDeviceSize,
  /// min required alignment for uniform buffer sizes and offsets (bytes)
  /// * Limit Type: min,pot
  pub min_uniform_buffer_offset_alignment: VkDeviceSize,
  /// min required alignment for storage buffer offsets (bytes)
  /// * Limit Type: min,pot
  pub min_storage_buffer_offset_alignment: VkDeviceSize,
  /// min texel offset for OpTextureSampleOffset
  /// * Limit Type: min
  pub min_texel_offset: i32,
  /// max texel offset for OpTextureSampleOffset
  /// * Limit Type: max
  pub max_texel_offset: u32,
  /// min texel offset for OpTextureGatherOffset
  /// * Limit Type: min
  pub min_texel_gather_offset: i32,
  /// max texel offset for OpTextureGatherOffset
  /// * Limit Type: max
  pub max_texel_gather_offset: u32,
  /// furthest negative offset for interpolateAtOffset
  /// * Limit Type: min
  pub min_interpolation_offset: c_float,
  /// furthest positive offset for interpolateAtOffset
  /// * Limit Type: max
  pub max_interpolation_offset: c_float,
  /// number of subpixel bits for interpolateAtOffset
  /// * Limit Type: bits
  pub sub_pixel_interpolation_offset_bits: u32,
  /// max width for a framebuffer
  /// * Limit Type: max
  pub max_framebuffer_width: u32,
  /// max height for a framebuffer
  /// * Limit Type: max
  pub max_framebuffer_height: u32,
  /// max layer count for a layered framebuffer
  /// * Limit Type: max
  pub max_framebuffer_layers: u32,
  /// supported color sample counts for a framebuffer
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebuffer_color_sample_counts: VkSampleCountFlags,
  /// supported depth sample counts for a framebuffer
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebuffer_depth_sample_counts: VkSampleCountFlags,
  /// supported stencil sample counts for a framebuffer
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebuffer_stencil_sample_counts: VkSampleCountFlags,
  /// supported sample counts for a subpass which uses no attachments
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebuffer_no_attachments_sample_counts: VkSampleCountFlags,
  /// max number of color attachments per subpass
  /// * Limit Type: max
  pub max_color_attachments: u32,
  /// supported color sample counts for a non-integer sampled image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub sampled_image_color_sample_counts: VkSampleCountFlags,
  /// supported sample counts for an integer image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub sampled_image_integer_sample_counts: VkSampleCountFlags,
  /// supported depth sample counts for a sampled image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub sampled_image_depth_sample_counts: VkSampleCountFlags,
  /// supported stencil sample counts for a sampled image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub sampled_image_stencil_sample_counts: VkSampleCountFlags,
  /// supported sample counts for a storage image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub storage_image_sample_counts: VkSampleCountFlags,
  /// max number of sample mask words
  /// * Limit Type: max
  pub max_sample_mask_words: u32,
  /// timestamps on graphics and compute queues
  /// * Limit Type: bitmask
  pub timestamp_compute_and_graphics: VkBool32,
  /// number of nanoseconds it takes for timestamp query value to increment by 1
  /// * Limit Type: min,mul
  pub timestamp_period: c_float,
  /// max number of clip distances
  /// * Limit Type: max
  pub max_clip_distances: u32,
  /// max number of cull distances
  /// * Limit Type: max
  pub max_cull_distances: u32,
  /// max combined number of user clipping
  /// * Limit Type: max
  pub max_combined_clip_and_cull_distances: u32,
  /// distinct queue priorities available
  /// * Limit Type: max
  pub discrete_queue_priorities: u32,
  /// range (min,max) of supported point sizes
  /// * Limit Type: range
  pub point_size_range: [c_float; 2],
  /// range (min,max) of supported line widths
  /// * Limit Type: range
  pub line_width_range: [c_float; 2],
  /// granularity of supported point sizes
  /// * Limit Type: min,mul
  pub point_size_granularity: c_float,
  /// granularity of supported line widths
  /// * Limit Type: min,mul
  pub line_width_granularity: c_float,
  /// line rasterization follows preferred rules
  /// * Limit Type: bitmask
  pub strict_lines: VkBool32,
  /// supports standard sample locations for all supported sample counts
  /// * Limit Type: bitmask
  pub standard_sample_locations: VkBool32,
  /// optimal offset of buffer copies
  /// * Limit Type: min,pot
  pub optimal_buffer_copy_offset_alignment: VkDeviceSize,
  /// optimal pitch of buffer copies
  /// * Limit Type: min,pot
  pub optimal_buffer_copy_row_pitch_alignment: VkDeviceSize,
  /// minimum size and alignment for non-coherent host-mapped device memory
  /// access
  /// * Limit Type: min,pot
  pub non_coherent_atom_size: VkDeviceSize,
}
impl Default for VkPhysicalDeviceLimits {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      max_image_dimension_1_d: Default::default(),
      max_image_dimension_2_d: Default::default(),
      max_image_dimension_3_d: Default::default(),
      max_image_dimension_cube: Default::default(),
      max_image_array_layers: Default::default(),
      max_texel_buffer_elements: Default::default(),
      max_uniform_buffer_range: Default::default(),
      max_storage_buffer_range: Default::default(),
      max_push_constants_size: Default::default(),
      max_memory_allocation_count: Default::default(),
      max_sampler_allocation_count: Default::default(),
      buffer_image_granularity: Default::default(),
      sparse_address_space_size: Default::default(),
      max_bound_descriptor_sets: Default::default(),
      max_per_stage_descriptor_samplers: Default::default(),
      max_per_stage_descriptor_uniform_buffers: Default::default(),
      max_per_stage_descriptor_storage_buffers: Default::default(),
      max_per_stage_descriptor_sampled_images: Default::default(),
      max_per_stage_descriptor_storage_images: Default::default(),
      max_per_stage_descriptor_input_attachments: Default::default(),
      max_per_stage_resources: Default::default(),
      max_descriptor_set_samplers: Default::default(),
      max_descriptor_set_uniform_buffers: Default::default(),
      max_descriptor_set_uniform_buffers_dynamic: Default::default(),
      max_descriptor_set_storage_buffers: Default::default(),
      max_descriptor_set_storage_buffers_dynamic: Default::default(),
      max_descriptor_set_sampled_images: Default::default(),
      max_descriptor_set_storage_images: Default::default(),
      max_descriptor_set_input_attachments: Default::default(),
      max_vertex_input_attributes: Default::default(),
      max_vertex_input_bindings: Default::default(),
      max_vertex_input_attribute_offset: Default::default(),
      max_vertex_input_binding_stride: Default::default(),
      max_vertex_output_components: Default::default(),
      max_tessellation_generation_level: Default::default(),
      max_tessellation_patch_size: Default::default(),
      max_tessellation_control_per_vertex_input_components: Default::default(),
      max_tessellation_control_per_vertex_output_components: Default::default(),
      max_tessellation_control_per_patch_output_components: Default::default(),
      max_tessellation_control_total_output_components: Default::default(),
      max_tessellation_evaluation_input_components: Default::default(),
      max_tessellation_evaluation_output_components: Default::default(),
      max_geometry_shader_invocations: Default::default(),
      max_geometry_input_components: Default::default(),
      max_geometry_output_components: Default::default(),
      max_geometry_output_vertices: Default::default(),
      max_geometry_total_output_components: Default::default(),
      max_fragment_input_components: Default::default(),
      max_fragment_output_attachments: Default::default(),
      max_fragment_dual_src_attachments: Default::default(),
      max_fragment_combined_output_resources: Default::default(),
      max_compute_shared_memory_size: Default::default(),
      max_compute_work_group_count: [Default::default(); 3],
      max_compute_work_group_invocations: Default::default(),
      max_compute_work_group_size: [Default::default(); 3],
      sub_pixel_precision_bits: Default::default(),
      sub_texel_precision_bits: Default::default(),
      mipmap_precision_bits: Default::default(),
      max_draw_indexed_index_value: Default::default(),
      max_draw_indirect_count: Default::default(),
      max_sampler_lod_bias: Default::default(),
      max_sampler_anisotropy: Default::default(),
      max_viewports: Default::default(),
      max_viewport_dimensions: [Default::default(); 2],
      viewport_bounds_range: [Default::default(); 2],
      viewport_sub_pixel_bits: Default::default(),
      min_memory_map_alignment: Default::default(),
      min_texel_buffer_offset_alignment: Default::default(),
      min_uniform_buffer_offset_alignment: Default::default(),
      min_storage_buffer_offset_alignment: Default::default(),
      min_texel_offset: Default::default(),
      max_texel_offset: Default::default(),
      min_texel_gather_offset: Default::default(),
      max_texel_gather_offset: Default::default(),
      min_interpolation_offset: Default::default(),
      max_interpolation_offset: Default::default(),
      sub_pixel_interpolation_offset_bits: Default::default(),
      max_framebuffer_width: Default::default(),
      max_framebuffer_height: Default::default(),
      max_framebuffer_layers: Default::default(),
      framebuffer_color_sample_counts: Default::default(),
      framebuffer_depth_sample_counts: Default::default(),
      framebuffer_stencil_sample_counts: Default::default(),
      framebuffer_no_attachments_sample_counts: Default::default(),
      max_color_attachments: Default::default(),
      sampled_image_color_sample_counts: Default::default(),
      sampled_image_integer_sample_counts: Default::default(),
      sampled_image_depth_sample_counts: Default::default(),
      sampled_image_stencil_sample_counts: Default::default(),
      storage_image_sample_counts: Default::default(),
      max_sample_mask_words: Default::default(),
      timestamp_compute_and_graphics: Default::default(),
      timestamp_period: Default::default(),
      max_clip_distances: Default::default(),
      max_cull_distances: Default::default(),
      max_combined_clip_and_cull_distances: Default::default(),
      discrete_queue_priorities: Default::default(),
      point_size_range: [Default::default(); 2],
      line_width_range: [Default::default(); 2],
      point_size_granularity: Default::default(),
      line_width_granularity: Default::default(),
      strict_lines: Default::default(),
      standard_sample_locations: Default::default(),
      optimal_buffer_copy_offset_alignment: Default::default(),
      optimal_buffer_copy_row_pitch_alignment: Default::default(),
      non_coherent_atom_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLineRasterizationFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub rectangular_lines: VkBool32,
  pub bresenham_lines: VkBool32,
  pub smooth_lines: VkBool32,
  pub stippled_rectangular_lines: VkBool32,
  pub stippled_bresenham_lines: VkBool32,
  pub stippled_smooth_lines: VkBool32,
}
impl Default for VkPhysicalDeviceLineRasterizationFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT,
      next: core::ptr::null_mut(),
      rectangular_lines: Default::default(),
      bresenham_lines: Default::default(),
      smooth_lines: Default::default(),
      stippled_rectangular_lines: Default::default(),
      stippled_bresenham_lines: Default::default(),
      stippled_smooth_lines: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLineRasterizationPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bits
  pub line_sub_pixel_precision_bits: u32,
}
impl Default for VkPhysicalDeviceLineRasterizationPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      line_sub_pixel_precision_bits: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLinearColorAttachmentFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub linear_color_attachment: VkBool32,
}
impl Default for VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV,
      next: core::ptr::null_mut(),
      linear_color_attachment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMaintenance3Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance3Properties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance3Properties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_per_set_descriptors: u32,
  /// * Limit Type: max
  pub max_memory_allocation_size: VkDeviceSize,
}
impl Default for VkPhysicalDeviceMaintenance3Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES,
      next: core::ptr::null_mut(),
      max_per_set_descriptors: Default::default(),
      max_memory_allocation_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMaintenance4Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Features.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Features {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub maintenance_4: VkBool32,
}
impl Default for VkPhysicalDeviceMaintenance4Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES,
      next: core::ptr::null_mut(),
      maintenance_4: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMaintenance4Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Properties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Properties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_buffer_size: VkDeviceSize,
}
impl Default for VkPhysicalDeviceMaintenance4Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES,
      next: core::ptr::null_mut(),
      max_buffer_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryBudgetPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceMemoryProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub heap_budget: [VkDeviceSize; VK_MAX_MEMORY_HEAPS],
  pub heap_usage: [VkDeviceSize; VK_MAX_MEMORY_HEAPS],
}
impl Default for VkPhysicalDeviceMemoryBudgetPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      heap_budget: [Default::default(); VK_MAX_MEMORY_HEAPS],
      heap_usage: [Default::default(); VK_MAX_MEMORY_HEAPS],
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryDecompressionFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryDecompressionFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_decompression: VkBool32,
}
impl Default for VkPhysicalDeviceMemoryDecompressionFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV,
      next: core::ptr::null_mut(),
      memory_decompression: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryDecompressionPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryDecompressionPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub decompression_methods: VkMemoryDecompressionMethodFlagsNV,
  /// * Limit Type: max
  pub max_decompression_indirect_count: u64,
}
impl Default for VkPhysicalDeviceMemoryDecompressionPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      decompression_methods: Default::default(),
      max_decompression_indirect_count: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryPriorityFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_priority: VkBool32,
}
impl Default for VkPhysicalDeviceMemoryPriorityFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT,
      next: core::ptr::null_mut(),
      memory_priority: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
  pub memory_type_count: u32,
  pub memory_types: [VkMemoryType; VK_MAX_MEMORY_TYPES],
  pub memory_heap_count: u32,
  pub memory_heaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}
impl Default for VkPhysicalDeviceMemoryProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      memory_type_count: Default::default(),
      memory_types: [Default::default(); VK_MAX_MEMORY_TYPES],
      memory_heap_count: Default::default(),
      memory_heaps: [Default::default(); VK_MAX_MEMORY_HEAPS],
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_properties: VkPhysicalDeviceMemoryProperties,
}
impl Default for VkPhysicalDeviceMemoryProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2,
      next: core::ptr::null_mut(),
      memory_properties: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMeshShaderFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub task_shader: VkBool32,
  pub mesh_shader: VkBool32,
  pub multiview_mesh_shader: VkBool32,
  pub primitive_fragment_shading_rate_mesh_shader: VkBool32,
  pub mesh_shader_queries: VkBool32,
}
impl Default for VkPhysicalDeviceMeshShaderFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT,
      next: core::ptr::null_mut(),
      task_shader: Default::default(),
      mesh_shader: Default::default(),
      multiview_mesh_shader: Default::default(),
      primitive_fragment_shading_rate_mesh_shader: Default::default(),
      mesh_shader_queries: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMeshShaderFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub task_shader: VkBool32,
  pub mesh_shader: VkBool32,
}
impl Default for VkPhysicalDeviceMeshShaderFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV,
      next: core::ptr::null_mut(),
      task_shader: Default::default(),
      mesh_shader: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMeshShaderPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_task_work_group_total_count: u32,
  /// * Limit Type: max
  pub max_task_work_group_count: [u32; 3],
  /// * Limit Type: max
  pub max_task_work_group_invocations: u32,
  /// * Limit Type: max
  pub max_task_work_group_size: [u32; 3],
  /// * Limit Type: max
  pub max_task_payload_size: u32,
  /// * Limit Type: max
  pub max_task_shared_memory_size: u32,
  /// * Limit Type: max
  pub max_task_payload_and_shared_memory_size: u32,
  /// * Limit Type: max
  pub max_mesh_work_group_total_count: u32,
  /// * Limit Type: max
  pub max_mesh_work_group_count: [u32; 3],
  /// * Limit Type: max
  pub max_mesh_work_group_invocations: u32,
  /// * Limit Type: max
  pub max_mesh_work_group_size: [u32; 3],
  /// * Limit Type: max
  pub max_mesh_shared_memory_size: u32,
  /// * Limit Type: max
  pub max_mesh_payload_and_shared_memory_size: u32,
  /// * Limit Type: max
  pub max_mesh_output_memory_size: u32,
  /// * Limit Type: max
  pub max_mesh_payload_and_output_memory_size: u32,
  /// * Limit Type: max
  pub max_mesh_output_components: u32,
  /// * Limit Type: max
  pub max_mesh_output_vertices: u32,
  /// * Limit Type: max
  pub max_mesh_output_primitives: u32,
  /// * Limit Type: max
  pub max_mesh_output_layers: u32,
  /// * Limit Type: max
  pub max_mesh_multiview_view_count: u32,
  /// * Limit Type: noauto
  pub mesh_output_per_vertex_granularity: u32,
  /// * Limit Type: noauto
  pub mesh_output_per_primitive_granularity: u32,
  /// * Limit Type: max
  pub max_preferred_task_work_group_invocations: u32,
  /// * Limit Type: max
  pub max_preferred_mesh_work_group_invocations: u32,
  /// * Limit Type: noauto
  pub prefers_local_invocation_vertex_output: VkBool32,
  /// * Limit Type: noauto
  pub prefers_local_invocation_primitive_output: VkBool32,
  /// * Limit Type: noauto
  pub prefers_compact_vertex_output: VkBool32,
  /// * Limit Type: noauto
  pub prefers_compact_primitive_output: VkBool32,
}
impl Default for VkPhysicalDeviceMeshShaderPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      max_task_work_group_total_count: Default::default(),
      max_task_work_group_count: [Default::default(); 3],
      max_task_work_group_invocations: Default::default(),
      max_task_work_group_size: [Default::default(); 3],
      max_task_payload_size: Default::default(),
      max_task_shared_memory_size: Default::default(),
      max_task_payload_and_shared_memory_size: Default::default(),
      max_mesh_work_group_total_count: Default::default(),
      max_mesh_work_group_count: [Default::default(); 3],
      max_mesh_work_group_invocations: Default::default(),
      max_mesh_work_group_size: [Default::default(); 3],
      max_mesh_shared_memory_size: Default::default(),
      max_mesh_payload_and_shared_memory_size: Default::default(),
      max_mesh_output_memory_size: Default::default(),
      max_mesh_payload_and_output_memory_size: Default::default(),
      max_mesh_output_components: Default::default(),
      max_mesh_output_vertices: Default::default(),
      max_mesh_output_primitives: Default::default(),
      max_mesh_output_layers: Default::default(),
      max_mesh_multiview_view_count: Default::default(),
      mesh_output_per_vertex_granularity: Default::default(),
      mesh_output_per_primitive_granularity: Default::default(),
      max_preferred_task_work_group_invocations: Default::default(),
      max_preferred_mesh_work_group_invocations: Default::default(),
      prefers_local_invocation_vertex_output: Default::default(),
      prefers_local_invocation_primitive_output: Default::default(),
      prefers_compact_vertex_output: Default::default(),
      prefers_compact_primitive_output: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMeshShaderPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_draw_mesh_tasks_count: u32,
  /// * Limit Type: max
  pub max_task_work_group_invocations: u32,
  /// * Limit Type: max
  pub max_task_work_group_size: [u32; 3],
  /// * Limit Type: max
  pub max_task_total_memory_size: u32,
  /// * Limit Type: max
  pub max_task_output_count: u32,
  /// * Limit Type: max
  pub max_mesh_work_group_invocations: u32,
  /// * Limit Type: max
  pub max_mesh_work_group_size: [u32; 3],
  /// * Limit Type: max
  pub max_mesh_total_memory_size: u32,
  /// * Limit Type: max
  pub max_mesh_output_vertices: u32,
  /// * Limit Type: max
  pub max_mesh_output_primitives: u32,
  /// * Limit Type: max
  pub max_mesh_multiview_view_count: u32,
  /// * Limit Type: min,mul
  pub mesh_output_per_vertex_granularity: u32,
  /// * Limit Type: min,mul
  pub mesh_output_per_primitive_granularity: u32,
}
impl Default for VkPhysicalDeviceMeshShaderPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      max_draw_mesh_tasks_count: Default::default(),
      max_task_work_group_invocations: Default::default(),
      max_task_work_group_size: [Default::default(); 3],
      max_task_total_memory_size: Default::default(),
      max_task_output_count: Default::default(),
      max_mesh_work_group_invocations: Default::default(),
      max_mesh_work_group_size: [Default::default(); 3],
      max_mesh_total_memory_size: Default::default(),
      max_mesh_output_vertices: Default::default(),
      max_mesh_output_primitives: Default::default(),
      max_mesh_multiview_view_count: Default::default(),
      mesh_output_per_vertex_granularity: Default::default(),
      mesh_output_per_primitive_granularity: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiDrawFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub multi_draw: VkBool32,
}
impl Default for VkPhysicalDeviceMultiDrawFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT,
      next: core::ptr::null_mut(),
      multi_draw: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiDrawPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_multi_draw_count: u32,
}
impl Default for VkPhysicalDeviceMultiDrawPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      max_multi_draw_count: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub multisampled_render_to_single_sampled: VkBool32,
}
impl Default for VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT,
      next: core::ptr::null_mut(),
      multisampled_render_to_single_sampled: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// Multiple views in a renderpass
  pub multiview: VkBool32,
  /// Multiple views in a renderpass w/ geometry shader
  pub multiview_geometry_shader: VkBool32,
  /// Multiple views in a renderpass w/ tessellation shader
  pub multiview_tessellation_shader: VkBool32,
}
impl Default for VkPhysicalDeviceMultiviewFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES,
      next: core::ptr::null_mut(),
      multiview: Default::default(),
      multiview_geometry_shader: Default::default(),
      multiview_tessellation_shader: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub per_view_position_all_components: VkBool32,
}
impl Default for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
      next: core::ptr::null_mut(),
      per_view_position_all_components: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub multiview_per_view_render_areas: VkBool32,
}
impl Default for VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM,
      next: core::ptr::null_mut(),
      multiview_per_view_render_areas: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub multiview_per_view_viewports: VkBool32,
}
impl Default for VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM,
      next: core::ptr::null_mut(),
      multiview_per_view_viewports: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// max number of views in a subpass
  /// * Limit Type: max
  pub max_multiview_view_count: u32,
  /// max instance index for a draw in a multiview subpass
  /// * Limit Type: max
  pub max_multiview_instance_index: u32,
}
impl Default for VkPhysicalDeviceMultiviewProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES,
      next: core::ptr::null_mut(),
      max_multiview_view_count: Default::default(),
      max_multiview_instance_index: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub mutable_descriptor_type: VkBool32,
}
impl Default for VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT,
      next: core::ptr::null_mut(),
      mutable_descriptor_type: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub non_seamless_cube_map: VkBool32,
}
impl Default for VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT,
      next: core::ptr::null_mut(),
      non_seamless_cube_map: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceOpacityMicromapFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpacityMicromapFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub micromap: VkBool32,
  pub micromap_capture_replay: VkBool32,
  pub micromap_host_commands: VkBool32,
}
impl Default for VkPhysicalDeviceOpacityMicromapFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT,
      next: core::ptr::null_mut(),
      micromap: Default::default(),
      micromap_capture_replay: Default::default(),
      micromap_host_commands: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceOpacityMicromapPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpacityMicromapPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_opacity_2_state_subdivision_level: u32,
  /// * Limit Type: max
  pub max_opacity_4_state_subdivision_level: u32,
}
impl Default for VkPhysicalDeviceOpacityMicromapPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      max_opacity_2_state_subdivision_level: Default::default(),
      max_opacity_4_state_subdivision_level: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceOpticalFlowFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpticalFlowFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub optical_flow: VkBool32,
}
impl Default for VkPhysicalDeviceOpticalFlowFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV,
      next: core::ptr::null_mut(),
      optical_flow: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceOpticalFlowPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpticalFlowPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub supported_output_grid_sizes: VkOpticalFlowGridSizeFlagsNV,
  /// * Limit Type: bitmask
  pub supported_hint_grid_sizes: VkOpticalFlowGridSizeFlagsNV,
  /// * Limit Type: noauto
  pub hint_supported: VkBool32,
  /// * Limit Type: noauto
  pub cost_supported: VkBool32,
  /// * Limit Type: noauto
  pub bidirectional_flow_supported: VkBool32,
  /// * Limit Type: noauto
  pub global_flow_supported: VkBool32,
  /// * Limit Type: noauto
  pub min_width: u32,
  /// * Limit Type: noauto
  pub min_height: u32,
  /// * Limit Type: noauto
  pub max_width: u32,
  /// * Limit Type: noauto
  pub max_height: u32,
  /// * Limit Type: noauto
  pub max_num_regions_of_interest: u32,
}
impl Default for VkPhysicalDeviceOpticalFlowPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      supported_output_grid_sizes: Default::default(),
      supported_hint_grid_sizes: Default::default(),
      hint_supported: Default::default(),
      cost_supported: Default::default(),
      bidirectional_flow_supported: Default::default(),
      global_flow_supported: Default::default(),
      min_width: Default::default(),
      min_height: Default::default(),
      max_width: Default::default(),
      max_height: Default::default(),
      max_num_regions_of_interest: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePCIBusInfoPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: noauto
  pub pci_domain: u32,
  /// * Limit Type: noauto
  pub pci_bus: u32,
  /// * Limit Type: noauto
  pub pci_device: u32,
  /// * Limit Type: noauto
  pub pci_function: u32,
}
impl Default for VkPhysicalDevicePCIBusInfoPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      pci_domain: Default::default(),
      pci_bus: Default::default(),
      pci_device: Default::default(),
      pci_function: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub pageable_device_local_memory: VkBool32,
}
impl Default for VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT,
      next: core::ptr::null_mut(),
      pageable_device_local_memory: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePerformanceQueryFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// performance counters supported in query pools
  pub performance_counter_query_pools: VkBool32,
  /// performance counters from multiple query pools can be accessed in the same
  /// primary command buffer
  pub performance_counter_multiple_query_pools: VkBool32,
}
impl Default for VkPhysicalDevicePerformanceQueryFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR,
      next: core::ptr::null_mut(),
      performance_counter_query_pools: Default::default(),
      performance_counter_multiple_query_pools: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePerformanceQueryPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// Flag to specify whether performance queries are allowed to be used in
  /// vkCmdCopyQueryPoolResults
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub allow_command_buffer_query_copies: VkBool32,
}
impl Default for VkPhysicalDevicePerformanceQueryPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      allow_command_buffer_query_copies: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineCreationCacheControlFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineCreationCacheControlFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub pipeline_creation_cache_control: VkBool32,
}
impl Default for VkPhysicalDevicePipelineCreationCacheControlFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES,
      next: core::ptr::null_mut(),
      pipeline_creation_cache_control: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub pipeline_executable_info: VkBool32,
}
impl Default for VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR,
      next: core::ptr::null_mut(),
      pipeline_executable_info: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub pipeline_library_group_handles: VkBool32,
}
impl Default for VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT,
      next: core::ptr::null_mut(),
      pipeline_library_group_handles: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelinePropertiesFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelinePropertiesFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePipelinePropertiesFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub pipeline_properties_identifier: VkBool32,
}
impl Default for VkPhysicalDevicePipelinePropertiesFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT,
      next: core::ptr::null_mut(),
      pipeline_properties_identifier: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineProtectedAccessFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineProtectedAccessFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineProtectedAccessFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub pipeline_protected_access: VkBool32,
}
impl Default for VkPhysicalDevicePipelineProtectedAccessFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT,
      next: core::ptr::null_mut(),
      pipeline_protected_access: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineRobustnessFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineRobustnessFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub pipeline_robustness: VkBool32,
}
impl Default for VkPhysicalDevicePipelineRobustnessFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT,
      next: core::ptr::null_mut(),
      pipeline_robustness: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineRobustnessPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineRobustnessPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub default_robustness_storage_buffers: VkPipelineRobustnessBufferBehaviorEXT,
  /// * Limit Type: exact
  pub default_robustness_uniform_buffers: VkPipelineRobustnessBufferBehaviorEXT,
  /// * Limit Type: exact
  pub default_robustness_vertex_inputs: VkPipelineRobustnessBufferBehaviorEXT,
  /// * Limit Type: exact
  pub default_robustness_images: VkPipelineRobustnessImageBehaviorEXT,
}
impl Default for VkPhysicalDevicePipelineRobustnessPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      default_robustness_storage_buffers: Default::default(),
      default_robustness_uniform_buffers: Default::default(),
      default_robustness_vertex_inputs: Default::default(),
      default_robustness_images: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePointClippingProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePointClippingProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePointClippingProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub point_clipping_behavior: VkPointClippingBehavior,
}
impl Default for VkPhysicalDevicePointClippingProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES,
      next: core::ptr::null_mut(),
      point_clipping_behavior: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePortabilitySubsetFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub constant_alpha_color_blend_factors: VkBool32,
  pub events: VkBool32,
  pub image_view_format_reinterpretation: VkBool32,
  pub image_view_format_swizzle: VkBool32,
  pub image_view_2_d_on_3_d_image: VkBool32,
  pub multisample_array_image: VkBool32,
  pub mutable_comparison_samplers: VkBool32,
  pub point_polygons: VkBool32,
  pub sampler_mip_lod_bias: VkBool32,
  pub separate_stencil_mask_ref: VkBool32,
  pub shader_sample_rate_interpolation_functions: VkBool32,
  pub tessellation_isolines: VkBool32,
  pub tessellation_point_mode: VkBool32,
  pub triangle_fans: VkBool32,
  pub vertex_attribute_access_beyond_stride: VkBool32,
}
impl Default for VkPhysicalDevicePortabilitySubsetFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR,
      next: core::ptr::null_mut(),
      constant_alpha_color_blend_factors: Default::default(),
      events: Default::default(),
      image_view_format_reinterpretation: Default::default(),
      image_view_format_swizzle: Default::default(),
      image_view_2_d_on_3_d_image: Default::default(),
      multisample_array_image: Default::default(),
      mutable_comparison_samplers: Default::default(),
      point_polygons: Default::default(),
      sampler_mip_lod_bias: Default::default(),
      separate_stencil_mask_ref: Default::default(),
      shader_sample_rate_interpolation_functions: Default::default(),
      tessellation_isolines: Default::default(),
      tessellation_point_mode: Default::default(),
      triangle_fans: Default::default(),
      vertex_attribute_access_beyond_stride: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePortabilitySubsetPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePortabilitySubsetPropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetPropertiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: min,pot
  pub min_vertex_input_binding_stride_alignment: u32,
}
impl Default for VkPhysicalDevicePortabilitySubsetPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      min_vertex_input_binding_stride_alignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePresentBarrierFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentBarrierFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePresentBarrierFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub present_barrier: VkBool32,
}
impl Default for VkPhysicalDevicePresentBarrierFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV,
      next: core::ptr::null_mut(),
      present_barrier: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePresentIdFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentIdFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePresentIdFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// Present ID in VkPresentInfoKHR
  pub present_id: VkBool32,
}
impl Default for VkPhysicalDevicePresentIdFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR,
      next: core::ptr::null_mut(),
      present_id: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePresentWaitFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentWaitFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePresentWaitFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// vkWaitForPresentKHR is supported
  pub present_wait: VkBool32,
}
impl Default for VkPhysicalDevicePresentWaitFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR,
      next: core::ptr::null_mut(),
      present_wait: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePresentationPropertiesANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentationPropertiesANDROID.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePresentationPropertiesANDROID {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub shared_image: VkBool32,
}
impl Default for VkPhysicalDevicePresentationPropertiesANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID,
      next: core::ptr::null(),
      shared_image: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub primitive_topology_list_restart: VkBool32,
  pub primitive_topology_patch_list_restart: VkBool32,
}
impl Default for VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT,
      next: core::ptr::null_mut(),
      primitive_topology_list_restart: Default::default(),
      primitive_topology_patch_list_restart: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub primitives_generated_query: VkBool32,
  pub primitives_generated_query_with_rasterizer_discard: VkBool32,
  pub primitives_generated_query_with_non_zero_streams: VkBool32,
}
impl Default for VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT,
      next: core::ptr::null_mut(),
      primitives_generated_query: Default::default(),
      primitives_generated_query_with_rasterizer_discard: Default::default(),
      primitives_generated_query_with_non_zero_streams: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePrivateDataFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrivateDataFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePrivateDataFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub private_data: VkBool32,
}
impl Default for VkPhysicalDevicePrivateDataFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES,
      next: core::ptr::null_mut(),
      private_data: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
  /// * Limit Type: noauto
  pub api_version: u32,
  /// * Limit Type: noauto
  pub driver_version: u32,
  /// * Limit Type: noauto
  pub vendor_id: u32,
  /// * Limit Type: noauto
  pub device_id: u32,
  /// * Limit Type: noauto
  pub device_type: VkPhysicalDeviceType,
  /// * Limit Type: noauto
  pub device_name: zstring::ArrayZString<VK_MAX_PHYSICAL_DEVICE_NAME_SIZE>,
  /// * Limit Type: noauto
  pub pipeline_cache_uuid: [u8; VK_UUID_SIZE],
  /// * Limit Type: struct
  pub limits: VkPhysicalDeviceLimits,
  /// * Limit Type: struct
  pub sparse_properties: VkPhysicalDeviceSparseProperties,
}
impl Default for VkPhysicalDeviceProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      api_version: Default::default(),
      driver_version: Default::default(),
      vendor_id: Default::default(),
      device_id: Default::default(),
      device_type: Default::default(),
      device_name: Default::default(),
      pipeline_cache_uuid: [Default::default(); VK_UUID_SIZE],
      limits: Default::default(),
      sparse_properties: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties2.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: struct
  pub properties: VkPhysicalDeviceProperties,
}
impl Default for VkPhysicalDeviceProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2,
      next: core::ptr::null_mut(),
      properties: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProtectedMemoryFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub protected_memory: VkBool32,
}
impl Default for VkPhysicalDeviceProtectedMemoryFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
      next: core::ptr::null_mut(),
      protected_memory: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProtectedMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub protected_no_fault: VkBool32,
}
impl Default for VkPhysicalDeviceProtectedMemoryProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES,
      next: core::ptr::null_mut(),
      protected_no_fault: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProvokingVertexFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub provoking_vertex_last: VkBool32,
  pub transform_feedback_preserves_provoking_vertex: VkBool32,
}
impl Default for VkPhysicalDeviceProvokingVertexFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT,
      next: core::ptr::null_mut(),
      provoking_vertex_last: Default::default(),
      transform_feedback_preserves_provoking_vertex: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProvokingVertexPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub provoking_vertex_mode_per_pipeline: VkBool32,
  /// * Limit Type: bitmask
  pub transform_feedback_preserves_triangle_fan_provoking_vertex: VkBool32,
}
impl Default for VkPhysicalDeviceProvokingVertexPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      provoking_vertex_mode_per_pipeline: Default::default(),
      transform_feedback_preserves_triangle_fan_provoking_vertex: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePushDescriptorPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_push_descriptors: u32,
}
impl Default for VkPhysicalDevicePushDescriptorPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      max_push_descriptors: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub format_rgba_10_x_6_without_y_cb_cr_sampler: VkBool32,
}
impl Default for VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT,
      next: core::ptr::null_mut(),
      format_rgba_10_x_6_without_y_cb_cr_sampler: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub rasterization_order_color_attachment_access: VkBool32,
  pub rasterization_order_depth_attachment_access: VkBool32,
  pub rasterization_order_stencil_attachment_access: VkBool32,
}
impl Default for VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT,
      next: core::ptr::null_mut(),
      rasterization_order_color_attachment_access: Default::default(),
      rasterization_order_depth_attachment_access: Default::default(),
      rasterization_order_stencil_attachment_access: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayQueryFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRayQueryFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub ray_query: VkBool32,
}
impl Default for VkPhysicalDeviceRayQueryFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR,
      next: core::ptr::null_mut(),
      ray_query: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub ray_tracing_invocation_reorder: VkBool32,
}
impl Default for VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV,
      next: core::ptr::null_mut(),
      ray_tracing_invocation_reorder: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  /// * Limit Type: noauto
  pub ray_tracing_invocation_reorder_reordering_hint: VkRayTracingInvocationReorderModeNV,
}
impl Default for VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      ray_tracing_invocation_reorder_reordering_hint: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub ray_tracing_maintenance_1: VkBool32,
  pub ray_tracing_pipeline_trace_rays_indirect_2: VkBool32,
}
impl Default for VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR,
      next: core::ptr::null_mut(),
      ray_tracing_maintenance_1: Default::default(),
      ray_tracing_pipeline_trace_rays_indirect_2: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingMotionBlurFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingMotionBlurFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub ray_tracing_motion_blur: VkBool32,
  pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: VkBool32,
}
impl Default for VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV,
      next: core::ptr::null_mut(),
      ray_tracing_motion_blur: Default::default(),
      ray_tracing_motion_blur_pipeline_trace_rays_indirect: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingPipelineFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub ray_tracing_pipeline: VkBool32,
  pub ray_tracing_pipeline_shader_group_handle_capture_replay: VkBool32,
  pub ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: VkBool32,
  pub ray_tracing_pipeline_trace_rays_indirect: VkBool32,
  pub ray_traversal_primitive_culling: VkBool32,
}
impl Default for VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR,
      next: core::ptr::null_mut(),
      ray_tracing_pipeline: Default::default(),
      ray_tracing_pipeline_shader_group_handle_capture_replay: Default::default(),
      ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Default::default(),
      ray_tracing_pipeline_trace_rays_indirect: Default::default(),
      ray_traversal_primitive_culling: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingPipelinePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub shader_group_handle_size: u32,
  /// * Limit Type: max
  pub max_ray_recursion_depth: u32,
  /// * Limit Type: max
  pub max_shader_group_stride: u32,
  /// * Limit Type: exact
  pub shader_group_base_alignment: u32,
  /// * Limit Type: exact
  pub shader_group_handle_capture_replay_size: u32,
  /// * Limit Type: max
  pub max_ray_dispatch_invocation_count: u32,
  /// * Limit Type: min,pot
  pub shader_group_handle_alignment: u32,
  /// * Limit Type: max
  pub max_ray_hit_attribute_size: u32,
}
impl Default for VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      shader_group_handle_size: Default::default(),
      max_ray_recursion_depth: Default::default(),
      max_shader_group_stride: Default::default(),
      shader_group_base_alignment: Default::default(),
      shader_group_handle_capture_replay_size: Default::default(),
      max_ray_dispatch_invocation_count: Default::default(),
      shader_group_handle_alignment: Default::default(),
      max_ray_hit_attribute_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub shader_group_handle_size: u32,
  /// * Limit Type: max
  pub max_recursion_depth: u32,
  /// * Limit Type: max
  pub max_shader_group_stride: u32,
  /// * Limit Type: exact
  pub shader_group_base_alignment: u32,
  /// * Limit Type: max
  pub max_geometry_count: u64,
  /// * Limit Type: max
  pub max_instance_count: u64,
  /// * Limit Type: max
  pub max_triangle_count: u64,
  /// * Limit Type: max
  pub max_descriptor_set_acceleration_structures: u32,
}
impl Default for VkPhysicalDeviceRayTracingPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      shader_group_handle_size: Default::default(),
      max_recursion_depth: Default::default(),
      max_shader_group_stride: Default::default(),
      shader_group_base_alignment: Default::default(),
      max_geometry_count: Default::default(),
      max_instance_count: Default::default(),
      max_triangle_count: Default::default(),
      max_descriptor_set_acceleration_structures: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub representative_fragment_test: VkBool32,
}
impl Default for VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV,
      next: core::ptr::null_mut(),
      representative_fragment_test: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRobustness2FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRobustness2FeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub robust_buffer_access_2: VkBool32,
  pub robust_image_access_2: VkBool32,
  pub null_descriptor: VkBool32,
}
impl Default for VkPhysicalDeviceRobustness2FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT,
      next: core::ptr::null_mut(),
      robust_buffer_access_2: Default::default(),
      robust_image_access_2: Default::default(),
      null_descriptor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRobustness2PropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceRobustness2PropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: min,pot
  pub robust_storage_buffer_access_size_alignment: VkDeviceSize,
  /// * Limit Type: min,pot
  pub robust_uniform_buffer_access_size_alignment: VkDeviceSize,
}
impl Default for VkPhysicalDeviceRobustness2PropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      robust_storage_buffer_access_size_alignment: Default::default(),
      robust_uniform_buffer_access_size_alignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSampleLocationsPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub sample_location_sample_counts: VkSampleCountFlags,
  /// * Limit Type: max
  pub max_sample_location_grid_size: VkExtent2D,
  /// * Limit Type: range
  pub sample_location_coordinate_range: [c_float; 2],
  /// * Limit Type: bits
  pub sample_location_sub_pixel_bits: u32,
  /// * Limit Type: bitmask
  pub variable_sample_locations: VkBool32,
}
impl Default for VkPhysicalDeviceSampleLocationsPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      sample_location_sample_counts: Default::default(),
      max_sample_location_grid_size: Default::default(),
      sample_location_coordinate_range: [Default::default(); 2],
      sample_location_sub_pixel_bits: Default::default(),
      variable_sample_locations: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSamplerFilterMinmaxProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub filter_minmax_single_component_formats: VkBool32,
  /// * Limit Type: bitmask
  pub filter_minmax_image_component_mapping: VkBool32,
}
impl Default for VkPhysicalDeviceSamplerFilterMinmaxProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES,
      next: core::ptr::null_mut(),
      filter_minmax_single_component_formats: Default::default(),
      filter_minmax_image_component_mapping: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSamplerYcbcrConversionFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// Sampler color conversion supported
  pub sampler_ycbcr_conversion: VkBool32,
}
impl Default for VkPhysicalDeviceSamplerYcbcrConversionFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
      next: core::ptr::null_mut(),
      sampler_ycbcr_conversion: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceScalarBlockLayoutFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub scalar_block_layout: VkBool32,
}
impl Default for VkPhysicalDeviceScalarBlockLayoutFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES,
      next: core::ptr::null_mut(),
      scalar_block_layout: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub separate_depth_stencil_layouts: VkBool32,
}
impl Default for VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES,
      next: core::ptr::null_mut(),
      separate_depth_stencil_layouts: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_buffer_float_16_atomics: VkBool32,
  pub shader_buffer_float_16_atomic_add: VkBool32,
  pub shader_buffer_float_16_atomic_min_max: VkBool32,
  pub shader_buffer_float_32_atomic_min_max: VkBool32,
  pub shader_buffer_float_64_atomic_min_max: VkBool32,
  pub shader_shared_float_16_atomics: VkBool32,
  pub shader_shared_float_16_atomic_add: VkBool32,
  pub shader_shared_float_16_atomic_min_max: VkBool32,
  pub shader_shared_float_32_atomic_min_max: VkBool32,
  pub shader_shared_float_64_atomic_min_max: VkBool32,
  pub shader_image_float_32_atomic_min_max: VkBool32,
  pub sparse_image_float_32_atomic_min_max: VkBool32,
}
impl Default for VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT,
      next: core::ptr::null_mut(),
      shader_buffer_float_16_atomics: Default::default(),
      shader_buffer_float_16_atomic_add: Default::default(),
      shader_buffer_float_16_atomic_min_max: Default::default(),
      shader_buffer_float_32_atomic_min_max: Default::default(),
      shader_buffer_float_64_atomic_min_max: Default::default(),
      shader_shared_float_16_atomics: Default::default(),
      shader_shared_float_16_atomic_add: Default::default(),
      shader_shared_float_16_atomic_min_max: Default::default(),
      shader_shared_float_32_atomic_min_max: Default::default(),
      shader_shared_float_64_atomic_min_max: Default::default(),
      shader_image_float_32_atomic_min_max: Default::default(),
      sparse_image_float_32_atomic_min_max: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderAtomicFloatFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_buffer_float_32_atomics: VkBool32,
  pub shader_buffer_float_32_atomic_add: VkBool32,
  pub shader_buffer_float_64_atomics: VkBool32,
  pub shader_buffer_float_64_atomic_add: VkBool32,
  pub shader_shared_float_32_atomics: VkBool32,
  pub shader_shared_float_32_atomic_add: VkBool32,
  pub shader_shared_float_64_atomics: VkBool32,
  pub shader_shared_float_64_atomic_add: VkBool32,
  pub shader_image_float_32_atomics: VkBool32,
  pub shader_image_float_32_atomic_add: VkBool32,
  pub sparse_image_float_32_atomics: VkBool32,
  pub sparse_image_float_32_atomic_add: VkBool32,
}
impl Default for VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT,
      next: core::ptr::null_mut(),
      shader_buffer_float_32_atomics: Default::default(),
      shader_buffer_float_32_atomic_add: Default::default(),
      shader_buffer_float_64_atomics: Default::default(),
      shader_buffer_float_64_atomic_add: Default::default(),
      shader_shared_float_32_atomics: Default::default(),
      shader_shared_float_32_atomic_add: Default::default(),
      shader_shared_float_64_atomics: Default::default(),
      shader_shared_float_64_atomic_add: Default::default(),
      shader_image_float_32_atomics: Default::default(),
      shader_image_float_32_atomic_add: Default::default(),
      sparse_image_float_32_atomics: Default::default(),
      sparse_image_float_32_atomic_add: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderAtomicInt64Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicInt64Features.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_buffer_int_64_atomics: VkBool32,
  pub shader_shared_int_64_atomics: VkBool32,
}
impl Default for VkPhysicalDeviceShaderAtomicInt64Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES,
      next: core::ptr::null_mut(),
      shader_buffer_int_64_atomics: Default::default(),
      shader_shared_int_64_atomics: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderClockFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderClockFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_subgroup_clock: VkBool32,
  pub shader_device_clock: VkBool32,
}
impl Default for VkPhysicalDeviceShaderClockFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR,
      next: core::ptr::null_mut(),
      shader_subgroup_clock: Default::default(),
      shader_device_clock: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_core_builtins: VkBool32,
}
impl Default for VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM,
      next: core::ptr::null_mut(),
      shader_core_builtins: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub shader_core_mask: u64,
  /// * Limit Type: max
  pub shader_core_count: u32,
  /// * Limit Type: max
  pub shader_warps_per_core: u32,
}
impl Default for VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM,
      next: core::ptr::null_mut(),
      shader_core_mask: Default::default(),
      shader_core_count: Default::default(),
      shader_warps_per_core: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCoreProperties2AMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreProperties2AMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD`
  pub struct_ty: VkStructureType,
  /// Pointer to next structure
  /// * Optional: true
  pub next: *mut c_void,
  /// features supported by the shader core
  /// * Limit Type: bitmask
  pub shader_core_features: VkShaderCorePropertiesFlagsAMD,
  /// number of active compute units across all shader engines/arrays
  /// * Limit Type: max
  pub active_compute_unit_count: u32,
}
impl Default for VkPhysicalDeviceShaderCoreProperties2AMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD,
      next: core::ptr::null_mut(),
      shader_core_features: Default::default(),
      active_compute_unit_count: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCorePropertiesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesAMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// number of shader engines
  /// * Limit Type: exact
  pub shader_engine_count: u32,
  /// number of shader arrays
  /// * Limit Type: exact
  pub shader_arrays_per_engine_count: u32,
  /// number of physical CUs per shader array
  /// * Limit Type: exact
  pub compute_units_per_shader_array: u32,
  /// number of SIMDs per compute unit
  /// * Limit Type: exact
  pub simd_per_compute_unit: u32,
  /// number of wavefront slots in each SIMD
  /// * Limit Type: exact
  pub wavefronts_per_simd: u32,
  /// maximum number of threads per wavefront
  /// * Limit Type: max
  pub wavefront_size: u32,
  /// number of physical SGPRs per SIMD
  /// * Limit Type: exact
  pub sgprs_per_simd: u32,
  /// minimum number of SGPRs that can be allocated by a wave
  /// * Limit Type: min
  pub min_sgpr_allocation: u32,
  /// number of available SGPRs
  /// * Limit Type: max
  pub max_sgpr_allocation: u32,
  /// SGPRs are allocated in groups of this size
  /// * Limit Type: min,mul
  pub sgpr_allocation_granularity: u32,
  /// number of physical VGPRs per SIMD
  /// * Limit Type: exact
  pub vgprs_per_simd: u32,
  /// minimum number of VGPRs that can be allocated by a wave
  /// * Limit Type: min
  pub min_vgpr_allocation: u32,
  /// number of available VGPRs
  /// * Limit Type: max
  pub max_vgpr_allocation: u32,
  /// VGPRs are allocated in groups of this size
  /// * Limit Type: min,mul
  pub vgpr_allocation_granularity: u32,
}
impl Default for VkPhysicalDeviceShaderCorePropertiesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD,
      next: core::ptr::null_mut(),
      shader_engine_count: Default::default(),
      shader_arrays_per_engine_count: Default::default(),
      compute_units_per_shader_array: Default::default(),
      simd_per_compute_unit: Default::default(),
      wavefronts_per_simd: Default::default(),
      wavefront_size: Default::default(),
      sgprs_per_simd: Default::default(),
      min_sgpr_allocation: Default::default(),
      max_sgpr_allocation: Default::default(),
      sgpr_allocation_granularity: Default::default(),
      vgprs_per_simd: Default::default(),
      min_vgpr_allocation: Default::default(),
      max_vgpr_allocation: Default::default(),
      vgpr_allocation_granularity: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCorePropertiesARM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesARM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesARM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub pixel_rate: u32,
  /// * Limit Type: exact
  pub texel_rate: u32,
  /// * Limit Type: exact
  pub fma_rate: u32,
}
impl Default for VkPhysicalDeviceShaderCorePropertiesARM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM,
      next: core::ptr::null_mut(),
      pixel_rate: Default::default(),
      texel_rate: Default::default(),
      fma_rate: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_demote_to_helper_invocation: VkBool32,
}
impl Default for VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES,
      next: core::ptr::null_mut(),
      shader_demote_to_helper_invocation: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderDrawParametersFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_draw_parameters: VkBool32,
}
impl Default for VkPhysicalDeviceShaderDrawParametersFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES,
      next: core::ptr::null_mut(),
      shader_draw_parameters: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub shader_early_and_late_fragment_tests: VkBool32,
}
impl Default for VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD,
      next: core::ptr::null_mut(),
      shader_early_and_late_fragment_tests: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderFloat16Int8Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderFloat16Int8Features.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// 16-bit floats (halfs) in shaders
  pub shader_float_16: VkBool32,
  /// 8-bit integers in shaders
  pub shader_int_8: VkBool32,
}
impl Default for VkPhysicalDeviceShaderFloat16Int8Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES,
      next: core::ptr::null_mut(),
      shader_float_16: Default::default(),
      shader_int_8: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_image_int_64_atomics: VkBool32,
  pub sparse_image_int_64_atomics: VkBool32,
}
impl Default for VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT,
      next: core::ptr::null_mut(),
      shader_image_int_64_atomics: Default::default(),
      sparse_image_int_64_atomics: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderImageFootprintFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub image_footprint: VkBool32,
}
impl Default for VkPhysicalDeviceShaderImageFootprintFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV,
      next: core::ptr::null_mut(),
      image_footprint: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderIntegerDotProductFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_integer_dot_product: VkBool32,
}
impl Default for VkPhysicalDeviceShaderIntegerDotProductFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES,
      next: core::ptr::null_mut(),
      shader_integer_dot_product: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderIntegerDotProductProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub integer_dot_product_8_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_8_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_8_bit_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_4_x_8_bit_packed_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_16_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_16_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_16_bit_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_32_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_32_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_32_bit_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_64_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_64_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_64_bit_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated:
    VkBool32,
}
impl Default for VkPhysicalDeviceShaderIntegerDotProductProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES,
      next: core::ptr::null_mut(),
      integer_dot_product_8_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_8_bit_signed_accelerated: Default::default(),
      integer_dot_product_8_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: Default::default(),
      integer_dot_product_4_x_8_bit_packed_signed_accelerated: Default::default(),
      integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_16_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_16_bit_signed_accelerated: Default::default(),
      integer_dot_product_16_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_32_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_32_bit_signed_accelerated: Default::default(),
      integer_dot_product_32_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_64_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_64_bit_signed_accelerated: Default::default(),
      integer_dot_product_64_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_integer_functions_2: VkBool32,
}
impl Default for VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL,
      next: core::ptr::null_mut(),
      shader_integer_functions_2: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub shader_module_identifier: VkBool32,
}
impl Default for VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT,
      next: core::ptr::null_mut(),
      shader_module_identifier: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  /// * Limit Type: noauto
  pub shader_module_identifier_algorithm_uuid: [u8; VK_UUID_SIZE],
}
impl Default for VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      shader_module_identifier_algorithm_uuid: [Default::default(); VK_UUID_SIZE],
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderSMBuiltinsFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_sm_builtins: VkBool32,
}
impl Default for VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV,
      next: core::ptr::null_mut(),
      shader_sm_builtins: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderSMBuiltinsPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub shader_sm_count: u32,
  /// * Limit Type: max
  pub shader_warps_per_sm: u32,
}
impl Default for VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      shader_sm_count: Default::default(),
      shader_warps_per_sm: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// Flag to specify whether subgroup operations with extended types are
  /// supported
  /// * No Auto-Validity
  pub shader_subgroup_extended_types: VkBool32,
}
impl Default for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES,
      next: core::ptr::null_mut(),
      shader_subgroup_extended_types: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_subgroup_uniform_control_flow: VkBool32,
}
impl Default for VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR,
      next: core::ptr::null_mut(),
      shader_subgroup_uniform_control_flow: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderTerminateInvocationFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderTerminateInvocationFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub shader_terminate_invocation: VkBool32,
}
impl Default for VkPhysicalDeviceShaderTerminateInvocationFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES,
      next: core::ptr::null_mut(),
      shader_terminate_invocation: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShadingRateImageFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShadingRateImageFeaturesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shading_rate_image: VkBool32,
  pub shading_rate_coarse_sample_order: VkBool32,
}
impl Default for VkPhysicalDeviceShadingRateImageFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV,
      next: core::ptr::null_mut(),
      shading_rate_image: Default::default(),
      shading_rate_coarse_sample_order: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShadingRateImagePropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShadingRateImagePropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub shading_rate_texel_size: VkExtent2D,
  /// * Limit Type: max
  pub shading_rate_palette_size: u32,
  /// * Limit Type: max
  pub shading_rate_max_coarse_samples: u32,
}
impl Default for VkPhysicalDeviceShadingRateImagePropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      shading_rate_texel_size: Default::default(),
      shading_rate_palette_size: Default::default(),
      shading_rate_max_coarse_samples: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSparseImageFormatInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2 {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub format: VkFormat,
  pub ty: VkImageType,
  pub samples: VkSampleCountFlagBits,
  pub usage: VkImageUsageFlags,
  pub tiling: VkImageTiling,
}
impl Default for VkPhysicalDeviceSparseImageFormatInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2,
      next: core::ptr::null(),
      format: Default::default(),
      ty: Default::default(),
      samples: Default::default(),
      usage: Default::default(),
      tiling: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSparseProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
  /// Sparse resources support: GPU will access all 2D (single sample) sparse
  /// resources using the standard sparse image block shapes (based on pixel
  /// format)
  /// * Limit Type: bitmask
  pub residency_standard_2_d_block_shape: VkBool32,
  /// Sparse resources support: GPU will access all 2D (multisample) sparse
  /// resources using the standard sparse image block shapes (based on pixel
  /// format)
  /// * Limit Type: bitmask
  pub residency_standard_2_d_multisample_block_shape: VkBool32,
  /// Sparse resources support: GPU will access all 3D sparse resources using
  /// the standard sparse image block shapes (based on pixel format)
  /// * Limit Type: bitmask
  pub residency_standard_3_d_block_shape: VkBool32,
  /// Sparse resources support: Images with mip level dimensions that are NOT a
  /// multiple of the sparse image block dimensions will be placed in the mip
  /// tail
  /// * Limit Type: bitmask
  pub residency_aligned_mip_size: VkBool32,
  /// Sparse resources support: GPU can consistently access non-resident regions
  /// of a resource, all reads return as if data is 0, writes are discarded
  /// * Limit Type: bitmask
  pub residency_non_resident_strict: VkBool32,
}
impl Default for VkPhysicalDeviceSparseProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      residency_standard_2_d_block_shape: Default::default(),
      residency_standard_2_d_multisample_block_shape: Default::default(),
      residency_standard_3_d_block_shape: Default::default(),
      residency_aligned_mip_size: Default::default(),
      residency_non_resident_strict: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubgroupProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSubgroupProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// The size of a subgroup for this queue.
  /// * Limit Type: max,pot
  /// * No Auto-Validity
  pub subgroup_size: u32,
  /// Bitfield of what shader stages support subgroup operations
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub supported_stages: VkShaderStageFlags,
  /// Bitfield of what subgroup operations are supported.
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub supported_operations: VkSubgroupFeatureFlags,
  /// Flag to specify whether quad operations are available in all stages.
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub quad_operations_in_all_stages: VkBool32,
}
impl Default for VkPhysicalDeviceSubgroupProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
      next: core::ptr::null_mut(),
      subgroup_size: Default::default(),
      supported_stages: Default::default(),
      supported_operations: Default::default(),
      quad_operations_in_all_stages: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubgroupSizeControlFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub subgroup_size_control: VkBool32,
  pub compute_full_subgroups: VkBool32,
}
impl Default for VkPhysicalDeviceSubgroupSizeControlFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES,
      next: core::ptr::null_mut(),
      subgroup_size_control: Default::default(),
      compute_full_subgroups: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubgroupSizeControlProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// The minimum subgroup size supported by this device
  /// * Limit Type: min,pot
  /// * No Auto-Validity
  pub min_subgroup_size: u32,
  /// The maximum subgroup size supported by this device
  /// * Limit Type: max,pot
  /// * No Auto-Validity
  pub max_subgroup_size: u32,
  /// The maximum number of subgroups supported in a workgroup
  /// * Limit Type: max
  /// * No Auto-Validity
  pub max_compute_workgroup_subgroups: u32,
  /// The shader stages that support specifying a subgroup size
  /// * Limit Type: bitmask
  pub required_subgroup_size_stages: VkShaderStageFlags,
}
impl Default for VkPhysicalDeviceSubgroupSizeControlProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES,
      next: core::ptr::null_mut(),
      min_subgroup_size: Default::default(),
      max_subgroup_size: Default::default(),
      max_compute_workgroup_subgroups: Default::default(),
      required_subgroup_size_stages: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub subpass_merge_feedback: VkBool32,
}
impl Default for VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT,
      next: core::ptr::null_mut(),
      subpass_merge_feedback: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubpassShadingFeaturesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub subpass_shading: VkBool32,
}
impl Default for VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI,
      next: core::ptr::null_mut(),
      subpass_shading: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubpassShadingPropertiesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max,pot
  pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
}
impl Default for VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI,
      next: core::ptr::null_mut(),
      max_subpass_shading_workgroup_size_aspect_ratio: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSurfaceInfo2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub surface: VkSurfaceKHR,
}
impl Default for VkPhysicalDeviceSurfaceInfo2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
      next: core::ptr::null(),
      surface: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub swapchain_maintenance_1: VkBool32,
}
impl Default for VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT,
      next: core::ptr::null_mut(),
      swapchain_maintenance_1: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSynchronization2Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSynchronization2Features.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceSynchronization2Features {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub synchronization_2: VkBool32,
}
impl Default for VkPhysicalDeviceSynchronization2Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
      next: core::ptr::null_mut(),
      synchronization_2: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub texel_buffer_alignment: VkBool32,
}
impl Default for VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT,
      next: core::ptr::null_mut(),
      texel_buffer_alignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTexelBufferAlignmentProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: min,pot
  pub storage_texel_buffer_offset_alignment_bytes: VkDeviceSize,
  /// * Limit Type: exact
  pub storage_texel_buffer_offset_single_texel_alignment: VkBool32,
  /// * Limit Type: min,pot
  pub uniform_texel_buffer_offset_alignment_bytes: VkDeviceSize,
  /// * Limit Type: exact
  pub uniform_texel_buffer_offset_single_texel_alignment: VkBool32,
}
impl Default for VkPhysicalDeviceTexelBufferAlignmentProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES,
      next: core::ptr::null_mut(),
      storage_texel_buffer_offset_alignment_bytes: Default::default(),
      storage_texel_buffer_offset_single_texel_alignment: Default::default(),
      uniform_texel_buffer_offset_alignment_bytes: Default::default(),
      uniform_texel_buffer_offset_single_texel_alignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTextureCompressionASTCHDRFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub texture_compression_astc_hdr: VkBool32,
}
impl Default for VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES,
      next: core::ptr::null_mut(),
      texture_compression_astc_hdr: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTilePropertiesFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTilePropertiesFeaturesQCOM.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceTilePropertiesFeaturesQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub tile_properties: VkBool32,
}
impl Default for VkPhysicalDeviceTilePropertiesFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM,
      next: core::ptr::null_mut(),
      tile_properties: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTimelineSemaphoreFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub timeline_semaphore: VkBool32,
}
impl Default for VkPhysicalDeviceTimelineSemaphoreFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES,
      next: core::ptr::null_mut(),
      timeline_semaphore: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTimelineSemaphoreProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreProperties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_timeline_semaphore_value_difference: u64,
}
impl Default for VkPhysicalDeviceTimelineSemaphoreProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES,
      next: core::ptr::null_mut(),
      max_timeline_semaphore_value_difference: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceToolProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceToolProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceToolProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub name: zstring::ArrayZString<VK_MAX_EXTENSION_NAME_SIZE>,
  pub version: zstring::ArrayZString<VK_MAX_EXTENSION_NAME_SIZE>,
  pub purposes: VkToolPurposeFlags,
  pub description: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  pub layer: zstring::ArrayZString<VK_MAX_EXTENSION_NAME_SIZE>,
}
impl Default for VkPhysicalDeviceToolProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES,
      next: core::ptr::null_mut(),
      name: Default::default(),
      version: Default::default(),
      purposes: Default::default(),
      description: Default::default(),
      layer: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTransformFeedbackFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub transform_feedback: VkBool32,
  pub geometry_streams: VkBool32,
}
impl Default for VkPhysicalDeviceTransformFeedbackFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT,
      next: core::ptr::null_mut(),
      transform_feedback: Default::default(),
      geometry_streams: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTransformFeedbackPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub max_transform_feedback_streams: u32,
  /// * Limit Type: max
  pub max_transform_feedback_buffers: u32,
  /// * Limit Type: max
  pub max_transform_feedback_buffer_size: VkDeviceSize,
  /// * Limit Type: max
  pub max_transform_feedback_stream_data_size: u32,
  /// * Limit Type: max
  pub max_transform_feedback_buffer_data_size: u32,
  /// * Limit Type: max
  pub max_transform_feedback_buffer_data_stride: u32,
  /// * Limit Type: bitmask
  pub transform_feedback_queries: VkBool32,
  /// * Limit Type: bitmask
  pub transform_feedback_streams_lines_triangles: VkBool32,
  /// * Limit Type: bitmask
  pub transform_feedback_rasterization_stream_select: VkBool32,
  /// * Limit Type: bitmask
  pub transform_feedback_draw: VkBool32,
}
impl Default for VkPhysicalDeviceTransformFeedbackPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      max_transform_feedback_streams: Default::default(),
      max_transform_feedback_buffers: Default::default(),
      max_transform_feedback_buffer_size: Default::default(),
      max_transform_feedback_stream_data_size: Default::default(),
      max_transform_feedback_buffer_data_size: Default::default(),
      max_transform_feedback_buffer_data_stride: Default::default(),
      transform_feedback_queries: Default::default(),
      transform_feedback_streams_lines_triangles: Default::default(),
      transform_feedback_rasterization_stream_select: Default::default(),
      transform_feedback_draw: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceUniformBufferStandardLayoutFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub uniform_buffer_standard_layout: VkBool32,
}
impl Default for VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES,
      next: core::ptr::null_mut(),
      uniform_buffer_standard_layout: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVariablePointersFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVariablePointersFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub variable_pointers_storage_buffer: VkBool32,
  pub variable_pointers: VkBool32,
}
impl Default for VkPhysicalDeviceVariablePointersFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES,
      next: core::ptr::null_mut(),
      variable_pointers_storage_buffer: Default::default(),
      variable_pointers: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub vertex_attribute_instance_rate_divisor: VkBool32,
  pub vertex_attribute_instance_rate_zero_divisor: VkBool32,
}
impl Default for VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT,
      next: core::ptr::null_mut(),
      vertex_attribute_instance_rate_divisor: Default::default(),
      vertex_attribute_instance_rate_zero_divisor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// max value of vertex attribute divisor
  /// * Limit Type: max
  pub max_vertex_attrib_divisor: u32,
}
impl Default for VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT,
      next: core::ptr::null_mut(),
      max_vertex_attrib_divisor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub vertex_input_dynamic_state: VkBool32,
}
impl Default for VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT,
      next: core::ptr::null_mut(),
      vertex_input_dynamic_state: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVideoFormatInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVideoFormatInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoFormatInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image_usage: VkImageUsageFlags,
}
impl Default for VkPhysicalDeviceVideoFormatInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR,
      next: core::ptr::null(),
      image_usage: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan11Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Features.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// 16-bit integer/floating-point variables supported in BufferBlock
  pub storage_buffer_16_bit_access: VkBool32,
  /// 16-bit integer/floating-point variables supported in BufferBlock and Block
  pub uniform_and_storage_buffer_16_bit_access: VkBool32,
  /// 16-bit integer/floating-point variables supported in PushConstant
  pub storage_push_constant_16: VkBool32,
  /// 16-bit integer/floating-point variables supported in shader inputs and
  /// outputs
  pub storage_input_output_16: VkBool32,
  /// Multiple views in a renderpass
  pub multiview: VkBool32,
  /// Multiple views in a renderpass w/ geometry shader
  pub multiview_geometry_shader: VkBool32,
  /// Multiple views in a renderpass w/ tessellation shader
  pub multiview_tessellation_shader: VkBool32,
  pub variable_pointers_storage_buffer: VkBool32,
  pub variable_pointers: VkBool32,
  pub protected_memory: VkBool32,
  /// Sampler color conversion supported
  pub sampler_ycbcr_conversion: VkBool32,
  pub shader_draw_parameters: VkBool32,
}
impl Default for VkPhysicalDeviceVulkan11Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
      next: core::ptr::null_mut(),
      storage_buffer_16_bit_access: Default::default(),
      uniform_and_storage_buffer_16_bit_access: Default::default(),
      storage_push_constant_16: Default::default(),
      storage_input_output_16: Default::default(),
      multiview: Default::default(),
      multiview_geometry_shader: Default::default(),
      multiview_tessellation_shader: Default::default(),
      variable_pointers_storage_buffer: Default::default(),
      variable_pointers: Default::default(),
      protected_memory: Default::default(),
      sampler_ycbcr_conversion: Default::default(),
      shader_draw_parameters: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan11Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Properties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Properties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: exact
  pub device_uuid: [u8; VK_UUID_SIZE],
  /// * Limit Type: exact
  pub driver_uuid: [u8; VK_UUID_SIZE],
  /// * Limit Type: exact
  pub device_luid: zstring::ArrayZString<VK_LUID_SIZE>,
  /// * Limit Type: exact
  pub device_node_mask: u32,
  /// * Limit Type: exact
  pub device_luid_valid: VkBool32,
  /// The size of a subgroup for this queue.
  /// * Limit Type: max,pot
  /// * No Auto-Validity
  pub subgroup_size: u32,
  /// Bitfield of what shader stages support subgroup operations
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub subgroup_supported_stages: VkShaderStageFlags,
  /// Bitfield of what subgroup operations are supported.
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub subgroup_supported_operations: VkSubgroupFeatureFlags,
  /// Flag to specify whether quad operations are available in all stages.
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub subgroup_quad_operations_in_all_stages: VkBool32,
  /// * Limit Type: exact
  pub point_clipping_behavior: VkPointClippingBehavior,
  /// max number of views in a subpass
  /// * Limit Type: max
  pub max_multiview_view_count: u32,
  /// max instance index for a draw in a multiview subpass
  /// * Limit Type: max
  pub max_multiview_instance_index: u32,
  /// * Limit Type: exact
  pub protected_no_fault: VkBool32,
  /// * Limit Type: max
  pub max_per_set_descriptors: u32,
  /// * Limit Type: max
  pub max_memory_allocation_size: VkDeviceSize,
}
impl Default for VkPhysicalDeviceVulkan11Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES,
      next: core::ptr::null_mut(),
      device_uuid: [Default::default(); VK_UUID_SIZE],
      driver_uuid: [Default::default(); VK_UUID_SIZE],
      device_luid: Default::default(),
      device_node_mask: Default::default(),
      device_luid_valid: Default::default(),
      subgroup_size: Default::default(),
      subgroup_supported_stages: Default::default(),
      subgroup_supported_operations: Default::default(),
      subgroup_quad_operations_in_all_stages: Default::default(),
      point_clipping_behavior: Default::default(),
      max_multiview_view_count: Default::default(),
      max_multiview_instance_index: Default::default(),
      protected_no_fault: Default::default(),
      max_per_set_descriptors: Default::default(),
      max_memory_allocation_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan12Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Features.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub sampler_mirror_clamp_to_edge: VkBool32,
  pub draw_indirect_count: VkBool32,
  /// 8-bit integer variables supported in StorageBuffer
  pub storage_buffer_8_bit_access: VkBool32,
  /// 8-bit integer variables supported in StorageBuffer and Uniform
  pub uniform_and_storage_buffer_8_bit_access: VkBool32,
  /// 8-bit integer variables supported in PushConstant
  pub storage_push_constant_8: VkBool32,
  pub shader_buffer_int_64_atomics: VkBool32,
  pub shader_shared_int_64_atomics: VkBool32,
  /// 16-bit floats (halfs) in shaders
  pub shader_float_16: VkBool32,
  /// 8-bit integers in shaders
  pub shader_int_8: VkBool32,
  pub descriptor_indexing: VkBool32,
  pub shader_input_attachment_array_dynamic_indexing: VkBool32,
  pub shader_uniform_texel_buffer_array_dynamic_indexing: VkBool32,
  pub shader_storage_texel_buffer_array_dynamic_indexing: VkBool32,
  pub shader_uniform_buffer_array_non_uniform_indexing: VkBool32,
  pub shader_sampled_image_array_non_uniform_indexing: VkBool32,
  pub shader_storage_buffer_array_non_uniform_indexing: VkBool32,
  pub shader_storage_image_array_non_uniform_indexing: VkBool32,
  pub shader_input_attachment_array_non_uniform_indexing: VkBool32,
  pub shader_uniform_texel_buffer_array_non_uniform_indexing: VkBool32,
  pub shader_storage_texel_buffer_array_non_uniform_indexing: VkBool32,
  pub descriptor_binding_uniform_buffer_update_after_bind: VkBool32,
  pub descriptor_binding_sampled_image_update_after_bind: VkBool32,
  pub descriptor_binding_storage_image_update_after_bind: VkBool32,
  pub descriptor_binding_storage_buffer_update_after_bind: VkBool32,
  pub descriptor_binding_uniform_texel_buffer_update_after_bind: VkBool32,
  pub descriptor_binding_storage_texel_buffer_update_after_bind: VkBool32,
  pub descriptor_binding_update_unused_while_pending: VkBool32,
  pub descriptor_binding_partially_bound: VkBool32,
  pub descriptor_binding_variable_descriptor_count: VkBool32,
  pub runtime_descriptor_array: VkBool32,
  pub sampler_filter_minmax: VkBool32,
  pub scalar_block_layout: VkBool32,
  pub imageless_framebuffer: VkBool32,
  pub uniform_buffer_standard_layout: VkBool32,
  pub shader_subgroup_extended_types: VkBool32,
  pub separate_depth_stencil_layouts: VkBool32,
  pub host_query_reset: VkBool32,
  pub timeline_semaphore: VkBool32,
  pub buffer_device_address: VkBool32,
  pub buffer_device_address_capture_replay: VkBool32,
  pub buffer_device_address_multi_device: VkBool32,
  pub vulkan_memory_model: VkBool32,
  pub vulkan_memory_model_device_scope: VkBool32,
  pub vulkan_memory_model_availability_visibility_chains: VkBool32,
  pub shader_output_viewport_index: VkBool32,
  pub shader_output_layer: VkBool32,
  pub subgroup_broadcast_dynamic_id: VkBool32,
}
impl Default for VkPhysicalDeviceVulkan12Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
      next: core::ptr::null_mut(),
      sampler_mirror_clamp_to_edge: Default::default(),
      draw_indirect_count: Default::default(),
      storage_buffer_8_bit_access: Default::default(),
      uniform_and_storage_buffer_8_bit_access: Default::default(),
      storage_push_constant_8: Default::default(),
      shader_buffer_int_64_atomics: Default::default(),
      shader_shared_int_64_atomics: Default::default(),
      shader_float_16: Default::default(),
      shader_int_8: Default::default(),
      descriptor_indexing: Default::default(),
      shader_input_attachment_array_dynamic_indexing: Default::default(),
      shader_uniform_texel_buffer_array_dynamic_indexing: Default::default(),
      shader_storage_texel_buffer_array_dynamic_indexing: Default::default(),
      shader_uniform_buffer_array_non_uniform_indexing: Default::default(),
      shader_sampled_image_array_non_uniform_indexing: Default::default(),
      shader_storage_buffer_array_non_uniform_indexing: Default::default(),
      shader_storage_image_array_non_uniform_indexing: Default::default(),
      shader_input_attachment_array_non_uniform_indexing: Default::default(),
      shader_uniform_texel_buffer_array_non_uniform_indexing: Default::default(),
      shader_storage_texel_buffer_array_non_uniform_indexing: Default::default(),
      descriptor_binding_uniform_buffer_update_after_bind: Default::default(),
      descriptor_binding_sampled_image_update_after_bind: Default::default(),
      descriptor_binding_storage_image_update_after_bind: Default::default(),
      descriptor_binding_storage_buffer_update_after_bind: Default::default(),
      descriptor_binding_uniform_texel_buffer_update_after_bind: Default::default(),
      descriptor_binding_storage_texel_buffer_update_after_bind: Default::default(),
      descriptor_binding_update_unused_while_pending: Default::default(),
      descriptor_binding_partially_bound: Default::default(),
      descriptor_binding_variable_descriptor_count: Default::default(),
      runtime_descriptor_array: Default::default(),
      sampler_filter_minmax: Default::default(),
      scalar_block_layout: Default::default(),
      imageless_framebuffer: Default::default(),
      uniform_buffer_standard_layout: Default::default(),
      shader_subgroup_extended_types: Default::default(),
      separate_depth_stencil_layouts: Default::default(),
      host_query_reset: Default::default(),
      timeline_semaphore: Default::default(),
      buffer_device_address: Default::default(),
      buffer_device_address_capture_replay: Default::default(),
      buffer_device_address_multi_device: Default::default(),
      vulkan_memory_model: Default::default(),
      vulkan_memory_model_device_scope: Default::default(),
      vulkan_memory_model_availability_visibility_chains: Default::default(),
      shader_output_viewport_index: Default::default(),
      shader_output_layer: Default::default(),
      subgroup_broadcast_dynamic_id: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan12Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Properties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Properties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: noauto
  pub driver_id: VkDriverId,
  /// * Limit Type: noauto
  pub driver_name: zstring::ArrayZString<VK_MAX_DRIVER_NAME_SIZE>,
  /// * Limit Type: noauto
  pub driver_info: zstring::ArrayZString<VK_MAX_DRIVER_INFO_SIZE>,
  /// * Limit Type: noauto
  pub conformance_version: VkConformanceVersion,
  /// * Limit Type: exact
  pub denorm_behavior_independence: VkShaderFloatControlsIndependence,
  /// * Limit Type: exact
  pub rounding_mode_independence: VkShaderFloatControlsIndependence,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shader_signed_zero_inf_nan_preserve_float_16: VkBool32,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shader_signed_zero_inf_nan_preserve_float_32: VkBool32,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shader_signed_zero_inf_nan_preserve_float_64: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_preserve_float_16: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_preserve_float_32: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_preserve_float_64: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_flush_to_zero_float_16: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_flush_to_zero_float_32: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shader_denorm_flush_to_zero_float_64: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rte_float_16: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rte_float_32: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rte_float_64: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rtz_float_16: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rtz_float_32: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shader_rounding_mode_rtz_float_64: VkBool32,
  /// * Limit Type: max
  pub max_update_after_bind_descriptors_in_all_pools: u32,
  /// * Limit Type: bitmask
  pub shader_uniform_buffer_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub shader_sampled_image_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub shader_storage_buffer_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub shader_storage_image_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub shader_input_attachment_array_non_uniform_indexing_native: VkBool32,
  /// * Limit Type: bitmask
  pub robust_buffer_access_update_after_bind: VkBool32,
  /// * Limit Type: bitmask
  pub quad_divergent_implicit_lod: VkBool32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_samplers: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
  /// * Limit Type: max
  pub max_per_stage_update_after_bind_resources: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_samplers: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_storage_buffers: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_sampled_images: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_storage_images: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_input_attachments: u32,
  /// supported depth resolve modes
  /// * Limit Type: bitmask
  pub supported_depth_resolve_modes: VkResolveModeFlags,
  /// supported stencil resolve modes
  /// * Limit Type: bitmask
  pub supported_stencil_resolve_modes: VkResolveModeFlags,
  /// depth and stencil resolve modes can be set independently if one of them is
  /// none
  /// * Limit Type: bitmask
  pub independent_resolve_none: VkBool32,
  /// depth and stencil resolve modes can be set independently
  /// * Limit Type: bitmask
  pub independent_resolve: VkBool32,
  /// * Limit Type: bitmask
  pub filter_minmax_single_component_formats: VkBool32,
  /// * Limit Type: bitmask
  pub filter_minmax_image_component_mapping: VkBool32,
  /// * Limit Type: max
  pub max_timeline_semaphore_value_difference: u64,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebuffer_integer_color_sample_counts: VkSampleCountFlags,
}
impl Default for VkPhysicalDeviceVulkan12Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES,
      next: core::ptr::null_mut(),
      driver_id: Default::default(),
      driver_name: Default::default(),
      driver_info: Default::default(),
      conformance_version: Default::default(),
      denorm_behavior_independence: Default::default(),
      rounding_mode_independence: Default::default(),
      shader_signed_zero_inf_nan_preserve_float_16: Default::default(),
      shader_signed_zero_inf_nan_preserve_float_32: Default::default(),
      shader_signed_zero_inf_nan_preserve_float_64: Default::default(),
      shader_denorm_preserve_float_16: Default::default(),
      shader_denorm_preserve_float_32: Default::default(),
      shader_denorm_preserve_float_64: Default::default(),
      shader_denorm_flush_to_zero_float_16: Default::default(),
      shader_denorm_flush_to_zero_float_32: Default::default(),
      shader_denorm_flush_to_zero_float_64: Default::default(),
      shader_rounding_mode_rte_float_16: Default::default(),
      shader_rounding_mode_rte_float_32: Default::default(),
      shader_rounding_mode_rte_float_64: Default::default(),
      shader_rounding_mode_rtz_float_16: Default::default(),
      shader_rounding_mode_rtz_float_32: Default::default(),
      shader_rounding_mode_rtz_float_64: Default::default(),
      max_update_after_bind_descriptors_in_all_pools: Default::default(),
      shader_uniform_buffer_array_non_uniform_indexing_native: Default::default(),
      shader_sampled_image_array_non_uniform_indexing_native: Default::default(),
      shader_storage_buffer_array_non_uniform_indexing_native: Default::default(),
      shader_storage_image_array_non_uniform_indexing_native: Default::default(),
      shader_input_attachment_array_non_uniform_indexing_native: Default::default(),
      robust_buffer_access_update_after_bind: Default::default(),
      quad_divergent_implicit_lod: Default::default(),
      max_per_stage_descriptor_update_after_bind_samplers: Default::default(),
      max_per_stage_descriptor_update_after_bind_uniform_buffers: Default::default(),
      max_per_stage_descriptor_update_after_bind_storage_buffers: Default::default(),
      max_per_stage_descriptor_update_after_bind_sampled_images: Default::default(),
      max_per_stage_descriptor_update_after_bind_storage_images: Default::default(),
      max_per_stage_descriptor_update_after_bind_input_attachments: Default::default(),
      max_per_stage_update_after_bind_resources: Default::default(),
      max_descriptor_set_update_after_bind_samplers: Default::default(),
      max_descriptor_set_update_after_bind_uniform_buffers: Default::default(),
      max_descriptor_set_update_after_bind_uniform_buffers_dynamic: Default::default(),
      max_descriptor_set_update_after_bind_storage_buffers: Default::default(),
      max_descriptor_set_update_after_bind_storage_buffers_dynamic: Default::default(),
      max_descriptor_set_update_after_bind_sampled_images: Default::default(),
      max_descriptor_set_update_after_bind_storage_images: Default::default(),
      max_descriptor_set_update_after_bind_input_attachments: Default::default(),
      supported_depth_resolve_modes: Default::default(),
      supported_stencil_resolve_modes: Default::default(),
      independent_resolve_none: Default::default(),
      independent_resolve: Default::default(),
      filter_minmax_single_component_formats: Default::default(),
      filter_minmax_image_component_mapping: Default::default(),
      max_timeline_semaphore_value_difference: Default::default(),
      framebuffer_integer_color_sample_counts: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan13Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Features.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub robust_image_access: VkBool32,
  pub inline_uniform_block: VkBool32,
  pub descriptor_binding_inline_uniform_block_update_after_bind: VkBool32,
  pub pipeline_creation_cache_control: VkBool32,
  pub private_data: VkBool32,
  pub shader_demote_to_helper_invocation: VkBool32,
  pub shader_terminate_invocation: VkBool32,
  pub subgroup_size_control: VkBool32,
  pub compute_full_subgroups: VkBool32,
  pub synchronization_2: VkBool32,
  pub texture_compression_astc_hdr: VkBool32,
  pub shader_zero_initialize_workgroup_memory: VkBool32,
  pub dynamic_rendering: VkBool32,
  pub shader_integer_dot_product: VkBool32,
  pub maintenance_4: VkBool32,
}
impl Default for VkPhysicalDeviceVulkan13Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
      next: core::ptr::null_mut(),
      robust_image_access: Default::default(),
      inline_uniform_block: Default::default(),
      descriptor_binding_inline_uniform_block_update_after_bind: Default::default(),
      pipeline_creation_cache_control: Default::default(),
      private_data: Default::default(),
      shader_demote_to_helper_invocation: Default::default(),
      shader_terminate_invocation: Default::default(),
      subgroup_size_control: Default::default(),
      compute_full_subgroups: Default::default(),
      synchronization_2: Default::default(),
      texture_compression_astc_hdr: Default::default(),
      shader_zero_initialize_workgroup_memory: Default::default(),
      dynamic_rendering: Default::default(),
      shader_integer_dot_product: Default::default(),
      maintenance_4: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan13Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Properties.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Properties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// The minimum subgroup size supported by this device
  /// * Limit Type: min,pot
  /// * No Auto-Validity
  pub min_subgroup_size: u32,
  /// The maximum subgroup size supported by this device
  /// * Limit Type: max,pot
  /// * No Auto-Validity
  pub max_subgroup_size: u32,
  /// The maximum number of subgroups supported in a workgroup
  /// * Limit Type: max
  /// * No Auto-Validity
  pub max_compute_workgroup_subgroups: u32,
  /// The shader stages that support specifying a subgroup size
  /// * Limit Type: bitmask
  pub required_subgroup_size_stages: VkShaderStageFlags,
  /// * Limit Type: max
  pub max_inline_uniform_block_size: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_inline_uniform_blocks: u32,
  /// * Limit Type: max
  pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
  /// * Limit Type: max
  pub max_descriptor_set_inline_uniform_blocks: u32,
  /// * Limit Type: max
  pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
  /// * Limit Type: max
  pub max_inline_uniform_total_size: u32,
  /// * Limit Type: bitmask
  pub integer_dot_product_8_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_8_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_8_bit_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_4_x_8_bit_packed_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_16_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_16_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_16_bit_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_32_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_32_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_32_bit_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_64_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_64_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_64_bit_mixed_signedness_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated:
    VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated:
    VkBool32,
  /// * Limit Type: min,pot
  pub storage_texel_buffer_offset_alignment_bytes: VkDeviceSize,
  /// * Limit Type: exact
  pub storage_texel_buffer_offset_single_texel_alignment: VkBool32,
  /// * Limit Type: min,pot
  pub uniform_texel_buffer_offset_alignment_bytes: VkDeviceSize,
  /// * Limit Type: exact
  pub uniform_texel_buffer_offset_single_texel_alignment: VkBool32,
  /// * Limit Type: max
  pub max_buffer_size: VkDeviceSize,
}
impl Default for VkPhysicalDeviceVulkan13Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES,
      next: core::ptr::null_mut(),
      min_subgroup_size: Default::default(),
      max_subgroup_size: Default::default(),
      max_compute_workgroup_subgroups: Default::default(),
      required_subgroup_size_stages: Default::default(),
      max_inline_uniform_block_size: Default::default(),
      max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
      max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(),
      max_descriptor_set_inline_uniform_blocks: Default::default(),
      max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default(),
      max_inline_uniform_total_size: Default::default(),
      integer_dot_product_8_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_8_bit_signed_accelerated: Default::default(),
      integer_dot_product_8_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: Default::default(),
      integer_dot_product_4_x_8_bit_packed_signed_accelerated: Default::default(),
      integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_16_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_16_bit_signed_accelerated: Default::default(),
      integer_dot_product_16_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_32_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_32_bit_signed_accelerated: Default::default(),
      integer_dot_product_32_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_64_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_64_bit_signed_accelerated: Default::default(),
      integer_dot_product_64_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: Default::default(),
      integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: Default::default(),
      storage_texel_buffer_offset_alignment_bytes: Default::default(),
      storage_texel_buffer_offset_single_texel_alignment: Default::default(),
      uniform_texel_buffer_offset_alignment_bytes: Default::default(),
      uniform_texel_buffer_offset_single_texel_alignment: Default::default(),
      max_buffer_size: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkanMemoryModelFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub vulkan_memory_model: VkBool32,
  pub vulkan_memory_model_device_scope: VkBool32,
  pub vulkan_memory_model_availability_visibility_chains: VkBool32,
}
impl Default for VkPhysicalDeviceVulkanMemoryModelFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES,
      next: core::ptr::null_mut(),
      vulkan_memory_model: Default::default(),
      vulkan_memory_model_device_scope: Default::default(),
      vulkan_memory_model_availability_visibility_chains: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub workgroup_memory_explicit_layout: VkBool32,
  pub workgroup_memory_explicit_layout_scalar_block_layout: VkBool32,
  pub workgroup_memory_explicit_layout_8_bit_access: VkBool32,
  pub workgroup_memory_explicit_layout_16_bit_access: VkBool32,
}
impl Default for VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR,
      next: core::ptr::null_mut(),
      workgroup_memory_explicit_layout: Default::default(),
      workgroup_memory_explicit_layout_scalar_block_layout: Default::default(),
      workgroup_memory_explicit_layout_8_bit_access: Default::default(),
      workgroup_memory_explicit_layout_16_bit_access: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub ycbcr_2_plane_444_formats: VkBool32,
}
impl Default for VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT,
      next: core::ptr::null_mut(),
      ycbcr_2_plane_444_formats: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceYcbcrImageArraysFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub ycbcr_image_arrays: VkBool32,
}
impl Default for VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT,
      next: core::ptr::null_mut(),
      ycbcr_image_arrays: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub shader_zero_initialize_workgroup_memory: VkBool32,
}
impl Default for VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES,
      next: core::ptr::null_mut(),
      shader_zero_initialize_workgroup_memory: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCacheCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineCacheCreateFlags,
  /// Size of initial data to populate cache, in bytes
  /// * Optional: true
  pub initial_data_size: c_size_t,
  /// Initial data to populate cache
  /// * Len: `initial_data_size`
  pub initial_data: *const c_void,
}
impl Default for VkPipelineCacheCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      initial_data_size: Default::default(),
      initial_data: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineCacheHeaderVersionOne](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersionOne.html) (structure)
///
/// The fields in this structure are non-normative since structure packing is
/// implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCacheHeaderVersionOne {
  pub header_size: u32,
  pub header_version: VkPipelineCacheHeaderVersion,
  pub vendor_id: u32,
  pub device_id: u32,
  pub pipeline_cache_uuid: [u8; VK_UUID_SIZE],
}
impl Default for VkPipelineCacheHeaderVersionOne {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      header_size: Default::default(),
      header_version: Default::default(),
      vendor_id: Default::default(),
      device_id: Default::default(),
      pipeline_cache_uuid: [Default::default(); VK_UUID_SIZE],
    }
  }
}

/// Khronos: [VkPipelineCacheHeaderVersionSafetyCriticalOne](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersionSafetyCriticalOne.html) (structure)
///
/// The fields in this structure are non-normative since structure packing is
/// implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCacheHeaderVersionSafetyCriticalOne {
  pub header_version_one: VkPipelineCacheHeaderVersionOne,
  pub validation_version: VkPipelineCacheValidationVersion,
  pub implementation_data: u32,
  pub pipeline_index_count: u32,
  pub pipeline_index_stride: u32,
  pub pipeline_index_offset: u64,
}
impl Default for VkPipelineCacheHeaderVersionSafetyCriticalOne {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      header_version_one: Default::default(),
      validation_version: Default::default(),
      implementation_data: Default::default(),
      pipeline_index_count: Default::default(),
      pipeline_index_stride: Default::default(),
      pipeline_index_offset: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCacheSafetyCriticalIndexEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheSafetyCriticalIndexEntry.html) (structure)
///
/// The fields in this structure are non-normative since structure packing is
/// implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCacheSafetyCriticalIndexEntry {
  pub pipeline_identifier: [u8; VK_UUID_SIZE],
  pub pipeline_memory_size: u64,
  pub json_size: u64,
  pub json_offset: u64,
  pub stage_index_count: u32,
  pub stage_index_stride: u32,
  pub stage_index_offset: u64,
}
impl Default for VkPipelineCacheSafetyCriticalIndexEntry {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      pipeline_identifier: [Default::default(); VK_UUID_SIZE],
      pipeline_memory_size: Default::default(),
      json_size: Default::default(),
      json_offset: Default::default(),
      stage_index_count: Default::default(),
      stage_index_stride: Default::default(),
      stage_index_offset: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCacheStageValidationIndexEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheStageValidationIndexEntry.html) (structure)
///
/// The fields in this structure are non-normative since structure packing is
/// implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCacheStageValidationIndexEntry {
  pub code_size: u64,
  pub code_offset: u64,
}
impl Default for VkPipelineCacheStageValidationIndexEntry {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { code_size: Default::default(), code_offset: Default::default() }
  }
}

/// Khronos: [VkPipelineColorBlendAdvancedStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineColorBlendStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_premultiplied: VkBool32,
  pub dst_premultiplied: VkBool32,
  pub blend_overlap: VkBlendOverlapEXT,
}
impl Default for VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      src_premultiplied: Default::default(),
      dst_premultiplied: Default::default(),
      blend_overlap: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineColorBlendAttachmentState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAttachmentState.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
  pub blend_enable: VkBool32,
  pub src_color_blend_factor: VkBlendFactor,
  pub dst_color_blend_factor: VkBlendFactor,
  pub color_blend_op: VkBlendOp,
  pub src_alpha_blend_factor: VkBlendFactor,
  pub dst_alpha_blend_factor: VkBlendFactor,
  pub alpha_blend_op: VkBlendOp,
  /// * Optional: true
  pub color_write_mask: VkColorComponentFlags,
}
impl Default for VkPipelineColorBlendAttachmentState {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      blend_enable: Default::default(),
      src_color_blend_factor: Default::default(),
      dst_color_blend_factor: Default::default(),
      color_blend_op: Default::default(),
      src_alpha_blend_factor: Default::default(),
      dst_alpha_blend_factor: Default::default(),
      alpha_blend_op: Default::default(),
      color_write_mask: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineColorBlendStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineColorBlendStateCreateFlags,
  pub logic_op_enable: VkBool32,
  /// * No Auto-Validity
  pub logic_op: VkLogicOp,
  /// # of pAttachments
  /// * Optional: true
  pub attachment_count: u32,
  /// * Optional: true
  /// * Len: `attachment_count`
  pub attachments: *const VkPipelineColorBlendAttachmentState,
  pub blend_constants: [c_float; 4],
}
impl Default for VkPipelineColorBlendStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      logic_op_enable: Default::default(),
      logic_op: Default::default(),
      attachment_count: Default::default(),
      attachments: core::ptr::null(),
      blend_constants: [Default::default(); 4],
    }
  }
}

/// Khronos: [VkPipelineColorWriteCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorWriteCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineColorBlendStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineColorWriteCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// # of pAttachments
  /// * Optional: true
  pub attachment_count: u32,
  /// * Len: `attachment_count`
  pub color_write_enables: *const VkBool32,
}
impl Default for VkPipelineColorWriteCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      attachment_count: Default::default(),
      color_write_enables: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineCompilerControlCreateInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
/// * Struct Extends: [`VkComputePipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCompilerControlCreateInfoAMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub compiler_control_flags: VkPipelineCompilerControlFlagsAMD,
}
impl Default for VkPipelineCompilerControlCreateInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD,
      next: core::ptr::null(),
      compiler_control_flags: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCoverageModulationStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
  pub coverage_modulation_mode: VkCoverageModulationModeNV,
  pub coverage_modulation_table_enable: VkBool32,
  /// * Optional: true
  pub coverage_modulation_table_count: u32,
  /// * Optional: true
  /// * Len: `coverage_modulation_table_count`
  /// * No Auto-Validity
  pub coverage_modulation_table: *const c_float,
}
impl Default for VkPipelineCoverageModulationStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      flags: Default::default(),
      coverage_modulation_mode: Default::default(),
      coverage_modulation_table_enable: Default::default(),
      coverage_modulation_table_count: Default::default(),
      coverage_modulation_table: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineCoverageReductionStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageReductionStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCoverageReductionStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineCoverageReductionStateCreateFlagsNV,
  pub coverage_reduction_mode: VkCoverageReductionModeNV,
}
impl Default for VkPipelineCoverageReductionStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      flags: Default::default(),
      coverage_reduction_mode: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCoverageToColorStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
  pub coverage_to_color_enable: VkBool32,
  /// * Optional: true
  pub coverage_to_color_location: u32,
}
impl Default for VkPipelineCoverageToColorStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      flags: Default::default(),
      coverage_to_color_enable: Default::default(),
      coverage_to_color_location: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCreationFeedback](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedback.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCreationFeedback {
  pub flags: VkPipelineCreationFeedbackFlags,
  pub duration: u64,
}
impl Default for VkPipelineCreationFeedback {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { flags: Default::default(), duration: Default::default() }
  }
}

/// Khronos: [VkPipelineCreationFeedbackCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
/// * Struct Extends: [`VkComputePipelineCreateInfo`]
/// * Struct Extends: [`VkRayTracingPipelineCreateInfoNV`]
/// * Struct Extends: [`VkRayTracingPipelineCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineCreationFeedbackCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Output pipeline creation feedback.
  pub pipeline_creation_feedback: *mut VkPipelineCreationFeedback,
  /// * Optional: true
  pub pipeline_stage_creation_feedback_count: u32,
  /// One entry for each shader stage specified in the parent
  /// Vk*PipelineCreateInfo struct
  /// * Len: `pipeline_stage_creation_feedback_count`
  pub pipeline_stage_creation_feedbacks: *mut VkPipelineCreationFeedback,
}
impl Default for VkPipelineCreationFeedbackCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO,
      next: core::ptr::null(),
      pipeline_creation_feedback: core::ptr::null_mut(),
      pipeline_stage_creation_feedback_count: Default::default(),
      pipeline_stage_creation_feedbacks: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkPipelineDepthStencilStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineDepthStencilStateCreateFlags,
  pub depth_test_enable: VkBool32,
  pub depth_write_enable: VkBool32,
  pub depth_compare_op: VkCompareOp,
  /// optional (depth_bounds_test)
  pub depth_bounds_test_enable: VkBool32,
  pub stencil_test_enable: VkBool32,
  pub front: VkStencilOpState,
  pub back: VkStencilOpState,
  pub min_depth_bounds: c_float,
  pub max_depth_bounds: c_float,
}
impl Default for VkPipelineDepthStencilStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      depth_test_enable: Default::default(),
      depth_write_enable: Default::default(),
      depth_compare_op: Default::default(),
      depth_bounds_test_enable: Default::default(),
      stencil_test_enable: Default::default(),
      front: Default::default(),
      back: Default::default(),
      min_depth_bounds: Default::default(),
      max_depth_bounds: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineDiscardRectangleStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
  pub discard_rectangle_mode: VkDiscardRectangleModeEXT,
  /// * Optional: true
  pub discard_rectangle_count: u32,
  /// * Len: `discard_rectangle_count`
  /// * No Auto-Validity
  pub discard_rectangles: *const VkRect2D,
}
impl Default for VkPipelineDiscardRectangleStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      discard_rectangle_mode: Default::default(),
      discard_rectangle_count: Default::default(),
      discard_rectangles: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineDynamicStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineDynamicStateCreateFlags,
  /// * Optional: true
  pub dynamic_state_count: u32,
  /// * Len: `dynamic_state_count`
  pub dynamic_states: *const VkDynamicState,
}
impl Default for VkPipelineDynamicStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      dynamic_state_count: Default::default(),
      dynamic_states: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineExecutableInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineExecutableInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub pipeline: VkPipeline,
  pub executable_index: u32,
}
impl Default for VkPipelineExecutableInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR,
      next: core::ptr::null(),
      pipeline: Default::default(),
      executable_index: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineExecutableInternalRepresentationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineExecutableInternalRepresentationKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub name: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  pub description: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  pub is_text: VkBool32,
  pub data_size: c_size_t,
  /// * Optional: true
  /// * Len: `data_size`
  pub data: *mut c_void,
}
impl Default for VkPipelineExecutableInternalRepresentationKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR,
      next: core::ptr::null_mut(),
      name: Default::default(),
      description: Default::default(),
      is_text: Default::default(),
      data_size: Default::default(),
      data: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkPipelineExecutablePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutablePropertiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineExecutablePropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub stages: VkShaderStageFlags,
  pub name: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  pub description: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  pub subgroup_size: u32,
}
impl Default for VkPipelineExecutablePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      stages: Default::default(),
      name: Default::default(),
      description: Default::default(),
      subgroup_size: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineFragmentShadingRateEnumStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * No Auto-Validity
  pub shading_rate_type: VkFragmentShadingRateTypeNV,
  /// * No Auto-Validity
  pub shading_rate: VkFragmentShadingRateNV,
  /// * No Auto-Validity
  pub combiner_ops: [VkFragmentShadingRateCombinerOpKHR; 2],
}
impl Default for VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      shading_rate_type: Default::default(),
      shading_rate: Default::default(),
      combiner_ops: [Default::default(); 2],
    }
  }
}

/// Khronos: [VkPipelineFragmentShadingRateStateCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateStateCreateInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub fragment_size: VkExtent2D,
  /// * No Auto-Validity
  pub combiner_ops: [VkFragmentShadingRateCombinerOpKHR; 2],
}
impl Default for VkPipelineFragmentShadingRateStateCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      fragment_size: Default::default(),
      combiner_ops: [Default::default(); 2],
    }
  }
}

/// Khronos: [VkPipelineInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub pipeline: VkPipeline,
}
impl Default for VkPipelineInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR,
      next: core::ptr::null(),
      pipeline: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineInputAssemblyStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineInputAssemblyStateCreateFlags,
  pub topology: VkPrimitiveTopology,
  pub primitive_restart_enable: VkBool32,
}
impl Default for VkPipelineInputAssemblyStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      topology: Default::default(),
      primitive_restart_enable: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineLayoutCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineLayoutCreateFlags,
  /// Number of descriptor sets interfaced by the pipeline
  /// * Optional: true
  pub set_layout_count: u32,
  /// Array of setCount number of descriptor set layout objects defining the
  /// layout of the
  /// * Optional: false,true
  /// * Len: `set_layout_count`
  pub set_layouts: *const VkDescriptorSetLayout,
  /// Number of push-constant ranges used by the pipeline
  /// * Optional: true
  pub push_constant_range_count: u32,
  /// Array of pushConstantRangeCount number of ranges used by various shader
  /// stages
  /// * Len: `push_constant_range_count`
  pub push_constant_ranges: *const VkPushConstantRange,
}
impl Default for VkPipelineLayoutCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      set_layout_count: Default::default(),
      set_layouts: core::ptr::null(),
      push_constant_range_count: Default::default(),
      push_constant_ranges: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineLibraryCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineLibraryCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub library_count: u32,
  /// * Len: `library_count`
  pub libraries: *const VkPipeline,
}
impl Default for VkPipelineLibraryCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR,
      next: core::ptr::null(),
      library_count: Default::default(),
      libraries: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineMultisampleStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineMultisampleStateCreateFlags,
  /// Number of samples used for rasterization
  pub rasterization_samples: VkSampleCountFlagBits,
  /// optional (GL45)
  pub sample_shading_enable: VkBool32,
  /// optional (GL45)
  pub min_sample_shading: c_float,
  /// Array of sampleMask words
  /// * Optional: true
  /// * Len: `(rasterization_samples + 31) / 32`
  pub sample_mask: *const VkSampleMask,
  pub alpha_to_coverage_enable: VkBool32,
  pub alpha_to_one_enable: VkBool32,
}
impl Default for VkPipelineMultisampleStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      rasterization_samples: Default::default(),
      sample_shading_enable: Default::default(),
      min_sample_shading: Default::default(),
      sample_mask: core::ptr::null(),
      alpha_to_coverage_enable: Default::default(),
      alpha_to_one_enable: Default::default(),
    }
  }
}

/// Khronos: [VkPipelinePropertiesIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelinePropertiesIdentifierEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelinePropertiesIdentifierEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_PROPERTIES_IDENTIFIER_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub pipeline_identifier: [u8; VK_UUID_SIZE],
}
impl Default for VkPipelinePropertiesIdentifierEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_PROPERTIES_IDENTIFIER_EXT,
      next: core::ptr::null_mut(),
      pipeline_identifier: [Default::default(); VK_UUID_SIZE],
    }
  }
}

/// Khronos: [VkPipelineRasterizationConservativeStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationConservativeStateCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Reserved
  /// * Optional: true
  pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
  /// Conservative rasterization mode
  pub conservative_rasterization_mode: VkConservativeRasterizationModeEXT,
  /// Extra overestimation to add to the primitive
  pub extra_primitive_overestimation_size: c_float,
}
impl Default for VkPipelineRasterizationConservativeStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      conservative_rasterization_mode: Default::default(),
      extra_primitive_overestimation_size: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationDepthClipStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Reserved
  /// * Optional: true
  pub flags: VkPipelineRasterizationDepthClipStateCreateFlagsEXT,
  pub depth_clip_enable: VkBool32,
}
impl Default for VkPipelineRasterizationDepthClipStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      depth_clip_enable: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationLineStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRasterizationLineStateCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub line_rasterization_mode: VkLineRasterizationModeEXT,
  pub stippled_line_enable: VkBool32,
  pub line_stipple_factor: u32,
  pub line_stipple_pattern: u16,
}
impl Default for VkPipelineRasterizationLineStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      line_rasterization_mode: Default::default(),
      stippled_line_enable: Default::default(),
      line_stipple_factor: Default::default(),
      line_stipple_pattern: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationProvokingVertexStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub provoking_vertex_mode: VkProvokingVertexModeEXT,
}
impl Default for VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      provoking_vertex_mode: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineRasterizationStateCreateFlags,
  pub depth_clamp_enable: VkBool32,
  pub rasterizer_discard_enable: VkBool32,
  /// optional (GL45)
  pub polygon_mode: VkPolygonMode,
  /// * Optional: true
  pub cull_mode: VkCullModeFlags,
  pub front_face: VkFrontFace,
  pub depth_bias_enable: VkBool32,
  pub depth_bias_constant_factor: c_float,
  pub depth_bias_clamp: c_float,
  pub depth_bias_slope_factor: c_float,
  pub line_width: c_float,
}
impl Default for VkPipelineRasterizationStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      depth_clamp_enable: Default::default(),
      rasterizer_discard_enable: Default::default(),
      polygon_mode: Default::default(),
      cull_mode: Default::default(),
      front_face: Default::default(),
      depth_bias_enable: Default::default(),
      depth_bias_constant_factor: Default::default(),
      depth_bias_clamp: Default::default(),
      depth_bias_slope_factor: Default::default(),
      line_width: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationStateRasterizationOrderAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html) (structure)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Rasterization order to use for the pipeline
  pub rasterization_order: VkRasterizationOrderAMD,
}
impl Default for VkPipelineRasterizationStateRasterizationOrderAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
      next: core::ptr::null(),
      rasterization_order: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationStateStreamCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRasterizationStateStreamCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineRasterizationStateStreamCreateFlagsEXT,
  pub rasterization_stream: u32,
}
impl Default for VkPipelineRasterizationStateStreamCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      rasterization_stream: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRenderingCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRenderingCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRenderingCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub view_mask: u32,
  /// * Optional: true
  pub color_attachment_count: u32,
  /// * Len: `color_attachment_count`
  /// * No Auto-Validity
  pub color_attachment_formats: *const VkFormat,
  /// * No Auto-Validity
  pub depth_attachment_format: VkFormat,
  /// * No Auto-Validity
  pub stencil_attachment_format: VkFormat,
}
impl Default for VkPipelineRenderingCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO,
      next: core::ptr::null(),
      view_mask: Default::default(),
      color_attachment_count: Default::default(),
      color_attachment_formats: core::ptr::null(),
      depth_attachment_format: Default::default(),
      stencil_attachment_format: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRepresentativeFragmentTestStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub representative_fragment_test_enable: VkBool32,
}
impl Default for VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      representative_fragment_test_enable: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRobustnessCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRobustnessCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
/// * Struct Extends: [`VkComputePipelineCreateInfo`]
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
/// * Struct Extends: [`VkRayTracingPipelineCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineRobustnessCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *const c_void,
  pub storage_buffers: VkPipelineRobustnessBufferBehaviorEXT,
  pub uniform_buffers: VkPipelineRobustnessBufferBehaviorEXT,
  pub vertex_inputs: VkPipelineRobustnessBufferBehaviorEXT,
  pub images: VkPipelineRobustnessImageBehaviorEXT,
}
impl Default for VkPipelineRobustnessCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT,
      next: core::ptr::null(),
      storage_buffers: Default::default(),
      uniform_buffers: Default::default(),
      vertex_inputs: Default::default(),
      images: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineSampleLocationsStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub sample_locations_enable: VkBool32,
  pub sample_locations_info: VkSampleLocationsInfoEXT,
}
impl Default for VkPipelineSampleLocationsStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      sample_locations_enable: Default::default(),
      sample_locations_info: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineShaderStageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineShaderStageCreateFlags,
  /// Shader stage
  pub stage: VkShaderStageFlagBits,
  /// Module containing entry point
  /// * Optional: true
  pub module: VkShaderModule,
  /// Null-terminated entry point name
  /// * Len: `null_terminated`
  pub name: *const u8,
  /// * Optional: true
  pub specialization_info: *const VkSpecializationInfo,
}
impl Default for VkPipelineShaderStageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      stage: Default::default(),
      module: Default::default(),
      name: core::ptr::null(),
      specialization_info: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineShaderStageModuleIdentifierCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageModuleIdentifierCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineShaderStageModuleIdentifierCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub identifier_size: u32,
  /// * Len: `identifier_size`
  pub identifier: *const u8,
}
impl Default for VkPipelineShaderStageModuleIdentifierCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT,
      next: core::ptr::null(),
      identifier_size: Default::default(),
      identifier: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineShaderStageRequiredSubgroupSizeCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub required_subgroup_size: u32,
}
impl Default for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO,
      next: core::ptr::null_mut(),
      required_subgroup_size: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineTessellationDomainOriginStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkPipelineTessellationStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub domain_origin: VkTessellationDomainOrigin,
}
impl Default for VkPipelineTessellationDomainOriginStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
      next: core::ptr::null(),
      domain_origin: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineTessellationStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineTessellationStateCreateFlags,
  pub patch_control_points: u32,
}
impl Default for VkPipelineTessellationStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      patch_control_points: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineVertexInputDivisorStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputDivisorStateCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineVertexInputStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub vertex_binding_divisor_count: u32,
  /// * Len: `vertex_binding_divisor_count`
  pub vertex_binding_divisors: *const VkVertexInputBindingDivisorDescriptionEXT,
}
impl Default for VkPipelineVertexInputDivisorStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      vertex_binding_divisor_count: Default::default(),
      vertex_binding_divisors: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineVertexInputStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineVertexInputStateCreateFlags,
  /// number of bindings
  /// * Optional: true
  pub vertex_binding_description_count: u32,
  /// * Len: `vertex_binding_description_count`
  pub vertex_binding_descriptions: *const VkVertexInputBindingDescription,
  /// number of attributes
  /// * Optional: true
  pub vertex_attribute_description_count: u32,
  /// * Len: `vertex_attribute_description_count`
  pub vertex_attribute_descriptions: *const VkVertexInputAttributeDescription,
}
impl Default for VkPipelineVertexInputStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      vertex_binding_description_count: Default::default(),
      vertex_binding_descriptions: core::ptr::null(),
      vertex_attribute_description_count: Default::default(),
      vertex_attribute_descriptions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportCoarseSampleOrderStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub sample_order_type: VkCoarseSampleOrderTypeNV,
  /// * Optional: true
  pub custom_sample_order_count: u32,
  /// * Len: `custom_sample_order_count`
  pub custom_sample_orders: *const VkCoarseSampleOrderCustomNV,
}
impl Default for VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      sample_order_type: Default::default(),
      custom_sample_order_count: Default::default(),
      custom_sample_orders: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportDepthClipControlCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineViewportDepthClipControlCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub negative_one_to_one: VkBool32,
}
impl Default for VkPipelineViewportDepthClipControlCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT,
      next: core::ptr::null(),
      negative_one_to_one: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineViewportExclusiveScissorStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub exclusive_scissor_count: u32,
  /// * Len: `exclusive_scissor_count`
  /// * No Auto-Validity
  pub exclusive_scissors: *const VkRect2D,
}
impl Default for VkPipelineViewportExclusiveScissorStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      exclusive_scissor_count: Default::default(),
      exclusive_scissors: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportShadingRateImageStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub shading_rate_image_enable: VkBool32,
  /// * Optional: true
  pub viewport_count: u32,
  /// * Len: `viewport_count`
  /// * No Auto-Validity
  pub shading_rate_palettes: *const VkShadingRatePaletteNV,
}
impl Default for VkPipelineViewportShadingRateImageStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty:
        VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      shading_rate_image_enable: Default::default(),
      viewport_count: Default::default(),
      shading_rate_palettes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineViewportStateCreateFlags,
  /// * Optional: true
  pub viewport_count: u32,
  /// * Optional: true
  /// * Len: `viewport_count`
  /// * No Auto-Validity
  pub viewports: *const VkViewport,
  /// * Optional: true
  pub scissor_count: u32,
  /// * Optional: true
  /// * Len: `scissor_count`
  /// * No Auto-Validity
  pub scissors: *const VkRect2D,
}
impl Default for VkPipelineViewportStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      viewport_count: Default::default(),
      viewports: core::ptr::null(),
      scissor_count: Default::default(),
      scissors: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportSwizzleStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
  pub viewport_count: u32,
  /// * Len: `viewport_count`
  pub viewport_swizzles: *const VkViewportSwizzleNV,
}
impl Default for VkPipelineViewportSwizzleStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      flags: Default::default(),
      viewport_count: Default::default(),
      viewport_swizzles: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportWScalingStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub viewport_w_scaling_enable: VkBool32,
  pub viewport_count: u32,
  /// * Optional: true
  /// * Len: `viewport_count`
  /// * No Auto-Validity
  pub viewport_w_scalings: *const VkViewportWScalingNV,
}
impl Default for VkPipelineViewportWScalingStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
      next: core::ptr::null(),
      viewport_w_scaling_enable: Default::default(),
      viewport_count: Default::default(),
      viewport_w_scalings: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPresentIdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentIdKHR.html) (structure)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPresentIdKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRESENT_ID_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchain_count: u32,
  /// Present ID values for each swapchain
  /// * Optional: true
  /// * Len: `swapchain_count`
  pub present_ids: *const u64,
}
impl Default for VkPresentIdKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PRESENT_ID_KHR,
      next: core::ptr::null(),
      swapchain_count: Default::default(),
      present_ids: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPresentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPresentInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Number of semaphores to wait for before presenting
  /// * Optional: true
  pub wait_semaphore_count: u32,
  /// Semaphores to wait for before presenting
  /// * Len: `wait_semaphore_count`
  pub wait_semaphores: *const VkSemaphore,
  /// Number of swapchains to present in this call
  pub swapchain_count: u32,
  /// Swapchains to present an image from
  /// * Len: `swapchain_count`
  pub swapchains: *const VkSwapchainKHR,
  /// Indices of which presentable images to present
  /// * Len: `swapchain_count`
  pub image_indices: *const u32,
  /// Optional (i.e. if non-NULL) VkResult for each swapchain
  /// * Optional: true
  /// * Len: `swapchain_count`
  pub results: *mut VkResult,
}
impl Default for VkPresentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
      next: core::ptr::null(),
      wait_semaphore_count: Default::default(),
      wait_semaphores: core::ptr::null(),
      swapchain_count: Default::default(),
      swapchains: core::ptr::null(),
      image_indices: core::ptr::null(),
      results: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkPresentRegionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentRegionKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPresentRegionKHR {
  /// Number of rectangles in pRectangles
  /// * Optional: true
  pub rectangle_count: u32,
  /// Array of rectangles that have changed in a swapchain's image(s)
  /// * Optional: true
  /// * Len: `rectangle_count`
  pub rectangles: *const VkRectLayerKHR,
}
impl Default for VkPresentRegionKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { rectangle_count: Default::default(), rectangles: core::ptr::null() }
  }
}

/// Khronos: [VkPresentRegionsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentRegionsKHR.html) (structure)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPresentRegionsKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchain_count: u32,
  /// The regions that have changed
  /// * Optional: true
  /// * Len: `swapchain_count`
  pub regions: *const VkPresentRegionKHR,
}
impl Default for VkPresentRegionsKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR,
      next: core::ptr::null(),
      swapchain_count: Default::default(),
      regions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPresentTimeGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentTimeGOOGLE.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPresentTimeGOOGLE {
  /// Application-provided identifier
  pub present_id: u32,
  /// Earliest time an image should be presented
  pub desired_present_time: u64,
}
impl Default for VkPresentTimeGOOGLE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { present_id: Default::default(), desired_present_time: Default::default() }
  }
}

/// Khronos: [VkPresentTimesInfoGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentTimesInfoGOOGLE.html) (structure)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPresentTimesInfoGOOGLE {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchain_count: u32,
  /// The earliest times to present images
  /// * Optional: true
  /// * Len: `swapchain_count`
  pub times: *const VkPresentTimeGOOGLE,
}
impl Default for VkPresentTimesInfoGOOGLE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE,
      next: core::ptr::null(),
      swapchain_count: Default::default(),
      times: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPrivateDataSlotCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPrivateDataSlotCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub flags: VkPrivateDataSlotCreateFlags,
}
impl Default for VkPrivateDataSlotCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkProtectedSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkProtectedSubmitInfo.html) (structure)
///
/// * Struct Extends: [`VkSubmitInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkProtectedSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Submit protected command buffers
  pub protected_submit: VkBool32,
}
impl Default for VkProtectedSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO,
      next: core::ptr::null(),
      protected_submit: Default::default(),
    }
  }
}

/// Khronos: [VkPushConstantRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPushConstantRange.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkPushConstantRange {
  /// Which stages use the range
  pub stage_flags: VkShaderStageFlags,
  /// Start of the range, in bytes
  pub offset: u32,
  /// Size of the range, in bytes
  pub size: u32,
}
impl Default for VkPushConstantRange {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      stage_flags: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkQueryLowLatencySupportNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryLowLatencySupportNV.html) (structure)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueryLowLatencySupportNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUERY_LOW_LATENCY_SUPPORT_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *const c_void,
  pub queried_low_latency_data: *mut c_void,
}
impl Default for VkQueryLowLatencySupportNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUERY_LOW_LATENCY_SUPPORT_NV,
      next: core::ptr::null(),
      queried_low_latency_data: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkQueryPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueryPoolCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkQueryPoolCreateFlags,
  pub query_type: VkQueryType,
  pub query_count: u32,
  /// Optional
  /// * Optional: true
  /// * No Auto-Validity
  pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}
impl Default for VkQueryPoolCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      query_type: Default::default(),
      query_count: Default::default(),
      pipeline_statistics: Default::default(),
    }
  }
}

/// Khronos: [VkQueryPoolPerformanceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueryPoolPerformanceCreateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub queue_family_index: u32,
  pub counter_index_count: u32,
  /// * Len: `counter_index_count`
  pub counter_indices: *const u32,
}
impl Default for VkQueryPoolPerformanceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      queue_family_index: Default::default(),
      counter_index_count: Default::default(),
      counter_indices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkQueryPoolPerformanceQueryCreateInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html) (structure)
///
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub performance_counters_sampling: VkQueryPoolSamplingModeINTEL,
}
impl Default for VkQueryPoolPerformanceQueryCreateInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL,
      next: core::ptr::null(),
      performance_counters_sampling: Default::default(),
    }
  }
}

/// Khronos: [VkQueryPoolVideoEncodeFeedbackCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolVideoEncodeFeedbackCreateInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueryPoolVideoEncodeFeedbackCreateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub encode_feedback_flags: VkVideoEncodeFeedbackFlagsKHR,
}
impl Default for VkQueryPoolVideoEncodeFeedbackCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR,
      next: core::ptr::null(),
      encode_feedback_flags: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyCheckpointProperties2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointProperties2NV.html) (structure)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueueFamilyCheckpointProperties2NV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub checkpoint_execution_stage_mask: VkPipelineStageFlags2,
}
impl Default for VkQueueFamilyCheckpointProperties2NV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV,
      next: core::ptr::null_mut(),
      checkpoint_execution_stage_mask: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyCheckpointPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html) (structure)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueueFamilyCheckpointPropertiesNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub checkpoint_execution_stage_mask: VkPipelineStageFlags,
}
impl Default for VkQueueFamilyCheckpointPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV,
      next: core::ptr::null_mut(),
      checkpoint_execution_stage_mask: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyGlobalPriorityPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueueFamilyGlobalPriorityPropertiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: max
  pub priority_count: u32,
  /// * Limit Type: bitmask
  pub priorities: [VkQueueGlobalPriorityKHR; VK_MAX_GLOBAL_PRIORITY_SIZE_KHR],
}
impl Default for VkQueueFamilyGlobalPriorityPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      priority_count: Default::default(),
      priorities: [Default::default(); VK_MAX_GLOBAL_PRIORITY_SIZE_KHR],
    }
  }
}

/// Khronos: [VkQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueueFamilyProperties {
  /// Queue flags
  /// * Optional: true
  /// * Limit Type: bitmask
  pub queue_flags: VkQueueFlags,
  /// * Limit Type: max
  pub queue_count: u32,
  /// * Limit Type: bits
  pub timestamp_valid_bits: u32,
  /// Minimum alignment requirement for image transfers
  /// * Limit Type: min,mul
  pub min_image_transfer_granularity: VkExtent3D,
}
impl Default for VkQueueFamilyProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      queue_flags: Default::default(),
      queue_count: Default::default(),
      timestamp_valid_bits: Default::default(),
      min_image_transfer_granularity: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties2.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueueFamilyProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: struct
  pub queue_family_properties: VkQueueFamilyProperties,
}
impl Default for VkQueueFamilyProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2,
      next: core::ptr::null_mut(),
      queue_family_properties: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyQueryResultStatusPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyQueryResultStatusPropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueueFamilyQueryResultStatusPropertiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub query_result_status_support: VkBool32,
}
impl Default for VkQueueFamilyQueryResultStatusPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      query_result_status_support: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyVideoPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyVideoPropertiesKHR.html) (structure)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkQueueFamilyVideoPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: bitmask
  pub video_codec_operations: VkVideoCodecOperationFlagsKHR,
}
impl Default for VkQueueFamilyVideoPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      video_codec_operations: Default::default(),
    }
  }
}

/// Khronos: [VkRayTracingPipelineCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Pipeline creation flags
  /// * Optional: true
  pub flags: VkPipelineCreateFlags,
  /// * Optional: true
  pub stage_count: u32,
  /// One entry for each active shader stage
  /// * Len: `stage_count`
  pub stages: *const VkPipelineShaderStageCreateInfo,
  /// * Optional: true
  pub group_count: u32,
  /// * Len: `group_count`
  pub groups: *const VkRayTracingShaderGroupCreateInfoKHR,
  pub max_pipeline_ray_recursion_depth: u32,
  /// * Optional: true
  pub library_info: *const VkPipelineLibraryCreateInfoKHR,
  /// * Optional: true
  pub library_interface: *const VkRayTracingPipelineInterfaceCreateInfoKHR,
  /// * Optional: true
  pub dynamic_state: *const VkPipelineDynamicStateCreateInfo,
  /// Interface layout of the pipeline
  pub layout: VkPipelineLayout,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * Optional: true
  /// * No Auto-Validity
  pub base_pipeline_handle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  pub base_pipeline_index: i32,
}
impl Default for VkRayTracingPipelineCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      stage_count: Default::default(),
      stages: core::ptr::null(),
      group_count: Default::default(),
      groups: core::ptr::null(),
      max_pipeline_ray_recursion_depth: Default::default(),
      library_info: core::ptr::null(),
      library_interface: core::ptr::null(),
      dynamic_state: core::ptr::null(),
      layout: Default::default(),
      base_pipeline_handle: Default::default(),
      base_pipeline_index: Default::default(),
    }
  }
}

/// Khronos: [VkRayTracingPipelineCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Pipeline creation flags
  /// * Optional: true
  pub flags: VkPipelineCreateFlags,
  pub stage_count: u32,
  /// One entry for each active shader stage
  /// * Len: `stage_count`
  pub stages: *const VkPipelineShaderStageCreateInfo,
  pub group_count: u32,
  /// * Len: `group_count`
  pub groups: *const VkRayTracingShaderGroupCreateInfoNV,
  pub max_recursion_depth: u32,
  /// Interface layout of the pipeline
  pub layout: VkPipelineLayout,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * Optional: true
  /// * No Auto-Validity
  pub base_pipeline_handle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  pub base_pipeline_index: i32,
}
impl Default for VkRayTracingPipelineCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV,
      next: core::ptr::null(),
      flags: Default::default(),
      stage_count: Default::default(),
      stages: core::ptr::null(),
      group_count: Default::default(),
      groups: core::ptr::null(),
      max_recursion_depth: Default::default(),
      layout: Default::default(),
      base_pipeline_handle: Default::default(),
      base_pipeline_index: Default::default(),
    }
  }
}

/// Khronos: [VkRayTracingPipelineInterfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub max_pipeline_ray_payload_size: u32,
  pub max_pipeline_ray_hit_attribute_size: u32,
}
impl Default for VkRayTracingPipelineInterfaceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      max_pipeline_ray_payload_size: Default::default(),
      max_pipeline_ray_hit_attribute_size: Default::default(),
    }
  }
}

/// Khronos: [VkRayTracingShaderGroupCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub ty: VkRayTracingShaderGroupTypeKHR,
  pub general_shader: u32,
  pub closest_hit_shader: u32,
  pub any_hit_shader: u32,
  pub intersection_shader: u32,
  /// * Optional: true
  pub shader_group_capture_replay_handle: *const c_void,
}
impl Default for VkRayTracingShaderGroupCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
      next: core::ptr::null(),
      ty: Default::default(),
      general_shader: Default::default(),
      closest_hit_shader: Default::default(),
      any_hit_shader: Default::default(),
      intersection_shader: Default::default(),
      shader_group_capture_replay_handle: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRayTracingShaderGroupCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub ty: VkRayTracingShaderGroupTypeKHR,
  pub general_shader: u32,
  pub closest_hit_shader: u32,
  pub any_hit_shader: u32,
  pub intersection_shader: u32,
}
impl Default for VkRayTracingShaderGroupCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV,
      next: core::ptr::null(),
      ty: Default::default(),
      general_shader: Default::default(),
      closest_hit_shader: Default::default(),
      any_hit_shader: Default::default(),
      intersection_shader: Default::default(),
    }
  }
}

/// Khronos: [VkRect2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRect2D.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
}
impl Default for VkRect2D {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { offset: Default::default(), extent: Default::default() }
  }
}

/// Khronos: [VkRectLayerKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRectLayerKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRectLayerKHR {
  /// upper-left corner of a rectangle that has not changed, in pixels of a
  /// presentation images
  pub offset: VkOffset2D,
  /// Dimensions of a rectangle that has not changed, in pixels of a
  /// presentation images
  /// * No Auto-Validity
  pub extent: VkExtent2D,
  /// Layer of a swapchain's image(s), for stereoscopic-3D images
  pub layer: u32,
}
impl Default for VkRectLayerKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      offset: Default::default(),
      extent: Default::default(),
      layer: Default::default(),
    }
  }
}

/// Khronos: [VkRefreshCycleDurationGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshCycleDurationGOOGLE.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRefreshCycleDurationGOOGLE {
  /// Number of nanoseconds from the start of one refresh cycle to the next
  pub refresh_duration: u64,
}
impl Default for VkRefreshCycleDurationGOOGLE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { refresh_duration: Default::default() }
  }
}

/// Khronos: [VkRefreshObjectKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshObjectKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRefreshObjectKHR {
  pub object_type: VkObjectType,
  /// * Object Type: objectType
  /// * Extern Sync: true
  pub object_handle: u64,
  /// * Optional: true
  pub flags: VkRefreshObjectFlagsKHR,
}
impl Default for VkRefreshObjectKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      object_type: Default::default(),
      object_handle: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkRefreshObjectListKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshObjectListKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRefreshObjectListKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_REFRESH_OBJECT_LIST_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub object_count: u32,
  /// * Len: `object_count`
  pub objects: *const VkRefreshObjectKHR,
}
impl Default for VkRefreshObjectListKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_REFRESH_OBJECT_LIST_KHR,
      next: core::ptr::null(),
      object_count: Default::default(),
      objects: core::ptr::null(),
    }
  }
}

/// Khronos: [VkReleaseSwapchainImagesInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkReleaseSwapchainImagesInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkReleaseSwapchainImagesInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RELEASE_SWAPCHAIN_IMAGES_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Swapchain for which images are being released
  /// * Extern Sync: true
  pub swapchain: VkSwapchainKHR,
  /// Number of indices to release
  pub image_index_count: u32,
  /// Indices of which presentable images to release
  /// * Len: `image_index_count`
  pub image_indices: *const u32,
}
impl Default for VkReleaseSwapchainImagesInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RELEASE_SWAPCHAIN_IMAGES_INFO_EXT,
      next: core::ptr::null(),
      swapchain: Default::default(),
      image_index_count: Default::default(),
      image_indices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassAttachmentBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassAttachmentBeginInfo.html) (structure)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassAttachmentBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub attachment_count: u32,
  /// * Len: `attachment_count`
  pub attachments: *const VkImageView,
}
impl Default for VkRenderPassAttachmentBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO,
      next: core::ptr::null(),
      attachment_count: Default::default(),
      attachments: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassBeginInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub render_pass: VkRenderPass,
  pub framebuffer: VkFramebuffer,
  pub render_area: VkRect2D,
  /// * Optional: true
  pub clear_value_count: u32,
  /// * Len: `clear_value_count`
  /// * No Auto-Validity
  pub clear_values: *const VkClearValue,
}
impl Default for VkRenderPassBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
      next: core::ptr::null(),
      render_pass: Default::default(),
      framebuffer: Default::default(),
      render_area: Default::default(),
      clear_value_count: Default::default(),
      clear_values: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkRenderPassCreateFlags,
  /// * Optional: true
  pub attachment_count: u32,
  /// * Len: `attachment_count`
  pub attachments: *const VkAttachmentDescription,
  pub subpass_count: u32,
  /// * Len: `subpass_count`
  pub subpasses: *const VkSubpassDescription,
  /// * Optional: true
  pub dependency_count: u32,
  /// * Len: `dependency_count`
  pub dependencies: *const VkSubpassDependency,
}
impl Default for VkRenderPassCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      attachment_count: Default::default(),
      attachments: core::ptr::null(),
      subpass_count: Default::default(),
      subpasses: core::ptr::null(),
      dependency_count: Default::default(),
      dependencies: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassCreateInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassCreateInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkRenderPassCreateFlags,
  /// * Optional: true
  pub attachment_count: u32,
  /// * Len: `attachment_count`
  pub attachments: *const VkAttachmentDescription2,
  pub subpass_count: u32,
  /// * Len: `subpass_count`
  pub subpasses: *const VkSubpassDescription2,
  /// * Optional: true
  pub dependency_count: u32,
  /// * Len: `dependency_count`
  pub dependencies: *const VkSubpassDependency2,
  /// * Optional: true
  pub correlated_view_mask_count: u32,
  /// * Len: `correlated_view_mask_count`
  pub correlated_view_masks: *const u32,
}
impl Default for VkRenderPassCreateInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2,
      next: core::ptr::null(),
      flags: Default::default(),
      attachment_count: Default::default(),
      attachments: core::ptr::null(),
      subpass_count: Default::default(),
      subpasses: core::ptr::null(),
      dependency_count: Default::default(),
      dependencies: core::ptr::null(),
      correlated_view_mask_count: Default::default(),
      correlated_view_masks: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassCreationControlEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationControlEXT.html) (structure)
///
/// * Struct Extends: [`VkRenderPassCreateInfo2`]
/// * Struct Extends: [`VkSubpassDescription2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassCreationControlEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_CONTROL_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub disallow_merging: VkBool32,
}
impl Default for VkRenderPassCreationControlEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_CONTROL_EXT,
      next: core::ptr::null(),
      disallow_merging: Default::default(),
    }
  }
}

/// Khronos: [VkRenderPassCreationFeedbackCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationFeedbackCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkRenderPassCreateInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassCreationFeedbackCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub render_pass_feedback: *mut VkRenderPassCreationFeedbackInfoEXT,
}
impl Default for VkRenderPassCreationFeedbackCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT,
      next: core::ptr::null(),
      render_pass_feedback: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkRenderPassCreationFeedbackInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationFeedbackInfoEXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassCreationFeedbackInfoEXT {
  pub post_merge_subpass_count: u32,
}
impl Default for VkRenderPassCreationFeedbackInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { post_merge_subpass_count: Default::default() }
  }
}

/// Khronos: [VkRenderPassFragmentDensityMapCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkRenderPassCreateInfo`]
/// * Struct Extends: [`VkRenderPassCreateInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassFragmentDensityMapCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub fragment_density_map_attachment: VkAttachmentReference,
}
impl Default for VkRenderPassFragmentDensityMapCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT,
      next: core::ptr::null(),
      fragment_density_map_attachment: Default::default(),
    }
  }
}

/// Khronos: [VkRenderPassInputAttachmentAspectCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkRenderPassCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub aspect_reference_count: u32,
  /// * Len: `aspect_reference_count`
  pub aspect_references: *const VkInputAttachmentAspectReference,
}
impl Default for VkRenderPassInputAttachmentAspectCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO,
      next: core::ptr::null(),
      aspect_reference_count: Default::default(),
      aspect_references: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassMultiviewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassMultiviewCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkRenderPassCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassMultiviewCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub subpass_count: u32,
  /// * Len: `subpass_count`
  pub view_masks: *const u32,
  /// * Optional: true
  pub dependency_count: u32,
  /// * Len: `dependency_count`
  pub view_offsets: *const i32,
  /// * Optional: true
  pub correlation_mask_count: u32,
  /// * Len: `correlation_mask_count`
  pub correlation_masks: *const u32,
}
impl Default for VkRenderPassMultiviewCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO,
      next: core::ptr::null(),
      subpass_count: Default::default(),
      view_masks: core::ptr::null(),
      dependency_count: Default::default(),
      view_offsets: core::ptr::null(),
      correlation_mask_count: Default::default(),
      correlation_masks: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassSampleLocationsBeginInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub attachment_initial_sample_locations_count: u32,
  /// * Len: `attachment_initial_sample_locations_count`
  pub attachment_initial_sample_locations: *const VkAttachmentSampleLocationsEXT,
  /// * Optional: true
  pub post_subpass_sample_locations_count: u32,
  /// * Len: `post_subpass_sample_locations_count`
  pub post_subpass_sample_locations: *const VkSubpassSampleLocationsEXT,
}
impl Default for VkRenderPassSampleLocationsBeginInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
      next: core::ptr::null(),
      attachment_initial_sample_locations_count: Default::default(),
      attachment_initial_sample_locations: core::ptr::null(),
      post_subpass_sample_locations_count: Default::default(),
      post_subpass_sample_locations: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassSubpassFeedbackCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassSubpassFeedbackCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkSubpassDescription2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassSubpassFeedbackCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub subpass_feedback: *mut VkRenderPassSubpassFeedbackInfoEXT,
}
impl Default for VkRenderPassSubpassFeedbackCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT,
      next: core::ptr::null(),
      subpass_feedback: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkRenderPassSubpassFeedbackInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassSubpassFeedbackInfoEXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassSubpassFeedbackInfoEXT {
  pub subpass_merge_status: VkSubpassMergeStatusEXT,
  pub description: zstring::ArrayZString<VK_MAX_DESCRIPTION_SIZE>,
  pub post_merge_index: u32,
}
impl Default for VkRenderPassSubpassFeedbackInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      subpass_merge_status: Default::default(),
      description: Default::default(),
      post_merge_index: Default::default(),
    }
  }
}

/// Khronos: [VkRenderPassTransformBeginInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassTransformBeginInfoQCOM.html) (structure)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderPassTransformBeginInfoQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM`
  pub struct_ty: VkStructureType,
  /// Pointer to next structure
  /// * Optional: true
  pub next: *mut c_void,
  /// * No Auto-Validity
  pub transform: VkSurfaceTransformFlagBitsKHR,
}
impl Default for VkRenderPassTransformBeginInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM,
      next: core::ptr::null_mut(),
      transform: Default::default(),
    }
  }
}

/// Khronos: [VkRenderingAttachmentInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingAttachmentInfo.html) (structure)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingAttachmentInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub image_view: VkImageView,
  pub image_layout: VkImageLayout,
  /// * Optional: true
  pub resolve_mode: VkResolveModeFlagBits,
  /// * Optional: true
  pub resolve_image_view: VkImageView,
  pub resolve_image_layout: VkImageLayout,
  pub load_op: VkAttachmentLoadOp,
  pub store_op: VkAttachmentStoreOp,
  pub clear_value: VkClearValue,
}
impl Default for VkRenderingAttachmentInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
      next: core::ptr::null(),
      image_view: Default::default(),
      image_layout: Default::default(),
      resolve_mode: Default::default(),
      resolve_image_view: Default::default(),
      resolve_image_layout: Default::default(),
      load_op: Default::default(),
      store_op: Default::default(),
      clear_value: Default::default(),
    }
  }
}

/// Khronos: [VkRenderingFragmentDensityMapAttachmentInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentDensityMapAttachmentInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderingFragmentDensityMapAttachmentInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub image_view: VkImageView,
  pub image_layout: VkImageLayout,
}
impl Default for VkRenderingFragmentDensityMapAttachmentInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT,
      next: core::ptr::null(),
      image_view: Default::default(),
      image_layout: Default::default(),
    }
  }
}

/// Khronos: [VkRenderingFragmentShadingRateAttachmentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentShadingRateAttachmentInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderingFragmentShadingRateAttachmentInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub image_view: VkImageView,
  pub image_layout: VkImageLayout,
  pub shading_rate_attachment_texel_size: VkExtent2D,
}
impl Default for VkRenderingFragmentShadingRateAttachmentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
      next: core::ptr::null(),
      image_view: Default::default(),
      image_layout: Default::default(),
      shading_rate_attachment_texel_size: Default::default(),
    }
  }
}

/// Khronos: [VkRenderingInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkRenderingInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDERING_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkRenderingFlags,
  pub render_area: VkRect2D,
  pub layer_count: u32,
  pub view_mask: u32,
  /// * Optional: true
  pub color_attachment_count: u32,
  /// * Len: `color_attachment_count`
  pub color_attachments: *const VkRenderingAttachmentInfo,
  /// * Optional: true
  pub depth_attachment: *const VkRenderingAttachmentInfo,
  /// * Optional: true
  pub stencil_attachment: *const VkRenderingAttachmentInfo,
}
impl Default for VkRenderingInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RENDERING_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      render_area: Default::default(),
      layer_count: Default::default(),
      view_mask: Default::default(),
      color_attachment_count: Default::default(),
      color_attachments: core::ptr::null(),
      depth_attachment: core::ptr::null(),
      stencil_attachment: core::ptr::null(),
    }
  }
}

/// Khronos: [VkResolveImageInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResolveImageInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkResolveImageInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_image: VkImage,
  pub src_image_layout: VkImageLayout,
  pub dst_image: VkImage,
  pub dst_image_layout: VkImageLayout,
  pub region_count: u32,
  /// * Len: `region_count`
  pub regions: *const VkImageResolve2,
}
impl Default for VkResolveImageInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2,
      next: core::ptr::null(),
      src_image: Default::default(),
      src_image_layout: Default::default(),
      dst_image: Default::default(),
      dst_image_layout: Default::default(),
      region_count: Default::default(),
      regions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSRTDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSRTDataNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSRTDataNV {
  pub sx: c_float,
  pub a: c_float,
  pub b: c_float,
  pub pvx: c_float,
  pub sy: c_float,
  pub c: c_float,
  pub pvy: c_float,
  pub sz: c_float,
  pub pvz: c_float,
  pub qx: c_float,
  pub qy: c_float,
  pub qz: c_float,
  pub qw: c_float,
  pub tx: c_float,
  pub ty: c_float,
  pub tz: c_float,
}
impl Default for VkSRTDataNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sx: Default::default(),
      a: Default::default(),
      b: Default::default(),
      pvx: Default::default(),
      sy: Default::default(),
      c: Default::default(),
      pvy: Default::default(),
      sz: Default::default(),
      pvz: Default::default(),
      qx: Default::default(),
      qy: Default::default(),
      qz: Default::default(),
      qw: Default::default(),
      tx: Default::default(),
      ty: Default::default(),
      tz: Default::default(),
    }
  }
}

/// Khronos: [VkSampleLocationEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleLocationEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSampleLocationEXT {
  pub x: c_float,
  pub y: c_float,
}
impl Default for VkSampleLocationEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { x: Default::default(), y: Default::default() }
  }
}

/// Khronos: [VkSampleLocationsInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleLocationsInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkImageMemoryBarrier`]
/// * Struct Extends: [`VkImageMemoryBarrier2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSampleLocationsInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * No Auto-Validity
  pub sample_locations_per_pixel: VkSampleCountFlagBits,
  pub sample_location_grid_size: VkExtent2D,
  /// * Optional: true
  pub sample_locations_count: u32,
  /// * Len: `sample_locations_count`
  pub sample_locations: *const VkSampleLocationEXT,
}
impl Default for VkSampleLocationsInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT,
      next: core::ptr::null(),
      sample_locations_per_pixel: Default::default(),
      sample_location_grid_size: Default::default(),
      sample_locations_count: Default::default(),
      sample_locations: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSamplerBorderColorComponentMappingCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkSamplerCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub components: VkComponentMapping,
  pub srgb: VkBool32,
}
impl Default for VkSamplerBorderColorComponentMappingCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT,
      next: core::ptr::null(),
      components: Default::default(),
      srgb: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCaptureDescriptorDataInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSamplerCaptureDescriptorDataInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub sampler: VkSampler,
}
impl Default for VkSamplerCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      next: core::ptr::null(),
      sampler: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSamplerCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkSamplerCreateFlags,
  /// Filter mode for magnification
  pub mag_filter: VkFilter,
  /// Filter mode for minifiation
  pub min_filter: VkFilter,
  /// Mipmap selection mode
  pub mipmap_mode: VkSamplerMipmapMode,
  pub address_mode_u: VkSamplerAddressMode,
  pub address_mode_v: VkSamplerAddressMode,
  pub address_mode_w: VkSamplerAddressMode,
  pub mip_lod_bias: c_float,
  pub anisotropy_enable: VkBool32,
  pub max_anisotropy: c_float,
  pub compare_enable: VkBool32,
  /// * No Auto-Validity
  pub compare_op: VkCompareOp,
  pub min_lod: c_float,
  pub max_lod: c_float,
  /// * No Auto-Validity
  pub border_color: VkBorderColor,
  pub unnormalized_coordinates: VkBool32,
}
impl Default for VkSamplerCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      mag_filter: Default::default(),
      min_filter: Default::default(),
      mipmap_mode: Default::default(),
      address_mode_u: Default::default(),
      address_mode_v: Default::default(),
      address_mode_w: Default::default(),
      mip_lod_bias: Default::default(),
      anisotropy_enable: Default::default(),
      max_anisotropy: Default::default(),
      compare_enable: Default::default(),
      compare_op: Default::default(),
      min_lod: Default::default(),
      max_lod: Default::default(),
      border_color: Default::default(),
      unnormalized_coordinates: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerReductionModeCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionModeCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkSamplerCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSamplerReductionModeCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub reduction_mode: VkSamplerReductionMode,
}
impl Default for VkSamplerReductionModeCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO,
      next: core::ptr::null(),
      reduction_mode: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerYcbcrConversionCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSamplerYcbcrConversionCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub format: VkFormat,
  pub ycbcr_model: VkSamplerYcbcrModelConversion,
  pub ycbcr_range: VkSamplerYcbcrRange,
  pub components: VkComponentMapping,
  pub x_chroma_offset: VkChromaLocation,
  pub y_chroma_offset: VkChromaLocation,
  pub chroma_filter: VkFilter,
  pub force_explicit_reconstruction: VkBool32,
}
impl Default for VkSamplerYcbcrConversionCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO,
      next: core::ptr::null(),
      format: Default::default(),
      ycbcr_model: Default::default(),
      ycbcr_range: Default::default(),
      components: Default::default(),
      x_chroma_offset: Default::default(),
      y_chroma_offset: Default::default(),
      chroma_filter: Default::default(),
      force_explicit_reconstruction: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerYcbcrConversionImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html) (structure)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSamplerYcbcrConversionImageFormatProperties {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub combined_image_sampler_descriptor_count: u32,
}
impl Default for VkSamplerYcbcrConversionImageFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES,
      next: core::ptr::null_mut(),
      combined_image_sampler_descriptor_count: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerYcbcrConversionInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionInfo.html) (structure)
///
/// * Struct Extends: [`VkSamplerCreateInfo`]
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSamplerYcbcrConversionInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub conversion: VkSamplerYcbcrConversion,
}
impl Default for VkSamplerYcbcrConversionInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO,
      next: core::ptr::null(),
      conversion: Default::default(),
    }
  }
}

/// Khronos: [VkSciSyncAttributesInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSciSyncAttributesInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSciSyncAttributesInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SCI_SYNC_ATTRIBUTES_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub client_type: VkSciSyncClientTypeNV,
  pub primitive_type: VkSciSyncPrimitiveTypeNV,
}
impl Default for VkSciSyncAttributesInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SCI_SYNC_ATTRIBUTES_INFO_NV,
      next: core::ptr::null(),
      client_type: Default::default(),
      primitive_type: Default::default(),
    }
  }
}

/// Khronos: [VkScreenSurfaceCreateInfoQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateInfoQNX.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkScreenSurfaceCreateInfoQNX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkScreenSurfaceCreateFlagsQNX,
  /// * No Auto-Validity
  pub context: *mut _screen_context,
  /// * No Auto-Validity
  pub window: *mut _screen_window,
}
impl Default for VkScreenSurfaceCreateInfoQNX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX,
      next: core::ptr::null(),
      flags: Default::default(),
      context: core::ptr::null_mut(),
      window: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkSemaphoreCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSemaphoreCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Semaphore creation flags
  /// * Optional: true
  pub flags: VkSemaphoreCreateFlags,
}
impl Default for VkSemaphoreCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreGetFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetFdInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSemaphoreGetFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub semaphore: VkSemaphore,
  pub handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkSemaphoreGetFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR,
      next: core::ptr::null(),
      semaphore: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreGetSciSyncInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetSciSyncInfoNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSemaphoreGetSciSyncInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_GET_SCI_SYNC_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub semaphore: VkSemaphore,
  pub handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkSemaphoreGetSciSyncInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SEMAPHORE_GET_SCI_SYNC_INFO_NV,
      next: core::ptr::null(),
      semaphore: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreGetWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetWin32HandleInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub semaphore: VkSemaphore,
  pub handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkSemaphoreGetWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
      next: core::ptr::null(),
      semaphore: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreGetZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetZirconHandleInfoFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub semaphore: VkSemaphore,
  pub handle_type: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkSemaphoreGetZirconHandleInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
      next: core::ptr::null(),
      semaphore: Default::default(),
      handle_type: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreSignalInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSignalInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSemaphoreSignalInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub semaphore: VkSemaphore,
  pub value: u64,
}
impl Default for VkSemaphoreSignalInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO,
      next: core::ptr::null(),
      semaphore: Default::default(),
      value: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSubmitInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSemaphoreSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub semaphore: VkSemaphore,
  pub value: u64,
  /// * Optional: true
  pub stage_mask: VkPipelineStageFlags2,
  pub device_index: u32,
}
impl Default for VkSemaphoreSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO,
      next: core::ptr::null(),
      semaphore: Default::default(),
      value: Default::default(),
      stage_mask: Default::default(),
      device_index: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreTypeCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
/// * Struct Extends: [`VkPhysicalDeviceExternalSemaphoreInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSemaphoreTypeCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub semaphore_type: VkSemaphoreType,
  pub initial_value: u64,
}
impl Default for VkSemaphoreTypeCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO,
      next: core::ptr::null(),
      semaphore_type: Default::default(),
      initial_value: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreWaitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSemaphoreWaitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkSemaphoreWaitFlags,
  pub semaphore_count: u32,
  /// * Len: `semaphore_count`
  pub semaphores: *const VkSemaphore,
  /// * Len: `semaphore_count`
  pub values: *const u64,
}
impl Default for VkSemaphoreWaitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      semaphore_count: Default::default(),
      semaphores: core::ptr::null(),
      values: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSetStateFlagsIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSetStateFlagsIndirectCommandNV {
  pub data: u32,
}
impl Default for VkSetStateFlagsIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { data: Default::default() }
  }
}

/// Khronos: [VkShaderModuleCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateInfo.html) (structure)
///
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkShaderModuleCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO`
  pub struct_ty: VkStructureType,
  /// noautovalidity because this structure can be either an explicit parameter,
  /// or passed in a pNext chain
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkShaderModuleCreateFlags,
  /// Specified in bytes
  pub code_size: c_size_t,
  /// Binary code of size codeSize
  /// * Len: `code_size / 4`
  pub code: *const u32,
}
impl Default for VkShaderModuleCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      code_size: Default::default(),
      code: core::ptr::null(),
    }
  }
}

/// Khronos: [VkShaderModuleIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleIdentifierEXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkShaderModuleIdentifierEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SHADER_MODULE_IDENTIFIER_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * No Auto-Validity
  pub identifier_size: u32,
  pub identifier: zstring::ArrayZString<VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT>,
}
impl Default for VkShaderModuleIdentifierEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SHADER_MODULE_IDENTIFIER_EXT,
      next: core::ptr::null_mut(),
      identifier_size: Default::default(),
      identifier: Default::default(),
    }
  }
}

/// Khronos: [VkShaderModuleValidationCacheCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkShaderModuleCreateInfo`]
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub validation_cache: VkValidationCacheEXT,
}
impl Default for VkShaderModuleValidationCacheCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      validation_cache: Default::default(),
    }
  }
}

/// Khronos: [VkShaderResourceUsageAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderResourceUsageAMD.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkShaderResourceUsageAMD {
  pub num_used_vgprs: u32,
  pub num_used_sgprs: u32,
  pub lds_size_per_local_work_group: u32,
  pub lds_usage_size_in_bytes: c_size_t,
  pub scratch_mem_usage_in_bytes: c_size_t,
}
impl Default for VkShaderResourceUsageAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      num_used_vgprs: Default::default(),
      num_used_sgprs: Default::default(),
      lds_size_per_local_work_group: Default::default(),
      lds_usage_size_in_bytes: Default::default(),
      scratch_mem_usage_in_bytes: Default::default(),
    }
  }
}

/// Khronos: [VkShaderStatisticsInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStatisticsInfoAMD.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkShaderStatisticsInfoAMD {
  pub shader_stage_mask: VkShaderStageFlags,
  pub resource_usage: VkShaderResourceUsageAMD,
  pub num_physical_vgprs: u32,
  pub num_physical_sgprs: u32,
  pub num_available_vgprs: u32,
  pub num_available_sgprs: u32,
  pub compute_work_group_size: [u32; 3],
}
impl Default for VkShaderStatisticsInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      shader_stage_mask: Default::default(),
      resource_usage: Default::default(),
      num_physical_vgprs: Default::default(),
      num_physical_sgprs: Default::default(),
      num_available_vgprs: Default::default(),
      num_available_sgprs: Default::default(),
      compute_work_group_size: [Default::default(); 3],
    }
  }
}

/// Khronos: [VkShadingRatePaletteNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkShadingRatePaletteNV {
  pub shading_rate_palette_entry_count: u32,
  /// * Len: `shading_rate_palette_entry_count`
  pub shading_rate_palette_entries: *const VkShadingRatePaletteEntryNV,
}
impl Default for VkShadingRatePaletteNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      shading_rate_palette_entry_count: Default::default(),
      shading_rate_palette_entries: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSharedPresentSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html) (structure)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// Supported image usage flags if swapchain created using a shared present
  /// mode
  /// * Optional: true
  pub shared_present_supported_usage_flags: VkImageUsageFlags,
}
impl Default for VkSharedPresentSurfaceCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR,
      next: core::ptr::null_mut(),
      shared_present_supported_usage_flags: Default::default(),
    }
  }
}

/// Khronos: [VkSparseBufferMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseBufferMemoryBindInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
  pub buffer: VkBuffer,
  pub bind_count: u32,
  /// * Len: `bind_count`
  pub binds: *const VkSparseMemoryBind,
}
impl Default for VkSparseBufferMemoryBindInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      buffer: Default::default(),
      bind_count: Default::default(),
      binds: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSparseImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSparseImageFormatProperties {
  /// * Optional: true
  /// * Limit Type: bitmask
  pub aspect_mask: VkImageAspectFlags,
  /// * Limit Type: min,mul
  pub image_granularity: VkExtent3D,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub flags: VkSparseImageFormatFlags,
}
impl Default for VkSparseImageFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      aspect_mask: Default::default(),
      image_granularity: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties2.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSparseImageFormatProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Limit Type: struct
  pub properties: VkSparseImageFormatProperties,
}
impl Default for VkSparseImageFormatProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2,
      next: core::ptr::null_mut(),
      properties: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageMemoryBind](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBind.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSparseImageMemoryBind {
  pub subresource: VkImageSubresource,
  pub offset: VkOffset3D,
  pub extent: VkExtent3D,
  /// * Optional: true
  pub memory: VkDeviceMemory,
  /// Specified in bytes
  pub memory_offset: VkDeviceSize,
  /// * Optional: true
  pub flags: VkSparseMemoryBindFlags,
}
impl Default for VkSparseImageMemoryBind {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      subresource: Default::default(),
      offset: Default::default(),
      extent: Default::default(),
      memory: Default::default(),
      memory_offset: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBindInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
  pub image: VkImage,
  pub bind_count: u32,
  /// * Len: `bind_count`
  pub binds: *const VkSparseImageMemoryBind,
}
impl Default for VkSparseImageMemoryBindInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      image: Default::default(),
      bind_count: Default::default(),
      binds: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSparseImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSparseImageMemoryRequirements {
  pub format_properties: VkSparseImageFormatProperties,
  pub image_mip_tail_first_lod: u32,
  /// Specified in bytes, must be a multiple of sparse block size in bytes /
  /// alignment
  pub image_mip_tail_size: VkDeviceSize,
  /// Specified in bytes, must be a multiple of sparse block size in bytes /
  /// alignment
  pub image_mip_tail_offset: VkDeviceSize,
  /// Specified in bytes, must be a multiple of sparse block size in bytes /
  /// alignment
  pub image_mip_tail_stride: VkDeviceSize,
}
impl Default for VkSparseImageMemoryRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      format_properties: Default::default(),
      image_mip_tail_first_lod: Default::default(),
      image_mip_tail_size: Default::default(),
      image_mip_tail_offset: Default::default(),
      image_mip_tail_stride: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements2.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSparseImageMemoryRequirements2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_requirements: VkSparseImageMemoryRequirements,
}
impl Default for VkSparseImageMemoryRequirements2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2,
      next: core::ptr::null_mut(),
      memory_requirements: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageOpaqueMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
  pub image: VkImage,
  pub bind_count: u32,
  /// * Len: `bind_count`
  pub binds: *const VkSparseMemoryBind,
}
impl Default for VkSparseImageOpaqueMemoryBindInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      image: Default::default(),
      bind_count: Default::default(),
      binds: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSparseMemoryBind](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBind.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSparseMemoryBind {
  /// Specified in bytes
  pub resource_offset: VkDeviceSize,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// * Optional: true
  pub memory: VkDeviceMemory,
  /// Specified in bytes
  pub memory_offset: VkDeviceSize,
  /// * Optional: true
  pub flags: VkSparseMemoryBindFlags,
}
impl Default for VkSparseMemoryBind {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      resource_offset: Default::default(),
      size: Default::default(),
      memory: Default::default(),
      memory_offset: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkSpecializationInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSpecializationInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSpecializationInfo {
  /// Number of entries in the map
  /// * Optional: true
  pub map_entry_count: u32,
  /// Array of map entries
  /// * Len: `map_entry_count`
  pub map_entries: *const VkSpecializationMapEntry,
  /// Size in bytes of pData
  /// * Optional: true
  pub data_size: c_size_t,
  /// Pointer to SpecConstant data
  /// * Len: `data_size`
  pub data: *const c_void,
}
impl Default for VkSpecializationInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      map_entry_count: Default::default(),
      map_entries: core::ptr::null(),
      data_size: Default::default(),
      data: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSpecializationMapEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSpecializationMapEntry.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSpecializationMapEntry {
  /// The SpecConstant ID specified in the BIL
  pub constant_id: u32,
  /// Offset of the value in the data block
  pub offset: u32,
  /// Size in bytes of the SpecConstant
  /// * No Auto-Validity
  pub size: c_size_t,
}
impl Default for VkSpecializationMapEntry {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      constant_id: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkStencilOpState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilOpState.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkStencilOpState {
  pub fail_op: VkStencilOp,
  pub pass_op: VkStencilOp,
  pub depth_fail_op: VkStencilOp,
  pub compare_op: VkCompareOp,
  pub compare_mask: u32,
  pub write_mask: u32,
  pub reference: u32,
}
impl Default for VkStencilOpState {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      fail_op: Default::default(),
      pass_op: Default::default(),
      depth_fail_op: Default::default(),
      compare_op: Default::default(),
      compare_mask: Default::default(),
      write_mask: Default::default(),
      reference: Default::default(),
    }
  }
}

/// Khronos: [VkStridedDeviceAddressRegionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStridedDeviceAddressRegionKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkStridedDeviceAddressRegionKHR {
  /// * Optional: true
  pub device_address: VkDeviceAddress,
  pub stride: VkDeviceSize,
  pub size: VkDeviceSize,
}
impl Default for VkStridedDeviceAddressRegionKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      device_address: Default::default(),
      stride: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBMIT_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub wait_semaphore_count: u32,
  /// * Len: `wait_semaphore_count`
  pub wait_semaphores: *const VkSemaphore,
  /// * Optional: false,true
  /// * Len: `wait_semaphore_count`
  pub wait_dst_stage_mask: *const VkPipelineStageFlags,
  /// * Optional: true
  pub command_buffer_count: u32,
  /// * Len: `command_buffer_count`
  pub command_buffers: *const VkCommandBuffer,
  /// * Optional: true
  pub signal_semaphore_count: u32,
  /// * Len: `signal_semaphore_count`
  pub signal_semaphores: *const VkSemaphore,
}
impl Default for VkSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBMIT_INFO,
      next: core::ptr::null(),
      wait_semaphore_count: Default::default(),
      wait_semaphores: core::ptr::null(),
      wait_dst_stage_mask: core::ptr::null(),
      command_buffer_count: Default::default(),
      command_buffers: core::ptr::null(),
      signal_semaphore_count: Default::default(),
      signal_semaphores: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubmitInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubmitInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBMIT_INFO_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkSubmitFlags,
  /// * Optional: true
  pub wait_semaphore_info_count: u32,
  /// * Len: `wait_semaphore_info_count`
  pub wait_semaphore_infos: *const VkSemaphoreSubmitInfo,
  /// * Optional: true
  pub command_buffer_info_count: u32,
  /// * Len: `command_buffer_info_count`
  pub command_buffer_infos: *const VkCommandBufferSubmitInfo,
  /// * Optional: true
  pub signal_semaphore_info_count: u32,
  /// * Len: `signal_semaphore_info_count`
  pub signal_semaphore_infos: *const VkSemaphoreSubmitInfo,
}
impl Default for VkSubmitInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBMIT_INFO_2,
      next: core::ptr::null(),
      flags: Default::default(),
      wait_semaphore_info_count: Default::default(),
      wait_semaphore_infos: core::ptr::null(),
      command_buffer_info_count: Default::default(),
      command_buffer_infos: core::ptr::null(),
      signal_semaphore_info_count: Default::default(),
      signal_semaphore_infos: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassBeginInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub contents: VkSubpassContents,
}
impl Default for VkSubpassBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO,
      next: core::ptr::null(),
      contents: Default::default(),
    }
  }
}

/// Khronos: [VkSubpassDependency](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassDependency {
  pub src_subpass: u32,
  pub dst_subpass: u32,
  /// * Optional: true
  pub src_stage_mask: VkPipelineStageFlags,
  /// * Optional: true
  pub dst_stage_mask: VkPipelineStageFlags,
  /// Memory accesses from the source of the dependency to synchronize
  /// * Optional: true
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * Optional: true
  pub dst_access_mask: VkAccessFlags,
  /// * Optional: true
  pub dependency_flags: VkDependencyFlags,
}
impl Default for VkSubpassDependency {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      src_subpass: Default::default(),
      dst_subpass: Default::default(),
      src_stage_mask: Default::default(),
      dst_stage_mask: Default::default(),
      src_access_mask: Default::default(),
      dst_access_mask: Default::default(),
      dependency_flags: Default::default(),
    }
  }
}

/// Khronos: [VkSubpassDependency2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassDependency2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub src_subpass: u32,
  pub dst_subpass: u32,
  /// * Optional: true
  pub src_stage_mask: VkPipelineStageFlags,
  /// * Optional: true
  pub dst_stage_mask: VkPipelineStageFlags,
  /// * Optional: true
  pub src_access_mask: VkAccessFlags,
  /// * Optional: true
  pub dst_access_mask: VkAccessFlags,
  /// * Optional: true
  pub dependency_flags: VkDependencyFlags,
  pub view_offset: i32,
}
impl Default for VkSubpassDependency2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2,
      next: core::ptr::null(),
      src_subpass: Default::default(),
      dst_subpass: Default::default(),
      src_stage_mask: Default::default(),
      dst_stage_mask: Default::default(),
      src_access_mask: Default::default(),
      dst_access_mask: Default::default(),
      dependency_flags: Default::default(),
      view_offset: Default::default(),
    }
  }
}

/// Khronos: [VkSubpassDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassDescription {
  /// * Optional: true
  pub flags: VkSubpassDescriptionFlags,
  /// Must be VK_PIPELINE_BIND_POINT_GRAPHICS for now
  pub pipeline_bind_point: VkPipelineBindPoint,
  /// * Optional: true
  pub input_attachment_count: u32,
  /// * Len: `input_attachment_count`
  pub input_attachments: *const VkAttachmentReference,
  /// * Optional: true
  pub color_attachment_count: u32,
  /// * Len: `color_attachment_count`
  pub color_attachments: *const VkAttachmentReference,
  /// * Optional: true
  /// * Len: `color_attachment_count`
  pub resolve_attachments: *const VkAttachmentReference,
  /// * Optional: true
  pub depth_stencil_attachment: *const VkAttachmentReference,
  /// * Optional: true
  pub preserve_attachment_count: u32,
  /// * Len: `preserve_attachment_count`
  pub preserve_attachments: *const u32,
}
impl Default for VkSubpassDescription {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      flags: Default::default(),
      pipeline_bind_point: Default::default(),
      input_attachment_count: Default::default(),
      input_attachments: core::ptr::null(),
      color_attachment_count: Default::default(),
      color_attachments: core::ptr::null(),
      resolve_attachments: core::ptr::null(),
      depth_stencil_attachment: core::ptr::null(),
      preserve_attachment_count: Default::default(),
      preserve_attachments: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassDescription2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription2.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassDescription2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkSubpassDescriptionFlags,
  pub pipeline_bind_point: VkPipelineBindPoint,
  pub view_mask: u32,
  /// * Optional: true
  pub input_attachment_count: u32,
  /// * Len: `input_attachment_count`
  pub input_attachments: *const VkAttachmentReference2,
  /// * Optional: true
  pub color_attachment_count: u32,
  /// * Len: `color_attachment_count`
  pub color_attachments: *const VkAttachmentReference2,
  /// * Optional: true
  /// * Len: `color_attachment_count`
  pub resolve_attachments: *const VkAttachmentReference2,
  /// * Optional: true
  pub depth_stencil_attachment: *const VkAttachmentReference2,
  /// * Optional: true
  pub preserve_attachment_count: u32,
  /// * Len: `preserve_attachment_count`
  pub preserve_attachments: *const u32,
}
impl Default for VkSubpassDescription2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2,
      next: core::ptr::null(),
      flags: Default::default(),
      pipeline_bind_point: Default::default(),
      view_mask: Default::default(),
      input_attachment_count: Default::default(),
      input_attachments: core::ptr::null(),
      color_attachment_count: Default::default(),
      color_attachments: core::ptr::null(),
      resolve_attachments: core::ptr::null(),
      depth_stencil_attachment: core::ptr::null(),
      preserve_attachment_count: Default::default(),
      preserve_attachments: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassDescriptionDepthStencilResolve](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionDepthStencilResolve.html) (structure)
///
/// * Struct Extends: [`VkSubpassDescription2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassDescriptionDepthStencilResolve {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// depth resolve mode
  /// * No Auto-Validity
  pub depth_resolve_mode: VkResolveModeFlagBits,
  /// stencil resolve mode
  /// * No Auto-Validity
  pub stencil_resolve_mode: VkResolveModeFlagBits,
  /// depth/stencil resolve attachment
  /// * Optional: true
  pub depth_stencil_resolve_attachment: *const VkAttachmentReference2,
}
impl Default for VkSubpassDescriptionDepthStencilResolve {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE,
      next: core::ptr::null(),
      depth_resolve_mode: Default::default(),
      stencil_resolve_mode: Default::default(),
      depth_stencil_resolve_attachment: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassEndInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassEndInfo.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassEndInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_END_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
}
impl Default for VkSubpassEndInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { struct_ty: VK_STRUCTURE_TYPE_SUBPASS_END_INFO, next: core::ptr::null() }
  }
}

/// Khronos: [VkSubpassFragmentDensityMapOffsetEndInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html) (structure)
///
/// * Struct Extends: [`VkSubpassEndInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub fragment_density_offset_count: u32,
  /// * Len: `fragment_density_offset_count`
  pub fragment_density_offsets: *const VkOffset2D,
}
impl Default for VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM,
      next: core::ptr::null(),
      fragment_density_offset_count: Default::default(),
      fragment_density_offsets: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassResolvePerformanceQueryEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassResolvePerformanceQueryEXT.html) (structure)
///
/// * Struct Extends: [`VkFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassResolvePerformanceQueryEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub optimal: VkBool32,
}
impl Default for VkSubpassResolvePerformanceQueryEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT,
      next: core::ptr::null_mut(),
      optimal: Default::default(),
    }
  }
}

/// Khronos: [VkSubpassSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassSampleLocationsEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassSampleLocationsEXT {
  pub subpass_index: u32,
  pub sample_locations_info: VkSampleLocationsInfoEXT,
}
impl Default for VkSubpassSampleLocationsEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { subpass_index: Default::default(), sample_locations_info: Default::default() }
  }
}

/// Khronos: [VkSubpassShadingPipelineCreateInfoHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html) (structure)
///
/// * Struct Extends: [`VkComputePipelineCreateInfo`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub render_pass: VkRenderPass,
  pub subpass: u32,
}
impl Default for VkSubpassShadingPipelineCreateInfoHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI,
      next: core::ptr::null_mut(),
      render_pass: Default::default(),
      subpass: Default::default(),
    }
  }
}

/// Khronos: [VkSubresourceLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubresourceLayout.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubresourceLayout {
  /// Specified in bytes
  pub offset: VkDeviceSize,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Specified in bytes
  pub row_pitch: VkDeviceSize,
  /// Specified in bytes
  pub array_pitch: VkDeviceSize,
  /// Specified in bytes
  pub depth_pitch: VkDeviceSize,
}
impl Default for VkSubresourceLayout {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      offset: Default::default(),
      size: Default::default(),
      row_pitch: Default::default(),
      array_pitch: Default::default(),
      depth_pitch: Default::default(),
    }
  }
}

/// Khronos: [VkSubresourceLayout2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubresourceLayout2EXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSubresourceLayout2EXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub subresource_layout: VkSubresourceLayout,
}
impl Default for VkSubresourceLayout2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT,
      next: core::ptr::null_mut(),
      subresource_layout: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilities2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2EXT.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceCapabilities2EXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// Supported minimum number of images for the surface
  pub min_image_count: u32,
  /// Supported maximum number of images for the surface, 0 for unlimited
  pub max_image_count: u32,
  /// Current image width and height for the surface, (0, 0) if undefined
  pub current_extent: VkExtent2D,
  /// Supported minimum image width and height for the surface
  pub min_image_extent: VkExtent2D,
  /// Supported maximum image width and height for the surface
  pub max_image_extent: VkExtent2D,
  /// Supported maximum number of image layers for the surface
  pub max_image_array_layers: u32,
  /// 1 or more bits representing the transforms supported
  pub supported_transforms: VkSurfaceTransformFlagsKHR,
  /// The surface's current transform relative to the device's natural
  /// orientation
  pub current_transform: VkSurfaceTransformFlagBitsKHR,
  /// 1 or more bits representing the alpha compositing modes supported
  pub supported_composite_alpha: VkCompositeAlphaFlagsKHR,
  /// Supported image usage flags for the surface
  pub supported_usage_flags: VkImageUsageFlags,
  /// * Optional: true
  pub supported_surface_counters: VkSurfaceCounterFlagsEXT,
}
impl Default for VkSurfaceCapabilities2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT,
      next: core::ptr::null_mut(),
      min_image_count: Default::default(),
      max_image_count: Default::default(),
      current_extent: Default::default(),
      min_image_extent: Default::default(),
      max_image_extent: Default::default(),
      max_image_array_layers: Default::default(),
      supported_transforms: Default::default(),
      current_transform: Default::default(),
      supported_composite_alpha: Default::default(),
      supported_usage_flags: Default::default(),
      supported_surface_counters: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2KHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceCapabilities2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub surface_capabilities: VkSurfaceCapabilitiesKHR,
}
impl Default for VkSurfaceCapabilities2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR,
      next: core::ptr::null_mut(),
      surface_capabilities: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilitiesFullScreenExclusiveEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html) (structure)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub full_screen_exclusive_supported: VkBool32,
}
impl Default for VkSurfaceCapabilitiesFullScreenExclusiveEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT,
      next: core::ptr::null_mut(),
      full_screen_exclusive_supported: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
  /// Supported minimum number of images for the surface
  pub min_image_count: u32,
  /// Supported maximum number of images for the surface, 0 for unlimited
  pub max_image_count: u32,
  /// Current image width and height for the surface, (0, 0) if undefined
  pub current_extent: VkExtent2D,
  /// Supported minimum image width and height for the surface
  pub min_image_extent: VkExtent2D,
  /// Supported maximum image width and height for the surface
  pub max_image_extent: VkExtent2D,
  /// Supported maximum number of image layers for the surface
  pub max_image_array_layers: u32,
  /// 1 or more bits representing the transforms supported
  pub supported_transforms: VkSurfaceTransformFlagsKHR,
  /// The surface's current transform relative to the device's natural
  /// orientation
  pub current_transform: VkSurfaceTransformFlagBitsKHR,
  /// 1 or more bits representing the alpha compositing modes supported
  pub supported_composite_alpha: VkCompositeAlphaFlagsKHR,
  /// Supported image usage flags for the surface
  pub supported_usage_flags: VkImageUsageFlags,
}
impl Default for VkSurfaceCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      min_image_count: Default::default(),
      max_image_count: Default::default(),
      current_extent: Default::default(),
      min_image_extent: Default::default(),
      max_image_extent: Default::default(),
      max_image_array_layers: Default::default(),
      supported_transforms: Default::default(),
      current_transform: Default::default(),
      supported_composite_alpha: Default::default(),
      supported_usage_flags: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilitiesPresentBarrierNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesPresentBarrierNV.html) (structure)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentBarrierNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_PRESENT_BARRIER_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub present_barrier_supported: VkBool32,
}
impl Default for VkSurfaceCapabilitiesPresentBarrierNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_PRESENT_BARRIER_NV,
      next: core::ptr::null_mut(),
      present_barrier_supported: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceFormat2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormat2KHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceFormat2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub surface_format: VkSurfaceFormatKHR,
}
impl Default for VkSurfaceFormat2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR,
      next: core::ptr::null_mut(),
      surface_format: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceFormatKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceFormatKHR {
  /// Supported pair of rendering format
  pub format: VkFormat,
  /// and color space for the surface
  pub color_space: VkColorSpaceKHR,
}
impl Default for VkSurfaceFormatKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { format: Default::default(), color_space: Default::default() }
  }
}

/// Khronos: [VkSurfaceFullScreenExclusiveInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceSurfaceInfo2KHR`]
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub full_screen_exclusive: VkFullScreenExclusiveEXT,
}
impl Default for VkSurfaceFullScreenExclusiveInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
      next: core::ptr::null_mut(),
      full_screen_exclusive: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceFullScreenExclusiveWin32InfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceSurfaceInfo2KHR`]
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub hmonitor: HMONITOR,
}
impl Default for VkSurfaceFullScreenExclusiveWin32InfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT,
      next: core::ptr::null(),
      hmonitor: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkSurfacePresentModeCompatibilityEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfacePresentModeCompatibilityEXT.html) (structure)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfacePresentModeCompatibilityEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  pub present_mode_count: u32,
  /// Output list of present modes compatible with the one specified in
  /// VkSurfacePresentModeEXT
  /// * Optional: true
  /// * Len: `present_mode_count`
  pub present_modes: *mut VkPresentModeKHR,
}
impl Default for VkSurfacePresentModeCompatibilityEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT,
      next: core::ptr::null_mut(),
      present_mode_count: Default::default(),
      present_modes: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkSurfacePresentModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfacePresentModeEXT.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceSurfaceInfo2KHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfacePresentModeEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub present_mode: VkPresentModeKHR,
}
impl Default for VkSurfacePresentModeEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT,
      next: core::ptr::null_mut(),
      present_mode: Default::default(),
    }
  }
}

/// Khronos: [VkSurfacePresentScalingCapabilitiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfacePresentScalingCapabilitiesEXT.html) (structure)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfacePresentScalingCapabilitiesEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * Optional: true
  pub supported_present_scaling: VkPresentScalingFlagsEXT,
  /// * Optional: true
  pub supported_present_gravity_x: VkPresentGravityFlagsEXT,
  /// * Optional: true
  pub supported_present_gravity_y: VkPresentGravityFlagsEXT,
  /// Supported minimum image width and height for the surface when scaling is
  /// used
  /// * Optional: true
  pub min_scaled_image_extent: VkExtent2D,
  /// Supported maximum image width and height for the surface when scaling is
  /// used
  /// * Optional: true
  pub max_scaled_image_extent: VkExtent2D,
}
impl Default for VkSurfacePresentScalingCapabilitiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT,
      next: core::ptr::null_mut(),
      supported_present_scaling: Default::default(),
      supported_present_gravity_x: Default::default(),
      supported_present_gravity_y: Default::default(),
      min_scaled_image_extent: Default::default(),
      max_scaled_image_extent: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceProtectedCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html) (structure)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSurfaceProtectedCapabilitiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Represents if surface can be protected
  pub supports_protected: VkBool32,
}
impl Default for VkSurfaceProtectedCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR,
      next: core::ptr::null(),
      supports_protected: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainCounterCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSwapchainCounterCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub surface_counters: VkSurfaceCounterFlagsEXT,
}
impl Default for VkSwapchainCounterCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
      next: core::ptr::null(),
      surface_counters: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkSwapchainCreateFlagsKHR,
  /// The swapchain's target surface
  pub surface: VkSurfaceKHR,
  /// Minimum number of presentation images the application needs
  pub min_image_count: u32,
  /// Format of the presentation images
  pub image_format: VkFormat,
  /// Colorspace of the presentation images
  pub image_color_space: VkColorSpaceKHR,
  /// Dimensions of the presentation images
  pub image_extent: VkExtent2D,
  /// Determines the number of views for multiview/stereo presentation
  pub image_array_layers: u32,
  /// Bits indicating how the presentation images will be used
  pub image_usage: VkImageUsageFlags,
  /// Sharing mode used for the presentation images
  pub image_sharing_mode: VkSharingMode,
  /// Number of queue families having access to the images in case of concurrent
  /// sharing mode
  /// * Optional: true
  pub queue_family_index_count: u32,
  /// Array of queue family indices having access to the images in case of
  /// concurrent sharing mode
  /// * Len: `queue_family_index_count`
  /// * No Auto-Validity
  pub queue_family_indices: *const u32,
  /// The transform, relative to the device's natural orientation, applied to
  /// the image content prior to presentation
  pub pre_transform: VkSurfaceTransformFlagBitsKHR,
  /// The alpha blending mode used when compositing this surface with other
  /// surfaces in the window system
  pub composite_alpha: VkCompositeAlphaFlagBitsKHR,
  /// Which presentation mode to use for presents on this swap chain
  pub present_mode: VkPresentModeKHR,
  /// Specifies whether presentable images may be affected by window clip
  /// regions
  pub clipped: VkBool32,
  /// Existing swap chain to replace, if any
  /// * Optional: true
  pub old_swapchain: VkSwapchainKHR,
}
impl Default for VkSwapchainCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      surface: Default::default(),
      min_image_count: Default::default(),
      image_format: Default::default(),
      image_color_space: Default::default(),
      image_extent: Default::default(),
      image_array_layers: Default::default(),
      image_usage: Default::default(),
      image_sharing_mode: Default::default(),
      queue_family_index_count: Default::default(),
      queue_family_indices: core::ptr::null(),
      pre_transform: Default::default(),
      composite_alpha: Default::default(),
      present_mode: Default::default(),
      clipped: Default::default(),
      old_swapchain: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainDisplayNativeHdrCreateInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html) (structure)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub local_dimming_enable: VkBool32,
}
impl Default for VkSwapchainDisplayNativeHdrCreateInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD,
      next: core::ptr::null(),
      local_dimming_enable: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainImageCreateInfoANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainImageCreateInfoANDROID.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSwapchainImageCreateInfoANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub usage: VkSwapchainImageUsageFlagsANDROID,
}
impl Default for VkSwapchainImageCreateInfoANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID,
      next: core::ptr::null(),
      usage: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainPresentBarrierCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentBarrierCreateInfoNV.html) (structure)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSwapchainPresentBarrierCreateInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub present_barrier_enable: VkBool32,
}
impl Default for VkSwapchainPresentBarrierCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV,
      next: core::ptr::null_mut(),
      present_barrier_enable: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainPresentFenceInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentFenceInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSwapchainPresentFenceInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_FENCE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchain_count: u32,
  /// Fence to signal for each swapchain
  /// * Len: `swapchain_count`
  pub fences: *const VkFence,
}
impl Default for VkSwapchainPresentFenceInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_FENCE_INFO_EXT,
      next: core::ptr::null(),
      swapchain_count: Default::default(),
      fences: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSwapchainPresentModeInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentModeInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSwapchainPresentModeInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchain_count: u32,
  /// Presentation mode for each swapchain
  /// * Len: `swapchain_count`
  pub present_modes: *const VkPresentModeKHR,
}
impl Default for VkSwapchainPresentModeInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODE_INFO_EXT,
      next: core::ptr::null(),
      swapchain_count: Default::default(),
      present_modes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSwapchainPresentModesCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentModesCreateInfoEXT.html) (structure)
///
/// Presentation modes which will be usable with this swapchain
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSwapchainPresentModesCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub present_mode_count: u32,
  /// * Len: `present_mode_count`
  pub present_modes: *const VkPresentModeKHR,
}
impl Default for VkSwapchainPresentModesCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT,
      next: core::ptr::null(),
      present_mode_count: Default::default(),
      present_modes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSwapchainPresentScalingCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentScalingCreateInfoEXT.html) (structure)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSwapchainPresentScalingCreateInfoEXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub scaling_behavior: VkPresentScalingFlagsEXT,
  /// * Optional: true
  pub present_gravity_x: VkPresentGravityFlagsEXT,
  /// * Optional: true
  pub present_gravity_y: VkPresentGravityFlagsEXT,
}
impl Default for VkSwapchainPresentScalingCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT,
      next: core::ptr::null(),
      scaling_behavior: Default::default(),
      present_gravity_x: Default::default(),
      present_gravity_y: Default::default(),
    }
  }
}

/// Khronos: [VkSysmemColorSpaceFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSysmemColorSpaceFUCHSIA.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkSysmemColorSpaceFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub color_space: u32,
}
impl Default for VkSysmemColorSpaceFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA,
      next: core::ptr::null(),
      color_space: Default::default(),
    }
  }
}

/// Khronos: [VkTextureLODGatherFormatPropertiesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html) (structure)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkTextureLODGatherFormatPropertiesAMD {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub supports_texture_gather_lod_bias_amd: VkBool32,
}
impl Default for VkTextureLODGatherFormatPropertiesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
      next: core::ptr::null_mut(),
      supports_texture_gather_lod_bias_amd: Default::default(),
    }
  }
}

/// Khronos: [VkTilePropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTilePropertiesQCOM.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkTilePropertiesQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_TILE_PROPERTIES_QCOM`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub tile_size: VkExtent3D,
  pub apron_size: VkExtent2D,
  pub origin: VkOffset2D,
}
impl Default for VkTilePropertiesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_TILE_PROPERTIES_QCOM,
      next: core::ptr::null_mut(),
      tile_size: Default::default(),
      apron_size: Default::default(),
      origin: Default::default(),
    }
  }
}

/// Khronos: [VkTimelineSemaphoreSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTimelineSemaphoreSubmitInfo.html) (structure)
///
/// * Struct Extends: [`VkSubmitInfo`]
/// * Struct Extends: [`VkBindSparseInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkTimelineSemaphoreSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub wait_semaphore_value_count: u32,
  /// * Optional: true
  /// * Len: `wait_semaphore_value_count`
  pub wait_semaphore_values: *const u64,
  /// * Optional: true
  pub signal_semaphore_value_count: u32,
  /// * Optional: true
  /// * Len: `signal_semaphore_value_count`
  pub signal_semaphore_values: *const u64,
}
impl Default for VkTimelineSemaphoreSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO,
      next: core::ptr::null(),
      wait_semaphore_value_count: Default::default(),
      wait_semaphore_values: core::ptr::null(),
      signal_semaphore_value_count: Default::default(),
      signal_semaphore_values: core::ptr::null(),
    }
  }
}

/// Khronos: [VkTraceRaysIndirectCommand2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommand2KHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkTraceRaysIndirectCommand2KHR {
  pub raygen_shader_record_address: VkDeviceAddress,
  pub raygen_shader_record_size: VkDeviceSize,
  pub miss_shader_binding_table_address: VkDeviceAddress,
  pub miss_shader_binding_table_size: VkDeviceSize,
  pub miss_shader_binding_table_stride: VkDeviceSize,
  pub hit_shader_binding_table_address: VkDeviceAddress,
  pub hit_shader_binding_table_size: VkDeviceSize,
  pub hit_shader_binding_table_stride: VkDeviceSize,
  pub callable_shader_binding_table_address: VkDeviceAddress,
  pub callable_shader_binding_table_size: VkDeviceSize,
  pub callable_shader_binding_table_stride: VkDeviceSize,
  pub width: u32,
  pub height: u32,
  pub depth: u32,
}
impl Default for VkTraceRaysIndirectCommand2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      raygen_shader_record_address: Default::default(),
      raygen_shader_record_size: Default::default(),
      miss_shader_binding_table_address: Default::default(),
      miss_shader_binding_table_size: Default::default(),
      miss_shader_binding_table_stride: Default::default(),
      hit_shader_binding_table_address: Default::default(),
      hit_shader_binding_table_size: Default::default(),
      hit_shader_binding_table_stride: Default::default(),
      callable_shader_binding_table_address: Default::default(),
      callable_shader_binding_table_size: Default::default(),
      callable_shader_binding_table_stride: Default::default(),
      width: Default::default(),
      height: Default::default(),
      depth: Default::default(),
    }
  }
}

/// Khronos: [VkTraceRaysIndirectCommandKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommandKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkTraceRaysIndirectCommandKHR {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
}
impl Default for VkTraceRaysIndirectCommandKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      width: Default::default(),
      height: Default::default(),
      depth: Default::default(),
    }
  }
}

/// Khronos: [VkTransformMatrixKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTransformMatrixKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkTransformMatrixKHR {
  pub matrix: [[c_float; 3]; 4],
}
impl Default for VkTransformMatrixKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { matrix: [[Default::default(); 3]; 4] }
  }
}

/// Khronos: [VkValidationCacheCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateInfoEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkValidationCacheCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkValidationCacheCreateFlagsEXT,
  /// * Optional: true
  pub initial_data_size: c_size_t,
  /// * Len: `initial_data_size`
  pub initial_data: *const c_void,
}
impl Default for VkValidationCacheCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT,
      next: core::ptr::null(),
      flags: Default::default(),
      initial_data_size: Default::default(),
      initial_data: core::ptr::null(),
    }
  }
}

/// Khronos: [VkValidationFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationFeaturesEXT.html) (structure)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkValidationFeaturesEXT {
  /// Must be VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT
  /// * Intended Value: `VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Number of validation features to enable
  /// * Optional: true
  pub enabled_validation_feature_count: u32,
  /// Validation features to enable
  /// * Len: `enabled_validation_feature_count`
  pub enabled_validation_features: *const VkValidationFeatureEnableEXT,
  /// Number of validation features to disable
  /// * Optional: true
  pub disabled_validation_feature_count: u32,
  /// Validation features to disable
  /// * Len: `disabled_validation_feature_count`
  pub disabled_validation_features: *const VkValidationFeatureDisableEXT,
}
impl Default for VkValidationFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT,
      next: core::ptr::null(),
      enabled_validation_feature_count: Default::default(),
      enabled_validation_features: core::ptr::null(),
      disabled_validation_feature_count: Default::default(),
      disabled_validation_features: core::ptr::null(),
    }
  }
}

/// Khronos: [VkValidationFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationFlagsEXT.html) (structure)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkValidationFlagsEXT {
  /// Must be VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT
  /// * Intended Value: `VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Number of validation checks to disable
  pub disabled_validation_check_count: u32,
  /// Validation checks to disable
  /// * Len: `disabled_validation_check_count`
  pub disabled_validation_checks: *const VkValidationCheckEXT,
}
impl Default for VkValidationFlagsEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT,
      next: core::ptr::null(),
      disabled_validation_check_count: Default::default(),
      disabled_validation_checks: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVertexInputAttributeDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVertexInputAttributeDescription {
  /// location of the shader vertex attrib
  pub location: u32,
  /// Vertex buffer binding id
  pub binding: u32,
  /// format of source data
  pub format: VkFormat,
  /// Offset of first element in bytes from base of vertex
  pub offset: u32,
}
impl Default for VkVertexInputAttributeDescription {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      location: Default::default(),
      binding: Default::default(),
      format: Default::default(),
      offset: Default::default(),
    }
  }
}

/// Khronos: [VkVertexInputAttributeDescription2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription2EXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVertexInputAttributeDescription2EXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  /// location of the shader vertex attrib
  pub location: u32,
  /// Vertex buffer binding id
  pub binding: u32,
  /// format of source data
  pub format: VkFormat,
  /// Offset of first element in bytes from base of vertex
  pub offset: u32,
}
impl Default for VkVertexInputAttributeDescription2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT,
      next: core::ptr::null_mut(),
      location: Default::default(),
      binding: Default::default(),
      format: Default::default(),
      offset: Default::default(),
    }
  }
}

/// Khronos: [VkVertexInputBindingDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVertexInputBindingDescription {
  /// Vertex buffer binding id
  pub binding: u32,
  /// Distance between vertices in bytes (0 = no advancement)
  pub stride: u32,
  /// The rate at which the vertex data is consumed
  pub input_rate: VkVertexInputRate,
}
impl Default for VkVertexInputBindingDescription {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      binding: Default::default(),
      stride: Default::default(),
      input_rate: Default::default(),
    }
  }
}

/// Khronos: [VkVertexInputBindingDescription2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription2EXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVertexInputBindingDescription2EXT {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub next: *mut c_void,
  pub binding: u32,
  pub stride: u32,
  pub input_rate: VkVertexInputRate,
  pub divisor: u32,
}
impl Default for VkVertexInputBindingDescription2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT,
      next: core::ptr::null_mut(),
      binding: Default::default(),
      stride: Default::default(),
      input_rate: Default::default(),
      divisor: Default::default(),
    }
  }
}

/// Khronos: [VkVertexInputBindingDivisorDescriptionEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVertexInputBindingDivisorDescriptionEXT {
  pub binding: u32,
  pub divisor: u32,
}
impl Default for VkVertexInputBindingDivisorDescriptionEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { binding: Default::default(), divisor: Default::default() }
  }
}

/// Khronos: [VkViSurfaceCreateInfoNN](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViSurfaceCreateInfoNN.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkViSurfaceCreateInfoNN {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkViSurfaceCreateFlagsNN,
  /// * No Auto-Validity
  pub window: *mut c_void,
}
impl Default for VkViSurfaceCreateInfoNN {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN,
      next: core::ptr::null(),
      flags: Default::default(),
      window: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkVideoBeginCodingInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoBeginCodingInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkVideoBeginCodingFlagsKHR,
  pub video_session: VkVideoSessionKHR,
  /// * Optional: true
  pub video_session_parameters: VkVideoSessionParametersKHR,
  /// * Optional: true
  pub reference_slot_count: u32,
  /// * Len: `reference_slot_count`
  pub reference_slots: *const VkVideoReferenceSlotInfoKHR,
}
impl Default for VkVideoBeginCodingInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      video_session: Default::default(),
      video_session_parameters: Default::default(),
      reference_slot_count: Default::default(),
      reference_slots: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilitiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoCapabilitiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub flags: VkVideoCapabilityFlagsKHR,
  pub min_bitstream_buffer_offset_alignment: VkDeviceSize,
  pub min_bitstream_buffer_size_alignment: VkDeviceSize,
  pub picture_access_granularity: VkExtent2D,
  pub min_coded_extent: VkExtent2D,
  pub max_coded_extent: VkExtent2D,
  pub max_dpb_slots: u32,
  pub max_active_reference_pictures: u32,
  pub std_header_version: VkExtensionProperties,
}
impl Default for VkVideoCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR,
      next: core::ptr::null_mut(),
      flags: Default::default(),
      min_bitstream_buffer_offset_alignment: Default::default(),
      min_bitstream_buffer_size_alignment: Default::default(),
      picture_access_granularity: Default::default(),
      min_coded_extent: Default::default(),
      max_coded_extent: Default::default(),
      max_dpb_slots: Default::default(),
      max_active_reference_pictures: Default::default(),
      std_header_version: Default::default(),
    }
  }
}

/// Khronos: [VkVideoCodingControlInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoCodingControlInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: false
  pub flags: VkVideoCodingControlFlagsKHR,
}
impl Default for VkVideoCodingControlInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkVideoDecodeCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilitiesKHR.html) (structure)
///
/// * Struct Extends: [`VkVideoCapabilitiesKHR`]
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoDecodeCapabilitiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  /// * No Auto-Validity
  pub flags: VkVideoDecodeCapabilityFlagsKHR,
}
impl Default for VkVideoDecodeCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR,
      next: core::ptr::null_mut(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkVideoDecodeInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoDecodeInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkVideoDecodeFlagsKHR,
  pub src_buffer: VkBuffer,
  pub src_buffer_offset: VkDeviceSize,
  pub src_buffer_range: VkDeviceSize,
  pub dst_picture_resource: VkVideoPictureResourceInfoKHR,
  /// * Optional: true
  pub setup_reference_slot: *const VkVideoReferenceSlotInfoKHR,
  /// * Optional: true
  pub reference_slot_count: u32,
  /// * Len: `reference_slot_count`
  pub reference_slots: *const VkVideoReferenceSlotInfoKHR,
}
impl Default for VkVideoDecodeInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      src_buffer: Default::default(),
      src_buffer_offset: Default::default(),
      src_buffer_range: Default::default(),
      dst_picture_resource: Default::default(),
      setup_reference_slot: core::ptr::null(),
      reference_slot_count: Default::default(),
      reference_slots: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoDecodeUsageInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeUsageInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkVideoProfileInfoKHR`]
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoDecodeUsageInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_DECODE_USAGE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub video_usage_hints: VkVideoDecodeUsageFlagsKHR,
}
impl Default for VkVideoDecodeUsageInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_DECODE_USAGE_INFO_KHR,
      next: core::ptr::null(),
      video_usage_hints: Default::default(),
    }
  }
}

/// Khronos: [VkVideoEncodeH264FrameSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264FrameSizeEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoEncodeH264FrameSizeEXT {
  /// * No Auto-Validity
  pub frame_i_size: u32,
  /// * No Auto-Validity
  pub frame_p_size: u32,
  /// * No Auto-Validity
  pub frame_b_size: u32,
}
impl Default for VkVideoEncodeH264FrameSizeEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      frame_i_size: Default::default(),
      frame_p_size: Default::default(),
      frame_b_size: Default::default(),
    }
  }
}

/// Khronos: [VkVideoEncodeH264QpEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264QpEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoEncodeH264QpEXT {
  /// * No Auto-Validity
  pub qp_i: i32,
  /// * No Auto-Validity
  pub qp_p: i32,
  /// * No Auto-Validity
  pub qp_b: i32,
}
impl Default for VkVideoEncodeH264QpEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { qp_i: Default::default(), qp_p: Default::default(), qp_b: Default::default() }
  }
}

/// Khronos: [VkVideoEncodeH265FrameSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265FrameSizeEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoEncodeH265FrameSizeEXT {
  /// * No Auto-Validity
  pub frame_i_size: u32,
  /// * No Auto-Validity
  pub frame_p_size: u32,
  /// * No Auto-Validity
  pub frame_b_size: u32,
}
impl Default for VkVideoEncodeH265FrameSizeEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      frame_i_size: Default::default(),
      frame_p_size: Default::default(),
      frame_b_size: Default::default(),
    }
  }
}

/// Khronos: [VkVideoEncodeH265QpEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265QpEXT.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoEncodeH265QpEXT {
  /// * No Auto-Validity
  pub qp_i: i32,
  /// * No Auto-Validity
  pub qp_p: i32,
  /// * No Auto-Validity
  pub qp_b: i32,
}
impl Default for VkVideoEncodeH265QpEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { qp_i: Default::default(), qp_p: Default::default(), qp_b: Default::default() }
  }
}

/// Khronos: [VkVideoEndCodingInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoEndCodingInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkVideoEndCodingFlagsKHR,
}
impl Default for VkVideoEndCodingInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkVideoFormatPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoFormatPropertiesKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoFormatPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub format: VkFormat,
  pub component_mapping: VkComponentMapping,
  pub image_create_flags: VkImageCreateFlags,
  pub image_type: VkImageType,
  pub image_tiling: VkImageTiling,
  pub image_usage_flags: VkImageUsageFlags,
}
impl Default for VkVideoFormatPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR,
      next: core::ptr::null_mut(),
      format: Default::default(),
      component_mapping: Default::default(),
      image_create_flags: Default::default(),
      image_type: Default::default(),
      image_tiling: Default::default(),
      image_usage_flags: Default::default(),
    }
  }
}

/// Khronos: [VkVideoPictureResourceInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoPictureResourceInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoPictureResourceInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// The offset to be used for the picture resource, currently only used in
  /// field mode
  pub coded_offset: VkOffset2D,
  /// The extent to be used for the picture resource
  pub coded_extent: VkExtent2D,
  /// The first array layer to be accessed for the Decode or Encode Operations
  pub base_array_layer: u32,
  /// The ImageView binding of the resource
  pub image_view_binding: VkImageView,
}
impl Default for VkVideoPictureResourceInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_INFO_KHR,
      next: core::ptr::null(),
      coded_offset: Default::default(),
      coded_extent: Default::default(),
      base_array_layer: Default::default(),
      image_view_binding: Default::default(),
    }
  }
}

/// Khronos: [VkVideoProfileInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoProfileInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoProfileInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_PROFILE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub video_codec_operation: VkVideoCodecOperationFlagBitsKHR,
  pub chroma_subsampling: VkVideoChromaSubsamplingFlagsKHR,
  pub luma_bit_depth: VkVideoComponentBitDepthFlagsKHR,
  /// * Optional: true
  pub chroma_bit_depth: VkVideoComponentBitDepthFlagsKHR,
}
impl Default for VkVideoProfileInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_PROFILE_INFO_KHR,
      next: core::ptr::null(),
      video_codec_operation: Default::default(),
      chroma_subsampling: Default::default(),
      luma_bit_depth: Default::default(),
      chroma_bit_depth: Default::default(),
    }
  }
}

/// Khronos: [VkVideoProfileListInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoProfileListInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
/// * Struct Extends: [`VkPhysicalDeviceVideoFormatInfoKHR`]
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoProfileListInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_PROFILE_LIST_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub profile_count: u32,
  /// * Len: `profile_count`
  pub profiles: *const VkVideoProfileInfoKHR,
}
impl Default for VkVideoProfileListInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_PROFILE_LIST_INFO_KHR,
      next: core::ptr::null(),
      profile_count: Default::default(),
      profiles: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoReferenceSlotInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoReferenceSlotInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoReferenceSlotInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// The reference slot index
  pub slot_index: i32,
  /// The reference picture resource
  /// * Optional: true
  pub picture_resource: *const VkVideoPictureResourceInfoKHR,
}
impl Default for VkVideoReferenceSlotInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_INFO_KHR,
      next: core::ptr::null(),
      slot_index: Default::default(),
      picture_resource: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoSessionCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoSessionCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub queue_family_index: u32,
  /// * Optional: true
  pub flags: VkVideoSessionCreateFlagsKHR,
  pub video_profile: *const VkVideoProfileInfoKHR,
  pub picture_format: VkFormat,
  pub max_coded_extent: VkExtent2D,
  pub reference_picture_format: VkFormat,
  pub max_dpb_slots: u32,
  pub max_active_reference_pictures: u32,
  pub std_header_version: *const VkExtensionProperties,
}
impl Default for VkVideoSessionCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR,
      next: core::ptr::null(),
      queue_family_index: Default::default(),
      flags: Default::default(),
      video_profile: core::ptr::null(),
      picture_format: Default::default(),
      max_coded_extent: Default::default(),
      reference_picture_format: Default::default(),
      max_dpb_slots: Default::default(),
      max_active_reference_pictures: Default::default(),
      std_header_version: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoSessionMemoryRequirementsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionMemoryRequirementsKHR.html) (structure)
///
/// * Returned Only
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoSessionMemoryRequirementsKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *mut c_void,
  pub memory_bind_index: u32,
  pub memory_requirements: VkMemoryRequirements,
}
impl Default for VkVideoSessionMemoryRequirementsKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR,
      next: core::ptr::null_mut(),
      memory_bind_index: Default::default(),
      memory_requirements: Default::default(),
    }
  }
}

/// Khronos: [VkVideoSessionParametersCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoSessionParametersCreateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkVideoSessionParametersCreateFlagsKHR,
  /// * Optional: true
  pub video_session_parameters_template: VkVideoSessionParametersKHR,
  pub video_session: VkVideoSessionKHR,
}
impl Default for VkVideoSessionParametersCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      video_session_parameters_template: Default::default(),
      video_session: Default::default(),
    }
  }
}

/// Khronos: [VkVideoSessionParametersUpdateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersUpdateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkVideoSessionParametersUpdateInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub update_sequence_count: u32,
}
impl Default for VkVideoSessionParametersUpdateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR,
      next: core::ptr::null(),
      update_sequence_count: Default::default(),
    }
  }
}

/// Khronos: [VkViewport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewport.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkViewport {
  /// * No Auto-Validity
  pub x: c_float,
  /// * No Auto-Validity
  pub y: c_float,
  /// * No Auto-Validity
  pub width: c_float,
  /// * No Auto-Validity
  pub height: c_float,
  pub min_depth: c_float,
  pub max_depth: c_float,
}
impl Default for VkViewport {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      x: Default::default(),
      y: Default::default(),
      width: Default::default(),
      height: Default::default(),
      min_depth: Default::default(),
      max_depth: Default::default(),
    }
  }
}

/// Khronos: [VkViewportSwizzleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewportSwizzleNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkViewportSwizzleNV {
  pub x: VkViewportCoordinateSwizzleNV,
  pub y: VkViewportCoordinateSwizzleNV,
  pub z: VkViewportCoordinateSwizzleNV,
  pub w: VkViewportCoordinateSwizzleNV,
}
impl Default for VkViewportSwizzleNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      x: Default::default(),
      y: Default::default(),
      z: Default::default(),
      w: Default::default(),
    }
  }
}

/// Khronos: [VkViewportWScalingNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewportWScalingNV.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkViewportWScalingNV {
  pub xcoeff: c_float,
  pub ycoeff: c_float,
}
impl Default for VkViewportWScalingNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { xcoeff: Default::default(), ycoeff: Default::default() }
  }
}

/// Khronos: [VkWaylandSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkWaylandSurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkWaylandSurfaceCreateFlagsKHR,
  /// * No Auto-Validity
  pub display: *mut wl_display,
  /// * No Auto-Validity
  pub surface: *mut wl_surface,
}
impl Default for VkWaylandSurfaceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      display: core::ptr::null_mut(),
      surface: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkWin32KeyedMutexAcquireReleaseInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoKHR.html) (structure)
///
/// * Struct Extends: [`VkSubmitInfo`]
/// * Struct Extends: [`VkSubmitInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub acquire_count: u32,
  /// * Len: `acquire_count`
  pub acquire_syncs: *const VkDeviceMemory,
  /// * Len: `acquire_count`
  pub acquire_keys: *const u64,
  /// * Len: `acquire_count`
  pub acquire_timeouts: *const u32,
  /// * Optional: true
  pub release_count: u32,
  /// * Len: `release_count`
  pub release_syncs: *const VkDeviceMemory,
  /// * Len: `release_count`
  pub release_keys: *const u64,
}
impl Default for VkWin32KeyedMutexAcquireReleaseInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR,
      next: core::ptr::null(),
      acquire_count: Default::default(),
      acquire_syncs: core::ptr::null(),
      acquire_keys: core::ptr::null(),
      acquire_timeouts: core::ptr::null(),
      release_count: Default::default(),
      release_syncs: core::ptr::null(),
      release_keys: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWin32KeyedMutexAcquireReleaseInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoNV.html) (structure)
///
/// * Struct Extends: [`VkSubmitInfo`]
/// * Struct Extends: [`VkSubmitInfo2`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub acquire_count: u32,
  /// * Len: `acquire_count`
  pub acquire_syncs: *const VkDeviceMemory,
  /// * Len: `acquire_count`
  pub acquire_keys: *const u64,
  /// * Len: `acquire_count`
  pub acquire_timeout_milliseconds: *const u32,
  /// * Optional: true
  pub release_count: u32,
  /// * Len: `release_count`
  pub release_syncs: *const VkDeviceMemory,
  /// * Len: `release_count`
  pub release_keys: *const u64,
}
impl Default for VkWin32KeyedMutexAcquireReleaseInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV,
      next: core::ptr::null(),
      acquire_count: Default::default(),
      acquire_syncs: core::ptr::null(),
      acquire_keys: core::ptr::null(),
      acquire_timeout_milliseconds: core::ptr::null(),
      release_count: Default::default(),
      release_syncs: core::ptr::null(),
      release_keys: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWin32SurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkWin32SurfaceCreateFlagsKHR,
  pub hinstance: HINSTANCE,
  pub hwnd: HWND,
}
impl Default for VkWin32SurfaceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      hinstance: core::ptr::null_mut(),
      hwnd: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkWriteDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSet.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkWriteDescriptorSet {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// Destination descriptor set
  /// * No Auto-Validity
  pub dst_set: VkDescriptorSet,
  /// Binding within the destination descriptor set to write
  pub dst_binding: u32,
  /// Array element within the destination binding to write
  pub dst_array_element: u32,
  /// Number of descriptors to write (determines the size of the array pointed
  /// by pDescriptors)
  pub descriptor_count: u32,
  /// Descriptor type to write (determines which members of the array pointed by
  /// pDescriptors are going to be used)
  pub descriptor_type: VkDescriptorType,
  /// Sampler, image view, and layout for SAMPLER, COMBINED_IMAGE_SAMPLER,
  /// {SAMPLED,STORAGE}_IMAGE, and INPUT_ATTACHMENT descriptor types.
  /// * Len: `descriptor_count`
  /// * No Auto-Validity
  pub image_info: *const VkDescriptorImageInfo,
  /// Raw buffer, size, and offset for {UNIFORM,STORAGE}_BUFFER\[_DYNAMIC\]
  /// descriptor types.
  /// * Len: `descriptor_count`
  /// * No Auto-Validity
  pub buffer_info: *const VkDescriptorBufferInfo,
  /// Buffer view to write to the descriptor for {UNIFORM,STORAGE}_TEXEL_BUFFER
  /// descriptor types.
  /// * Len: `descriptor_count`
  /// * No Auto-Validity
  pub texel_buffer_view: *const VkBufferView,
}
impl Default for VkWriteDescriptorSet {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
      next: core::ptr::null(),
      dst_set: Default::default(),
      dst_binding: Default::default(),
      dst_array_element: Default::default(),
      descriptor_count: Default::default(),
      descriptor_type: Default::default(),
      image_info: core::ptr::null(),
      buffer_info: core::ptr::null(),
      texel_buffer_view: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWriteDescriptorSetAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html) (structure)
///
/// * Struct Extends: [`VkWriteDescriptorSet`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub acceleration_structure_count: u32,
  /// * Optional: false,true
  /// * Len: `acceleration_structure_count`
  pub acceleration_structures: *const VkAccelerationStructureKHR,
}
impl Default for VkWriteDescriptorSetAccelerationStructureKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR,
      next: core::ptr::null(),
      acceleration_structure_count: Default::default(),
      acceleration_structures: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWriteDescriptorSetAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureNV.html) (structure)
///
/// * Struct Extends: [`VkWriteDescriptorSet`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureNV {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub acceleration_structure_count: u32,
  /// * Optional: false,true
  /// * Len: `acceleration_structure_count`
  pub acceleration_structures: *const VkAccelerationStructureNV,
}
impl Default for VkWriteDescriptorSetAccelerationStructureNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV,
      next: core::ptr::null(),
      acceleration_structure_count: Default::default(),
      acceleration_structures: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWriteDescriptorSetInlineUniformBlock](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetInlineUniformBlock.html) (structure)
///
/// * Struct Extends: [`VkWriteDescriptorSet`]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkWriteDescriptorSetInlineUniformBlock {
  /// * Intended Value:
  ///   `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  pub data_size: u32,
  /// * Len: `data_size`
  pub data: *const c_void,
}
impl Default for VkWriteDescriptorSetInlineUniformBlock {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK,
      next: core::ptr::null(),
      data_size: Default::default(),
      data: core::ptr::null(),
    }
  }
}

/// Khronos: [VkXYColorEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXYColorEXT.html) (structure)
///
/// Chromaticity coordinate
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkXYColorEXT {
  pub x: c_float,
  pub y: c_float,
}
impl Default for VkXYColorEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self { x: Default::default(), y: Default::default() }
  }
}

/// Khronos: [VkXcbSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkXcbSurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkXcbSurfaceCreateFlagsKHR,
  /// * No Auto-Validity
  pub connection: *mut xcb_connection_t,
  pub window: xcb_window_t,
}
impl Default for VkXcbSurfaceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      connection: core::ptr::null_mut(),
      window: Default::default(),
    }
  }
}

/// Khronos: [VkXlibSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html) (structure)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VkXlibSurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR`
  pub struct_ty: VkStructureType,
  /// * Optional: true
  pub next: *const c_void,
  /// * Optional: true
  pub flags: VkXlibSurfaceCreateFlagsKHR,
  /// * No Auto-Validity
  pub dpy: *mut XlibDisplay,
  pub window: XlibWindow,
}
impl Default for VkXlibSurfaceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      struct_ty: VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
      next: core::ptr::null(),
      flags: Default::default(),
      dpy: core::ptr::null_mut(),
      window: Default::default(),
    }
  }
}
