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

/// Khronos: [VkAccelerationStructureBuildRangeInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
  pub primitiveCount: u32,
  pub primitiveOffset: u32,
  pub firstVertex: u32,
  pub transformOffset: u32,
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

/// Khronos: [VkAttachmentReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentReference {
  pub attachment: u32,
  pub layout: VkImageLayout,
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

/// Khronos: [VkAttachmentSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleLocationsEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentSampleLocationsEXT {
  pub attachmentIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}

/// Khronos: [VkBaseInStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseInStructure.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseInStructure {
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const VkBaseInStructure,
}

/// Khronos: [VkBaseOutStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseOutStructure.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseOutStructure {
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut VkBaseOutStructure,
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

/// Khronos: [VkBindIndexBufferIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindIndexBufferIndirectCommandNV {
  pub bufferAddress: VkDeviceAddress,
  pub size: u32,
  pub indexType: VkIndexType,
}

/// Khronos: [VkBindShaderGroupIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindShaderGroupIndirectCommandNV {
  pub groupIndex: u32,
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

/// Khronos: [VkBindVertexBufferIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindVertexBufferIndirectCommandNV {
  pub bufferAddress: VkDeviceAddress,
  pub size: u32,
  pub stride: u32,
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

/// Khronos: [VkClearAttachment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearAttachment.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearAttachment {
  pub aspectMask: VkImageAspectFlags,
  pub colorAttachment: u32,
  /// * No Auto-Validity
  pub clearValue: VkClearValue,
}

/// Khronos: [VkClearDepthStencilValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearDepthStencilValue.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearDepthStencilValue {
  pub depth: c_float,
  pub stencil: u32,
}

/// Khronos: [VkClearRect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearRect.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearRect {
  pub rect: VkRect2D,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}

/// Khronos: [VkCoarseSampleLocationNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleLocationNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCoarseSampleLocationNV {
  pub pixelX: u32,
  pub pixelY: u32,
  pub sample: u32,
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

/// Khronos: [VkCommandPoolMemoryConsumption](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolMemoryConsumption.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandPoolMemoryConsumption {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_POOL_MEMORY_CONSUMPTION`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub commandPoolAllocated: VkDeviceSize,
  pub commandPoolReservedSize: VkDeviceSize,
  pub commandBufferAllocated: VkDeviceSize,
}

/// Khronos: [VkCommandPoolMemoryReservationCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolMemoryReservationCreateInfo.html)
///
/// * Struct Extends: [`VkCommandPoolCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandPoolMemoryReservationCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_COMMAND_POOL_MEMORY_RESERVATION_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: false
  pub commandPoolReservedSize: VkDeviceSize,
  /// * Optional: false
  pub commandPoolMaxCommandBuffers: u32,
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

/// Khronos: [VkConformanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConformanceVersion.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkConformanceVersion {
  pub major: u8,
  pub minor: u8,
  pub subminor: u8,
  pub patch: u8,
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

/// Khronos: [VkCopyMemoryIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyMemoryIndirectCommandNV {
  pub srcAddress: VkDeviceAddress,
  pub dstAddress: VkDeviceAddress,
  /// Specified in bytes
  pub size: VkDeviceSize,
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

/// Khronos: [VkDescriptorPoolSize](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolSize.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorPoolSize {
  pub ty: VkDescriptorType,
  pub descriptorCount: u32,
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

/// Khronos: [VkDeviceFaultAddressInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultAddressInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceFaultAddressInfoEXT {
  pub addressType: VkDeviceFaultAddressTypeEXT,
  pub reportedAddress: VkDeviceAddress,
  pub addressPrecision: VkDeviceSize,
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

/// Khronos: [VkDeviceObjectReservationCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceObjectReservationCreateInfo.html)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceObjectReservationCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_OBJECT_RESERVATION_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub pipelineCacheCreateInfoCount: u32,
  /// * Len: `pipelineCacheCreateInfoCount`
  pub pPipelineCacheCreateInfos: *const VkPipelineCacheCreateInfo,
  /// * Optional: true
  pub pipelinePoolSizeCount: u32,
  /// * Len: `pipelinePoolSizeCount`
  pub pPipelinePoolSizes: *const VkPipelinePoolSize,
  /// * Optional: true
  pub semaphoreRequestCount: u32,
  /// * Optional: true
  pub commandBufferRequestCount: u32,
  /// * Optional: true
  pub fenceRequestCount: u32,
  /// * Optional: true
  pub deviceMemoryRequestCount: u32,
  /// * Optional: true
  pub bufferRequestCount: u32,
  /// * Optional: true
  pub imageRequestCount: u32,
  /// * Optional: true
  pub eventRequestCount: u32,
  /// * Optional: true
  pub queryPoolRequestCount: u32,
  /// * Optional: true
  pub bufferViewRequestCount: u32,
  /// * Optional: true
  pub imageViewRequestCount: u32,
  /// * Optional: true
  pub layeredImageViewRequestCount: u32,
  /// * Optional: true
  pub pipelineCacheRequestCount: u32,
  /// * Optional: true
  pub pipelineLayoutRequestCount: u32,
  /// * Optional: true
  pub renderPassRequestCount: u32,
  /// * Optional: true
  pub graphicsPipelineRequestCount: u32,
  /// * Optional: true
  pub computePipelineRequestCount: u32,
  /// * Optional: true
  pub descriptorSetLayoutRequestCount: u32,
  /// * Optional: true
  pub samplerRequestCount: u32,
  /// * Optional: true
  pub descriptorPoolRequestCount: u32,
  /// * Optional: true
  pub descriptorSetRequestCount: u32,
  /// * Optional: true
  pub framebufferRequestCount: u32,
  /// * Optional: true
  pub commandPoolRequestCount: u32,
  /// * Optional: true
  pub samplerYcbcrConversionRequestCount: u32,
  /// * Optional: true
  pub surfaceRequestCount: u32,
  /// * Optional: true
  pub swapchainRequestCount: u32,
  /// * Optional: true
  pub displayModeRequestCount: u32,
  /// * Optional: true
  pub subpassDescriptionRequestCount: u32,
  /// * Optional: true
  pub attachmentDescriptionRequestCount: u32,
  /// * Optional: true
  pub descriptorSetLayoutBindingRequestCount: u32,
  pub descriptorSetLayoutBindingLimit: u32,
  pub maxImageViewMipLevels: u32,
  pub maxImageViewArrayLayers: u32,
  pub maxLayeredImageViewMipLevels: u32,
  pub maxOcclusionQueriesPerPool: u32,
  pub maxPipelineStatisticsQueriesPerPool: u32,
  pub maxTimestampQueriesPerPool: u32,
  pub maxImmutableSamplersPerDescriptorSetLayout: u32,
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

/// Khronos: [VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV.html)
///
/// * Struct Extends: [`VkDeviceObjectReservationCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV {
  /// * Intended Value: `VK_STRUCTURE_TYPE_DEVICE_SEMAPHORE_SCI_SYNC_POOL_RESERVATION_CREATE_INFO_NV`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub semaphoreSciSyncPoolRequestCount: u32,
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

/// Khronos: [VkDrawMeshTasksIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandNV {
  pub taskCount: u32,
  pub firstTask: u32,
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

/// Khronos: [VkExportMetalObjectsInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectsInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExportMetalObjectsInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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

/// Khronos: [VkExtent2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtent2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtent2D {
  pub width: u32,
  pub height: u32,
}

/// Khronos: [VkExtent3D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtent3D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtent3D {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
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

/// Khronos: [VkFaultCallbackInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFaultCallbackInfo.html)
///
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFaultCallbackInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FAULT_CALLBACK_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Optional: true
  pub faultCount: u32,
  /// * Optional: true
  /// * Len: `faultCount`
  pub pFaults: *mut VkFaultData,
  pub pfnFaultCallback: PFN_vkFaultCallbackFunction,
}

/// Khronos: [VkFaultData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFaultData.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFaultData {
  /// * Intended Value: `VK_STRUCTURE_TYPE_FAULT_DATA`
  pub sType: VkStructureType,
  /// * Optional: true
  /// * No Auto-Validity
  pub pNext: *mut c_void,
  pub faultLevel: VkFaultLevel,
  pub faultType: VkFaultType,
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

