#![allow(non_upper_case_globals)]

use crate::prelude::*;

/// Khronos: [VkAabbPositionsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAabbPositionsKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAabbPositionsKHR {
  pub minX: c_float,
  pub minY: c_float,
  pub minZ: c_float,
  pub maxX: c_float,
  pub maxY: c_float,
  pub maxZ: c_float,
}
impl Default for VkAabbPositionsKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      minX: Default::default(),
      minY: Default::default(),
      minZ: Default::default(),
      maxX: Default::default(),
      maxY: Default::default(),
      maxZ: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureBuildGeometryInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub ty: VkAccelerationStructureTypeKHR,
  /// * Optional: true
  pub flags: VkBuildAccelerationStructureFlagsKHR,
  /// * No Auto-Validity
  pub mode: VkBuildAccelerationStructureModeKHR,
  /// * Optional: true
  /// * No Auto-Validity
  pub srcAccelerationStructure: VkAccelerationStructureKHR,
  /// * Optional: true
  /// * No Auto-Validity
  pub dstAccelerationStructure: VkAccelerationStructureKHR,
  /// * Optional: true
  pub geometryCount: u32,
  /// * Optional: true
  /// * Len: `geometryCount`
  pub pGeometries: *const VkAccelerationStructureGeometryKHR,
  /// * Optional: true,false
  /// * Len: `geometryCount,1`
  pub ppGeometries: *const *const VkAccelerationStructureGeometryKHR,
  /// * No Auto-Validity
  pub scratchData: VkDeviceOrHostAddressKHR,
}
impl Default for VkAccelerationStructureBuildGeometryInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
      pNext: core::ptr::null(),
      ty: Default::default(),
      flags: Default::default(),
      mode: Default::default(),
      srcAccelerationStructure: Default::default(),
      dstAccelerationStructure: Default::default(),
      geometryCount: Default::default(),
      pGeometries: core::ptr::null(),
      ppGeometries: core::ptr::null(),
      scratchData: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureBuildRangeInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
  pub primitiveCount: u32,
  pub primitiveOffset: u32,
  pub firstVertex: u32,
  pub transformOffset: u32,
}
impl Default for VkAccelerationStructureBuildRangeInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      primitiveCount: Default::default(),
      primitiveOffset: Default::default(),
      firstVertex: Default::default(),
      transformOffset: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureBuildSizesInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureBuildSizesInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub accelerationStructureSize: VkDeviceSize,
  pub updateScratchSize: VkDeviceSize,
  pub buildScratchSize: VkDeviceSize,
}
impl Default for VkAccelerationStructureBuildSizesInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR,
      pNext: core::ptr::null(),
      accelerationStructureSize: Default::default(),
      updateScratchSize: Default::default(),
      buildScratchSize: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCaptureDescriptorDataInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureCaptureDescriptorDataInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub accelerationStructure: VkAccelerationStructureKHR,
  /// * Optional: true
  pub accelerationStructureNV: VkAccelerationStructureNV,
}
impl Default for VkAccelerationStructureCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      pNext: core::ptr::null(),
      accelerationStructure: Default::default(),
      accelerationStructureNV: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub createFlags: VkAccelerationStructureCreateFlagsKHR,
  pub buffer: VkBuffer,
  /// Specified in bytes
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  pub ty: VkAccelerationStructureTypeKHR,
  /// * Optional: true
  pub deviceAddress: VkDeviceAddress,
}
impl Default for VkAccelerationStructureCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      createFlags: Default::default(),
      buffer: Default::default(),
      offset: Default::default(),
      size: Default::default(),
      ty: Default::default(),
      deviceAddress: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub compactedSize: VkDeviceSize,
  pub info: VkAccelerationStructureInfoNV,
}
impl Default for VkAccelerationStructureCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      compactedSize: Default::default(),
      info: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureDeviceAddressInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub accelerationStructure: VkAccelerationStructureKHR,
}
impl Default for VkAccelerationStructureDeviceAddressInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
      pNext: core::ptr::null(),
      accelerationStructure: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureGeometryAabbsDataKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryAabbsDataKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * No Auto-Validity
  pub data: VkDeviceOrHostAddressConstKHR,
  pub stride: VkDeviceSize,
}
impl Default for VkAccelerationStructureGeometryAabbsDataKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR,
      pNext: core::ptr::null(),
      data: Default::default(),
      stride: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureGeometryInstancesDataKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryInstancesDataKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub arrayOfPointers: VkBool32,
  /// * No Auto-Validity
  pub data: VkDeviceOrHostAddressConstKHR,
}
impl Default for VkAccelerationStructureGeometryInstancesDataKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR,
      pNext: core::ptr::null(),
      arrayOfPointers: Default::default(),
      data: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureGeometryKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub geometryType: VkGeometryTypeKHR,
  /// * Selector: geometryType
  pub geometry: VkAccelerationStructureGeometryDataKHR,
  /// * Optional: true
  pub flags: VkGeometryFlagsKHR,
}
impl Default for VkAccelerationStructureGeometryKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR,
      pNext: core::ptr::null(),
      geometryType: Default::default(),
      geometry: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureGeometryMotionTrianglesDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryMotionTrianglesDataNV.html)
///
/// * Struct Extends: [`VkAccelerationStructureGeometryTrianglesDataKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * No Auto-Validity
  pub vertexData: VkDeviceOrHostAddressConstKHR,
}
impl Default for VkAccelerationStructureGeometryMotionTrianglesDataNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV,
      pNext: core::ptr::null(),
      vertexData: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureGeometryTrianglesDataKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureGeometryTrianglesDataKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub vertexFormat: VkFormat,
  /// * No Auto-Validity
  pub vertexData: VkDeviceOrHostAddressConstKHR,
  pub vertexStride: VkDeviceSize,
  pub maxVertex: u32,
  pub indexType: VkIndexType,
  /// * No Auto-Validity
  pub indexData: VkDeviceOrHostAddressConstKHR,
  /// * No Auto-Validity
  pub transformData: VkDeviceOrHostAddressConstKHR,
}
impl Default for VkAccelerationStructureGeometryTrianglesDataKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR,
      pNext: core::ptr::null(),
      vertexFormat: Default::default(),
      vertexData: Default::default(),
      vertexStride: Default::default(),
      maxVertex: Default::default(),
      indexType: Default::default(),
      indexData: Default::default(),
      transformData: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub ty: VkAccelerationStructureTypeNV,
  /// * Optional: true
  pub flags: VkBuildAccelerationStructureFlagsNV,
  /// * Optional: true
  pub instanceCount: u32,
  /// * Optional: true
  pub geometryCount: u32,
  /// * Len: `geometryCount`
  pub pGeometries: *const VkGeometryNV,
}
impl Default for VkAccelerationStructureInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV,
      pNext: core::ptr::null(),
      ty: Default::default(),
      flags: Default::default(),
      instanceCount: Default::default(),
      geometryCount: Default::default(),
      pGeometries: core::ptr::null(),
    }
  }
}

/// Khronos: [VkAccelerationStructureMemoryRequirementsInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub ty: VkAccelerationStructureMemoryRequirementsTypeNV,
  pub accelerationStructure: VkAccelerationStructureNV,
}
impl Default for VkAccelerationStructureMemoryRequirementsInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
      pNext: core::ptr::null(),
      ty: Default::default(),
      accelerationStructure: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureMotionInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInfoNV.html)
///
/// * Struct Extends: [`VkAccelerationStructureCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureMotionInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub maxInstances: u32,
  /// * Optional: true
  pub flags: VkAccelerationStructureMotionInfoFlagsNV,
}
impl Default for VkAccelerationStructureMotionInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV,
      pNext: core::ptr::null(),
      maxInstances: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureTrianglesOpacityMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTrianglesOpacityMicromapEXT.html)
///
/// * Struct Extends: [`VkAccelerationStructureGeometryTrianglesDataKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureTrianglesOpacityMicromapEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub indexType: VkIndexType,
  /// * No Auto-Validity
  pub indexBuffer: VkDeviceOrHostAddressConstKHR,
  pub indexStride: VkDeviceSize,
  pub baseTriangle: u32,
  /// * Optional: true
  pub usageCountsCount: u32,
  /// * Optional: true
  /// * Len: `usageCountsCount`
  pub pUsageCounts: *const VkMicromapUsageEXT,
  /// * Optional: true,false
  /// * Len: `usageCountsCount,1`
  pub ppUsageCounts: *const *const VkMicromapUsageEXT,
  pub micromap: VkMicromapEXT,
}
impl Default for VkAccelerationStructureTrianglesOpacityMicromapEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT,
      pNext: core::ptr::null_mut(),
      indexType: Default::default(),
      indexBuffer: Default::default(),
      indexStride: Default::default(),
      baseTriangle: Default::default(),
      usageCountsCount: Default::default(),
      pUsageCounts: core::ptr::null(),
      ppUsageCounts: core::ptr::null(),
      micromap: Default::default(),
    }
  }
}

/// Khronos: [VkAccelerationStructureVersionInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureVersionInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Len: `2*VK_UUID_SIZE`
  pub pVersionData: *const u8,
}
impl Default for VkAccelerationStructureVersionInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR,
      pNext: core::ptr::null(),
      pVersionData: core::ptr::null(),
    }
  }
}

/// Khronos: [VkAcquireNextImageInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireNextImageInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAcquireNextImageInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub swapchain: VkSwapchainKHR,
  pub timeout: u64,
  /// * Optional: true
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  /// * Extern Sync: true
  pub fence: VkFence,
  pub deviceMask: u32,
}
impl Default for VkAcquireNextImageInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR,
      pNext: core::ptr::null(),
      swapchain: Default::default(),
      timeout: Default::default(),
      semaphore: Default::default(),
      fence: Default::default(),
      deviceMask: Default::default(),
    }
  }
}

/// Khronos: [VkAcquireProfilingLockInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAcquireProfilingLockInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      timeout: Default::default(),
    }
  }
}

/// Khronos: [VkAllocationCallbacks](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAllocationCallbacks.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAllocationCallbacks {
  /// * Optional: true
  pub pUserData: *mut c_void,
  /// * No Auto-Validity
  pub pfnAllocation: PFN_vkAllocationFunction,
  /// * No Auto-Validity
  pub pfnReallocation: PFN_vkReallocationFunction,
  /// * No Auto-Validity
  pub pfnFree: PFN_vkFreeFunction,
  /// * Optional: true
  /// * No Auto-Validity
  pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
  /// * Optional: true
  /// * No Auto-Validity
  pub pfnInternalFree: PFN_vkInternalFreeNotification,
}
impl Default for VkAllocationCallbacks {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      pUserData: core::ptr::null_mut(),
      pfnAllocation: Default::default(),
      pfnReallocation: Default::default(),
      pfnFree: Default::default(),
      pfnInternalAllocation: Default::default(),
      pfnInternalFree: Default::default(),
    }
  }
}

/// Khronos: [VkAmigoProfilingSubmitInfoSEC](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAmigoProfilingSubmitInfoSEC.html)
///
/// * Struct Extends: [`VkSubmitInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAmigoProfilingSubmitInfoSEC {
  /// * Intended Value: `VK_STRUCTURE_TYPE_AMIGO_PROFILING_SUBMIT_INFO_SEC`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub firstDrawTimestamp: u64,
  pub swapBufferTimestamp: u64,
}
impl Default for VkAmigoProfilingSubmitInfoSEC {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_AMIGO_PROFILING_SUBMIT_INFO_SEC,
      pNext: core::ptr::null(),
      firstDrawTimestamp: Default::default(),
      swapBufferTimestamp: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidHardwareBufferFormatProperties2ANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatProperties2ANDROID.html)
///
/// * Struct Extends: [`VkAndroidHardwareBufferPropertiesANDROID`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidHardwareBufferFormatProperties2ANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub format: VkFormat,
  pub externalFormat: u64,
  pub formatFeatures: VkFormatFeatureFlags2,
  pub samplerYcbcrConversionComponents: VkComponentMapping,
  pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
  pub suggestedYcbcrRange: VkSamplerYcbcrRange,
  pub suggestedXChromaOffset: VkChromaLocation,
  pub suggestedYChromaOffset: VkChromaLocation,
}
impl Default for VkAndroidHardwareBufferFormatProperties2ANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID,
      pNext: core::ptr::null_mut(),
      format: Default::default(),
      externalFormat: Default::default(),
      formatFeatures: Default::default(),
      samplerYcbcrConversionComponents: Default::default(),
      suggestedYcbcrModel: Default::default(),
      suggestedYcbcrRange: Default::default(),
      suggestedXChromaOffset: Default::default(),
      suggestedYChromaOffset: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidHardwareBufferFormatPropertiesANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html)
///
/// * Struct Extends: [`VkAndroidHardwareBufferPropertiesANDROID`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidHardwareBufferFormatPropertiesANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub format: VkFormat,
  pub externalFormat: u64,
  pub formatFeatures: VkFormatFeatureFlags,
  pub samplerYcbcrConversionComponents: VkComponentMapping,
  pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
  pub suggestedYcbcrRange: VkSamplerYcbcrRange,
  pub suggestedXChromaOffset: VkChromaLocation,
  pub suggestedYChromaOffset: VkChromaLocation,
}
impl Default for VkAndroidHardwareBufferFormatPropertiesANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID,
      pNext: core::ptr::null_mut(),
      format: Default::default(),
      externalFormat: Default::default(),
      formatFeatures: Default::default(),
      samplerYcbcrConversionComponents: Default::default(),
      suggestedYcbcrModel: Default::default(),
      suggestedYcbcrRange: Default::default(),
      suggestedXChromaOffset: Default::default(),
      suggestedYChromaOffset: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidHardwareBufferPropertiesANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferPropertiesANDROID.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidHardwareBufferPropertiesANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub allocationSize: VkDeviceSize,
  pub memoryTypeBits: u32,
}
impl Default for VkAndroidHardwareBufferPropertiesANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
      pNext: core::ptr::null_mut(),
      allocationSize: Default::default(),
      memoryTypeBits: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidHardwareBufferUsageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferUsageANDROID.html)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidHardwareBufferUsageANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub androidHardwareBufferUsage: u64,
}
impl Default for VkAndroidHardwareBufferUsageANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
      pNext: core::ptr::null_mut(),
      androidHardwareBufferUsage: Default::default(),
    }
  }
}

/// Khronos: [VkAndroidSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidSurfaceCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAndroidSurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      window: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkApplicationInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkApplicationInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkApplicationInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_APPLICATION_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  /// * Len: `null-terminated`
  pub pApplicationName: *const u8,
  pub applicationVersion: u32,
  /// * Optional: true
  /// * Len: `null-terminated`
  pub pEngineName: *const u8,
  pub engineVersion: u32,
  pub apiVersion: u32,
}
impl Default for VkApplicationInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
      pNext: core::ptr::null(),
      pApplicationName: core::ptr::null(),
      applicationVersion: Default::default(),
      pEngineName: core::ptr::null(),
      engineVersion: Default::default(),
      apiVersion: Default::default(),
    }
  }
}

/// Khronos: [VkApplicationParametersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkApplicationParametersEXT.html)
///
/// * Struct Extends: [`VkApplicationInfo`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkApplicationParametersEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_APPLICATION_PARAMETERS_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub vendorID: u32,
  /// * Optional: true
  pub deviceID: u32,
  pub key: u32,
  pub value: u64,
}
impl Default for VkApplicationParametersEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_APPLICATION_PARAMETERS_EXT,
      pNext: core::ptr::null(),
      vendorID: Default::default(),
      deviceID: Default::default(),
      key: Default::default(),
      value: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentDescription {
  /// * Optional: true
  pub flags: VkAttachmentDescriptionFlags,
  pub format: VkFormat,
  pub samples: VkSampleCountFlagBits,
  /// Load operation for color or depth data
  pub loadOp: VkAttachmentLoadOp,
  /// Store operation for color or depth data
  pub storeOp: VkAttachmentStoreOp,
  /// Load operation for stencil data
  pub stencilLoadOp: VkAttachmentLoadOp,
  /// Store operation for stencil data
  pub stencilStoreOp: VkAttachmentStoreOp,
  pub initialLayout: VkImageLayout,
  pub finalLayout: VkImageLayout,
}
impl Default for VkAttachmentDescription {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      flags: Default::default(),
      format: Default::default(),
      samples: Default::default(),
      loadOp: Default::default(),
      storeOp: Default::default(),
      stencilLoadOp: Default::default(),
      stencilStoreOp: Default::default(),
      initialLayout: Default::default(),
      finalLayout: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentDescription2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentDescription2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkAttachmentDescriptionFlags,
  pub format: VkFormat,
  pub samples: VkSampleCountFlagBits,
  /// Load operation for color or depth data
  pub loadOp: VkAttachmentLoadOp,
  /// Store operation for color or depth data
  pub storeOp: VkAttachmentStoreOp,
  /// Load operation for stencil data
  pub stencilLoadOp: VkAttachmentLoadOp,
  /// Store operation for stencil data
  pub stencilStoreOp: VkAttachmentStoreOp,
  pub initialLayout: VkImageLayout,
  pub finalLayout: VkImageLayout,
}
impl Default for VkAttachmentDescription2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2,
      pNext: core::ptr::null(),
      flags: Default::default(),
      format: Default::default(),
      samples: Default::default(),
      loadOp: Default::default(),
      storeOp: Default::default(),
      stencilLoadOp: Default::default(),
      stencilStoreOp: Default::default(),
      initialLayout: Default::default(),
      finalLayout: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentDescriptionStencilLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionStencilLayout.html)
///
/// * Struct Extends: [`VkAttachmentDescription2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentDescriptionStencilLayout {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub stencilInitialLayout: VkImageLayout,
  pub stencilFinalLayout: VkImageLayout,
}
impl Default for VkAttachmentDescriptionStencilLayout {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT,
      pNext: core::ptr::null_mut(),
      stencilInitialLayout: Default::default(),
      stencilFinalLayout: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentReference {
  pub attachment: u32,
  pub layout: VkImageLayout,
}
impl Default for VkAttachmentReference {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      attachment: Default::default(),
      layout: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentReference2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentReference2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub attachment: u32,
  pub layout: VkImageLayout,
  /// * No Auto-Validity
  pub aspectMask: VkImageAspectFlags,
}
impl Default for VkAttachmentReference2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2,
      pNext: core::ptr::null(),
      attachment: Default::default(),
      layout: Default::default(),
      aspectMask: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentReferenceStencilLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentReferenceStencilLayout.html)
///
/// * Struct Extends: [`VkAttachmentReference2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentReferenceStencilLayout {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub stencilLayout: VkImageLayout,
}
impl Default for VkAttachmentReferenceStencilLayout {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT,
      pNext: core::ptr::null_mut(),
      stencilLayout: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentSampleCountInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleCountInfoAMD.html)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentSampleCountInfoAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub colorAttachmentCount: u32,
  /// * Len: `colorAttachmentCount`
  /// * No Auto-Validity
  pub pColorAttachmentSamples: *const VkSampleCountFlagBits,
  /// * Optional: true
  /// * No Auto-Validity
  pub depthStencilAttachmentSamples: VkSampleCountFlagBits,
}
impl Default for VkAttachmentSampleCountInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD,
      pNext: core::ptr::null(),
      colorAttachmentCount: Default::default(),
      pColorAttachmentSamples: core::ptr::null(),
      depthStencilAttachmentSamples: Default::default(),
    }
  }
}

/// Khronos: [VkAttachmentSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleLocationsEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentSampleLocationsEXT {
  pub attachmentIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
impl Default for VkAttachmentSampleLocationsEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      attachmentIndex: Default::default(),
      sampleLocationsInfo: Default::default(),
    }
  }
}

/// Khronos: [VkBaseInStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseInStructure.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseInStructure {
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const VkBaseInStructure,
}
impl Default for VkBaseInStructure {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: Default::default(),
      pNext: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBaseOutStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseOutStructure.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseOutStructure {
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut VkBaseOutStructure,
}
impl Default for VkBaseOutStructure {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: Default::default(),
      pNext: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkBindAccelerationStructureMemoryInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindAccelerationStructureMemoryInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub accelerationStructure: VkAccelerationStructureNV,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  /// * Optional: true
  pub deviceIndexCount: u32,
  /// * Len: `deviceIndexCount`
  pub pDeviceIndices: *const u32,
}
impl Default for VkBindAccelerationStructureMemoryInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV,
      pNext: core::ptr::null(),
      accelerationStructure: Default::default(),
      memory: Default::default(),
      memoryOffset: Default::default(),
      deviceIndexCount: Default::default(),
      pDeviceIndices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBindBufferMemoryDeviceGroupInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html)
///
/// * Struct Extends: [`VkBindBufferMemoryInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindBufferMemoryDeviceGroupInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub deviceIndexCount: u32,
  /// * Len: `deviceIndexCount`
  pub pDeviceIndices: *const u32,
}
impl Default for VkBindBufferMemoryDeviceGroupInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO,
      pNext: core::ptr::null(),
      deviceIndexCount: Default::default(),
      pDeviceIndices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBindBufferMemoryInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindBufferMemoryInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub buffer: VkBuffer,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
}
impl Default for VkBindBufferMemoryInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO,
      pNext: core::ptr::null(),
      buffer: Default::default(),
      memory: Default::default(),
      memoryOffset: Default::default(),
    }
  }
}

/// Khronos: [VkBindImageMemoryDeviceGroupInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html)
///
/// * Struct Extends: [`VkBindImageMemoryInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindImageMemoryDeviceGroupInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub deviceIndexCount: u32,
  /// * Len: `deviceIndexCount`
  pub pDeviceIndices: *const u32,
  /// * Optional: true
  pub splitInstanceBindRegionCount: u32,
  /// * Len: `splitInstanceBindRegionCount`
  pub pSplitInstanceBindRegions: *const VkRect2D,
}
impl Default for VkBindImageMemoryDeviceGroupInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO,
      pNext: core::ptr::null(),
      deviceIndexCount: Default::default(),
      pDeviceIndices: core::ptr::null(),
      splitInstanceBindRegionCount: Default::default(),
      pSplitInstanceBindRegions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBindImageMemoryInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindImageMemoryInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub image: VkImage,
  /// * No Auto-Validity
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
}
impl Default for VkBindImageMemoryInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO,
      pNext: core::ptr::null(),
      image: Default::default(),
      memory: Default::default(),
      memoryOffset: Default::default(),
    }
  }
}

/// Khronos: [VkBindImageMemorySwapchainInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html)
///
/// * Struct Extends: [`VkBindImageMemoryInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindImageMemorySwapchainInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub swapchain: VkSwapchainKHR,
  pub imageIndex: u32,
}
impl Default for VkBindImageMemorySwapchainInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
      pNext: core::ptr::null(),
      swapchain: Default::default(),
      imageIndex: Default::default(),
    }
  }
}

/// Khronos: [VkBindImagePlaneMemoryInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImagePlaneMemoryInfo.html)
///
/// * Struct Extends: [`VkBindImageMemoryInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindImagePlaneMemoryInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub planeAspect: VkImageAspectFlagBits,
}
impl Default for VkBindImagePlaneMemoryInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO,
      pNext: core::ptr::null(),
      planeAspect: Default::default(),
    }
  }
}

/// Khronos: [VkBindIndexBufferIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindIndexBufferIndirectCommandNV {
  pub bufferAddress: VkDeviceAddress,
  pub size: u32,
  pub indexType: VkIndexType,
}
impl Default for VkBindIndexBufferIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      bufferAddress: Default::default(),
      size: Default::default(),
      indexType: Default::default(),
    }
  }
}

/// Khronos: [VkBindShaderGroupIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindShaderGroupIndirectCommandNV {
  pub groupIndex: u32,
}
impl Default for VkBindShaderGroupIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      groupIndex: Default::default(),
    }
  }
}

/// Khronos: [VkBindSparseInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindSparseInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindSparseInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_SPARSE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub waitSemaphoreCount: u32,
  /// * Len: `waitSemaphoreCount`
  pub pWaitSemaphores: *const VkSemaphore,
  /// * Optional: true
  pub bufferBindCount: u32,
  /// * Len: `bufferBindCount`
  pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
  /// * Optional: true
  pub imageOpaqueBindCount: u32,
  /// * Len: `imageOpaqueBindCount`
  pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
  /// * Optional: true
  pub imageBindCount: u32,
  /// * Len: `imageBindCount`
  pub pImageBinds: *const VkSparseImageMemoryBindInfo,
  /// * Optional: true
  pub signalSemaphoreCount: u32,
  /// * Len: `signalSemaphoreCount`
  pub pSignalSemaphores: *const VkSemaphore,
}
impl Default for VkBindSparseInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BIND_SPARSE_INFO,
      pNext: core::ptr::null(),
      waitSemaphoreCount: Default::default(),
      pWaitSemaphores: core::ptr::null(),
      bufferBindCount: Default::default(),
      pBufferBinds: core::ptr::null(),
      imageOpaqueBindCount: Default::default(),
      pImageOpaqueBinds: core::ptr::null(),
      imageBindCount: Default::default(),
      pImageBinds: core::ptr::null(),
      signalSemaphoreCount: Default::default(),
      pSignalSemaphores: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBindVertexBufferIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindVertexBufferIndirectCommandNV {
  pub bufferAddress: VkDeviceAddress,
  pub size: u32,
  pub stride: u32,
}
impl Default for VkBindVertexBufferIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      bufferAddress: Default::default(),
      size: Default::default(),
      stride: Default::default(),
    }
  }
}

/// Khronos: [VkBindVideoSessionMemoryInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindVideoSessionMemoryInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindVideoSessionMemoryInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BIND_VIDEO_SESSION_MEMORY_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memoryBindIndex: u32,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  pub memorySize: VkDeviceSize,
}
impl Default for VkBindVideoSessionMemoryInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BIND_VIDEO_SESSION_MEMORY_INFO_KHR,
      pNext: core::ptr::null(),
      memoryBindIndex: Default::default(),
      memory: Default::default(),
      memoryOffset: Default::default(),
      memorySize: Default::default(),
    }
  }
}

/// Khronos: [VkBlitImageInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlitImageInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBlitImageInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// * Len: `regionCount`
  pub pRegions: *const VkImageBlit2,
  pub filter: VkFilter,
}
impl Default for VkBlitImageInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2,
      pNext: core::ptr::null(),
      srcImage: Default::default(),
      srcImageLayout: Default::default(),
      dstImage: Default::default(),
      dstImageLayout: Default::default(),
      regionCount: Default::default(),
      pRegions: core::ptr::null(),
      filter: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCaptureDescriptorDataInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCaptureDescriptorDataInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub buffer: VkBuffer,
}
impl Default for VkBufferCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      pNext: core::ptr::null(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionBufferCreateInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionBufferCreateInfoFUCHSIA.html)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionBufferCreateInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub collection: VkBufferCollectionFUCHSIA,
  pub index: u32,
}
impl Default for VkBufferCollectionBufferCreateInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      collection: Default::default(),
      index: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionConstraintsInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionConstraintsInfoFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionConstraintsInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub minBufferCount: u32,
  pub maxBufferCount: u32,
  pub minBufferCountForCamping: u32,
  pub minBufferCountForDedicatedSlack: u32,
  pub minBufferCountForSharedSlack: u32,
}
impl Default for VkBufferCollectionConstraintsInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      minBufferCount: Default::default(),
      maxBufferCount: Default::default(),
      minBufferCountForCamping: Default::default(),
      minBufferCountForDedicatedSlack: Default::default(),
      minBufferCountForSharedSlack: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionCreateInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionCreateInfoFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionCreateInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub collectionToken: zx_handle_t,
}
impl Default for VkBufferCollectionCreateInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      collectionToken: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionImageCreateInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionImageCreateInfoFUCHSIA.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionImageCreateInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub collection: VkBufferCollectionFUCHSIA,
  pub index: u32,
}
impl Default for VkBufferCollectionImageCreateInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      collection: Default::default(),
      index: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCollectionPropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionPropertiesFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCollectionPropertiesFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
  pub bufferCount: u32,
  pub createInfoIndex: u32,
  pub sysmemPixelFormat: u64,
  pub formatFeatures: VkFormatFeatureFlags,
  pub sysmemColorSpaceIndex: VkSysmemColorSpaceFUCHSIA,
  pub samplerYcbcrConversionComponents: VkComponentMapping,
  pub suggestedYcbcrModel: VkSamplerYcbcrModelConversion,
  pub suggestedYcbcrRange: VkSamplerYcbcrRange,
  pub suggestedXChromaOffset: VkChromaLocation,
  pub suggestedYChromaOffset: VkChromaLocation,
}
impl Default for VkBufferCollectionPropertiesFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA,
      pNext: core::ptr::null_mut(),
      memoryTypeBits: Default::default(),
      bufferCount: Default::default(),
      createInfoIndex: Default::default(),
      sysmemPixelFormat: Default::default(),
      formatFeatures: Default::default(),
      sysmemColorSpaceIndex: Default::default(),
      samplerYcbcrConversionComponents: Default::default(),
      suggestedYcbcrModel: Default::default(),
      suggestedYcbcrRange: Default::default(),
      suggestedXChromaOffset: Default::default(),
      suggestedYChromaOffset: Default::default(),
    }
  }
}

/// Khronos: [VkBufferConstraintsInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferConstraintsInfoFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferConstraintsInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub createInfo: VkBufferCreateInfo,
  /// * Optional: true
  pub requiredFormatFeatures: VkFormatFeatureFlags,
  pub bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA,
}
impl Default for VkBufferConstraintsInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      createInfo: Default::default(),
      requiredFormatFeatures: Default::default(),
      bufferCollectionConstraints: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCopy](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCopy.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCopy {
  /// Specified in bytes
  pub srcOffset: VkDeviceSize,
  /// Specified in bytes
  pub dstOffset: VkDeviceSize,
  /// Specified in bytes
  /// * No Auto-Validity
  pub size: VkDeviceSize,
}
impl Default for VkBufferCopy {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      srcOffset: Default::default(),
      dstOffset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCopy2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCopy2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCopy2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_COPY_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Specified in bytes
  pub srcOffset: VkDeviceSize,
  /// Specified in bytes
  pub dstOffset: VkDeviceSize,
  /// Specified in bytes
  /// * No Auto-Validity
  pub size: VkDeviceSize,
}
impl Default for VkBufferCopy2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_COPY_2,
      pNext: core::ptr::null(),
      srcOffset: Default::default(),
      dstOffset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkBufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Buffer creation flags
  /// * Optional: true
  pub flags: VkBufferCreateFlags,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Buffer usage flags
  pub usage: VkBufferUsageFlags,
  pub sharingMode: VkSharingMode,
  /// * Optional: true
  pub queueFamilyIndexCount: u32,
  /// * Len: `queueFamilyIndexCount`
  /// * No Auto-Validity
  pub pQueueFamilyIndices: *const u32,
}
impl Default for VkBufferCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      size: Default::default(),
      usage: Default::default(),
      sharingMode: Default::default(),
      queueFamilyIndexCount: Default::default(),
      pQueueFamilyIndices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkBufferDeviceAddressCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferDeviceAddressCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub deviceAddress: VkDeviceAddress,
}
impl Default for VkBufferDeviceAddressCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      deviceAddress: Default::default(),
    }
  }
}

/// Khronos: [VkBufferDeviceAddressInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferDeviceAddressInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub buffer: VkBuffer,
}
impl Default for VkBufferDeviceAddressInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO,
      pNext: core::ptr::null(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkBufferImageCopy](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferImageCopy {
  /// Specified in bytes
  pub bufferOffset: VkDeviceSize,
  /// Specified in texels
  pub bufferRowLength: u32,
  pub bufferImageHeight: u32,
  pub imageSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub imageOffset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub imageExtent: VkExtent3D,
}
impl Default for VkBufferImageCopy {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      bufferOffset: Default::default(),
      bufferRowLength: Default::default(),
      bufferImageHeight: Default::default(),
      imageSubresource: Default::default(),
      imageOffset: Default::default(),
      imageExtent: Default::default(),
    }
  }
}

/// Khronos: [VkBufferImageCopy2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferImageCopy2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Specified in bytes
  pub bufferOffset: VkDeviceSize,
  /// Specified in texels
  pub bufferRowLength: u32,
  pub bufferImageHeight: u32,
  pub imageSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub imageOffset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub imageExtent: VkExtent3D,
}
impl Default for VkBufferImageCopy2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2,
      pNext: core::ptr::null(),
      bufferOffset: Default::default(),
      bufferRowLength: Default::default(),
      bufferImageHeight: Default::default(),
      imageSubresource: Default::default(),
      imageOffset: Default::default(),
      imageExtent: Default::default(),
    }
  }
}

/// Khronos: [VkBufferMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferMemoryBarrier {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * No Auto-Validity
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * No Auto-Validity
  pub dstAccessMask: VkAccessFlags,
  /// Queue family to transition ownership from
  pub srcQueueFamilyIndex: u32,
  /// Queue family to transition ownership to
  pub dstQueueFamilyIndex: u32,
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
      sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
      pNext: core::ptr::null(),
      srcAccessMask: Default::default(),
      dstAccessMask: Default::default(),
      srcQueueFamilyIndex: Default::default(),
      dstQueueFamilyIndex: Default::default(),
      buffer: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkBufferMemoryBarrier2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferMemoryBarrier2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub srcStageMask: VkPipelineStageFlags2,
  /// * Optional: true
  pub srcAccessMask: VkAccessFlags2,
  /// * Optional: true
  pub dstStageMask: VkPipelineStageFlags2,
  /// * Optional: true
  pub dstAccessMask: VkAccessFlags2,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
}
impl Default for VkBufferMemoryBarrier2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2,
      pNext: core::ptr::null(),
      srcStageMask: Default::default(),
      srcAccessMask: Default::default(),
      dstStageMask: Default::default(),
      dstAccessMask: Default::default(),
      srcQueueFamilyIndex: Default::default(),
      dstQueueFamilyIndex: Default::default(),
      buffer: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkBufferMemoryRequirementsInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryRequirementsInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferMemoryRequirementsInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub buffer: VkBuffer,
}
impl Default for VkBufferMemoryRequirementsInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2,
      pNext: core::ptr::null(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkBufferOpaqueCaptureAddressCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferOpaqueCaptureAddressCreateInfo.html)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub opaqueCaptureAddress: u64,
}
impl Default for VkBufferOpaqueCaptureAddressCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO,
      pNext: core::ptr::null(),
      opaqueCaptureAddress: Default::default(),
    }
  }
}

/// Khronos: [VkBufferViewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferViewCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      buffer: Default::default(),
      format: Default::default(),
      offset: Default::default(),
      range: Default::default(),
    }
  }
}

/// Khronos: [VkCalibratedTimestampInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCalibratedTimestampInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCalibratedTimestampInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub timeDomain: VkTimeDomainEXT,
}
impl Default for VkCalibratedTimestampInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT,
      pNext: core::ptr::null(),
      timeDomain: Default::default(),
    }
  }
}

/// Khronos: [VkCheckpointData2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCheckpointData2NV.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCheckpointData2NV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub stage: VkPipelineStageFlags2,
  /// * No Auto-Validity
  pub pCheckpointMarker: *mut c_void,
}
impl Default for VkCheckpointData2NV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV,
      pNext: core::ptr::null_mut(),
      stage: Default::default(),
      pCheckpointMarker: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkCheckpointDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCheckpointDataNV.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCheckpointDataNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub stage: VkPipelineStageFlagBits,
  /// * No Auto-Validity
  pub pCheckpointMarker: *mut c_void,
}
impl Default for VkCheckpointDataNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV,
      pNext: core::ptr::null_mut(),
      stage: Default::default(),
      pCheckpointMarker: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkClearAttachment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearAttachment.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearAttachment {
  pub aspectMask: VkImageAspectFlags,
  pub colorAttachment: u32,
  /// * No Auto-Validity
  pub clearValue: VkClearValue,
}
impl Default for VkClearAttachment {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      aspectMask: Default::default(),
      colorAttachment: Default::default(),
      clearValue: Default::default(),
    }
  }
}

/// Khronos: [VkClearDepthStencilValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearDepthStencilValue.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearDepthStencilValue {
  pub depth: c_float,
  pub stencil: u32,
}
impl Default for VkClearDepthStencilValue {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      depth: Default::default(),
      stencil: Default::default(),
    }
  }
}

/// Khronos: [VkClearRect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearRect.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearRect {
  pub rect: VkRect2D,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
impl Default for VkClearRect {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      rect: Default::default(),
      baseArrayLayer: Default::default(),
      layerCount: Default::default(),
    }
  }
}

/// Khronos: [VkCoarseSampleLocationNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleLocationNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCoarseSampleLocationNV {
  pub pixelX: u32,
  pub pixelY: u32,
  pub sample: u32,
}
impl Default for VkCoarseSampleLocationNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      pixelX: Default::default(),
      pixelY: Default::default(),
      sample: Default::default(),
    }
  }
}

/// Khronos: [VkCoarseSampleOrderCustomNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderCustomNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCoarseSampleOrderCustomNV {
  pub shadingRate: VkShadingRatePaletteEntryNV,
  pub sampleCount: u32,
  pub sampleLocationCount: u32,
  /// * Len: `sampleLocationCount`
  pub pSampleLocations: *const VkCoarseSampleLocationNV,
}
impl Default for VkCoarseSampleOrderCustomNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      shadingRate: Default::default(),
      sampleCount: Default::default(),
      sampleLocationCount: Default::default(),
      pSampleLocations: core::ptr::null(),
    }
  }
}

/// Khronos: [VkColorBlendAdvancedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorBlendAdvancedEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkColorBlendAdvancedEXT {
  pub advancedBlendOp: VkBlendOp,
  pub srcPremultiplied: VkBool32,
  pub dstPremultiplied: VkBool32,
  pub blendOverlap: VkBlendOverlapEXT,
  pub clampResults: VkBool32,
}
impl Default for VkColorBlendAdvancedEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      advancedBlendOp: Default::default(),
      srcPremultiplied: Default::default(),
      dstPremultiplied: Default::default(),
      blendOverlap: Default::default(),
      clampResults: Default::default(),
    }
  }
}

/// Khronos: [VkColorBlendEquationEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorBlendEquationEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkColorBlendEquationEXT {
  pub srcColorBlendFactor: VkBlendFactor,
  pub dstColorBlendFactor: VkBlendFactor,
  pub colorBlendOp: VkBlendOp,
  pub srcAlphaBlendFactor: VkBlendFactor,
  pub dstAlphaBlendFactor: VkBlendFactor,
  pub alphaBlendOp: VkBlendOp,
}
impl Default for VkColorBlendEquationEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      srcColorBlendFactor: Default::default(),
      dstColorBlendFactor: Default::default(),
      colorBlendOp: Default::default(),
      srcAlphaBlendFactor: Default::default(),
      dstAlphaBlendFactor: Default::default(),
      alphaBlendOp: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferAllocateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub commandPool: VkCommandPool,
  pub level: VkCommandBufferLevel,
  pub commandBufferCount: u32,
}
impl Default for VkCommandBufferAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
      pNext: core::ptr::null(),
      commandPool: Default::default(),
      level: Default::default(),
      commandBufferCount: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferBeginInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Command buffer usage flags
  /// * Optional: true
  pub flags: VkCommandBufferUsageFlags,
  /// Pointer to inheritance info for secondary command buffers
  /// * Optional: true
  /// * No Auto-Validity
  pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}
impl Default for VkCommandBufferBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pInheritanceInfo: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceConditionalRenderingInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Whether this secondary command buffer may be executed during an active conditional rendering
  pub conditionalRenderingEnable: VkBool32,
}
impl Default for VkCommandBufferInheritanceConditionalRenderingInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT,
      pNext: core::ptr::null(),
      conditionalRenderingEnable: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Render pass for secondary command buffers
  /// * Optional: true
  /// * No Auto-Validity
  pub renderPass: VkRenderPass,
  pub subpass: u32,
  /// Framebuffer for secondary command buffers
  /// * Optional: true
  /// * No Auto-Validity
  pub framebuffer: VkFramebuffer,
  /// Whether this secondary command buffer may be executed during an occlusion query
  pub occlusionQueryEnable: VkBool32,
  /// Query flags used by this secondary command buffer, if executed during an occlusion query
  /// * Optional: true
  /// * No Auto-Validity
  pub queryFlags: VkQueryControlFlags,
  /// Pipeline statistics that may be counted for this secondary command buffer
  /// * Optional: true
  /// * No Auto-Validity
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}
impl Default for VkCommandBufferInheritanceInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
      pNext: core::ptr::null(),
      renderPass: Default::default(),
      subpass: Default::default(),
      framebuffer: Default::default(),
      occlusionQueryEnable: Default::default(),
      queryFlags: Default::default(),
      pipelineStatistics: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceRenderPassTransformInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM`
  pub sType: VkStructureType,
  /// Pointer to next structure
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * No Auto-Validity
  pub transform: VkSurfaceTransformFlagBitsKHR,
  pub renderArea: VkRect2D,
}
impl Default for VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM,
      pNext: core::ptr::null_mut(),
      transform: Default::default(),
      renderArea: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceRenderingInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderingInfo.html)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferInheritanceRenderingInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkRenderingFlags,
  pub viewMask: u32,
  /// * Optional: true
  pub colorAttachmentCount: u32,
  /// * Len: `colorAttachmentCount`
  pub pColorAttachmentFormats: *const VkFormat,
  pub depthAttachmentFormat: VkFormat,
  pub stencilAttachmentFormat: VkFormat,
  /// * Optional: true
  pub rasterizationSamples: VkSampleCountFlagBits,
}
impl Default for VkCommandBufferInheritanceRenderingInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      viewMask: Default::default(),
      colorAttachmentCount: Default::default(),
      pColorAttachmentFormats: core::ptr::null(),
      depthAttachmentFormat: Default::default(),
      stencilAttachmentFormat: Default::default(),
      rasterizationSamples: Default::default(),
    }
  }
}

/// Khronos: [VkCommandBufferInheritanceViewportScissorInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceViewportScissorInfoNV.html)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferInheritanceViewportScissorInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub viewportScissor2D: VkBool32,
  pub viewportDepthCount: u32,
  /// * No Auto-Validity
  pub pViewportDepths: *const VkViewport,
}
impl Default for VkCommandBufferInheritanceViewportScissorInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV,
      pNext: core::ptr::null(),
      viewportScissor2D: Default::default(),
      viewportDepthCount: Default::default(),
      pViewportDepths: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCommandBufferSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferSubmitInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub commandBuffer: VkCommandBuffer,
  pub deviceMask: u32,
}
impl Default for VkCommandBufferSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO,
      pNext: core::ptr::null(),
      commandBuffer: Default::default(),
      deviceMask: Default::default(),
    }
  }
}

/// Khronos: [VkCommandPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandPoolCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Command pool creation flags
  /// * Optional: true
  pub flags: VkCommandPoolCreateFlags,
  pub queueFamilyIndex: u32,
}
impl Default for VkCommandPoolCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      queueFamilyIndex: Default::default(),
    }
  }
}

/// Khronos: [VkComponentMapping](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentMapping.html)
#[derive(Clone, Copy)]
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

/// Khronos: [VkComputePipelineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComputePipelineCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkComputePipelineCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Pipeline creation flags
  /// * Optional: true
  pub flags: VkPipelineCreateFlags,
  pub stage: VkPipelineShaderStageCreateInfo,
  /// Interface layout of the pipeline
  pub layout: VkPipelineLayout,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it specifies the handle of the base pipeline this is a derivative of
  /// * Optional: true
  /// * No Auto-Validity
  pub basePipelineHandle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it specifies an index into pCreateInfos of the base pipeline this is a derivative of
  pub basePipelineIndex: i32,
}
impl Default for VkComputePipelineCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      stage: Default::default(),
      layout: Default::default(),
      basePipelineHandle: Default::default(),
      basePipelineIndex: Default::default(),
    }
  }
}

/// Khronos: [VkConditionalRenderingBeginInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkConditionalRenderingBeginInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT,
      pNext: core::ptr::null(),
      buffer: Default::default(),
      offset: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkConformanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConformanceVersion.html)
#[derive(Clone, Copy)]
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

/// Khronos: [VkCooperativeMatrixPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCooperativeMatrixPropertiesNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCooperativeMatrixPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub MSize: u32,
  pub NSize: u32,
  pub KSize: u32,
  pub AType: VkComponentTypeNV,
  pub BType: VkComponentTypeNV,
  pub CType: VkComponentTypeNV,
  pub DType: VkComponentTypeNV,
  pub scope: VkScopeNV,
}
impl Default for VkCooperativeMatrixPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      MSize: Default::default(),
      NSize: Default::default(),
      KSize: Default::default(),
      AType: Default::default(),
      BType: Default::default(),
      CType: Default::default(),
      DType: Default::default(),
      scope: Default::default(),
    }
  }
}

/// Khronos: [VkCopyAccelerationStructureInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyAccelerationStructureInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub src: VkAccelerationStructureKHR,
  pub dst: VkAccelerationStructureKHR,
  pub mode: VkCopyAccelerationStructureModeKHR,
}
impl Default for VkCopyAccelerationStructureInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR,
      pNext: core::ptr::null(),
      src: Default::default(),
      dst: Default::default(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkCopyAccelerationStructureToMemoryInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyAccelerationStructureToMemoryInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub src: VkAccelerationStructureKHR,
  /// * No Auto-Validity
  pub dst: VkDeviceOrHostAddressKHR,
  pub mode: VkCopyAccelerationStructureModeKHR,
}
impl Default for VkCopyAccelerationStructureToMemoryInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR,
      pNext: core::ptr::null(),
      src: Default::default(),
      dst: Default::default(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkCopyBufferInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyBufferInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyBufferInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcBuffer: VkBuffer,
  pub dstBuffer: VkBuffer,
  pub regionCount: u32,
  /// * Len: `regionCount`
  pub pRegions: *const VkBufferCopy2,
}
impl Default for VkCopyBufferInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2,
      pNext: core::ptr::null(),
      srcBuffer: Default::default(),
      dstBuffer: Default::default(),
      regionCount: Default::default(),
      pRegions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCopyBufferToImageInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyBufferToImageInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyBufferToImageInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcBuffer: VkBuffer,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// * Len: `regionCount`
  pub pRegions: *const VkBufferImageCopy2,
}
impl Default for VkCopyBufferToImageInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2,
      pNext: core::ptr::null(),
      srcBuffer: Default::default(),
      dstImage: Default::default(),
      dstImageLayout: Default::default(),
      regionCount: Default::default(),
      pRegions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCopyCommandTransformInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyCommandTransformInfoQCOM.html)
///
/// * Struct Extends: [`VkBufferImageCopy2`]
/// * Struct Extends: [`VkImageBlit2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyCommandTransformInfoQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *const c_void,
  /// * No Auto-Validity
  pub transform: VkSurfaceTransformFlagBitsKHR,
}
impl Default for VkCopyCommandTransformInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM,
      pNext: core::ptr::null(),
      transform: Default::default(),
    }
  }
}

/// Khronos: [VkCopyDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyDescriptorSet.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyDescriptorSet {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Source descriptor set
  pub srcSet: VkDescriptorSet,
  /// Binding within the source descriptor set to copy from
  pub srcBinding: u32,
  /// Array element within the source binding to copy from
  pub srcArrayElement: u32,
  /// Destination descriptor set
  pub dstSet: VkDescriptorSet,
  /// Binding within the destination descriptor set to copy to
  pub dstBinding: u32,
  /// Array element within the destination binding to copy to
  pub dstArrayElement: u32,
  /// Number of descriptors to write (determines the size of the array pointed by pDescriptors)
  pub descriptorCount: u32,
}
impl Default for VkCopyDescriptorSet {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET,
      pNext: core::ptr::null(),
      srcSet: Default::default(),
      srcBinding: Default::default(),
      srcArrayElement: Default::default(),
      dstSet: Default::default(),
      dstBinding: Default::default(),
      dstArrayElement: Default::default(),
      descriptorCount: Default::default(),
    }
  }
}

/// Khronos: [VkCopyImageInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyImageInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyImageInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// * Len: `regionCount`
  pub pRegions: *const VkImageCopy2,
}
impl Default for VkCopyImageInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2,
      pNext: core::ptr::null(),
      srcImage: Default::default(),
      srcImageLayout: Default::default(),
      dstImage: Default::default(),
      dstImageLayout: Default::default(),
      regionCount: Default::default(),
      pRegions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCopyImageToBufferInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyImageToBufferInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyImageToBufferInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub dstBuffer: VkBuffer,
  pub regionCount: u32,
  /// * Len: `regionCount`
  pub pRegions: *const VkBufferImageCopy2,
}
impl Default for VkCopyImageToBufferInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2,
      pNext: core::ptr::null(),
      srcImage: Default::default(),
      srcImageLayout: Default::default(),
      dstBuffer: Default::default(),
      regionCount: Default::default(),
      pRegions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCopyMemoryIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryIndirectCommandNV {
  pub srcAddress: VkDeviceAddress,
  pub dstAddress: VkDeviceAddress,
  /// Specified in bytes
  pub size: VkDeviceSize,
}
impl Default for VkCopyMemoryIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      srcAddress: Default::default(),
      dstAddress: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkCopyMemoryToAccelerationStructureInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryToAccelerationStructureInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * No Auto-Validity
  pub src: VkDeviceOrHostAddressConstKHR,
  pub dst: VkAccelerationStructureKHR,
  pub mode: VkCopyAccelerationStructureModeKHR,
}
impl Default for VkCopyMemoryToAccelerationStructureInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR,
      pNext: core::ptr::null(),
      src: Default::default(),
      dst: Default::default(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkCopyMemoryToImageIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToImageIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryToImageIndirectCommandNV {
  pub srcAddress: VkDeviceAddress,
  /// Specified in texels
  pub bufferRowLength: u32,
  pub bufferImageHeight: u32,
  pub imageSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub imageOffset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub imageExtent: VkExtent3D,
}
impl Default for VkCopyMemoryToImageIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      srcAddress: Default::default(),
      bufferRowLength: Default::default(),
      bufferImageHeight: Default::default(),
      imageSubresource: Default::default(),
      imageOffset: Default::default(),
      imageExtent: Default::default(),
    }
  }
}

/// Khronos: [VkCopyMemoryToMicromapInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToMicromapInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryToMicromapInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_MEMORY_TO_MICROMAP_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * No Auto-Validity
  pub src: VkDeviceOrHostAddressConstKHR,
  pub dst: VkMicromapEXT,
  pub mode: VkCopyMicromapModeEXT,
}
impl Default for VkCopyMemoryToMicromapInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_MEMORY_TO_MICROMAP_INFO_EXT,
      pNext: core::ptr::null(),
      src: Default::default(),
      dst: Default::default(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkCopyMicromapInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMicromapInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMicromapInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_MICROMAP_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub src: VkMicromapEXT,
  pub dst: VkMicromapEXT,
  pub mode: VkCopyMicromapModeEXT,
}
impl Default for VkCopyMicromapInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_MICROMAP_INFO_EXT,
      pNext: core::ptr::null(),
      src: Default::default(),
      dst: Default::default(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkCopyMicromapToMemoryInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMicromapToMemoryInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMicromapToMemoryInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COPY_MICROMAP_TO_MEMORY_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub src: VkMicromapEXT,
  /// * No Auto-Validity
  pub dst: VkDeviceOrHostAddressKHR,
  pub mode: VkCopyMicromapModeEXT,
}
impl Default for VkCopyMicromapToMemoryInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_COPY_MICROMAP_TO_MEMORY_INFO_EXT,
      pNext: core::ptr::null(),
      src: Default::default(),
      dst: Default::default(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkCuFunctionCreateInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuFunctionCreateInfoNVX.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCuFunctionCreateInfoNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub module: VkCuModuleNVX,
  /// * Len: `null-terminated`
  pub pName: *const u8,
}
impl Default for VkCuFunctionCreateInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX,
      pNext: core::ptr::null(),
      module: Default::default(),
      pName: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCuLaunchInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuLaunchInfoNVX.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCuLaunchInfoNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub function: VkCuFunctionNVX,
  pub gridDimX: u32,
  pub gridDimY: u32,
  pub gridDimZ: u32,
  pub blockDimX: u32,
  pub blockDimY: u32,
  pub blockDimZ: u32,
  pub sharedMemBytes: u32,
  /// * Optional: true
  pub paramCount: c_size_t,
  /// * Len: `paramCount`
  pub pParams: *const *const c_void,
  /// * Optional: true
  pub extraCount: c_size_t,
  /// * Len: `extraCount`
  pub pExtras: *const *const c_void,
}
impl Default for VkCuLaunchInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX,
      pNext: core::ptr::null(),
      function: Default::default(),
      gridDimX: Default::default(),
      gridDimY: Default::default(),
      gridDimZ: Default::default(),
      blockDimX: Default::default(),
      blockDimY: Default::default(),
      blockDimZ: Default::default(),
      sharedMemBytes: Default::default(),
      paramCount: Default::default(),
      pParams: core::ptr::null(),
      extraCount: Default::default(),
      pExtras: core::ptr::null(),
    }
  }
}

/// Khronos: [VkCuModuleCreateInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuModuleCreateInfoNVX.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCuModuleCreateInfoNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub dataSize: c_size_t,
  /// * Len: `dataSize`
  pub pData: *const c_void,
}
impl Default for VkCuModuleCreateInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX,
      pNext: core::ptr::null(),
      dataSize: Default::default(),
      pData: core::ptr::null(),
    }
  }
}

/// Khronos: [VkD3D12FenceSubmitInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkD3D12FenceSubmitInfoKHR.html)
///
/// * Struct Extends: [`VkSubmitInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkD3D12FenceSubmitInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub waitSemaphoreValuesCount: u32,
  /// * Optional: true
  /// * Len: `waitSemaphoreValuesCount`
  pub pWaitSemaphoreValues: *const u64,
  /// * Optional: true
  pub signalSemaphoreValuesCount: u32,
  /// * Optional: true
  /// * Len: `signalSemaphoreValuesCount`
  pub pSignalSemaphoreValues: *const u64,
}
impl Default for VkD3D12FenceSubmitInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR,
      pNext: core::ptr::null(),
      waitSemaphoreValuesCount: Default::default(),
      pWaitSemaphoreValues: core::ptr::null(),
      signalSemaphoreValuesCount: Default::default(),
      pSignalSemaphoreValues: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugMarkerMarkerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugMarkerMarkerInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Name of the debug marker
  /// * Len: `null-terminated`
  pub pMarkerName: *const u8,
  /// Optional color for debug marker
  pub color: [c_float; 4],
}
impl Default for VkDebugMarkerMarkerInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT,
      pNext: core::ptr::null(),
      pMarkerName: core::ptr::null(),
      color: [Default::default(); 4],
    }
  }
}

/// Khronos: [VkDebugMarkerObjectNameInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugMarkerObjectNameInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// The type of the object
  pub objectType: VkDebugReportObjectTypeEXT,
  /// The handle of the object, cast to uint64_t
  /// * Object Type: objectType
  pub object: u64,
  /// Name to apply to the object
  /// * Len: `null-terminated`
  pub pObjectName: *const u8,
}
impl Default for VkDebugMarkerObjectNameInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
      pNext: core::ptr::null(),
      objectType: Default::default(),
      object: Default::default(),
      pObjectName: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugMarkerObjectTagInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugMarkerObjectTagInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// The type of the object
  pub objectType: VkDebugReportObjectTypeEXT,
  /// The handle of the object, cast to uint64_t
  /// * Object Type: objectType
  pub object: u64,
  /// The name of the tag to set on the object
  pub tagName: u64,
  /// The length in bytes of the tag data
  pub tagSize: c_size_t,
  /// Tag data to attach to the object
  /// * Len: `tagSize`
  pub pTag: *const c_void,
}
impl Default for VkDebugMarkerObjectTagInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
      pNext: core::ptr::null(),
      objectType: Default::default(),
      object: Default::default(),
      tagName: Default::default(),
      tagSize: Default::default(),
      pTag: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugReportCallbackCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Indicates which events call this callback
  /// * Optional: true
  pub flags: VkDebugReportFlagsEXT,
  /// Function pointer of a callback function
  pub pfnCallback: PFN_vkDebugReportCallbackEXT,
  /// User data provided to callback function
  /// * Optional: true
  pub pUserData: *mut c_void,
}
impl Default for VkDebugReportCallbackCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pfnCallback: Default::default(),
      pUserData: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsLabelEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Len: `null-terminated`
  pub pLabelName: *const u8,
  pub color: [c_float; 4],
}
impl Default for VkDebugUtilsLabelEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT,
      pNext: core::ptr::null(),
      pLabelName: core::ptr::null(),
      color: [Default::default(); 4],
    }
  }
}

/// Khronos: [VkDebugUtilsMessengerCallbackDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
  /// * Optional: true
  /// * Len: `null-terminated`
  pub pMessageIdName: *const u8,
  pub messageIdNumber: i32,
  /// * Len: `null-terminated`
  pub pMessage: *const u8,
  /// * Optional: true
  pub queueLabelCount: u32,
  /// * Len: `queueLabelCount`
  pub pQueueLabels: *const VkDebugUtilsLabelEXT,
  /// * Optional: true
  pub cmdBufLabelCount: u32,
  /// * Len: `cmdBufLabelCount`
  pub pCmdBufLabels: *const VkDebugUtilsLabelEXT,
  /// * Optional: true
  pub objectCount: u32,
  /// * Len: `objectCount`
  pub pObjects: *const VkDebugUtilsObjectNameInfoEXT,
}
impl Default for VkDebugUtilsMessengerCallbackDataEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pMessageIdName: core::ptr::null(),
      messageIdNumber: Default::default(),
      pMessage: core::ptr::null(),
      queueLabelCount: Default::default(),
      pQueueLabels: core::ptr::null(),
      cmdBufLabelCount: Default::default(),
      pCmdBufLabels: core::ptr::null(),
      objectCount: Default::default(),
      pObjects: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugUtilsMessengerCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
  pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
  pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
  pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
  /// * Optional: true
  pub pUserData: *mut c_void,
}
impl Default for VkDebugUtilsMessengerCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      messageSeverity: Default::default(),
      messageType: Default::default(),
      pfnUserCallback: Default::default(),
      pUserData: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDebugUtilsObjectNameInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsObjectNameInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub objectType: VkObjectType,
  /// * Object Type: objectType
  pub objectHandle: u64,
  /// * Optional: true
  /// * Len: `null-terminated`
  pub pObjectName: *const u8,
}
impl Default for VkDebugUtilsObjectNameInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
      pNext: core::ptr::null(),
      objectType: Default::default(),
      objectHandle: Default::default(),
      pObjectName: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDebugUtilsObjectTagInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDebugUtilsObjectTagInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub objectType: VkObjectType,
  /// * Object Type: objectType
  pub objectHandle: u64,
  pub tagName: u64,
  pub tagSize: c_size_t,
  /// * Len: `tagSize`
  pub pTag: *const c_void,
}
impl Default for VkDebugUtilsObjectTagInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
      pNext: core::ptr::null(),
      objectType: Default::default(),
      objectHandle: Default::default(),
      tagName: Default::default(),
      tagSize: Default::default(),
      pTag: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDecompressMemoryRegionNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDecompressMemoryRegionNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDecompressMemoryRegionNV {
  pub srcAddress: VkDeviceAddress,
  pub dstAddress: VkDeviceAddress,
  /// Specified in bytes
  pub compressedSize: VkDeviceSize,
  /// Specified in bytes
  pub decompressedSize: VkDeviceSize,
  pub decompressionMethod: VkMemoryDecompressionMethodFlagsNV,
}
impl Default for VkDecompressMemoryRegionNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      srcAddress: Default::default(),
      dstAddress: Default::default(),
      compressedSize: Default::default(),
      decompressedSize: Default::default(),
      decompressionMethod: Default::default(),
    }
  }
}

/// Khronos: [VkDedicatedAllocationBufferCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationBufferCreateInfoNV.html)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Whether this buffer uses a dedicated allocation
  pub dedicatedAllocation: VkBool32,
}
impl Default for VkDedicatedAllocationBufferCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      dedicatedAllocation: Default::default(),
    }
  }
}

/// Khronos: [VkDedicatedAllocationImageCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Whether this image uses a dedicated allocation
  pub dedicatedAllocation: VkBool32,
}
impl Default for VkDedicatedAllocationImageCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      dedicatedAllocation: Default::default(),
    }
  }
}

/// Khronos: [VkDedicatedAllocationMemoryAllocateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
      pNext: core::ptr::null(),
      image: Default::default(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkDependencyInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDependencyInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEPENDENCY_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub dependencyFlags: VkDependencyFlags,
  /// * Optional: true
  pub memoryBarrierCount: u32,
  /// * Len: `memoryBarrierCount`
  pub pMemoryBarriers: *const VkMemoryBarrier2,
  /// * Optional: true
  pub bufferMemoryBarrierCount: u32,
  /// * Len: `bufferMemoryBarrierCount`
  pub pBufferMemoryBarriers: *const VkBufferMemoryBarrier2,
  /// * Optional: true
  pub imageMemoryBarrierCount: u32,
  /// * Len: `imageMemoryBarrierCount`
  pub pImageMemoryBarriers: *const VkImageMemoryBarrier2,
}
impl Default for VkDependencyInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEPENDENCY_INFO,
      pNext: core::ptr::null(),
      dependencyFlags: Default::default(),
      memoryBarrierCount: Default::default(),
      pMemoryBarriers: core::ptr::null(),
      bufferMemoryBarrierCount: Default::default(),
      pBufferMemoryBarriers: core::ptr::null(),
      imageMemoryBarrierCount: Default::default(),
      pImageMemoryBarriers: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorAddressInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorAddressInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorAddressInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub address: VkDeviceAddress,
  pub range: VkDeviceSize,
  pub format: VkFormat,
}
impl Default for VkDescriptorAddressInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT,
      pNext: core::ptr::null_mut(),
      address: Default::default(),
      range: Default::default(),
      format: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorBufferBindingInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferBindingInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorBufferBindingInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub address: VkDeviceAddress,
  pub usage: VkBufferUsageFlags,
}
impl Default for VkDescriptorBufferBindingInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT,
      pNext: core::ptr::null_mut(),
      address: Default::default(),
      usage: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorBufferBindingPushDescriptorBufferHandleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferBindingPushDescriptorBufferHandleEXT.html)
///
/// * Struct Extends: [`VkDescriptorBufferBindingInfoEXT`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub buffer: VkBuffer,
}
impl Default for VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT,
      pNext: core::ptr::null_mut(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorBufferInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferInfo.html)
#[derive(Clone, Copy)]
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

/// Khronos: [VkDescriptorGetInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorGetInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorGetInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub ty: VkDescriptorType,
  /// * Selector: type
  /// * No Auto-Validity
  pub data: VkDescriptorDataEXT,
}
impl Default for VkDescriptorGetInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT,
      pNext: core::ptr::null(),
      ty: Default::default(),
      data: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorImageInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorImageInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorImageInfo {
  /// Sampler to write to the descriptor in case it is a SAMPLER or COMBINED_IMAGE_SAMPLER descriptor. Ignored otherwise.
  /// * No Auto-Validity
  pub sampler: VkSampler,
  /// Image view to write to the descriptor in case it is a SAMPLED_IMAGE, STORAGE_IMAGE, COMBINED_IMAGE_SAMPLER, or INPUT_ATTACHMENT descriptor. Ignored otherwise.
  /// * No Auto-Validity
  pub imageView: VkImageView,
  /// Layout the image is expected to be in when accessed using this descriptor (only used if imageView is not VK_NULL_HANDLE).
  /// * No Auto-Validity
  pub imageLayout: VkImageLayout,
}
impl Default for VkDescriptorImageInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sampler: Default::default(),
      imageView: Default::default(),
      imageLayout: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDescriptorPoolCreateFlags,
  pub maxSets: u32,
  /// * Optional: true
  pub poolSizeCount: u32,
  /// * Len: `poolSizeCount`
  pub pPoolSizes: *const VkDescriptorPoolSize,
}
impl Default for VkDescriptorPoolCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      maxSets: Default::default(),
      poolSizeCount: Default::default(),
      pPoolSizes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorPoolInlineUniformBlockCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolInlineUniformBlockCreateInfo.html)
///
/// * Struct Extends: [`VkDescriptorPoolCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub maxInlineUniformBlockBindings: u32,
}
impl Default for VkDescriptorPoolInlineUniformBlockCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO,
      pNext: core::ptr::null(),
      maxInlineUniformBlockBindings: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorPoolSize](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolSize.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorPoolSize {
  pub ty: VkDescriptorType,
  pub descriptorCount: u32,
}
impl Default for VkDescriptorPoolSize {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      ty: Default::default(),
      descriptorCount: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorSetAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetAllocateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub descriptorPool: VkDescriptorPool,
  pub descriptorSetCount: u32,
  /// * Len: `descriptorSetCount`
  pub pSetLayouts: *const VkDescriptorSetLayout,
}
impl Default for VkDescriptorSetAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
      pNext: core::ptr::null(),
      descriptorPool: Default::default(),
      descriptorSetCount: Default::default(),
      pSetLayouts: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetBindingReferenceVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetBindingReferenceVALVE.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetBindingReferenceVALVE {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub descriptorSetLayout: VkDescriptorSetLayout,
  pub binding: u32,
}
impl Default for VkDescriptorSetBindingReferenceVALVE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE,
      pNext: core::ptr::null(),
      descriptorSetLayout: Default::default(),
      binding: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutBinding](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBinding.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
  /// Binding number for this entry
  pub binding: u32,
  /// Type of the descriptors in this binding
  pub descriptorType: VkDescriptorType,
  /// Number of descriptors in this binding
  /// * Optional: true
  pub descriptorCount: u32,
  /// Shader stages this binding is visible to
  /// * No Auto-Validity
  pub stageFlags: VkShaderStageFlags,
  /// Immutable samplers (used if descriptor type is SAMPLER or COMBINED_IMAGE_SAMPLER, is either NULL or contains count number of elements)
  /// * Optional: true
  /// * Len: `descriptorCount`
  /// * No Auto-Validity
  pub pImmutableSamplers: *const VkSampler,
}
impl Default for VkDescriptorSetLayoutBinding {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      binding: Default::default(),
      descriptorType: Default::default(),
      descriptorCount: Default::default(),
      stageFlags: Default::default(),
      pImmutableSamplers: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutBindingFlagsCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfo.html)
///
/// * Struct Extends: [`VkDescriptorSetLayoutCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub bindingCount: u32,
  /// * Optional: false,true
  /// * Len: `bindingCount`
  pub pBindingFlags: *const VkDescriptorBindingFlags,
}
impl Default for VkDescriptorSetLayoutBindingFlagsCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
      pNext: core::ptr::null(),
      bindingCount: Default::default(),
      pBindingFlags: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDescriptorSetLayoutCreateFlags,
  /// Number of bindings in the descriptor set layout
  /// * Optional: true
  pub bindingCount: u32,
  /// Array of descriptor set layout bindings
  /// * Len: `bindingCount`
  pub pBindings: *const VkDescriptorSetLayoutBinding,
}
impl Default for VkDescriptorSetLayoutCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      bindingCount: Default::default(),
      pBindings: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutHostMappingInfoVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutHostMappingInfoVALVE.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutHostMappingInfoVALVE {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub descriptorOffset: c_size_t,
  pub descriptorSize: u32,
}
impl Default for VkDescriptorSetLayoutHostMappingInfoVALVE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE,
      pNext: core::ptr::null_mut(),
      descriptorOffset: Default::default(),
      descriptorSize: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorSetLayoutSupport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutSupport.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutSupport {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub supported: VkBool32,
}
impl Default for VkDescriptorSetLayoutSupport {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT,
      pNext: core::ptr::null_mut(),
      supported: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorSetVariableDescriptorCountAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfo.html)
///
/// * Struct Extends: [`VkDescriptorSetAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub descriptorSetCount: u32,
  /// * Len: `descriptorSetCount`
  pub pDescriptorCounts: *const u32,
}
impl Default for VkDescriptorSetVariableDescriptorCountAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
      pNext: core::ptr::null(),
      descriptorSetCount: Default::default(),
      pDescriptorCounts: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDescriptorSetVariableDescriptorCountLayoutSupport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupport.html)
///
/// * Struct Extends: [`VkDescriptorSetLayoutSupport`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub maxVariableDescriptorCount: u32,
}
impl Default for VkDescriptorSetVariableDescriptorCountLayoutSupport {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT,
      pNext: core::ptr::null_mut(),
      maxVariableDescriptorCount: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorUpdateTemplateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorUpdateTemplateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDescriptorUpdateTemplateCreateFlags,
  /// Number of descriptor update entries to use for the update template
  pub descriptorUpdateEntryCount: u32,
  /// Descriptor update entries for the template
  /// * Len: `descriptorUpdateEntryCount`
  pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntry,
  pub templateType: VkDescriptorUpdateTemplateType,
  /// * No Auto-Validity
  pub descriptorSetLayout: VkDescriptorSetLayout,
  /// * No Auto-Validity
  pub pipelineBindPoint: VkPipelineBindPoint,
  /// If used for push descriptors, this is the only allowed layout
  /// * No Auto-Validity
  pub pipelineLayout: VkPipelineLayout,
  /// * No Auto-Validity
  pub set: u32,
}
impl Default for VkDescriptorUpdateTemplateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      descriptorUpdateEntryCount: Default::default(),
      pDescriptorUpdateEntries: core::ptr::null(),
      templateType: Default::default(),
      descriptorSetLayout: Default::default(),
      pipelineBindPoint: Default::default(),
      pipelineLayout: Default::default(),
      set: Default::default(),
    }
  }
}

/// Khronos: [VkDescriptorUpdateTemplateEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateEntry.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorUpdateTemplateEntry {
  /// Binding within the destination descriptor set to write
  pub dstBinding: u32,
  /// Array element within the destination binding to write
  pub dstArrayElement: u32,
  /// Number of descriptors to write
  pub descriptorCount: u32,
  /// Descriptor type to write
  pub descriptorType: VkDescriptorType,
  /// Offset into pData where the descriptors to update are stored
  pub offset: c_size_t,
  /// Stride between two descriptors in pData when writing more than one descriptor
  pub stride: c_size_t,
}
impl Default for VkDescriptorUpdateTemplateEntry {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      dstBinding: Default::default(),
      dstArrayElement: Default::default(),
      descriptorCount: Default::default(),
      descriptorType: Default::default(),
      offset: Default::default(),
      stride: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceAddressBindingCallbackDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingCallbackDataEXT.html)
///
/// * Struct Extends: [`VkDebugUtilsMessengerCallbackDataEXT`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceAddressBindingCallbackDataEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  pub flags: VkDeviceAddressBindingFlagsEXT,
  pub baseAddress: VkDeviceAddress,
  pub size: VkDeviceSize,
  pub bindingType: VkDeviceAddressBindingTypeEXT,
}
impl Default for VkDeviceAddressBindingCallbackDataEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT,
      pNext: core::ptr::null_mut(),
      flags: Default::default(),
      baseAddress: Default::default(),
      size: Default::default(),
      bindingType: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceBufferMemoryRequirements.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceBufferMemoryRequirements {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub pCreateInfo: *const VkBufferCreateInfo,
}
impl Default for VkDeviceBufferMemoryRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS,
      pNext: core::ptr::null(),
      pCreateInfo: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDeviceCreateFlags,
  pub queueCreateInfoCount: u32,
  /// * Len: `queueCreateInfoCount`
  pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
  /// * Optional: true
  #[deprecated = "ignored"]
  pub enabledLayerCount: u32,
  /// Ordered list of layer names to be enabled
  /// * Len: `enabledLayerCount,null-terminated`
  #[deprecated = "ignored"]
  pub ppEnabledLayerNames: *const *const u8,
  /// * Optional: true
  pub enabledExtensionCount: u32,
  /// * Len: `enabledExtensionCount,null-terminated`
  pub ppEnabledExtensionNames: *const *const u8,
  /// * Optional: true
  pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}
impl Default for VkDeviceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    #[allow(deprecated)]
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      queueCreateInfoCount: Default::default(),
      pQueueCreateInfos: core::ptr::null(),
      enabledLayerCount: Default::default(),
      ppEnabledLayerNames: core::ptr::null(),
      enabledExtensionCount: Default::default(),
      ppEnabledExtensionNames: core::ptr::null(),
      pEnabledFeatures: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceDeviceMemoryReportCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceDeviceMemoryReportCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub flags: VkDeviceMemoryReportFlagsEXT,
  pub pfnUserCallback: PFN_vkDeviceMemoryReportCallbackEXT,
  pub pUserData: *mut c_void,
}
impl Default for VkDeviceDeviceMemoryReportCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pfnUserCallback: Default::default(),
      pUserData: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDeviceDiagnosticsConfigCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceDiagnosticsConfigCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDeviceDiagnosticsConfigFlagsNV,
}
impl Default for VkDeviceDiagnosticsConfigCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceEventInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceEventInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub deviceEvent: VkDeviceEventTypeEXT,
}
impl Default for VkDeviceEventInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT,
      pNext: core::ptr::null(),
      deviceEvent: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceFaultAddressInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultAddressInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultAddressInfoEXT {
  pub addressType: VkDeviceFaultAddressTypeEXT,
  pub reportedAddress: VkDeviceAddress,
  pub addressPrecision: VkDeviceSize,
}
impl Default for VkDeviceFaultAddressInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      addressType: Default::default(),
      reportedAddress: Default::default(),
      addressPrecision: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceFaultCountsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultCountsEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultCountsEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_FAULT_COUNTS_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  pub addressInfoCount: u32,
  /// * Optional: true
  pub vendorInfoCount: u32,
  /// Specified in bytes
  /// * Optional: true
  pub vendorBinarySize: VkDeviceSize,
}
impl Default for VkDeviceFaultCountsEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_FAULT_COUNTS_EXT,
      pNext: core::ptr::null_mut(),
      addressInfoCount: Default::default(),
      vendorInfoCount: Default::default(),
      vendorBinarySize: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceFaultInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_FAULT_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Free-form description of the fault
  /// * No Auto-Validity
  pub description: [u8; VK_MAX_DESCRIPTION_SIZE],
  /// * Optional: true
  pub pAddressInfos: *mut VkDeviceFaultAddressInfoEXT,
  /// * Optional: true
  pub pVendorInfos: *mut VkDeviceFaultVendorInfoEXT,
  /// * Optional: true
  pub pVendorBinaryData: *mut c_void,
}
impl Default for VkDeviceFaultInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_FAULT_INFO_EXT,
      pNext: core::ptr::null_mut(),
      description: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      pAddressInfos: core::ptr::null_mut(),
      pVendorInfos: core::ptr::null_mut(),
      pVendorBinaryData: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDeviceFaultVendorBinaryHeaderVersionOneEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultVendorBinaryHeaderVersionOneEXT.html)
///
/// The fields in this structure are non-normative since structure packing is implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultVendorBinaryHeaderVersionOneEXT {
  pub headerSize: u32,
  pub headerVersion: VkDeviceFaultVendorBinaryHeaderVersionEXT,
  pub vendorID: u32,
  pub deviceID: u32,
  pub driverVersion: u32,
  pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
  pub applicationNameOffset: u32,
  pub applicationVersion: u32,
  pub engineNameOffset: u32,
}
impl Default for VkDeviceFaultVendorBinaryHeaderVersionOneEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      headerSize: Default::default(),
      headerVersion: Default::default(),
      vendorID: Default::default(),
      deviceID: Default::default(),
      driverVersion: Default::default(),
      pipelineCacheUUID: [Default::default(); VK_UUID_SIZE],
      applicationNameOffset: Default::default(),
      applicationVersion: Default::default(),
      engineNameOffset: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceFaultVendorInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultVendorInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultVendorInfoEXT {
  /// Free-form description of the fault
  /// * No Auto-Validity
  pub description: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub vendorFaultCode: u64,
  pub vendorFaultData: u64,
}
impl Default for VkDeviceFaultVendorInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      description: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      vendorFaultCode: Default::default(),
      vendorFaultData: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupBindSparseInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupBindSparseInfo.html)
///
/// * Struct Extends: [`VkBindSparseInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupBindSparseInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub resourceDeviceIndex: u32,
  pub memoryDeviceIndex: u32,
}
impl Default for VkDeviceGroupBindSparseInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO,
      pNext: core::ptr::null(),
      resourceDeviceIndex: Default::default(),
      memoryDeviceIndex: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupCommandBufferBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html)
///
/// * Struct Extends: [`VkCommandBufferBeginInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupCommandBufferBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub deviceMask: u32,
}
impl Default for VkDeviceGroupCommandBufferBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
      pNext: core::ptr::null(),
      deviceMask: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupDeviceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupDeviceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub physicalDeviceCount: u32,
  /// * Len: `physicalDeviceCount`
  pub pPhysicalDevices: *const VkPhysicalDevice,
}
impl Default for VkDeviceGroupDeviceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO,
      pNext: core::ptr::null(),
      physicalDeviceCount: Default::default(),
      pPhysicalDevices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceGroupPresentCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE],
  pub modes: VkDeviceGroupPresentModeFlagsKHR,
}
impl Default for VkDeviceGroupPresentCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
      pNext: core::ptr::null_mut(),
      presentMask: [Default::default(); VK_MAX_DEVICE_GROUP_SIZE],
      modes: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupPresentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentInfoKHR.html)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupPresentInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub swapchainCount: u32,
  /// * Len: `swapchainCount`
  pub pDeviceMasks: *const u32,
  pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
}
impl Default for VkDeviceGroupPresentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR,
      pNext: core::ptr::null(),
      swapchainCount: Default::default(),
      pDeviceMasks: core::ptr::null(),
      mode: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceGroupRenderPassBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupRenderPassBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub deviceMask: u32,
  /// * Optional: true
  pub deviceRenderAreaCount: u32,
  /// * Len: `deviceRenderAreaCount`
  pub pDeviceRenderAreas: *const VkRect2D,
}
impl Default for VkDeviceGroupRenderPassBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO,
      pNext: core::ptr::null(),
      deviceMask: Default::default(),
      deviceRenderAreaCount: Default::default(),
      pDeviceRenderAreas: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceGroupSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSubmitInfo.html)
///
/// * Struct Extends: [`VkSubmitInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub waitSemaphoreCount: u32,
  /// * Len: `waitSemaphoreCount`
  pub pWaitSemaphoreDeviceIndices: *const u32,
  /// * Optional: true
  pub commandBufferCount: u32,
  /// * Len: `commandBufferCount`
  pub pCommandBufferDeviceMasks: *const u32,
  /// * Optional: true
  pub signalSemaphoreCount: u32,
  /// * Len: `signalSemaphoreCount`
  pub pSignalSemaphoreDeviceIndices: *const u32,
}
impl Default for VkDeviceGroupSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO,
      pNext: core::ptr::null(),
      waitSemaphoreCount: Default::default(),
      pWaitSemaphoreDeviceIndices: core::ptr::null(),
      commandBufferCount: Default::default(),
      pCommandBufferDeviceMasks: core::ptr::null(),
      signalSemaphoreCount: Default::default(),
      pSignalSemaphoreDeviceIndices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceGroupSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub modes: VkDeviceGroupPresentModeFlagsKHR,
}
impl Default for VkDeviceGroupSwapchainCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      modes: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceImageMemoryRequirements.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceImageMemoryRequirements {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub pCreateInfo: *const VkImageCreateInfo,
  /// * Optional: true
  pub planeAspect: VkImageAspectFlagBits,
}
impl Default for VkDeviceImageMemoryRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS,
      pNext: core::ptr::null(),
      pCreateInfo: core::ptr::null(),
      planeAspect: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceMemoryOpaqueCaptureAddressInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOpaqueCaptureAddressInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
}
impl Default for VkDeviceMemoryOpaqueCaptureAddressInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO,
      pNext: core::ptr::null(),
      memory: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceMemoryOverallocationCreateInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceMemoryOverallocationCreateInfoAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub overallocationBehavior: VkMemoryOverallocationBehaviorAMD,
}
impl Default for VkDeviceMemoryOverallocationCreateInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD,
      pNext: core::ptr::null(),
      overallocationBehavior: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceMemoryReportCallbackDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportCallbackDataEXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceMemoryReportCallbackDataEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub flags: VkDeviceMemoryReportFlagsEXT,
  pub ty: VkDeviceMemoryReportEventTypeEXT,
  pub memoryObjectId: u64,
  pub size: VkDeviceSize,
  pub objectType: VkObjectType,
  /// * Object Type: objectType
  pub objectHandle: u64,
  pub heapIndex: u32,
}
impl Default for VkDeviceMemoryReportCallbackDataEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT,
      pNext: core::ptr::null_mut(),
      flags: Default::default(),
      ty: Default::default(),
      memoryObjectId: Default::default(),
      size: Default::default(),
      objectType: Default::default(),
      objectHandle: Default::default(),
      heapIndex: Default::default(),
    }
  }
}

/// Khronos: [VkDevicePrivateDataCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDevicePrivateDataCreateInfo.html)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDevicePrivateDataCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub privateDataSlotRequestCount: u32,
}
impl Default for VkDevicePrivateDataCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO,
      pNext: core::ptr::null(),
      privateDataSlotRequestCount: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceQueueCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDeviceQueueCreateFlags,
  pub queueFamilyIndex: u32,
  pub queueCount: u32,
  /// * Len: `queueCount`
  pub pQueuePriorities: *const c_float,
}
impl Default for VkDeviceQueueCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      queueFamilyIndex: Default::default(),
      queueCount: Default::default(),
      pQueuePriorities: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDeviceQueueGlobalPriorityCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoKHR.html)
///
/// * Struct Extends: [`VkDeviceQueueCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub globalPriority: VkQueueGlobalPriorityKHR,
}
impl Default for VkDeviceQueueGlobalPriorityCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      globalPriority: Default::default(),
    }
  }
}

/// Khronos: [VkDeviceQueueInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceQueueInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDeviceQueueCreateFlags,
  pub queueFamilyIndex: u32,
  pub queueIndex: u32,
}
impl Default for VkDeviceQueueInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2,
      pNext: core::ptr::null(),
      flags: Default::default(),
      queueFamilyIndex: Default::default(),
      queueIndex: Default::default(),
    }
  }
}

/// Khronos: [VkDirectDriverLoadingInfoLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingInfoLUNARG.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDirectDriverLoadingInfoLUNARG {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_INFO_LUNARG`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub flags: VkDirectDriverLoadingFlagsLUNARG,
  /// * No Auto-Validity
  pub pfnGetInstanceProcAddr: PFN_vkGetInstanceProcAddrLUNARG,
}
impl Default for VkDirectDriverLoadingInfoLUNARG {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_INFO_LUNARG,
      pNext: core::ptr::null_mut(),
      flags: Default::default(),
      pfnGetInstanceProcAddr: Default::default(),
    }
  }
}

/// Khronos: [VkDirectDriverLoadingListLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingListLUNARG.html)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDirectDriverLoadingListLUNARG {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_LIST_LUNARG`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub mode: VkDirectDriverLoadingModeLUNARG,
  pub driverCount: u32,
  /// * Len: `driverCount`
  pub pDrivers: *const VkDirectDriverLoadingInfoLUNARG,
}
impl Default for VkDirectDriverLoadingListLUNARG {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DIRECT_DRIVER_LOADING_LIST_LUNARG,
      pNext: core::ptr::null_mut(),
      mode: Default::default(),
      driverCount: Default::default(),
      pDrivers: core::ptr::null(),
    }
  }
}

/// Khronos: [VkDirectFBSurfaceCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDirectFBSurfaceCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      dfb: core::ptr::null_mut(),
      surface: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDispatchIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDispatchIndirectCommand.html)
#[derive(Clone, Copy)]
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
    Self {
      x: Default::default(),
      y: Default::default(),
      z: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayEventInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayEventInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub displayEvent: VkDisplayEventTypeEXT,
}
impl Default for VkDisplayEventInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT,
      pNext: core::ptr::null(),
      displayEvent: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayModeCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayModeCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      parameters: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayModeParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeParametersKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayModeParametersKHR {
  /// Visible scanout region.
  pub visibleRegion: VkExtent2D,
  /// Number of times per second the display is updated.
  /// * No Auto-Validity
  pub refreshRate: u32,
}
impl Default for VkDisplayModeParametersKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      visibleRegion: Default::default(),
      refreshRate: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayModeProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeProperties2KHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayModeProperties2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub displayModeProperties: VkDisplayModePropertiesKHR,
}
impl Default for VkDisplayModeProperties2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR,
      pNext: core::ptr::null_mut(),
      displayModeProperties: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayModePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModePropertiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayModePropertiesKHR {
  /// Handle of this display mode.
  pub displayMode: VkDisplayModeKHR,
  /// The parameters this mode uses.
  pub parameters: VkDisplayModeParametersKHR,
}
impl Default for VkDisplayModePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      displayMode: Default::default(),
      parameters: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayNativeHdrSurfaceCapabilitiesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub localDimmingSupport: VkBool32,
}
impl Default for VkDisplayNativeHdrSurfaceCapabilitiesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD,
      pNext: core::ptr::null_mut(),
      localDimmingSupport: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlaneCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilities2KHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlaneCapabilities2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub capabilities: VkDisplayPlaneCapabilitiesKHR,
}
impl Default for VkDisplayPlaneCapabilities2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR,
      pNext: core::ptr::null_mut(),
      capabilities: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlaneCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlaneCapabilitiesKHR {
  /// Types of alpha blending supported, if any.
  /// * Optional: true
  pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
  /// Does the plane have any position and extent restrictions?
  pub minSrcPosition: VkOffset2D,
  pub maxSrcPosition: VkOffset2D,
  pub minSrcExtent: VkExtent2D,
  pub maxSrcExtent: VkExtent2D,
  pub minDstPosition: VkOffset2D,
  pub maxDstPosition: VkOffset2D,
  pub minDstExtent: VkExtent2D,
  pub maxDstExtent: VkExtent2D,
}
impl Default for VkDisplayPlaneCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      supportedAlpha: Default::default(),
      minSrcPosition: Default::default(),
      maxSrcPosition: Default::default(),
      minSrcExtent: Default::default(),
      maxSrcExtent: Default::default(),
      minDstPosition: Default::default(),
      maxDstPosition: Default::default(),
      minDstExtent: Default::default(),
      maxDstExtent: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlaneInfo2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneInfo2KHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlaneInfo2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub mode: VkDisplayModeKHR,
  pub planeIndex: u32,
}
impl Default for VkDisplayPlaneInfo2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR,
      pNext: core::ptr::null(),
      mode: Default::default(),
      planeIndex: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlaneProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneProperties2KHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlaneProperties2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub displayPlaneProperties: VkDisplayPlanePropertiesKHR,
}
impl Default for VkDisplayPlaneProperties2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR,
      pNext: core::ptr::null_mut(),
      displayPlaneProperties: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPlanePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlanePropertiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPlanePropertiesKHR {
  /// Display the plane is currently associated with.  Will be VK_NULL_HANDLE if the plane is not in use.
  pub currentDisplay: VkDisplayKHR,
  /// Current z-order of the plane.
  pub currentStackIndex: u32,
}
impl Default for VkDisplayPlanePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      currentDisplay: Default::default(),
      currentStackIndex: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPowerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPowerInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub powerState: VkDisplayPowerStateEXT,
}
impl Default for VkDisplayPowerInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT,
      pNext: core::ptr::null(),
      powerState: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPresentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPresentInfoKHR.html)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPresentInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Rectangle within the presentable image to read pixel data from when presenting to the display.
  pub srcRect: VkRect2D,
  /// Rectangle within the current display mode's visible region to display srcRectangle in.
  pub dstRect: VkRect2D,
  /// For smart displays, use buffered mode.  If the display properties member "persistentMode" is VK_FALSE, this member must always be VK_FALSE.
  pub persistent: VkBool32,
}
impl Default for VkDisplayPresentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR,
      pNext: core::ptr::null(),
      srcRect: Default::default(),
      dstRect: Default::default(),
      persistent: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayProperties2KHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayProperties2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub displayProperties: VkDisplayPropertiesKHR,
}
impl Default for VkDisplayProperties2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR,
      pNext: core::ptr::null_mut(),
      displayProperties: Default::default(),
    }
  }
}

/// Khronos: [VkDisplayPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPropertiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplayPropertiesKHR {
  /// Handle of the display object
  pub display: VkDisplayKHR,
  /// Name of the display
  /// * Len: `null-terminated`
  pub displayName: *const u8,
  /// In millimeters?
  pub physicalDimensions: VkExtent2D,
  /// Max resolution for CRT?
  pub physicalResolution: VkExtent2D,
  /// one or more bits from VkSurfaceTransformFlagsKHR
  /// * Optional: true
  pub supportedTransforms: VkSurfaceTransformFlagsKHR,
  /// VK_TRUE if the overlay plane's z-order can be changed on this display.
  pub planeReorderPossible: VkBool32,
  /// VK_TRUE if this is a "smart" display that supports self-refresh/internal buffering.
  pub persistentContent: VkBool32,
}
impl Default for VkDisplayPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      display: Default::default(),
      displayName: core::ptr::null(),
      physicalDimensions: Default::default(),
      physicalResolution: Default::default(),
      supportedTransforms: Default::default(),
      planeReorderPossible: Default::default(),
      persistentContent: Default::default(),
    }
  }
}

/// Khronos: [VkDisplaySurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDisplaySurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkDisplaySurfaceCreateFlagsKHR,
  /// The mode to use when displaying this surface
  pub displayMode: VkDisplayModeKHR,
  /// The plane on which this surface appears.  Must be between 0 and the value returned by vkGetPhysicalDeviceDisplayPlanePropertiesKHR() in pPropertyCount.
  pub planeIndex: u32,
  /// The z-order of the plane.
  pub planeStackIndex: u32,
  /// Transform to apply to the images as part of the scanout operation
  pub transform: VkSurfaceTransformFlagBitsKHR,
  /// Global alpha value.  Must be between 0 and 1, inclusive.  Ignored if alphaMode is not VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR
  pub globalAlpha: c_float,
  /// What type of alpha blending to use.  Must be a bit from vkGetDisplayPlanePropertiesKHR::supportedAlpha.
  pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
  /// size of the images to use with this surface
  pub imageExtent: VkExtent2D,
}
impl Default for VkDisplaySurfaceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      displayMode: Default::default(),
      planeIndex: Default::default(),
      planeStackIndex: Default::default(),
      transform: Default::default(),
      globalAlpha: Default::default(),
      alphaMode: Default::default(),
      imageExtent: Default::default(),
    }
  }
}

/// Khronos: [VkDrawIndexedIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawIndexedIndirectCommand.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
  pub indexCount: u32,
  pub instanceCount: u32,
  pub firstIndex: u32,
  pub vertexOffset: i32,
  /// * No Auto-Validity
  pub firstInstance: u32,
}
impl Default for VkDrawIndexedIndirectCommand {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      indexCount: Default::default(),
      instanceCount: Default::default(),
      firstIndex: Default::default(),
      vertexOffset: Default::default(),
      firstInstance: Default::default(),
    }
  }
}

/// Khronos: [VkDrawIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawIndirectCommand.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawIndirectCommand {
  pub vertexCount: u32,
  pub instanceCount: u32,
  pub firstVertex: u32,
  /// * No Auto-Validity
  pub firstInstance: u32,
}
impl Default for VkDrawIndirectCommand {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      vertexCount: Default::default(),
      instanceCount: Default::default(),
      firstVertex: Default::default(),
      firstInstance: Default::default(),
    }
  }
}

/// Khronos: [VkDrawMeshTasksIndirectCommandEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandEXT {
  /// * No Auto-Validity
  pub groupCountX: u32,
  /// * No Auto-Validity
  pub groupCountY: u32,
  /// * No Auto-Validity
  pub groupCountZ: u32,
}
impl Default for VkDrawMeshTasksIndirectCommandEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      groupCountX: Default::default(),
      groupCountY: Default::default(),
      groupCountZ: Default::default(),
    }
  }
}

/// Khronos: [VkDrawMeshTasksIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandNV {
  pub taskCount: u32,
  pub firstTask: u32,
}
impl Default for VkDrawMeshTasksIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      taskCount: Default::default(),
      firstTask: Default::default(),
    }
  }
}

/// Khronos: [VkDrmFormatModifierProperties2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierProperties2EXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrmFormatModifierProperties2EXT {
  pub drmFormatModifier: u64,
  pub drmFormatModifierPlaneCount: u32,
  pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags2,
}
impl Default for VkDrmFormatModifierProperties2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      drmFormatModifier: Default::default(),
      drmFormatModifierPlaneCount: Default::default(),
      drmFormatModifierTilingFeatures: Default::default(),
    }
  }
}

/// Khronos: [VkDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrmFormatModifierPropertiesEXT {
  pub drmFormatModifier: u64,
  pub drmFormatModifierPlaneCount: u32,
  pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags,
}
impl Default for VkDrmFormatModifierPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      drmFormatModifier: Default::default(),
      drmFormatModifierPlaneCount: Default::default(),
      drmFormatModifierTilingFeatures: Default::default(),
    }
  }
}

/// Khronos: [VkDrmFormatModifierPropertiesList2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesList2EXT.html)
///
/// * Struct Extends: [`VkFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrmFormatModifierPropertiesList2EXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  pub drmFormatModifierCount: u32,
  /// * Optional: true
  /// * Len: `drmFormatModifierCount`
  pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierProperties2EXT,
}
impl Default for VkDrmFormatModifierPropertiesList2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT,
      pNext: core::ptr::null_mut(),
      drmFormatModifierCount: Default::default(),
      pDrmFormatModifierProperties: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkDrmFormatModifierPropertiesListEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html)
///
/// * Struct Extends: [`VkFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrmFormatModifierPropertiesListEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  pub drmFormatModifierCount: u32,
  /// * Optional: true
  /// * Len: `drmFormatModifierCount`
  pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierPropertiesEXT,
}
impl Default for VkDrmFormatModifierPropertiesListEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT,
      pNext: core::ptr::null_mut(),
      drmFormatModifierCount: Default::default(),
      pDrmFormatModifierProperties: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkEventCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkEventCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EVENT_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Event creation flags
  /// * Optional: true
  pub flags: VkEventCreateFlags,
}
impl Default for VkEventCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EVENT_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkExportFenceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportFenceCreateInfo.html)
///
/// * Struct Extends: [`VkFenceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportFenceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleTypes: VkExternalFenceHandleTypeFlags,
}
impl Default for VkExportFenceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO,
      pNext: core::ptr::null(),
      handleTypes: Default::default(),
    }
  }
}

/// Khronos: [VkExportFenceWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html)
///
/// * Struct Extends: [`VkFenceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportFenceWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub pAttributes: *const SECURITY_ATTRIBUTES,
  pub dwAccess: DWORD,
  pub name: LPCWSTR,
}
impl Default for VkExportFenceWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
      pNext: core::ptr::null(),
      pAttributes: core::ptr::null(),
      dwAccess: Default::default(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMemoryAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfo.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMemoryAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlags,
}
impl Default for VkExportMemoryAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO,
      pNext: core::ptr::null(),
      handleTypes: Default::default(),
    }
  }
}

/// Khronos: [VkExportMemoryAllocateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfoNV.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMemoryAllocateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}
impl Default for VkExportMemoryAllocateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV,
      pNext: core::ptr::null(),
      handleTypes: Default::default(),
    }
  }
}

/// Khronos: [VkExportMemoryWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoKHR.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub pAttributes: *const SECURITY_ATTRIBUTES,
  pub dwAccess: DWORD,
  pub name: LPCWSTR,
}
impl Default for VkExportMemoryWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
      pNext: core::ptr::null(),
      pAttributes: core::ptr::null(),
      dwAccess: Default::default(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMemoryWin32HandleInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoNV.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub pAttributes: *const SECURITY_ATTRIBUTES,
  /// * Optional: true
  pub dwAccess: DWORD,
}
impl Default for VkExportMemoryWin32HandleInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
      pNext: core::ptr::null(),
      pAttributes: core::ptr::null(),
      dwAccess: Default::default(),
    }
  }
}

/// Khronos: [VkExportMetalBufferInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalBufferInfoEXT.html)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalBufferInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub mtlBuffer: MTLBuffer_id,
}
impl Default for VkExportMetalBufferInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT,
      pNext: core::ptr::null(),
      memory: Default::default(),
      mtlBuffer: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalCommandQueueInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalCommandQueueInfoEXT.html)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalCommandQueueInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub queue: VkQueue,
  pub mtlCommandQueue: MTLCommandQueue_id,
}
impl Default for VkExportMetalCommandQueueInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT,
      pNext: core::ptr::null(),
      queue: Default::default(),
      mtlCommandQueue: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalDeviceInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalDeviceInfoEXT.html)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalDeviceInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub mtlDevice: MTLDevice_id,
}
impl Default for VkExportMetalDeviceInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT,
      pNext: core::ptr::null(),
      mtlDevice: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalIOSurfaceInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalIOSurfaceInfoEXT.html)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalIOSurfaceInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub image: VkImage,
  pub ioSurface: IOSurfaceRef,
}
impl Default for VkExportMetalIOSurfaceInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT,
      pNext: core::ptr::null(),
      image: Default::default(),
      ioSurface: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalObjectCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectCreateInfoEXT.html)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
/// * Struct Extends: [`VkMemoryAllocateInfo`]
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkImageViewCreateInfo`]
/// * Struct Extends: [`VkBufferViewCreateInfo`]
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
/// * Struct Extends: [`VkEventCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalObjectCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub exportObjectType: VkExportMetalObjectTypeFlagBitsEXT,
}
impl Default for VkExportMetalObjectCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      exportObjectType: Default::default(),
    }
  }
}

/// Khronos: [VkExportMetalObjectsInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectsInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalObjectsInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
}
impl Default for VkExportMetalObjectsInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT,
      pNext: core::ptr::null(),
    }
  }
}

/// Khronos: [VkExportMetalSharedEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalSharedEventInfoEXT.html)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalSharedEventInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  pub event: VkEvent,
  pub mtlSharedEvent: MTLSharedEvent_id,
}
impl Default for VkExportMetalSharedEventInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      event: Default::default(),
      mtlSharedEvent: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportMetalTextureInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalTextureInfoEXT.html)
///
/// * Struct Extends: [`VkExportMetalObjectsInfoEXT`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalTextureInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub image: VkImage,
  /// * Optional: true
  pub imageView: VkImageView,
  /// * Optional: true
  pub bufferView: VkBufferView,
  pub plane: VkImageAspectFlagBits,
  pub mtlTexture: MTLTexture_id,
}
impl Default for VkExportMetalTextureInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT,
      pNext: core::ptr::null(),
      image: Default::default(),
      imageView: Default::default(),
      bufferView: Default::default(),
      plane: Default::default(),
      mtlTexture: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExportSemaphoreCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreCreateInfo.html)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportSemaphoreCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleTypes: VkExternalSemaphoreHandleTypeFlags,
}
impl Default for VkExportSemaphoreCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO,
      pNext: core::ptr::null(),
      handleTypes: Default::default(),
    }
  }
}

/// Khronos: [VkExportSemaphoreWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreWin32HandleInfoKHR.html)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub pAttributes: *const SECURITY_ATTRIBUTES,
  pub dwAccess: DWORD,
  pub name: LPCWSTR,
}
impl Default for VkExportSemaphoreWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
      pNext: core::ptr::null(),
      pAttributes: core::ptr::null(),
      dwAccess: Default::default(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtensionProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtensionProperties {
  /// extension name
  pub extensionName: [u8; VK_MAX_EXTENSION_NAME_SIZE],
  /// version of the extension specification implemented
  pub specVersion: u32,
}
impl Default for VkExtensionProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      extensionName: [Default::default(); VK_MAX_EXTENSION_NAME_SIZE],
      specVersion: Default::default(),
    }
  }
}

/// Khronos: [VkExtent2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtent2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtent2D {
  pub width: u32,
  pub height: u32,
}
impl Default for VkExtent2D {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      width: Default::default(),
      height: Default::default(),
    }
  }
}

/// Khronos: [VkExtent3D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtent3D.html)
#[derive(Clone, Copy)]
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

/// Khronos: [VkExternalBufferProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalBufferProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalBufferProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub externalMemoryProperties: VkExternalMemoryProperties,
}
impl Default for VkExternalBufferProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES,
      pNext: core::ptr::null_mut(),
      externalMemoryProperties: Default::default(),
    }
  }
}

/// Khronos: [VkExternalFenceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFenceProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalFenceProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags,
  pub compatibleHandleTypes: VkExternalFenceHandleTypeFlags,
  /// * Optional: true
  pub externalFenceFeatures: VkExternalFenceFeatureFlags,
}
impl Default for VkExternalFenceProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES,
      pNext: core::ptr::null_mut(),
      exportFromImportedHandleTypes: Default::default(),
      compatibleHandleTypes: Default::default(),
      externalFenceFeatures: Default::default(),
    }
  }
}

/// Khronos: [VkExternalFormatANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalFormatANDROID.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkSamplerYcbcrConversionCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalFormatANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub externalFormat: u64,
}
impl Default for VkExternalFormatANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID,
      pNext: core::ptr::null_mut(),
      externalFormat: Default::default(),
    }
  }
}

/// Khronos: [VkExternalImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatProperties.html)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalImageFormatProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub externalMemoryProperties: VkExternalMemoryProperties,
}
impl Default for VkExternalImageFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES,
      pNext: core::ptr::null_mut(),
      externalMemoryProperties: Default::default(),
    }
  }
}

/// Khronos: [VkExternalImageFormatPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatPropertiesNV.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalImageFormatPropertiesNV {
  pub imageFormatProperties: VkImageFormatProperties,
  /// * Optional: true
  pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
  /// * Optional: true
  pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
  /// * Optional: true
  pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
}
impl Default for VkExternalImageFormatPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      imageFormatProperties: Default::default(),
      externalMemoryFeatures: Default::default(),
      exportFromImportedHandleTypes: Default::default(),
      compatibleHandleTypes: Default::default(),
    }
  }
}

/// Khronos: [VkExternalMemoryBufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryBufferCreateInfo.html)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalMemoryBufferCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlags,
}
impl Default for VkExternalMemoryBufferCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO,
      pNext: core::ptr::null(),
      handleTypes: Default::default(),
    }
  }
}

/// Khronos: [VkExternalMemoryImageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfo.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalMemoryImageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlags,
}
impl Default for VkExternalMemoryImageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO,
      pNext: core::ptr::null(),
      handleTypes: Default::default(),
    }
  }
}

/// Khronos: [VkExternalMemoryImageCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalMemoryImageCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}
impl Default for VkExternalMemoryImageCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      handleTypes: Default::default(),
    }
  }
}

/// Khronos: [VkExternalMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalMemoryProperties {
  pub externalMemoryFeatures: VkExternalMemoryFeatureFlags,
  /// * Optional: true
  pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags,
  pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlags,
}
impl Default for VkExternalMemoryProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      externalMemoryFeatures: Default::default(),
      exportFromImportedHandleTypes: Default::default(),
      compatibleHandleTypes: Default::default(),
    }
  }
}

/// Khronos: [VkExternalSemaphoreProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExternalSemaphoreProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags,
  pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags,
  /// * Optional: true
  pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags,
}
impl Default for VkExternalSemaphoreProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES,
      pNext: core::ptr::null_mut(),
      exportFromImportedHandleTypes: Default::default(),
      compatibleHandleTypes: Default::default(),
      externalSemaphoreFeatures: Default::default(),
    }
  }
}

/// Khronos: [VkFenceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFenceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Fence creation flags
  /// * Optional: true
  pub flags: VkFenceCreateFlags,
}
impl Default for VkFenceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkFenceGetFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceGetFdInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFenceGetFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkFenceGetFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR,
      pNext: core::ptr::null(),
      fence: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkFenceGetSciSyncInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceGetSciSyncInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFenceGetSciSyncInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FENCE_GET_SCI_SYNC_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkFenceGetSciSyncInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FENCE_GET_SCI_SYNC_INFO_NV,
      pNext: core::ptr::null(),
      fence: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkFenceGetWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFenceGetWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkFenceGetWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR,
      pNext: core::ptr::null(),
      fence: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkFilterCubicImageViewImageFormatPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFilterCubicImageViewImageFormatPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// The combinations of format, image type (and image view type if provided) can be filtered with VK_FILTER_CUBIC_EXT
  pub filterCubic: VkBool32,
  /// The combination of format, image type (and image view type if provided) can be filtered with VK_FILTER_CUBIC_EXT and ReductionMode of Min or Max
  pub filterCubicMinmax: VkBool32,
}
impl Default for VkFilterCubicImageViewImageFormatPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      filterCubic: Default::default(),
      filterCubicMinmax: Default::default(),
    }
  }
}

/// Khronos: [VkFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFormatProperties {
  /// Format features in case of linear tiling
  /// * Optional: true
  /// * Limit Type: bitmask
  pub linearTilingFeatures: VkFormatFeatureFlags,
  /// Format features in case of optimal tiling
  /// * Optional: true
  /// * Limit Type: bitmask
  pub optimalTilingFeatures: VkFormatFeatureFlags,
  /// Format features supported by buffers
  /// * Optional: true
  /// * Limit Type: bitmask
  pub bufferFeatures: VkFormatFeatureFlags,
}
impl Default for VkFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      linearTilingFeatures: Default::default(),
      optimalTilingFeatures: Default::default(),
      bufferFeatures: Default::default(),
    }
  }
}

/// Khronos: [VkFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatProperties2.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFormatProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub formatProperties: VkFormatProperties,
}
impl Default for VkFormatProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2,
      pNext: core::ptr::null_mut(),
      formatProperties: Default::default(),
    }
  }
}

/// Khronos: [VkFormatProperties3](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatProperties3.html)
///
/// * Struct Extends: [`VkFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFormatProperties3 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub linearTilingFeatures: VkFormatFeatureFlags2,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub optimalTilingFeatures: VkFormatFeatureFlags2,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub bufferFeatures: VkFormatFeatureFlags2,
}
impl Default for VkFormatProperties3 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3,
      pNext: core::ptr::null_mut(),
      linearTilingFeatures: Default::default(),
      optimalTilingFeatures: Default::default(),
      bufferFeatures: Default::default(),
    }
  }
}

/// Khronos: [VkFragmentShadingRateAttachmentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateAttachmentInfoKHR.html)
///
/// * Struct Extends: [`VkSubpassDescription2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFragmentShadingRateAttachmentInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub pFragmentShadingRateAttachment: *const VkAttachmentReference2,
  pub shadingRateAttachmentTexelSize: VkExtent2D,
}
impl Default for VkFragmentShadingRateAttachmentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
      pNext: core::ptr::null(),
      pFragmentShadingRateAttachment: core::ptr::null(),
      shadingRateAttachmentTexelSize: Default::default(),
    }
  }
}

/// Khronos: [VkFramebufferAttachmentImageInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentImageInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFramebufferAttachmentImageInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Image creation flags
  /// * Optional: true
  pub flags: VkImageCreateFlags,
  /// Image usage flags
  pub usage: VkImageUsageFlags,
  pub width: u32,
  pub height: u32,
  pub layerCount: u32,
  /// * Optional: true
  pub viewFormatCount: u32,
  /// * Len: `viewFormatCount`
  pub pViewFormats: *const VkFormat,
}
impl Default for VkFramebufferAttachmentImageInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      usage: Default::default(),
      width: Default::default(),
      height: Default::default(),
      layerCount: Default::default(),
      viewFormatCount: Default::default(),
      pViewFormats: core::ptr::null(),
    }
  }
}

/// Khronos: [VkFramebufferAttachmentsCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentsCreateInfo.html)
///
/// * Struct Extends: [`VkFramebufferCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFramebufferAttachmentsCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub attachmentImageInfoCount: u32,
  /// * Len: `attachmentImageInfoCount`
  pub pAttachmentImageInfos: *const VkFramebufferAttachmentImageInfo,
}
impl Default for VkFramebufferAttachmentsCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO,
      pNext: core::ptr::null(),
      attachmentImageInfoCount: Default::default(),
      pAttachmentImageInfos: core::ptr::null(),
    }
  }
}

/// Khronos: [VkFramebufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFramebufferCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkFramebufferCreateFlags,
  pub renderPass: VkRenderPass,
  /// * Optional: true
  pub attachmentCount: u32,
  /// * Len: `attachmentCount`
  /// * No Auto-Validity
  pub pAttachments: *const VkImageView,
  pub width: u32,
  pub height: u32,
  pub layers: u32,
}
impl Default for VkFramebufferCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      renderPass: Default::default(),
      attachmentCount: Default::default(),
      pAttachments: core::ptr::null(),
      width: Default::default(),
      height: Default::default(),
      layers: Default::default(),
    }
  }
}

/// Khronos: [VkFramebufferMixedSamplesCombinationNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferMixedSamplesCombinationNV.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFramebufferMixedSamplesCombinationNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub coverageReductionMode: VkCoverageReductionModeNV,
  pub rasterizationSamples: VkSampleCountFlagBits,
  pub depthStencilSamples: VkSampleCountFlags,
  pub colorSamples: VkSampleCountFlags,
}
impl Default for VkFramebufferMixedSamplesCombinationNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV,
      pNext: core::ptr::null_mut(),
      coverageReductionMode: Default::default(),
      rasterizationSamples: Default::default(),
      depthStencilSamples: Default::default(),
      colorSamples: Default::default(),
    }
  }
}

/// Khronos: [VkGeneratedCommandsInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeneratedCommandsInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub pipeline: VkPipeline,
  pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
  pub streamCount: u32,
  /// * Len: `streamCount`
  pub pStreams: *const VkIndirectCommandsStreamNV,
  pub sequencesCount: u32,
  pub preprocessBuffer: VkBuffer,
  pub preprocessOffset: VkDeviceSize,
  pub preprocessSize: VkDeviceSize,
  /// * Optional: true
  pub sequencesCountBuffer: VkBuffer,
  pub sequencesCountOffset: VkDeviceSize,
  /// * Optional: true
  pub sequencesIndexBuffer: VkBuffer,
  pub sequencesIndexOffset: VkDeviceSize,
}
impl Default for VkGeneratedCommandsInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV,
      pNext: core::ptr::null(),
      pipelineBindPoint: Default::default(),
      pipeline: Default::default(),
      indirectCommandsLayout: Default::default(),
      streamCount: Default::default(),
      pStreams: core::ptr::null(),
      sequencesCount: Default::default(),
      preprocessBuffer: Default::default(),
      preprocessOffset: Default::default(),
      preprocessSize: Default::default(),
      sequencesCountBuffer: Default::default(),
      sequencesCountOffset: Default::default(),
      sequencesIndexBuffer: Default::default(),
      sequencesIndexOffset: Default::default(),
    }
  }
}

/// Khronos: [VkGeneratedCommandsMemoryRequirementsInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub pipeline: VkPipeline,
  pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
  pub maxSequencesCount: u32,
}
impl Default for VkGeneratedCommandsMemoryRequirementsInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV,
      pNext: core::ptr::null(),
      pipelineBindPoint: Default::default(),
      pipeline: Default::default(),
      indirectCommandsLayout: Default::default(),
      maxSequencesCount: Default::default(),
    }
  }
}

/// Khronos: [VkGeometryAABBNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryAABBNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeometryAABBNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub aabbData: VkBuffer,
  pub numAABBs: u32,
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
      sType: VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV,
      pNext: core::ptr::null(),
      aabbData: Default::default(),
      numAABBs: Default::default(),
      stride: Default::default(),
      offset: Default::default(),
    }
  }
}

/// Khronos: [VkGeometryDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryDataNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeometryDataNV {
  pub triangles: VkGeometryTrianglesNV,
  pub aabbs: VkGeometryAABBNV,
}
impl Default for VkGeometryDataNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      triangles: Default::default(),
      aabbs: Default::default(),
    }
  }
}

/// Khronos: [VkGeometryNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeometryNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GEOMETRY_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub geometryType: VkGeometryTypeKHR,
  pub geometry: VkGeometryDataNV,
  /// * Optional: true
  pub flags: VkGeometryFlagsKHR,
}
impl Default for VkGeometryNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_GEOMETRY_NV,
      pNext: core::ptr::null(),
      geometryType: Default::default(),
      geometry: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkGeometryTrianglesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryTrianglesNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeometryTrianglesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub vertexData: VkBuffer,
  pub vertexOffset: VkDeviceSize,
  pub vertexCount: u32,
  pub vertexStride: VkDeviceSize,
  pub vertexFormat: VkFormat,
  /// * Optional: true
  pub indexData: VkBuffer,
  pub indexOffset: VkDeviceSize,
  pub indexCount: u32,
  pub indexType: VkIndexType,
  /// Optional reference to array of floats representing a 3x4 row major affine transformation matrix.
  /// * Optional: true
  pub transformData: VkBuffer,
  pub transformOffset: VkDeviceSize,
}
impl Default for VkGeometryTrianglesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV,
      pNext: core::ptr::null(),
      vertexData: Default::default(),
      vertexOffset: Default::default(),
      vertexCount: Default::default(),
      vertexStride: Default::default(),
      vertexFormat: Default::default(),
      indexData: Default::default(),
      indexOffset: Default::default(),
      indexCount: Default::default(),
      indexType: Default::default(),
      transformData: Default::default(),
      transformOffset: Default::default(),
    }
  }
}

/// Khronos: [VkGraphicsPipelineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Pipeline creation flags
  /// * Optional: true
  pub flags: VkPipelineCreateFlags,
  /// * Optional: true
  /// * No Auto-Validity
  pub stageCount: u32,
  /// One entry for each active shader stage
  /// * Optional: true
  /// * Len: `stageCount`
  /// * No Auto-Validity
  pub pStages: *const VkPipelineShaderStageCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pViewportState: *const VkPipelineViewportStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
  /// * Optional: true
  pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
  /// Interface layout of the pipeline
  /// * Optional: true
  /// * No Auto-Validity
  pub layout: VkPipelineLayout,
  /// * Optional: true
  /// * No Auto-Validity
  pub renderPass: VkRenderPass,
  /// * No Auto-Validity
  pub subpass: u32,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it specifies the handle of the base pipeline this is a derivative of
  /// * Optional: true
  /// * No Auto-Validity
  pub basePipelineHandle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it specifies an index into pCreateInfos of the base pipeline this is a derivative of
  pub basePipelineIndex: i32,
}
impl Default for VkGraphicsPipelineCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      stageCount: Default::default(),
      pStages: core::ptr::null(),
      pVertexInputState: core::ptr::null(),
      pInputAssemblyState: core::ptr::null(),
      pTessellationState: core::ptr::null(),
      pViewportState: core::ptr::null(),
      pRasterizationState: core::ptr::null(),
      pMultisampleState: core::ptr::null(),
      pDepthStencilState: core::ptr::null(),
      pColorBlendState: core::ptr::null(),
      pDynamicState: core::ptr::null(),
      layout: Default::default(),
      renderPass: Default::default(),
      subpass: Default::default(),
      basePipelineHandle: Default::default(),
      basePipelineIndex: Default::default(),
    }
  }
}

/// Khronos: [VkGraphicsPipelineLibraryCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryCreateInfoEXT.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGraphicsPipelineLibraryCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub flags: VkGraphicsPipelineLibraryFlagsEXT,
}
impl Default for VkGraphicsPipelineLibraryCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT,
      pNext: core::ptr::null_mut(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkGraphicsPipelineShaderGroupsCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub groupCount: u32,
  /// * Len: `groupCount`
  pub pGroups: *const VkGraphicsShaderGroupCreateInfoNV,
  /// * Optional: true
  pub pipelineCount: u32,
  /// * Len: `pipelineCount`
  pub pPipelines: *const VkPipeline,
}
impl Default for VkGraphicsPipelineShaderGroupsCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      groupCount: Default::default(),
      pGroups: core::ptr::null(),
      pipelineCount: Default::default(),
      pPipelines: core::ptr::null(),
    }
  }
}

/// Khronos: [VkGraphicsShaderGroupCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGraphicsShaderGroupCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub stageCount: u32,
  /// * Len: `stageCount`
  pub pStages: *const VkPipelineShaderStageCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
  /// * Optional: true
  /// * No Auto-Validity
  pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
}
impl Default for VkGraphicsShaderGroupCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      stageCount: Default::default(),
      pStages: core::ptr::null(),
      pVertexInputState: core::ptr::null(),
      pTessellationState: core::ptr::null(),
    }
  }
}

/// Khronos: [VkHdrMetadataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHdrMetadataEXT.html)
///
/// From CTA 861.3
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkHdrMetadataEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_HDR_METADATA_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Display primary's Red
  /// * No Auto-Validity
  pub displayPrimaryRed: VkXYColorEXT,
  /// Display primary's Green
  /// * No Auto-Validity
  pub displayPrimaryGreen: VkXYColorEXT,
  /// Display primary's Blue
  /// * No Auto-Validity
  pub displayPrimaryBlue: VkXYColorEXT,
  /// Display primary's Blue
  /// * No Auto-Validity
  pub whitePoint: VkXYColorEXT,
  /// Display maximum luminance
  /// * No Auto-Validity
  pub maxLuminance: c_float,
  /// Display minimum luminance
  /// * No Auto-Validity
  pub minLuminance: c_float,
  /// Content maximum luminance
  /// * No Auto-Validity
  pub maxContentLightLevel: c_float,
  /// * No Auto-Validity
  pub maxFrameAverageLightLevel: c_float,
}
impl Default for VkHdrMetadataEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_HDR_METADATA_EXT,
      pNext: core::ptr::null(),
      displayPrimaryRed: Default::default(),
      displayPrimaryGreen: Default::default(),
      displayPrimaryBlue: Default::default(),
      whitePoint: Default::default(),
      maxLuminance: Default::default(),
      minLuminance: Default::default(),
      maxContentLightLevel: Default::default(),
      maxFrameAverageLightLevel: Default::default(),
    }
  }
}

/// Khronos: [VkHeadlessSurfaceCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkHeadlessSurfaceCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkHeadlessSurfaceCreateFlagsEXT,
}
impl Default for VkHeadlessSurfaceCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkIOSSurfaceCreateInfoMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIOSSurfaceCreateInfoMVK {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkIOSSurfaceCreateFlagsMVK,
  /// * No Auto-Validity
  pub pView: *const c_void,
}
impl Default for VkIOSSurfaceCreateInfoMVK {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pView: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageBlit](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageBlit.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageBlit {
  pub srcSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub srcOffsets: [VkOffset3D; 2],
  pub dstSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub dstOffsets: [VkOffset3D; 2],
}
impl Default for VkImageBlit {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      srcSubresource: Default::default(),
      srcOffsets: [Default::default(); 2],
      dstSubresource: Default::default(),
      dstOffsets: [Default::default(); 2],
    }
  }
}

/// Khronos: [VkImageBlit2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageBlit2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageBlit2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_BLIT_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub srcOffsets: [VkOffset3D; 2],
  pub dstSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub dstOffsets: [VkOffset3D; 2],
}
impl Default for VkImageBlit2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_BLIT_2,
      pNext: core::ptr::null(),
      srcSubresource: Default::default(),
      srcOffsets: [Default::default(); 2],
      dstSubresource: Default::default(),
      dstOffsets: [Default::default(); 2],
    }
  }
}

/// Khronos: [VkImageCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCaptureDescriptorDataInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCaptureDescriptorDataInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub image: VkImage,
}
impl Default for VkImageCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      pNext: core::ptr::null(),
      image: Default::default(),
    }
  }
}

/// Khronos: [VkImageCompressionControlEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionControlEXT.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCompressionControlEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_CONTROL_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * No Auto-Validity
  pub flags: VkImageCompressionFlagsEXT,
  /// * Optional: true
  pub compressionControlPlaneCount: u32,
  /// * Len: `compressionControlPlaneCount`
  /// * No Auto-Validity
  pub pFixedRateFlags: *mut VkImageCompressionFixedRateFlagsEXT,
}
impl Default for VkImageCompressionControlEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_CONTROL_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      compressionControlPlaneCount: Default::default(),
      pFixedRateFlags: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImageCompressionPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCompressionPropertiesEXT.html)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Struct Extends: [`VkSurfaceFormat2KHR`]
/// * Struct Extends: [`VkSubresourceLayout2EXT`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCompressionPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub imageCompressionFlags: VkImageCompressionFlagsEXT,
  pub imageCompressionFixedRateFlags: VkImageCompressionFixedRateFlagsEXT,
}
impl Default for VkImageCompressionPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      imageCompressionFlags: Default::default(),
      imageCompressionFixedRateFlags: Default::default(),
    }
  }
}

/// Khronos: [VkImageConstraintsInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageConstraintsInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub formatConstraintsCount: u32,
  /// * Len: `formatConstraintsCount`
  pub pFormatConstraints: *const VkImageFormatConstraintsInfoFUCHSIA,
  pub bufferCollectionConstraints: VkBufferCollectionConstraintsInfoFUCHSIA,
  /// * Optional: true
  pub flags: VkImageConstraintsInfoFlagsFUCHSIA,
}
impl Default for VkImageConstraintsInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      formatConstraintsCount: Default::default(),
      pFormatConstraints: core::ptr::null(),
      bufferCollectionConstraints: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkImageCopy](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCopy.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCopy {
  pub srcSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub dstOffset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub extent: VkExtent3D,
}
impl Default for VkImageCopy {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      srcSubresource: Default::default(),
      srcOffset: Default::default(),
      dstSubresource: Default::default(),
      dstOffset: Default::default(),
      extent: Default::default(),
    }
  }
}

/// Khronos: [VkImageCopy2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCopy2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCopy2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_COPY_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub dstOffset: VkOffset3D,
  /// Specified in pixels for both compressed and uncompressed images
  pub extent: VkExtent3D,
}
impl Default for VkImageCopy2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_COPY_2,
      pNext: core::ptr::null(),
      srcSubresource: Default::default(),
      srcOffset: Default::default(),
      dstSubresource: Default::default(),
      dstOffset: Default::default(),
      extent: Default::default(),
    }
  }
}

/// Khronos: [VkImageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Image creation flags
  /// * Optional: true
  pub flags: VkImageCreateFlags,
  pub imageType: VkImageType,
  pub format: VkFormat,
  pub extent: VkExtent3D,
  pub mipLevels: u32,
  pub arrayLayers: u32,
  pub samples: VkSampleCountFlagBits,
  pub tiling: VkImageTiling,
  /// Image usage flags
  pub usage: VkImageUsageFlags,
  /// Cross-queue-family sharing mode
  pub sharingMode: VkSharingMode,
  /// Number of queue families to share across
  /// * Optional: true
  pub queueFamilyIndexCount: u32,
  /// Array of queue family indices to share across
  /// * Len: `queueFamilyIndexCount`
  /// * No Auto-Validity
  pub pQueueFamilyIndices: *const u32,
  /// Initial image layout for all subresources
  pub initialLayout: VkImageLayout,
}
impl Default for VkImageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      imageType: Default::default(),
      format: Default::default(),
      extent: Default::default(),
      mipLevels: Default::default(),
      arrayLayers: Default::default(),
      samples: Default::default(),
      tiling: Default::default(),
      usage: Default::default(),
      sharingMode: Default::default(),
      queueFamilyIndexCount: Default::default(),
      pQueueFamilyIndices: core::ptr::null(),
      initialLayout: Default::default(),
    }
  }
}

/// Khronos: [VkImageDrmFormatModifierExplicitCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub drmFormatModifier: u64,
  /// * Optional: false
  pub drmFormatModifierPlaneCount: u32,
  /// * Len: `drmFormatModifierPlaneCount`
  pub pPlaneLayouts: *const VkSubresourceLayout,
}
impl Default for VkImageDrmFormatModifierExplicitCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      drmFormatModifier: Default::default(),
      drmFormatModifierPlaneCount: Default::default(),
      pPlaneLayouts: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageDrmFormatModifierListCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageDrmFormatModifierListCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub drmFormatModifierCount: u32,
  /// * Len: `drmFormatModifierCount`
  pub pDrmFormatModifiers: *const u64,
}
impl Default for VkImageDrmFormatModifierListCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      drmFormatModifierCount: Default::default(),
      pDrmFormatModifiers: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageDrmFormatModifierPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub drmFormatModifier: u64,
}
impl Default for VkImageDrmFormatModifierPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      drmFormatModifier: Default::default(),
    }
  }
}

/// Khronos: [VkImageFormatConstraintsInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsInfoFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatConstraintsInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub imageCreateInfo: VkImageCreateInfo,
  pub requiredFormatFeatures: VkFormatFeatureFlags,
  /// * Optional: true
  pub flags: VkImageFormatConstraintsFlagsFUCHSIA,
  /// * Optional: true
  pub sysmemPixelFormat: u64,
  pub colorSpaceCount: u32,
  /// * Len: `colorSpaceCount`
  pub pColorSpaces: *const VkSysmemColorSpaceFUCHSIA,
}
impl Default for VkImageFormatConstraintsInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      imageCreateInfo: Default::default(),
      requiredFormatFeatures: Default::default(),
      flags: Default::default(),
      sysmemPixelFormat: Default::default(),
      colorSpaceCount: Default::default(),
      pColorSpaces: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageFormatListCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatListCreateInfo.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatListCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub viewFormatCount: u32,
  /// * Len: `viewFormatCount`
  pub pViewFormats: *const VkFormat,
}
impl Default for VkImageFormatListCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO,
      pNext: core::ptr::null(),
      viewFormatCount: Default::default(),
      pViewFormats: core::ptr::null(),
    }
  }
}

/// Khronos: [VkImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatProperties {
  /// max image dimensions for this resource type
  pub maxExtent: VkExtent3D,
  /// max number of mipmap levels for this resource type
  pub maxMipLevels: u32,
  /// max array size for this resource type
  pub maxArrayLayers: u32,
  /// supported sample counts for this resource type
  /// * Optional: true
  pub sampleCounts: VkSampleCountFlags,
  /// max size (in bytes) of this resource type
  pub maxResourceSize: VkDeviceSize,
}
impl Default for VkImageFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      maxExtent: Default::default(),
      maxMipLevels: Default::default(),
      maxArrayLayers: Default::default(),
      sampleCounts: Default::default(),
      maxResourceSize: Default::default(),
    }
  }
}

/// Khronos: [VkImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties2.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub imageFormatProperties: VkImageFormatProperties,
}
impl Default for VkImageFormatProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2,
      pNext: core::ptr::null_mut(),
      imageFormatProperties: Default::default(),
    }
  }
}

/// Khronos: [VkImageMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageMemoryBarrier {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * No Auto-Validity
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * No Auto-Validity
  pub dstAccessMask: VkAccessFlags,
  /// Current layout of the image
  pub oldLayout: VkImageLayout,
  /// New layout to transition the image to
  pub newLayout: VkImageLayout,
  /// Queue family to transition ownership from
  pub srcQueueFamilyIndex: u32,
  /// Queue family to transition ownership to
  pub dstQueueFamilyIndex: u32,
  /// Image to sync
  pub image: VkImage,
  /// Subresource range to sync
  pub subresourceRange: VkImageSubresourceRange,
}
impl Default for VkImageMemoryBarrier {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
      pNext: core::ptr::null(),
      srcAccessMask: Default::default(),
      dstAccessMask: Default::default(),
      oldLayout: Default::default(),
      newLayout: Default::default(),
      srcQueueFamilyIndex: Default::default(),
      dstQueueFamilyIndex: Default::default(),
      image: Default::default(),
      subresourceRange: Default::default(),
    }
  }
}

/// Khronos: [VkImageMemoryBarrier2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageMemoryBarrier2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub srcStageMask: VkPipelineStageFlags2,
  /// * Optional: true
  pub srcAccessMask: VkAccessFlags2,
  /// * Optional: true
  pub dstStageMask: VkPipelineStageFlags2,
  /// * Optional: true
  pub dstAccessMask: VkAccessFlags2,
  pub oldLayout: VkImageLayout,
  pub newLayout: VkImageLayout,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub image: VkImage,
  pub subresourceRange: VkImageSubresourceRange,
}
impl Default for VkImageMemoryBarrier2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2,
      pNext: core::ptr::null(),
      srcStageMask: Default::default(),
      srcAccessMask: Default::default(),
      dstStageMask: Default::default(),
      dstAccessMask: Default::default(),
      oldLayout: Default::default(),
      newLayout: Default::default(),
      srcQueueFamilyIndex: Default::default(),
      dstQueueFamilyIndex: Default::default(),
      image: Default::default(),
      subresourceRange: Default::default(),
    }
  }
}

/// Khronos: [VkImageMemoryRequirementsInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageMemoryRequirementsInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageMemoryRequirementsInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub image: VkImage,
}
impl Default for VkImageMemoryRequirementsInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2,
      pNext: core::ptr::null(),
      image: Default::default(),
    }
  }
}

/// Khronos: [VkImagePipeSurfaceCreateInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkImagePipeSurfaceCreateFlagsFUCHSIA,
  pub imagePipeHandle: zx_handle_t,
}
impl Default for VkImagePipeSurfaceCreateInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      flags: Default::default(),
      imagePipeHandle: Default::default(),
    }
  }
}

/// Khronos: [VkImagePlaneMemoryRequirementsInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html)
///
/// * Struct Extends: [`VkImageMemoryRequirementsInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImagePlaneMemoryRequirementsInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub planeAspect: VkImageAspectFlagBits,
}
impl Default for VkImagePlaneMemoryRequirementsInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO,
      pNext: core::ptr::null(),
      planeAspect: Default::default(),
    }
  }
}

/// Khronos: [VkImageResolve](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageResolve.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageResolve {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
}
impl Default for VkImageResolve {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      srcSubresource: Default::default(),
      srcOffset: Default::default(),
      dstSubresource: Default::default(),
      dstOffset: Default::default(),
      extent: Default::default(),
    }
  }
}

/// Khronos: [VkImageResolve2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageResolve2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageResolve2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
}
impl Default for VkImageResolve2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2,
      pNext: core::ptr::null(),
      srcSubresource: Default::default(),
      srcOffset: Default::default(),
      dstSubresource: Default::default(),
      dstOffset: Default::default(),
      extent: Default::default(),
    }
  }
}

/// Khronos: [VkImageSparseMemoryRequirementsInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSparseMemoryRequirementsInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub image: VkImage,
}
impl Default for VkImageSparseMemoryRequirementsInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2,
      pNext: core::ptr::null(),
      image: Default::default(),
    }
  }
}

/// Khronos: [VkImageStencilUsageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageStencilUsageCreateInfo.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageStencilUsageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub stencilUsage: VkImageUsageFlags,
}
impl Default for VkImageStencilUsageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO,
      pNext: core::ptr::null(),
      stencilUsage: Default::default(),
    }
  }
}

/// Khronos: [VkImageSubresource](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresource.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresource {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub arrayLayer: u32,
}
impl Default for VkImageSubresource {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      aspectMask: Default::default(),
      mipLevel: Default::default(),
      arrayLayer: Default::default(),
    }
  }
}

/// Khronos: [VkImageSubresource2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresource2EXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresource2EXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub imageSubresource: VkImageSubresource,
}
impl Default for VkImageSubresource2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT,
      pNext: core::ptr::null_mut(),
      imageSubresource: Default::default(),
    }
  }
}

/// Khronos: [VkImageSubresourceLayers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceLayers.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresourceLayers {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
impl Default for VkImageSubresourceLayers {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      aspectMask: Default::default(),
      mipLevel: Default::default(),
      baseArrayLayer: Default::default(),
      layerCount: Default::default(),
    }
  }
}

/// Khronos: [VkImageSubresourceRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceRange.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresourceRange {
  pub aspectMask: VkImageAspectFlags,
  pub baseMipLevel: u32,
  pub levelCount: u32,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
impl Default for VkImageSubresourceRange {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      aspectMask: Default::default(),
      baseMipLevel: Default::default(),
      levelCount: Default::default(),
      baseArrayLayer: Default::default(),
      layerCount: Default::default(),
    }
  }
}

/// Khronos: [VkImageSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSwapchainCreateInfoKHR.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSwapchainCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub swapchain: VkSwapchainKHR,
}
impl Default for VkImageSwapchainCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      swapchain: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewASTCDecodeModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewASTCDecodeModeEXT.html)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewASTCDecodeModeEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub decodeMode: VkFormat,
}
impl Default for VkImageViewASTCDecodeModeEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT,
      pNext: core::ptr::null(),
      decodeMode: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewAddressPropertiesNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewAddressPropertiesNVX.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewAddressPropertiesNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub deviceAddress: VkDeviceAddress,
  pub size: VkDeviceSize,
}
impl Default for VkImageViewAddressPropertiesNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX,
      pNext: core::ptr::null_mut(),
      deviceAddress: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCaptureDescriptorDataInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewCaptureDescriptorDataInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub imageView: VkImageView,
}
impl Default for VkImageViewCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      pNext: core::ptr::null(),
      imageView: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkImageViewCreateFlags,
  pub image: VkImage,
  pub viewType: VkImageViewType,
  pub format: VkFormat,
  pub components: VkComponentMapping,
  pub subresourceRange: VkImageSubresourceRange,
}
impl Default for VkImageViewCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      image: Default::default(),
      viewType: Default::default(),
      format: Default::default(),
      components: Default::default(),
      subresourceRange: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewHandleInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewHandleInfoNVX.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewHandleInfoNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub imageView: VkImageView,
  pub descriptorType: VkDescriptorType,
  /// * Optional: true
  pub sampler: VkSampler,
}
impl Default for VkImageViewHandleInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX,
      pNext: core::ptr::null(),
      imageView: Default::default(),
      descriptorType: Default::default(),
      sampler: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewMinLodCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewMinLodCreateInfoEXT.html)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewMinLodCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub minLod: c_float,
}
impl Default for VkImageViewMinLodCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      minLod: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewSampleWeightCreateInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewSampleWeightCreateInfoQCOM.html)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewSampleWeightCreateInfoQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub filterCenter: VkOffset2D,
  pub filterSize: VkExtent2D,
  pub numPhases: u32,
}
impl Default for VkImageViewSampleWeightCreateInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM,
      pNext: core::ptr::null(),
      filterCenter: Default::default(),
      filterSize: Default::default(),
      numPhases: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewSlicedCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewSlicedCreateInfoEXT.html)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewSlicedCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_SLICED_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub sliceOffset: u32,
  pub sliceCount: u32,
}
impl Default for VkImageViewSlicedCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_SLICED_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      sliceOffset: Default::default(),
      sliceCount: Default::default(),
    }
  }
}

/// Khronos: [VkImageViewUsageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewUsageCreateInfo.html)
///
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewUsageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub usage: VkImageUsageFlags,
}
impl Default for VkImageViewUsageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO,
      pNext: core::ptr::null(),
      usage: Default::default(),
    }
  }
}

/// Khronos: [VkImportAndroidHardwareBufferInfoANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportAndroidHardwareBufferInfoANDROID.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportAndroidHardwareBufferInfoANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub buffer: *mut AHardwareBuffer,
}
impl Default for VkImportAndroidHardwareBufferInfoANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
      pNext: core::ptr::null(),
      buffer: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportFenceFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportFenceFdInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportFenceFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub fence: VkFence,
  /// * Optional: true
  pub flags: VkFenceImportFlags,
  pub handleType: VkExternalFenceHandleTypeFlagBits,
  pub fd: c_int,
}
impl Default for VkImportFenceFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR,
      pNext: core::ptr::null(),
      fence: Default::default(),
      flags: Default::default(),
      handleType: Default::default(),
      fd: Default::default(),
    }
  }
}

/// Khronos: [VkImportFenceSciSyncInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportFenceSciSyncInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportFenceSciSyncInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_FENCE_SCI_SYNC_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBits,
  pub handle: *mut c_void,
}
impl Default for VkImportFenceSciSyncInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_FENCE_SCI_SYNC_INFO_NV,
      pNext: core::ptr::null(),
      fence: Default::default(),
      handleType: Default::default(),
      handle: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportFenceWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportFenceWin32HandleInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportFenceWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub fence: VkFence,
  /// * Optional: true
  pub flags: VkFenceImportFlags,
  /// * No Auto-Validity
  pub handleType: VkExternalFenceHandleTypeFlagBits,
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
      sType: VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
      pNext: core::ptr::null(),
      fence: Default::default(),
      flags: Default::default(),
      handleType: Default::default(),
      handle: core::ptr::null_mut(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMemoryBufferCollectionFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryBufferCollectionFUCHSIA.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryBufferCollectionFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub collection: VkBufferCollectionFUCHSIA,
  pub index: u32,
}
impl Default for VkImportMemoryBufferCollectionFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA,
      pNext: core::ptr::null(),
      collection: Default::default(),
      index: Default::default(),
    }
  }
}

/// Khronos: [VkImportMemoryFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryFdInfoKHR.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  pub fd: c_int,
}
impl Default for VkImportMemoryFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR,
      pNext: core::ptr::null(),
      handleType: Default::default(),
      fd: Default::default(),
    }
  }
}

/// Khronos: [VkImportMemoryHostPointerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryHostPointerInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  /// * Optional: false
  pub pHostPointer: *mut c_void,
}
impl Default for VkImportMemoryHostPointerInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
      pNext: core::ptr::null(),
      handleType: Default::default(),
      pHostPointer: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMemoryWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoKHR.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
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
      sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
      pNext: core::ptr::null(),
      handleType: Default::default(),
      handle: core::ptr::null_mut(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMemoryWin32HandleInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleType: VkExternalMemoryHandleTypeFlagsNV,
  /// * Optional: true
  pub handle: HANDLE,
}
impl Default for VkImportMemoryWin32HandleInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
      pNext: core::ptr::null(),
      handleType: Default::default(),
      handle: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMemoryZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryZirconHandleInfoFUCHSIA.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMemoryZirconHandleInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  /// * Optional: true
  pub handle: zx_handle_t,
}
impl Default for VkImportMemoryZirconHandleInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      handleType: Default::default(),
      handle: Default::default(),
    }
  }
}

/// Khronos: [VkImportMetalBufferInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMetalBufferInfoEXT.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMetalBufferInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub mtlBuffer: MTLBuffer_id,
}
impl Default for VkImportMetalBufferInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT,
      pNext: core::ptr::null(),
      mtlBuffer: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMetalIOSurfaceInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMetalIOSurfaceInfoEXT.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMetalIOSurfaceInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub ioSurface: IOSurfaceRef,
}
impl Default for VkImportMetalIOSurfaceInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT,
      pNext: core::ptr::null(),
      ioSurface: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMetalSharedEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMetalSharedEventInfoEXT.html)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
/// * Struct Extends: [`VkEventCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMetalSharedEventInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub mtlSharedEvent: MTLSharedEvent_id,
}
impl Default for VkImportMetalSharedEventInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT,
      pNext: core::ptr::null(),
      mtlSharedEvent: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportMetalTextureInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMetalTextureInfoEXT.html)
///
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportMetalTextureInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub plane: VkImageAspectFlagBits,
  pub mtlTexture: MTLTexture_id,
}
impl Default for VkImportMetalTextureInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT,
      pNext: core::ptr::null(),
      plane: Default::default(),
      mtlTexture: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportSemaphoreFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreFdInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportSemaphoreFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  pub flags: VkSemaphoreImportFlags,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
  pub fd: c_int,
}
impl Default for VkImportSemaphoreFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      flags: Default::default(),
      handleType: Default::default(),
      fd: Default::default(),
    }
  }
}

/// Khronos: [VkImportSemaphoreSciSyncInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreSciSyncInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportSemaphoreSciSyncInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
  pub handle: *mut c_void,
}
impl Default for VkImportSemaphoreSciSyncInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      handleType: Default::default(),
      handle: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportSemaphoreWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreWin32HandleInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  pub flags: VkSemaphoreImportFlags,
  /// * No Auto-Validity
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
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
      sType: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      flags: Default::default(),
      handleType: Default::default(),
      handle: core::ptr::null_mut(),
      name: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkImportSemaphoreZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreZirconHandleInfoFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Extern Sync: true
  pub semaphore: VkSemaphore,
  /// * Optional: true
  pub flags: VkSemaphoreImportFlags,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
  pub zirconHandle: zx_handle_t,
}
impl Default for VkImportSemaphoreZirconHandleInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      flags: Default::default(),
      handleType: Default::default(),
      zirconHandle: Default::default(),
    }
  }
}

/// Khronos: [VkIndirectCommandsLayoutCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsLayoutCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkIndirectCommandsLayoutUsageFlagsNV,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub tokenCount: u32,
  /// * Len: `tokenCount`
  pub pTokens: *const VkIndirectCommandsLayoutTokenNV,
  pub streamCount: u32,
  /// * Len: `streamCount`
  pub pStreamStrides: *const u32,
}
impl Default for VkIndirectCommandsLayoutCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pipelineBindPoint: Default::default(),
      tokenCount: Default::default(),
      pTokens: core::ptr::null(),
      streamCount: Default::default(),
      pStreamStrides: core::ptr::null(),
    }
  }
}

/// Khronos: [VkIndirectCommandsLayoutTokenNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsLayoutTokenNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub tokenType: VkIndirectCommandsTokenTypeNV,
  pub stream: u32,
  pub offset: u32,
  pub vertexBindingUnit: u32,
  pub vertexDynamicStride: VkBool32,
  /// * Optional: true
  pub pushconstantPipelineLayout: VkPipelineLayout,
  /// * Optional: true
  pub pushconstantShaderStageFlags: VkShaderStageFlags,
  pub pushconstantOffset: u32,
  pub pushconstantSize: u32,
  /// * Optional: true
  pub indirectStateFlags: VkIndirectStateFlagsNV,
  /// * Optional: true
  pub indexTypeCount: u32,
  /// * Len: `indexTypeCount`
  pub pIndexTypes: *const VkIndexType,
  /// * Len: `indexTypeCount`
  pub pIndexTypeValues: *const u32,
}
impl Default for VkIndirectCommandsLayoutTokenNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV,
      pNext: core::ptr::null(),
      tokenType: Default::default(),
      stream: Default::default(),
      offset: Default::default(),
      vertexBindingUnit: Default::default(),
      vertexDynamicStride: Default::default(),
      pushconstantPipelineLayout: Default::default(),
      pushconstantShaderStageFlags: Default::default(),
      pushconstantOffset: Default::default(),
      pushconstantSize: Default::default(),
      indirectStateFlags: Default::default(),
      indexTypeCount: Default::default(),
      pIndexTypes: core::ptr::null(),
      pIndexTypeValues: core::ptr::null(),
    }
  }
}

/// Khronos: [VkIndirectCommandsStreamNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsStreamNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsStreamNV {
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
}
impl Default for VkIndirectCommandsStreamNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      buffer: Default::default(),
      offset: Default::default(),
    }
  }
}

/// Khronos: [VkInitializePerformanceApiInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInitializePerformanceApiInfoINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub pUserData: *mut c_void,
}
impl Default for VkInitializePerformanceApiInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL,
      pNext: core::ptr::null(),
      pUserData: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkInputAttachmentAspectReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInputAttachmentAspectReference.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInputAttachmentAspectReference {
  pub subpass: u32,
  pub inputAttachmentIndex: u32,
  pub aspectMask: VkImageAspectFlags,
}
impl Default for VkInputAttachmentAspectReference {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      subpass: Default::default(),
      inputAttachmentIndex: Default::default(),
      aspectMask: Default::default(),
    }
  }
}

/// Khronos: [VkInstanceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkInstanceCreateFlags,
  /// * Optional: true
  pub pApplicationInfo: *const VkApplicationInfo,
  /// * Optional: true
  pub enabledLayerCount: u32,
  /// Ordered list of layer names to be enabled
  /// * Len: `enabledLayerCount,null-terminated`
  pub ppEnabledLayerNames: *const *const u8,
  /// * Optional: true
  pub enabledExtensionCount: u32,
  /// Extension names to be enabled
  /// * Len: `enabledExtensionCount,null-terminated`
  pub ppEnabledExtensionNames: *const *const u8,
}
impl Default for VkInstanceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pApplicationInfo: core::ptr::null(),
      enabledLayerCount: Default::default(),
      ppEnabledLayerNames: core::ptr::null(),
      enabledExtensionCount: Default::default(),
      ppEnabledExtensionNames: core::ptr::null(),
    }
  }
}

/// Khronos: [VkLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLayerProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkLayerProperties {
  /// layer name
  pub layerName: [u8; VK_MAX_EXTENSION_NAME_SIZE],
  /// version of the layer specification implemented
  pub specVersion: u32,
  /// build or release version of the layer's library
  pub implementationVersion: u32,
  /// Free-form description of the layer
  pub description: [u8; VK_MAX_DESCRIPTION_SIZE],
}
impl Default for VkLayerProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      layerName: [Default::default(); VK_MAX_EXTENSION_NAME_SIZE],
      specVersion: Default::default(),
      implementationVersion: Default::default(),
      description: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
    }
  }
}

/// Khronos: [VkMacOSSurfaceCreateInfoMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMacOSSurfaceCreateInfoMVK {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkMacOSSurfaceCreateFlagsMVK,
  /// * No Auto-Validity
  pub pView: *const c_void,
}
impl Default for VkMacOSSurfaceCreateInfoMVK {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pView: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMappedMemoryRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMappedMemoryRange.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMappedMemoryRange {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
      pNext: core::ptr::null(),
      memory: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryAllocateFlagsInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsInfo.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryAllocateFlagsInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkMemoryAllocateFlags,
  pub deviceMask: u32,
}
impl Default for VkMemoryAllocateFlagsInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      deviceMask: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Size of memory allocation
  pub allocationSize: VkDeviceSize,
  /// Index of the of the memory type to allocate from
  pub memoryTypeIndex: u32,
}
impl Default for VkMemoryAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
      pNext: core::ptr::null(),
      allocationSize: Default::default(),
      memoryTypeIndex: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryBarrier {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_BARRIER`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * Optional: true
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * Optional: true
  pub dstAccessMask: VkAccessFlags,
}
impl Default for VkMemoryBarrier {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER,
      pNext: core::ptr::null(),
      srcAccessMask: Default::default(),
      dstAccessMask: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryBarrier2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier2.html)
///
/// * Struct Extends: [`VkSubpassDependency2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryBarrier2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_BARRIER_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub srcStageMask: VkPipelineStageFlags2,
  /// * Optional: true
  pub srcAccessMask: VkAccessFlags2,
  /// * Optional: true
  pub dstStageMask: VkPipelineStageFlags2,
  /// * Optional: true
  pub dstAccessMask: VkAccessFlags2,
}
impl Default for VkMemoryBarrier2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER_2,
      pNext: core::ptr::null(),
      srcStageMask: Default::default(),
      srcAccessMask: Default::default(),
      dstStageMask: Default::default(),
      dstAccessMask: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryDedicatedAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedAllocateInfo.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO,
      pNext: core::ptr::null(),
      image: Default::default(),
      buffer: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryDedicatedRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedRequirements.html)
///
/// * Struct Extends: [`VkMemoryRequirements2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryDedicatedRequirements {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub prefersDedicatedAllocation: VkBool32,
  pub requiresDedicatedAllocation: VkBool32,
}
impl Default for VkMemoryDedicatedRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS,
      pNext: core::ptr::null_mut(),
      prefersDedicatedAllocation: Default::default(),
      requiresDedicatedAllocation: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryFdPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryFdPropertiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryFdPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
impl Default for VkMemoryFdPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      memoryTypeBits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetAndroidHardwareBufferInfoANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
}
impl Default for VkMemoryGetAndroidHardwareBufferInfoANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
      pNext: core::ptr::null(),
      memory: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetFdInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR,
      pNext: core::ptr::null(),
      memory: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetRemoteAddressInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetRemoteAddressInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetRemoteAddressInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetRemoteAddressInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV,
      pNext: core::ptr::null(),
      memory: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetSciBufInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetSciBufInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetSciBufInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_SCI_BUF_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetSciBufInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_GET_SCI_BUF_INFO_NV,
      pNext: core::ptr::null(),
      memory: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetWin32HandleInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR,
      pNext: core::ptr::null(),
      memory: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryGetZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetZirconHandleInfoFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryGetZirconHandleInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkMemoryGetZirconHandleInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      memory: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryHeap](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeap.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
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
    Self {
      size: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryHostPointerPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryHostPointerPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
impl Default for VkMemoryHostPointerPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      memoryTypeBits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryOpaqueCaptureAddressAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfo.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub opaqueCaptureAddress: u64,
}
impl Default for VkMemoryOpaqueCaptureAddressAllocateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO,
      pNext: core::ptr::null(),
      opaqueCaptureAddress: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryPriorityAllocateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html)
///
/// * Struct Extends: [`VkMemoryAllocateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryPriorityAllocateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub priority: c_float,
}
impl Default for VkMemoryPriorityAllocateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT,
      pNext: core::ptr::null(),
      priority: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryRequirements {
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Specified in bytes
  pub alignment: VkDeviceSize,
  /// Bitmask of the allowed memory type indices into memoryTypes\[\] for this object
  pub memoryTypeBits: u32,
}
impl Default for VkMemoryRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      size: Default::default(),
      alignment: Default::default(),
      memoryTypeBits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements2.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryRequirements2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryRequirements: VkMemoryRequirements,
}
impl Default for VkMemoryRequirements2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2,
      pNext: core::ptr::null_mut(),
      memoryRequirements: Default::default(),
    }
  }
}

/// Khronos: [VkMemorySciBufPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemorySciBufPropertiesNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemorySciBufPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_SCI_BUF_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub memoryTypeBits: u32,
}
impl Default for VkMemorySciBufPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_SCI_BUF_PROPERTIES_NV,
      pNext: core::ptr::null(),
      memoryTypeBits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryType.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryType {
  /// Memory properties of this memory type
  /// * Optional: true
  pub propertyFlags: VkMemoryPropertyFlags,
  /// Index of the memory heap allocations of this memory type are taken from
  pub heapIndex: u32,
}
impl Default for VkMemoryType {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      propertyFlags: Default::default(),
      heapIndex: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryWin32HandlePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryWin32HandlePropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
impl Default for VkMemoryWin32HandlePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      memoryTypeBits: Default::default(),
    }
  }
}

/// Khronos: [VkMemoryZirconHandlePropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryZirconHandlePropertiesFUCHSIA.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryZirconHandlePropertiesFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
impl Default for VkMemoryZirconHandlePropertiesFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA,
      pNext: core::ptr::null_mut(),
      memoryTypeBits: Default::default(),
    }
  }
}

/// Khronos: [VkMetalSurfaceCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMetalSurfaceCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkMetalSurfaceCreateFlagsEXT,
  /// * No Auto-Validity
  pub pLayer: *const CAMetalLayer,
}
impl Default for VkMetalSurfaceCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pLayer: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMicromapBuildInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapBuildInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapBuildInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MICROMAP_BUILD_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub ty: VkMicromapTypeEXT,
  /// * Optional: true
  pub flags: VkBuildMicromapFlagsEXT,
  /// * No Auto-Validity
  pub mode: VkBuildMicromapModeEXT,
  /// * Optional: true
  /// * No Auto-Validity
  pub dstMicromap: VkMicromapEXT,
  /// * Optional: true
  pub usageCountsCount: u32,
  /// * Optional: true
  /// * Len: `usageCountsCount`
  pub pUsageCounts: *const VkMicromapUsageEXT,
  /// * Optional: true,false
  /// * Len: `usageCountsCount,1`
  pub ppUsageCounts: *const *const VkMicromapUsageEXT,
  /// * No Auto-Validity
  pub data: VkDeviceOrHostAddressConstKHR,
  /// * No Auto-Validity
  pub scratchData: VkDeviceOrHostAddressKHR,
  /// * No Auto-Validity
  pub triangleArray: VkDeviceOrHostAddressConstKHR,
  pub triangleArrayStride: VkDeviceSize,
}
impl Default for VkMicromapBuildInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MICROMAP_BUILD_INFO_EXT,
      pNext: core::ptr::null(),
      ty: Default::default(),
      flags: Default::default(),
      mode: Default::default(),
      dstMicromap: Default::default(),
      usageCountsCount: Default::default(),
      pUsageCounts: core::ptr::null(),
      ppUsageCounts: core::ptr::null(),
      data: Default::default(),
      scratchData: Default::default(),
      triangleArray: Default::default(),
      triangleArrayStride: Default::default(),
    }
  }
}

/// Khronos: [VkMicromapBuildSizesInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapBuildSizesInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapBuildSizesInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MICROMAP_BUILD_SIZES_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub micromapSize: VkDeviceSize,
  pub buildScratchSize: VkDeviceSize,
  pub discardable: VkBool32,
}
impl Default for VkMicromapBuildSizesInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MICROMAP_BUILD_SIZES_INFO_EXT,
      pNext: core::ptr::null(),
      micromapSize: Default::default(),
      buildScratchSize: Default::default(),
      discardable: Default::default(),
    }
  }
}

/// Khronos: [VkMicromapCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapCreateInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MICROMAP_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub createFlags: VkMicromapCreateFlagsEXT,
  pub buffer: VkBuffer,
  /// Specified in bytes
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  pub ty: VkMicromapTypeEXT,
  /// * Optional: true
  pub deviceAddress: VkDeviceAddress,
}
impl Default for VkMicromapCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MICROMAP_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      createFlags: Default::default(),
      buffer: Default::default(),
      offset: Default::default(),
      size: Default::default(),
      ty: Default::default(),
      deviceAddress: Default::default(),
    }
  }
}

/// Khronos: [VkMicromapTriangleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapTriangleEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapTriangleEXT {
  /// Specified in bytes
  pub dataOffset: u32,
  pub subdivisionLevel: u16,
  pub format: u16,
}
impl Default for VkMicromapTriangleEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      dataOffset: Default::default(),
      subdivisionLevel: Default::default(),
      format: Default::default(),
    }
  }
}

/// Khronos: [VkMicromapUsageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapUsageEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapUsageEXT {
  pub count: u32,
  pub subdivisionLevel: u32,
  /// Interpretation depends on parent type
  pub format: u32,
}
impl Default for VkMicromapUsageEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      count: Default::default(),
      subdivisionLevel: Default::default(),
      format: Default::default(),
    }
  }
}

/// Khronos: [VkMicromapVersionInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapVersionInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapVersionInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MICROMAP_VERSION_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Len: `2*VK_UUID_SIZE`
  pub pVersionData: *const u8,
}
impl Default for VkMicromapVersionInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MICROMAP_VERSION_INFO_EXT,
      pNext: core::ptr::null(),
      pVersionData: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMultiDrawIndexedInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiDrawIndexedInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiDrawIndexedInfoEXT {
  pub firstIndex: u32,
  pub indexCount: u32,
  pub vertexOffset: i32,
}
impl Default for VkMultiDrawIndexedInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      firstIndex: Default::default(),
      indexCount: Default::default(),
      vertexOffset: Default::default(),
    }
  }
}

/// Khronos: [VkMultiDrawInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiDrawInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiDrawInfoEXT {
  pub firstVertex: u32,
  pub vertexCount: u32,
}
impl Default for VkMultiDrawInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      firstVertex: Default::default(),
      vertexCount: Default::default(),
    }
  }
}

/// Khronos: [VkMultisamplePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultisamplePropertiesEXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultisamplePropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub maxSampleLocationGridSize: VkExtent2D,
}
impl Default for VkMultisamplePropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      maxSampleLocationGridSize: Default::default(),
    }
  }
}

/// Khronos: [VkMultisampledRenderToSingleSampledInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultisampledRenderToSingleSampledInfoEXT.html)
///
/// * Struct Extends: [`VkSubpassDescription2`]
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultisampledRenderToSingleSampledInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub multisampledRenderToSingleSampledEnable: VkBool32,
  pub rasterizationSamples: VkSampleCountFlagBits,
}
impl Default for VkMultisampledRenderToSingleSampledInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT,
      pNext: core::ptr::null(),
      multisampledRenderToSingleSampledEnable: Default::default(),
      rasterizationSamples: Default::default(),
    }
  }
}

/// Khronos: [VkMultiviewPerViewAttributesInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiviewPerViewAttributesInfoNVX.html)
///
/// * Struct Extends: [`VkCommandBufferInheritanceInfo`]
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiviewPerViewAttributesInfoNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub perViewAttributes: VkBool32,
  pub perViewAttributesPositionXOnly: VkBool32,
}
impl Default for VkMultiviewPerViewAttributesInfoNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX,
      pNext: core::ptr::null(),
      perViewAttributes: Default::default(),
      perViewAttributesPositionXOnly: Default::default(),
    }
  }
}

/// Khronos: [VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM.html)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub perViewRenderAreaCount: u32,
  /// * Len: `perViewRenderAreaCount`
  pub pPerViewRenderAreas: *const VkRect2D,
}
impl Default for VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM,
      pNext: core::ptr::null(),
      perViewRenderAreaCount: Default::default(),
      pPerViewRenderAreas: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMutableDescriptorTypeCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeCreateInfoEXT.html)
///
/// * Struct Extends: [`VkDescriptorSetLayoutCreateInfo`]
/// * Struct Extends: [`VkDescriptorPoolCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMutableDescriptorTypeCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub mutableDescriptorTypeListCount: u32,
  /// * Len: `mutableDescriptorTypeListCount`
  pub pMutableDescriptorTypeLists: *const VkMutableDescriptorTypeListEXT,
}
impl Default for VkMutableDescriptorTypeCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      mutableDescriptorTypeListCount: Default::default(),
      pMutableDescriptorTypeLists: core::ptr::null(),
    }
  }
}

/// Khronos: [VkMutableDescriptorTypeListEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeListEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMutableDescriptorTypeListEXT {
  /// * Optional: true
  pub descriptorTypeCount: u32,
  /// * Len: `descriptorTypeCount`
  pub pDescriptorTypes: *const VkDescriptorType,
}
impl Default for VkMutableDescriptorTypeListEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      descriptorTypeCount: Default::default(),
      pDescriptorTypes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkNativeBufferANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkNativeBufferANDROID.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkNativeBufferANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_NATIVE_BUFFER_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub handle: *const c_void,
  pub stride: c_int,
  pub format: c_int,
  pub usage: c_int,
  pub usage2: VkNativeBufferUsage2ANDROID,
}
impl Default for VkNativeBufferANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_NATIVE_BUFFER_ANDROID,
      pNext: core::ptr::null(),
      handle: core::ptr::null(),
      stride: Default::default(),
      format: Default::default(),
      usage: Default::default(),
      usage2: Default::default(),
    }
  }
}

/// Khronos: [VkNativeBufferUsage2ANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkNativeBufferUsage2ANDROID.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkNativeBufferUsage2ANDROID {
  pub consumer: u64,
  pub producer: u64,
}
impl Default for VkNativeBufferUsage2ANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      consumer: Default::default(),
      producer: Default::default(),
    }
  }
}

/// Khronos: [VkOffset2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOffset2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOffset2D {
  pub x: i32,
  pub y: i32,
}
impl Default for VkOffset2D {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      x: Default::default(),
      y: Default::default(),
    }
  }
}

/// Khronos: [VkOffset3D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOffset3D.html)
#[derive(Clone, Copy)]
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
    Self {
      x: Default::default(),
      y: Default::default(),
      z: Default::default(),
    }
  }
}

/// Khronos: [VkOpaqueCaptureDescriptorDataCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpaqueCaptureDescriptorDataCreateInfoEXT.html)
///
/// * Struct Extends: [`VkBufferCreateInfo`]
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkImageViewCreateInfo`]
/// * Struct Extends: [`VkSamplerCreateInfo`]
/// * Struct Extends: [`VkAccelerationStructureCreateInfoKHR`]
/// * Struct Extends: [`VkAccelerationStructureCreateInfoNV`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpaqueCaptureDescriptorDataCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub opaqueCaptureDescriptorData: *const c_void,
}
impl Default for VkOpaqueCaptureDescriptorDataCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      opaqueCaptureDescriptorData: core::ptr::null(),
    }
  }
}

/// Khronos: [VkOpticalFlowExecuteInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowExecuteInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_OPTICAL_FLOW_EXECUTE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  pub flags: VkOpticalFlowExecuteFlagsNV,
  /// * Optional: true
  pub regionCount: u32,
  /// * Len: `regionCount`
  pub pRegions: *const VkRect2D,
}
impl Default for VkOpticalFlowExecuteInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_OPTICAL_FLOW_EXECUTE_INFO_NV,
      pNext: core::ptr::null_mut(),
      flags: Default::default(),
      regionCount: Default::default(),
      pRegions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkOpticalFlowImageFormatInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowImageFormatInfoNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
/// * Struct Extends: [`VkImageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowImageFormatInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub usage: VkOpticalFlowUsageFlagsNV,
}
impl Default for VkOpticalFlowImageFormatInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV,
      pNext: core::ptr::null(),
      usage: Default::default(),
    }
  }
}

/// Khronos: [VkOpticalFlowImageFormatPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowImageFormatPropertiesNV.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowImageFormatPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub format: VkFormat,
}
impl Default for VkOpticalFlowImageFormatPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV,
      pNext: core::ptr::null(),
      format: Default::default(),
    }
  }
}

/// Khronos: [VkOpticalFlowSessionCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowSessionCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub width: u32,
  pub height: u32,
  pub imageFormat: VkFormat,
  pub flowVectorFormat: VkFormat,
  /// * Optional: true
  pub costFormat: VkFormat,
  pub outputGridSize: VkOpticalFlowGridSizeFlagsNV,
  /// * Optional: true
  pub hintGridSize: VkOpticalFlowGridSizeFlagsNV,
  /// * Optional: true
  pub performanceLevel: VkOpticalFlowPerformanceLevelNV,
  /// * Optional: true
  pub flags: VkOpticalFlowSessionCreateFlagsNV,
}
impl Default for VkOpticalFlowSessionCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_INFO_NV,
      pNext: core::ptr::null_mut(),
      width: Default::default(),
      height: Default::default(),
      imageFormat: Default::default(),
      flowVectorFormat: Default::default(),
      costFormat: Default::default(),
      outputGridSize: Default::default(),
      hintGridSize: Default::default(),
      performanceLevel: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkOpticalFlowSessionCreatePrivateDataInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreatePrivateDataInfoNV.html)
///
/// NV internal use only
/// * Struct Extends: [`VkOpticalFlowSessionCreateInfoNV`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOpticalFlowSessionCreatePrivateDataInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub id: u32,
  pub size: u32,
  pub pPrivateData: *const c_void,
}
impl Default for VkOpticalFlowSessionCreatePrivateDataInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV,
      pNext: core::ptr::null_mut(),
      id: Default::default(),
      size: Default::default(),
      pPrivateData: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPastPresentationTimingGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPastPresentationTimingGOOGLE.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPastPresentationTimingGOOGLE {
  /// Application-provided identifier, previously given to vkQueuePresentKHR
  pub presentID: u32,
  /// Earliest time an image should have been presented, previously given to vkQueuePresentKHR
  pub desiredPresentTime: u64,
  /// Time the image was actually displayed
  pub actualPresentTime: u64,
  /// Earliest time the image could have been displayed
  pub earliestPresentTime: u64,
  /// How early vkQueuePresentKHR was processed vs. how soon it needed to be and make earliestPresentTime
  pub presentMargin: u64,
}
impl Default for VkPastPresentationTimingGOOGLE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      presentID: Default::default(),
      desiredPresentTime: Default::default(),
      actualPresentTime: Default::default(),
      earliestPresentTime: Default::default(),
      presentMargin: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceConfigurationAcquireInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceConfigurationAcquireInfoINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub ty: VkPerformanceConfigurationTypeINTEL,
}
impl Default for VkPerformanceConfigurationAcquireInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL,
      pNext: core::ptr::null(),
      ty: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceCounterDescriptionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceCounterDescriptionKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  pub flags: VkPerformanceCounterDescriptionFlagsKHR,
  pub name: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub category: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub description: [u8; VK_MAX_DESCRIPTION_SIZE],
}
impl Default for VkPerformanceCounterDescriptionKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR,
      pNext: core::ptr::null_mut(),
      flags: Default::default(),
      name: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      category: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      description: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
    }
  }
}

/// Khronos: [VkPerformanceCounterKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceCounterKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
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
      sType: VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR,
      pNext: core::ptr::null_mut(),
      unit: Default::default(),
      scope: Default::default(),
      storage: Default::default(),
      uuid: [Default::default(); VK_UUID_SIZE],
    }
  }
}

/// Khronos: [VkPerformanceMarkerInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceMarkerInfoINTEL.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceMarkerInfoINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub marker: u64,
}
impl Default for VkPerformanceMarkerInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL,
      pNext: core::ptr::null(),
      marker: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceOverrideInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideInfoINTEL.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceOverrideInfoINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub ty: VkPerformanceOverrideTypeINTEL,
  pub enable: VkBool32,
  pub parameter: u64,
}
impl Default for VkPerformanceOverrideInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL,
      pNext: core::ptr::null(),
      ty: Default::default(),
      enable: Default::default(),
      parameter: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceQuerySubmitInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html)
///
/// * Struct Extends: [`VkSubmitInfo`]
/// * Struct Extends: [`VkSubmitInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceQuerySubmitInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Index for which counter pass to submit
  pub counterPassIndex: u32,
}
impl Default for VkPerformanceQuerySubmitInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR,
      pNext: core::ptr::null(),
      counterPassIndex: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceStreamMarkerInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceStreamMarkerInfoINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub marker: u32,
}
impl Default for VkPerformanceStreamMarkerInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL,
      pNext: core::ptr::null(),
      marker: Default::default(),
    }
  }
}

/// Khronos: [VkPerformanceValueINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueINTEL.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceValueINTEL {
  pub ty: VkPerformanceValueTypeINTEL,
  /// * Selector: type
  /// * No Auto-Validity
  pub data: VkPerformanceValueDataINTEL,
}
impl Default for VkPerformanceValueINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      ty: Default::default(),
      data: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevice16BitStorageFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevice16BitStorageFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// 16-bit integer/floating-point variables supported in BufferBlock
  pub storageBuffer16BitAccess: VkBool32,
  /// 16-bit integer/floating-point variables supported in BufferBlock and Block
  pub uniformAndStorageBuffer16BitAccess: VkBool32,
  /// 16-bit integer/floating-point variables supported in PushConstant
  pub storagePushConstant16: VkBool32,
  /// 16-bit integer/floating-point variables supported in shader inputs and outputs
  pub storageInputOutput16: VkBool32,
}
impl Default for VkPhysicalDevice16BitStorageFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
      pNext: core::ptr::null_mut(),
      storageBuffer16BitAccess: Default::default(),
      uniformAndStorageBuffer16BitAccess: Default::default(),
      storagePushConstant16: Default::default(),
      storageInputOutput16: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevice4444FormatsFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevice4444FormatsFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub formatA4R4G4B4: VkBool32,
  pub formatA4B4G4R4: VkBool32,
}
impl Default for VkPhysicalDevice4444FormatsFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      formatA4R4G4B4: Default::default(),
      formatA4B4G4R4: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevice8BitStorageFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice8BitStorageFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevice8BitStorageFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// 8-bit integer variables supported in StorageBuffer
  pub storageBuffer8BitAccess: VkBool32,
  /// 8-bit integer variables supported in StorageBuffer and Uniform
  pub uniformAndStorageBuffer8BitAccess: VkBool32,
  /// 8-bit integer variables supported in PushConstant
  pub storagePushConstant8: VkBool32,
}
impl Default for VkPhysicalDevice8BitStorageFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES,
      pNext: core::ptr::null_mut(),
      storageBuffer8BitAccess: Default::default(),
      uniformAndStorageBuffer8BitAccess: Default::default(),
      storagePushConstant8: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceASTCDecodeFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceASTCDecodeFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub decodeModeSharedExponent: VkBool32,
}
impl Default for VkPhysicalDeviceASTCDecodeFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      decodeModeSharedExponent: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAccelerationStructureFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub accelerationStructure: VkBool32,
  pub accelerationStructureCaptureReplay: VkBool32,
  pub accelerationStructureIndirectBuild: VkBool32,
  pub accelerationStructureHostCommands: VkBool32,
  pub descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32,
}
impl Default for VkPhysicalDeviceAccelerationStructureFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      accelerationStructure: Default::default(),
      accelerationStructureCaptureReplay: Default::default(),
      accelerationStructureIndirectBuild: Default::default(),
      accelerationStructureHostCommands: Default::default(),
      descriptorBindingAccelerationStructureUpdateAfterBind: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAccelerationStructurePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxGeometryCount: u64,
  /// * Limit Type: max
  pub maxInstanceCount: u64,
  /// * Limit Type: max
  pub maxPrimitiveCount: u64,
  /// * Limit Type: max
  pub maxPerStageDescriptorAccelerationStructures: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindAccelerationStructures: u32,
  /// * Limit Type: max
  pub maxDescriptorSetAccelerationStructures: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindAccelerationStructures: u32,
  /// * Limit Type: min
  pub minAccelerationStructureScratchOffsetAlignment: u32,
}
impl Default for VkPhysicalDeviceAccelerationStructurePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      maxGeometryCount: Default::default(),
      maxInstanceCount: Default::default(),
      maxPrimitiveCount: Default::default(),
      maxPerStageDescriptorAccelerationStructures: Default::default(),
      maxPerStageDescriptorUpdateAfterBindAccelerationStructures: Default::default(),
      maxDescriptorSetAccelerationStructures: Default::default(),
      maxDescriptorSetUpdateAfterBindAccelerationStructures: Default::default(),
      minAccelerationStructureScratchOffsetAlignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAddressBindingReportFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAddressBindingReportFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAddressBindingReportFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub reportAddressBinding: VkBool32,
}
impl Default for VkPhysicalDeviceAddressBindingReportFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      reportAddressBinding: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAmigoProfilingFeaturesSEC](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAmigoProfilingFeaturesSEC.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAmigoProfilingFeaturesSEC {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub amigoProfiling: VkBool32,
}
impl Default for VkPhysicalDeviceAmigoProfilingFeaturesSEC {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC,
      pNext: core::ptr::null_mut(),
      amigoProfiling: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub attachmentFeedbackLoopLayout: VkBool32,
}
impl Default for VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      attachmentFeedbackLoopLayout: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub advancedBlendCoherentOperations: VkBool32,
}
impl Default for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      advancedBlendCoherentOperations: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub advancedBlendMaxColorAttachments: u32,
  /// * Limit Type: bitmask
  pub advancedBlendIndependentBlend: VkBool32,
  /// * Limit Type: bitmask
  pub advancedBlendNonPremultipliedSrcColor: VkBool32,
  /// * Limit Type: bitmask
  pub advancedBlendNonPremultipliedDstColor: VkBool32,
  /// * Limit Type: bitmask
  pub advancedBlendCorrelatedOverlap: VkBool32,
  /// * Limit Type: bitmask
  pub advancedBlendAllOperations: VkBool32,
}
impl Default for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      advancedBlendMaxColorAttachments: Default::default(),
      advancedBlendIndependentBlend: Default::default(),
      advancedBlendNonPremultipliedSrcColor: Default::default(),
      advancedBlendNonPremultipliedDstColor: Default::default(),
      advancedBlendCorrelatedOverlap: Default::default(),
      advancedBlendAllOperations: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBorderColorSwizzleFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub borderColorSwizzle: VkBool32,
  pub borderColorSwizzleFromImage: VkBool32,
}
impl Default for VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      borderColorSwizzle: Default::default(),
      borderColorSwizzleFromImage: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBufferDeviceAddressFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub bufferDeviceAddress: VkBool32,
  pub bufferDeviceAddressCaptureReplay: VkBool32,
  pub bufferDeviceAddressMultiDevice: VkBool32,
}
impl Default for VkPhysicalDeviceBufferDeviceAddressFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES,
      pNext: core::ptr::null_mut(),
      bufferDeviceAddress: Default::default(),
      bufferDeviceAddressCaptureReplay: Default::default(),
      bufferDeviceAddressMultiDevice: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceBufferDeviceAddressFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub bufferDeviceAddress: VkBool32,
  pub bufferDeviceAddressCaptureReplay: VkBool32,
  pub bufferDeviceAddressMultiDevice: VkBool32,
}
impl Default for VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      bufferDeviceAddress: Default::default(),
      bufferDeviceAddressCaptureReplay: Default::default(),
      bufferDeviceAddressMultiDevice: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub clustercullingShader: VkBool32,
  pub multiviewClusterCullingShader: VkBool32,
}
impl Default for VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI,
      pNext: core::ptr::null_mut(),
      clustercullingShader: Default::default(),
      multiviewClusterCullingShader: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max,pot
  pub maxWorkGroupCount: [u32; 3],
  /// * Limit Type: max,pot
  pub maxWorkGroupSize: [u32; 3],
  /// * Limit Type: max
  pub maxOutputClusterCount: u32,
}
impl Default for VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI,
      pNext: core::ptr::null_mut(),
      maxWorkGroupCount: [Default::default(); 3],
      maxWorkGroupSize: [Default::default(); 3],
      maxOutputClusterCount: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCoherentMemoryFeaturesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub deviceCoherentMemory: VkBool32,
}
impl Default for VkPhysicalDeviceCoherentMemoryFeaturesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
      pNext: core::ptr::null_mut(),
      deviceCoherentMemory: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceColorWriteEnableFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub colorWriteEnable: VkBool32,
}
impl Default for VkPhysicalDeviceColorWriteEnableFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      colorWriteEnable: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceComputeShaderDerivativesFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub computeDerivativeGroupQuads: VkBool32,
  pub computeDerivativeGroupLinear: VkBool32,
}
impl Default for VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      computeDerivativeGroupQuads: Default::default(),
      computeDerivativeGroupLinear: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceConditionalRenderingFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub conditionalRendering: VkBool32,
  pub inheritedConditionalRendering: VkBool32,
}
impl Default for VkPhysicalDeviceConditionalRenderingFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      conditionalRendering: Default::default(),
      inheritedConditionalRendering: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceConservativeRasterizationPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// The size in pixels the primitive is enlarged at each edge during conservative rasterization
  /// * Limit Type: exact
  pub primitiveOverestimationSize: c_float,
  /// The maximum additional overestimation the client can specify in the pipeline state
  /// * Limit Type: max
  pub maxExtraPrimitiveOverestimationSize: c_float,
  /// The granularity of extra overestimation sizes the implementations supports between 0 and maxExtraOverestimationSize
  /// * Limit Type: min,mul
  pub extraPrimitiveOverestimationSizeGranularity: c_float,
  /// true if the implementation supports conservative rasterization underestimation mode
  /// * Limit Type: bitmask
  pub primitiveUnderestimation: VkBool32,
  /// true if conservative rasterization also applies to points and lines
  /// * Limit Type: bitmask
  pub conservativePointAndLineRasterization: VkBool32,
  /// true if degenerate triangles (those with zero area after snap) are rasterized
  /// * Limit Type: exact
  pub degenerateTrianglesRasterized: VkBool32,
  /// true if degenerate lines (those with zero length after snap) are rasterized
  /// * Limit Type: exact
  pub degenerateLinesRasterized: VkBool32,
  /// true if the implementation supports the FullyCoveredEXT SPIR-V builtin fragment shader input variable
  /// * Limit Type: bitmask
  pub fullyCoveredFragmentShaderInputVariable: VkBool32,
  /// true if the implementation supports both conservative rasterization and post depth coverage sample coverage mask
  /// * Limit Type: bitmask
  pub conservativeRasterizationPostDepthCoverage: VkBool32,
}
impl Default for VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      primitiveOverestimationSize: Default::default(),
      maxExtraPrimitiveOverestimationSize: Default::default(),
      extraPrimitiveOverestimationSizeGranularity: Default::default(),
      primitiveUnderestimation: Default::default(),
      conservativePointAndLineRasterization: Default::default(),
      degenerateTrianglesRasterized: Default::default(),
      degenerateLinesRasterized: Default::default(),
      fullyCoveredFragmentShaderInputVariable: Default::default(),
      conservativeRasterizationPostDepthCoverage: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCooperativeMatrixFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub cooperativeMatrix: VkBool32,
  pub cooperativeMatrixRobustBufferAccess: VkBool32,
}
impl Default for VkPhysicalDeviceCooperativeMatrixFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      cooperativeMatrix: Default::default(),
      cooperativeMatrixRobustBufferAccess: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCooperativeMatrixPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub cooperativeMatrixSupportedStages: VkShaderStageFlags,
}
impl Default for VkPhysicalDeviceCooperativeMatrixPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      cooperativeMatrixSupportedStages: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCopyMemoryIndirectFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCopyMemoryIndirectFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub indirectCopy: VkBool32,
}
impl Default for VkPhysicalDeviceCopyMemoryIndirectFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      indirectCopy: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCopyMemoryIndirectPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCopyMemoryIndirectPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Bitfield of which queues are supported for indirect copy
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub supportedQueues: VkQueueFlags,
}
impl Default for VkPhysicalDeviceCopyMemoryIndirectPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      supportedQueues: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCornerSampledImageFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub cornerSampledImage: VkBool32,
}
impl Default for VkPhysicalDeviceCornerSampledImageFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      cornerSampledImage: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCoverageReductionModeFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub coverageReductionMode: VkBool32,
}
impl Default for VkPhysicalDeviceCoverageReductionModeFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      coverageReductionMode: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCustomBorderColorFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub customBorderColors: VkBool32,
  pub customBorderColorWithoutFormat: VkBool32,
}
impl Default for VkPhysicalDeviceCustomBorderColorFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      customBorderColors: Default::default(),
      customBorderColorWithoutFormat: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceCustomBorderColorPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxCustomBorderColorSamplers: u32,
}
impl Default for VkPhysicalDeviceCustomBorderColorPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      maxCustomBorderColorSamplers: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub dedicatedAllocationImageAliasing: VkBool32,
}
impl Default for VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      dedicatedAllocationImageAliasing: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDepthClampZeroOneFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClampZeroOneFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClampZeroOneFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub depthClampZeroOne: VkBool32,
}
impl Default for VkPhysicalDeviceDepthClampZeroOneFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      depthClampZeroOne: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDepthClipControlFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipControlFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClipControlFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub depthClipControl: VkBool32,
}
impl Default for VkPhysicalDeviceDepthClipControlFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      depthClipControl: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDepthClipEnableFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub depthClipEnable: VkBool32,
}
impl Default for VkPhysicalDeviceDepthClipEnableFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      depthClipEnable: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDepthStencilResolveProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthStencilResolveProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// supported depth resolve modes
  /// * Limit Type: bitmask
  pub supportedDepthResolveModes: VkResolveModeFlags,
  /// supported stencil resolve modes
  /// * Limit Type: bitmask
  pub supportedStencilResolveModes: VkResolveModeFlags,
  /// depth and stencil resolve modes can be set independently if one of them is none
  /// * Limit Type: bitmask
  pub independentResolveNone: VkBool32,
  /// depth and stencil resolve modes can be set independently
  /// * Limit Type: bitmask
  pub independentResolve: VkBool32,
}
impl Default for VkPhysicalDeviceDepthStencilResolveProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES,
      pNext: core::ptr::null_mut(),
      supportedDepthResolveModes: Default::default(),
      supportedStencilResolveModes: Default::default(),
      independentResolveNone: Default::default(),
      independentResolve: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub combinedImageSamplerDensityMapDescriptorSize: c_size_t,
}
impl Default for VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      combinedImageSamplerDensityMapDescriptorSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorBufferFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub descriptorBuffer: VkBool32,
  pub descriptorBufferCaptureReplay: VkBool32,
  pub descriptorBufferImageLayoutIgnored: VkBool32,
  pub descriptorBufferPushDescriptors: VkBool32,
}
impl Default for VkPhysicalDeviceDescriptorBufferFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      descriptorBuffer: Default::default(),
      descriptorBufferCaptureReplay: Default::default(),
      descriptorBufferImageLayoutIgnored: Default::default(),
      descriptorBufferPushDescriptors: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorBufferPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorBufferPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: noauto
  pub combinedImageSamplerDescriptorSingleArray: VkBool32,
  /// * Limit Type: noauto
  pub bufferlessPushDescriptors: VkBool32,
  /// * Limit Type: noauto
  pub allowSamplerImageViewPostSubmitCreation: VkBool32,
  /// * Limit Type: noauto
  pub descriptorBufferOffsetAlignment: VkDeviceSize,
  /// * Limit Type: max
  pub maxDescriptorBufferBindings: u32,
  /// * Limit Type: max
  pub maxResourceDescriptorBufferBindings: u32,
  /// * Limit Type: max
  pub maxSamplerDescriptorBufferBindings: u32,
  /// * Limit Type: max
  pub maxEmbeddedImmutableSamplerBindings: u32,
  /// * Limit Type: max
  pub maxEmbeddedImmutableSamplers: u32,
  /// * Limit Type: noauto
  pub bufferCaptureReplayDescriptorDataSize: c_size_t,
  /// * Limit Type: noauto
  pub imageCaptureReplayDescriptorDataSize: c_size_t,
  /// * Limit Type: noauto
  pub imageViewCaptureReplayDescriptorDataSize: c_size_t,
  /// * Limit Type: noauto
  pub samplerCaptureReplayDescriptorDataSize: c_size_t,
  /// * Limit Type: noauto
  pub accelerationStructureCaptureReplayDescriptorDataSize: c_size_t,
  /// * Limit Type: max
  pub samplerDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub combinedImageSamplerDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub sampledImageDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub storageImageDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub uniformTexelBufferDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub robustUniformTexelBufferDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub storageTexelBufferDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub robustStorageTexelBufferDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub uniformBufferDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub robustUniformBufferDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub storageBufferDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub robustStorageBufferDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub inputAttachmentDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub accelerationStructureDescriptorSize: c_size_t,
  /// * Limit Type: max
  pub maxSamplerDescriptorBufferRange: VkDeviceSize,
  /// * Limit Type: max
  pub maxResourceDescriptorBufferRange: VkDeviceSize,
  /// * Limit Type: max
  pub samplerDescriptorBufferAddressSpaceSize: VkDeviceSize,
  /// * Limit Type: max
  pub resourceDescriptorBufferAddressSpaceSize: VkDeviceSize,
  /// * Limit Type: max
  pub descriptorBufferAddressSpaceSize: VkDeviceSize,
}
impl Default for VkPhysicalDeviceDescriptorBufferPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      combinedImageSamplerDescriptorSingleArray: Default::default(),
      bufferlessPushDescriptors: Default::default(),
      allowSamplerImageViewPostSubmitCreation: Default::default(),
      descriptorBufferOffsetAlignment: Default::default(),
      maxDescriptorBufferBindings: Default::default(),
      maxResourceDescriptorBufferBindings: Default::default(),
      maxSamplerDescriptorBufferBindings: Default::default(),
      maxEmbeddedImmutableSamplerBindings: Default::default(),
      maxEmbeddedImmutableSamplers: Default::default(),
      bufferCaptureReplayDescriptorDataSize: Default::default(),
      imageCaptureReplayDescriptorDataSize: Default::default(),
      imageViewCaptureReplayDescriptorDataSize: Default::default(),
      samplerCaptureReplayDescriptorDataSize: Default::default(),
      accelerationStructureCaptureReplayDescriptorDataSize: Default::default(),
      samplerDescriptorSize: Default::default(),
      combinedImageSamplerDescriptorSize: Default::default(),
      sampledImageDescriptorSize: Default::default(),
      storageImageDescriptorSize: Default::default(),
      uniformTexelBufferDescriptorSize: Default::default(),
      robustUniformTexelBufferDescriptorSize: Default::default(),
      storageTexelBufferDescriptorSize: Default::default(),
      robustStorageTexelBufferDescriptorSize: Default::default(),
      uniformBufferDescriptorSize: Default::default(),
      robustUniformBufferDescriptorSize: Default::default(),
      storageBufferDescriptorSize: Default::default(),
      robustStorageBufferDescriptorSize: Default::default(),
      inputAttachmentDescriptorSize: Default::default(),
      accelerationStructureDescriptorSize: Default::default(),
      maxSamplerDescriptorBufferRange: Default::default(),
      maxResourceDescriptorBufferRange: Default::default(),
      samplerDescriptorBufferAddressSpaceSize: Default::default(),
      resourceDescriptorBufferAddressSpaceSize: Default::default(),
      descriptorBufferAddressSpaceSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorIndexingFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
  pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
  pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
  pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
  pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
  pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
  pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
  pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
  pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
  pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
  pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
  pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
  pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
  pub descriptorBindingPartiallyBound: VkBool32,
  pub descriptorBindingVariableDescriptorCount: VkBool32,
  pub runtimeDescriptorArray: VkBool32,
}
impl Default for VkPhysicalDeviceDescriptorIndexingFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES,
      pNext: core::ptr::null_mut(),
      shaderInputAttachmentArrayDynamicIndexing: Default::default(),
      shaderUniformTexelBufferArrayDynamicIndexing: Default::default(),
      shaderStorageTexelBufferArrayDynamicIndexing: Default::default(),
      shaderUniformBufferArrayNonUniformIndexing: Default::default(),
      shaderSampledImageArrayNonUniformIndexing: Default::default(),
      shaderStorageBufferArrayNonUniformIndexing: Default::default(),
      shaderStorageImageArrayNonUniformIndexing: Default::default(),
      shaderInputAttachmentArrayNonUniformIndexing: Default::default(),
      shaderUniformTexelBufferArrayNonUniformIndexing: Default::default(),
      shaderStorageTexelBufferArrayNonUniformIndexing: Default::default(),
      descriptorBindingUniformBufferUpdateAfterBind: Default::default(),
      descriptorBindingSampledImageUpdateAfterBind: Default::default(),
      descriptorBindingStorageImageUpdateAfterBind: Default::default(),
      descriptorBindingStorageBufferUpdateAfterBind: Default::default(),
      descriptorBindingUniformTexelBufferUpdateAfterBind: Default::default(),
      descriptorBindingStorageTexelBufferUpdateAfterBind: Default::default(),
      descriptorBindingUpdateUnusedWhilePending: Default::default(),
      descriptorBindingPartiallyBound: Default::default(),
      descriptorBindingVariableDescriptorCount: Default::default(),
      runtimeDescriptorArray: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorIndexingProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorIndexingProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxUpdateAfterBindDescriptorsInAllPools: u32,
  /// * Limit Type: bitmask
  pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub robustBufferAccessUpdateAfterBind: VkBool32,
  /// * Limit Type: bitmask
  pub quadDivergentImplicitLod: VkBool32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
  /// * Limit Type: max
  pub maxPerStageUpdateAfterBindResources: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindSamplers: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
}
impl Default for VkPhysicalDeviceDescriptorIndexingProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES,
      pNext: core::ptr::null_mut(),
      maxUpdateAfterBindDescriptorsInAllPools: Default::default(),
      shaderUniformBufferArrayNonUniformIndexingNative: Default::default(),
      shaderSampledImageArrayNonUniformIndexingNative: Default::default(),
      shaderStorageBufferArrayNonUniformIndexingNative: Default::default(),
      shaderStorageImageArrayNonUniformIndexingNative: Default::default(),
      shaderInputAttachmentArrayNonUniformIndexingNative: Default::default(),
      robustBufferAccessUpdateAfterBind: Default::default(),
      quadDivergentImplicitLod: Default::default(),
      maxPerStageDescriptorUpdateAfterBindSamplers: Default::default(),
      maxPerStageDescriptorUpdateAfterBindUniformBuffers: Default::default(),
      maxPerStageDescriptorUpdateAfterBindStorageBuffers: Default::default(),
      maxPerStageDescriptorUpdateAfterBindSampledImages: Default::default(),
      maxPerStageDescriptorUpdateAfterBindStorageImages: Default::default(),
      maxPerStageDescriptorUpdateAfterBindInputAttachments: Default::default(),
      maxPerStageUpdateAfterBindResources: Default::default(),
      maxDescriptorSetUpdateAfterBindSamplers: Default::default(),
      maxDescriptorSetUpdateAfterBindUniformBuffers: Default::default(),
      maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: Default::default(),
      maxDescriptorSetUpdateAfterBindStorageBuffers: Default::default(),
      maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: Default::default(),
      maxDescriptorSetUpdateAfterBindSampledImages: Default::default(),
      maxDescriptorSetUpdateAfterBindStorageImages: Default::default(),
      maxDescriptorSetUpdateAfterBindInputAttachments: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub descriptorSetHostMapping: VkBool32,
}
impl Default for VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE,
      pNext: core::ptr::null_mut(),
      descriptorSetHostMapping: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub deviceGeneratedCommands: VkBool32,
}
impl Default for VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      deviceGeneratedCommands: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxGraphicsShaderGroupCount: u32,
  /// * Limit Type: max
  pub maxIndirectSequenceCount: u32,
  /// * Limit Type: max
  pub maxIndirectCommandsTokenCount: u32,
  /// * Limit Type: max
  pub maxIndirectCommandsStreamCount: u32,
  /// * Limit Type: max
  pub maxIndirectCommandsTokenOffset: u32,
  /// * Limit Type: max
  pub maxIndirectCommandsStreamStride: u32,
  /// * Limit Type: min
  pub minSequencesCountBufferOffsetAlignment: u32,
  /// * Limit Type: min
  pub minSequencesIndexBufferOffsetAlignment: u32,
  /// * Limit Type: min
  pub minIndirectCommandsBufferOffsetAlignment: u32,
}
impl Default for VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      maxGraphicsShaderGroupCount: Default::default(),
      maxIndirectSequenceCount: Default::default(),
      maxIndirectCommandsTokenCount: Default::default(),
      maxIndirectCommandsStreamCount: Default::default(),
      maxIndirectCommandsTokenOffset: Default::default(),
      maxIndirectCommandsStreamStride: Default::default(),
      minSequencesCountBufferOffsetAlignment: Default::default(),
      minSequencesIndexBufferOffsetAlignment: Default::default(),
      minIndirectCommandsBufferOffsetAlignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDeviceMemoryReportFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub deviceMemoryReport: VkBool32,
}
impl Default for VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      deviceMemoryReport: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDiagnosticsConfigFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub diagnosticsConfig: VkBool32,
}
impl Default for VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      diagnosticsConfig: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDiscardRectanglePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// max number of active discard rectangles
  /// * Limit Type: max
  pub maxDiscardRectangles: u32,
}
impl Default for VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      maxDiscardRectangles: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDriverProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDriverProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDriverProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub driverID: VkDriverId,
  /// * Limit Type: exact
  pub driverName: [u8; VK_MAX_DRIVER_NAME_SIZE],
  /// * Limit Type: exact
  pub driverInfo: [u8; VK_MAX_DRIVER_INFO_SIZE],
  /// * Limit Type: exact
  pub conformanceVersion: VkConformanceVersion,
}
impl Default for VkPhysicalDeviceDriverProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES,
      pNext: core::ptr::null_mut(),
      driverID: Default::default(),
      driverName: [Default::default(); VK_MAX_DRIVER_NAME_SIZE],
      driverInfo: [Default::default(); VK_MAX_DRIVER_INFO_SIZE],
      conformanceVersion: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDrmPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDrmPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDrmPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub hasPrimary: VkBool32,
  /// * Limit Type: bitmask
  pub hasRender: VkBool32,
  /// * Limit Type: noauto
  pub primaryMajor: i64,
  /// * Limit Type: noauto
  pub primaryMinor: i64,
  /// * Limit Type: noauto
  pub renderMajor: i64,
  /// * Limit Type: noauto
  pub renderMinor: i64,
}
impl Default for VkPhysicalDeviceDrmPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      hasPrimary: Default::default(),
      hasRender: Default::default(),
      primaryMajor: Default::default(),
      primaryMinor: Default::default(),
      renderMajor: Default::default(),
      renderMinor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceDynamicRenderingFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDynamicRenderingFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub dynamicRendering: VkBool32,
}
impl Default for VkPhysicalDeviceDynamicRenderingFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES,
      pNext: core::ptr::null_mut(),
      dynamicRendering: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExclusiveScissorFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub exclusiveScissor: VkBool32,
}
impl Default for VkPhysicalDeviceExclusiveScissorFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      exclusiveScissor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExtendedDynamicState2FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub extendedDynamicState2: VkBool32,
  pub extendedDynamicState2LogicOp: VkBool32,
  pub extendedDynamicState2PatchControlPoints: VkBool32,
}
impl Default for VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      extendedDynamicState2: Default::default(),
      extendedDynamicState2LogicOp: Default::default(),
      extendedDynamicState2PatchControlPoints: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExtendedDynamicState3FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3FeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub extendedDynamicState3TessellationDomainOrigin: VkBool32,
  pub extendedDynamicState3DepthClampEnable: VkBool32,
  pub extendedDynamicState3PolygonMode: VkBool32,
  pub extendedDynamicState3RasterizationSamples: VkBool32,
  pub extendedDynamicState3SampleMask: VkBool32,
  pub extendedDynamicState3AlphaToCoverageEnable: VkBool32,
  pub extendedDynamicState3AlphaToOneEnable: VkBool32,
  pub extendedDynamicState3LogicOpEnable: VkBool32,
  pub extendedDynamicState3ColorBlendEnable: VkBool32,
  pub extendedDynamicState3ColorBlendEquation: VkBool32,
  pub extendedDynamicState3ColorWriteMask: VkBool32,
  pub extendedDynamicState3RasterizationStream: VkBool32,
  pub extendedDynamicState3ConservativeRasterizationMode: VkBool32,
  pub extendedDynamicState3ExtraPrimitiveOverestimationSize: VkBool32,
  pub extendedDynamicState3DepthClipEnable: VkBool32,
  pub extendedDynamicState3SampleLocationsEnable: VkBool32,
  pub extendedDynamicState3ColorBlendAdvanced: VkBool32,
  pub extendedDynamicState3ProvokingVertexMode: VkBool32,
  pub extendedDynamicState3LineRasterizationMode: VkBool32,
  pub extendedDynamicState3LineStippleEnable: VkBool32,
  pub extendedDynamicState3DepthClipNegativeOneToOne: VkBool32,
  pub extendedDynamicState3ViewportWScalingEnable: VkBool32,
  pub extendedDynamicState3ViewportSwizzle: VkBool32,
  pub extendedDynamicState3CoverageToColorEnable: VkBool32,
  pub extendedDynamicState3CoverageToColorLocation: VkBool32,
  pub extendedDynamicState3CoverageModulationMode: VkBool32,
  pub extendedDynamicState3CoverageModulationTableEnable: VkBool32,
  pub extendedDynamicState3CoverageModulationTable: VkBool32,
  pub extendedDynamicState3CoverageReductionMode: VkBool32,
  pub extendedDynamicState3RepresentativeFragmentTestEnable: VkBool32,
  pub extendedDynamicState3ShadingRateImageEnable: VkBool32,
}
impl Default for VkPhysicalDeviceExtendedDynamicState3FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      extendedDynamicState3TessellationDomainOrigin: Default::default(),
      extendedDynamicState3DepthClampEnable: Default::default(),
      extendedDynamicState3PolygonMode: Default::default(),
      extendedDynamicState3RasterizationSamples: Default::default(),
      extendedDynamicState3SampleMask: Default::default(),
      extendedDynamicState3AlphaToCoverageEnable: Default::default(),
      extendedDynamicState3AlphaToOneEnable: Default::default(),
      extendedDynamicState3LogicOpEnable: Default::default(),
      extendedDynamicState3ColorBlendEnable: Default::default(),
      extendedDynamicState3ColorBlendEquation: Default::default(),
      extendedDynamicState3ColorWriteMask: Default::default(),
      extendedDynamicState3RasterizationStream: Default::default(),
      extendedDynamicState3ConservativeRasterizationMode: Default::default(),
      extendedDynamicState3ExtraPrimitiveOverestimationSize: Default::default(),
      extendedDynamicState3DepthClipEnable: Default::default(),
      extendedDynamicState3SampleLocationsEnable: Default::default(),
      extendedDynamicState3ColorBlendAdvanced: Default::default(),
      extendedDynamicState3ProvokingVertexMode: Default::default(),
      extendedDynamicState3LineRasterizationMode: Default::default(),
      extendedDynamicState3LineStippleEnable: Default::default(),
      extendedDynamicState3DepthClipNegativeOneToOne: Default::default(),
      extendedDynamicState3ViewportWScalingEnable: Default::default(),
      extendedDynamicState3ViewportSwizzle: Default::default(),
      extendedDynamicState3CoverageToColorEnable: Default::default(),
      extendedDynamicState3CoverageToColorLocation: Default::default(),
      extendedDynamicState3CoverageModulationMode: Default::default(),
      extendedDynamicState3CoverageModulationTableEnable: Default::default(),
      extendedDynamicState3CoverageModulationTable: Default::default(),
      extendedDynamicState3CoverageReductionMode: Default::default(),
      extendedDynamicState3RepresentativeFragmentTestEnable: Default::default(),
      extendedDynamicState3ShadingRateImageEnable: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExtendedDynamicState3PropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState3PropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3PropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub dynamicPrimitiveTopologyUnrestricted: VkBool32,
}
impl Default for VkPhysicalDeviceExtendedDynamicState3PropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      dynamicPrimitiveTopologyUnrestricted: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExtendedDynamicStateFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub extendedDynamicState: VkBool32,
}
impl Default for VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      extendedDynamicState: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalBufferInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalBufferInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkBufferCreateFlags,
  pub usage: VkBufferUsageFlags,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkPhysicalDeviceExternalBufferInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      usage: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalFenceInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalFenceInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub handleType: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkPhysicalDeviceExternalFenceInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO,
      pNext: core::ptr::null(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalImageFormatInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalImageFormatInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl Default for VkPhysicalDeviceExternalImageFormatInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO,
      pNext: core::ptr::null(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalMemoryHostPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: min,pot
  pub minImportedHostPointerAlignment: VkDeviceSize,
}
impl Default for VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      minImportedHostPointerAlignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalMemoryRDMAFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryRDMAFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub externalMemoryRDMA: VkBool32,
}
impl Default for VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      externalMemoryRDMA: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalMemorySciBufFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemorySciBufFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalMemorySciBufFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub sciBufImport: VkBool32,
  pub sciBufExport: VkBool32,
}
impl Default for VkPhysicalDeviceExternalMemorySciBufFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      sciBufImport: Default::default(),
      sciBufExport: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalSciSync2FeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSciSync2FeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSync2FeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub sciSyncFence: VkBool32,
  pub sciSyncSemaphore2: VkBool32,
  pub sciSyncImport: VkBool32,
  pub sciSyncExport: VkBool32,
}
impl Default for VkPhysicalDeviceExternalSciSync2FeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      sciSyncFence: Default::default(),
      sciSyncSemaphore2: Default::default(),
      sciSyncImport: Default::default(),
      sciSyncExport: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalSciSyncFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSciSyncFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSyncFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub sciSyncFence: VkBool32,
  pub sciSyncSemaphore: VkBool32,
  pub sciSyncImport: VkBool32,
  pub sciSyncExport: VkBool32,
}
impl Default for VkPhysicalDeviceExternalSciSyncFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      sciSyncFence: Default::default(),
      sciSyncSemaphore: Default::default(),
      sciSyncImport: Default::default(),
      sciSyncExport: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceExternalSemaphoreInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkPhysicalDeviceExternalSemaphoreInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO,
      pNext: core::ptr::null(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFaultFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFaultFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFaultFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FAULT_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub deviceFault: VkBool32,
  pub deviceFaultVendorBinary: VkBool32,
}
impl Default for VkPhysicalDeviceFaultFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FAULT_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      deviceFault: Default::default(),
      deviceFaultVendorBinary: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
  /// out of bounds buffer accesses are well defined
  pub robustBufferAccess: VkBool32,
  /// full 32-bit range of indices for indexed draw calls
  pub fullDrawIndexUint32: VkBool32,
  /// image views which are arrays of cube maps
  pub imageCubeArray: VkBool32,
  /// blending operations are controlled per-attachment
  pub independentBlend: VkBool32,
  /// geometry stage
  pub geometryShader: VkBool32,
  /// tessellation control and evaluation stage
  pub tessellationShader: VkBool32,
  /// per-sample shading and interpolation
  pub sampleRateShading: VkBool32,
  /// blend operations which take two sources
  pub dualSrcBlend: VkBool32,
  /// logic operations
  pub logicOp: VkBool32,
  /// multi draw indirect
  pub multiDrawIndirect: VkBool32,
  /// indirect drawing can use non-zero firstInstance
  pub drawIndirectFirstInstance: VkBool32,
  /// depth clamping
  pub depthClamp: VkBool32,
  /// depth bias clamping
  pub depthBiasClamp: VkBool32,
  /// point and wireframe fill modes
  pub fillModeNonSolid: VkBool32,
  /// depth bounds test
  pub depthBounds: VkBool32,
  /// lines with width greater than 1
  pub wideLines: VkBool32,
  /// points with size greater than 1
  pub largePoints: VkBool32,
  /// the fragment alpha component can be forced to maximum representable alpha value
  pub alphaToOne: VkBool32,
  /// viewport arrays
  pub multiViewport: VkBool32,
  /// anisotropic sampler filtering
  pub samplerAnisotropy: VkBool32,
  /// ETC texture compression formats
  pub textureCompressionETC2: VkBool32,
  /// ASTC LDR texture compression formats
  pub textureCompressionASTC_LDR: VkBool32,
  /// BC1-7 texture compressed formats
  pub textureCompressionBC: VkBool32,
  /// precise occlusion queries returning actual sample counts
  pub occlusionQueryPrecise: VkBool32,
  /// pipeline statistics query
  pub pipelineStatisticsQuery: VkBool32,
  /// stores and atomic ops on storage buffers and images are supported in vertex, tessellation, and geometry stages
  pub vertexPipelineStoresAndAtomics: VkBool32,
  /// stores and atomic ops on storage buffers and images are supported in the fragment stage
  pub fragmentStoresAndAtomics: VkBool32,
  /// tessellation and geometry stages can export point size
  pub shaderTessellationAndGeometryPointSize: VkBool32,
  /// image gather with run-time values and independent offsets
  pub shaderImageGatherExtended: VkBool32,
  /// the extended set of formats can be used for storage images
  pub shaderStorageImageExtendedFormats: VkBool32,
  /// multisample images can be used for storage images
  pub shaderStorageImageMultisample: VkBool32,
  /// read from storage image does not require format qualifier
  pub shaderStorageImageReadWithoutFormat: VkBool32,
  /// write to storage image does not require format qualifier
  pub shaderStorageImageWriteWithoutFormat: VkBool32,
  /// arrays of uniform buffers can be accessed with dynamically uniform indices
  pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
  /// arrays of sampled images can be accessed with dynamically uniform indices
  pub shaderSampledImageArrayDynamicIndexing: VkBool32,
  /// arrays of storage buffers can be accessed with dynamically uniform indices
  pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
  /// arrays of storage images can be accessed with dynamically uniform indices
  pub shaderStorageImageArrayDynamicIndexing: VkBool32,
  /// clip distance in shaders
  pub shaderClipDistance: VkBool32,
  /// cull distance in shaders
  pub shaderCullDistance: VkBool32,
  /// 64-bit floats (doubles) in shaders
  pub shaderFloat64: VkBool32,
  /// 64-bit integers in shaders
  pub shaderInt64: VkBool32,
  /// 16-bit integers in shaders
  pub shaderInt16: VkBool32,
  /// shader can use texture operations that return resource residency information (requires sparseNonResident support)
  pub shaderResourceResidency: VkBool32,
  /// shader can use texture operations that specify minimum resource LOD
  pub shaderResourceMinLod: VkBool32,
  /// Sparse resources support: Resource memory can be managed at opaque page level rather than object level
  pub sparseBinding: VkBool32,
  /// Sparse resources support: GPU can access partially resident buffers
  pub sparseResidencyBuffer: VkBool32,
  /// Sparse resources support: GPU can access partially resident 2D (non-MSAA non-depth/stencil) images
  pub sparseResidencyImage2D: VkBool32,
  /// Sparse resources support: GPU can access partially resident 3D images
  pub sparseResidencyImage3D: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images with 2 samples
  pub sparseResidency2Samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images with 4 samples
  pub sparseResidency4Samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images with 8 samples
  pub sparseResidency8Samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images with 16 samples
  pub sparseResidency16Samples: VkBool32,
  /// Sparse resources support: GPU can correctly access data aliased into multiple locations (opt-in)
  pub sparseResidencyAliased: VkBool32,
  /// multisample rate must be the same for all pipelines in a subpass
  pub variableMultisampleRate: VkBool32,
  /// Queries may be inherited from primary to secondary command buffers
  pub inheritedQueries: VkBool32,
}
impl Default for VkPhysicalDeviceFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      robustBufferAccess: Default::default(),
      fullDrawIndexUint32: Default::default(),
      imageCubeArray: Default::default(),
      independentBlend: Default::default(),
      geometryShader: Default::default(),
      tessellationShader: Default::default(),
      sampleRateShading: Default::default(),
      dualSrcBlend: Default::default(),
      logicOp: Default::default(),
      multiDrawIndirect: Default::default(),
      drawIndirectFirstInstance: Default::default(),
      depthClamp: Default::default(),
      depthBiasClamp: Default::default(),
      fillModeNonSolid: Default::default(),
      depthBounds: Default::default(),
      wideLines: Default::default(),
      largePoints: Default::default(),
      alphaToOne: Default::default(),
      multiViewport: Default::default(),
      samplerAnisotropy: Default::default(),
      textureCompressionETC2: Default::default(),
      textureCompressionASTC_LDR: Default::default(),
      textureCompressionBC: Default::default(),
      occlusionQueryPrecise: Default::default(),
      pipelineStatisticsQuery: Default::default(),
      vertexPipelineStoresAndAtomics: Default::default(),
      fragmentStoresAndAtomics: Default::default(),
      shaderTessellationAndGeometryPointSize: Default::default(),
      shaderImageGatherExtended: Default::default(),
      shaderStorageImageExtendedFormats: Default::default(),
      shaderStorageImageMultisample: Default::default(),
      shaderStorageImageReadWithoutFormat: Default::default(),
      shaderStorageImageWriteWithoutFormat: Default::default(),
      shaderUniformBufferArrayDynamicIndexing: Default::default(),
      shaderSampledImageArrayDynamicIndexing: Default::default(),
      shaderStorageBufferArrayDynamicIndexing: Default::default(),
      shaderStorageImageArrayDynamicIndexing: Default::default(),
      shaderClipDistance: Default::default(),
      shaderCullDistance: Default::default(),
      shaderFloat64: Default::default(),
      shaderInt64: Default::default(),
      shaderInt16: Default::default(),
      shaderResourceResidency: Default::default(),
      shaderResourceMinLod: Default::default(),
      sparseBinding: Default::default(),
      sparseResidencyBuffer: Default::default(),
      sparseResidencyImage2D: Default::default(),
      sparseResidencyImage3D: Default::default(),
      sparseResidency2Samples: Default::default(),
      sparseResidency4Samples: Default::default(),
      sparseResidency8Samples: Default::default(),
      sparseResidency16Samples: Default::default(),
      sparseResidencyAliased: Default::default(),
      variableMultisampleRate: Default::default(),
      inheritedQueries: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFeatures2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures2.html)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub features: VkPhysicalDeviceFeatures,
}
impl Default for VkPhysicalDeviceFeatures2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
      pNext: core::ptr::null_mut(),
      features: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFloatControlsProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFloatControlsProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFloatControlsProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
  /// * Limit Type: exact
  pub roundingModeIndependence: VkShaderFloatControlsIndependence,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shaderDenormPreserveFloat16: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shaderDenormPreserveFloat32: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shaderDenormPreserveFloat64: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shaderDenormFlushToZeroFloat16: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shaderDenormFlushToZeroFloat32: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shaderDenormFlushToZeroFloat64: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTEFloat16: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTEFloat32: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTEFloat64: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTZFloat16: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTZFloat32: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTZFloat64: VkBool32,
}
impl Default for VkPhysicalDeviceFloatControlsProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES,
      pNext: core::ptr::null_mut(),
      denormBehaviorIndependence: Default::default(),
      roundingModeIndependence: Default::default(),
      shaderSignedZeroInfNanPreserveFloat16: Default::default(),
      shaderSignedZeroInfNanPreserveFloat32: Default::default(),
      shaderSignedZeroInfNanPreserveFloat64: Default::default(),
      shaderDenormPreserveFloat16: Default::default(),
      shaderDenormPreserveFloat32: Default::default(),
      shaderDenormPreserveFloat64: Default::default(),
      shaderDenormFlushToZeroFloat16: Default::default(),
      shaderDenormFlushToZeroFloat32: Default::default(),
      shaderDenormFlushToZeroFloat64: Default::default(),
      shaderRoundingModeRTEFloat16: Default::default(),
      shaderRoundingModeRTEFloat32: Default::default(),
      shaderRoundingModeRTEFloat64: Default::default(),
      shaderRoundingModeRTZFloat16: Default::default(),
      shaderRoundingModeRTZFloat32: Default::default(),
      shaderRoundingModeRTZFloat64: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMap2FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2FeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub fragmentDensityMapDeferred: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      fragmentDensityMapDeferred: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMap2PropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub subsampledLoads: VkBool32,
  /// * Limit Type: exact
  pub subsampledCoarseReconstructionEarlyAccess: VkBool32,
  /// * Limit Type: max
  pub maxSubsampledArrayLayers: u32,
  /// * Limit Type: max
  pub maxDescriptorSetSubsampledSamplers: u32,
}
impl Default for VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      subsampledLoads: Default::default(),
      subsampledCoarseReconstructionEarlyAccess: Default::default(),
      maxSubsampledArrayLayers: Default::default(),
      maxDescriptorSetSubsampledSamplers: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMapFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub fragmentDensityMap: VkBool32,
  pub fragmentDensityMapDynamic: VkBool32,
  pub fragmentDensityMapNonSubsampledImages: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      fragmentDensityMap: Default::default(),
      fragmentDensityMapDynamic: Default::default(),
      fragmentDensityMapNonSubsampledImages: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub fragmentDensityMapOffset: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM,
      pNext: core::ptr::null_mut(),
      fragmentDensityMapOffset: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: min,mul
  pub fragmentDensityOffsetGranularity: VkExtent2D,
}
impl Default for VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM,
      pNext: core::ptr::null_mut(),
      fragmentDensityOffsetGranularity: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentDensityMapPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: min
  pub minFragmentDensityTexelSize: VkExtent2D,
  /// * Limit Type: max
  pub maxFragmentDensityTexelSize: VkExtent2D,
  /// * Limit Type: bitmask
  pub fragmentDensityInvocations: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      minFragmentDensityTexelSize: Default::default(),
      maxFragmentDensityTexelSize: Default::default(),
      fragmentDensityInvocations: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub fragmentShaderBarycentric: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      fragmentShaderBarycentric: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub triStripVertexOrderIndependentOfProvokingVertex: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      triStripVertexOrderIndependentOfProvokingVertex: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT`
  pub sType: VkStructureType,
  /// Pointer to next structure
  /// * Optional: true
  pub pNext: *mut c_void,
  pub fragmentShaderSampleInterlock: VkBool32,
  pub fragmentShaderPixelInterlock: VkBool32,
  pub fragmentShaderShadingRateInterlock: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      fragmentShaderSampleInterlock: Default::default(),
      fragmentShaderPixelInterlock: Default::default(),
      fragmentShaderShadingRateInterlock: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub fragmentShadingRateEnums: VkBool32,
  pub supersampleFragmentShadingRates: VkBool32,
  pub noInvocationFragmentShadingRates: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      fragmentShadingRateEnums: Default::default(),
      supersampleFragmentShadingRates: Default::default(),
      noInvocationFragmentShadingRates: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxFragmentShadingRateInvocationCount: VkSampleCountFlagBits,
}
impl Default for VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      maxFragmentShadingRateInvocationCount: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRateFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub pipelineFragmentShadingRate: VkBool32,
  pub primitiveFragmentShadingRate: VkBool32,
  pub attachmentFragmentShadingRate: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      pipelineFragmentShadingRate: Default::default(),
      primitiveFragmentShadingRate: Default::default(),
      attachmentFragmentShadingRate: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRateKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub sampleCounts: VkSampleCountFlags,
  pub fragmentSize: VkExtent2D,
}
impl Default for VkPhysicalDeviceFragmentShadingRateKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR,
      pNext: core::ptr::null_mut(),
      sampleCounts: Default::default(),
      fragmentSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceFragmentShadingRatePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: min
  pub minFragmentShadingRateAttachmentTexelSize: VkExtent2D,
  /// * Limit Type: max
  pub maxFragmentShadingRateAttachmentTexelSize: VkExtent2D,
  /// * Limit Type: max,pot
  pub maxFragmentShadingRateAttachmentTexelSizeAspectRatio: u32,
  /// * Limit Type: bitmask
  pub primitiveFragmentShadingRateWithMultipleViewports: VkBool32,
  /// * Limit Type: bitmask
  pub layeredShadingRateAttachments: VkBool32,
  /// * Limit Type: bitmask
  pub fragmentShadingRateNonTrivialCombinerOps: VkBool32,
  /// * Limit Type: max
  pub maxFragmentSize: VkExtent2D,
  /// * Limit Type: max,pot
  pub maxFragmentSizeAspectRatio: u32,
  /// * Limit Type: max
  pub maxFragmentShadingRateCoverageSamples: u32,
  /// * Limit Type: max
  pub maxFragmentShadingRateRasterizationSamples: VkSampleCountFlagBits,
  /// * Limit Type: bitmask
  pub fragmentShadingRateWithShaderDepthStencilWrites: VkBool32,
  /// * Limit Type: bitmask
  pub fragmentShadingRateWithSampleMask: VkBool32,
  /// * Limit Type: bitmask
  pub fragmentShadingRateWithShaderSampleMask: VkBool32,
  /// * Limit Type: bitmask
  pub fragmentShadingRateWithConservativeRasterization: VkBool32,
  /// * Limit Type: bitmask
  pub fragmentShadingRateWithFragmentShaderInterlock: VkBool32,
  /// * Limit Type: bitmask
  pub fragmentShadingRateWithCustomSampleLocations: VkBool32,
  /// * Limit Type: bitmask
  pub fragmentShadingRateStrictMultiplyCombiner: VkBool32,
}
impl Default for VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      minFragmentShadingRateAttachmentTexelSize: Default::default(),
      maxFragmentShadingRateAttachmentTexelSize: Default::default(),
      maxFragmentShadingRateAttachmentTexelSizeAspectRatio: Default::default(),
      primitiveFragmentShadingRateWithMultipleViewports: Default::default(),
      layeredShadingRateAttachments: Default::default(),
      fragmentShadingRateNonTrivialCombinerOps: Default::default(),
      maxFragmentSize: Default::default(),
      maxFragmentSizeAspectRatio: Default::default(),
      maxFragmentShadingRateCoverageSamples: Default::default(),
      maxFragmentShadingRateRasterizationSamples: Default::default(),
      fragmentShadingRateWithShaderDepthStencilWrites: Default::default(),
      fragmentShadingRateWithSampleMask: Default::default(),
      fragmentShadingRateWithShaderSampleMask: Default::default(),
      fragmentShadingRateWithConservativeRasterization: Default::default(),
      fragmentShadingRateWithFragmentShaderInterlock: Default::default(),
      fragmentShadingRateWithCustomSampleLocations: Default::default(),
      fragmentShadingRateStrictMultiplyCombiner: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub globalPriorityQuery: VkBool32,
}
impl Default for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      globalPriorityQuery: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub graphicsPipelineLibrary: VkBool32,
}
impl Default for VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      graphicsPipelineLibrary: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub graphicsPipelineLibraryFastLinking: VkBool32,
  /// * Limit Type: bitmask
  pub graphicsPipelineLibraryIndependentInterpolationDecoration: VkBool32,
}
impl Default for VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      graphicsPipelineLibraryFastLinking: Default::default(),
      graphicsPipelineLibraryIndependentInterpolationDecoration: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceGroupProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGroupProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceGroupProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub physicalDeviceCount: u32,
  pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE],
  pub subsetAllocation: VkBool32,
}
impl Default for VkPhysicalDeviceGroupProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES,
      pNext: core::ptr::null_mut(),
      physicalDeviceCount: Default::default(),
      physicalDevices: [Default::default(); VK_MAX_DEVICE_GROUP_SIZE],
      subsetAllocation: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceHostQueryResetFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceHostQueryResetFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceHostQueryResetFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub hostQueryReset: VkBool32,
}
impl Default for VkPhysicalDeviceHostQueryResetFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES,
      pNext: core::ptr::null_mut(),
      hostQueryReset: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceIDProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIDProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceIDProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: noauto
  pub deviceUUID: [u8; VK_UUID_SIZE],
  /// * Limit Type: noauto
  pub driverUUID: [u8; VK_UUID_SIZE],
  /// * Limit Type: noauto
  pub deviceLUID: [u8; VK_LUID_SIZE],
  /// * Limit Type: noauto
  pub deviceNodeMask: u32,
  /// * Limit Type: noauto
  pub deviceLUIDValid: VkBool32,
}
impl Default for VkPhysicalDeviceIDProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES,
      pNext: core::ptr::null_mut(),
      deviceUUID: [Default::default(); VK_UUID_SIZE],
      driverUUID: [Default::default(); VK_UUID_SIZE],
      deviceLUID: [Default::default(); VK_LUID_SIZE],
      deviceNodeMask: Default::default(),
      deviceLUIDValid: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImage2DViewOf3DFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImage2DViewOf3DFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub image2DViewOf3D: VkBool32,
  pub sampler2DViewOf3D: VkBool32,
}
impl Default for VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      image2DViewOf3D: Default::default(),
      sampler2DViewOf3D: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageCompressionControlFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageCompressionControlFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub imageCompressionControl: VkBool32,
}
impl Default for VkPhysicalDeviceImageCompressionControlFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      imageCompressionControl: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub imageCompressionControlSwapchain: VkBool32,
}
impl Default for VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      imageCompressionControlSwapchain: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageDrmFormatModifierInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub drmFormatModifier: u64,
  pub sharingMode: VkSharingMode,
  /// * Optional: true
  pub queueFamilyIndexCount: u32,
  /// * Len: `queueFamilyIndexCount`
  /// * No Auto-Validity
  pub pQueueFamilyIndices: *const u32,
}
impl Default for VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT,
      pNext: core::ptr::null(),
      drmFormatModifier: Default::default(),
      sharingMode: Default::default(),
      queueFamilyIndexCount: Default::default(),
      pQueueFamilyIndices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageFormatInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageFormatInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2,
      pNext: core::ptr::null(),
      format: Default::default(),
      ty: Default::default(),
      tiling: Default::default(),
      usage: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageProcessingFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageProcessingFeaturesQCOM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingFeaturesQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub textureSampleWeighted: VkBool32,
  pub textureBoxFilter: VkBool32,
  pub textureBlockMatch: VkBool32,
}
impl Default for VkPhysicalDeviceImageProcessingFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM,
      pNext: core::ptr::null_mut(),
      textureSampleWeighted: Default::default(),
      textureBoxFilter: Default::default(),
      textureBlockMatch: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageProcessingPropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageProcessingPropertiesQCOM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingPropertiesQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  /// * Limit Type: max
  pub maxWeightFilterPhases: u32,
  /// * Optional: true
  /// * Limit Type: max
  pub maxWeightFilterDimension: VkExtent2D,
  /// * Optional: true
  /// * Limit Type: max
  pub maxBlockMatchRegion: VkExtent2D,
  /// * Optional: true
  /// * Limit Type: max
  pub maxBoxFilterBlockSize: VkExtent2D,
}
impl Default for VkPhysicalDeviceImageProcessingPropertiesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM,
      pNext: core::ptr::null_mut(),
      maxWeightFilterPhases: Default::default(),
      maxWeightFilterDimension: Default::default(),
      maxBlockMatchRegion: Default::default(),
      maxBoxFilterBlockSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageRobustnessFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageRobustnessFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageRobustnessFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub robustImageAccess: VkBool32,
}
impl Default for VkPhysicalDeviceImageRobustnessFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES,
      pNext: core::ptr::null_mut(),
      robustImageAccess: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub imageSlicedViewOf3D: VkBool32,
}
impl Default for VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      imageSlicedViewOf3D: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageViewImageFormatInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub imageViewType: VkImageViewType,
}
impl Default for VkPhysicalDeviceImageViewImageFormatInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT,
      pNext: core::ptr::null_mut(),
      imageViewType: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImageViewMinLodFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub minLod: VkBool32,
}
impl Default for VkPhysicalDeviceImageViewMinLodFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      minLod: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceImagelessFramebufferFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub imagelessFramebuffer: VkBool32,
}
impl Default for VkPhysicalDeviceImagelessFramebufferFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES,
      pNext: core::ptr::null_mut(),
      imagelessFramebuffer: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceIndexTypeUint8FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub indexTypeUint8: VkBool32,
}
impl Default for VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      indexTypeUint8: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceInheritedViewportScissorFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub inheritedViewportScissor2D: VkBool32,
}
impl Default for VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      inheritedViewportScissor2D: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceInlineUniformBlockFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInlineUniformBlockFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub inlineUniformBlock: VkBool32,
  pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
}
impl Default for VkPhysicalDeviceInlineUniformBlockFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES,
      pNext: core::ptr::null_mut(),
      inlineUniformBlock: Default::default(),
      descriptorBindingInlineUniformBlockUpdateAfterBind: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceInlineUniformBlockProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInlineUniformBlockProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxInlineUniformBlockSize: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorInlineUniformBlocks: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
  /// * Limit Type: max
  pub maxDescriptorSetInlineUniformBlocks: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
}
impl Default for VkPhysicalDeviceInlineUniformBlockProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES,
      pNext: core::ptr::null_mut(),
      maxInlineUniformBlockSize: Default::default(),
      maxPerStageDescriptorInlineUniformBlocks: Default::default(),
      maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: Default::default(),
      maxDescriptorSetInlineUniformBlocks: Default::default(),
      maxDescriptorSetUpdateAfterBindInlineUniformBlocks: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceInvocationMaskFeaturesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub invocationMask: VkBool32,
}
impl Default for VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI,
      pNext: core::ptr::null_mut(),
      invocationMask: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLegacyDitheringFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLegacyDitheringFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLegacyDitheringFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub legacyDithering: VkBool32,
}
impl Default for VkPhysicalDeviceLegacyDitheringFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      legacyDithering: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLimits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLimits.html)
///
/// compute stage limits
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
  /// max 1D image dimension
  /// * Limit Type: max
  pub maxImageDimension1D: u32,
  /// max 2D image dimension
  /// * Limit Type: max
  pub maxImageDimension2D: u32,
  /// max 3D image dimension
  /// * Limit Type: max
  pub maxImageDimension3D: u32,
  /// max cubemap image dimension
  /// * Limit Type: max
  pub maxImageDimensionCube: u32,
  /// max layers for image arrays
  /// * Limit Type: max
  pub maxImageArrayLayers: u32,
  /// max texel buffer size (fstexels)
  /// * Limit Type: max
  pub maxTexelBufferElements: u32,
  /// max uniform buffer range (bytes)
  /// * Limit Type: max
  pub maxUniformBufferRange: u32,
  /// max storage buffer range (bytes)
  /// * Limit Type: max
  pub maxStorageBufferRange: u32,
  /// max size of the push constants pool (bytes)
  /// * Limit Type: max
  pub maxPushConstantsSize: u32,
  /// max number of device memory allocations supported
  /// * Limit Type: max
  pub maxMemoryAllocationCount: u32,
  /// max number of samplers that can be allocated on a device
  /// * Limit Type: max
  pub maxSamplerAllocationCount: u32,
  /// Granularity (in bytes) at which buffers and images can be bound to adjacent memory for simultaneous usage
  /// * Limit Type: min,mul
  pub bufferImageGranularity: VkDeviceSize,
  /// Total address space available for sparse allocations (bytes)
  /// * Limit Type: max
  pub sparseAddressSpaceSize: VkDeviceSize,
  /// max number of descriptors sets that can be bound to a pipeline
  /// * Limit Type: max
  pub maxBoundDescriptorSets: u32,
  /// max number of samplers allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub maxPerStageDescriptorSamplers: u32,
  /// max number of uniform buffers allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub maxPerStageDescriptorUniformBuffers: u32,
  /// max number of storage buffers allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub maxPerStageDescriptorStorageBuffers: u32,
  /// max number of sampled images allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub maxPerStageDescriptorSampledImages: u32,
  /// max number of storage images allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub maxPerStageDescriptorStorageImages: u32,
  /// max number of input attachments allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub maxPerStageDescriptorInputAttachments: u32,
  /// max number of resources allowed by a single stage
  /// * Limit Type: max
  pub maxPerStageResources: u32,
  /// max number of samplers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub maxDescriptorSetSamplers: u32,
  /// max number of uniform buffers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub maxDescriptorSetUniformBuffers: u32,
  /// max number of dynamic uniform buffers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub maxDescriptorSetUniformBuffersDynamic: u32,
  /// max number of storage buffers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub maxDescriptorSetStorageBuffers: u32,
  /// max number of dynamic storage buffers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub maxDescriptorSetStorageBuffersDynamic: u32,
  /// max number of sampled images allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub maxDescriptorSetSampledImages: u32,
  /// max number of storage images allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub maxDescriptorSetStorageImages: u32,
  /// max number of input attachments allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub maxDescriptorSetInputAttachments: u32,
  /// max number of vertex input attribute slots
  /// * Limit Type: max
  pub maxVertexInputAttributes: u32,
  /// max number of vertex input binding slots
  /// * Limit Type: max
  pub maxVertexInputBindings: u32,
  /// max vertex input attribute offset added to vertex buffer offset
  /// * Limit Type: max
  pub maxVertexInputAttributeOffset: u32,
  /// max vertex input binding stride
  /// * Limit Type: max
  pub maxVertexInputBindingStride: u32,
  /// max number of output components written by vertex shader
  /// * Limit Type: max
  pub maxVertexOutputComponents: u32,
  /// max level supported by tessellation primitive generator
  /// * Limit Type: max
  pub maxTessellationGenerationLevel: u32,
  /// max patch size (vertices)
  /// * Limit Type: max
  pub maxTessellationPatchSize: u32,
  /// max number of input components per-vertex in TCS
  /// * Limit Type: max
  pub maxTessellationControlPerVertexInputComponents: u32,
  /// max number of output components per-vertex in TCS
  /// * Limit Type: max
  pub maxTessellationControlPerVertexOutputComponents: u32,
  /// max number of output components per-patch in TCS
  /// * Limit Type: max
  pub maxTessellationControlPerPatchOutputComponents: u32,
  /// max total number of per-vertex and per-patch output components in TCS
  /// * Limit Type: max
  pub maxTessellationControlTotalOutputComponents: u32,
  /// max number of input components per vertex in TES
  /// * Limit Type: max
  pub maxTessellationEvaluationInputComponents: u32,
  /// max number of output components per vertex in TES
  /// * Limit Type: max
  pub maxTessellationEvaluationOutputComponents: u32,
  /// max invocation count supported in geometry shader
  /// * Limit Type: max
  pub maxGeometryShaderInvocations: u32,
  /// max number of input components read in geometry stage
  /// * Limit Type: max
  pub maxGeometryInputComponents: u32,
  /// max number of output components written in geometry stage
  /// * Limit Type: max
  pub maxGeometryOutputComponents: u32,
  /// max number of vertices that can be emitted in geometry stage
  /// * Limit Type: max
  pub maxGeometryOutputVertices: u32,
  /// max total number of components (all vertices) written in geometry stage
  /// * Limit Type: max
  pub maxGeometryTotalOutputComponents: u32,
  /// max number of input components read in fragment stage
  /// * Limit Type: max
  pub maxFragmentInputComponents: u32,
  /// max number of output attachments written in fragment stage
  /// * Limit Type: max
  pub maxFragmentOutputAttachments: u32,
  /// max number of output attachments written when using dual source blending
  /// * Limit Type: max
  pub maxFragmentDualSrcAttachments: u32,
  /// max total number of storage buffers, storage images and output buffers
  /// * Limit Type: max
  pub maxFragmentCombinedOutputResources: u32,
  /// max total storage size of work group local storage (bytes)
  /// * Limit Type: max
  pub maxComputeSharedMemorySize: u32,
  /// max num of compute work groups that may be dispatched by a single command (x,y,z)
  /// * Limit Type: max
  pub maxComputeWorkGroupCount: [u32; 3],
  /// max total compute invocations in a single local work group
  /// * Limit Type: max
  pub maxComputeWorkGroupInvocations: u32,
  /// max local size of a compute work group (x,y,z)
  /// * Limit Type: max
  pub maxComputeWorkGroupSize: [u32; 3],
  /// number bits of subpixel precision in screen x and y
  /// * Limit Type: bits
  pub subPixelPrecisionBits: u32,
  /// number bits of precision for selecting texel weights
  /// * Limit Type: bits
  pub subTexelPrecisionBits: u32,
  /// number bits of precision for selecting mipmap weights
  /// * Limit Type: bits
  pub mipmapPrecisionBits: u32,
  /// max index value for indexed draw calls (for 32-bit indices)
  /// * Limit Type: max
  pub maxDrawIndexedIndexValue: u32,
  /// max draw count for indirect drawing calls
  /// * Limit Type: max
  pub maxDrawIndirectCount: u32,
  /// max absolute sampler LOD bias
  /// * Limit Type: max
  pub maxSamplerLodBias: c_float,
  /// max degree of sampler anisotropy
  /// * Limit Type: max
  pub maxSamplerAnisotropy: c_float,
  /// max number of active viewports
  /// * Limit Type: max
  pub maxViewports: u32,
  /// max viewport dimensions (x,y)
  /// * Limit Type: max
  pub maxViewportDimensions: [u32; 2],
  /// viewport bounds range (min,max)
  /// * Limit Type: range
  pub viewportBoundsRange: [c_float; 2],
  /// number bits of subpixel precision for viewport
  /// * Limit Type: bits
  pub viewportSubPixelBits: u32,
  /// min required alignment of pointers returned by MapMemory (bytes)
  /// * Limit Type: min,pot
  pub minMemoryMapAlignment: c_size_t,
  /// min required alignment for texel buffer offsets (bytes)
  /// * Limit Type: min,pot
  pub minTexelBufferOffsetAlignment: VkDeviceSize,
  /// min required alignment for uniform buffer sizes and offsets (bytes)
  /// * Limit Type: min,pot
  pub minUniformBufferOffsetAlignment: VkDeviceSize,
  /// min required alignment for storage buffer offsets (bytes)
  /// * Limit Type: min,pot
  pub minStorageBufferOffsetAlignment: VkDeviceSize,
  /// min texel offset for OpTextureSampleOffset
  /// * Limit Type: min
  pub minTexelOffset: i32,
  /// max texel offset for OpTextureSampleOffset
  /// * Limit Type: max
  pub maxTexelOffset: u32,
  /// min texel offset for OpTextureGatherOffset
  /// * Limit Type: min
  pub minTexelGatherOffset: i32,
  /// max texel offset for OpTextureGatherOffset
  /// * Limit Type: max
  pub maxTexelGatherOffset: u32,
  /// furthest negative offset for interpolateAtOffset
  /// * Limit Type: min
  pub minInterpolationOffset: c_float,
  /// furthest positive offset for interpolateAtOffset
  /// * Limit Type: max
  pub maxInterpolationOffset: c_float,
  /// number of subpixel bits for interpolateAtOffset
  /// * Limit Type: bits
  pub subPixelInterpolationOffsetBits: u32,
  /// max width for a framebuffer
  /// * Limit Type: max
  pub maxFramebufferWidth: u32,
  /// max height for a framebuffer
  /// * Limit Type: max
  pub maxFramebufferHeight: u32,
  /// max layer count for a layered framebuffer
  /// * Limit Type: max
  pub maxFramebufferLayers: u32,
  /// supported color sample counts for a framebuffer
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebufferColorSampleCounts: VkSampleCountFlags,
  /// supported depth sample counts for a framebuffer
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebufferDepthSampleCounts: VkSampleCountFlags,
  /// supported stencil sample counts for a framebuffer
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebufferStencilSampleCounts: VkSampleCountFlags,
  /// supported sample counts for a subpass which uses no attachments
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
  /// max number of color attachments per subpass
  /// * Limit Type: max
  pub maxColorAttachments: u32,
  /// supported color sample counts for a non-integer sampled image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub sampledImageColorSampleCounts: VkSampleCountFlags,
  /// supported sample counts for an integer image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
  /// supported depth sample counts for a sampled image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub sampledImageDepthSampleCounts: VkSampleCountFlags,
  /// supported stencil sample counts for a sampled image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub sampledImageStencilSampleCounts: VkSampleCountFlags,
  /// supported sample counts for a storage image
  /// * Optional: true
  /// * Limit Type: bitmask
  pub storageImageSampleCounts: VkSampleCountFlags,
  /// max number of sample mask words
  /// * Limit Type: max
  pub maxSampleMaskWords: u32,
  /// timestamps on graphics and compute queues
  /// * Limit Type: bitmask
  pub timestampComputeAndGraphics: VkBool32,
  /// number of nanoseconds it takes for timestamp query value to increment by 1
  /// * Limit Type: min,mul
  pub timestampPeriod: c_float,
  /// max number of clip distances
  /// * Limit Type: max
  pub maxClipDistances: u32,
  /// max number of cull distances
  /// * Limit Type: max
  pub maxCullDistances: u32,
  /// max combined number of user clipping
  /// * Limit Type: max
  pub maxCombinedClipAndCullDistances: u32,
  /// distinct queue priorities available
  /// * Limit Type: max
  pub discreteQueuePriorities: u32,
  /// range (min,max) of supported point sizes
  /// * Limit Type: range
  pub pointSizeRange: [c_float; 2],
  /// range (min,max) of supported line widths
  /// * Limit Type: range
  pub lineWidthRange: [c_float; 2],
  /// granularity of supported point sizes
  /// * Limit Type: min,mul
  pub pointSizeGranularity: c_float,
  /// granularity of supported line widths
  /// * Limit Type: min,mul
  pub lineWidthGranularity: c_float,
  /// line rasterization follows preferred rules
  /// * Limit Type: bitmask
  pub strictLines: VkBool32,
  /// supports standard sample locations for all supported sample counts
  /// * Limit Type: bitmask
  pub standardSampleLocations: VkBool32,
  /// optimal offset of buffer copies
  /// * Limit Type: min,pot
  pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
  /// optimal pitch of buffer copies
  /// * Limit Type: min,pot
  pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
  /// minimum size and alignment for non-coherent host-mapped device memory access
  /// * Limit Type: min,pot
  pub nonCoherentAtomSize: VkDeviceSize,
}
impl Default for VkPhysicalDeviceLimits {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      maxImageDimension1D: Default::default(),
      maxImageDimension2D: Default::default(),
      maxImageDimension3D: Default::default(),
      maxImageDimensionCube: Default::default(),
      maxImageArrayLayers: Default::default(),
      maxTexelBufferElements: Default::default(),
      maxUniformBufferRange: Default::default(),
      maxStorageBufferRange: Default::default(),
      maxPushConstantsSize: Default::default(),
      maxMemoryAllocationCount: Default::default(),
      maxSamplerAllocationCount: Default::default(),
      bufferImageGranularity: Default::default(),
      sparseAddressSpaceSize: Default::default(),
      maxBoundDescriptorSets: Default::default(),
      maxPerStageDescriptorSamplers: Default::default(),
      maxPerStageDescriptorUniformBuffers: Default::default(),
      maxPerStageDescriptorStorageBuffers: Default::default(),
      maxPerStageDescriptorSampledImages: Default::default(),
      maxPerStageDescriptorStorageImages: Default::default(),
      maxPerStageDescriptorInputAttachments: Default::default(),
      maxPerStageResources: Default::default(),
      maxDescriptorSetSamplers: Default::default(),
      maxDescriptorSetUniformBuffers: Default::default(),
      maxDescriptorSetUniformBuffersDynamic: Default::default(),
      maxDescriptorSetStorageBuffers: Default::default(),
      maxDescriptorSetStorageBuffersDynamic: Default::default(),
      maxDescriptorSetSampledImages: Default::default(),
      maxDescriptorSetStorageImages: Default::default(),
      maxDescriptorSetInputAttachments: Default::default(),
      maxVertexInputAttributes: Default::default(),
      maxVertexInputBindings: Default::default(),
      maxVertexInputAttributeOffset: Default::default(),
      maxVertexInputBindingStride: Default::default(),
      maxVertexOutputComponents: Default::default(),
      maxTessellationGenerationLevel: Default::default(),
      maxTessellationPatchSize: Default::default(),
      maxTessellationControlPerVertexInputComponents: Default::default(),
      maxTessellationControlPerVertexOutputComponents: Default::default(),
      maxTessellationControlPerPatchOutputComponents: Default::default(),
      maxTessellationControlTotalOutputComponents: Default::default(),
      maxTessellationEvaluationInputComponents: Default::default(),
      maxTessellationEvaluationOutputComponents: Default::default(),
      maxGeometryShaderInvocations: Default::default(),
      maxGeometryInputComponents: Default::default(),
      maxGeometryOutputComponents: Default::default(),
      maxGeometryOutputVertices: Default::default(),
      maxGeometryTotalOutputComponents: Default::default(),
      maxFragmentInputComponents: Default::default(),
      maxFragmentOutputAttachments: Default::default(),
      maxFragmentDualSrcAttachments: Default::default(),
      maxFragmentCombinedOutputResources: Default::default(),
      maxComputeSharedMemorySize: Default::default(),
      maxComputeWorkGroupCount: [Default::default(); 3],
      maxComputeWorkGroupInvocations: Default::default(),
      maxComputeWorkGroupSize: [Default::default(); 3],
      subPixelPrecisionBits: Default::default(),
      subTexelPrecisionBits: Default::default(),
      mipmapPrecisionBits: Default::default(),
      maxDrawIndexedIndexValue: Default::default(),
      maxDrawIndirectCount: Default::default(),
      maxSamplerLodBias: Default::default(),
      maxSamplerAnisotropy: Default::default(),
      maxViewports: Default::default(),
      maxViewportDimensions: [Default::default(); 2],
      viewportBoundsRange: [Default::default(); 2],
      viewportSubPixelBits: Default::default(),
      minMemoryMapAlignment: Default::default(),
      minTexelBufferOffsetAlignment: Default::default(),
      minUniformBufferOffsetAlignment: Default::default(),
      minStorageBufferOffsetAlignment: Default::default(),
      minTexelOffset: Default::default(),
      maxTexelOffset: Default::default(),
      minTexelGatherOffset: Default::default(),
      maxTexelGatherOffset: Default::default(),
      minInterpolationOffset: Default::default(),
      maxInterpolationOffset: Default::default(),
      subPixelInterpolationOffsetBits: Default::default(),
      maxFramebufferWidth: Default::default(),
      maxFramebufferHeight: Default::default(),
      maxFramebufferLayers: Default::default(),
      framebufferColorSampleCounts: Default::default(),
      framebufferDepthSampleCounts: Default::default(),
      framebufferStencilSampleCounts: Default::default(),
      framebufferNoAttachmentsSampleCounts: Default::default(),
      maxColorAttachments: Default::default(),
      sampledImageColorSampleCounts: Default::default(),
      sampledImageIntegerSampleCounts: Default::default(),
      sampledImageDepthSampleCounts: Default::default(),
      sampledImageStencilSampleCounts: Default::default(),
      storageImageSampleCounts: Default::default(),
      maxSampleMaskWords: Default::default(),
      timestampComputeAndGraphics: Default::default(),
      timestampPeriod: Default::default(),
      maxClipDistances: Default::default(),
      maxCullDistances: Default::default(),
      maxCombinedClipAndCullDistances: Default::default(),
      discreteQueuePriorities: Default::default(),
      pointSizeRange: [Default::default(); 2],
      lineWidthRange: [Default::default(); 2],
      pointSizeGranularity: Default::default(),
      lineWidthGranularity: Default::default(),
      strictLines: Default::default(),
      standardSampleLocations: Default::default(),
      optimalBufferCopyOffsetAlignment: Default::default(),
      optimalBufferCopyRowPitchAlignment: Default::default(),
      nonCoherentAtomSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLineRasterizationFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub rectangularLines: VkBool32,
  pub bresenhamLines: VkBool32,
  pub smoothLines: VkBool32,
  pub stippledRectangularLines: VkBool32,
  pub stippledBresenhamLines: VkBool32,
  pub stippledSmoothLines: VkBool32,
}
impl Default for VkPhysicalDeviceLineRasterizationFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      rectangularLines: Default::default(),
      bresenhamLines: Default::default(),
      smoothLines: Default::default(),
      stippledRectangularLines: Default::default(),
      stippledBresenhamLines: Default::default(),
      stippledSmoothLines: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLineRasterizationPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bits
  pub lineSubPixelPrecisionBits: u32,
}
impl Default for VkPhysicalDeviceLineRasterizationPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      lineSubPixelPrecisionBits: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceLinearColorAttachmentFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub linearColorAttachment: VkBool32,
}
impl Default for VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      linearColorAttachment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMaintenance3Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance3Properties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance3Properties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxPerSetDescriptors: u32,
  /// * Limit Type: max
  pub maxMemoryAllocationSize: VkDeviceSize,
}
impl Default for VkPhysicalDeviceMaintenance3Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES,
      pNext: core::ptr::null_mut(),
      maxPerSetDescriptors: Default::default(),
      maxMemoryAllocationSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMaintenance4Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Features.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub maintenance4: VkBool32,
}
impl Default for VkPhysicalDeviceMaintenance4Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES,
      pNext: core::ptr::null_mut(),
      maintenance4: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMaintenance4Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Properties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Properties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxBufferSize: VkDeviceSize,
}
impl Default for VkPhysicalDeviceMaintenance4Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES,
      pNext: core::ptr::null_mut(),
      maxBufferSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryBudgetPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceMemoryProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub heapBudget: [VkDeviceSize; VK_MAX_MEMORY_HEAPS],
  pub heapUsage: [VkDeviceSize; VK_MAX_MEMORY_HEAPS],
}
impl Default for VkPhysicalDeviceMemoryBudgetPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      heapBudget: [Default::default(); VK_MAX_MEMORY_HEAPS],
      heapUsage: [Default::default(); VK_MAX_MEMORY_HEAPS],
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryDecompressionFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryDecompressionFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryDecompression: VkBool32,
}
impl Default for VkPhysicalDeviceMemoryDecompressionFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      memoryDecompression: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryDecompressionPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryDecompressionPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub decompressionMethods: VkMemoryDecompressionMethodFlagsNV,
  /// * Limit Type: max
  pub maxDecompressionIndirectCount: u64,
}
impl Default for VkPhysicalDeviceMemoryDecompressionPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      decompressionMethods: Default::default(),
      maxDecompressionIndirectCount: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryPriorityFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryPriority: VkBool32,
}
impl Default for VkPhysicalDeviceMemoryPriorityFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      memoryPriority: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
  pub memoryTypeCount: u32,
  pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
  pub memoryHeapCount: u32,
  pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}
impl Default for VkPhysicalDeviceMemoryProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      memoryTypeCount: Default::default(),
      memoryTypes: [Default::default(); VK_MAX_MEMORY_TYPES],
      memoryHeapCount: Default::default(),
      memoryHeaps: [Default::default(); VK_MAX_MEMORY_HEAPS],
    }
  }
}

/// Khronos: [VkPhysicalDeviceMemoryProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}
impl Default for VkPhysicalDeviceMemoryProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2,
      pNext: core::ptr::null_mut(),
      memoryProperties: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMeshShaderFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub taskShader: VkBool32,
  pub meshShader: VkBool32,
  pub multiviewMeshShader: VkBool32,
  pub primitiveFragmentShadingRateMeshShader: VkBool32,
  pub meshShaderQueries: VkBool32,
}
impl Default for VkPhysicalDeviceMeshShaderFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      taskShader: Default::default(),
      meshShader: Default::default(),
      multiviewMeshShader: Default::default(),
      primitiveFragmentShadingRateMeshShader: Default::default(),
      meshShaderQueries: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMeshShaderFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub taskShader: VkBool32,
  pub meshShader: VkBool32,
}
impl Default for VkPhysicalDeviceMeshShaderFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      taskShader: Default::default(),
      meshShader: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMeshShaderPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxTaskWorkGroupTotalCount: u32,
  /// * Limit Type: max
  pub maxTaskWorkGroupCount: [u32; 3],
  /// * Limit Type: max
  pub maxTaskWorkGroupInvocations: u32,
  /// * Limit Type: max
  pub maxTaskWorkGroupSize: [u32; 3],
  /// * Limit Type: max
  pub maxTaskPayloadSize: u32,
  /// * Limit Type: max
  pub maxTaskSharedMemorySize: u32,
  /// * Limit Type: max
  pub maxTaskPayloadAndSharedMemorySize: u32,
  /// * Limit Type: max
  pub maxMeshWorkGroupTotalCount: u32,
  /// * Limit Type: max
  pub maxMeshWorkGroupCount: [u32; 3],
  /// * Limit Type: max
  pub maxMeshWorkGroupInvocations: u32,
  /// * Limit Type: max
  pub maxMeshWorkGroupSize: [u32; 3],
  /// * Limit Type: max
  pub maxMeshSharedMemorySize: u32,
  /// * Limit Type: max
  pub maxMeshPayloadAndSharedMemorySize: u32,
  /// * Limit Type: max
  pub maxMeshOutputMemorySize: u32,
  /// * Limit Type: max
  pub maxMeshPayloadAndOutputMemorySize: u32,
  /// * Limit Type: max
  pub maxMeshOutputComponents: u32,
  /// * Limit Type: max
  pub maxMeshOutputVertices: u32,
  /// * Limit Type: max
  pub maxMeshOutputPrimitives: u32,
  /// * Limit Type: max
  pub maxMeshOutputLayers: u32,
  /// * Limit Type: max
  pub maxMeshMultiviewViewCount: u32,
  /// * Limit Type: noauto
  pub meshOutputPerVertexGranularity: u32,
  /// * Limit Type: noauto
  pub meshOutputPerPrimitiveGranularity: u32,
  /// * Limit Type: max
  pub maxPreferredTaskWorkGroupInvocations: u32,
  /// * Limit Type: max
  pub maxPreferredMeshWorkGroupInvocations: u32,
  /// * Limit Type: noauto
  pub prefersLocalInvocationVertexOutput: VkBool32,
  /// * Limit Type: noauto
  pub prefersLocalInvocationPrimitiveOutput: VkBool32,
  /// * Limit Type: noauto
  pub prefersCompactVertexOutput: VkBool32,
  /// * Limit Type: noauto
  pub prefersCompactPrimitiveOutput: VkBool32,
}
impl Default for VkPhysicalDeviceMeshShaderPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      maxTaskWorkGroupTotalCount: Default::default(),
      maxTaskWorkGroupCount: [Default::default(); 3],
      maxTaskWorkGroupInvocations: Default::default(),
      maxTaskWorkGroupSize: [Default::default(); 3],
      maxTaskPayloadSize: Default::default(),
      maxTaskSharedMemorySize: Default::default(),
      maxTaskPayloadAndSharedMemorySize: Default::default(),
      maxMeshWorkGroupTotalCount: Default::default(),
      maxMeshWorkGroupCount: [Default::default(); 3],
      maxMeshWorkGroupInvocations: Default::default(),
      maxMeshWorkGroupSize: [Default::default(); 3],
      maxMeshSharedMemorySize: Default::default(),
      maxMeshPayloadAndSharedMemorySize: Default::default(),
      maxMeshOutputMemorySize: Default::default(),
      maxMeshPayloadAndOutputMemorySize: Default::default(),
      maxMeshOutputComponents: Default::default(),
      maxMeshOutputVertices: Default::default(),
      maxMeshOutputPrimitives: Default::default(),
      maxMeshOutputLayers: Default::default(),
      maxMeshMultiviewViewCount: Default::default(),
      meshOutputPerVertexGranularity: Default::default(),
      meshOutputPerPrimitiveGranularity: Default::default(),
      maxPreferredTaskWorkGroupInvocations: Default::default(),
      maxPreferredMeshWorkGroupInvocations: Default::default(),
      prefersLocalInvocationVertexOutput: Default::default(),
      prefersLocalInvocationPrimitiveOutput: Default::default(),
      prefersCompactVertexOutput: Default::default(),
      prefersCompactPrimitiveOutput: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMeshShaderPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxDrawMeshTasksCount: u32,
  /// * Limit Type: max
  pub maxTaskWorkGroupInvocations: u32,
  /// * Limit Type: max
  pub maxTaskWorkGroupSize: [u32; 3],
  /// * Limit Type: max
  pub maxTaskTotalMemorySize: u32,
  /// * Limit Type: max
  pub maxTaskOutputCount: u32,
  /// * Limit Type: max
  pub maxMeshWorkGroupInvocations: u32,
  /// * Limit Type: max
  pub maxMeshWorkGroupSize: [u32; 3],
  /// * Limit Type: max
  pub maxMeshTotalMemorySize: u32,
  /// * Limit Type: max
  pub maxMeshOutputVertices: u32,
  /// * Limit Type: max
  pub maxMeshOutputPrimitives: u32,
  /// * Limit Type: max
  pub maxMeshMultiviewViewCount: u32,
  /// * Limit Type: min,mul
  pub meshOutputPerVertexGranularity: u32,
  /// * Limit Type: min,mul
  pub meshOutputPerPrimitiveGranularity: u32,
}
impl Default for VkPhysicalDeviceMeshShaderPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      maxDrawMeshTasksCount: Default::default(),
      maxTaskWorkGroupInvocations: Default::default(),
      maxTaskWorkGroupSize: [Default::default(); 3],
      maxTaskTotalMemorySize: Default::default(),
      maxTaskOutputCount: Default::default(),
      maxMeshWorkGroupInvocations: Default::default(),
      maxMeshWorkGroupSize: [Default::default(); 3],
      maxMeshTotalMemorySize: Default::default(),
      maxMeshOutputVertices: Default::default(),
      maxMeshOutputPrimitives: Default::default(),
      maxMeshMultiviewViewCount: Default::default(),
      meshOutputPerVertexGranularity: Default::default(),
      meshOutputPerPrimitiveGranularity: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiDrawFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub multiDraw: VkBool32,
}
impl Default for VkPhysicalDeviceMultiDrawFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      multiDraw: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiDrawPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxMultiDrawCount: u32,
}
impl Default for VkPhysicalDeviceMultiDrawPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      maxMultiDrawCount: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub multisampledRenderToSingleSampled: VkBool32,
}
impl Default for VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      multisampledRenderToSingleSampled: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Multiple views in a renderpass
  pub multiview: VkBool32,
  /// Multiple views in a renderpass w/ geometry shader
  pub multiviewGeometryShader: VkBool32,
  /// Multiple views in a renderpass w/ tessellation shader
  pub multiviewTessellationShader: VkBool32,
}
impl Default for VkPhysicalDeviceMultiviewFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES,
      pNext: core::ptr::null_mut(),
      multiview: Default::default(),
      multiviewGeometryShader: Default::default(),
      multiviewTessellationShader: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub perViewPositionAllComponents: VkBool32,
}
impl Default for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
      pNext: core::ptr::null_mut(),
      perViewPositionAllComponents: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub multiviewPerViewRenderAreas: VkBool32,
}
impl Default for VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM,
      pNext: core::ptr::null_mut(),
      multiviewPerViewRenderAreas: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub multiviewPerViewViewports: VkBool32,
}
impl Default for VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM,
      pNext: core::ptr::null_mut(),
      multiviewPerViewViewports: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMultiviewProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMultiviewProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// max number of views in a subpass
  /// * Limit Type: max
  pub maxMultiviewViewCount: u32,
  /// max instance index for a draw in a multiview subpass
  /// * Limit Type: max
  pub maxMultiviewInstanceIndex: u32,
}
impl Default for VkPhysicalDeviceMultiviewProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES,
      pNext: core::ptr::null_mut(),
      maxMultiviewViewCount: Default::default(),
      maxMultiviewInstanceIndex: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub mutableDescriptorType: VkBool32,
}
impl Default for VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      mutableDescriptorType: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub nonSeamlessCubeMap: VkBool32,
}
impl Default for VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      nonSeamlessCubeMap: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceOpacityMicromapFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpacityMicromapFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub micromap: VkBool32,
  pub micromapCaptureReplay: VkBool32,
  pub micromapHostCommands: VkBool32,
}
impl Default for VkPhysicalDeviceOpacityMicromapFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      micromap: Default::default(),
      micromapCaptureReplay: Default::default(),
      micromapHostCommands: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceOpacityMicromapPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpacityMicromapPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxOpacity2StateSubdivisionLevel: u32,
  /// * Limit Type: max
  pub maxOpacity4StateSubdivisionLevel: u32,
}
impl Default for VkPhysicalDeviceOpacityMicromapPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      maxOpacity2StateSubdivisionLevel: Default::default(),
      maxOpacity4StateSubdivisionLevel: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceOpticalFlowFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpticalFlowFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub opticalFlow: VkBool32,
}
impl Default for VkPhysicalDeviceOpticalFlowFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      opticalFlow: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceOpticalFlowPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceOpticalFlowPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub supportedOutputGridSizes: VkOpticalFlowGridSizeFlagsNV,
  /// * Limit Type: bitmask
  pub supportedHintGridSizes: VkOpticalFlowGridSizeFlagsNV,
  /// * Limit Type: noauto
  pub hintSupported: VkBool32,
  /// * Limit Type: noauto
  pub costSupported: VkBool32,
  /// * Limit Type: noauto
  pub bidirectionalFlowSupported: VkBool32,
  /// * Limit Type: noauto
  pub globalFlowSupported: VkBool32,
  /// * Limit Type: noauto
  pub minWidth: u32,
  /// * Limit Type: noauto
  pub minHeight: u32,
  /// * Limit Type: noauto
  pub maxWidth: u32,
  /// * Limit Type: noauto
  pub maxHeight: u32,
  /// * Limit Type: noauto
  pub maxNumRegionsOfInterest: u32,
}
impl Default for VkPhysicalDeviceOpticalFlowPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      supportedOutputGridSizes: Default::default(),
      supportedHintGridSizes: Default::default(),
      hintSupported: Default::default(),
      costSupported: Default::default(),
      bidirectionalFlowSupported: Default::default(),
      globalFlowSupported: Default::default(),
      minWidth: Default::default(),
      minHeight: Default::default(),
      maxWidth: Default::default(),
      maxHeight: Default::default(),
      maxNumRegionsOfInterest: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePCIBusInfoPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: noauto
  pub pciDomain: u32,
  /// * Limit Type: noauto
  pub pciBus: u32,
  /// * Limit Type: noauto
  pub pciDevice: u32,
  /// * Limit Type: noauto
  pub pciFunction: u32,
}
impl Default for VkPhysicalDevicePCIBusInfoPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      pciDomain: Default::default(),
      pciBus: Default::default(),
      pciDevice: Default::default(),
      pciFunction: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub pageableDeviceLocalMemory: VkBool32,
}
impl Default for VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      pageableDeviceLocalMemory: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePerformanceQueryFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// performance counters supported in query pools
  pub performanceCounterQueryPools: VkBool32,
  /// performance counters from multiple query pools can be accessed in the same primary command buffer
  pub performanceCounterMultipleQueryPools: VkBool32,
}
impl Default for VkPhysicalDevicePerformanceQueryFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      performanceCounterQueryPools: Default::default(),
      performanceCounterMultipleQueryPools: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePerformanceQueryPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Flag to specify whether performance queries are allowed to be used in vkCmdCopyQueryPoolResults
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub allowCommandBufferQueryCopies: VkBool32,
}
impl Default for VkPhysicalDevicePerformanceQueryPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      allowCommandBufferQueryCopies: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineCreationCacheControlFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineCreationCacheControlFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub pipelineCreationCacheControl: VkBool32,
}
impl Default for VkPhysicalDevicePipelineCreationCacheControlFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES,
      pNext: core::ptr::null_mut(),
      pipelineCreationCacheControl: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub pipelineExecutableInfo: VkBool32,
}
impl Default for VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      pipelineExecutableInfo: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub pipelineLibraryGroupHandles: VkBool32,
}
impl Default for VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      pipelineLibraryGroupHandles: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelinePropertiesFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelinePropertiesFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelinePropertiesFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub pipelinePropertiesIdentifier: VkBool32,
}
impl Default for VkPhysicalDevicePipelinePropertiesFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      pipelinePropertiesIdentifier: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineProtectedAccessFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineProtectedAccessFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineProtectedAccessFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub pipelineProtectedAccess: VkBool32,
}
impl Default for VkPhysicalDevicePipelineProtectedAccessFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      pipelineProtectedAccess: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineRobustnessFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineRobustnessFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub pipelineRobustness: VkBool32,
}
impl Default for VkPhysicalDevicePipelineRobustnessFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      pipelineRobustness: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePipelineRobustnessPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineRobustnessPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub defaultRobustnessStorageBuffers: VkPipelineRobustnessBufferBehaviorEXT,
  /// * Limit Type: exact
  pub defaultRobustnessUniformBuffers: VkPipelineRobustnessBufferBehaviorEXT,
  /// * Limit Type: exact
  pub defaultRobustnessVertexInputs: VkPipelineRobustnessBufferBehaviorEXT,
  /// * Limit Type: exact
  pub defaultRobustnessImages: VkPipelineRobustnessImageBehaviorEXT,
}
impl Default for VkPhysicalDevicePipelineRobustnessPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      defaultRobustnessStorageBuffers: Default::default(),
      defaultRobustnessUniformBuffers: Default::default(),
      defaultRobustnessVertexInputs: Default::default(),
      defaultRobustnessImages: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePointClippingProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePointClippingProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePointClippingProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub pointClippingBehavior: VkPointClippingBehavior,
}
impl Default for VkPhysicalDevicePointClippingProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES,
      pNext: core::ptr::null_mut(),
      pointClippingBehavior: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePresentBarrierFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentBarrierFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentBarrierFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub presentBarrier: VkBool32,
}
impl Default for VkPhysicalDevicePresentBarrierFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      presentBarrier: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePresentIdFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentIdFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentIdFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Present ID in VkPresentInfoKHR
  pub presentId: VkBool32,
}
impl Default for VkPhysicalDevicePresentIdFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      presentId: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePresentWaitFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentWaitFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentWaitFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// vkWaitForPresentKHR is supported
  pub presentWait: VkBool32,
}
impl Default for VkPhysicalDevicePresentWaitFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      presentWait: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePresentationPropertiesANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentationPropertiesANDROID.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePresentationPropertiesANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub sharedImage: VkBool32,
}
impl Default for VkPhysicalDevicePresentationPropertiesANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID,
      pNext: core::ptr::null(),
      sharedImage: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub primitiveTopologyListRestart: VkBool32,
  pub primitiveTopologyPatchListRestart: VkBool32,
}
impl Default for VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      primitiveTopologyListRestart: Default::default(),
      primitiveTopologyPatchListRestart: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub primitivesGeneratedQuery: VkBool32,
  pub primitivesGeneratedQueryWithRasterizerDiscard: VkBool32,
  pub primitivesGeneratedQueryWithNonZeroStreams: VkBool32,
}
impl Default for VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      primitivesGeneratedQuery: Default::default(),
      primitivesGeneratedQueryWithRasterizerDiscard: Default::default(),
      primitivesGeneratedQueryWithNonZeroStreams: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePrivateDataFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrivateDataFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePrivateDataFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub privateData: VkBool32,
}
impl Default for VkPhysicalDevicePrivateDataFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES,
      pNext: core::ptr::null_mut(),
      privateData: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
  /// * Limit Type: noauto
  pub apiVersion: u32,
  /// * Limit Type: noauto
  pub driverVersion: u32,
  /// * Limit Type: noauto
  pub vendorID: u32,
  /// * Limit Type: noauto
  pub deviceID: u32,
  /// * Limit Type: noauto
  pub deviceType: VkPhysicalDeviceType,
  /// * Limit Type: noauto
  pub deviceName: [u8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
  /// * Limit Type: noauto
  pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
  /// * Limit Type: struct
  pub limits: VkPhysicalDeviceLimits,
  /// * Limit Type: struct
  pub sparseProperties: VkPhysicalDeviceSparseProperties,
}
impl Default for VkPhysicalDeviceProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      apiVersion: Default::default(),
      driverVersion: Default::default(),
      vendorID: Default::default(),
      deviceID: Default::default(),
      deviceType: Default::default(),
      deviceName: [Default::default(); VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
      pipelineCacheUUID: [Default::default(); VK_UUID_SIZE],
      limits: Default::default(),
      sparseProperties: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties2.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: struct
  pub properties: VkPhysicalDeviceProperties,
}
impl Default for VkPhysicalDeviceProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2,
      pNext: core::ptr::null_mut(),
      properties: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProtectedMemoryFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub protectedMemory: VkBool32,
}
impl Default for VkPhysicalDeviceProtectedMemoryFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
      pNext: core::ptr::null_mut(),
      protectedMemory: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProtectedMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub protectedNoFault: VkBool32,
}
impl Default for VkPhysicalDeviceProtectedMemoryProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES,
      pNext: core::ptr::null_mut(),
      protectedNoFault: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProvokingVertexFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub provokingVertexLast: VkBool32,
  pub transformFeedbackPreservesProvokingVertex: VkBool32,
}
impl Default for VkPhysicalDeviceProvokingVertexFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      provokingVertexLast: Default::default(),
      transformFeedbackPreservesProvokingVertex: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceProvokingVertexPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub provokingVertexModePerPipeline: VkBool32,
  /// * Limit Type: bitmask
  pub transformFeedbackPreservesTriangleFanProvokingVertex: VkBool32,
}
impl Default for VkPhysicalDeviceProvokingVertexPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      provokingVertexModePerPipeline: Default::default(),
      transformFeedbackPreservesTriangleFanProvokingVertex: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDevicePushDescriptorPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxPushDescriptors: u32,
}
impl Default for VkPhysicalDevicePushDescriptorPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      maxPushDescriptors: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub formatRgba10x6WithoutYCbCrSampler: VkBool32,
}
impl Default for VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      formatRgba10x6WithoutYCbCrSampler: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub rasterizationOrderColorAttachmentAccess: VkBool32,
  pub rasterizationOrderDepthAttachmentAccess: VkBool32,
  pub rasterizationOrderStencilAttachmentAccess: VkBool32,
}
impl Default for VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      rasterizationOrderColorAttachmentAccess: Default::default(),
      rasterizationOrderDepthAttachmentAccess: Default::default(),
      rasterizationOrderStencilAttachmentAccess: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayQueryFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayQueryFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub rayQuery: VkBool32,
}
impl Default for VkPhysicalDeviceRayQueryFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      rayQuery: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub rayTracingInvocationReorder: VkBool32,
}
impl Default for VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      rayTracingInvocationReorder: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  /// * Limit Type: noauto
  pub rayTracingInvocationReorderReorderingHint: VkRayTracingInvocationReorderModeNV,
}
impl Default for VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      rayTracingInvocationReorderReorderingHint: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub rayTracingMaintenance1: VkBool32,
  pub rayTracingPipelineTraceRaysIndirect2: VkBool32,
}
impl Default for VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      rayTracingMaintenance1: Default::default(),
      rayTracingPipelineTraceRaysIndirect2: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingMotionBlurFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingMotionBlurFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub rayTracingMotionBlur: VkBool32,
  pub rayTracingMotionBlurPipelineTraceRaysIndirect: VkBool32,
}
impl Default for VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      rayTracingMotionBlur: Default::default(),
      rayTracingMotionBlurPipelineTraceRaysIndirect: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingPipelineFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub rayTracingPipeline: VkBool32,
  pub rayTracingPipelineShaderGroupHandleCaptureReplay: VkBool32,
  pub rayTracingPipelineShaderGroupHandleCaptureReplayMixed: VkBool32,
  pub rayTracingPipelineTraceRaysIndirect: VkBool32,
  pub rayTraversalPrimitiveCulling: VkBool32,
}
impl Default for VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      rayTracingPipeline: Default::default(),
      rayTracingPipelineShaderGroupHandleCaptureReplay: Default::default(),
      rayTracingPipelineShaderGroupHandleCaptureReplayMixed: Default::default(),
      rayTracingPipelineTraceRaysIndirect: Default::default(),
      rayTraversalPrimitiveCulling: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingPipelinePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub shaderGroupHandleSize: u32,
  /// * Limit Type: max
  pub maxRayRecursionDepth: u32,
  /// * Limit Type: max
  pub maxShaderGroupStride: u32,
  /// * Limit Type: exact
  pub shaderGroupBaseAlignment: u32,
  /// * Limit Type: exact
  pub shaderGroupHandleCaptureReplaySize: u32,
  /// * Limit Type: max
  pub maxRayDispatchInvocationCount: u32,
  /// * Limit Type: min,pot
  pub shaderGroupHandleAlignment: u32,
  /// * Limit Type: max
  pub maxRayHitAttributeSize: u32,
}
impl Default for VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      shaderGroupHandleSize: Default::default(),
      maxRayRecursionDepth: Default::default(),
      maxShaderGroupStride: Default::default(),
      shaderGroupBaseAlignment: Default::default(),
      shaderGroupHandleCaptureReplaySize: Default::default(),
      maxRayDispatchInvocationCount: Default::default(),
      shaderGroupHandleAlignment: Default::default(),
      maxRayHitAttributeSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRayTracingPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub shaderGroupHandleSize: u32,
  /// * Limit Type: max
  pub maxRecursionDepth: u32,
  /// * Limit Type: max
  pub maxShaderGroupStride: u32,
  /// * Limit Type: exact
  pub shaderGroupBaseAlignment: u32,
  /// * Limit Type: max
  pub maxGeometryCount: u64,
  /// * Limit Type: max
  pub maxInstanceCount: u64,
  /// * Limit Type: max
  pub maxTriangleCount: u64,
  /// * Limit Type: max
  pub maxDescriptorSetAccelerationStructures: u32,
}
impl Default for VkPhysicalDeviceRayTracingPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      shaderGroupHandleSize: Default::default(),
      maxRecursionDepth: Default::default(),
      maxShaderGroupStride: Default::default(),
      shaderGroupBaseAlignment: Default::default(),
      maxGeometryCount: Default::default(),
      maxInstanceCount: Default::default(),
      maxTriangleCount: Default::default(),
      maxDescriptorSetAccelerationStructures: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub representativeFragmentTest: VkBool32,
}
impl Default for VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      representativeFragmentTest: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRobustness2FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRobustness2FeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub robustBufferAccess2: VkBool32,
  pub robustImageAccess2: VkBool32,
  pub nullDescriptor: VkBool32,
}
impl Default for VkPhysicalDeviceRobustness2FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      robustBufferAccess2: Default::default(),
      robustImageAccess2: Default::default(),
      nullDescriptor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceRobustness2PropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceRobustness2PropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: min,pot
  pub robustStorageBufferAccessSizeAlignment: VkDeviceSize,
  /// * Limit Type: min,pot
  pub robustUniformBufferAccessSizeAlignment: VkDeviceSize,
}
impl Default for VkPhysicalDeviceRobustness2PropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      robustStorageBufferAccessSizeAlignment: Default::default(),
      robustUniformBufferAccessSizeAlignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSampleLocationsPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub sampleLocationSampleCounts: VkSampleCountFlags,
  /// * Limit Type: max
  pub maxSampleLocationGridSize: VkExtent2D,
  /// * Limit Type: range
  pub sampleLocationCoordinateRange: [c_float; 2],
  /// * Limit Type: bits
  pub sampleLocationSubPixelBits: u32,
  /// * Limit Type: bitmask
  pub variableSampleLocations: VkBool32,
}
impl Default for VkPhysicalDeviceSampleLocationsPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      sampleLocationSampleCounts: Default::default(),
      maxSampleLocationGridSize: Default::default(),
      sampleLocationCoordinateRange: [Default::default(); 2],
      sampleLocationSubPixelBits: Default::default(),
      variableSampleLocations: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSamplerFilterMinmaxProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub filterMinmaxSingleComponentFormats: VkBool32,
  /// * Limit Type: bitmask
  pub filterMinmaxImageComponentMapping: VkBool32,
}
impl Default for VkPhysicalDeviceSamplerFilterMinmaxProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES,
      pNext: core::ptr::null_mut(),
      filterMinmaxSingleComponentFormats: Default::default(),
      filterMinmaxImageComponentMapping: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSamplerYcbcrConversionFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Sampler color conversion supported
  pub samplerYcbcrConversion: VkBool32,
}
impl Default for VkPhysicalDeviceSamplerYcbcrConversionFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
      pNext: core::ptr::null_mut(),
      samplerYcbcrConversion: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceScalarBlockLayoutFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub scalarBlockLayout: VkBool32,
}
impl Default for VkPhysicalDeviceScalarBlockLayoutFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES,
      pNext: core::ptr::null_mut(),
      scalarBlockLayout: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub separateDepthStencilLayouts: VkBool32,
}
impl Default for VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES,
      pNext: core::ptr::null_mut(),
      separateDepthStencilLayouts: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderBufferFloat16Atomics: VkBool32,
  pub shaderBufferFloat16AtomicAdd: VkBool32,
  pub shaderBufferFloat16AtomicMinMax: VkBool32,
  pub shaderBufferFloat32AtomicMinMax: VkBool32,
  pub shaderBufferFloat64AtomicMinMax: VkBool32,
  pub shaderSharedFloat16Atomics: VkBool32,
  pub shaderSharedFloat16AtomicAdd: VkBool32,
  pub shaderSharedFloat16AtomicMinMax: VkBool32,
  pub shaderSharedFloat32AtomicMinMax: VkBool32,
  pub shaderSharedFloat64AtomicMinMax: VkBool32,
  pub shaderImageFloat32AtomicMinMax: VkBool32,
  pub sparseImageFloat32AtomicMinMax: VkBool32,
}
impl Default for VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      shaderBufferFloat16Atomics: Default::default(),
      shaderBufferFloat16AtomicAdd: Default::default(),
      shaderBufferFloat16AtomicMinMax: Default::default(),
      shaderBufferFloat32AtomicMinMax: Default::default(),
      shaderBufferFloat64AtomicMinMax: Default::default(),
      shaderSharedFloat16Atomics: Default::default(),
      shaderSharedFloat16AtomicAdd: Default::default(),
      shaderSharedFloat16AtomicMinMax: Default::default(),
      shaderSharedFloat32AtomicMinMax: Default::default(),
      shaderSharedFloat64AtomicMinMax: Default::default(),
      shaderImageFloat32AtomicMinMax: Default::default(),
      sparseImageFloat32AtomicMinMax: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderAtomicFloatFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderBufferFloat32Atomics: VkBool32,
  pub shaderBufferFloat32AtomicAdd: VkBool32,
  pub shaderBufferFloat64Atomics: VkBool32,
  pub shaderBufferFloat64AtomicAdd: VkBool32,
  pub shaderSharedFloat32Atomics: VkBool32,
  pub shaderSharedFloat32AtomicAdd: VkBool32,
  pub shaderSharedFloat64Atomics: VkBool32,
  pub shaderSharedFloat64AtomicAdd: VkBool32,
  pub shaderImageFloat32Atomics: VkBool32,
  pub shaderImageFloat32AtomicAdd: VkBool32,
  pub sparseImageFloat32Atomics: VkBool32,
  pub sparseImageFloat32AtomicAdd: VkBool32,
}
impl Default for VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      shaderBufferFloat32Atomics: Default::default(),
      shaderBufferFloat32AtomicAdd: Default::default(),
      shaderBufferFloat64Atomics: Default::default(),
      shaderBufferFloat64AtomicAdd: Default::default(),
      shaderSharedFloat32Atomics: Default::default(),
      shaderSharedFloat32AtomicAdd: Default::default(),
      shaderSharedFloat64Atomics: Default::default(),
      shaderSharedFloat64AtomicAdd: Default::default(),
      shaderImageFloat32Atomics: Default::default(),
      shaderImageFloat32AtomicAdd: Default::default(),
      sparseImageFloat32Atomics: Default::default(),
      sparseImageFloat32AtomicAdd: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderAtomicInt64Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicInt64Features.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderBufferInt64Atomics: VkBool32,
  pub shaderSharedInt64Atomics: VkBool32,
}
impl Default for VkPhysicalDeviceShaderAtomicInt64Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES,
      pNext: core::ptr::null_mut(),
      shaderBufferInt64Atomics: Default::default(),
      shaderSharedInt64Atomics: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderClockFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderClockFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderSubgroupClock: VkBool32,
  pub shaderDeviceClock: VkBool32,
}
impl Default for VkPhysicalDeviceShaderClockFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      shaderSubgroupClock: Default::default(),
      shaderDeviceClock: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderCoreBuiltins: VkBool32,
}
impl Default for VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM,
      pNext: core::ptr::null_mut(),
      shaderCoreBuiltins: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub shaderCoreMask: u64,
  /// * Limit Type: max
  pub shaderCoreCount: u32,
  /// * Limit Type: max
  pub shaderWarpsPerCore: u32,
}
impl Default for VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM,
      pNext: core::ptr::null_mut(),
      shaderCoreMask: Default::default(),
      shaderCoreCount: Default::default(),
      shaderWarpsPerCore: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCoreProperties2AMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreProperties2AMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD`
  pub sType: VkStructureType,
  /// Pointer to next structure
  /// * Optional: true
  pub pNext: *mut c_void,
  /// features supported by the shader core
  /// * Limit Type: bitmask
  pub shaderCoreFeatures: VkShaderCorePropertiesFlagsAMD,
  /// number of active compute units across all shader engines/arrays
  /// * Limit Type: max
  pub activeComputeUnitCount: u32,
}
impl Default for VkPhysicalDeviceShaderCoreProperties2AMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD,
      pNext: core::ptr::null_mut(),
      shaderCoreFeatures: Default::default(),
      activeComputeUnitCount: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCorePropertiesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// number of shader engines
  /// * Limit Type: exact
  pub shaderEngineCount: u32,
  /// number of shader arrays
  /// * Limit Type: exact
  pub shaderArraysPerEngineCount: u32,
  /// number of physical CUs per shader array
  /// * Limit Type: exact
  pub computeUnitsPerShaderArray: u32,
  /// number of SIMDs per compute unit
  /// * Limit Type: exact
  pub simdPerComputeUnit: u32,
  /// number of wavefront slots in each SIMD
  /// * Limit Type: exact
  pub wavefrontsPerSimd: u32,
  /// maximum number of threads per wavefront
  /// * Limit Type: max
  pub wavefrontSize: u32,
  /// number of physical SGPRs per SIMD
  /// * Limit Type: exact
  pub sgprsPerSimd: u32,
  /// minimum number of SGPRs that can be allocated by a wave
  /// * Limit Type: min
  pub minSgprAllocation: u32,
  /// number of available SGPRs
  /// * Limit Type: max
  pub maxSgprAllocation: u32,
  /// SGPRs are allocated in groups of this size
  /// * Limit Type: min,mul
  pub sgprAllocationGranularity: u32,
  /// number of physical VGPRs per SIMD
  /// * Limit Type: exact
  pub vgprsPerSimd: u32,
  /// minimum number of VGPRs that can be allocated by a wave
  /// * Limit Type: min
  pub minVgprAllocation: u32,
  /// number of available VGPRs
  /// * Limit Type: max
  pub maxVgprAllocation: u32,
  /// VGPRs are allocated in groups of this size
  /// * Limit Type: min,mul
  pub vgprAllocationGranularity: u32,
}
impl Default for VkPhysicalDeviceShaderCorePropertiesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD,
      pNext: core::ptr::null_mut(),
      shaderEngineCount: Default::default(),
      shaderArraysPerEngineCount: Default::default(),
      computeUnitsPerShaderArray: Default::default(),
      simdPerComputeUnit: Default::default(),
      wavefrontsPerSimd: Default::default(),
      wavefrontSize: Default::default(),
      sgprsPerSimd: Default::default(),
      minSgprAllocation: Default::default(),
      maxSgprAllocation: Default::default(),
      sgprAllocationGranularity: Default::default(),
      vgprsPerSimd: Default::default(),
      minVgprAllocation: Default::default(),
      maxVgprAllocation: Default::default(),
      vgprAllocationGranularity: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderCorePropertiesARM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesARM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesARM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub pixelRate: u32,
  /// * Limit Type: exact
  pub texelRate: u32,
  /// * Limit Type: exact
  pub fmaRate: u32,
}
impl Default for VkPhysicalDeviceShaderCorePropertiesARM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM,
      pNext: core::ptr::null_mut(),
      pixelRate: Default::default(),
      texelRate: Default::default(),
      fmaRate: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderDemoteToHelperInvocation: VkBool32,
}
impl Default for VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES,
      pNext: core::ptr::null_mut(),
      shaderDemoteToHelperInvocation: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderDrawParametersFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderDrawParameters: VkBool32,
}
impl Default for VkPhysicalDeviceShaderDrawParametersFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES,
      pNext: core::ptr::null_mut(),
      shaderDrawParameters: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub shaderEarlyAndLateFragmentTests: VkBool32,
}
impl Default for VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD,
      pNext: core::ptr::null_mut(),
      shaderEarlyAndLateFragmentTests: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderFloat16Int8Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderFloat16Int8Features.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// 16-bit floats (halfs) in shaders
  pub shaderFloat16: VkBool32,
  /// 8-bit integers in shaders
  pub shaderInt8: VkBool32,
}
impl Default for VkPhysicalDeviceShaderFloat16Int8Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES,
      pNext: core::ptr::null_mut(),
      shaderFloat16: Default::default(),
      shaderInt8: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderImageInt64Atomics: VkBool32,
  pub sparseImageInt64Atomics: VkBool32,
}
impl Default for VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      shaderImageInt64Atomics: Default::default(),
      sparseImageInt64Atomics: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderImageFootprintFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub imageFootprint: VkBool32,
}
impl Default for VkPhysicalDeviceShaderImageFootprintFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      imageFootprint: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderIntegerDotProductFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderIntegerDotProduct: VkBool32,
}
impl Default for VkPhysicalDeviceShaderIntegerDotProductFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES,
      pNext: core::ptr::null_mut(),
      shaderIntegerDotProduct: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderIntegerDotProductProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct8BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct16BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct32BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct64BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
}
impl Default for VkPhysicalDeviceShaderIntegerDotProductProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES,
      pNext: core::ptr::null_mut(),
      integerDotProduct8BitUnsignedAccelerated: Default::default(),
      integerDotProduct8BitSignedAccelerated: Default::default(),
      integerDotProduct8BitMixedSignednessAccelerated: Default::default(),
      integerDotProduct4x8BitPackedUnsignedAccelerated: Default::default(),
      integerDotProduct4x8BitPackedSignedAccelerated: Default::default(),
      integerDotProduct4x8BitPackedMixedSignednessAccelerated: Default::default(),
      integerDotProduct16BitUnsignedAccelerated: Default::default(),
      integerDotProduct16BitSignedAccelerated: Default::default(),
      integerDotProduct16BitMixedSignednessAccelerated: Default::default(),
      integerDotProduct32BitUnsignedAccelerated: Default::default(),
      integerDotProduct32BitSignedAccelerated: Default::default(),
      integerDotProduct32BitMixedSignednessAccelerated: Default::default(),
      integerDotProduct64BitUnsignedAccelerated: Default::default(),
      integerDotProduct64BitSignedAccelerated: Default::default(),
      integerDotProduct64BitMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating8BitSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating16BitSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating32BitSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating64BitSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderIntegerFunctions2: VkBool32,
}
impl Default for VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL,
      pNext: core::ptr::null_mut(),
      shaderIntegerFunctions2: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub shaderModuleIdentifier: VkBool32,
}
impl Default for VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      shaderModuleIdentifier: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  /// * Limit Type: noauto
  pub shaderModuleIdentifierAlgorithmUUID: [u8; VK_UUID_SIZE],
}
impl Default for VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      shaderModuleIdentifierAlgorithmUUID: [Default::default(); VK_UUID_SIZE],
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderSMBuiltinsFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderSMBuiltins: VkBool32,
}
impl Default for VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      shaderSMBuiltins: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderSMBuiltinsPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub shaderSMCount: u32,
  /// * Limit Type: max
  pub shaderWarpsPerSM: u32,
}
impl Default for VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      shaderSMCount: Default::default(),
      shaderWarpsPerSM: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Flag to specify whether subgroup operations with extended types are supported
  /// * No Auto-Validity
  pub shaderSubgroupExtendedTypes: VkBool32,
}
impl Default for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES,
      pNext: core::ptr::null_mut(),
      shaderSubgroupExtendedTypes: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderSubgroupUniformControlFlow: VkBool32,
}
impl Default for VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      shaderSubgroupUniformControlFlow: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShaderTerminateInvocationFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderTerminateInvocationFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub shaderTerminateInvocation: VkBool32,
}
impl Default for VkPhysicalDeviceShaderTerminateInvocationFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES,
      pNext: core::ptr::null_mut(),
      shaderTerminateInvocation: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShadingRateImageFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShadingRateImageFeaturesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shadingRateImage: VkBool32,
  pub shadingRateCoarseSampleOrder: VkBool32,
}
impl Default for VkPhysicalDeviceShadingRateImageFeaturesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV,
      pNext: core::ptr::null_mut(),
      shadingRateImage: Default::default(),
      shadingRateCoarseSampleOrder: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceShadingRateImagePropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShadingRateImagePropertiesNV.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub shadingRateTexelSize: VkExtent2D,
  /// * Limit Type: max
  pub shadingRatePaletteSize: u32,
  /// * Limit Type: max
  pub shadingRateMaxCoarseSamples: u32,
}
impl Default for VkPhysicalDeviceShadingRateImagePropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      shadingRateTexelSize: Default::default(),
      shadingRatePaletteSize: Default::default(),
      shadingRateMaxCoarseSamples: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSparseImageFormatInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2,
      pNext: core::ptr::null(),
      format: Default::default(),
      ty: Default::default(),
      samples: Default::default(),
      usage: Default::default(),
      tiling: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSparseProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
  /// Sparse resources support: GPU will access all 2D (single sample) sparse resources using the standard sparse image block shapes (based on pixel format)
  /// * Limit Type: bitmask
  pub residencyStandard2DBlockShape: VkBool32,
  /// Sparse resources support: GPU will access all 2D (multisample) sparse resources using the standard sparse image block shapes (based on pixel format)
  /// * Limit Type: bitmask
  pub residencyStandard2DMultisampleBlockShape: VkBool32,
  /// Sparse resources support: GPU will access all 3D sparse resources using the standard sparse image block shapes (based on pixel format)
  /// * Limit Type: bitmask
  pub residencyStandard3DBlockShape: VkBool32,
  /// Sparse resources support: Images with mip level dimensions that are NOT a multiple of the sparse image block dimensions will be placed in the mip tail
  /// * Limit Type: bitmask
  pub residencyAlignedMipSize: VkBool32,
  /// Sparse resources support: GPU can consistently access non-resident regions of a resource, all reads return as if data is 0, writes are discarded
  /// * Limit Type: bitmask
  pub residencyNonResidentStrict: VkBool32,
}
impl Default for VkPhysicalDeviceSparseProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      residencyStandard2DBlockShape: Default::default(),
      residencyStandard2DMultisampleBlockShape: Default::default(),
      residencyStandard3DBlockShape: Default::default(),
      residencyAlignedMipSize: Default::default(),
      residencyNonResidentStrict: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubgroupProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubgroupProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// The size of a subgroup for this queue.
  /// * Limit Type: max,pot
  /// * No Auto-Validity
  pub subgroupSize: u32,
  /// Bitfield of what shader stages support subgroup operations
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub supportedStages: VkShaderStageFlags,
  /// Bitfield of what subgroup operations are supported.
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub supportedOperations: VkSubgroupFeatureFlags,
  /// Flag to specify whether quad operations are available in all stages.
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub quadOperationsInAllStages: VkBool32,
}
impl Default for VkPhysicalDeviceSubgroupProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
      pNext: core::ptr::null_mut(),
      subgroupSize: Default::default(),
      supportedStages: Default::default(),
      supportedOperations: Default::default(),
      quadOperationsInAllStages: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubgroupSizeControlFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub subgroupSizeControl: VkBool32,
  pub computeFullSubgroups: VkBool32,
}
impl Default for VkPhysicalDeviceSubgroupSizeControlFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES,
      pNext: core::ptr::null_mut(),
      subgroupSizeControl: Default::default(),
      computeFullSubgroups: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubgroupSizeControlProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// The minimum subgroup size supported by this device
  /// * Limit Type: min,pot
  /// * No Auto-Validity
  pub minSubgroupSize: u32,
  /// The maximum subgroup size supported by this device
  /// * Limit Type: max,pot
  /// * No Auto-Validity
  pub maxSubgroupSize: u32,
  /// The maximum number of subgroups supported in a workgroup
  /// * Limit Type: max
  /// * No Auto-Validity
  pub maxComputeWorkgroupSubgroups: u32,
  /// The shader stages that support specifying a subgroup size
  /// * Limit Type: bitmask
  pub requiredSubgroupSizeStages: VkShaderStageFlags,
}
impl Default for VkPhysicalDeviceSubgroupSizeControlProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES,
      pNext: core::ptr::null_mut(),
      minSubgroupSize: Default::default(),
      maxSubgroupSize: Default::default(),
      maxComputeWorkgroupSubgroups: Default::default(),
      requiredSubgroupSizeStages: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub subpassMergeFeedback: VkBool32,
}
impl Default for VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      subpassMergeFeedback: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubpassShadingFeaturesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub subpassShading: VkBool32,
}
impl Default for VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI,
      pNext: core::ptr::null_mut(),
      subpassShading: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSubpassShadingPropertiesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max,pot
  pub maxSubpassShadingWorkgroupSizeAspectRatio: u32,
}
impl Default for VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI,
      pNext: core::ptr::null_mut(),
      maxSubpassShadingWorkgroupSizeAspectRatio: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSurfaceInfo2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub surface: VkSurfaceKHR,
}
impl Default for VkPhysicalDeviceSurfaceInfo2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
      pNext: core::ptr::null(),
      surface: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub swapchainMaintenance1: VkBool32,
}
impl Default for VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      swapchainMaintenance1: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceSynchronization2Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSynchronization2Features.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSynchronization2Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub synchronization2: VkBool32,
}
impl Default for VkPhysicalDeviceSynchronization2Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
      pNext: core::ptr::null_mut(),
      synchronization2: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub texelBufferAlignment: VkBool32,
}
impl Default for VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      texelBufferAlignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTexelBufferAlignmentProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: min,pot
  pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
  /// * Limit Type: exact
  pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
  /// * Limit Type: min,pot
  pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
  /// * Limit Type: exact
  pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
}
impl Default for VkPhysicalDeviceTexelBufferAlignmentProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES,
      pNext: core::ptr::null_mut(),
      storageTexelBufferOffsetAlignmentBytes: Default::default(),
      storageTexelBufferOffsetSingleTexelAlignment: Default::default(),
      uniformTexelBufferOffsetAlignmentBytes: Default::default(),
      uniformTexelBufferOffsetSingleTexelAlignment: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTextureCompressionASTCHDRFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub textureCompressionASTC_HDR: VkBool32,
}
impl Default for VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES,
      pNext: core::ptr::null_mut(),
      textureCompressionASTC_HDR: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTilePropertiesFeaturesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTilePropertiesFeaturesQCOM.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTilePropertiesFeaturesQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub tileProperties: VkBool32,
}
impl Default for VkPhysicalDeviceTilePropertiesFeaturesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM,
      pNext: core::ptr::null_mut(),
      tileProperties: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTimelineSemaphoreFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub timelineSemaphore: VkBool32,
}
impl Default for VkPhysicalDeviceTimelineSemaphoreFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES,
      pNext: core::ptr::null_mut(),
      timelineSemaphore: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTimelineSemaphoreProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreProperties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxTimelineSemaphoreValueDifference: u64,
}
impl Default for VkPhysicalDeviceTimelineSemaphoreProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES,
      pNext: core::ptr::null_mut(),
      maxTimelineSemaphoreValueDifference: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceToolProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceToolProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceToolProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub name: [u8; VK_MAX_EXTENSION_NAME_SIZE],
  pub version: [u8; VK_MAX_EXTENSION_NAME_SIZE],
  pub purposes: VkToolPurposeFlags,
  pub description: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub layer: [u8; VK_MAX_EXTENSION_NAME_SIZE],
}
impl Default for VkPhysicalDeviceToolProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES,
      pNext: core::ptr::null_mut(),
      name: [Default::default(); VK_MAX_EXTENSION_NAME_SIZE],
      version: [Default::default(); VK_MAX_EXTENSION_NAME_SIZE],
      purposes: Default::default(),
      description: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      layer: [Default::default(); VK_MAX_EXTENSION_NAME_SIZE],
    }
  }
}

/// Khronos: [VkPhysicalDeviceTransformFeedbackFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub transformFeedback: VkBool32,
  pub geometryStreams: VkBool32,
}
impl Default for VkPhysicalDeviceTransformFeedbackFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      transformFeedback: Default::default(),
      geometryStreams: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceTransformFeedbackPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub maxTransformFeedbackStreams: u32,
  /// * Limit Type: max
  pub maxTransformFeedbackBuffers: u32,
  /// * Limit Type: max
  pub maxTransformFeedbackBufferSize: VkDeviceSize,
  /// * Limit Type: max
  pub maxTransformFeedbackStreamDataSize: u32,
  /// * Limit Type: max
  pub maxTransformFeedbackBufferDataSize: u32,
  /// * Limit Type: max
  pub maxTransformFeedbackBufferDataStride: u32,
  /// * Limit Type: bitmask
  pub transformFeedbackQueries: VkBool32,
  /// * Limit Type: bitmask
  pub transformFeedbackStreamsLinesTriangles: VkBool32,
  /// * Limit Type: bitmask
  pub transformFeedbackRasterizationStreamSelect: VkBool32,
  /// * Limit Type: bitmask
  pub transformFeedbackDraw: VkBool32,
}
impl Default for VkPhysicalDeviceTransformFeedbackPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      maxTransformFeedbackStreams: Default::default(),
      maxTransformFeedbackBuffers: Default::default(),
      maxTransformFeedbackBufferSize: Default::default(),
      maxTransformFeedbackStreamDataSize: Default::default(),
      maxTransformFeedbackBufferDataSize: Default::default(),
      maxTransformFeedbackBufferDataStride: Default::default(),
      transformFeedbackQueries: Default::default(),
      transformFeedbackStreamsLinesTriangles: Default::default(),
      transformFeedbackRasterizationStreamSelect: Default::default(),
      transformFeedbackDraw: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceUniformBufferStandardLayoutFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub uniformBufferStandardLayout: VkBool32,
}
impl Default for VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES,
      pNext: core::ptr::null_mut(),
      uniformBufferStandardLayout: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVariablePointersFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVariablePointersFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub variablePointersStorageBuffer: VkBool32,
  pub variablePointers: VkBool32,
}
impl Default for VkPhysicalDeviceVariablePointersFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES,
      pNext: core::ptr::null_mut(),
      variablePointersStorageBuffer: Default::default(),
      variablePointers: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub vertexAttributeInstanceRateDivisor: VkBool32,
  pub vertexAttributeInstanceRateZeroDivisor: VkBool32,
}
impl Default for VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      vertexAttributeInstanceRateDivisor: Default::default(),
      vertexAttributeInstanceRateZeroDivisor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// max value of vertex attribute divisor
  /// * Limit Type: max
  pub maxVertexAttribDivisor: u32,
}
impl Default for VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT,
      pNext: core::ptr::null_mut(),
      maxVertexAttribDivisor: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub vertexInputDynamicState: VkBool32,
}
impl Default for VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      vertexInputDynamicState: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVideoFormatInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVideoFormatInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVideoFormatInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub imageUsage: VkImageUsageFlags,
}
impl Default for VkPhysicalDeviceVideoFormatInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR,
      pNext: core::ptr::null(),
      imageUsage: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan11Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Features.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// 16-bit integer/floating-point variables supported in BufferBlock
  pub storageBuffer16BitAccess: VkBool32,
  /// 16-bit integer/floating-point variables supported in BufferBlock and Block
  pub uniformAndStorageBuffer16BitAccess: VkBool32,
  /// 16-bit integer/floating-point variables supported in PushConstant
  pub storagePushConstant16: VkBool32,
  /// 16-bit integer/floating-point variables supported in shader inputs and outputs
  pub storageInputOutput16: VkBool32,
  /// Multiple views in a renderpass
  pub multiview: VkBool32,
  /// Multiple views in a renderpass w/ geometry shader
  pub multiviewGeometryShader: VkBool32,
  /// Multiple views in a renderpass w/ tessellation shader
  pub multiviewTessellationShader: VkBool32,
  pub variablePointersStorageBuffer: VkBool32,
  pub variablePointers: VkBool32,
  pub protectedMemory: VkBool32,
  /// Sampler color conversion supported
  pub samplerYcbcrConversion: VkBool32,
  pub shaderDrawParameters: VkBool32,
}
impl Default for VkPhysicalDeviceVulkan11Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
      pNext: core::ptr::null_mut(),
      storageBuffer16BitAccess: Default::default(),
      uniformAndStorageBuffer16BitAccess: Default::default(),
      storagePushConstant16: Default::default(),
      storageInputOutput16: Default::default(),
      multiview: Default::default(),
      multiviewGeometryShader: Default::default(),
      multiviewTessellationShader: Default::default(),
      variablePointersStorageBuffer: Default::default(),
      variablePointers: Default::default(),
      protectedMemory: Default::default(),
      samplerYcbcrConversion: Default::default(),
      shaderDrawParameters: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan11Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Properties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Properties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: exact
  pub deviceUUID: [u8; VK_UUID_SIZE],
  /// * Limit Type: exact
  pub driverUUID: [u8; VK_UUID_SIZE],
  /// * Limit Type: exact
  pub deviceLUID: [u8; VK_LUID_SIZE],
  /// * Limit Type: exact
  pub deviceNodeMask: u32,
  /// * Limit Type: exact
  pub deviceLUIDValid: VkBool32,
  /// The size of a subgroup for this queue.
  /// * Limit Type: max,pot
  /// * No Auto-Validity
  pub subgroupSize: u32,
  /// Bitfield of what shader stages support subgroup operations
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub subgroupSupportedStages: VkShaderStageFlags,
  /// Bitfield of what subgroup operations are supported.
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub subgroupSupportedOperations: VkSubgroupFeatureFlags,
  /// Flag to specify whether quad operations are available in all stages.
  /// * Limit Type: bitmask
  /// * No Auto-Validity
  pub subgroupQuadOperationsInAllStages: VkBool32,
  /// * Limit Type: exact
  pub pointClippingBehavior: VkPointClippingBehavior,
  /// max number of views in a subpass
  /// * Limit Type: max
  pub maxMultiviewViewCount: u32,
  /// max instance index for a draw in a multiview subpass
  /// * Limit Type: max
  pub maxMultiviewInstanceIndex: u32,
  /// * Limit Type: exact
  pub protectedNoFault: VkBool32,
  /// * Limit Type: max
  pub maxPerSetDescriptors: u32,
  /// * Limit Type: max
  pub maxMemoryAllocationSize: VkDeviceSize,
}
impl Default for VkPhysicalDeviceVulkan11Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES,
      pNext: core::ptr::null_mut(),
      deviceUUID: [Default::default(); VK_UUID_SIZE],
      driverUUID: [Default::default(); VK_UUID_SIZE],
      deviceLUID: [Default::default(); VK_LUID_SIZE],
      deviceNodeMask: Default::default(),
      deviceLUIDValid: Default::default(),
      subgroupSize: Default::default(),
      subgroupSupportedStages: Default::default(),
      subgroupSupportedOperations: Default::default(),
      subgroupQuadOperationsInAllStages: Default::default(),
      pointClippingBehavior: Default::default(),
      maxMultiviewViewCount: Default::default(),
      maxMultiviewInstanceIndex: Default::default(),
      protectedNoFault: Default::default(),
      maxPerSetDescriptors: Default::default(),
      maxMemoryAllocationSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan12Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Features.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub samplerMirrorClampToEdge: VkBool32,
  pub drawIndirectCount: VkBool32,
  /// 8-bit integer variables supported in StorageBuffer
  pub storageBuffer8BitAccess: VkBool32,
  /// 8-bit integer variables supported in StorageBuffer and Uniform
  pub uniformAndStorageBuffer8BitAccess: VkBool32,
  /// 8-bit integer variables supported in PushConstant
  pub storagePushConstant8: VkBool32,
  pub shaderBufferInt64Atomics: VkBool32,
  pub shaderSharedInt64Atomics: VkBool32,
  /// 16-bit floats (halfs) in shaders
  pub shaderFloat16: VkBool32,
  /// 8-bit integers in shaders
  pub shaderInt8: VkBool32,
  pub descriptorIndexing: VkBool32,
  pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
  pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
  pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
  pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
  pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
  pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
  pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
  pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
  pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
  pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
  pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
  pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
  pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
  pub descriptorBindingPartiallyBound: VkBool32,
  pub descriptorBindingVariableDescriptorCount: VkBool32,
  pub runtimeDescriptorArray: VkBool32,
  pub samplerFilterMinmax: VkBool32,
  pub scalarBlockLayout: VkBool32,
  pub imagelessFramebuffer: VkBool32,
  pub uniformBufferStandardLayout: VkBool32,
  pub shaderSubgroupExtendedTypes: VkBool32,
  pub separateDepthStencilLayouts: VkBool32,
  pub hostQueryReset: VkBool32,
  pub timelineSemaphore: VkBool32,
  pub bufferDeviceAddress: VkBool32,
  pub bufferDeviceAddressCaptureReplay: VkBool32,
  pub bufferDeviceAddressMultiDevice: VkBool32,
  pub vulkanMemoryModel: VkBool32,
  pub vulkanMemoryModelDeviceScope: VkBool32,
  pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
  pub shaderOutputViewportIndex: VkBool32,
  pub shaderOutputLayer: VkBool32,
  pub subgroupBroadcastDynamicId: VkBool32,
}
impl Default for VkPhysicalDeviceVulkan12Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
      pNext: core::ptr::null_mut(),
      samplerMirrorClampToEdge: Default::default(),
      drawIndirectCount: Default::default(),
      storageBuffer8BitAccess: Default::default(),
      uniformAndStorageBuffer8BitAccess: Default::default(),
      storagePushConstant8: Default::default(),
      shaderBufferInt64Atomics: Default::default(),
      shaderSharedInt64Atomics: Default::default(),
      shaderFloat16: Default::default(),
      shaderInt8: Default::default(),
      descriptorIndexing: Default::default(),
      shaderInputAttachmentArrayDynamicIndexing: Default::default(),
      shaderUniformTexelBufferArrayDynamicIndexing: Default::default(),
      shaderStorageTexelBufferArrayDynamicIndexing: Default::default(),
      shaderUniformBufferArrayNonUniformIndexing: Default::default(),
      shaderSampledImageArrayNonUniformIndexing: Default::default(),
      shaderStorageBufferArrayNonUniformIndexing: Default::default(),
      shaderStorageImageArrayNonUniformIndexing: Default::default(),
      shaderInputAttachmentArrayNonUniformIndexing: Default::default(),
      shaderUniformTexelBufferArrayNonUniformIndexing: Default::default(),
      shaderStorageTexelBufferArrayNonUniformIndexing: Default::default(),
      descriptorBindingUniformBufferUpdateAfterBind: Default::default(),
      descriptorBindingSampledImageUpdateAfterBind: Default::default(),
      descriptorBindingStorageImageUpdateAfterBind: Default::default(),
      descriptorBindingStorageBufferUpdateAfterBind: Default::default(),
      descriptorBindingUniformTexelBufferUpdateAfterBind: Default::default(),
      descriptorBindingStorageTexelBufferUpdateAfterBind: Default::default(),
      descriptorBindingUpdateUnusedWhilePending: Default::default(),
      descriptorBindingPartiallyBound: Default::default(),
      descriptorBindingVariableDescriptorCount: Default::default(),
      runtimeDescriptorArray: Default::default(),
      samplerFilterMinmax: Default::default(),
      scalarBlockLayout: Default::default(),
      imagelessFramebuffer: Default::default(),
      uniformBufferStandardLayout: Default::default(),
      shaderSubgroupExtendedTypes: Default::default(),
      separateDepthStencilLayouts: Default::default(),
      hostQueryReset: Default::default(),
      timelineSemaphore: Default::default(),
      bufferDeviceAddress: Default::default(),
      bufferDeviceAddressCaptureReplay: Default::default(),
      bufferDeviceAddressMultiDevice: Default::default(),
      vulkanMemoryModel: Default::default(),
      vulkanMemoryModelDeviceScope: Default::default(),
      vulkanMemoryModelAvailabilityVisibilityChains: Default::default(),
      shaderOutputViewportIndex: Default::default(),
      shaderOutputLayer: Default::default(),
      subgroupBroadcastDynamicId: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan12Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Properties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Properties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: noauto
  pub driverID: VkDriverId,
  /// * Limit Type: noauto
  pub driverName: [u8; VK_MAX_DRIVER_NAME_SIZE],
  /// * Limit Type: noauto
  pub driverInfo: [u8; VK_MAX_DRIVER_INFO_SIZE],
  /// * Limit Type: noauto
  pub conformanceVersion: VkConformanceVersion,
  /// * Limit Type: exact
  pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
  /// * Limit Type: exact
  pub roundingModeIndependence: VkShaderFloatControlsIndependence,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
  /// An implementation can preserve signed zero, nan, inf
  /// * Limit Type: bitmask
  pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shaderDenormPreserveFloat16: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shaderDenormPreserveFloat32: VkBool32,
  /// An implementation can preserve  denormals
  /// * Limit Type: bitmask
  pub shaderDenormPreserveFloat64: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shaderDenormFlushToZeroFloat16: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shaderDenormFlushToZeroFloat32: VkBool32,
  /// An implementation can flush to zero  denormals
  /// * Limit Type: bitmask
  pub shaderDenormFlushToZeroFloat64: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTEFloat16: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTEFloat32: VkBool32,
  /// An implementation can support RTE
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTEFloat64: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTZFloat16: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTZFloat32: VkBool32,
  /// An implementation can support RTZ
  /// * Limit Type: bitmask
  pub shaderRoundingModeRTZFloat64: VkBool32,
  /// * Limit Type: max
  pub maxUpdateAfterBindDescriptorsInAllPools: u32,
  /// * Limit Type: bitmask
  pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
  /// * Limit Type: bitmask
  pub robustBufferAccessUpdateAfterBind: VkBool32,
  /// * Limit Type: bitmask
  pub quadDivergentImplicitLod: VkBool32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
  /// * Limit Type: max
  pub maxPerStageUpdateAfterBindResources: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindSamplers: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
  /// supported depth resolve modes
  /// * Limit Type: bitmask
  pub supportedDepthResolveModes: VkResolveModeFlags,
  /// supported stencil resolve modes
  /// * Limit Type: bitmask
  pub supportedStencilResolveModes: VkResolveModeFlags,
  /// depth and stencil resolve modes can be set independently if one of them is none
  /// * Limit Type: bitmask
  pub independentResolveNone: VkBool32,
  /// depth and stencil resolve modes can be set independently
  /// * Limit Type: bitmask
  pub independentResolve: VkBool32,
  /// * Limit Type: bitmask
  pub filterMinmaxSingleComponentFormats: VkBool32,
  /// * Limit Type: bitmask
  pub filterMinmaxImageComponentMapping: VkBool32,
  /// * Limit Type: max
  pub maxTimelineSemaphoreValueDifference: u64,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub framebufferIntegerColorSampleCounts: VkSampleCountFlags,
}
impl Default for VkPhysicalDeviceVulkan12Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES,
      pNext: core::ptr::null_mut(),
      driverID: Default::default(),
      driverName: [Default::default(); VK_MAX_DRIVER_NAME_SIZE],
      driverInfo: [Default::default(); VK_MAX_DRIVER_INFO_SIZE],
      conformanceVersion: Default::default(),
      denormBehaviorIndependence: Default::default(),
      roundingModeIndependence: Default::default(),
      shaderSignedZeroInfNanPreserveFloat16: Default::default(),
      shaderSignedZeroInfNanPreserveFloat32: Default::default(),
      shaderSignedZeroInfNanPreserveFloat64: Default::default(),
      shaderDenormPreserveFloat16: Default::default(),
      shaderDenormPreserveFloat32: Default::default(),
      shaderDenormPreserveFloat64: Default::default(),
      shaderDenormFlushToZeroFloat16: Default::default(),
      shaderDenormFlushToZeroFloat32: Default::default(),
      shaderDenormFlushToZeroFloat64: Default::default(),
      shaderRoundingModeRTEFloat16: Default::default(),
      shaderRoundingModeRTEFloat32: Default::default(),
      shaderRoundingModeRTEFloat64: Default::default(),
      shaderRoundingModeRTZFloat16: Default::default(),
      shaderRoundingModeRTZFloat32: Default::default(),
      shaderRoundingModeRTZFloat64: Default::default(),
      maxUpdateAfterBindDescriptorsInAllPools: Default::default(),
      shaderUniformBufferArrayNonUniformIndexingNative: Default::default(),
      shaderSampledImageArrayNonUniformIndexingNative: Default::default(),
      shaderStorageBufferArrayNonUniformIndexingNative: Default::default(),
      shaderStorageImageArrayNonUniformIndexingNative: Default::default(),
      shaderInputAttachmentArrayNonUniformIndexingNative: Default::default(),
      robustBufferAccessUpdateAfterBind: Default::default(),
      quadDivergentImplicitLod: Default::default(),
      maxPerStageDescriptorUpdateAfterBindSamplers: Default::default(),
      maxPerStageDescriptorUpdateAfterBindUniformBuffers: Default::default(),
      maxPerStageDescriptorUpdateAfterBindStorageBuffers: Default::default(),
      maxPerStageDescriptorUpdateAfterBindSampledImages: Default::default(),
      maxPerStageDescriptorUpdateAfterBindStorageImages: Default::default(),
      maxPerStageDescriptorUpdateAfterBindInputAttachments: Default::default(),
      maxPerStageUpdateAfterBindResources: Default::default(),
      maxDescriptorSetUpdateAfterBindSamplers: Default::default(),
      maxDescriptorSetUpdateAfterBindUniformBuffers: Default::default(),
      maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: Default::default(),
      maxDescriptorSetUpdateAfterBindStorageBuffers: Default::default(),
      maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: Default::default(),
      maxDescriptorSetUpdateAfterBindSampledImages: Default::default(),
      maxDescriptorSetUpdateAfterBindStorageImages: Default::default(),
      maxDescriptorSetUpdateAfterBindInputAttachments: Default::default(),
      supportedDepthResolveModes: Default::default(),
      supportedStencilResolveModes: Default::default(),
      independentResolveNone: Default::default(),
      independentResolve: Default::default(),
      filterMinmaxSingleComponentFormats: Default::default(),
      filterMinmaxImageComponentMapping: Default::default(),
      maxTimelineSemaphoreValueDifference: Default::default(),
      framebufferIntegerColorSampleCounts: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan13Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Features.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub robustImageAccess: VkBool32,
  pub inlineUniformBlock: VkBool32,
  pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
  pub pipelineCreationCacheControl: VkBool32,
  pub privateData: VkBool32,
  pub shaderDemoteToHelperInvocation: VkBool32,
  pub shaderTerminateInvocation: VkBool32,
  pub subgroupSizeControl: VkBool32,
  pub computeFullSubgroups: VkBool32,
  pub synchronization2: VkBool32,
  pub textureCompressionASTC_HDR: VkBool32,
  pub shaderZeroInitializeWorkgroupMemory: VkBool32,
  pub dynamicRendering: VkBool32,
  pub shaderIntegerDotProduct: VkBool32,
  pub maintenance4: VkBool32,
}
impl Default for VkPhysicalDeviceVulkan13Features {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
      pNext: core::ptr::null_mut(),
      robustImageAccess: Default::default(),
      inlineUniformBlock: Default::default(),
      descriptorBindingInlineUniformBlockUpdateAfterBind: Default::default(),
      pipelineCreationCacheControl: Default::default(),
      privateData: Default::default(),
      shaderDemoteToHelperInvocation: Default::default(),
      shaderTerminateInvocation: Default::default(),
      subgroupSizeControl: Default::default(),
      computeFullSubgroups: Default::default(),
      synchronization2: Default::default(),
      textureCompressionASTC_HDR: Default::default(),
      shaderZeroInitializeWorkgroupMemory: Default::default(),
      dynamicRendering: Default::default(),
      shaderIntegerDotProduct: Default::default(),
      maintenance4: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkan13Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Properties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Properties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// The minimum subgroup size supported by this device
  /// * Limit Type: min,pot
  /// * No Auto-Validity
  pub minSubgroupSize: u32,
  /// The maximum subgroup size supported by this device
  /// * Limit Type: max,pot
  /// * No Auto-Validity
  pub maxSubgroupSize: u32,
  /// The maximum number of subgroups supported in a workgroup
  /// * Limit Type: max
  /// * No Auto-Validity
  pub maxComputeWorkgroupSubgroups: u32,
  /// The shader stages that support specifying a subgroup size
  /// * Limit Type: bitmask
  pub requiredSubgroupSizeStages: VkShaderStageFlags,
  /// * Limit Type: max
  pub maxInlineUniformBlockSize: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorInlineUniformBlocks: u32,
  /// * Limit Type: max
  pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
  /// * Limit Type: max
  pub maxDescriptorSetInlineUniformBlocks: u32,
  /// * Limit Type: max
  pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
  /// * Limit Type: max
  pub maxInlineUniformTotalSize: u32,
  /// * Limit Type: bitmask
  pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct8BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct16BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct32BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct64BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
  /// * Limit Type: bitmask
  pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
  /// * Limit Type: min,pot
  pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
  /// * Limit Type: exact
  pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
  /// * Limit Type: min,pot
  pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
  /// * Limit Type: exact
  pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
  /// * Limit Type: max
  pub maxBufferSize: VkDeviceSize,
}
impl Default for VkPhysicalDeviceVulkan13Properties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES,
      pNext: core::ptr::null_mut(),
      minSubgroupSize: Default::default(),
      maxSubgroupSize: Default::default(),
      maxComputeWorkgroupSubgroups: Default::default(),
      requiredSubgroupSizeStages: Default::default(),
      maxInlineUniformBlockSize: Default::default(),
      maxPerStageDescriptorInlineUniformBlocks: Default::default(),
      maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: Default::default(),
      maxDescriptorSetInlineUniformBlocks: Default::default(),
      maxDescriptorSetUpdateAfterBindInlineUniformBlocks: Default::default(),
      maxInlineUniformTotalSize: Default::default(),
      integerDotProduct8BitUnsignedAccelerated: Default::default(),
      integerDotProduct8BitSignedAccelerated: Default::default(),
      integerDotProduct8BitMixedSignednessAccelerated: Default::default(),
      integerDotProduct4x8BitPackedUnsignedAccelerated: Default::default(),
      integerDotProduct4x8BitPackedSignedAccelerated: Default::default(),
      integerDotProduct4x8BitPackedMixedSignednessAccelerated: Default::default(),
      integerDotProduct16BitUnsignedAccelerated: Default::default(),
      integerDotProduct16BitSignedAccelerated: Default::default(),
      integerDotProduct16BitMixedSignednessAccelerated: Default::default(),
      integerDotProduct32BitUnsignedAccelerated: Default::default(),
      integerDotProduct32BitSignedAccelerated: Default::default(),
      integerDotProduct32BitMixedSignednessAccelerated: Default::default(),
      integerDotProduct64BitUnsignedAccelerated: Default::default(),
      integerDotProduct64BitSignedAccelerated: Default::default(),
      integerDotProduct64BitMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating8BitSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating16BitSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating32BitSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating64BitSignedAccelerated: Default::default(),
      integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: Default::default(),
      storageTexelBufferOffsetAlignmentBytes: Default::default(),
      storageTexelBufferOffsetSingleTexelAlignment: Default::default(),
      uniformTexelBufferOffsetAlignmentBytes: Default::default(),
      uniformTexelBufferOffsetSingleTexelAlignment: Default::default(),
      maxBufferSize: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceVulkanMemoryModelFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub vulkanMemoryModel: VkBool32,
  pub vulkanMemoryModelDeviceScope: VkBool32,
  pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
}
impl Default for VkPhysicalDeviceVulkanMemoryModelFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES,
      pNext: core::ptr::null_mut(),
      vulkanMemoryModel: Default::default(),
      vulkanMemoryModelDeviceScope: Default::default(),
      vulkanMemoryModelAvailabilityVisibilityChains: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub workgroupMemoryExplicitLayout: VkBool32,
  pub workgroupMemoryExplicitLayoutScalarBlockLayout: VkBool32,
  pub workgroupMemoryExplicitLayout8BitAccess: VkBool32,
  pub workgroupMemoryExplicitLayout16BitAccess: VkBool32,
}
impl Default for VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR,
      pNext: core::ptr::null_mut(),
      workgroupMemoryExplicitLayout: Default::default(),
      workgroupMemoryExplicitLayoutScalarBlockLayout: Default::default(),
      workgroupMemoryExplicitLayout8BitAccess: Default::default(),
      workgroupMemoryExplicitLayout16BitAccess: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub ycbcr2plane444Formats: VkBool32,
}
impl Default for VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      ycbcr2plane444Formats: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceYcbcrImageArraysFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub ycbcrImageArrays: VkBool32,
}
impl Default for VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT,
      pNext: core::ptr::null_mut(),
      ycbcrImageArrays: Default::default(),
    }
  }
}

/// Khronos: [VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderZeroInitializeWorkgroupMemory: VkBool32,
}
impl Default for VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES,
      pNext: core::ptr::null_mut(),
      shaderZeroInitializeWorkgroupMemory: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCacheCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineCacheCreateFlags,
  /// Size of initial data to populate cache, in bytes
  /// * Optional: true
  pub initialDataSize: c_size_t,
  /// Initial data to populate cache
  /// * Len: `initialDataSize`
  pub pInitialData: *const c_void,
}
impl Default for VkPipelineCacheCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      initialDataSize: Default::default(),
      pInitialData: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineCacheHeaderVersionOne](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersionOne.html)
///
/// The fields in this structure are non-normative since structure packing is implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheHeaderVersionOne {
  pub headerSize: u32,
  pub headerVersion: VkPipelineCacheHeaderVersion,
  pub vendorID: u32,
  pub deviceID: u32,
  pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
}
impl Default for VkPipelineCacheHeaderVersionOne {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      headerSize: Default::default(),
      headerVersion: Default::default(),
      vendorID: Default::default(),
      deviceID: Default::default(),
      pipelineCacheUUID: [Default::default(); VK_UUID_SIZE],
    }
  }
}

/// Khronos: [VkPipelineCacheHeaderVersionSafetyCriticalOne](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersionSafetyCriticalOne.html)
///
/// The fields in this structure are non-normative since structure packing is implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheHeaderVersionSafetyCriticalOne {
  pub headerVersionOne: VkPipelineCacheHeaderVersionOne,
  pub validationVersion: VkPipelineCacheValidationVersion,
  pub implementationData: u32,
  pub pipelineIndexCount: u32,
  pub pipelineIndexStride: u32,
  pub pipelineIndexOffset: u64,
}
impl Default for VkPipelineCacheHeaderVersionSafetyCriticalOne {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      headerVersionOne: Default::default(),
      validationVersion: Default::default(),
      implementationData: Default::default(),
      pipelineIndexCount: Default::default(),
      pipelineIndexStride: Default::default(),
      pipelineIndexOffset: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCacheSafetyCriticalIndexEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheSafetyCriticalIndexEntry.html)
///
/// The fields in this structure are non-normative since structure packing is implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheSafetyCriticalIndexEntry {
  pub pipelineIdentifier: [u8; VK_UUID_SIZE],
  pub pipelineMemorySize: u64,
  pub jsonSize: u64,
  pub jsonOffset: u64,
  pub stageIndexCount: u32,
  pub stageIndexStride: u32,
  pub stageIndexOffset: u64,
}
impl Default for VkPipelineCacheSafetyCriticalIndexEntry {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      pipelineIdentifier: [Default::default(); VK_UUID_SIZE],
      pipelineMemorySize: Default::default(),
      jsonSize: Default::default(),
      jsonOffset: Default::default(),
      stageIndexCount: Default::default(),
      stageIndexStride: Default::default(),
      stageIndexOffset: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCacheStageValidationIndexEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheStageValidationIndexEntry.html)
///
/// The fields in this structure are non-normative since structure packing is implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheStageValidationIndexEntry {
  pub codeSize: u64,
  pub codeOffset: u64,
}
impl Default for VkPipelineCacheStageValidationIndexEntry {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      codeSize: Default::default(),
      codeOffset: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineColorBlendAdvancedStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineColorBlendStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcPremultiplied: VkBool32,
  pub dstPremultiplied: VkBool32,
  pub blendOverlap: VkBlendOverlapEXT,
}
impl Default for VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      srcPremultiplied: Default::default(),
      dstPremultiplied: Default::default(),
      blendOverlap: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineColorBlendAttachmentState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAttachmentState.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
  pub blendEnable: VkBool32,
  pub srcColorBlendFactor: VkBlendFactor,
  pub dstColorBlendFactor: VkBlendFactor,
  pub colorBlendOp: VkBlendOp,
  pub srcAlphaBlendFactor: VkBlendFactor,
  pub dstAlphaBlendFactor: VkBlendFactor,
  pub alphaBlendOp: VkBlendOp,
  /// * Optional: true
  pub colorWriteMask: VkColorComponentFlags,
}
impl Default for VkPipelineColorBlendAttachmentState {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      blendEnable: Default::default(),
      srcColorBlendFactor: Default::default(),
      dstColorBlendFactor: Default::default(),
      colorBlendOp: Default::default(),
      srcAlphaBlendFactor: Default::default(),
      dstAlphaBlendFactor: Default::default(),
      alphaBlendOp: Default::default(),
      colorWriteMask: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineColorBlendStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineColorBlendStateCreateFlags,
  pub logicOpEnable: VkBool32,
  /// * No Auto-Validity
  pub logicOp: VkLogicOp,
  /// # of pAttachments
  /// * Optional: true
  pub attachmentCount: u32,
  /// * Optional: true
  /// * Len: `attachmentCount`
  pub pAttachments: *const VkPipelineColorBlendAttachmentState,
  pub blendConstants: [c_float; 4],
}
impl Default for VkPipelineColorBlendStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      logicOpEnable: Default::default(),
      logicOp: Default::default(),
      attachmentCount: Default::default(),
      pAttachments: core::ptr::null(),
      blendConstants: [Default::default(); 4],
    }
  }
}

/// Khronos: [VkPipelineColorWriteCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorWriteCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineColorBlendStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorWriteCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// # of pAttachments
  /// * Optional: true
  pub attachmentCount: u32,
  /// * Len: `attachmentCount`
  pub pColorWriteEnables: *const VkBool32,
}
impl Default for VkPipelineColorWriteCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      attachmentCount: Default::default(),
      pColorWriteEnables: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineCompilerControlCreateInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
/// * Struct Extends: [`VkComputePipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCompilerControlCreateInfoAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub compilerControlFlags: VkPipelineCompilerControlFlagsAMD,
}
impl Default for VkPipelineCompilerControlCreateInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD,
      pNext: core::ptr::null(),
      compilerControlFlags: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCoverageModulationStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
  pub coverageModulationMode: VkCoverageModulationModeNV,
  pub coverageModulationTableEnable: VkBool32,
  /// * Optional: true
  pub coverageModulationTableCount: u32,
  /// * Optional: true
  /// * Len: `coverageModulationTableCount`
  /// * No Auto-Validity
  pub pCoverageModulationTable: *const c_float,
}
impl Default for VkPipelineCoverageModulationStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      flags: Default::default(),
      coverageModulationMode: Default::default(),
      coverageModulationTableEnable: Default::default(),
      coverageModulationTableCount: Default::default(),
      pCoverageModulationTable: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineCoverageReductionStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageReductionStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCoverageReductionStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineCoverageReductionStateCreateFlagsNV,
  pub coverageReductionMode: VkCoverageReductionModeNV,
}
impl Default for VkPipelineCoverageReductionStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      flags: Default::default(),
      coverageReductionMode: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCoverageToColorStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
  pub coverageToColorEnable: VkBool32,
  /// * Optional: true
  pub coverageToColorLocation: u32,
}
impl Default for VkPipelineCoverageToColorStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      flags: Default::default(),
      coverageToColorEnable: Default::default(),
      coverageToColorLocation: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCreationFeedback](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedback.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCreationFeedback {
  pub flags: VkPipelineCreationFeedbackFlags,
  pub duration: u64,
}
impl Default for VkPipelineCreationFeedback {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      flags: Default::default(),
      duration: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineCreationFeedbackCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackCreateInfo.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
/// * Struct Extends: [`VkComputePipelineCreateInfo`]
/// * Struct Extends: [`VkRayTracingPipelineCreateInfoNV`]
/// * Struct Extends: [`VkRayTracingPipelineCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCreationFeedbackCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Output pipeline creation feedback.
  pub pPipelineCreationFeedback: *mut VkPipelineCreationFeedback,
  /// * Optional: true
  pub pipelineStageCreationFeedbackCount: u32,
  /// One entry for each shader stage specified in the parent Vk*PipelineCreateInfo struct
  /// * Len: `pipelineStageCreationFeedbackCount`
  pub pPipelineStageCreationFeedbacks: *mut VkPipelineCreationFeedback,
}
impl Default for VkPipelineCreationFeedbackCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO,
      pNext: core::ptr::null(),
      pPipelineCreationFeedback: core::ptr::null_mut(),
      pipelineStageCreationFeedbackCount: Default::default(),
      pPipelineStageCreationFeedbacks: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkPipelineDepthStencilStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineDepthStencilStateCreateFlags,
  pub depthTestEnable: VkBool32,
  pub depthWriteEnable: VkBool32,
  pub depthCompareOp: VkCompareOp,
  /// optional (depth_bounds_test)
  pub depthBoundsTestEnable: VkBool32,
  pub stencilTestEnable: VkBool32,
  pub front: VkStencilOpState,
  pub back: VkStencilOpState,
  pub minDepthBounds: c_float,
  pub maxDepthBounds: c_float,
}
impl Default for VkPipelineDepthStencilStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      depthTestEnable: Default::default(),
      depthWriteEnable: Default::default(),
      depthCompareOp: Default::default(),
      depthBoundsTestEnable: Default::default(),
      stencilTestEnable: Default::default(),
      front: Default::default(),
      back: Default::default(),
      minDepthBounds: Default::default(),
      maxDepthBounds: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineDiscardRectangleStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
  pub discardRectangleMode: VkDiscardRectangleModeEXT,
  /// * Optional: true
  pub discardRectangleCount: u32,
  /// * Len: `discardRectangleCount`
  /// * No Auto-Validity
  pub pDiscardRectangles: *const VkRect2D,
}
impl Default for VkPipelineDiscardRectangleStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      discardRectangleMode: Default::default(),
      discardRectangleCount: Default::default(),
      pDiscardRectangles: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineDynamicStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineDynamicStateCreateFlags,
  /// * Optional: true
  pub dynamicStateCount: u32,
  /// * Len: `dynamicStateCount`
  pub pDynamicStates: *const VkDynamicState,
}
impl Default for VkPipelineDynamicStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      dynamicStateCount: Default::default(),
      pDynamicStates: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineExecutableInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineExecutableInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub pipeline: VkPipeline,
  pub executableIndex: u32,
}
impl Default for VkPipelineExecutableInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR,
      pNext: core::ptr::null(),
      pipeline: Default::default(),
      executableIndex: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineExecutableInternalRepresentationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineExecutableInternalRepresentationKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub name: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub description: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub isText: VkBool32,
  pub dataSize: c_size_t,
  /// * Optional: true
  /// * Len: `dataSize`
  pub pData: *mut c_void,
}
impl Default for VkPipelineExecutableInternalRepresentationKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR,
      pNext: core::ptr::null_mut(),
      name: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      description: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      isText: Default::default(),
      dataSize: Default::default(),
      pData: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkPipelineExecutablePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutablePropertiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineExecutablePropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub stages: VkShaderStageFlags,
  pub name: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub description: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub subgroupSize: u32,
}
impl Default for VkPipelineExecutablePropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      stages: Default::default(),
      name: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      description: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      subgroupSize: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineExecutableStatisticKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineExecutableStatisticKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub name: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub description: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub format: VkPipelineExecutableStatisticFormatKHR,
  /// * Selector: format
  /// * No Auto-Validity
  pub value: VkPipelineExecutableStatisticValueKHR,
}
impl Default for VkPipelineExecutableStatisticKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR,
      pNext: core::ptr::null_mut(),
      name: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      description: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      format: Default::default(),
      value: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineFragmentShadingRateEnumStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * No Auto-Validity
  pub shadingRateType: VkFragmentShadingRateTypeNV,
  /// * No Auto-Validity
  pub shadingRate: VkFragmentShadingRateNV,
  /// * No Auto-Validity
  pub combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
}
impl Default for VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      shadingRateType: Default::default(),
      shadingRate: Default::default(),
      combinerOps: [Default::default(); 2],
    }
  }
}

/// Khronos: [VkPipelineFragmentShadingRateStateCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateStateCreateInfoKHR.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub fragmentSize: VkExtent2D,
  /// * No Auto-Validity
  pub combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
}
impl Default for VkPipelineFragmentShadingRateStateCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      fragmentSize: Default::default(),
      combinerOps: [Default::default(); 2],
    }
  }
}

/// Khronos: [VkPipelineInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub pipeline: VkPipeline,
}
impl Default for VkPipelineInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR,
      pNext: core::ptr::null(),
      pipeline: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineInputAssemblyStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineInputAssemblyStateCreateFlags,
  pub topology: VkPrimitiveTopology,
  pub primitiveRestartEnable: VkBool32,
}
impl Default for VkPipelineInputAssemblyStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      topology: Default::default(),
      primitiveRestartEnable: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineLayoutCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineLayoutCreateFlags,
  /// Number of descriptor sets interfaced by the pipeline
  /// * Optional: true
  pub setLayoutCount: u32,
  /// Array of setCount number of descriptor set layout objects defining the layout of the
  /// * Optional: false,true
  /// * Len: `setLayoutCount`
  pub pSetLayouts: *const VkDescriptorSetLayout,
  /// Number of push-constant ranges used by the pipeline
  /// * Optional: true
  pub pushConstantRangeCount: u32,
  /// Array of pushConstantRangeCount number of ranges used by various shader stages
  /// * Len: `pushConstantRangeCount`
  pub pPushConstantRanges: *const VkPushConstantRange,
}
impl Default for VkPipelineLayoutCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      setLayoutCount: Default::default(),
      pSetLayouts: core::ptr::null(),
      pushConstantRangeCount: Default::default(),
      pPushConstantRanges: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineLibraryCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineLibraryCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub libraryCount: u32,
  /// * Len: `libraryCount`
  pub pLibraries: *const VkPipeline,
}
impl Default for VkPipelineLibraryCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      libraryCount: Default::default(),
      pLibraries: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineMultisampleStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineMultisampleStateCreateFlags,
  /// Number of samples used for rasterization
  pub rasterizationSamples: VkSampleCountFlagBits,
  /// optional (GL45)
  pub sampleShadingEnable: VkBool32,
  /// optional (GL45)
  pub minSampleShading: c_float,
  /// Array of sampleMask words
  /// * Optional: true
  /// * Len: `(rasterizationSamples + 31) / 32`
  pub pSampleMask: *const VkSampleMask,
  pub alphaToCoverageEnable: VkBool32,
  pub alphaToOneEnable: VkBool32,
}
impl Default for VkPipelineMultisampleStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      rasterizationSamples: Default::default(),
      sampleShadingEnable: Default::default(),
      minSampleShading: Default::default(),
      pSampleMask: core::ptr::null(),
      alphaToCoverageEnable: Default::default(),
      alphaToOneEnable: Default::default(),
    }
  }
}

/// Khronos: [VkPipelinePropertiesIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelinePropertiesIdentifierEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelinePropertiesIdentifierEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_PROPERTIES_IDENTIFIER_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub pipelineIdentifier: [u8; VK_UUID_SIZE],
}
impl Default for VkPipelinePropertiesIdentifierEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_PROPERTIES_IDENTIFIER_EXT,
      pNext: core::ptr::null_mut(),
      pipelineIdentifier: [Default::default(); VK_UUID_SIZE],
    }
  }
}

/// Khronos: [VkPipelineRasterizationConservativeStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationConservativeStateCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Reserved
  /// * Optional: true
  pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
  /// Conservative rasterization mode
  pub conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
  /// Extra overestimation to add to the primitive
  pub extraPrimitiveOverestimationSize: c_float,
}
impl Default for VkPipelineRasterizationConservativeStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      conservativeRasterizationMode: Default::default(),
      extraPrimitiveOverestimationSize: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationDepthClipStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Reserved
  /// * Optional: true
  pub flags: VkPipelineRasterizationDepthClipStateCreateFlagsEXT,
  pub depthClipEnable: VkBool32,
}
impl Default for VkPipelineRasterizationDepthClipStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      depthClipEnable: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationLineStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationLineStateCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub lineRasterizationMode: VkLineRasterizationModeEXT,
  pub stippledLineEnable: VkBool32,
  pub lineStippleFactor: u32,
  pub lineStipplePattern: u16,
}
impl Default for VkPipelineRasterizationLineStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      lineRasterizationMode: Default::default(),
      stippledLineEnable: Default::default(),
      lineStippleFactor: Default::default(),
      lineStipplePattern: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationProvokingVertexStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub provokingVertexMode: VkProvokingVertexModeEXT,
}
impl Default for VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      provokingVertexMode: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineRasterizationStateCreateFlags,
  pub depthClampEnable: VkBool32,
  pub rasterizerDiscardEnable: VkBool32,
  /// optional (GL45)
  pub polygonMode: VkPolygonMode,
  /// * Optional: true
  pub cullMode: VkCullModeFlags,
  pub frontFace: VkFrontFace,
  pub depthBiasEnable: VkBool32,
  pub depthBiasConstantFactor: c_float,
  pub depthBiasClamp: c_float,
  pub depthBiasSlopeFactor: c_float,
  pub lineWidth: c_float,
}
impl Default for VkPipelineRasterizationStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      depthClampEnable: Default::default(),
      rasterizerDiscardEnable: Default::default(),
      polygonMode: Default::default(),
      cullMode: Default::default(),
      frontFace: Default::default(),
      depthBiasEnable: Default::default(),
      depthBiasConstantFactor: Default::default(),
      depthBiasClamp: Default::default(),
      depthBiasSlopeFactor: Default::default(),
      lineWidth: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationStateRasterizationOrderAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Rasterization order to use for the pipeline
  pub rasterizationOrder: VkRasterizationOrderAMD,
}
impl Default for VkPipelineRasterizationStateRasterizationOrderAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
      pNext: core::ptr::null(),
      rasterizationOrder: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRasterizationStateStreamCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineRasterizationStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationStateStreamCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineRasterizationStateStreamCreateFlagsEXT,
  pub rasterizationStream: u32,
}
impl Default for VkPipelineRasterizationStateStreamCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      rasterizationStream: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRenderingCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRenderingCreateInfo.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRenderingCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub viewMask: u32,
  /// * Optional: true
  pub colorAttachmentCount: u32,
  /// * Len: `colorAttachmentCount`
  /// * No Auto-Validity
  pub pColorAttachmentFormats: *const VkFormat,
  /// * No Auto-Validity
  pub depthAttachmentFormat: VkFormat,
  /// * No Auto-Validity
  pub stencilAttachmentFormat: VkFormat,
}
impl Default for VkPipelineRenderingCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO,
      pNext: core::ptr::null(),
      viewMask: Default::default(),
      colorAttachmentCount: Default::default(),
      pColorAttachmentFormats: core::ptr::null(),
      depthAttachmentFormat: Default::default(),
      stencilAttachmentFormat: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRepresentativeFragmentTestStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub representativeFragmentTestEnable: VkBool32,
}
impl Default for VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      representativeFragmentTestEnable: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineRobustnessCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRobustnessCreateInfoEXT.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
/// * Struct Extends: [`VkComputePipelineCreateInfo`]
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
/// * Struct Extends: [`VkRayTracingPipelineCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRobustnessCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *const c_void,
  pub storageBuffers: VkPipelineRobustnessBufferBehaviorEXT,
  pub uniformBuffers: VkPipelineRobustnessBufferBehaviorEXT,
  pub vertexInputs: VkPipelineRobustnessBufferBehaviorEXT,
  pub images: VkPipelineRobustnessImageBehaviorEXT,
}
impl Default for VkPipelineRobustnessCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      storageBuffers: Default::default(),
      uniformBuffers: Default::default(),
      vertexInputs: Default::default(),
      images: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineSampleLocationsStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineMultisampleStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub sampleLocationsEnable: VkBool32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
impl Default for VkPipelineSampleLocationsStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      sampleLocationsEnable: Default::default(),
      sampleLocationsInfo: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineShaderStageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineShaderStageCreateFlags,
  /// Shader stage
  pub stage: VkShaderStageFlagBits,
  /// Module containing entry point
  /// * Optional: true
  pub module: VkShaderModule,
  /// Null-terminated entry point name
  /// * Len: `null-terminated`
  pub pName: *const u8,
  /// * Optional: true
  pub pSpecializationInfo: *const VkSpecializationInfo,
}
impl Default for VkPipelineShaderStageCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      stage: Default::default(),
      module: Default::default(),
      pName: core::ptr::null(),
      pSpecializationInfo: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineShaderStageModuleIdentifierCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageModuleIdentifierCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineShaderStageModuleIdentifierCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub identifierSize: u32,
  /// * Len: `identifierSize`
  pub pIdentifier: *const u8,
}
impl Default for VkPipelineShaderStageModuleIdentifierCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      identifierSize: Default::default(),
      pIdentifier: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineShaderStageRequiredSubgroupSizeCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfo.html)
///
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub requiredSubgroupSize: u32,
}
impl Default for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO,
      pNext: core::ptr::null_mut(),
      requiredSubgroupSize: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineTessellationDomainOriginStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html)
///
/// * Struct Extends: [`VkPipelineTessellationStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub domainOrigin: VkTessellationDomainOrigin,
}
impl Default for VkPipelineTessellationDomainOriginStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      domainOrigin: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineTessellationStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineTessellationStateCreateFlags,
  pub patchControlPoints: u32,
}
impl Default for VkPipelineTessellationStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      patchControlPoints: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineVertexInputDivisorStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputDivisorStateCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineVertexInputStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub vertexBindingDivisorCount: u32,
  /// * Len: `vertexBindingDivisorCount`
  pub pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescriptionEXT,
}
impl Default for VkPipelineVertexInputDivisorStateCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      vertexBindingDivisorCount: Default::default(),
      pVertexBindingDivisors: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineVertexInputStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineVertexInputStateCreateFlags,
  /// number of bindings
  /// * Optional: true
  pub vertexBindingDescriptionCount: u32,
  /// * Len: `vertexBindingDescriptionCount`
  pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
  /// number of attributes
  /// * Optional: true
  pub vertexAttributeDescriptionCount: u32,
  /// * Len: `vertexAttributeDescriptionCount`
  pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}
impl Default for VkPipelineVertexInputStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      vertexBindingDescriptionCount: Default::default(),
      pVertexBindingDescriptions: core::ptr::null(),
      vertexAttributeDescriptionCount: Default::default(),
      pVertexAttributeDescriptions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportCoarseSampleOrderStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub sampleOrderType: VkCoarseSampleOrderTypeNV,
  /// * Optional: true
  pub customSampleOrderCount: u32,
  /// * Len: `customSampleOrderCount`
  pub pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
}
impl Default for VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      sampleOrderType: Default::default(),
      customSampleOrderCount: Default::default(),
      pCustomSampleOrders: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportDepthClipControlCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportDepthClipControlCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub negativeOneToOne: VkBool32,
}
impl Default for VkPipelineViewportDepthClipControlCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      negativeOneToOne: Default::default(),
    }
  }
}

/// Khronos: [VkPipelineViewportExclusiveScissorStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub exclusiveScissorCount: u32,
  /// * Len: `exclusiveScissorCount`
  /// * No Auto-Validity
  pub pExclusiveScissors: *const VkRect2D,
}
impl Default for VkPipelineViewportExclusiveScissorStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      exclusiveScissorCount: Default::default(),
      pExclusiveScissors: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportShadingRateImageStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub shadingRateImageEnable: VkBool32,
  /// * Optional: true
  pub viewportCount: u32,
  /// * Len: `viewportCount`
  /// * No Auto-Validity
  pub pShadingRatePalettes: *const VkShadingRatePaletteNV,
}
impl Default for VkPipelineViewportShadingRateImageStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      shadingRateImageEnable: Default::default(),
      viewportCount: Default::default(),
      pShadingRatePalettes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineViewportStateCreateFlags,
  /// * Optional: true
  pub viewportCount: u32,
  /// * Optional: true
  /// * Len: `viewportCount`
  /// * No Auto-Validity
  pub pViewports: *const VkViewport,
  /// * Optional: true
  pub scissorCount: u32,
  /// * Optional: true
  /// * Len: `scissorCount`
  /// * No Auto-Validity
  pub pScissors: *const VkRect2D,
}
impl Default for VkPipelineViewportStateCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      viewportCount: Default::default(),
      pViewports: core::ptr::null(),
      scissorCount: Default::default(),
      pScissors: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportSwizzleStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
  pub viewportCount: u32,
  /// * Len: `viewportCount`
  pub pViewportSwizzles: *const VkViewportSwizzleNV,
}
impl Default for VkPipelineViewportSwizzleStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      flags: Default::default(),
      viewportCount: Default::default(),
      pViewportSwizzles: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPipelineViewportWScalingStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html)
///
/// * Struct Extends: [`VkPipelineViewportStateCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub viewportWScalingEnable: VkBool32,
  pub viewportCount: u32,
  /// * Optional: true
  /// * Len: `viewportCount`
  /// * No Auto-Validity
  pub pViewportWScalings: *const VkViewportWScalingNV,
}
impl Default for VkPipelineViewportWScalingStateCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      viewportWScalingEnable: Default::default(),
      viewportCount: Default::default(),
      pViewportWScalings: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPresentIdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentIdKHR.html)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentIdKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRESENT_ID_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchainCount: u32,
  /// Present ID values for each swapchain
  /// * Optional: true
  /// * Len: `swapchainCount`
  pub pPresentIds: *const u64,
}
impl Default for VkPresentIdKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PRESENT_ID_KHR,
      pNext: core::ptr::null(),
      swapchainCount: Default::default(),
      pPresentIds: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPresentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Number of semaphores to wait for before presenting
  /// * Optional: true
  pub waitSemaphoreCount: u32,
  /// Semaphores to wait for before presenting
  /// * Len: `waitSemaphoreCount`
  pub pWaitSemaphores: *const VkSemaphore,
  /// Number of swapchains to present in this call
  pub swapchainCount: u32,
  /// Swapchains to present an image from
  /// * Len: `swapchainCount`
  pub pSwapchains: *const VkSwapchainKHR,
  /// Indices of which presentable images to present
  /// * Len: `swapchainCount`
  pub pImageIndices: *const u32,
  /// Optional (i.e. if non-NULL) VkResult for each swapchain
  /// * Optional: true
  /// * Len: `swapchainCount`
  pub pResults: *mut VkResult,
}
impl Default for VkPresentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
      pNext: core::ptr::null(),
      waitSemaphoreCount: Default::default(),
      pWaitSemaphores: core::ptr::null(),
      swapchainCount: Default::default(),
      pSwapchains: core::ptr::null(),
      pImageIndices: core::ptr::null(),
      pResults: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkPresentRegionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentRegionKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentRegionKHR {
  /// Number of rectangles in pRectangles
  /// * Optional: true
  pub rectangleCount: u32,
  /// Array of rectangles that have changed in a swapchain's image(s)
  /// * Optional: true
  /// * Len: `rectangleCount`
  pub pRectangles: *const VkRectLayerKHR,
}
impl Default for VkPresentRegionKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      rectangleCount: Default::default(),
      pRectangles: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPresentRegionsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentRegionsKHR.html)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentRegionsKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchainCount: u32,
  /// The regions that have changed
  /// * Optional: true
  /// * Len: `swapchainCount`
  pub pRegions: *const VkPresentRegionKHR,
}
impl Default for VkPresentRegionsKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR,
      pNext: core::ptr::null(),
      swapchainCount: Default::default(),
      pRegions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPresentTimeGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentTimeGOOGLE.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentTimeGOOGLE {
  /// Application-provided identifier
  pub presentID: u32,
  /// Earliest time an image should be presented
  pub desiredPresentTime: u64,
}
impl Default for VkPresentTimeGOOGLE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      presentID: Default::default(),
      desiredPresentTime: Default::default(),
    }
  }
}

/// Khronos: [VkPresentTimesInfoGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentTimesInfoGOOGLE.html)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentTimesInfoGOOGLE {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchainCount: u32,
  /// The earliest times to present images
  /// * Optional: true
  /// * Len: `swapchainCount`
  pub pTimes: *const VkPresentTimeGOOGLE,
}
impl Default for VkPresentTimesInfoGOOGLE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE,
      pNext: core::ptr::null(),
      swapchainCount: Default::default(),
      pTimes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkPrivateDataSlotCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPrivateDataSlotCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub flags: VkPrivateDataSlotCreateFlags,
}
impl Default for VkPrivateDataSlotCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkProtectedSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkProtectedSubmitInfo.html)
///
/// * Struct Extends: [`VkSubmitInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkProtectedSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Submit protected command buffers
  pub protectedSubmit: VkBool32,
}
impl Default for VkProtectedSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO,
      pNext: core::ptr::null(),
      protectedSubmit: Default::default(),
    }
  }
}

/// Khronos: [VkPushConstantRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPushConstantRange.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPushConstantRange {
  /// Which stages use the range
  pub stageFlags: VkShaderStageFlags,
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
      stageFlags: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkQueryLowLatencySupportNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryLowLatencySupportNV.html)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueryLowLatencySupportNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUERY_LOW_LATENCY_SUPPORT_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *const c_void,
  pub pQueriedLowLatencyData: *mut c_void,
}
impl Default for VkQueryLowLatencySupportNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUERY_LOW_LATENCY_SUPPORT_NV,
      pNext: core::ptr::null(),
      pQueriedLowLatencyData: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkQueryPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueryPoolCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkQueryPoolCreateFlags,
  pub queryType: VkQueryType,
  pub queryCount: u32,
  /// Optional
  /// * Optional: true
  /// * No Auto-Validity
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}
impl Default for VkQueryPoolCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      queryType: Default::default(),
      queryCount: Default::default(),
      pipelineStatistics: Default::default(),
    }
  }
}

/// Khronos: [VkQueryPoolPerformanceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html)
///
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueryPoolPerformanceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub queueFamilyIndex: u32,
  pub counterIndexCount: u32,
  /// * Len: `counterIndexCount`
  pub pCounterIndices: *const u32,
}
impl Default for VkQueryPoolPerformanceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      queueFamilyIndex: Default::default(),
      counterIndexCount: Default::default(),
      pCounterIndices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkQueryPoolPerformanceQueryCreateInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html)
///
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub performanceCountersSampling: VkQueryPoolSamplingModeINTEL,
}
impl Default for VkQueryPoolPerformanceQueryCreateInfoINTEL {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL,
      pNext: core::ptr::null(),
      performanceCountersSampling: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyCheckpointProperties2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointProperties2NV.html)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyCheckpointProperties2NV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub checkpointExecutionStageMask: VkPipelineStageFlags2,
}
impl Default for VkQueueFamilyCheckpointProperties2NV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV,
      pNext: core::ptr::null_mut(),
      checkpointExecutionStageMask: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyCheckpointPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyCheckpointPropertiesNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub checkpointExecutionStageMask: VkPipelineStageFlags,
}
impl Default for VkQueueFamilyCheckpointPropertiesNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV,
      pNext: core::ptr::null_mut(),
      checkpointExecutionStageMask: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyGlobalPriorityPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesKHR.html)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyGlobalPriorityPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: max
  pub priorityCount: u32,
  /// * Limit Type: bitmask
  pub priorities: [VkQueueGlobalPriorityKHR; VK_MAX_GLOBAL_PRIORITY_SIZE_KHR],
}
impl Default for VkQueueFamilyGlobalPriorityPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      priorityCount: Default::default(),
      priorities: [Default::default(); VK_MAX_GLOBAL_PRIORITY_SIZE_KHR],
    }
  }
}

/// Khronos: [VkQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyProperties {
  /// Queue flags
  /// * Optional: true
  /// * Limit Type: bitmask
  pub queueFlags: VkQueueFlags,
  /// * Limit Type: max
  pub queueCount: u32,
  /// * Limit Type: bits
  pub timestampValidBits: u32,
  /// Minimum alignment requirement for image transfers
  /// * Limit Type: min,mul
  pub minImageTransferGranularity: VkExtent3D,
}
impl Default for VkQueueFamilyProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      queueFlags: Default::default(),
      queueCount: Default::default(),
      timestampValidBits: Default::default(),
      minImageTransferGranularity: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties2.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: struct
  pub queueFamilyProperties: VkQueueFamilyProperties,
}
impl Default for VkQueueFamilyProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2,
      pNext: core::ptr::null_mut(),
      queueFamilyProperties: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyQueryResultStatusPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyQueryResultStatusPropertiesKHR.html)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyQueryResultStatusPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub queryResultStatusSupport: VkBool32,
}
impl Default for VkQueueFamilyQueryResultStatusPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      queryResultStatusSupport: Default::default(),
    }
  }
}

/// Khronos: [VkQueueFamilyVideoPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyVideoPropertiesKHR.html)
///
/// * Struct Extends: [`VkQueueFamilyProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyVideoPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub videoCodecOperations: VkVideoCodecOperationFlagsKHR,
}
impl Default for VkQueueFamilyVideoPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      videoCodecOperations: Default::default(),
    }
  }
}

/// Khronos: [VkRayTracingPipelineCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Pipeline creation flags
  /// * Optional: true
  pub flags: VkPipelineCreateFlags,
  /// * Optional: true
  pub stageCount: u32,
  /// One entry for each active shader stage
  /// * Len: `stageCount`
  pub pStages: *const VkPipelineShaderStageCreateInfo,
  /// * Optional: true
  pub groupCount: u32,
  /// * Len: `groupCount`
  pub pGroups: *const VkRayTracingShaderGroupCreateInfoKHR,
  pub maxPipelineRayRecursionDepth: u32,
  /// * Optional: true
  pub pLibraryInfo: *const VkPipelineLibraryCreateInfoKHR,
  /// * Optional: true
  pub pLibraryInterface: *const VkRayTracingPipelineInterfaceCreateInfoKHR,
  /// * Optional: true
  pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
  /// Interface layout of the pipeline
  pub layout: VkPipelineLayout,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it specifies the handle of the base pipeline this is a derivative of
  /// * Optional: true
  /// * No Auto-Validity
  pub basePipelineHandle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it specifies an index into pCreateInfos of the base pipeline this is a derivative of
  pub basePipelineIndex: i32,
}
impl Default for VkRayTracingPipelineCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      stageCount: Default::default(),
      pStages: core::ptr::null(),
      groupCount: Default::default(),
      pGroups: core::ptr::null(),
      maxPipelineRayRecursionDepth: Default::default(),
      pLibraryInfo: core::ptr::null(),
      pLibraryInterface: core::ptr::null(),
      pDynamicState: core::ptr::null(),
      layout: Default::default(),
      basePipelineHandle: Default::default(),
      basePipelineIndex: Default::default(),
    }
  }
}

/// Khronos: [VkRayTracingPipelineCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Pipeline creation flags
  /// * Optional: true
  pub flags: VkPipelineCreateFlags,
  pub stageCount: u32,
  /// One entry for each active shader stage
  /// * Len: `stageCount`
  pub pStages: *const VkPipelineShaderStageCreateInfo,
  pub groupCount: u32,
  /// * Len: `groupCount`
  pub pGroups: *const VkRayTracingShaderGroupCreateInfoNV,
  pub maxRecursionDepth: u32,
  /// Interface layout of the pipeline
  pub layout: VkPipelineLayout,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it specifies the handle of the base pipeline this is a derivative of
  /// * Optional: true
  /// * No Auto-Validity
  pub basePipelineHandle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it specifies an index into pCreateInfos of the base pipeline this is a derivative of
  pub basePipelineIndex: i32,
}
impl Default for VkRayTracingPipelineCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      flags: Default::default(),
      stageCount: Default::default(),
      pStages: core::ptr::null(),
      groupCount: Default::default(),
      pGroups: core::ptr::null(),
      maxRecursionDepth: Default::default(),
      layout: Default::default(),
      basePipelineHandle: Default::default(),
      basePipelineIndex: Default::default(),
    }
  }
}

/// Khronos: [VkRayTracingPipelineInterfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub maxPipelineRayPayloadSize: u32,
  pub maxPipelineRayHitAttributeSize: u32,
}
impl Default for VkRayTracingPipelineInterfaceCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      maxPipelineRayPayloadSize: Default::default(),
      maxPipelineRayHitAttributeSize: Default::default(),
    }
  }
}

/// Khronos: [VkRayTracingShaderGroupCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub ty: VkRayTracingShaderGroupTypeKHR,
  pub generalShader: u32,
  pub closestHitShader: u32,
  pub anyHitShader: u32,
  pub intersectionShader: u32,
  /// * Optional: true
  pub pShaderGroupCaptureReplayHandle: *const c_void,
}
impl Default for VkRayTracingShaderGroupCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      ty: Default::default(),
      generalShader: Default::default(),
      closestHitShader: Default::default(),
      anyHitShader: Default::default(),
      intersectionShader: Default::default(),
      pShaderGroupCaptureReplayHandle: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRayTracingShaderGroupCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub ty: VkRayTracingShaderGroupTypeKHR,
  pub generalShader: u32,
  pub closestHitShader: u32,
  pub anyHitShader: u32,
  pub intersectionShader: u32,
}
impl Default for VkRayTracingShaderGroupCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV,
      pNext: core::ptr::null(),
      ty: Default::default(),
      generalShader: Default::default(),
      closestHitShader: Default::default(),
      anyHitShader: Default::default(),
      intersectionShader: Default::default(),
    }
  }
}

/// Khronos: [VkRect2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRect2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
}
impl Default for VkRect2D {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      offset: Default::default(),
      extent: Default::default(),
    }
  }
}

/// Khronos: [VkRectLayerKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRectLayerKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRectLayerKHR {
  /// upper-left corner of a rectangle that has not changed, in pixels of a presentation images
  pub offset: VkOffset2D,
  /// Dimensions of a rectangle that has not changed, in pixels of a presentation images
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

/// Khronos: [VkRefreshCycleDurationGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshCycleDurationGOOGLE.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRefreshCycleDurationGOOGLE {
  /// Number of nanoseconds from the start of one refresh cycle to the next
  pub refreshDuration: u64,
}
impl Default for VkRefreshCycleDurationGOOGLE {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      refreshDuration: Default::default(),
    }
  }
}

/// Khronos: [VkRefreshObjectKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshObjectKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRefreshObjectKHR {
  pub objectType: VkObjectType,
  /// * Object Type: objectType
  /// * Extern Sync: true
  pub objectHandle: u64,
  /// * Optional: true
  pub flags: VkRefreshObjectFlagsKHR,
}
impl Default for VkRefreshObjectKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      objectType: Default::default(),
      objectHandle: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkRefreshObjectListKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshObjectListKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRefreshObjectListKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_REFRESH_OBJECT_LIST_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub objectCount: u32,
  /// * Len: `objectCount`
  pub pObjects: *const VkRefreshObjectKHR,
}
impl Default for VkRefreshObjectListKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_REFRESH_OBJECT_LIST_KHR,
      pNext: core::ptr::null(),
      objectCount: Default::default(),
      pObjects: core::ptr::null(),
    }
  }
}

/// Khronos: [VkReleaseSwapchainImagesInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkReleaseSwapchainImagesInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkReleaseSwapchainImagesInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RELEASE_SWAPCHAIN_IMAGES_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Swapchain for which images are being released
  /// * Extern Sync: true
  pub swapchain: VkSwapchainKHR,
  /// Number of indices to release
  pub imageIndexCount: u32,
  /// Indices of which presentable images to release
  /// * Len: `imageIndexCount`
  pub pImageIndices: *const u32,
}
impl Default for VkReleaseSwapchainImagesInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RELEASE_SWAPCHAIN_IMAGES_INFO_EXT,
      pNext: core::ptr::null(),
      swapchain: Default::default(),
      imageIndexCount: Default::default(),
      pImageIndices: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassAttachmentBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassAttachmentBeginInfo.html)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassAttachmentBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub attachmentCount: u32,
  /// * Len: `attachmentCount`
  pub pAttachments: *const VkImageView,
}
impl Default for VkRenderPassAttachmentBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO,
      pNext: core::ptr::null(),
      attachmentCount: Default::default(),
      pAttachments: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassBeginInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub renderPass: VkRenderPass,
  pub framebuffer: VkFramebuffer,
  pub renderArea: VkRect2D,
  /// * Optional: true
  pub clearValueCount: u32,
  /// * Len: `clearValueCount`
  /// * No Auto-Validity
  pub pClearValues: *const VkClearValue,
}
impl Default for VkRenderPassBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
      pNext: core::ptr::null(),
      renderPass: Default::default(),
      framebuffer: Default::default(),
      renderArea: Default::default(),
      clearValueCount: Default::default(),
      pClearValues: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkRenderPassCreateFlags,
  /// * Optional: true
  pub attachmentCount: u32,
  /// * Len: `attachmentCount`
  pub pAttachments: *const VkAttachmentDescription,
  pub subpassCount: u32,
  /// * Len: `subpassCount`
  pub pSubpasses: *const VkSubpassDescription,
  /// * Optional: true
  pub dependencyCount: u32,
  /// * Len: `dependencyCount`
  pub pDependencies: *const VkSubpassDependency,
}
impl Default for VkRenderPassCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      attachmentCount: Default::default(),
      pAttachments: core::ptr::null(),
      subpassCount: Default::default(),
      pSubpasses: core::ptr::null(),
      dependencyCount: Default::default(),
      pDependencies: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassCreateInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreateInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkRenderPassCreateFlags,
  /// * Optional: true
  pub attachmentCount: u32,
  /// * Len: `attachmentCount`
  pub pAttachments: *const VkAttachmentDescription2,
  pub subpassCount: u32,
  /// * Len: `subpassCount`
  pub pSubpasses: *const VkSubpassDescription2,
  /// * Optional: true
  pub dependencyCount: u32,
  /// * Len: `dependencyCount`
  pub pDependencies: *const VkSubpassDependency2,
  /// * Optional: true
  pub correlatedViewMaskCount: u32,
  /// * Len: `correlatedViewMaskCount`
  pub pCorrelatedViewMasks: *const u32,
}
impl Default for VkRenderPassCreateInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2,
      pNext: core::ptr::null(),
      flags: Default::default(),
      attachmentCount: Default::default(),
      pAttachments: core::ptr::null(),
      subpassCount: Default::default(),
      pSubpasses: core::ptr::null(),
      dependencyCount: Default::default(),
      pDependencies: core::ptr::null(),
      correlatedViewMaskCount: Default::default(),
      pCorrelatedViewMasks: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassCreationControlEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationControlEXT.html)
///
/// * Struct Extends: [`VkRenderPassCreateInfo2`]
/// * Struct Extends: [`VkSubpassDescription2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreationControlEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_CONTROL_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub disallowMerging: VkBool32,
}
impl Default for VkRenderPassCreationControlEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_CONTROL_EXT,
      pNext: core::ptr::null(),
      disallowMerging: Default::default(),
    }
  }
}

/// Khronos: [VkRenderPassCreationFeedbackCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationFeedbackCreateInfoEXT.html)
///
/// * Struct Extends: [`VkRenderPassCreateInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreationFeedbackCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub pRenderPassFeedback: *mut VkRenderPassCreationFeedbackInfoEXT,
}
impl Default for VkRenderPassCreationFeedbackCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      pRenderPassFeedback: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkRenderPassCreationFeedbackInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationFeedbackInfoEXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreationFeedbackInfoEXT {
  pub postMergeSubpassCount: u32,
}
impl Default for VkRenderPassCreationFeedbackInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      postMergeSubpassCount: Default::default(),
    }
  }
}

/// Khronos: [VkRenderPassFragmentDensityMapCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html)
///
/// * Struct Extends: [`VkRenderPassCreateInfo`]
/// * Struct Extends: [`VkRenderPassCreateInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassFragmentDensityMapCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub fragmentDensityMapAttachment: VkAttachmentReference,
}
impl Default for VkRenderPassFragmentDensityMapCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      fragmentDensityMapAttachment: Default::default(),
    }
  }
}

/// Khronos: [VkRenderPassInputAttachmentAspectCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html)
///
/// * Struct Extends: [`VkRenderPassCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub aspectReferenceCount: u32,
  /// * Len: `aspectReferenceCount`
  pub pAspectReferences: *const VkInputAttachmentAspectReference,
}
impl Default for VkRenderPassInputAttachmentAspectCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO,
      pNext: core::ptr::null(),
      aspectReferenceCount: Default::default(),
      pAspectReferences: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassMultiviewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassMultiviewCreateInfo.html)
///
/// * Struct Extends: [`VkRenderPassCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassMultiviewCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub subpassCount: u32,
  /// * Len: `subpassCount`
  pub pViewMasks: *const u32,
  /// * Optional: true
  pub dependencyCount: u32,
  /// * Len: `dependencyCount`
  pub pViewOffsets: *const i32,
  /// * Optional: true
  pub correlationMaskCount: u32,
  /// * Len: `correlationMaskCount`
  pub pCorrelationMasks: *const u32,
}
impl Default for VkRenderPassMultiviewCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO,
      pNext: core::ptr::null(),
      subpassCount: Default::default(),
      pViewMasks: core::ptr::null(),
      dependencyCount: Default::default(),
      pViewOffsets: core::ptr::null(),
      correlationMaskCount: Default::default(),
      pCorrelationMasks: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassSampleLocationsBeginInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub attachmentInitialSampleLocationsCount: u32,
  /// * Len: `attachmentInitialSampleLocationsCount`
  pub pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT,
  /// * Optional: true
  pub postSubpassSampleLocationsCount: u32,
  /// * Len: `postSubpassSampleLocationsCount`
  pub pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT,
}
impl Default for VkRenderPassSampleLocationsBeginInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
      pNext: core::ptr::null(),
      attachmentInitialSampleLocationsCount: Default::default(),
      pAttachmentInitialSampleLocations: core::ptr::null(),
      postSubpassSampleLocationsCount: Default::default(),
      pPostSubpassSampleLocations: core::ptr::null(),
    }
  }
}

/// Khronos: [VkRenderPassSubpassFeedbackCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassSubpassFeedbackCreateInfoEXT.html)
///
/// * Struct Extends: [`VkSubpassDescription2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassSubpassFeedbackCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub pSubpassFeedback: *mut VkRenderPassSubpassFeedbackInfoEXT,
}
impl Default for VkRenderPassSubpassFeedbackCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      pSubpassFeedback: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkRenderPassSubpassFeedbackInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassSubpassFeedbackInfoEXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassSubpassFeedbackInfoEXT {
  pub subpassMergeStatus: VkSubpassMergeStatusEXT,
  pub description: [u8; VK_MAX_DESCRIPTION_SIZE],
  pub postMergeIndex: u32,
}
impl Default for VkRenderPassSubpassFeedbackInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      subpassMergeStatus: Default::default(),
      description: [Default::default(); VK_MAX_DESCRIPTION_SIZE],
      postMergeIndex: Default::default(),
    }
  }
}

/// Khronos: [VkRenderPassTransformBeginInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassTransformBeginInfoQCOM.html)
///
/// * Struct Extends: [`VkRenderPassBeginInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassTransformBeginInfoQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM`
  pub sType: VkStructureType,
  /// Pointer to next structure
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * No Auto-Validity
  pub transform: VkSurfaceTransformFlagBitsKHR,
}
impl Default for VkRenderPassTransformBeginInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM,
      pNext: core::ptr::null_mut(),
      transform: Default::default(),
    }
  }
}

/// Khronos: [VkRenderingAttachmentInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingAttachmentInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingAttachmentInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub imageView: VkImageView,
  pub imageLayout: VkImageLayout,
  /// * Optional: true
  pub resolveMode: VkResolveModeFlagBits,
  /// * Optional: true
  pub resolveImageView: VkImageView,
  pub resolveImageLayout: VkImageLayout,
  pub loadOp: VkAttachmentLoadOp,
  pub storeOp: VkAttachmentStoreOp,
  pub clearValue: VkClearValue,
}
impl Default for VkRenderingAttachmentInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
      pNext: core::ptr::null(),
      imageView: Default::default(),
      imageLayout: Default::default(),
      resolveMode: Default::default(),
      resolveImageView: Default::default(),
      resolveImageLayout: Default::default(),
      loadOp: Default::default(),
      storeOp: Default::default(),
      clearValue: Default::default(),
    }
  }
}

/// Khronos: [VkRenderingFragmentDensityMapAttachmentInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentDensityMapAttachmentInfoEXT.html)
///
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingFragmentDensityMapAttachmentInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub imageView: VkImageView,
  pub imageLayout: VkImageLayout,
}
impl Default for VkRenderingFragmentDensityMapAttachmentInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT,
      pNext: core::ptr::null(),
      imageView: Default::default(),
      imageLayout: Default::default(),
    }
  }
}

/// Khronos: [VkRenderingFragmentShadingRateAttachmentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentShadingRateAttachmentInfoKHR.html)
///
/// * Struct Extends: [`VkRenderingInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingFragmentShadingRateAttachmentInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub imageView: VkImageView,
  pub imageLayout: VkImageLayout,
  pub shadingRateAttachmentTexelSize: VkExtent2D,
}
impl Default for VkRenderingFragmentShadingRateAttachmentInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
      pNext: core::ptr::null(),
      imageView: Default::default(),
      imageLayout: Default::default(),
      shadingRateAttachmentTexelSize: Default::default(),
    }
  }
}

/// Khronos: [VkRenderingInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderingInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RENDERING_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkRenderingFlags,
  pub renderArea: VkRect2D,
  pub layerCount: u32,
  pub viewMask: u32,
  /// * Optional: true
  pub colorAttachmentCount: u32,
  /// * Len: `colorAttachmentCount`
  pub pColorAttachments: *const VkRenderingAttachmentInfo,
  /// * Optional: true
  pub pDepthAttachment: *const VkRenderingAttachmentInfo,
  /// * Optional: true
  pub pStencilAttachment: *const VkRenderingAttachmentInfo,
}
impl Default for VkRenderingInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RENDERING_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      renderArea: Default::default(),
      layerCount: Default::default(),
      viewMask: Default::default(),
      colorAttachmentCount: Default::default(),
      pColorAttachments: core::ptr::null(),
      pDepthAttachment: core::ptr::null(),
      pStencilAttachment: core::ptr::null(),
    }
  }
}

/// Khronos: [VkResolveImageInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResolveImageInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkResolveImageInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// * Len: `regionCount`
  pub pRegions: *const VkImageResolve2,
}
impl Default for VkResolveImageInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2,
      pNext: core::ptr::null(),
      srcImage: Default::default(),
      srcImageLayout: Default::default(),
      dstImage: Default::default(),
      dstImageLayout: Default::default(),
      regionCount: Default::default(),
      pRegions: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSRTDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSRTDataNV.html)
#[derive(Clone, Copy)]
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

/// Khronos: [VkSampleLocationEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleLocationEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSampleLocationEXT {
  pub x: c_float,
  pub y: c_float,
}
impl Default for VkSampleLocationEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      x: Default::default(),
      y: Default::default(),
    }
  }
}

/// Khronos: [VkSampleLocationsInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleLocationsInfoEXT.html)
///
/// * Struct Extends: [`VkImageMemoryBarrier`]
/// * Struct Extends: [`VkImageMemoryBarrier2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSampleLocationsInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * No Auto-Validity
  pub sampleLocationsPerPixel: VkSampleCountFlagBits,
  pub sampleLocationGridSize: VkExtent2D,
  /// * Optional: true
  pub sampleLocationsCount: u32,
  /// * Len: `sampleLocationsCount`
  pub pSampleLocations: *const VkSampleLocationEXT,
}
impl Default for VkSampleLocationsInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT,
      pNext: core::ptr::null(),
      sampleLocationsPerPixel: Default::default(),
      sampleLocationGridSize: Default::default(),
      sampleLocationsCount: Default::default(),
      pSampleLocations: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSamplerBorderColorComponentMappingCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html)
///
/// * Struct Extends: [`VkSamplerCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub components: VkComponentMapping,
  pub srgb: VkBool32,
}
impl Default for VkSamplerBorderColorComponentMappingCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      components: Default::default(),
      srgb: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerCaptureDescriptorDataInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCaptureDescriptorDataInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerCaptureDescriptorDataInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub sampler: VkSampler,
}
impl Default for VkSamplerCaptureDescriptorDataInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
      pNext: core::ptr::null(),
      sampler: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkSamplerCreateFlags,
  /// Filter mode for magnification
  pub magFilter: VkFilter,
  /// Filter mode for minifiation
  pub minFilter: VkFilter,
  /// Mipmap selection mode
  pub mipmapMode: VkSamplerMipmapMode,
  pub addressModeU: VkSamplerAddressMode,
  pub addressModeV: VkSamplerAddressMode,
  pub addressModeW: VkSamplerAddressMode,
  pub mipLodBias: c_float,
  pub anisotropyEnable: VkBool32,
  pub maxAnisotropy: c_float,
  pub compareEnable: VkBool32,
  /// * No Auto-Validity
  pub compareOp: VkCompareOp,
  pub minLod: c_float,
  pub maxLod: c_float,
  /// * No Auto-Validity
  pub borderColor: VkBorderColor,
  pub unnormalizedCoordinates: VkBool32,
}
impl Default for VkSamplerCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      magFilter: Default::default(),
      minFilter: Default::default(),
      mipmapMode: Default::default(),
      addressModeU: Default::default(),
      addressModeV: Default::default(),
      addressModeW: Default::default(),
      mipLodBias: Default::default(),
      anisotropyEnable: Default::default(),
      maxAnisotropy: Default::default(),
      compareEnable: Default::default(),
      compareOp: Default::default(),
      minLod: Default::default(),
      maxLod: Default::default(),
      borderColor: Default::default(),
      unnormalizedCoordinates: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerCustomBorderColorCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html)
///
/// * Struct Extends: [`VkSamplerCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerCustomBorderColorCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * No Auto-Validity
  pub customBorderColor: VkClearColorValue,
  pub format: VkFormat,
}
impl Default for VkSamplerCustomBorderColorCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      customBorderColor: Default::default(),
      format: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerReductionModeCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionModeCreateInfo.html)
///
/// * Struct Extends: [`VkSamplerCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerReductionModeCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub reductionMode: VkSamplerReductionMode,
}
impl Default for VkSamplerReductionModeCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO,
      pNext: core::ptr::null(),
      reductionMode: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerYcbcrConversionCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerYcbcrConversionCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub format: VkFormat,
  pub ycbcrModel: VkSamplerYcbcrModelConversion,
  pub ycbcrRange: VkSamplerYcbcrRange,
  pub components: VkComponentMapping,
  pub xChromaOffset: VkChromaLocation,
  pub yChromaOffset: VkChromaLocation,
  pub chromaFilter: VkFilter,
  pub forceExplicitReconstruction: VkBool32,
}
impl Default for VkSamplerYcbcrConversionCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO,
      pNext: core::ptr::null(),
      format: Default::default(),
      ycbcrModel: Default::default(),
      ycbcrRange: Default::default(),
      components: Default::default(),
      xChromaOffset: Default::default(),
      yChromaOffset: Default::default(),
      chromaFilter: Default::default(),
      forceExplicitReconstruction: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerYcbcrConversionImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerYcbcrConversionImageFormatProperties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub combinedImageSamplerDescriptorCount: u32,
}
impl Default for VkSamplerYcbcrConversionImageFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES,
      pNext: core::ptr::null_mut(),
      combinedImageSamplerDescriptorCount: Default::default(),
    }
  }
}

/// Khronos: [VkSamplerYcbcrConversionInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionInfo.html)
///
/// * Struct Extends: [`VkSamplerCreateInfo`]
/// * Struct Extends: [`VkImageViewCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerYcbcrConversionInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub conversion: VkSamplerYcbcrConversion,
}
impl Default for VkSamplerYcbcrConversionInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO,
      pNext: core::ptr::null(),
      conversion: Default::default(),
    }
  }
}

/// Khronos: [VkSciSyncAttributesInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSciSyncAttributesInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSciSyncAttributesInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SCI_SYNC_ATTRIBUTES_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub clientType: VkSciSyncClientTypeNV,
  pub primitiveType: VkSciSyncPrimitiveTypeNV,
}
impl Default for VkSciSyncAttributesInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SCI_SYNC_ATTRIBUTES_INFO_NV,
      pNext: core::ptr::null(),
      clientType: Default::default(),
      primitiveType: Default::default(),
    }
  }
}

/// Khronos: [VkScreenSurfaceCreateInfoQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkScreenSurfaceCreateInfoQNX.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkScreenSurfaceCreateInfoQNX {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX,
      pNext: core::ptr::null(),
      flags: Default::default(),
      context: core::ptr::null_mut(),
      window: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkSemaphoreCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Semaphore creation flags
  /// * Optional: true
  pub flags: VkSemaphoreCreateFlags,
}
impl Default for VkSemaphoreCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreGetFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetFdInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreGetFdInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkSemaphoreGetFdInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreGetSciSyncInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetSciSyncInfoNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreGetSciSyncInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_GET_SCI_SYNC_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkSemaphoreGetSciSyncInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_GET_SCI_SYNC_INFO_NV,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreGetWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetWin32HandleInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkSemaphoreGetWin32HandleInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreGetZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetZirconHandleInfoFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl Default for VkSemaphoreGetZirconHandleInfoFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      handleType: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreSignalInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSignalInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreSignalInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub value: u64,
}
impl Default for VkSemaphoreSignalInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      value: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSubmitInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub value: u64,
  /// * Optional: true
  pub stageMask: VkPipelineStageFlags2,
  pub deviceIndex: u32,
}
impl Default for VkSemaphoreSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO,
      pNext: core::ptr::null(),
      semaphore: Default::default(),
      value: Default::default(),
      stageMask: Default::default(),
      deviceIndex: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreTypeCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeCreateInfo.html)
///
/// * Struct Extends: [`VkSemaphoreCreateInfo`]
/// * Struct Extends: [`VkPhysicalDeviceExternalSemaphoreInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreTypeCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub semaphoreType: VkSemaphoreType,
  pub initialValue: u64,
}
impl Default for VkSemaphoreTypeCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO,
      pNext: core::ptr::null(),
      semaphoreType: Default::default(),
      initialValue: Default::default(),
    }
  }
}

/// Khronos: [VkSemaphoreWaitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreWaitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkSemaphoreWaitFlags,
  pub semaphoreCount: u32,
  /// * Len: `semaphoreCount`
  pub pSemaphores: *const VkSemaphore,
  /// * Len: `semaphoreCount`
  pub pValues: *const u64,
}
impl Default for VkSemaphoreWaitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      semaphoreCount: Default::default(),
      pSemaphores: core::ptr::null(),
      pValues: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSetStateFlagsIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSetStateFlagsIndirectCommandNV {
  pub data: u32,
}
impl Default for VkSetStateFlagsIndirectCommandNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      data: Default::default(),
    }
  }
}

/// Khronos: [VkShaderModuleCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateInfo.html)
///
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderModuleCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO`
  pub sType: VkStructureType,
  /// noautovalidity because this structure can be either an explicit parameter, or passed in a pNext chain
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkShaderModuleCreateFlags,
  /// Specified in bytes
  pub codeSize: c_size_t,
  /// Binary code of size codeSize
  /// * Len: `codeSize / 4`
  pub pCode: *const u32,
}
impl Default for VkShaderModuleCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
      pNext: core::ptr::null(),
      flags: Default::default(),
      codeSize: Default::default(),
      pCode: core::ptr::null(),
    }
  }
}

/// Khronos: [VkShaderModuleIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleIdentifierEXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderModuleIdentifierEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SHADER_MODULE_IDENTIFIER_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * No Auto-Validity
  pub identifierSize: u32,
  pub identifier: [u8; VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT],
}
impl Default for VkShaderModuleIdentifierEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SHADER_MODULE_IDENTIFIER_EXT,
      pNext: core::ptr::null_mut(),
      identifierSize: Default::default(),
      identifier: [Default::default(); VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT],
    }
  }
}

/// Khronos: [VkShaderModuleValidationCacheCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html)
///
/// * Struct Extends: [`VkShaderModuleCreateInfo`]
/// * Struct Extends: [`VkPipelineShaderStageCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub validationCache: VkValidationCacheEXT,
}
impl Default for VkShaderModuleValidationCacheCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      validationCache: Default::default(),
    }
  }
}

/// Khronos: [VkShaderResourceUsageAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderResourceUsageAMD.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderResourceUsageAMD {
  pub numUsedVgprs: u32,
  pub numUsedSgprs: u32,
  pub ldsSizePerLocalWorkGroup: u32,
  pub ldsUsageSizeInBytes: c_size_t,
  pub scratchMemUsageInBytes: c_size_t,
}
impl Default for VkShaderResourceUsageAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      numUsedVgprs: Default::default(),
      numUsedSgprs: Default::default(),
      ldsSizePerLocalWorkGroup: Default::default(),
      ldsUsageSizeInBytes: Default::default(),
      scratchMemUsageInBytes: Default::default(),
    }
  }
}

/// Khronos: [VkShaderStatisticsInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStatisticsInfoAMD.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderStatisticsInfoAMD {
  pub shaderStageMask: VkShaderStageFlags,
  pub resourceUsage: VkShaderResourceUsageAMD,
  pub numPhysicalVgprs: u32,
  pub numPhysicalSgprs: u32,
  pub numAvailableVgprs: u32,
  pub numAvailableSgprs: u32,
  pub computeWorkGroupSize: [u32; 3],
}
impl Default for VkShaderStatisticsInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      shaderStageMask: Default::default(),
      resourceUsage: Default::default(),
      numPhysicalVgprs: Default::default(),
      numPhysicalSgprs: Default::default(),
      numAvailableVgprs: Default::default(),
      numAvailableSgprs: Default::default(),
      computeWorkGroupSize: [Default::default(); 3],
    }
  }
}

/// Khronos: [VkShadingRatePaletteNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShadingRatePaletteNV {
  pub shadingRatePaletteEntryCount: u32,
  /// * Len: `shadingRatePaletteEntryCount`
  pub pShadingRatePaletteEntries: *const VkShadingRatePaletteEntryNV,
}
impl Default for VkShadingRatePaletteNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      shadingRatePaletteEntryCount: Default::default(),
      pShadingRatePaletteEntries: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSharedPresentSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Supported image usage flags if swapchain created using a shared present mode
  /// * Optional: true
  pub sharedPresentSupportedUsageFlags: VkImageUsageFlags,
}
impl Default for VkSharedPresentSurfaceCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR,
      pNext: core::ptr::null_mut(),
      sharedPresentSupportedUsageFlags: Default::default(),
    }
  }
}

/// Khronos: [VkSparseBufferMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseBufferMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
  pub buffer: VkBuffer,
  pub bindCount: u32,
  /// * Len: `bindCount`
  pub pBinds: *const VkSparseMemoryBind,
}
impl Default for VkSparseBufferMemoryBindInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      buffer: Default::default(),
      bindCount: Default::default(),
      pBinds: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSparseImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageFormatProperties {
  /// * Optional: true
  /// * Limit Type: bitmask
  pub aspectMask: VkImageAspectFlags,
  /// * Limit Type: min,mul
  pub imageGranularity: VkExtent3D,
  /// * Optional: true
  /// * Limit Type: bitmask
  pub flags: VkSparseImageFormatFlags,
}
impl Default for VkSparseImageFormatProperties {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      aspectMask: Default::default(),
      imageGranularity: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties2.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageFormatProperties2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: struct
  pub properties: VkSparseImageFormatProperties,
}
impl Default for VkSparseImageFormatProperties2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2,
      pNext: core::ptr::null_mut(),
      properties: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageMemoryBind](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBind.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryBind {
  pub subresource: VkImageSubresource,
  pub offset: VkOffset3D,
  pub extent: VkExtent3D,
  /// * Optional: true
  pub memory: VkDeviceMemory,
  /// Specified in bytes
  pub memoryOffset: VkDeviceSize,
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
      memoryOffset: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
  pub image: VkImage,
  pub bindCount: u32,
  /// * Len: `bindCount`
  pub pBinds: *const VkSparseImageMemoryBind,
}
impl Default for VkSparseImageMemoryBindInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      image: Default::default(),
      bindCount: Default::default(),
      pBinds: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSparseImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryRequirements {
  pub formatProperties: VkSparseImageFormatProperties,
  pub imageMipTailFirstLod: u32,
  /// Specified in bytes, must be a multiple of sparse block size in bytes / alignment
  pub imageMipTailSize: VkDeviceSize,
  /// Specified in bytes, must be a multiple of sparse block size in bytes / alignment
  pub imageMipTailOffset: VkDeviceSize,
  /// Specified in bytes, must be a multiple of sparse block size in bytes / alignment
  pub imageMipTailStride: VkDeviceSize,
}
impl Default for VkSparseImageMemoryRequirements {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      formatProperties: Default::default(),
      imageMipTailFirstLod: Default::default(),
      imageMipTailSize: Default::default(),
      imageMipTailOffset: Default::default(),
      imageMipTailStride: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements2.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryRequirements2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryRequirements: VkSparseImageMemoryRequirements,
}
impl Default for VkSparseImageMemoryRequirements2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2,
      pNext: core::ptr::null_mut(),
      memoryRequirements: Default::default(),
    }
  }
}

/// Khronos: [VkSparseImageOpaqueMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
  pub image: VkImage,
  pub bindCount: u32,
  /// * Len: `bindCount`
  pub pBinds: *const VkSparseMemoryBind,
}
impl Default for VkSparseImageOpaqueMemoryBindInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      image: Default::default(),
      bindCount: Default::default(),
      pBinds: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSparseMemoryBind](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBind.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseMemoryBind {
  /// Specified in bytes
  pub resourceOffset: VkDeviceSize,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// * Optional: true
  pub memory: VkDeviceMemory,
  /// Specified in bytes
  pub memoryOffset: VkDeviceSize,
  /// * Optional: true
  pub flags: VkSparseMemoryBindFlags,
}
impl Default for VkSparseMemoryBind {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      resourceOffset: Default::default(),
      size: Default::default(),
      memory: Default::default(),
      memoryOffset: Default::default(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkSpecializationInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSpecializationInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSpecializationInfo {
  /// Number of entries in the map
  /// * Optional: true
  pub mapEntryCount: u32,
  /// Array of map entries
  /// * Len: `mapEntryCount`
  pub pMapEntries: *const VkSpecializationMapEntry,
  /// Size in bytes of pData
  /// * Optional: true
  pub dataSize: c_size_t,
  /// Pointer to SpecConstant data
  /// * Len: `dataSize`
  pub pData: *const c_void,
}
impl Default for VkSpecializationInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      mapEntryCount: Default::default(),
      pMapEntries: core::ptr::null(),
      dataSize: Default::default(),
      pData: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSpecializationMapEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSpecializationMapEntry.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSpecializationMapEntry {
  /// The SpecConstant ID specified in the BIL
  pub constantID: u32,
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
      constantID: Default::default(),
      offset: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkStencilOpState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilOpState.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkStencilOpState {
  pub failOp: VkStencilOp,
  pub passOp: VkStencilOp,
  pub depthFailOp: VkStencilOp,
  pub compareOp: VkCompareOp,
  pub compareMask: u32,
  pub writeMask: u32,
  pub reference: u32,
}
impl Default for VkStencilOpState {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      failOp: Default::default(),
      passOp: Default::default(),
      depthFailOp: Default::default(),
      compareOp: Default::default(),
      compareMask: Default::default(),
      writeMask: Default::default(),
      reference: Default::default(),
    }
  }
}

/// Khronos: [VkStridedDeviceAddressRegionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStridedDeviceAddressRegionKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkStridedDeviceAddressRegionKHR {
  /// * Optional: true
  pub deviceAddress: VkDeviceAddress,
  pub stride: VkDeviceSize,
  pub size: VkDeviceSize,
}
impl Default for VkStridedDeviceAddressRegionKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      deviceAddress: Default::default(),
      stride: Default::default(),
      size: Default::default(),
    }
  }
}

/// Khronos: [VkSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBMIT_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub waitSemaphoreCount: u32,
  /// * Len: `waitSemaphoreCount`
  pub pWaitSemaphores: *const VkSemaphore,
  /// * Optional: false,true
  /// * Len: `waitSemaphoreCount`
  pub pWaitDstStageMask: *const VkPipelineStageFlags,
  /// * Optional: true
  pub commandBufferCount: u32,
  /// * Len: `commandBufferCount`
  pub pCommandBuffers: *const VkCommandBuffer,
  /// * Optional: true
  pub signalSemaphoreCount: u32,
  /// * Len: `signalSemaphoreCount`
  pub pSignalSemaphores: *const VkSemaphore,
}
impl Default for VkSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
      pNext: core::ptr::null(),
      waitSemaphoreCount: Default::default(),
      pWaitSemaphores: core::ptr::null(),
      pWaitDstStageMask: core::ptr::null(),
      commandBufferCount: Default::default(),
      pCommandBuffers: core::ptr::null(),
      signalSemaphoreCount: Default::default(),
      pSignalSemaphores: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubmitInfo2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubmitInfo2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBMIT_INFO_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkSubmitFlags,
  /// * Optional: true
  pub waitSemaphoreInfoCount: u32,
  /// * Len: `waitSemaphoreInfoCount`
  pub pWaitSemaphoreInfos: *const VkSemaphoreSubmitInfo,
  /// * Optional: true
  pub commandBufferInfoCount: u32,
  /// * Len: `commandBufferInfoCount`
  pub pCommandBufferInfos: *const VkCommandBufferSubmitInfo,
  /// * Optional: true
  pub signalSemaphoreInfoCount: u32,
  /// * Len: `signalSemaphoreInfoCount`
  pub pSignalSemaphoreInfos: *const VkSemaphoreSubmitInfo,
}
impl Default for VkSubmitInfo2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBMIT_INFO_2,
      pNext: core::ptr::null(),
      flags: Default::default(),
      waitSemaphoreInfoCount: Default::default(),
      pWaitSemaphoreInfos: core::ptr::null(),
      commandBufferInfoCount: Default::default(),
      pCommandBufferInfos: core::ptr::null(),
      signalSemaphoreInfoCount: Default::default(),
      pSignalSemaphoreInfos: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassBeginInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassBeginInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub contents: VkSubpassContents,
}
impl Default for VkSubpassBeginInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO,
      pNext: core::ptr::null(),
      contents: Default::default(),
    }
  }
}

/// Khronos: [VkSubpassDependency](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDependency {
  pub srcSubpass: u32,
  pub dstSubpass: u32,
  /// * Optional: true
  pub srcStageMask: VkPipelineStageFlags,
  /// * Optional: true
  pub dstStageMask: VkPipelineStageFlags,
  /// Memory accesses from the source of the dependency to synchronize
  /// * Optional: true
  pub srcAccessMask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * Optional: true
  pub dstAccessMask: VkAccessFlags,
  /// * Optional: true
  pub dependencyFlags: VkDependencyFlags,
}
impl Default for VkSubpassDependency {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      srcSubpass: Default::default(),
      dstSubpass: Default::default(),
      srcStageMask: Default::default(),
      dstStageMask: Default::default(),
      srcAccessMask: Default::default(),
      dstAccessMask: Default::default(),
      dependencyFlags: Default::default(),
    }
  }
}

/// Khronos: [VkSubpassDependency2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDependency2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub srcSubpass: u32,
  pub dstSubpass: u32,
  /// * Optional: true
  pub srcStageMask: VkPipelineStageFlags,
  /// * Optional: true
  pub dstStageMask: VkPipelineStageFlags,
  /// * Optional: true
  pub srcAccessMask: VkAccessFlags,
  /// * Optional: true
  pub dstAccessMask: VkAccessFlags,
  /// * Optional: true
  pub dependencyFlags: VkDependencyFlags,
  pub viewOffset: i32,
}
impl Default for VkSubpassDependency2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2,
      pNext: core::ptr::null(),
      srcSubpass: Default::default(),
      dstSubpass: Default::default(),
      srcStageMask: Default::default(),
      dstStageMask: Default::default(),
      srcAccessMask: Default::default(),
      dstAccessMask: Default::default(),
      dependencyFlags: Default::default(),
      viewOffset: Default::default(),
    }
  }
}

/// Khronos: [VkSubpassDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDescription {
  /// * Optional: true
  pub flags: VkSubpassDescriptionFlags,
  /// Must be VK_PIPELINE_BIND_POINT_GRAPHICS for now
  pub pipelineBindPoint: VkPipelineBindPoint,
  /// * Optional: true
  pub inputAttachmentCount: u32,
  /// * Len: `inputAttachmentCount`
  pub pInputAttachments: *const VkAttachmentReference,
  /// * Optional: true
  pub colorAttachmentCount: u32,
  /// * Len: `colorAttachmentCount`
  pub pColorAttachments: *const VkAttachmentReference,
  /// * Optional: true
  /// * Len: `colorAttachmentCount`
  pub pResolveAttachments: *const VkAttachmentReference,
  /// * Optional: true
  pub pDepthStencilAttachment: *const VkAttachmentReference,
  /// * Optional: true
  pub preserveAttachmentCount: u32,
  /// * Len: `preserveAttachmentCount`
  pub pPreserveAttachments: *const u32,
}
impl Default for VkSubpassDescription {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      flags: Default::default(),
      pipelineBindPoint: Default::default(),
      inputAttachmentCount: Default::default(),
      pInputAttachments: core::ptr::null(),
      colorAttachmentCount: Default::default(),
      pColorAttachments: core::ptr::null(),
      pResolveAttachments: core::ptr::null(),
      pDepthStencilAttachment: core::ptr::null(),
      preserveAttachmentCount: Default::default(),
      pPreserveAttachments: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassDescription2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription2.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDescription2 {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkSubpassDescriptionFlags,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub viewMask: u32,
  /// * Optional: true
  pub inputAttachmentCount: u32,
  /// * Len: `inputAttachmentCount`
  pub pInputAttachments: *const VkAttachmentReference2,
  /// * Optional: true
  pub colorAttachmentCount: u32,
  /// * Len: `colorAttachmentCount`
  pub pColorAttachments: *const VkAttachmentReference2,
  /// * Optional: true
  /// * Len: `colorAttachmentCount`
  pub pResolveAttachments: *const VkAttachmentReference2,
  /// * Optional: true
  pub pDepthStencilAttachment: *const VkAttachmentReference2,
  /// * Optional: true
  pub preserveAttachmentCount: u32,
  /// * Len: `preserveAttachmentCount`
  pub pPreserveAttachments: *const u32,
}
impl Default for VkSubpassDescription2 {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2,
      pNext: core::ptr::null(),
      flags: Default::default(),
      pipelineBindPoint: Default::default(),
      viewMask: Default::default(),
      inputAttachmentCount: Default::default(),
      pInputAttachments: core::ptr::null(),
      colorAttachmentCount: Default::default(),
      pColorAttachments: core::ptr::null(),
      pResolveAttachments: core::ptr::null(),
      pDepthStencilAttachment: core::ptr::null(),
      preserveAttachmentCount: Default::default(),
      pPreserveAttachments: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassDescriptionDepthStencilResolve](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionDepthStencilResolve.html)
///
/// * Struct Extends: [`VkSubpassDescription2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDescriptionDepthStencilResolve {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// depth resolve mode
  /// * No Auto-Validity
  pub depthResolveMode: VkResolveModeFlagBits,
  /// stencil resolve mode
  /// * No Auto-Validity
  pub stencilResolveMode: VkResolveModeFlagBits,
  /// depth/stencil resolve attachment
  /// * Optional: true
  pub pDepthStencilResolveAttachment: *const VkAttachmentReference2,
}
impl Default for VkSubpassDescriptionDepthStencilResolve {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE,
      pNext: core::ptr::null(),
      depthResolveMode: Default::default(),
      stencilResolveMode: Default::default(),
      pDepthStencilResolveAttachment: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassEndInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassEndInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassEndInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_END_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
}
impl Default for VkSubpassEndInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBPASS_END_INFO,
      pNext: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassFragmentDensityMapOffsetEndInfoQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html)
///
/// * Struct Extends: [`VkSubpassEndInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub fragmentDensityOffsetCount: u32,
  /// * Len: `fragmentDensityOffsetCount`
  pub pFragmentDensityOffsets: *const VkOffset2D,
}
impl Default for VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM,
      pNext: core::ptr::null(),
      fragmentDensityOffsetCount: Default::default(),
      pFragmentDensityOffsets: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSubpassResolvePerformanceQueryEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassResolvePerformanceQueryEXT.html)
///
/// * Struct Extends: [`VkFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassResolvePerformanceQueryEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub optimal: VkBool32,
}
impl Default for VkSubpassResolvePerformanceQueryEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT,
      pNext: core::ptr::null_mut(),
      optimal: Default::default(),
    }
  }
}

/// Khronos: [VkSubpassSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassSampleLocationsEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassSampleLocationsEXT {
  pub subpassIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
impl Default for VkSubpassSampleLocationsEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      subpassIndex: Default::default(),
      sampleLocationsInfo: Default::default(),
    }
  }
}

/// Khronos: [VkSubpassShadingPipelineCreateInfoHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html)
///
/// * Struct Extends: [`VkComputePipelineCreateInfo`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub renderPass: VkRenderPass,
  pub subpass: u32,
}
impl Default for VkSubpassShadingPipelineCreateInfoHUAWEI {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI,
      pNext: core::ptr::null_mut(),
      renderPass: Default::default(),
      subpass: Default::default(),
    }
  }
}

/// Khronos: [VkSubresourceLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubresourceLayout.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubresourceLayout {
  /// Specified in bytes
  pub offset: VkDeviceSize,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Specified in bytes
  pub rowPitch: VkDeviceSize,
  /// Specified in bytes
  pub arrayPitch: VkDeviceSize,
  /// Specified in bytes
  pub depthPitch: VkDeviceSize,
}
impl Default for VkSubresourceLayout {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      offset: Default::default(),
      size: Default::default(),
      rowPitch: Default::default(),
      arrayPitch: Default::default(),
      depthPitch: Default::default(),
    }
  }
}

/// Khronos: [VkSubresourceLayout2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubresourceLayout2EXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubresourceLayout2EXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub subresourceLayout: VkSubresourceLayout,
}
impl Default for VkSubresourceLayout2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT,
      pNext: core::ptr::null_mut(),
      subresourceLayout: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilities2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2EXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilities2EXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Supported minimum number of images for the surface
  pub minImageCount: u32,
  /// Supported maximum number of images for the surface, 0 for unlimited
  pub maxImageCount: u32,
  /// Current image width and height for the surface, (0, 0) if undefined
  pub currentExtent: VkExtent2D,
  /// Supported minimum image width and height for the surface
  pub minImageExtent: VkExtent2D,
  /// Supported maximum image width and height for the surface
  pub maxImageExtent: VkExtent2D,
  /// Supported maximum number of image layers for the surface
  pub maxImageArrayLayers: u32,
  /// 1 or more bits representing the transforms supported
  pub supportedTransforms: VkSurfaceTransformFlagsKHR,
  /// The surface's current transform relative to the device's natural orientation
  pub currentTransform: VkSurfaceTransformFlagBitsKHR,
  /// 1 or more bits representing the alpha compositing modes supported
  pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
  /// Supported image usage flags for the surface
  pub supportedUsageFlags: VkImageUsageFlags,
  /// * Optional: true
  pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT,
}
impl Default for VkSurfaceCapabilities2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT,
      pNext: core::ptr::null_mut(),
      minImageCount: Default::default(),
      maxImageCount: Default::default(),
      currentExtent: Default::default(),
      minImageExtent: Default::default(),
      maxImageExtent: Default::default(),
      maxImageArrayLayers: Default::default(),
      supportedTransforms: Default::default(),
      currentTransform: Default::default(),
      supportedCompositeAlpha: Default::default(),
      supportedUsageFlags: Default::default(),
      supportedSurfaceCounters: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2KHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilities2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub surfaceCapabilities: VkSurfaceCapabilitiesKHR,
}
impl Default for VkSurfaceCapabilities2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR,
      pNext: core::ptr::null_mut(),
      surfaceCapabilities: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilitiesFullScreenExclusiveEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub fullScreenExclusiveSupported: VkBool32,
}
impl Default for VkSurfaceCapabilitiesFullScreenExclusiveEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT,
      pNext: core::ptr::null_mut(),
      fullScreenExclusiveSupported: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
  /// Supported minimum number of images for the surface
  pub minImageCount: u32,
  /// Supported maximum number of images for the surface, 0 for unlimited
  pub maxImageCount: u32,
  /// Current image width and height for the surface, (0, 0) if undefined
  pub currentExtent: VkExtent2D,
  /// Supported minimum image width and height for the surface
  pub minImageExtent: VkExtent2D,
  /// Supported maximum image width and height for the surface
  pub maxImageExtent: VkExtent2D,
  /// Supported maximum number of image layers for the surface
  pub maxImageArrayLayers: u32,
  /// 1 or more bits representing the transforms supported
  pub supportedTransforms: VkSurfaceTransformFlagsKHR,
  /// The surface's current transform relative to the device's natural orientation
  pub currentTransform: VkSurfaceTransformFlagBitsKHR,
  /// 1 or more bits representing the alpha compositing modes supported
  pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
  /// Supported image usage flags for the surface
  pub supportedUsageFlags: VkImageUsageFlags,
}
impl Default for VkSurfaceCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      minImageCount: Default::default(),
      maxImageCount: Default::default(),
      currentExtent: Default::default(),
      minImageExtent: Default::default(),
      maxImageExtent: Default::default(),
      maxImageArrayLayers: Default::default(),
      supportedTransforms: Default::default(),
      currentTransform: Default::default(),
      supportedCompositeAlpha: Default::default(),
      supportedUsageFlags: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceCapabilitiesPresentBarrierNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesPresentBarrierNV.html)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentBarrierNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_PRESENT_BARRIER_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub presentBarrierSupported: VkBool32,
}
impl Default for VkSurfaceCapabilitiesPresentBarrierNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_PRESENT_BARRIER_NV,
      pNext: core::ptr::null_mut(),
      presentBarrierSupported: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceFormat2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormat2KHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceFormat2KHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub surfaceFormat: VkSurfaceFormatKHR,
}
impl Default for VkSurfaceFormat2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR,
      pNext: core::ptr::null_mut(),
      surfaceFormat: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceFormatKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceFormatKHR {
  /// Supported pair of rendering format
  pub format: VkFormat,
  /// and color space for the surface
  pub colorSpace: VkColorSpaceKHR,
}
impl Default for VkSurfaceFormatKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      format: Default::default(),
      colorSpace: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceFullScreenExclusiveInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceSurfaceInfo2KHR`]
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub fullScreenExclusive: VkFullScreenExclusiveEXT,
}
impl Default for VkSurfaceFullScreenExclusiveInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
      pNext: core::ptr::null_mut(),
      fullScreenExclusive: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceFullScreenExclusiveWin32InfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceSurfaceInfo2KHR`]
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub hmonitor: HMONITOR,
}
impl Default for VkSurfaceFullScreenExclusiveWin32InfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT,
      pNext: core::ptr::null(),
      hmonitor: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkSurfacePresentModeCompatibilityEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfacePresentModeCompatibilityEXT.html)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfacePresentModeCompatibilityEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  pub presentModeCount: u32,
  /// Output list of present modes compatible with the one specified in VkSurfacePresentModeEXT
  /// * Optional: true
  /// * Len: `presentModeCount`
  pub pPresentModes: *mut VkPresentModeKHR,
}
impl Default for VkSurfacePresentModeCompatibilityEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT,
      pNext: core::ptr::null_mut(),
      presentModeCount: Default::default(),
      pPresentModes: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkSurfacePresentModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfacePresentModeEXT.html)
///
/// * Struct Extends: [`VkPhysicalDeviceSurfaceInfo2KHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfacePresentModeEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub presentMode: VkPresentModeKHR,
}
impl Default for VkSurfacePresentModeEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT,
      pNext: core::ptr::null_mut(),
      presentMode: Default::default(),
    }
  }
}

/// Khronos: [VkSurfacePresentScalingCapabilitiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfacePresentScalingCapabilitiesEXT.html)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfacePresentScalingCapabilitiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  pub supportedPresentScaling: VkPresentScalingFlagsEXT,
  /// * Optional: true
  pub supportedPresentGravityX: VkPresentGravityFlagsEXT,
  /// * Optional: true
  pub supportedPresentGravityY: VkPresentGravityFlagsEXT,
  /// Supported minimum image width and height for the surface when scaling is used
  /// * Optional: true
  pub minScaledImageExtent: VkExtent2D,
  /// Supported maximum image width and height for the surface when scaling is used
  /// * Optional: true
  pub maxScaledImageExtent: VkExtent2D,
}
impl Default for VkSurfacePresentScalingCapabilitiesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT,
      pNext: core::ptr::null_mut(),
      supportedPresentScaling: Default::default(),
      supportedPresentGravityX: Default::default(),
      supportedPresentGravityY: Default::default(),
      minScaledImageExtent: Default::default(),
      maxScaledImageExtent: Default::default(),
    }
  }
}

/// Khronos: [VkSurfaceProtectedCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html)
///
/// * Struct Extends: [`VkSurfaceCapabilities2KHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSurfaceProtectedCapabilitiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Represents if surface can be protected
  pub supportsProtected: VkBool32,
}
impl Default for VkSurfaceProtectedCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR,
      pNext: core::ptr::null(),
      supportsProtected: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainCounterCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainCounterCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub surfaceCounters: VkSurfaceCounterFlagsEXT,
}
impl Default for VkSwapchainCounterCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      surfaceCounters: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkSwapchainCreateFlagsKHR,
  /// The swapchain's target surface
  pub surface: VkSurfaceKHR,
  /// Minimum number of presentation images the application needs
  pub minImageCount: u32,
  /// Format of the presentation images
  pub imageFormat: VkFormat,
  /// Colorspace of the presentation images
  pub imageColorSpace: VkColorSpaceKHR,
  /// Dimensions of the presentation images
  pub imageExtent: VkExtent2D,
  /// Determines the number of views for multiview/stereo presentation
  pub imageArrayLayers: u32,
  /// Bits indicating how the presentation images will be used
  pub imageUsage: VkImageUsageFlags,
  /// Sharing mode used for the presentation images
  pub imageSharingMode: VkSharingMode,
  /// Number of queue families having access to the images in case of concurrent sharing mode
  /// * Optional: true
  pub queueFamilyIndexCount: u32,
  /// Array of queue family indices having access to the images in case of concurrent sharing mode
  /// * Len: `queueFamilyIndexCount`
  /// * No Auto-Validity
  pub pQueueFamilyIndices: *const u32,
  /// The transform, relative to the device's natural orientation, applied to the image content prior to presentation
  pub preTransform: VkSurfaceTransformFlagBitsKHR,
  /// The alpha blending mode used when compositing this surface with other surfaces in the window system
  pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
  /// Which presentation mode to use for presents on this swap chain
  pub presentMode: VkPresentModeKHR,
  /// Specifies whether presentable images may be affected by window clip regions
  pub clipped: VkBool32,
  /// Existing swap chain to replace, if any
  /// * Optional: true
  pub oldSwapchain: VkSwapchainKHR,
}
impl Default for VkSwapchainCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      surface: Default::default(),
      minImageCount: Default::default(),
      imageFormat: Default::default(),
      imageColorSpace: Default::default(),
      imageExtent: Default::default(),
      imageArrayLayers: Default::default(),
      imageUsage: Default::default(),
      imageSharingMode: Default::default(),
      queueFamilyIndexCount: Default::default(),
      pQueueFamilyIndices: core::ptr::null(),
      preTransform: Default::default(),
      compositeAlpha: Default::default(),
      presentMode: Default::default(),
      clipped: Default::default(),
      oldSwapchain: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainDisplayNativeHdrCreateInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub localDimmingEnable: VkBool32,
}
impl Default for VkSwapchainDisplayNativeHdrCreateInfoAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD,
      pNext: core::ptr::null(),
      localDimmingEnable: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainImageCreateInfoANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainImageCreateInfoANDROID.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainImageCreateInfoANDROID {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub usage: VkSwapchainImageUsageFlagsANDROID,
}
impl Default for VkSwapchainImageCreateInfoANDROID {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID,
      pNext: core::ptr::null(),
      usage: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainPresentBarrierCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentBarrierCreateInfoNV.html)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentBarrierCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub presentBarrierEnable: VkBool32,
}
impl Default for VkSwapchainPresentBarrierCreateInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV,
      pNext: core::ptr::null_mut(),
      presentBarrierEnable: Default::default(),
    }
  }
}

/// Khronos: [VkSwapchainPresentFenceInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentFenceInfoEXT.html)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentFenceInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_FENCE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchainCount: u32,
  /// Fence to signal for each swapchain
  /// * Len: `swapchainCount`
  pub pFences: *const VkFence,
}
impl Default for VkSwapchainPresentFenceInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_FENCE_INFO_EXT,
      pNext: core::ptr::null_mut(),
      swapchainCount: Default::default(),
      pFences: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSwapchainPresentModeInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentModeInfoEXT.html)
///
/// * Struct Extends: [`VkPresentInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentModeInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// Copy of VkPresentInfoKHR::swapchainCount
  pub swapchainCount: u32,
  /// Presentation mode for each swapchain
  /// * Len: `swapchainCount`
  pub pPresentModes: *const VkPresentModeKHR,
}
impl Default for VkSwapchainPresentModeInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODE_INFO_EXT,
      pNext: core::ptr::null_mut(),
      swapchainCount: Default::default(),
      pPresentModes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSwapchainPresentModesCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentModesCreateInfoEXT.html)
///
/// Presentation modes which will be usable with this swapchain
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentModesCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub presentModeCount: u32,
  /// * Len: `presentModeCount`
  pub pPresentModes: *const VkPresentModeKHR,
}
impl Default for VkSwapchainPresentModesCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT,
      pNext: core::ptr::null_mut(),
      presentModeCount: Default::default(),
      pPresentModes: core::ptr::null(),
    }
  }
}

/// Khronos: [VkSwapchainPresentScalingCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainPresentScalingCreateInfoEXT.html)
///
/// * Struct Extends: [`VkSwapchainCreateInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainPresentScalingCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub scalingBehavior: VkPresentScalingFlagsEXT,
  /// * Optional: true
  pub presentGravityX: VkPresentGravityFlagsEXT,
  /// * Optional: true
  pub presentGravityY: VkPresentGravityFlagsEXT,
}
impl Default for VkSwapchainPresentScalingCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      scalingBehavior: Default::default(),
      presentGravityX: Default::default(),
      presentGravityY: Default::default(),
    }
  }
}

/// Khronos: [VkSysmemColorSpaceFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSysmemColorSpaceFUCHSIA.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSysmemColorSpaceFUCHSIA {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub colorSpace: u32,
}
impl Default for VkSysmemColorSpaceFUCHSIA {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA,
      pNext: core::ptr::null(),
      colorSpace: Default::default(),
    }
  }
}

/// Khronos: [VkTextureLODGatherFormatPropertiesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTextureLODGatherFormatPropertiesAMD.html)
///
/// * Struct Extends: [`VkImageFormatProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTextureLODGatherFormatPropertiesAMD {
  /// * Intended Value: `VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub supportsTextureGatherLODBiasAMD: VkBool32,
}
impl Default for VkTextureLODGatherFormatPropertiesAMD {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
      pNext: core::ptr::null_mut(),
      supportsTextureGatherLODBiasAMD: Default::default(),
    }
  }
}

/// Khronos: [VkTilePropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTilePropertiesQCOM.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTilePropertiesQCOM {
  /// * Intended Value: `VK_STRUCTURE_TYPE_TILE_PROPERTIES_QCOM`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub tileSize: VkExtent3D,
  pub apronSize: VkExtent2D,
  pub origin: VkOffset2D,
}
impl Default for VkTilePropertiesQCOM {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_TILE_PROPERTIES_QCOM,
      pNext: core::ptr::null_mut(),
      tileSize: Default::default(),
      apronSize: Default::default(),
      origin: Default::default(),
    }
  }
}

/// Khronos: [VkTimelineSemaphoreSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTimelineSemaphoreSubmitInfo.html)
///
/// * Struct Extends: [`VkSubmitInfo`]
/// * Struct Extends: [`VkBindSparseInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTimelineSemaphoreSubmitInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub waitSemaphoreValueCount: u32,
  /// * Optional: true
  /// * Len: `waitSemaphoreValueCount`
  pub pWaitSemaphoreValues: *const u64,
  /// * Optional: true
  pub signalSemaphoreValueCount: u32,
  /// * Optional: true
  /// * Len: `signalSemaphoreValueCount`
  pub pSignalSemaphoreValues: *const u64,
}
impl Default for VkTimelineSemaphoreSubmitInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO,
      pNext: core::ptr::null(),
      waitSemaphoreValueCount: Default::default(),
      pWaitSemaphoreValues: core::ptr::null(),
      signalSemaphoreValueCount: Default::default(),
      pSignalSemaphoreValues: core::ptr::null(),
    }
  }
}

/// Khronos: [VkTraceRaysIndirectCommand2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommand2KHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTraceRaysIndirectCommand2KHR {
  pub raygenShaderRecordAddress: VkDeviceAddress,
  pub raygenShaderRecordSize: VkDeviceSize,
  pub missShaderBindingTableAddress: VkDeviceAddress,
  pub missShaderBindingTableSize: VkDeviceSize,
  pub missShaderBindingTableStride: VkDeviceSize,
  pub hitShaderBindingTableAddress: VkDeviceAddress,
  pub hitShaderBindingTableSize: VkDeviceSize,
  pub hitShaderBindingTableStride: VkDeviceSize,
  pub callableShaderBindingTableAddress: VkDeviceAddress,
  pub callableShaderBindingTableSize: VkDeviceSize,
  pub callableShaderBindingTableStride: VkDeviceSize,
  pub width: u32,
  pub height: u32,
  pub depth: u32,
}
impl Default for VkTraceRaysIndirectCommand2KHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      raygenShaderRecordAddress: Default::default(),
      raygenShaderRecordSize: Default::default(),
      missShaderBindingTableAddress: Default::default(),
      missShaderBindingTableSize: Default::default(),
      missShaderBindingTableStride: Default::default(),
      hitShaderBindingTableAddress: Default::default(),
      hitShaderBindingTableSize: Default::default(),
      hitShaderBindingTableStride: Default::default(),
      callableShaderBindingTableAddress: Default::default(),
      callableShaderBindingTableSize: Default::default(),
      callableShaderBindingTableStride: Default::default(),
      width: Default::default(),
      height: Default::default(),
      depth: Default::default(),
    }
  }
}

/// Khronos: [VkTraceRaysIndirectCommandKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommandKHR.html)
#[derive(Clone, Copy)]
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

/// Khronos: [VkTransformMatrixKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTransformMatrixKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTransformMatrixKHR {
  pub matrix: [[c_float; 3]; 4],
}
impl Default for VkTransformMatrixKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      matrix: [[Default::default(); 3]; 4],
    }
  }
}

/// Khronos: [VkValidationCacheCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkValidationCacheCreateInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkValidationCacheCreateFlagsEXT,
  /// * Optional: true
  pub initialDataSize: c_size_t,
  /// * Len: `initialDataSize`
  pub pInitialData: *const c_void,
}
impl Default for VkValidationCacheCreateInfoEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT,
      pNext: core::ptr::null(),
      flags: Default::default(),
      initialDataSize: Default::default(),
      pInitialData: core::ptr::null(),
    }
  }
}

/// Khronos: [VkValidationFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationFeaturesEXT.html)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkValidationFeaturesEXT {
  /// Must be VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT
  /// * Intended Value: `VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Number of validation features to enable
  /// * Optional: true
  pub enabledValidationFeatureCount: u32,
  /// Validation features to enable
  /// * Len: `enabledValidationFeatureCount`
  pub pEnabledValidationFeatures: *const VkValidationFeatureEnableEXT,
  /// Number of validation features to disable
  /// * Optional: true
  pub disabledValidationFeatureCount: u32,
  /// Validation features to disable
  /// * Len: `disabledValidationFeatureCount`
  pub pDisabledValidationFeatures: *const VkValidationFeatureDisableEXT,
}
impl Default for VkValidationFeaturesEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT,
      pNext: core::ptr::null(),
      enabledValidationFeatureCount: Default::default(),
      pEnabledValidationFeatures: core::ptr::null(),
      disabledValidationFeatureCount: Default::default(),
      pDisabledValidationFeatures: core::ptr::null(),
    }
  }
}

/// Khronos: [VkValidationFlagsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationFlagsEXT.html)
///
/// * Struct Extends: [`VkInstanceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkValidationFlagsEXT {
  /// Must be VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT
  /// * Intended Value: `VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Number of validation checks to disable
  pub disabledValidationCheckCount: u32,
  /// Validation checks to disable
  /// * Len: `disabledValidationCheckCount`
  pub pDisabledValidationChecks: *const VkValidationCheckEXT,
}
impl Default for VkValidationFlagsEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT,
      pNext: core::ptr::null(),
      disabledValidationCheckCount: Default::default(),
      pDisabledValidationChecks: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVertexInputAttributeDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription.html)
#[derive(Clone, Copy)]
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

/// Khronos: [VkVertexInputAttributeDescription2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription2EXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputAttributeDescription2EXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
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
      sType: VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT,
      pNext: core::ptr::null_mut(),
      location: Default::default(),
      binding: Default::default(),
      format: Default::default(),
      offset: Default::default(),
    }
  }
}

/// Khronos: [VkVertexInputBindingDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputBindingDescription {
  /// Vertex buffer binding id
  pub binding: u32,
  /// Distance between vertices in bytes (0 = no advancement)
  pub stride: u32,
  /// The rate at which the vertex data is consumed
  pub inputRate: VkVertexInputRate,
}
impl Default for VkVertexInputBindingDescription {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      binding: Default::default(),
      stride: Default::default(),
      inputRate: Default::default(),
    }
  }
}

/// Khronos: [VkVertexInputBindingDescription2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription2EXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputBindingDescription2EXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub binding: u32,
  pub stride: u32,
  pub inputRate: VkVertexInputRate,
  pub divisor: u32,
}
impl Default for VkVertexInputBindingDescription2EXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT,
      pNext: core::ptr::null_mut(),
      binding: Default::default(),
      stride: Default::default(),
      inputRate: Default::default(),
      divisor: Default::default(),
    }
  }
}

/// Khronos: [VkVertexInputBindingDivisorDescriptionEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputBindingDivisorDescriptionEXT {
  pub binding: u32,
  pub divisor: u32,
}
impl Default for VkVertexInputBindingDivisorDescriptionEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      binding: Default::default(),
      divisor: Default::default(),
    }
  }
}

/// Khronos: [VkViSurfaceCreateInfoNN](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViSurfaceCreateInfoNN.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViSurfaceCreateInfoNN {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN,
      pNext: core::ptr::null(),
      flags: Default::default(),
      window: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkVideoBeginCodingInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoBeginCodingInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkVideoBeginCodingFlagsKHR,
  pub videoSession: VkVideoSessionKHR,
  /// * Optional: true
  pub videoSessionParameters: VkVideoSessionParametersKHR,
  /// * Optional: true
  pub referenceSlotCount: u32,
  /// * Len: `referenceSlotCount`
  pub pReferenceSlots: *const VkVideoReferenceSlotInfoKHR,
}
impl Default for VkVideoBeginCodingInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      videoSession: Default::default(),
      videoSessionParameters: Default::default(),
      referenceSlotCount: Default::default(),
      pReferenceSlots: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilitiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoCapabilitiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub flags: VkVideoCapabilityFlagsKHR,
  pub minBitstreamBufferOffsetAlignment: VkDeviceSize,
  pub minBitstreamBufferSizeAlignment: VkDeviceSize,
  pub pictureAccessGranularity: VkExtent2D,
  pub minCodedExtent: VkExtent2D,
  pub maxCodedExtent: VkExtent2D,
  pub maxDpbSlots: u32,
  pub maxActiveReferencePictures: u32,
  pub stdHeaderVersion: VkExtensionProperties,
}
impl Default for VkVideoCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR,
      pNext: core::ptr::null_mut(),
      flags: Default::default(),
      minBitstreamBufferOffsetAlignment: Default::default(),
      minBitstreamBufferSizeAlignment: Default::default(),
      pictureAccessGranularity: Default::default(),
      minCodedExtent: Default::default(),
      maxCodedExtent: Default::default(),
      maxDpbSlots: Default::default(),
      maxActiveReferencePictures: Default::default(),
      stdHeaderVersion: Default::default(),
    }
  }
}

/// Khronos: [VkVideoCodingControlInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoCodingControlInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: false
  pub flags: VkVideoCodingControlFlagsKHR,
}
impl Default for VkVideoCodingControlInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkVideoDecodeCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilitiesKHR.html)
///
/// * Struct Extends: [`VkVideoCapabilitiesKHR`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeCapabilitiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * No Auto-Validity
  pub flags: VkVideoDecodeCapabilityFlagsKHR,
}
impl Default for VkVideoDecodeCapabilitiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_DECODE_CAPABILITIES_KHR,
      pNext: core::ptr::null_mut(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkVideoDecodeInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkVideoDecodeFlagsKHR,
  pub srcBuffer: VkBuffer,
  pub srcBufferOffset: VkDeviceSize,
  pub srcBufferRange: VkDeviceSize,
  pub dstPictureResource: VkVideoPictureResourceInfoKHR,
  /// * Optional: true
  pub pSetupReferenceSlot: *const VkVideoReferenceSlotInfoKHR,
  /// * Optional: true
  pub referenceSlotCount: u32,
  /// * Len: `referenceSlotCount`
  pub pReferenceSlots: *const VkVideoReferenceSlotInfoKHR,
}
impl Default for VkVideoDecodeInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      srcBuffer: Default::default(),
      srcBufferOffset: Default::default(),
      srcBufferRange: Default::default(),
      dstPictureResource: Default::default(),
      pSetupReferenceSlot: core::ptr::null(),
      referenceSlotCount: Default::default(),
      pReferenceSlots: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoDecodeUsageInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeUsageInfoKHR.html)
///
/// * Struct Extends: [`VkVideoProfileInfoKHR`]
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoDecodeUsageInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_DECODE_USAGE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub videoUsageHints: VkVideoDecodeUsageFlagsKHR,
}
impl Default for VkVideoDecodeUsageInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_DECODE_USAGE_INFO_KHR,
      pNext: core::ptr::null(),
      videoUsageHints: Default::default(),
    }
  }
}

/// Khronos: [VkVideoEncodeH264FrameSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264FrameSizeEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264FrameSizeEXT {
  /// * No Auto-Validity
  pub frameISize: u32,
  /// * No Auto-Validity
  pub framePSize: u32,
  /// * No Auto-Validity
  pub frameBSize: u32,
}
impl Default for VkVideoEncodeH264FrameSizeEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      frameISize: Default::default(),
      framePSize: Default::default(),
      frameBSize: Default::default(),
    }
  }
}

/// Khronos: [VkVideoEncodeH264QpEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264QpEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264QpEXT {
  /// * No Auto-Validity
  pub qpI: i32,
  /// * No Auto-Validity
  pub qpP: i32,
  /// * No Auto-Validity
  pub qpB: i32,
}
impl Default for VkVideoEncodeH264QpEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      qpI: Default::default(),
      qpP: Default::default(),
      qpB: Default::default(),
    }
  }
}

/// Khronos: [VkVideoEncodeH265FrameSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265FrameSizeEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265FrameSizeEXT {
  /// * No Auto-Validity
  pub frameISize: u32,
  /// * No Auto-Validity
  pub framePSize: u32,
  /// * No Auto-Validity
  pub frameBSize: u32,
}
impl Default for VkVideoEncodeH265FrameSizeEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      frameISize: Default::default(),
      framePSize: Default::default(),
      frameBSize: Default::default(),
    }
  }
}

/// Khronos: [VkVideoEncodeH265QpEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265QpEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265QpEXT {
  /// * No Auto-Validity
  pub qpI: i32,
  /// * No Auto-Validity
  pub qpP: i32,
  /// * No Auto-Validity
  pub qpB: i32,
}
impl Default for VkVideoEncodeH265QpEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      qpI: Default::default(),
      qpP: Default::default(),
      qpB: Default::default(),
    }
  }
}

/// Khronos: [VkVideoEndCodingInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEndCodingInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkVideoEndCodingFlagsKHR,
}
impl Default for VkVideoEndCodingInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
    }
  }
}

/// Khronos: [VkVideoFormatPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoFormatPropertiesKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoFormatPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub format: VkFormat,
  pub componentMapping: VkComponentMapping,
  pub imageCreateFlags: VkImageCreateFlags,
  pub imageType: VkImageType,
  pub imageTiling: VkImageTiling,
  pub imageUsageFlags: VkImageUsageFlags,
}
impl Default for VkVideoFormatPropertiesKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR,
      pNext: core::ptr::null_mut(),
      format: Default::default(),
      componentMapping: Default::default(),
      imageCreateFlags: Default::default(),
      imageType: Default::default(),
      imageTiling: Default::default(),
      imageUsageFlags: Default::default(),
    }
  }
}

/// Khronos: [VkVideoPictureResourceInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoPictureResourceInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoPictureResourceInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// The offset to be used for the picture resource, currently only used in field mode
  pub codedOffset: VkOffset2D,
  /// The extent to be used for the picture resource
  pub codedExtent: VkExtent2D,
  /// The first array layer to be accessed for the Decode or Encode Operations
  pub baseArrayLayer: u32,
  /// The ImageView binding of the resource
  pub imageViewBinding: VkImageView,
}
impl Default for VkVideoPictureResourceInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_INFO_KHR,
      pNext: core::ptr::null(),
      codedOffset: Default::default(),
      codedExtent: Default::default(),
      baseArrayLayer: Default::default(),
      imageViewBinding: Default::default(),
    }
  }
}

/// Khronos: [VkVideoProfileInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoProfileInfoKHR.html)
///
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoProfileInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_PROFILE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub videoCodecOperation: VkVideoCodecOperationFlagBitsKHR,
  pub chromaSubsampling: VkVideoChromaSubsamplingFlagsKHR,
  pub lumaBitDepth: VkVideoComponentBitDepthFlagsKHR,
  /// * Optional: true
  pub chromaBitDepth: VkVideoComponentBitDepthFlagsKHR,
}
impl Default for VkVideoProfileInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_PROFILE_INFO_KHR,
      pNext: core::ptr::null(),
      videoCodecOperation: Default::default(),
      chromaSubsampling: Default::default(),
      lumaBitDepth: Default::default(),
      chromaBitDepth: Default::default(),
    }
  }
}

/// Khronos: [VkVideoProfileListInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoProfileListInfoKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceImageFormatInfo2`]
/// * Struct Extends: [`VkPhysicalDeviceVideoFormatInfoKHR`]
/// * Struct Extends: [`VkImageCreateInfo`]
/// * Struct Extends: [`VkBufferCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoProfileListInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_PROFILE_LIST_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub profileCount: u32,
  /// * Len: `profileCount`
  pub pProfiles: *const VkVideoProfileInfoKHR,
}
impl Default for VkVideoProfileListInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_PROFILE_LIST_INFO_KHR,
      pNext: core::ptr::null(),
      profileCount: Default::default(),
      pProfiles: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoReferenceSlotInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoReferenceSlotInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoReferenceSlotInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// The reference slot index
  pub slotIndex: i32,
  /// The reference picture resource
  /// * Optional: true
  pub pPictureResource: *const VkVideoPictureResourceInfoKHR,
}
impl Default for VkVideoReferenceSlotInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_INFO_KHR,
      pNext: core::ptr::null(),
      slotIndex: Default::default(),
      pPictureResource: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoSessionCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoSessionCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub queueFamilyIndex: u32,
  /// * Optional: true
  pub flags: VkVideoSessionCreateFlagsKHR,
  pub pVideoProfile: *const VkVideoProfileInfoKHR,
  pub pictureFormat: VkFormat,
  pub maxCodedExtent: VkExtent2D,
  pub referencePictureFormat: VkFormat,
  pub maxDpbSlots: u32,
  pub maxActiveReferencePictures: u32,
  pub pStdHeaderVersion: *const VkExtensionProperties,
}
impl Default for VkVideoSessionCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      queueFamilyIndex: Default::default(),
      flags: Default::default(),
      pVideoProfile: core::ptr::null(),
      pictureFormat: Default::default(),
      maxCodedExtent: Default::default(),
      referencePictureFormat: Default::default(),
      maxDpbSlots: Default::default(),
      maxActiveReferencePictures: Default::default(),
      pStdHeaderVersion: core::ptr::null(),
    }
  }
}

/// Khronos: [VkVideoSessionMemoryRequirementsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionMemoryRequirementsKHR.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoSessionMemoryRequirementsKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub memoryBindIndex: u32,
  pub memoryRequirements: VkMemoryRequirements,
}
impl Default for VkVideoSessionMemoryRequirementsKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR,
      pNext: core::ptr::null_mut(),
      memoryBindIndex: Default::default(),
      memoryRequirements: Default::default(),
    }
  }
}

/// Khronos: [VkVideoSessionParametersCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoSessionParametersCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkVideoSessionParametersCreateFlagsKHR,
  /// * Optional: true
  pub videoSessionParametersTemplate: VkVideoSessionParametersKHR,
  pub videoSession: VkVideoSessionKHR,
}
impl Default for VkVideoSessionParametersCreateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      videoSessionParametersTemplate: Default::default(),
      videoSession: Default::default(),
    }
  }
}

/// Khronos: [VkVideoSessionParametersUpdateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersUpdateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoSessionParametersUpdateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub updateSequenceCount: u32,
}
impl Default for VkVideoSessionParametersUpdateInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR,
      pNext: core::ptr::null(),
      updateSequenceCount: Default::default(),
    }
  }
}

/// Khronos: [VkViewport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewport.html)
#[derive(Clone, Copy)]
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
  pub minDepth: c_float,
  pub maxDepth: c_float,
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
      minDepth: Default::default(),
      maxDepth: Default::default(),
    }
  }
}

/// Khronos: [VkViewportSwizzleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewportSwizzleNV.html)
#[derive(Clone, Copy)]
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

/// Khronos: [VkViewportWScalingNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewportWScalingNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViewportWScalingNV {
  pub xcoeff: c_float,
  pub ycoeff: c_float,
}
impl Default for VkViewportWScalingNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      xcoeff: Default::default(),
      ycoeff: Default::default(),
    }
  }
}

/// Khronos: [VkWaylandSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWaylandSurfaceCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWaylandSurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      display: core::ptr::null_mut(),
      surface: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkWin32KeyedMutexAcquireReleaseInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoKHR.html)
///
/// * Struct Extends: [`VkSubmitInfo`]
/// * Struct Extends: [`VkSubmitInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub acquireCount: u32,
  /// * Len: `acquireCount`
  pub pAcquireSyncs: *const VkDeviceMemory,
  /// * Len: `acquireCount`
  pub pAcquireKeys: *const u64,
  /// * Len: `acquireCount`
  pub pAcquireTimeouts: *const u32,
  /// * Optional: true
  pub releaseCount: u32,
  /// * Len: `releaseCount`
  pub pReleaseSyncs: *const VkDeviceMemory,
  /// * Len: `releaseCount`
  pub pReleaseKeys: *const u64,
}
impl Default for VkWin32KeyedMutexAcquireReleaseInfoKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR,
      pNext: core::ptr::null(),
      acquireCount: Default::default(),
      pAcquireSyncs: core::ptr::null(),
      pAcquireKeys: core::ptr::null(),
      pAcquireTimeouts: core::ptr::null(),
      releaseCount: Default::default(),
      pReleaseSyncs: core::ptr::null(),
      pReleaseKeys: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWin32KeyedMutexAcquireReleaseInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoNV.html)
///
/// * Struct Extends: [`VkSubmitInfo`]
/// * Struct Extends: [`VkSubmitInfo2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub acquireCount: u32,
  /// * Len: `acquireCount`
  pub pAcquireSyncs: *const VkDeviceMemory,
  /// * Len: `acquireCount`
  pub pAcquireKeys: *const u64,
  /// * Len: `acquireCount`
  pub pAcquireTimeoutMilliseconds: *const u32,
  /// * Optional: true
  pub releaseCount: u32,
  /// * Len: `releaseCount`
  pub pReleaseSyncs: *const VkDeviceMemory,
  /// * Len: `releaseCount`
  pub pReleaseKeys: *const u64,
}
impl Default for VkWin32KeyedMutexAcquireReleaseInfoNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV,
      pNext: core::ptr::null(),
      acquireCount: Default::default(),
      pAcquireSyncs: core::ptr::null(),
      pAcquireKeys: core::ptr::null(),
      pAcquireTimeoutMilliseconds: core::ptr::null(),
      releaseCount: Default::default(),
      pReleaseSyncs: core::ptr::null(),
      pReleaseKeys: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWin32SurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      hinstance: core::ptr::null_mut(),
      hwnd: core::ptr::null_mut(),
    }
  }
}

/// Khronos: [VkWriteDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSet.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSet {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Destination descriptor set
  /// * No Auto-Validity
  pub dstSet: VkDescriptorSet,
  /// Binding within the destination descriptor set to write
  pub dstBinding: u32,
  /// Array element within the destination binding to write
  pub dstArrayElement: u32,
  /// Number of descriptors to write (determines the size of the array pointed by pDescriptors)
  pub descriptorCount: u32,
  /// Descriptor type to write (determines which members of the array pointed by pDescriptors are going to be used)
  pub descriptorType: VkDescriptorType,
  /// Sampler, image view, and layout for SAMPLER, COMBINED_IMAGE_SAMPLER, {SAMPLED,STORAGE}_IMAGE, and INPUT_ATTACHMENT descriptor types.
  /// * Len: `descriptorCount`
  /// * No Auto-Validity
  pub pImageInfo: *const VkDescriptorImageInfo,
  /// Raw buffer, size, and offset for {UNIFORM,STORAGE}_BUFFER\[_DYNAMIC\] descriptor types.
  /// * Len: `descriptorCount`
  /// * No Auto-Validity
  pub pBufferInfo: *const VkDescriptorBufferInfo,
  /// Buffer view to write to the descriptor for {UNIFORM,STORAGE}_TEXEL_BUFFER descriptor types.
  /// * Len: `descriptorCount`
  /// * No Auto-Validity
  pub pTexelBufferView: *const VkBufferView,
}
impl Default for VkWriteDescriptorSet {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
      pNext: core::ptr::null(),
      dstSet: Default::default(),
      dstBinding: Default::default(),
      dstArrayElement: Default::default(),
      descriptorCount: Default::default(),
      descriptorType: Default::default(),
      pImageInfo: core::ptr::null(),
      pBufferInfo: core::ptr::null(),
      pTexelBufferView: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWriteDescriptorSetAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html)
///
/// * Struct Extends: [`VkWriteDescriptorSet`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub accelerationStructureCount: u32,
  /// * Optional: false,true
  /// * Len: `accelerationStructureCount`
  pub pAccelerationStructures: *const VkAccelerationStructureKHR,
}
impl Default for VkWriteDescriptorSetAccelerationStructureKHR {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR,
      pNext: core::ptr::null(),
      accelerationStructureCount: Default::default(),
      pAccelerationStructures: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWriteDescriptorSetAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureNV.html)
///
/// * Struct Extends: [`VkWriteDescriptorSet`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub accelerationStructureCount: u32,
  /// * Optional: false,true
  /// * Len: `accelerationStructureCount`
  pub pAccelerationStructures: *const VkAccelerationStructureNV,
}
impl Default for VkWriteDescriptorSetAccelerationStructureNV {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV,
      pNext: core::ptr::null(),
      accelerationStructureCount: Default::default(),
      pAccelerationStructures: core::ptr::null(),
    }
  }
}

/// Khronos: [VkWriteDescriptorSetInlineUniformBlock](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetInlineUniformBlock.html)
///
/// * Struct Extends: [`VkWriteDescriptorSet`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSetInlineUniformBlock {
  /// * Intended Value: `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub dataSize: u32,
  /// * Len: `dataSize`
  pub pData: *const c_void,
}
impl Default for VkWriteDescriptorSetInlineUniformBlock {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK,
      pNext: core::ptr::null(),
      dataSize: Default::default(),
      pData: core::ptr::null(),
    }
  }
}

/// Khronos: [VkXYColorEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXYColorEXT.html)
///
/// Chromaticity coordinate
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkXYColorEXT {
  pub x: c_float,
  pub y: c_float,
}
impl Default for VkXYColorEXT {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self {
      x: Default::default(),
      y: Default::default(),
    }
  }
}

/// Khronos: [VkXcbSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkXcbSurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      connection: core::ptr::null_mut(),
      window: Default::default(),
    }
  }
}

/// Khronos: [VkXlibSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkXlibSurfaceCreateInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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
      sType: VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
      pNext: core::ptr::null(),
      flags: Default::default(),
      dpy: core::ptr::null_mut(),
      window: Default::default(),
    }
  }
}