/// Khronos: [VkGeometryDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryDataNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGeometryDataNV {
  pub triangles: VkGeometryTrianglesNV,
  pub aabbs: VkGeometryAABBNV,
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

/// Khronos: [VkImageSubresource](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresource.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresource {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub arrayLayer: u32,
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

/// Khronos: [VkImageSubresourceLayers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceLayers.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresourceLayers {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
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

/// Khronos: [VkIndirectCommandsStreamNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsStreamNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkIndirectCommandsStreamNV {
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
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

/// Khronos: [VkInputAttachmentAspectReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInputAttachmentAspectReference.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInputAttachmentAspectReference {
  pub subpass: u32,
  pub inputAttachmentIndex: u32,
  pub aspectMask: VkImageAspectFlags,
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

/// Khronos: [VkMicromapTriangleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapTriangleEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMicromapTriangleEXT {
  /// Specified in bytes
  pub dataOffset: u32,
  pub subdivisionLevel: u16,
  pub format: u16,
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

/// Khronos: [VkMultiDrawIndexedInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiDrawIndexedInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiDrawIndexedInfoEXT {
  pub firstIndex: u32,
  pub indexCount: u32,
  pub vertexOffset: i32,
}

/// Khronos: [VkMultiDrawInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiDrawInfoEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMultiDrawInfoEXT {
  pub firstVertex: u32,
  pub vertexCount: u32,
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

/// Khronos: [VkMutableDescriptorTypeListEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeListEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMutableDescriptorTypeListEXT {
  /// * Optional: true
  pub descriptorTypeCount: u32,
  /// * Len: `descriptorTypeCount`
  pub pDescriptorTypes: *const VkDescriptorType,
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

/// Khronos: [VkNativeBufferUsage2ANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkNativeBufferUsage2ANDROID.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkNativeBufferUsage2ANDROID {
  pub consumer: u64,
  pub producer: u64,
}

/// Khronos: [VkOffset2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOffset2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOffset2D {
  pub x: i32,
  pub y: i32,
}

/// Khronos: [VkOffset3D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOffset3D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOffset3D {
  pub x: i32,
  pub y: i32,
  pub z: i32,
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

/// Khronos: [VkPerformanceQueryReservationInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceQueryReservationInfoKHR.html)
///
/// * Struct Extends: [`VkDeviceObjectReservationCreateInfo`]
/// * Duplicates Allowed
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceQueryReservationInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_RESERVATION_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// Maximum number of VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR queries in a query pool
  pub maxPerformanceQueriesPerPool: u32,
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

/// Khronos: [VkPerformanceValueINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueINTEL.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPerformanceValueINTEL {
  pub ty: VkPerformanceValueTypeINTEL,
  /// * Selector: type
  /// * No Auto-Validity
  pub data: VkPerformanceValueDataINTEL,
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

/// Khronos: [VkPhysicalDevicePortabilitySubsetFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetFeaturesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub constantAlphaColorBlendFactors: VkBool32,
  pub events: VkBool32,
  pub imageViewFormatReinterpretation: VkBool32,
  pub imageViewFormatSwizzle: VkBool32,
  pub imageView2DOn3DImage: VkBool32,
  pub multisampleArrayImage: VkBool32,
  pub mutableComparisonSamplers: VkBool32,
  pub pointPolygons: VkBool32,
  pub samplerMipLodBias: VkBool32,
  pub separateStencilMaskRef: VkBool32,
  pub shaderSampleRateInterpolationFunctions: VkBool32,
  pub tessellationIsolines: VkBool32,
  pub tessellationPointMode: VkBool32,
  pub triangleFans: VkBool32,
  pub vertexAttributeAccessBeyondStride: VkBool32,
}

/// Khronos: [VkPhysicalDevicePortabilitySubsetPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePortabilitySubsetPropertiesKHR.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetPropertiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: min,pot
  pub minVertexInputBindingStrideAlignment: u32,
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

/// Khronos: [VkPhysicalDeviceVulkanSC10Features](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkanSC10Features.html)
///
/// * Struct Extends: [`VkPhysicalDeviceFeatures2`]
/// * Struct Extends: [`VkDeviceCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkanSC10Features {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_SC_1_0_FEATURES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  pub shaderAtomicInstructions: VkBool32,
}

/// Khronos: [VkPhysicalDeviceVulkanSC10Properties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkanSC10Properties.html)
///
/// * Struct Extends: [`VkPhysicalDeviceProperties2`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceVulkanSC10Properties {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_SC_1_0_PROPERTIES`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * Limit Type: bitmask
  pub deviceNoDynamicHostAllocations: VkBool32,
  /// * Limit Type: bitmask
  pub deviceDestroyFreesMemory: VkBool32,
  /// * Limit Type: bitmask
  pub commandPoolMultipleCommandBuffersRecording: VkBool32,
  /// * Limit Type: bitmask
  pub commandPoolResetCommandBuffer: VkBool32,
  /// * Limit Type: bitmask
  pub commandBufferSimultaneousUse: VkBool32,
  /// * Limit Type: bitmask
  pub secondaryCommandBufferNullOrImagelessFramebuffer: VkBool32,
  /// * Limit Type: bitmask
  pub recycleDescriptorSetMemory: VkBool32,
  /// * Limit Type: bitmask
  pub recyclePipelineMemory: VkBool32,
  /// * Limit Type: max
  pub maxRenderPassSubpasses: u32,
  /// * Limit Type: max
  pub maxRenderPassDependencies: u32,
  /// * Limit Type: max
  pub maxSubpassInputAttachments: u32,
  /// * Limit Type: max
  pub maxSubpassPreserveAttachments: u32,
  /// * Limit Type: max
  pub maxFramebufferAttachments: u32,
  /// * Limit Type: max
  pub maxDescriptorSetLayoutBindings: u32,
  /// * Limit Type: max
  pub maxQueryFaultCount: u32,
  /// * Limit Type: max
  pub maxCallbackFaultCount: u32,
  /// * Limit Type: max
  pub maxCommandPoolCommandBuffers: u32,
  /// * Limit Type: max
  pub maxCommandBufferSize: VkDeviceSize,
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

/// Khronos: [VkPipelineCacheStageValidationIndexEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheStageValidationIndexEntry.html)
///
/// The fields in this structure are non-normative since structure packing is implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheStageValidationIndexEntry {
  pub codeSize: u64,
  pub codeOffset: u64,
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

/// Khronos: [VkPipelineCreationFeedback](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedback.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCreationFeedback {
  pub flags: VkPipelineCreationFeedbackFlags,
  pub duration: u64,
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

/// Khronos: [VkPipelineOfflineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineOfflineCreateInfo.html)
///
/// * Struct Extends: [`VkGraphicsPipelineCreateInfo`]
/// * Struct Extends: [`VkComputePipelineCreateInfo`]
/// * Struct Extends: [`VkRayTracingPipelineCreateInfoKHR`]
/// * Struct Extends: [`VkRayTracingPipelineCreateInfoNV`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineOfflineCreateInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_OFFLINE_CREATE_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub pipelineIdentifier: [u8; VK_UUID_SIZE],
  pub matchControl: VkPipelineMatchControl,
  pub poolEntrySize: VkDeviceSize,
}

/// Khronos: [VkPipelinePoolSize](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelinePoolSize.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelinePoolSize {
  /// * Intended Value: `VK_STRUCTURE_TYPE_PIPELINE_POOL_SIZE`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub poolEntrySize: VkDeviceSize,
  pub poolEntryCount: u32,
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

/// Khronos: [VkPresentTimeGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentTimeGOOGLE.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPresentTimeGOOGLE {
  /// Application-provided identifier
  pub presentID: u32,
  /// Earliest time an image should be presented
  pub desiredPresentTime: u64,
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

/// Khronos: [VkRect2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRect2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
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

/// Khronos: [VkRefreshCycleDurationGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRefreshCycleDurationGOOGLE.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRefreshCycleDurationGOOGLE {
  /// Number of nanoseconds from the start of one refresh cycle to the next
  pub refreshDuration: u64,
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

/// Khronos: [VkRenderPassCreationFeedbackInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreationFeedbackInfoEXT.html)
///
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreationFeedbackInfoEXT {
  pub postMergeSubpassCount: u32,
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

/// Khronos: [VkSampleLocationEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleLocationEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSampleLocationEXT {
  pub x: c_float,
  pub y: c_float,
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

/// Khronos: [VkSetStateFlagsIndirectCommandNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSetStateFlagsIndirectCommandNV {
  pub data: u32,
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

/// Khronos: [VkShadingRatePaletteNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShadingRatePaletteNV {
  pub shadingRatePaletteEntryCount: u32,
  /// * Len: `shadingRatePaletteEntryCount`
  pub pShadingRatePaletteEntries: *const VkShadingRatePaletteEntryNV,
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

/// Khronos: [VkSparseBufferMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseBufferMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
  pub buffer: VkBuffer,
  pub bindCount: u32,
  /// * Len: `bindCount`
  pub pBinds: *const VkSparseMemoryBind,
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

/// Khronos: [VkSparseImageMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
  pub image: VkImage,
  pub bindCount: u32,
  /// * Len: `bindCount`
  pub pBinds: *const VkSparseImageMemoryBind,
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

/// Khronos: [VkSparseImageOpaqueMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
  pub image: VkImage,
  pub bindCount: u32,
  /// * Len: `bindCount`
  pub pBinds: *const VkSparseMemoryBind,
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

/// Khronos: [VkStridedDeviceAddressRegionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStridedDeviceAddressRegionKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkStridedDeviceAddressRegionKHR {
  /// * Optional: true
  pub deviceAddress: VkDeviceAddress,
  pub stride: VkDeviceSize,
  pub size: VkDeviceSize,
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

/// Khronos: [VkSubpassEndInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassEndInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassEndInfo {
  /// * Intended Value: `VK_STRUCTURE_TYPE_SUBPASS_END_INFO`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
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

/// Khronos: [VkSubpassSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassSampleLocationsEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassSampleLocationsEXT {
  pub subpassIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
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

/// Khronos: [VkTraceRaysIndirectCommandKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommandKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTraceRaysIndirectCommandKHR {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
}

/// Khronos: [VkTransformMatrixKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTransformMatrixKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkTransformMatrixKHR {
  pub matrix: [[c_float; 3]; 4],
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

/// Khronos: [VkVertexInputBindingDivisorDescriptionEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputBindingDivisorDescriptionEXT {
  pub binding: u32,
  pub divisor: u32,
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

/// Khronos: [VkVideoEncodeCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeCapabilitiesKHR.html)
///
/// * Struct Extends: [`VkVideoCapabilitiesKHR`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeCapabilitiesKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_CAPABILITIES_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * No Auto-Validity
  pub flags: VkVideoEncodeCapabilityFlagsKHR,
  pub rateControlModes: VkVideoEncodeRateControlModeFlagsKHR,
  pub rateControlLayerCount: u8,
  pub qualityLevelCount: u8,
  pub inputImageDataFillAlignment: VkExtent2D,
}

/// Khronos: [VkVideoEncodeH264CapabilitiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264CapabilitiesEXT.html)
///
/// * Struct Extends: [`VkVideoCapabilitiesKHR`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264CapabilitiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_CAPABILITIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * No Auto-Validity
  pub flags: VkVideoEncodeH264CapabilityFlagsEXT,
  pub inputModeFlags: VkVideoEncodeH264InputModeFlagsEXT,
  pub outputModeFlags: VkVideoEncodeH264OutputModeFlagsEXT,
  pub maxPPictureL0ReferenceCount: u8,
  pub maxBPictureL0ReferenceCount: u8,
  pub maxL1ReferenceCount: u8,
  pub motionVectorsOverPicBoundariesFlag: VkBool32,
  pub maxBytesPerPicDenom: u32,
  pub maxBitsPerMbDenom: u32,
  pub log2MaxMvLengthHorizontal: u32,
  pub log2MaxMvLengthVertical: u32,
}

/// Khronos: [VkVideoEncodeH264EmitPictureParametersInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264EmitPictureParametersInfoEXT.html)
///
/// * Struct Extends: [`VkVideoEncodeInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264EmitPictureParametersInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub spsId: u8,
  pub emitSpsEnable: VkBool32,
  pub ppsIdEntryCount: u32,
  /// * Len: `ppsIdEntryCount`
  pub ppsIdEntries: *const u8,
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

/// Khronos: [VkVideoEncodeH264RateControlInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlInfoEXT.html)
///
/// * Struct Extends: [`VkVideoCodingControlInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264RateControlInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub gopFrameCount: u32,
  pub idrPeriod: u32,
  pub consecutiveBFrameCount: u32,
  pub rateControlStructure: VkVideoEncodeH264RateControlStructureEXT,
  pub temporalLayerCount: u8,
}

/// Khronos: [VkVideoEncodeH264RateControlLayerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH264RateControlLayerInfoEXT.html)
///
/// * Struct Extends: [`VkVideoCodingControlInfoKHR`]
/// * Struct Extends: [`VkVideoEncodeRateControlLayerInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH264RateControlLayerInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub temporalLayerId: u8,
  pub useInitialRcQp: VkBool32,
  pub initialRcQp: VkVideoEncodeH264QpEXT,
  pub useMinQp: VkBool32,
  pub minQp: VkVideoEncodeH264QpEXT,
  pub useMaxQp: VkBool32,
  pub maxQp: VkVideoEncodeH264QpEXT,
  pub useMaxFrameSize: VkBool32,
  pub maxFrameSize: VkVideoEncodeH264FrameSizeEXT,
}

/// Khronos: [VkVideoEncodeH265CapabilitiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265CapabilitiesEXT.html)
///
/// * Struct Extends: [`VkVideoCapabilitiesKHR`]
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265CapabilitiesEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_CAPABILITIES_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *mut c_void,
  /// * No Auto-Validity
  pub flags: VkVideoEncodeH265CapabilityFlagsEXT,
  pub inputModeFlags: VkVideoEncodeH265InputModeFlagsEXT,
  pub outputModeFlags: VkVideoEncodeH265OutputModeFlagsEXT,
  pub ctbSizes: VkVideoEncodeH265CtbSizeFlagsEXT,
  pub transformBlockSizes: VkVideoEncodeH265TransformBlockSizeFlagsEXT,
  pub maxPPictureL0ReferenceCount: u8,
  pub maxBPictureL0ReferenceCount: u8,
  pub maxL1ReferenceCount: u8,
  pub maxSubLayersCount: u8,
  pub minLog2MinLumaCodingBlockSizeMinus3: u8,
  pub maxLog2MinLumaCodingBlockSizeMinus3: u8,
  pub minLog2MinLumaTransformBlockSizeMinus2: u8,
  pub maxLog2MinLumaTransformBlockSizeMinus2: u8,
  pub minMaxTransformHierarchyDepthInter: u8,
  pub maxMaxTransformHierarchyDepthInter: u8,
  pub minMaxTransformHierarchyDepthIntra: u8,
  pub maxMaxTransformHierarchyDepthIntra: u8,
  pub maxDiffCuQpDeltaDepth: u8,
  pub minMaxNumMergeCand: u8,
  pub maxMaxNumMergeCand: u8,
}

/// Khronos: [VkVideoEncodeH265EmitPictureParametersInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265EmitPictureParametersInfoEXT.html)
///
/// * Struct Extends: [`VkVideoEncodeInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265EmitPictureParametersInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_EMIT_PICTURE_PARAMETERS_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub vpsId: u8,
  pub spsId: u8,
  pub emitVpsEnable: VkBool32,
  pub emitSpsEnable: VkBool32,
  /// * Optional: true
  pub ppsIdEntryCount: u32,
  /// * Len: `ppsIdEntryCount`
  pub ppsIdEntries: *const u8,
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

/// Khronos: [VkVideoEncodeH265RateControlInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265RateControlInfoEXT.html)
///
/// * Struct Extends: [`VkVideoCodingControlInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265RateControlInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub gopFrameCount: u32,
  pub idrPeriod: u32,
  pub consecutiveBFrameCount: u32,
  pub rateControlStructure: VkVideoEncodeH265RateControlStructureEXT,
  pub subLayerCount: u8,
}

/// Khronos: [VkVideoEncodeH265RateControlLayerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeH265RateControlLayerInfoEXT.html)
///
/// * Struct Extends: [`VkVideoCodingControlInfoKHR`]
/// * Struct Extends: [`VkVideoEncodeRateControlLayerInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeH265RateControlLayerInfoEXT {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_EXT`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  pub temporalId: u8,
  pub useInitialRcQp: VkBool32,
  pub initialRcQp: VkVideoEncodeH265QpEXT,
  pub useMinQp: VkBool32,
  pub minQp: VkVideoEncodeH265QpEXT,
  pub useMaxQp: VkBool32,
  pub maxQp: VkVideoEncodeH265QpEXT,
  pub useMaxFrameSize: VkBool32,
  pub maxFrameSize: VkVideoEncodeH265FrameSizeEXT,
}

/// Khronos: [VkVideoEncodeInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeInfoKHR.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkVideoEncodeFlagsKHR,
  pub qualityLevel: u32,
  pub dstBitstreamBuffer: VkBuffer,
  pub dstBitstreamBufferOffset: VkDeviceSize,
  pub dstBitstreamBufferMaxRange: VkDeviceSize,
  pub srcPictureResource: VkVideoPictureResourceInfoKHR,
  /// * Optional: true
  pub pSetupReferenceSlot: *const VkVideoReferenceSlotInfoKHR,
  /// * Optional: true
  pub referenceSlotCount: u32,
  /// * Len: `referenceSlotCount`
  pub pReferenceSlots: *const VkVideoReferenceSlotInfoKHR,
  pub precedingExternallyEncodedBytes: u32,
}

/// Khronos: [VkVideoEncodeRateControlInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlInfoKHR.html)
///
/// * Struct Extends: [`VkVideoCodingControlInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeRateControlInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub flags: VkVideoEncodeRateControlFlagsKHR,
  pub rateControlMode: VkVideoEncodeRateControlModeFlagBitsKHR,
  pub layerCount: u8,
  /// * Len: `layerCount`
  pub pLayerConfigs: *const VkVideoEncodeRateControlLayerInfoKHR,
}

/// Khronos: [VkVideoEncodeRateControlLayerInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeRateControlLayerInfoKHR.html)
///
/// * Struct Extends: [`VkVideoCodingControlInfoKHR`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeRateControlLayerInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR`
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub averageBitrate: u32,
  pub maxBitrate: u32,
  pub frameRateNumerator: u32,
  pub frameRateDenominator: u32,
  pub virtualBufferSizeInMs: u32,
  pub initialVirtualBufferSizeInMs: u32,
}

/// Khronos: [VkVideoEncodeUsageInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoEncodeUsageInfoKHR.html)
///
/// * Struct Extends: [`VkVideoProfileInfoKHR`]
/// * Struct Extends: [`VkQueryPoolCreateInfo`]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVideoEncodeUsageInfoKHR {
  /// * Intended Value: `VK_STRUCTURE_TYPE_VIDEO_ENCODE_USAGE_INFO_KHR`
  pub sType: VkStructureType,
  /// * Optional: true
  pub pNext: *const c_void,
  /// * Optional: true
  pub videoUsageHints: VkVideoEncodeUsageFlagsKHR,
  /// * Optional: true
  pub videoContentHints: VkVideoEncodeContentFlagsKHR,
  /// * Optional: true
  pub tuningMode: VkVideoEncodeTuningModeKHR,
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

/// Khronos: [VkViewportSwizzleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewportSwizzleNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViewportSwizzleNV {
  pub x: VkViewportCoordinateSwizzleNV,
  pub y: VkViewportCoordinateSwizzleNV,
  pub z: VkViewportCoordinateSwizzleNV,
  pub w: VkViewportCoordinateSwizzleNV,
}

/// Khronos: [VkViewportWScalingNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewportWScalingNV.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViewportWScalingNV {
  pub xcoeff: c_float,
  pub ycoeff: c_float,
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

/// Khronos: [VkXYColorEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkXYColorEXT.html)
///
/// Chromaticity coordinate
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkXYColorEXT {
  pub x: c_float,
  pub y: c_float,
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

