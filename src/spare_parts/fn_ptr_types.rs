use super::*;

macro_rules! fn_ptr {
  (
    $pfn:ident;
    $fn_t:ident;
    $the_type:ty;
  ) => {
    pub type $fn_t = $the_type;
    pub type $pfn = Option<$fn_t>;
  };
}

fn_ptr!(
  PFN_vkDebugReportCallbackEXT;
  vkDebugReportCallbackEXT_t;
  unsafe extern "system" fn(flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: uint64_t, location: size_t, messageCode: int32_t, pLayerPrefix: *const  char, pMessage: *const  char, pUserData: *mut void) -> VkBool32;
);

fn_ptr!(
  PFN_vkDebugUtilsMessengerCallbackEXT;
  vkDebugUtilsMessengerCallbackEXT_t;
  unsafe extern "system" fn(messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const  VkDebugUtilsMessengerCallbackDataEXT, pUserData: *mut void) -> VkBool32;
);

fn_ptr!(
  PFN_vkDeviceMemoryReportCallbackEXT;
  vkDeviceMemoryReportCallbackEXT_t;
  unsafe extern "system" fn(pCallbackData: *const VkDeviceMemoryReportCallbackDataEXT, pUserData: *mut void);
);

/// Nullable pointer to [vkAcquireImageANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireImageANDROID.html)
pub type PFN_vkAcquireImageANDROID = Option<vkAcquireImageANDROID_t>;
/// Non-nullable pointer to [vkAcquireImageANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireImageANDROID.html)
pub type vkAcquireImageANDROID_t =
  unsafe extern "system" fn(device: VkDevice, image: VkImage, nativeFenceFd: int, semaphore: VkSemaphore, fence: VkFence) -> VkResult;

/// Nullable pointer to [vkDebugReportMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugReportMessageEXT.html)
pub type PFN_vkDebugReportMessageEXT = Option<vkDebugReportMessageEXT_t>;
/// Non-nullable pointer to [vkDebugReportMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugReportMessageEXT.html)
pub type vkDebugReportMessageEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  flags: VkDebugReportFlagsEXT,
  objectType: VkDebugReportObjectTypeEXT,
  object: uint64_t,
  location: size_t,
  messageCode: int32_t,
  pLayerPrefix: *const char,
  pMessage: *const char,
);

/// Nullable pointer to [vkDestroyAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureKHR.html)
pub type PFN_vkDestroyAccelerationStructureKHR = Option<vkDestroyAccelerationStructureKHR_t>;
/// Non-nullable pointer to [vkDestroyAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureKHR.html)
pub type vkDestroyAccelerationStructureKHR_t =
  unsafe extern "system" fn(device: VkDevice, accelerationStructure: VkAccelerationStructureKHR, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureNV.html)
pub type PFN_vkDestroyAccelerationStructureNV = Option<vkDestroyAccelerationStructureNV_t>;
/// Non-nullable pointer to [vkDestroyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureNV.html)
pub type vkDestroyAccelerationStructureNV_t =
  unsafe extern "system" fn(device: VkDevice, accelerationStructure: VkAccelerationStructureNV, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroyDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugReportCallbackEXT.html)
pub type PFN_vkDestroyDebugReportCallbackEXT = Option<vkDestroyDebugReportCallbackEXT_t>;
/// Non-nullable pointer to [vkDestroyDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugReportCallbackEXT.html)
pub type vkDestroyDebugReportCallbackEXT_t =
  unsafe extern "system" fn(instance: VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroyDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)
pub type PFN_vkDestroyDebugUtilsMessengerEXT = Option<vkDestroyDebugUtilsMessengerEXT_t>;
/// Non-nullable pointer to [vkDestroyDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)
pub type vkDestroyDebugUtilsMessengerEXT_t =
  unsafe extern "system" fn(instance: VkInstance, messenger: VkDebugUtilsMessengerEXT, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroyDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDeferredOperationKHR.html)
pub type PFN_vkDestroyDeferredOperationKHR = Option<vkDestroyDeferredOperationKHR_t>;
/// Non-nullable pointer to [vkDestroyDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDeferredOperationKHR.html)
pub type vkDestroyDeferredOperationKHR_t =
  unsafe extern "system" fn(device: VkDevice, operation: VkDeferredOperationKHR, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroyDescriptorUpdateTemplate](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html)
pub type PFN_vkDestroyDescriptorUpdateTemplate = Option<vkDestroyDescriptorUpdateTemplate_t>;
/// Non-nullable pointer to [vkDestroyDescriptorUpdateTemplate](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html)
pub type vkDestroyDescriptorUpdateTemplate_t =
  unsafe extern "system" fn(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroyIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html)
pub type PFN_vkDestroyIndirectCommandsLayoutNV = Option<vkDestroyIndirectCommandsLayoutNV_t>;
/// Non-nullable pointer to [vkDestroyIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html)
pub type vkDestroyIndirectCommandsLayoutNV_t =
  unsafe extern "system" fn(device: VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNV, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroyPrivateDataSlotEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPrivateDataSlotEXT.html)
pub type PFN_vkDestroyPrivateDataSlotEXT = Option<vkDestroyPrivateDataSlotEXT_t>;
/// Non-nullable pointer to [vkDestroyPrivateDataSlotEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPrivateDataSlotEXT.html)
pub type vkDestroyPrivateDataSlotEXT_t =
  unsafe extern "system" fn(device: VkDevice, privateDataSlot: VkPrivateDataSlotEXT, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroySamplerYcbcrConversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversion.html)
pub type PFN_vkDestroySamplerYcbcrConversion = Option<vkDestroySamplerYcbcrConversion_t>;
/// Non-nullable pointer to [vkDestroySamplerYcbcrConversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversion.html)
pub type vkDestroySamplerYcbcrConversion_t =
  unsafe extern "system" fn(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroySurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySurfaceKHR.html)
pub type PFN_vkDestroySurfaceKHR = Option<vkDestroySurfaceKHR_t>;
/// Non-nullable pointer to [vkDestroySurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySurfaceKHR.html)
pub type vkDestroySurfaceKHR_t = unsafe extern "system" fn(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroySwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySwapchainKHR.html)
pub type PFN_vkDestroySwapchainKHR = Option<vkDestroySwapchainKHR_t>;
/// Non-nullable pointer to [vkDestroySwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySwapchainKHR.html)
pub type vkDestroySwapchainKHR_t = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkDestroyValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyValidationCacheEXT.html)
pub type PFN_vkDestroyValidationCacheEXT = Option<vkDestroyValidationCacheEXT_t>;
/// Non-nullable pointer to [vkDestroyValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyValidationCacheEXT.html)
pub type vkDestroyValidationCacheEXT_t =
  unsafe extern "system" fn(device: VkDevice, validationCache: VkValidationCacheEXT, pAllocator: *const VkAllocationCallbacks);

/// Nullable pointer to [vkGetAccelerationStructureBuildSizesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html)
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = Option<vkGetAccelerationStructureBuildSizesKHR_t>;
/// Non-nullable pointer to [vkGetAccelerationStructureBuildSizesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html)
pub type vkGetAccelerationStructureBuildSizesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  buildType: VkAccelerationStructureBuildTypeKHR,
  pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
  pMaxPrimitiveCounts: *const uint32_t,
  pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
);

/// Nullable pointer to [vkGetAccelerationStructureDeviceAddressKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html)
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = Option<vkGetAccelerationStructureDeviceAddressKHR_t>;
/// Non-nullable pointer to [vkGetAccelerationStructureDeviceAddressKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html)
pub type vkGetAccelerationStructureDeviceAddressKHR_t =
  unsafe extern "system" fn(device: VkDevice, pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR) -> VkDeviceAddress;

/// Nullable pointer to [vkGetAccelerationStructureMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html)
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = Option<vkGetAccelerationStructureMemoryRequirementsNV_t>;
/// Non-nullable pointer to [vkGetAccelerationStructureMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html)
pub type vkGetAccelerationStructureMemoryRequirementsNV_t = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV,
  pMemoryRequirements: *mut VkMemoryRequirements2KHR,
);

/// Nullable pointer to [vkGetBufferDeviceAddress](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddress.html)
pub type PFN_vkGetBufferDeviceAddress = Option<vkGetBufferDeviceAddress_t>;
/// Non-nullable pointer to [vkGetBufferDeviceAddress](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddress.html)
pub type vkGetBufferDeviceAddress_t = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;

/// Nullable pointer to [vkGetBufferMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2.html)
pub type PFN_vkGetBufferMemoryRequirements2 = Option<vkGetBufferMemoryRequirements2_t>;
/// Non-nullable pointer to [vkGetBufferMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2.html)
pub type vkGetBufferMemoryRequirements2_t =
  unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

/// Nullable pointer to [vkGetBufferOpaqueCaptureAddress](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html)
pub type PFN_vkGetBufferOpaqueCaptureAddress = Option<vkGetBufferOpaqueCaptureAddress_t>;
/// Non-nullable pointer to [vkGetBufferOpaqueCaptureAddress](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html)
pub type vkGetBufferOpaqueCaptureAddress_t = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> uint64_t;

/// Nullable pointer to [vkGetDeferredOperationMaxConcurrencyKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html)
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR = Option<vkGetDeferredOperationMaxConcurrencyKHR_t>;
/// Non-nullable pointer to [vkGetDeferredOperationMaxConcurrencyKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html)
pub type vkGetDeferredOperationMaxConcurrencyKHR_t = unsafe extern "system" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> uint32_t;

/// Nullable pointer to [vkGetDescriptorSetLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupport.html)
pub type PFN_vkGetDescriptorSetLayoutSupport = Option<vkGetDescriptorSetLayoutSupport_t>;
/// Non-nullable pointer to [vkGetDescriptorSetLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupport.html)
pub type vkGetDescriptorSetLayoutSupport_t =
  unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);

/// Nullable pointer to [vkGetDeviceAccelerationStructureCompatibilityKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html)
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = Option<vkGetDeviceAccelerationStructureCompatibilityKHR_t>;
/// Non-nullable pointer to [vkGetDeviceAccelerationStructureCompatibilityKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html)
pub type vkGetDeviceAccelerationStructureCompatibilityKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pVersionInfo: *const VkAccelerationStructureVersionInfoKHR,
  pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
);

/// Nullable pointer to [vkGetDeviceGroupPeerMemoryFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html)
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = Option<vkGetDeviceGroupPeerMemoryFeatures_t>;
/// Non-nullable pointer to [vkGetDeviceGroupPeerMemoryFeatures](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html)
pub type vkGetDeviceGroupPeerMemoryFeatures_t = unsafe extern "system" fn(
  device: VkDevice,
  heapIndex: uint32_t,
  localDeviceIndex: uint32_t,
  remoteDeviceIndex: uint32_t,
  pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags,
);

/// Nullable pointer to [vkGetDeviceMemoryOpaqueCaptureAddress](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html)
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = Option<vkGetDeviceMemoryOpaqueCaptureAddress_t>;
/// Non-nullable pointer to [vkGetDeviceMemoryOpaqueCaptureAddress](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html)
pub type vkGetDeviceMemoryOpaqueCaptureAddress_t =
  unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo) -> uint64_t;

/// Nullable pointer to [vkGetDeviceQueue2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue2.html)
pub type PFN_vkGetDeviceQueue2 = Option<vkGetDeviceQueue2_t>;
/// Non-nullable pointer to [vkGetDeviceQueue2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue2.html)
pub type vkGetDeviceQueue2_t = unsafe extern "system" fn(device: VkDevice, pQueueInfo: *const VkDeviceQueueInfo2, pQueue: *mut VkQueue);

/// Nullable pointer to [vkGetGeneratedCommandsMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html)
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = Option<vkGetGeneratedCommandsMemoryRequirementsNV_t>;
/// Non-nullable pointer to [vkGetGeneratedCommandsMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html)
pub type vkGetGeneratedCommandsMemoryRequirementsNV_t = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV,
  pMemoryRequirements: *mut VkMemoryRequirements2,
);

/// Nullable pointer to [vkGetImageMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2.html)
pub type PFN_vkGetImageMemoryRequirements2 = Option<vkGetImageMemoryRequirements2_t>;
/// Non-nullable pointer to [vkGetImageMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2.html)
pub type vkGetImageMemoryRequirements2_t =
  unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

/// Nullable pointer to [vkGetImageSparseMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2.html)
pub type PFN_vkGetImageSparseMemoryRequirements2 = Option<vkGetImageSparseMemoryRequirements2_t>;
/// Non-nullable pointer to [vkGetImageSparseMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2.html)
pub type vkGetImageSparseMemoryRequirements2_t = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkImageSparseMemoryRequirementsInfo2,
  pSparseMemoryRequirementCount: *mut uint32_t,
  pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);

/// Nullable pointer to [vkGetImageViewHandleNVX](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewHandleNVX.html)
pub type PFN_vkGetImageViewHandleNVX = Option<vkGetImageViewHandleNVX_t>;
/// Non-nullable pointer to [vkGetImageViewHandleNVX](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewHandleNVX.html)
pub type vkGetImageViewHandleNVX_t = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageViewHandleInfoNVX) -> uint32_t;

/// Nullable pointer to [vkGetPhysicalDeviceDirectFBPresentationSupportEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT = Option<vkGetPhysicalDeviceDirectFBPresentationSupportEXT_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceDirectFBPresentationSupportEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)
pub type vkGetPhysicalDeviceDirectFBPresentationSupportEXT_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: uint32_t, dfb: *mut IDirectFB) -> VkBool32;

/// Nullable pointer to [vkGetPhysicalDeviceExternalBufferProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html)
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = Option<vkGetPhysicalDeviceExternalBufferProperties_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceExternalBufferProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html)
pub type vkGetPhysicalDeviceExternalBufferProperties_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
  pExternalBufferProperties: *mut VkExternalBufferProperties,
);

/// Nullable pointer to [vkGetPhysicalDeviceExternalFenceProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html)
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = Option<vkGetPhysicalDeviceExternalFenceProperties_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceExternalFenceProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html)
pub type vkGetPhysicalDeviceExternalFenceProperties_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
  pExternalFenceProperties: *mut VkExternalFenceProperties,
);

/// Nullable pointer to [vkGetPhysicalDeviceExternalSemaphoreProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = Option<vkGetPhysicalDeviceExternalSemaphoreProperties_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceExternalSemaphoreProperties](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
pub type vkGetPhysicalDeviceExternalSemaphoreProperties_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
  pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
);

/// Nullable pointer to [vkGetPhysicalDeviceFeatures2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2.html)
pub type PFN_vkGetPhysicalDeviceFeatures2 = Option<vkGetPhysicalDeviceFeatures2_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceFeatures2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2.html)
pub type vkGetPhysicalDeviceFeatures2_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);

/// Nullable pointer to [vkGetPhysicalDeviceFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html)
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = Option<vkGetPhysicalDeviceFormatProperties2_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html)
pub type vkGetPhysicalDeviceFormatProperties2_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);

/// Nullable pointer to [vkGetPhysicalDeviceMemoryProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = Option<vkGetPhysicalDeviceMemoryProperties2_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceMemoryProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)
pub type vkGetPhysicalDeviceMemoryProperties2_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);

/// Nullable pointer to [vkGetPhysicalDeviceMultisamplePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = Option<vkGetPhysicalDeviceMultisamplePropertiesEXT_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceMultisamplePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)
pub type vkGetPhysicalDeviceMultisamplePropertiesEXT_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  samples: VkSampleCountFlagBits,
  pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
);

/// Nullable pointer to [vkGetPhysicalDeviceProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2.html)
pub type PFN_vkGetPhysicalDeviceProperties2 = Option<vkGetPhysicalDeviceProperties2_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2.html)
pub type vkGetPhysicalDeviceProperties2_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);

/// Nullable pointer to [vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = Option<vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)
pub type vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR,
  pNumPasses: *mut uint32_t,
);

/// Nullable pointer to [vkGetPhysicalDeviceQueueFamilyProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = Option<vkGetPhysicalDeviceQueueFamilyProperties2_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceQueueFamilyProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)
pub type vkGetPhysicalDeviceQueueFamilyProperties2_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pQueueFamilyPropertyCount: *mut uint32_t,
  pQueueFamilyProperties: *mut VkQueueFamilyProperties2,
);

/// Nullable pointer to [vkGetPhysicalDeviceSparseImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = Option<vkGetPhysicalDeviceSparseImageFormatProperties2_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSparseImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
pub type vkGetPhysicalDeviceSparseImageFormatProperties2_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
  pPropertyCount: *mut uint32_t,
  pProperties: *mut VkSparseImageFormatProperties2,
);

/// Nullable pointer to [vkGetPhysicalDeviceWaylandPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = Option<vkGetPhysicalDeviceWaylandPresentationSupportKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceWaylandPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
pub type vkGetPhysicalDeviceWaylandPresentationSupportKHR_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: uint32_t, display: *mut wl_display) -> VkBool32;

/// Nullable pointer to [vkGetPhysicalDeviceWin32PresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = Option<vkGetPhysicalDeviceWin32PresentationSupportKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceWin32PresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
pub type vkGetPhysicalDeviceWin32PresentationSupportKHR_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: uint32_t) -> VkBool32;

/// Nullable pointer to [vkGetPhysicalDeviceXcbPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = Option<vkGetPhysicalDeviceXcbPresentationSupportKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceXcbPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
pub type vkGetPhysicalDeviceXcbPresentationSupportKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: uint32_t,
  connection: *mut xcb_connection_t,
  visual_id: xcb_visualid_t,
) -> VkBool32;

/// Nullable pointer to [vkGetPhysicalDeviceXlibPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = Option<vkGetPhysicalDeviceXlibPresentationSupportKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceXlibPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
pub type vkGetPhysicalDeviceXlibPresentationSupportKHR_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: uint32_t, dpy: *mut Display, visualID: VisualID) -> VkBool32;

/// Nullable pointer to [vkGetPrivateDataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPrivateDataEXT.html)
pub type PFN_vkGetPrivateDataEXT = Option<vkGetPrivateDataEXT_t>;
/// Non-nullable pointer to [vkGetPrivateDataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPrivateDataEXT.html)
pub type vkGetPrivateDataEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  objectType: VkObjectType,
  objectHandle: uint64_t,
  privateDataSlot: VkPrivateDataSlotEXT,
  pData: *mut uint64_t,
);

/// Nullable pointer to [vkGetQueueCheckpointDataNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueueCheckpointDataNV.html)
pub type PFN_vkGetQueueCheckpointDataNV = Option<vkGetQueueCheckpointDataNV_t>;
/// Non-nullable pointer to [vkGetQueueCheckpointDataNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueueCheckpointDataNV.html)
pub type vkGetQueueCheckpointDataNV_t =
  unsafe extern "system" fn(queue: VkQueue, pCheckpointDataCount: *mut uint32_t, pCheckpointData: *mut VkCheckpointDataNV);

/// Nullable pointer to [vkGetRayTracingShaderGroupStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html)
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = Option<vkGetRayTracingShaderGroupStackSizeKHR_t>;
/// Non-nullable pointer to [vkGetRayTracingShaderGroupStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html)
pub type vkGetRayTracingShaderGroupStackSizeKHR_t =
  unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, group: uint32_t, groupShader: VkShaderGroupShaderKHR) -> VkDeviceSize;

/// Nullable pointer to [vkGetSwapchainGrallocUsage2ANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainGrallocUsage2ANDROID.html)
pub type PFN_vkGetSwapchainGrallocUsage2ANDROID = Option<vkGetSwapchainGrallocUsage2ANDROID_t>;
/// Non-nullable pointer to [vkGetSwapchainGrallocUsage2ANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainGrallocUsage2ANDROID.html)
pub type vkGetSwapchainGrallocUsage2ANDROID_t = unsafe extern "system" fn(
  device: VkDevice,
  format: VkFormat,
  imageUsage: VkImageUsageFlags,
  swapchainImageUsage: VkSwapchainImageUsageFlagsANDROID,
  grallocConsumerUsage: *mut uint64_t,
  grallocProducerUsage: *mut uint64_t,
) -> VkResult;

/// Nullable pointer to [vkGetSwapchainGrallocUsageANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainGrallocUsageANDROID.html)
pub type PFN_vkGetSwapchainGrallocUsageANDROID = Option<vkGetSwapchainGrallocUsageANDROID_t>;
/// Non-nullable pointer to [vkGetSwapchainGrallocUsageANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainGrallocUsageANDROID.html)
pub type vkGetSwapchainGrallocUsageANDROID_t =
  unsafe extern "system" fn(device: VkDevice, format: VkFormat, imageUsage: VkImageUsageFlags, grallocUsage: *mut int) -> VkResult;

/// Nullable pointer to [vkQueueBeginDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html)
pub type PFN_vkQueueBeginDebugUtilsLabelEXT = Option<vkQueueBeginDebugUtilsLabelEXT_t>;
/// Non-nullable pointer to [vkQueueBeginDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html)
pub type vkQueueBeginDebugUtilsLabelEXT_t = unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);

/// Nullable pointer to [vkQueueEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html)
pub type PFN_vkQueueEndDebugUtilsLabelEXT = Option<vkQueueEndDebugUtilsLabelEXT_t>;
/// Non-nullable pointer to [vkQueueEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html)
pub type vkQueueEndDebugUtilsLabelEXT_t = unsafe extern "system" fn(queue: VkQueue);

/// Nullable pointer to [vkQueueInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html)
pub type PFN_vkQueueInsertDebugUtilsLabelEXT = Option<vkQueueInsertDebugUtilsLabelEXT_t>;
/// Non-nullable pointer to [vkQueueInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html)
pub type vkQueueInsertDebugUtilsLabelEXT_t = unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);

/// Nullable pointer to [vkQueueSignalReleaseImageANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSignalReleaseImageANDROID.html)
pub type PFN_vkQueueSignalReleaseImageANDROID = Option<vkQueueSignalReleaseImageANDROID_t>;
/// Non-nullable pointer to [vkQueueSignalReleaseImageANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSignalReleaseImageANDROID.html)
pub type vkQueueSignalReleaseImageANDROID_t = unsafe extern "system" fn(
  queue: VkQueue,
  waitSemaphoreCount: uint32_t,
  pWaitSemaphores: *const VkSemaphore,
  image: VkImage,
  pNativeFenceFd: *mut int,
) -> VkResult;

/// Nullable pointer to [vkReleaseProfilingLockKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseProfilingLockKHR.html)
pub type PFN_vkReleaseProfilingLockKHR = Option<vkReleaseProfilingLockKHR_t>;
/// Non-nullable pointer to [vkReleaseProfilingLockKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseProfilingLockKHR.html)
pub type vkReleaseProfilingLockKHR_t = unsafe extern "system" fn(device: VkDevice);

/// Nullable pointer to [vkResetQueryPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPool.html)
pub type PFN_vkResetQueryPool = Option<vkResetQueryPool_t>;
/// Non-nullable pointer to [vkResetQueryPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPool.html)
pub type vkResetQueryPool_t = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: uint32_t, queryCount: uint32_t);

/// Nullable pointer to [vkSetHdrMetadataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetHdrMetadataEXT.html)
pub type PFN_vkSetHdrMetadataEXT = Option<vkSetHdrMetadataEXT_t>;
/// Non-nullable pointer to [vkSetHdrMetadataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetHdrMetadataEXT.html)
pub type vkSetHdrMetadataEXT_t =
  unsafe extern "system" fn(device: VkDevice, swapchainCount: uint32_t, pSwapchains: *const VkSwapchainKHR, pMetadata: *const VkHdrMetadataEXT);

/// Nullable pointer to [vkSetLocalDimmingAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetLocalDimmingAMD.html)
pub type PFN_vkSetLocalDimmingAMD = Option<vkSetLocalDimmingAMD_t>;
/// Non-nullable pointer to [vkSetLocalDimmingAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetLocalDimmingAMD.html)
pub type vkSetLocalDimmingAMD_t = unsafe extern "system" fn(device: VkDevice, swapChain: VkSwapchainKHR, localDimmingEnable: VkBool32);

/// Nullable pointer to [vkSubmitDebugUtilsMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html)
pub type PFN_vkSubmitDebugUtilsMessageEXT = Option<vkSubmitDebugUtilsMessageEXT_t>;
/// Non-nullable pointer to [vkSubmitDebugUtilsMessageEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html)
pub type vkSubmitDebugUtilsMessageEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
  messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
  pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
);

/// Nullable pointer to [vkTrimCommandPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPool.html)
pub type PFN_vkTrimCommandPool = Option<vkTrimCommandPool_t>;
/// Non-nullable pointer to [vkTrimCommandPool](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPool.html)
pub type vkTrimCommandPool_t = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);

/// Nullable pointer to [vkUninitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUninitializePerformanceApiINTEL.html)
pub type PFN_vkUninitializePerformanceApiINTEL = Option<vkUninitializePerformanceApiINTEL_t>;
/// Non-nullable pointer to [vkUninitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUninitializePerformanceApiINTEL.html)
pub type vkUninitializePerformanceApiINTEL_t = unsafe extern "system" fn(device: VkDevice);

/// Nullable pointer to [vkUpdateDescriptorSetWithTemplate](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html)
pub type PFN_vkUpdateDescriptorSetWithTemplate = Option<vkUpdateDescriptorSetWithTemplate_t>;
/// Non-nullable pointer to [vkUpdateDescriptorSetWithTemplate](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html)
pub type vkUpdateDescriptorSetWithTemplate_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptorSet: VkDescriptorSet,
  descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
  pData: *const c_void,
);

/// Nullable pointer to [vkCmdBuildAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureNV.html)
pub type PFN_vkCmdBuildAccelerationStructureNV = Option<vkCmdBuildAccelerationStructureNV_t>;
/// Non-nullable pointer to [vkCmdBuildAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureNV.html)
pub type vkCmdBuildAccelerationStructureNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInfo: *const VkAccelerationStructureInfoNV,
  instanceData: VkBuffer,
  instanceOffset: VkDeviceSize,
  update: VkBool32,
  dst: VkAccelerationStructureNV,
  src: VkAccelerationStructureNV,
  scratch: VkBuffer,
  scratchOffset: VkDeviceSize,
);

/// Nullable pointer to [vkCmdBuildAccelerationStructuresIndirectKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html)
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = Option<vkCmdBuildAccelerationStructuresIndirectKHR_t>;
/// Non-nullable pointer to [vkCmdBuildAccelerationStructuresIndirectKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html)
pub type vkCmdBuildAccelerationStructuresIndirectKHR_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  infoCount: uint32_t,
  pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
  pIndirectDeviceAddresses: *const VkDeviceAddress,
  pIndirectStrides: *const uint32_t,
  ppMaxPrimitiveCounts: *mut uint32_t,
);

/// Nullable pointer to [vkCmdBuildAccelerationStructuresKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html)
pub type PFN_vkCmdBuildAccelerationStructuresKHR = Option<vkCmdBuildAccelerationStructuresKHR_t>;
/// Non-nullable pointer to [vkCmdBuildAccelerationStructuresKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html)
pub type vkCmdBuildAccelerationStructuresKHR_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  infoCount: uint32_t,
  pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
  ppBuildRangeInfos: *mut VkAccelerationStructureBuildRangeInfoKHR,
);

/// Nullable pointer to [vkCmdCopyAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html)
pub type PFN_vkCmdCopyAccelerationStructureKHR = Option<vkCmdCopyAccelerationStructureKHR_t>;
/// Non-nullable pointer to [vkCmdCopyAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html)
pub type vkCmdCopyAccelerationStructureKHR_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyAccelerationStructureInfoKHR);

/// Nullable pointer to [vkCmdCopyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureNV.html)
pub type PFN_vkCmdCopyAccelerationStructureNV = Option<vkCmdCopyAccelerationStructureNV_t>;
/// Non-nullable pointer to [vkCmdCopyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureNV.html)
pub type vkCmdCopyAccelerationStructureNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  dst: VkAccelerationStructureNV,
  src: VkAccelerationStructureNV,
  mode: VkCopyAccelerationStructureModeKHR,
);

/// Nullable pointer to [vkCmdCopyAccelerationStructureToMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html)
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = Option<vkCmdCopyAccelerationStructureToMemoryKHR_t>;
/// Non-nullable pointer to [vkCmdCopyAccelerationStructureToMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html)
pub type vkCmdCopyAccelerationStructureToMemoryKHR_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR);

/// Nullable pointer to [vkCmdCopyMemoryToAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html)
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = Option<vkCmdCopyMemoryToAccelerationStructureKHR_t>;
/// Non-nullable pointer to [vkCmdCopyMemoryToAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html)
pub type vkCmdCopyMemoryToAccelerationStructureKHR_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR);

/// Nullable pointer to [vkCmdDispatchBase](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBase.html)
pub type PFN_vkCmdDispatchBase = Option<vkCmdDispatchBase_t>;
/// Non-nullable pointer to [vkCmdDispatchBase](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBase.html)
pub type vkCmdDispatchBase_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  baseGroupX: uint32_t,
  baseGroupY: uint32_t,
  baseGroupZ: uint32_t,
  groupCountX: uint32_t,
  groupCountY: uint32_t,
  groupCountZ: uint32_t,
);

/// Nullable pointer to [vkCmdSetRayTracingPipelineStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html)
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR = Option<vkCmdSetRayTracingPipelineStackSizeKHR_t>;
/// Non-nullable pointer to [vkCmdSetRayTracingPipelineStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html)
pub type vkCmdSetRayTracingPipelineStackSizeKHR_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStackSize: uint32_t);

/// Nullable pointer to [vkCmdTraceRaysIndirectKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysIndirectKHR.html)
pub type PFN_vkCmdTraceRaysIndirectKHR = Option<vkCmdTraceRaysIndirectKHR_t>;
/// Non-nullable pointer to [vkCmdTraceRaysIndirectKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysIndirectKHR.html)
pub type vkCmdTraceRaysIndirectKHR_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
  pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
  pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
  pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
  indirectDeviceAddress: VkDeviceAddress,
);

/// Nullable pointer to [vkCmdTraceRaysKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysKHR.html)
pub type PFN_vkCmdTraceRaysKHR = Option<vkCmdTraceRaysKHR_t>;
/// Non-nullable pointer to [vkCmdTraceRaysKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysKHR.html)
pub type vkCmdTraceRaysKHR_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
  pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
  pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
  pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
  width: uint32_t,
  height: uint32_t,
  depth: uint32_t,
);

/// Nullable pointer to [vkCmdTraceRaysNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysNV.html)
pub type PFN_vkCmdTraceRaysNV = Option<vkCmdTraceRaysNV_t>;
/// Non-nullable pointer to [vkCmdTraceRaysNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysNV.html)
pub type vkCmdTraceRaysNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  raygenShaderBindingTableBuffer: VkBuffer,
  raygenShaderBindingOffset: VkDeviceSize,
  missShaderBindingTableBuffer: VkBuffer,
  missShaderBindingOffset: VkDeviceSize,
  missShaderBindingStride: VkDeviceSize,
  hitShaderBindingTableBuffer: VkBuffer,
  hitShaderBindingOffset: VkDeviceSize,
  hitShaderBindingStride: VkDeviceSize,
  callableShaderBindingTableBuffer: VkBuffer,
  callableShaderBindingOffset: VkDeviceSize,
  callableShaderBindingStride: VkDeviceSize,
  width: uint32_t,
  height: uint32_t,
  depth: uint32_t,
);

/// Nullable pointer to [vkCmdWriteAccelerationStructuresPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html)
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = Option<vkCmdWriteAccelerationStructuresPropertiesKHR_t>;
/// Non-nullable pointer to [vkCmdWriteAccelerationStructuresPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html)
pub type vkCmdWriteAccelerationStructuresPropertiesKHR_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  accelerationStructureCount: uint32_t,
  pAccelerationStructures: *const VkAccelerationStructureKHR,
  queryType: VkQueryType,
  queryPool: VkQueryPool,
  firstQuery: uint32_t,
);

/// Nullable pointer to [vkCmdWriteAccelerationStructuresPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html)
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = Option<vkCmdWriteAccelerationStructuresPropertiesNV_t>;
/// Non-nullable pointer to [vkCmdWriteAccelerationStructuresPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html)
pub type vkCmdWriteAccelerationStructuresPropertiesNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  accelerationStructureCount: uint32_t,
  pAccelerationStructures: *const VkAccelerationStructureNV,
  queryType: VkQueryType,
  queryPool: VkQueryPool,
  firstQuery: uint32_t,
);
/// Nullable pointer to [vkCmdBindShadingRateImageNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindShadingRateImageNV.html)
pub type PFN_vkCmdBindShadingRateImageNV = Option<vkCmdBindShadingRateImageNV_t>;
/// Non-nullable pointer to [vkCmdBindShadingRateImageNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindShadingRateImageNV.html)
pub type vkCmdBindShadingRateImageNV_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, imageView: VkImageView, imageLayout: VkImageLayout);

/// Nullable pointer to [vkCmdBindTransformFeedbackBuffersEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html)
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = Option<vkCmdBindTransformFeedbackBuffersEXT_t>;
/// Non-nullable pointer to [vkCmdBindTransformFeedbackBuffersEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html)
pub type vkCmdBindTransformFeedbackBuffersEXT_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstBinding: uint32_t,
  bindingCount: uint32_t,
  pBuffers: *const VkBuffer,
  pOffsets: *const VkDeviceSize,
  pSizes: *const VkDeviceSize,
);

/// Nullable pointer to [vkCmdBindVertexBuffers2EXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers2EXT.html)
pub type PFN_vkCmdBindVertexBuffers2EXT = Option<vkCmdBindVertexBuffers2EXT_t>;
/// Non-nullable pointer to [vkCmdBindVertexBuffers2EXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers2EXT.html)
pub type vkCmdBindVertexBuffers2EXT_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstBinding: uint32_t,
  bindingCount: uint32_t,
  pBuffers: *const VkBuffer,
  pOffsets: *const VkDeviceSize,
  pSizes: *const VkDeviceSize,
  pStrides: *const VkDeviceSize,
);

/// Nullable pointer to [vkCmdSetCoarseSampleOrderNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html)
pub type PFN_vkCmdSetCoarseSampleOrderNV = Option<vkCmdSetCoarseSampleOrderNV_t>;
/// Non-nullable pointer to [vkCmdSetCoarseSampleOrderNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html)
pub type vkCmdSetCoarseSampleOrderNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  sampleOrderType: VkCoarseSampleOrderTypeNV,
  customSampleOrderCount: uint32_t,
  pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
);

/// Nullable pointer to [vkCmdSetCullModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCullModeEXT.html)
pub type PFN_vkCmdSetCullModeEXT = Option<vkCmdSetCullModeEXT_t>;
/// Non-nullable pointer to [vkCmdSetCullModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCullModeEXT.html)
pub type vkCmdSetCullModeEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);

/// Nullable pointer to [vkCmdSetDepthBoundsTestEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html)
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = Option<vkCmdSetDepthBoundsTestEnableEXT_t>;
/// Non-nullable pointer to [vkCmdSetDepthBoundsTestEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html)
pub type vkCmdSetDepthBoundsTestEnableEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);

/// Nullable pointer to [vkCmdSetDepthCompareOpEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthCompareOpEXT.html)
pub type PFN_vkCmdSetDepthCompareOpEXT = Option<vkCmdSetDepthCompareOpEXT_t>;
/// Non-nullable pointer to [vkCmdSetDepthCompareOpEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthCompareOpEXT.html)
pub type vkCmdSetDepthCompareOpEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);

/// Nullable pointer to [vkCmdSetDepthTestEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthTestEnableEXT.html)
pub type PFN_vkCmdSetDepthTestEnableEXT = Option<vkCmdSetDepthTestEnableEXT_t>;
/// Non-nullable pointer to [vkCmdSetDepthTestEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthTestEnableEXT.html)
pub type vkCmdSetDepthTestEnableEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);

/// Nullable pointer to [vkCmdSetDepthWriteEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html)
pub type PFN_vkCmdSetDepthWriteEnableEXT = Option<vkCmdSetDepthWriteEnableEXT_t>;
/// Non-nullable pointer to [vkCmdSetDepthWriteEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html)
pub type vkCmdSetDepthWriteEnableEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);

/// Nullable pointer to [vkCmdSetDiscardRectangleEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDiscardRectangleEXT.html)
pub type PFN_vkCmdSetDiscardRectangleEXT = Option<vkCmdSetDiscardRectangleEXT_t>;
/// Non-nullable pointer to [vkCmdSetDiscardRectangleEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDiscardRectangleEXT.html)
pub type vkCmdSetDiscardRectangleEXT_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstDiscardRectangle: uint32_t,
  discardRectangleCount: uint32_t,
  pDiscardRectangles: *const VkRect2D,
);

/// Nullable pointer to [vkCmdSetExclusiveScissorNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetExclusiveScissorNV.html)
pub type PFN_vkCmdSetExclusiveScissorNV = Option<vkCmdSetExclusiveScissorNV_t>;
/// Non-nullable pointer to [vkCmdSetExclusiveScissorNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetExclusiveScissorNV.html)
pub type vkCmdSetExclusiveScissorNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstExclusiveScissor: uint32_t,
  exclusiveScissorCount: uint32_t,
  pExclusiveScissors: *const VkRect2D,
);

/// Nullable pointer to [vkCmdSetFragmentShadingRateEnumNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html)
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = Option<vkCmdSetFragmentShadingRateEnumNV_t>;
/// Non-nullable pointer to [vkCmdSetFragmentShadingRateEnumNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html)
pub type vkCmdSetFragmentShadingRateEnumNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  shadingRate: VkFragmentShadingRateNV,
  combinerOps: *const [VkFragmentShadingRateCombinerOpKHR; 2],
);

/// Nullable pointer to [vkCmdSetFragmentShadingRateKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html)
pub type PFN_vkCmdSetFragmentShadingRateKHR = Option<vkCmdSetFragmentShadingRateKHR_t>;
/// Non-nullable pointer to [vkCmdSetFragmentShadingRateKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html)
pub type vkCmdSetFragmentShadingRateKHR_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pFragmentSize: *const VkExtent2D,
  combinerOps: *const [VkFragmentShadingRateCombinerOpKHR; 2],
);

/// Nullable pointer to [vkCmdSetFrontFaceEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFrontFaceEXT.html)
pub type PFN_vkCmdSetFrontFaceEXT = Option<vkCmdSetFrontFaceEXT_t>;
/// Non-nullable pointer to [vkCmdSetFrontFaceEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFrontFaceEXT.html)
pub type vkCmdSetFrontFaceEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);

/// Nullable pointer to [vkCmdSetLineStippleEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineStippleEXT.html)
pub type PFN_vkCmdSetLineStippleEXT = Option<vkCmdSetLineStippleEXT_t>;
/// Non-nullable pointer to [vkCmdSetLineStippleEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineStippleEXT.html)
pub type vkCmdSetLineStippleEXT_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineStippleFactor: uint32_t, lineStipplePattern: uint16_t);

/// Nullable pointer to [vkCmdSetPrimitiveTopologyEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html)
pub type PFN_vkCmdSetPrimitiveTopologyEXT = Option<vkCmdSetPrimitiveTopologyEXT_t>;
/// Non-nullable pointer to [vkCmdSetPrimitiveTopologyEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html)
pub type vkCmdSetPrimitiveTopologyEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);

/// Nullable pointer to [vkCmdSetSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetSampleLocationsEXT.html)
pub type PFN_vkCmdSetSampleLocationsEXT = Option<vkCmdSetSampleLocationsEXT_t>;
/// Non-nullable pointer to [vkCmdSetSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetSampleLocationsEXT.html)
pub type vkCmdSetSampleLocationsEXT_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSampleLocationsInfo: *const VkSampleLocationsInfoEXT);

/// Nullable pointer to [vkCmdSetScissorWithCountEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissorWithCountEXT.html)
pub type PFN_vkCmdSetScissorWithCountEXT = Option<vkCmdSetScissorWithCountEXT_t>;
/// Non-nullable pointer to [vkCmdSetScissorWithCountEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissorWithCountEXT.html)
pub type vkCmdSetScissorWithCountEXT_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, scissorCount: uint32_t, pScissors: *const VkRect2D);

/// Nullable pointer to [vkCmdSetStencilOpEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilOpEXT.html)
pub type PFN_vkCmdSetStencilOpEXT = Option<vkCmdSetStencilOpEXT_t>;
/// Non-nullable pointer to [vkCmdSetStencilOpEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilOpEXT.html)
pub type vkCmdSetStencilOpEXT_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  faceMask: VkStencilFaceFlags,
  failOp: VkStencilOp,
  passOp: VkStencilOp,
  depthFailOp: VkStencilOp,
  compareOp: VkCompareOp,
);

/// Nullable pointer to [vkCmdSetStencilTestEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilTestEnableEXT.html)
pub type PFN_vkCmdSetStencilTestEnableEXT = Option<vkCmdSetStencilTestEnableEXT_t>;
/// Non-nullable pointer to [vkCmdSetStencilTestEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilTestEnableEXT.html)
pub type vkCmdSetStencilTestEnableEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);

/// Nullable pointer to [vkCmdSetViewportShadingRatePaletteNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html)
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = Option<vkCmdSetViewportShadingRatePaletteNV_t>;
/// Non-nullable pointer to [vkCmdSetViewportShadingRatePaletteNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html)
pub type vkCmdSetViewportShadingRatePaletteNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstViewport: uint32_t,
  viewportCount: uint32_t,
  pShadingRatePalettes: *const VkShadingRatePaletteNV,
);

/// Nullable pointer to [vkCmdSetViewportWScalingNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWScalingNV.html)
pub type PFN_vkCmdSetViewportWScalingNV = Option<vkCmdSetViewportWScalingNV_t>;
/// Non-nullable pointer to [vkCmdSetViewportWScalingNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWScalingNV.html)
pub type vkCmdSetViewportWScalingNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstViewport: uint32_t,
  viewportCount: uint32_t,
  pViewportWScalings: *const VkViewportWScalingNV,
);

/// Nullable pointer to [vkCmdSetViewportWithCountEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWithCountEXT.html)
pub type PFN_vkCmdSetViewportWithCountEXT = Option<vkCmdSetViewportWithCountEXT_t>;
/// Non-nullable pointer to [vkCmdSetViewportWithCountEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWithCountEXT.html)
pub type vkCmdSetViewportWithCountEXT_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, viewportCount: uint32_t, pViewports: *const VkViewport);

/// Nullable pointer to [vkCmdEndRenderPass2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass2.html)
pub type PFN_vkCmdEndRenderPass2 = Option<vkCmdEndRenderPass2_t>;
/// Non-nullable pointer to [vkCmdEndRenderPass2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass2.html)
pub type vkCmdEndRenderPass2_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);

/// Nullable pointer to [vkCmdNextSubpass2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass2.html)
pub type PFN_vkCmdNextSubpass2 = Option<vkCmdNextSubpass2_t>;
/// Non-nullable pointer to [vkCmdNextSubpass2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass2.html)
pub type vkCmdNextSubpass2_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfo, pSubpassEndInfo: *const VkSubpassEndInfo);

/// Nullable pointer to [vkCmdBeginTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html)
pub type PFN_vkCmdBeginTransformFeedbackEXT = Option<vkCmdBeginTransformFeedbackEXT_t>;
/// Non-nullable pointer to [vkCmdBeginTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html)
pub type vkCmdBeginTransformFeedbackEXT_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstCounterBuffer: uint32_t,
  counterBufferCount: uint32_t,
  pCounterBuffers: *const VkBuffer,
  pCounterBufferOffsets: *const VkDeviceSize,
);

/// Nullable pointer to [vkCmdEndTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndTransformFeedbackEXT.html)
pub type PFN_vkCmdEndTransformFeedbackEXT = Option<vkCmdEndTransformFeedbackEXT_t>;
/// Non-nullable pointer to [vkCmdEndTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndTransformFeedbackEXT.html)
pub type vkCmdEndTransformFeedbackEXT_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstCounterBuffer: uint32_t,
  counterBufferCount: uint32_t,
  pCounterBuffers: *const VkBuffer,
  pCounterBufferOffsets: *const VkDeviceSize,
);

/// Nullable pointer to [vkCmdDrawIndexedIndirectCount](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCount.html)
pub type PFN_vkCmdDrawIndexedIndirectCount = Option<vkCmdDrawIndexedIndirectCount_t>;
/// Non-nullable pointer to [vkCmdDrawIndexedIndirectCount](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCount.html)
pub type vkCmdDrawIndexedIndirectCount_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  countBuffer: VkBuffer,
  countBufferOffset: VkDeviceSize,
  maxDrawCount: uint32_t,
  stride: uint32_t,
);

/// Nullable pointer to [vkCmdDrawIndirectByteCountEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html)
pub type PFN_vkCmdDrawIndirectByteCountEXT = Option<vkCmdDrawIndirectByteCountEXT_t>;
/// Non-nullable pointer to [vkCmdDrawIndirectByteCountEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html)
pub type vkCmdDrawIndirectByteCountEXT_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  instanceCount: uint32_t,
  firstInstance: uint32_t,
  counterBuffer: VkBuffer,
  counterBufferOffset: VkDeviceSize,
  counterOffset: uint32_t,
  vertexStride: uint32_t,
);

/// Nullable pointer to [vkCmdDrawIndirectCount](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCount.html)
pub type PFN_vkCmdDrawIndirectCount = Option<vkCmdDrawIndirectCount_t>;
/// Non-nullable pointer to [vkCmdDrawIndirectCount](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCount.html)
pub type vkCmdDrawIndirectCount_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  countBuffer: VkBuffer,
  countBufferOffset: VkDeviceSize,
  maxDrawCount: uint32_t,
  stride: uint32_t,
);

/// Nullable pointer to [vkCmdDrawMeshTasksIndirectCountNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html)
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = Option<vkCmdDrawMeshTasksIndirectCountNV_t>;
/// Non-nullable pointer to [vkCmdDrawMeshTasksIndirectCountNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html)
pub type vkCmdDrawMeshTasksIndirectCountNV_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  countBuffer: VkBuffer,
  countBufferOffset: VkDeviceSize,
  maxDrawCount: uint32_t,
  stride: uint32_t,
);

/// Nullable pointer to [vkCmdDrawMeshTasksIndirectNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html)
pub type PFN_vkCmdDrawMeshTasksIndirectNV = Option<vkCmdDrawMeshTasksIndirectNV_t>;
/// Non-nullable pointer to [vkCmdDrawMeshTasksIndirectNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html)
pub type vkCmdDrawMeshTasksIndirectNV_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: uint32_t, stride: uint32_t);

/// Nullable pointer to [vkCmdDrawMeshTasksNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksNV.html)
pub type PFN_vkCmdDrawMeshTasksNV = Option<vkCmdDrawMeshTasksNV_t>;
/// Non-nullable pointer to [vkCmdDrawMeshTasksNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMeshTasksNV.html)
pub type vkCmdDrawMeshTasksNV_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, taskCount: uint32_t, firstTask: uint32_t);

/// Nullable pointer to [vkCmdBeginRenderPass2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass2.html)
pub type PFN_vkCmdBeginRenderPass2 = Option<vkCmdBeginRenderPass2_t>;
/// Non-nullable pointer to [vkCmdBeginRenderPass2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass2.html)
pub type vkCmdBeginRenderPass2_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pRenderPassBegin: *const VkRenderPassBeginInfo,
  pSubpassBeginInfo: *const VkSubpassBeginInfo,
);

/// Nullable pointer to [vkCmdBlitImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBlitImage2KHR.html)
pub type PFN_vkCmdBlitImage2KHR = Option<vkCmdBlitImage2KHR_t>;
/// Non-nullable pointer to [vkCmdBlitImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBlitImage2KHR.html)
pub type vkCmdBlitImage2KHR_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2KHR);

/// Nullable pointer to [vkCmdResolveImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResolveImage2KHR.html)
pub type PFN_vkCmdResolveImage2KHR = Option<vkCmdResolveImage2KHR_t>;
/// Non-nullable pointer to [vkCmdResolveImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResolveImage2KHR.html)
pub type vkCmdResolveImage2KHR_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pResolveImageInfo: *const VkResolveImageInfo2KHR);

/// Nullable pointer to [vkCmdBeginConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html)
pub type PFN_vkCmdBeginConditionalRenderingEXT = Option<vkCmdBeginConditionalRenderingEXT_t>;
/// Non-nullable pointer to [vkCmdBeginConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html)
pub type vkCmdBeginConditionalRenderingEXT_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT);

/// Nullable pointer to [vkCmdBeginDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html)
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = Option<vkCmdBeginDebugUtilsLabelEXT_t>;
/// Non-nullable pointer to [vkCmdBeginDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html)
pub type vkCmdBeginDebugUtilsLabelEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);

/// Nullable pointer to [vkCmdBeginQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginQueryIndexedEXT.html)
pub type PFN_vkCmdBeginQueryIndexedEXT = Option<vkCmdBeginQueryIndexedEXT_t>;
/// Non-nullable pointer to [vkCmdBeginQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginQueryIndexedEXT.html)
pub type vkCmdBeginQueryIndexedEXT_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: uint32_t, flags: VkQueryControlFlags, index: uint32_t);

/// Nullable pointer to [vkCmdBindPipelineShaderGroupNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html)
pub type PFN_vkCmdBindPipelineShaderGroupNV = Option<vkCmdBindPipelineShaderGroupNV_t>;
/// Non-nullable pointer to [vkCmdBindPipelineShaderGroupNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html)
pub type vkCmdBindPipelineShaderGroupNV_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline, groupIndex: uint32_t);

/// Nullable pointer to [vkCmdDebugMarkerBeginEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerBeginEXT.html)
pub type PFN_vkCmdDebugMarkerBeginEXT = Option<vkCmdDebugMarkerBeginEXT_t>;
/// Non-nullable pointer to [vkCmdDebugMarkerBeginEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerBeginEXT.html)
pub type vkCmdDebugMarkerBeginEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);

/// Nullable pointer to [vkCmdDebugMarkerEndEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerEndEXT.html)
pub type PFN_vkCmdDebugMarkerEndEXT = Option<vkCmdDebugMarkerEndEXT_t>;
/// Non-nullable pointer to [vkCmdDebugMarkerEndEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerEndEXT.html)
pub type vkCmdDebugMarkerEndEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);

/// Nullable pointer to [vkCmdDebugMarkerInsertEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerInsertEXT.html)
pub type PFN_vkCmdDebugMarkerInsertEXT = Option<vkCmdDebugMarkerInsertEXT_t>;
/// Non-nullable pointer to [vkCmdDebugMarkerInsertEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerInsertEXT.html)
pub type vkCmdDebugMarkerInsertEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);

/// Nullable pointer to [vkCmdEndConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndConditionalRenderingEXT.html)
pub type PFN_vkCmdEndConditionalRenderingEXT = Option<vkCmdEndConditionalRenderingEXT_t>;
/// Non-nullable pointer to [vkCmdEndConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndConditionalRenderingEXT.html)
pub type vkCmdEndConditionalRenderingEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);

/// Nullable pointer to [vkCmdEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html)
pub type PFN_vkCmdEndDebugUtilsLabelEXT = Option<vkCmdEndDebugUtilsLabelEXT_t>;
/// Non-nullable pointer to [vkCmdEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html)
pub type vkCmdEndDebugUtilsLabelEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);

/// Nullable pointer to [vkCmdEndQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndQueryIndexedEXT.html)
pub type PFN_vkCmdEndQueryIndexedEXT = Option<vkCmdEndQueryIndexedEXT_t>;
/// Non-nullable pointer to [vkCmdEndQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndQueryIndexedEXT.html)
pub type vkCmdEndQueryIndexedEXT_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: uint32_t, index: uint32_t);

/// Nullable pointer to [vkCmdInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html)
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = Option<vkCmdInsertDebugUtilsLabelEXT_t>;
/// Non-nullable pointer to [vkCmdInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html)
pub type vkCmdInsertDebugUtilsLabelEXT_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);

/// Nullable pointer to [vkCmdPushDescriptorSetKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetKHR.html)
pub type PFN_vkCmdPushDescriptorSetKHR = Option<vkCmdPushDescriptorSetKHR_t>;
/// Non-nullable pointer to [vkCmdPushDescriptorSetKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetKHR.html)
pub type vkCmdPushDescriptorSetKHR_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pipelineBindPoint: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  set: uint32_t,
  descriptorWriteCount: uint32_t,
  pDescriptorWrites: *const VkWriteDescriptorSet,
);

/// Nullable pointer to [vkCmdPushDescriptorSetWithTemplateKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html)
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = Option<vkCmdPushDescriptorSetWithTemplateKHR_t>;
/// Non-nullable pointer to [vkCmdPushDescriptorSetWithTemplateKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html)
pub type vkCmdPushDescriptorSetWithTemplateKHR_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
  layout: VkPipelineLayout,
  set: uint32_t,
  pData: *const c_void,
);

/// Nullable pointer to [vkCmdExecuteGeneratedCommandsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html)
pub type PFN_vkCmdExecuteGeneratedCommandsNV = Option<vkCmdExecuteGeneratedCommandsNV_t>;
/// Non-nullable pointer to [vkCmdExecuteGeneratedCommandsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html)
pub type vkCmdExecuteGeneratedCommandsNV_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, isPreprocessed: VkBool32, pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV);

/// Nullable pointer to [vkCmdPreprocessGeneratedCommandsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html)
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = Option<vkCmdPreprocessGeneratedCommandsNV_t>;
/// Non-nullable pointer to [vkCmdPreprocessGeneratedCommandsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html)
pub type vkCmdPreprocessGeneratedCommandsNV_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV);

/// Nullable pointer to [vkCmdSetCheckpointNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCheckpointNV.html)
pub type PFN_vkCmdSetCheckpointNV = Option<vkCmdSetCheckpointNV_t>;
/// Non-nullable pointer to [vkCmdSetCheckpointNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCheckpointNV.html)
pub type vkCmdSetCheckpointNV_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCheckpointMarker: *const c_void);

/// Nullable pointer to [vkCmdSetDeviceMask](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMask.html)
pub type PFN_vkCmdSetDeviceMask = Option<vkCmdSetDeviceMask_t>;
/// Non-nullable pointer to [vkCmdSetDeviceMask](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMask.html)
pub type vkCmdSetDeviceMask_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: uint32_t);

/// Nullable pointer to [vkCmdWriteBufferMarkerAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteBufferMarkerAMD.html)
pub type PFN_vkCmdWriteBufferMarkerAMD = Option<vkCmdWriteBufferMarkerAMD_t>;
/// Non-nullable pointer to [vkCmdWriteBufferMarkerAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteBufferMarkerAMD.html)
pub type vkCmdWriteBufferMarkerAMD_t = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pipelineStage: VkPipelineStageFlagBits,
  dstBuffer: VkBuffer,
  dstOffset: VkDeviceSize,
  marker: uint32_t,
);

/// Nullable pointer to [vkCmdCopyBuffer2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBuffer2KHR.html)
pub type PFN_vkCmdCopyBuffer2KHR = Option<vkCmdCopyBuffer2KHR_t>;
/// Non-nullable pointer to [vkCmdCopyBuffer2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBuffer2KHR.html)
pub type vkCmdCopyBuffer2KHR_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2KHR);

/// Nullable pointer to [vkCmdCopyBufferToImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage2KHR.html)
pub type PFN_vkCmdCopyBufferToImage2KHR = Option<vkCmdCopyBufferToImage2KHR_t>;
/// Non-nullable pointer to [vkCmdCopyBufferToImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage2KHR.html)
pub type vkCmdCopyBufferToImage2KHR_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2KHR);

/// Nullable pointer to [vkCmdCopyImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImage2KHR.html)
pub type PFN_vkCmdCopyImage2KHR = Option<vkCmdCopyImage2KHR_t>;
/// Non-nullable pointer to [vkCmdCopyImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImage2KHR.html)
pub type vkCmdCopyImage2KHR_t = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2KHR);

/// Nullable pointer to [vkCmdCopyImageToBuffer2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html)
pub type PFN_vkCmdCopyImageToBuffer2KHR = Option<vkCmdCopyImageToBuffer2KHR_t>;
/// Non-nullable pointer to [vkCmdCopyImageToBuffer2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html)
pub type vkCmdCopyImageToBuffer2KHR_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2KHR);

/// Nullable pointer to [vkReleaseDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseDisplayEXT.html)
pub type PFN_vkReleaseDisplayEXT = Option<vkReleaseDisplayEXT_t>;
/// Non-nullable pointer to [vkReleaseDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseDisplayEXT.html)
pub type vkReleaseDisplayEXT_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;

/// Nullable pointer to [vkCreateAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureNV.html)
pub type PFN_vkCreateAccelerationStructureNV = Option<vkCreateAccelerationStructureNV_t>;
/// Non-nullable pointer to [vkCreateAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureNV.html)
pub type vkCreateAccelerationStructureNV_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
  pAllocator: *const VkAllocationCallbacks,
  pAccelerationStructure: *mut VkAccelerationStructureNV,
) -> VkResult;

/// Nullable pointer to [vkCreateDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugReportCallbackEXT.html)
pub type PFN_vkCreateDebugReportCallbackEXT = Option<vkCreateDebugReportCallbackEXT_t>;
/// Non-nullable pointer to [vkCreateDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugReportCallbackEXT.html)
pub type vkCreateDebugReportCallbackEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
  pAllocator: *const VkAllocationCallbacks,
  pCallback: *mut VkDebugReportCallbackEXT,
) -> VkResult;

/// Nullable pointer to [vkCreateDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)
pub type PFN_vkCreateDebugUtilsMessengerEXT = Option<vkCreateDebugUtilsMessengerEXT_t>;
/// Non-nullable pointer to [vkCreateDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)
pub type vkCreateDebugUtilsMessengerEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
  pAllocator: *const VkAllocationCallbacks,
  pMessenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult;

/// Nullable pointer to [vkCreateDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDeferredOperationKHR.html)
pub type PFN_vkCreateDeferredOperationKHR = Option<vkCreateDeferredOperationKHR_t>;
/// Non-nullable pointer to [vkCreateDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDeferredOperationKHR.html)
pub type vkCreateDeferredOperationKHR_t =
  unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks, pDeferredOperation: *mut VkDeferredOperationKHR) -> VkResult;

/// Nullable pointer to [vkCreatePrivateDataSlotEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePrivateDataSlotEXT.html)
pub type PFN_vkCreatePrivateDataSlotEXT = Option<vkCreatePrivateDataSlotEXT_t>;
/// Non-nullable pointer to [vkCreatePrivateDataSlotEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePrivateDataSlotEXT.html)
pub type vkCreatePrivateDataSlotEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkPrivateDataSlotCreateInfoEXT,
  pAllocator: *const VkAllocationCallbacks,
  pPrivateDataSlot: *mut VkPrivateDataSlotEXT,
) -> VkResult;

/// Nullable pointer to [vkCreateValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateValidationCacheEXT.html)
pub type PFN_vkCreateValidationCacheEXT = Option<vkCreateValidationCacheEXT_t>;
/// Non-nullable pointer to [vkCreateValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateValidationCacheEXT.html)
pub type vkCreateValidationCacheEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkValidationCacheCreateInfoEXT,
  pAllocator: *const VkAllocationCallbacks,
  pValidationCache: *mut VkValidationCacheEXT,
) -> VkResult;

/// Nullable pointer to [vkDisplayPowerControlEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDisplayPowerControlEXT.html)
pub type PFN_vkDisplayPowerControlEXT = Option<vkDisplayPowerControlEXT_t>;
/// Non-nullable pointer to [vkDisplayPowerControlEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDisplayPowerControlEXT.html)
pub type vkDisplayPowerControlEXT_t =
  unsafe extern "system" fn(device: VkDevice, display: VkDisplayKHR, pDisplayPowerInfo: *const VkDisplayPowerInfoEXT) -> VkResult;

/// Nullable pointer to [vkEnumerateInstanceVersion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html)
pub type PFN_vkEnumerateInstanceVersion = Option<vkEnumerateInstanceVersion_t>;
/// Non-nullable pointer to [vkEnumerateInstanceVersion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html)
pub type vkEnumerateInstanceVersion_t = extern "system" fn(pApiVersion: &mut VulkanVersion) -> VkResult;

/// Nullable pointer to [vkGetImageDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html)
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = Option<vkGetImageDrmFormatModifierPropertiesEXT_t>;
/// Non-nullable pointer to [vkGetImageDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html)
pub type vkGetImageDrmFormatModifierPropertiesEXT_t =
  unsafe extern "system" fn(device: VkDevice, image: VkImage, pProperties: *mut VkImageDrmFormatModifierPropertiesEXT) -> VkResult;

/// Nullable pointer to [vkGetRandROutputDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRandROutputDisplayEXT.html)
pub type PFN_vkGetRandROutputDisplayEXT = Option<vkGetRandROutputDisplayEXT_t>;
/// Non-nullable pointer to [vkGetRandROutputDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRandROutputDisplayEXT.html)
pub type vkGetRandROutputDisplayEXT_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut Display, rrOutput: RROutput, pDisplay: *mut VkDisplayKHR) -> VkResult;

/// Nullable pointer to [vkRegisterDeviceEventEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDeviceEventEXT.html)
pub type PFN_vkRegisterDeviceEventEXT = Option<vkRegisterDeviceEventEXT_t>;
/// Non-nullable pointer to [vkRegisterDeviceEventEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDeviceEventEXT.html)
pub type vkRegisterDeviceEventEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  pDeviceEventInfo: *const VkDeviceEventInfoEXT,
  pAllocator: *const VkAllocationCallbacks,
  pFence: *mut VkFence,
) -> VkResult;

/// Nullable pointer to [vkRegisterDisplayEventEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDisplayEventEXT.html)
pub type PFN_vkRegisterDisplayEventEXT = Option<vkRegisterDisplayEventEXT_t>;
/// Non-nullable pointer to [vkRegisterDisplayEventEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkRegisterDisplayEventEXT.html)
pub type vkRegisterDisplayEventEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  display: VkDisplayKHR,
  pDisplayEventInfo: *const VkDisplayEventInfoEXT,
  pAllocator: *const VkAllocationCallbacks,
  pFence: *mut VkFence,
) -> VkResult;

/// Nullable pointer to [vkSetPrivateDataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetPrivateDataEXT.html)
pub type PFN_vkSetPrivateDataEXT = Option<vkSetPrivateDataEXT_t>;
/// Non-nullable pointer to [vkSetPrivateDataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetPrivateDataEXT.html)
pub type vkSetPrivateDataEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  objectType: VkObjectType,
  objectHandle: uint64_t,
  privateDataSlot: VkPrivateDataSlotEXT,
  data: uint64_t,
) -> VkResult;

/// Nullable pointer to [vkAcquireWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireWinrtDisplayNV.html)
pub type PFN_vkAcquireWinrtDisplayNV = Option<vkAcquireWinrtDisplayNV_t>;
/// Non-nullable pointer to [vkAcquireWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireWinrtDisplayNV.html)
pub type vkAcquireWinrtDisplayNV_t = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;

/// Nullable pointer to [vkGetWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetWinrtDisplayNV.html)
pub type PFN_vkGetWinrtDisplayNV = Option<vkGetWinrtDisplayNV_t>;
/// Non-nullable pointer to [vkGetWinrtDisplayNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetWinrtDisplayNV.html)
pub type vkGetWinrtDisplayNV_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, deviceRelativeId: uint32_t, pDisplay: *mut VkDisplayKHR) -> VkResult;

/// Nullable pointer to [vkGetSwapchainCounterEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainCounterEXT.html)
pub type PFN_vkGetSwapchainCounterEXT = Option<vkGetSwapchainCounterEXT_t>;
/// Non-nullable pointer to [vkGetSwapchainCounterEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainCounterEXT.html)
pub type vkGetSwapchainCounterEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  counter: VkSurfaceCounterFlagBitsEXT,
  pCounterValue: *mut uint64_t,
) -> VkResult;

/// Nullable pointer to [vkGetRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html)
pub type PFN_vkGetRefreshCycleDurationGOOGLE = Option<vkGetRefreshCycleDurationGOOGLE_t>;
/// Non-nullable pointer to [vkGetRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html)
pub type vkGetRefreshCycleDurationGOOGLE_t =
  unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE) -> VkResult;

/// Nullable pointer to [vkAcquireXlibDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireXlibDisplayEXT.html)
pub type PFN_vkAcquireXlibDisplayEXT = Option<vkAcquireXlibDisplayEXT_t>;
/// Non-nullable pointer to [vkAcquireXlibDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireXlibDisplayEXT.html)
pub type vkAcquireXlibDisplayEXT_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut Display, display: VkDisplayKHR) -> VkResult;

/// Nullable pointer to [vkGetMemoryFdPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdPropertiesKHR.html)
pub type PFN_vkGetMemoryFdPropertiesKHR = Option<vkGetMemoryFdPropertiesKHR_t>;
/// Non-nullable pointer to [vkGetMemoryFdPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdPropertiesKHR.html)
pub type vkGetMemoryFdPropertiesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBits,
  fd: int,
  pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR,
) -> VkResult;

/// Nullable pointer to [vkGetMemoryHostPointerPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html)
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = Option<vkGetMemoryHostPointerPropertiesEXT_t>;
/// Non-nullable pointer to [vkGetMemoryHostPointerPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html)
pub type vkGetMemoryHostPointerPropertiesEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBits,
  pHostPointer: *const c_void,
  pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
) -> VkResult;

/// Nullable pointer to [vkGetMemoryWin32HandlePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html)
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = Option<vkGetMemoryWin32HandlePropertiesKHR_t>;
/// Non-nullable pointer to [vkGetMemoryWin32HandlePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html)
pub type vkGetMemoryWin32HandlePropertiesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBits,
  handle: HANDLE,
  pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
) -> VkResult;

/// Nullable pointer to [vkImportFenceFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceFdKHR.html)
pub type PFN_vkImportFenceFdKHR = Option<vkImportFenceFdKHR_t>;
/// Non-nullable pointer to [vkImportFenceFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceFdKHR.html)
pub type vkImportFenceFdKHR_t = unsafe extern "system" fn(device: VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> VkResult;

/// Nullable pointer to [vkImportFenceWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceWin32HandleKHR.html)
pub type PFN_vkImportFenceWin32HandleKHR = Option<vkImportFenceWin32HandleKHR_t>;
/// Non-nullable pointer to [vkImportFenceWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceWin32HandleKHR.html)
pub type vkImportFenceWin32HandleKHR_t =
  unsafe extern "system" fn(device: VkDevice, pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR) -> VkResult;

/// Nullable pointer to [vkImportSemaphoreFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreFdKHR.html)
pub type PFN_vkImportSemaphoreFdKHR = Option<vkImportSemaphoreFdKHR_t>;
/// Non-nullable pointer to [vkImportSemaphoreFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreFdKHR.html)
pub type vkImportSemaphoreFdKHR_t =
  unsafe extern "system" fn(device: VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR) -> VkResult;

/// Nullable pointer to [vkImportSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html)
pub type PFN_vkImportSemaphoreWin32HandleKHR = Option<vkImportSemaphoreWin32HandleKHR_t>;
/// Non-nullable pointer to [vkImportSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html)
pub type vkImportSemaphoreWin32HandleKHR_t =
  unsafe extern "system" fn(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult;

/// Nullable pointer to [vkGetAndroidHardwareBufferPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html)
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = Option<vkGetAndroidHardwareBufferPropertiesANDROID_t>;
/// Non-nullable pointer to [vkGetAndroidHardwareBufferPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html)
pub type vkGetAndroidHardwareBufferPropertiesANDROID_t =
  unsafe extern "system" fn(device: VkDevice, buffer: *const AHardwareBuffer, pProperties: *mut VkAndroidHardwareBufferPropertiesANDROID) -> VkResult;

/// Nullable pointer to [vkCreateAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureKHR.html)
pub type PFN_vkCreateAccelerationStructureKHR = Option<vkCreateAccelerationStructureKHR_t>;
/// Non-nullable pointer to [vkCreateAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureKHR.html)
pub type vkCreateAccelerationStructureKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pAccelerationStructure: *mut VkAccelerationStructureKHR,
) -> VkResult;

/// Nullable pointer to [vkBindAccelerationStructureMemoryNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryNV.html)
pub type PFN_vkBindAccelerationStructureMemoryNV = Option<vkBindAccelerationStructureMemoryNV_t>;
/// Non-nullable pointer to [vkBindAccelerationStructureMemoryNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryNV.html)
pub type vkBindAccelerationStructureMemoryNV_t =
  unsafe extern "system" fn(device: VkDevice, bindInfoCount: uint32_t, pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV) -> VkResult;

/// Nullable pointer to [vkBindImageMemory2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2.html)
pub type PFN_vkBindImageMemory2 = Option<vkBindImageMemory2_t>;
/// Non-nullable pointer to [vkBindImageMemory2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2.html)
pub type vkBindImageMemory2_t =
  unsafe extern "system" fn(device: VkDevice, bindInfoCount: uint32_t, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult;

/// Nullable pointer to [vkCompileDeferredNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCompileDeferredNV.html)
pub type PFN_vkCompileDeferredNV = Option<vkCompileDeferredNV_t>;
/// Non-nullable pointer to [vkCompileDeferredNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCompileDeferredNV.html)
pub type vkCompileDeferredNV_t = unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, shader: uint32_t) -> VkResult;

/// Nullable pointer to [vkCreateDescriptorUpdateTemplate](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplate.html)
pub type PFN_vkCreateDescriptorUpdateTemplate = Option<vkCreateDescriptorUpdateTemplate_t>;
/// Non-nullable pointer to [vkCreateDescriptorUpdateTemplate](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplate.html)
pub type vkCreateDescriptorUpdateTemplate_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
  pAllocator: *const VkAllocationCallbacks,
  pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult;

/// Nullable pointer to [vkCreateDirectFBSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDirectFBSurfaceEXT.html)
pub type PFN_vkCreateDirectFBSurfaceEXT = Option<vkCreateDirectFBSurfaceEXT_t>;
/// Non-nullable pointer to [vkCreateDirectFBSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDirectFBSurfaceEXT.html)
pub type vkCreateDirectFBSurfaceEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkDirectFBSurfaceCreateInfoEXT,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateDisplayPlaneSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html)
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = Option<vkCreateDisplayPlaneSurfaceKHR_t>;
/// Non-nullable pointer to [vkCreateDisplayPlaneSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html)
pub type vkCreateDisplayPlaneSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateHeadlessSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateHeadlessSurfaceEXT.html)
pub type PFN_vkCreateHeadlessSurfaceEXT = Option<vkCreateHeadlessSurfaceEXT_t>;
/// Non-nullable pointer to [vkCreateHeadlessSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateHeadlessSurfaceEXT.html)
pub type vkCreateHeadlessSurfaceEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateImagePipeSurfaceFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html)
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = Option<vkCreateImagePipeSurfaceFUCHSIA_t>;
/// Non-nullable pointer to [vkCreateImagePipeSurfaceFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html)
pub type vkCreateImagePipeSurfaceFUCHSIA_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkImagePipeSurfaceCreateInfoFUCHSIA,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html)
pub type PFN_vkCreateIndirectCommandsLayoutNV = Option<vkCreateIndirectCommandsLayoutNV_t>;
/// Non-nullable pointer to [vkCreateIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html)
pub type vkCreateIndirectCommandsLayoutNV_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV,
  pAllocator: *const VkAllocationCallbacks,
  pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNV,
) -> VkResult;

/// Nullable pointer to [vkCreateRenderPass2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass2.html)
pub type PFN_vkCreateRenderPass2 = Option<vkCreateRenderPass2_t>;
/// Non-nullable pointer to [vkCreateRenderPass2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass2.html)
pub type vkCreateRenderPass2_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkRenderPassCreateInfo2,
  pAllocator: *const VkAllocationCallbacks,
  pRenderPass: *mut VkRenderPass,
) -> VkResult;

/// Nullable pointer to [vkCreateSamplerYcbcrConversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversion.html)
pub type PFN_vkCreateSamplerYcbcrConversion = Option<vkCreateSamplerYcbcrConversion_t>;
/// Non-nullable pointer to [vkCreateSamplerYcbcrConversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversion.html)
pub type vkCreateSamplerYcbcrConversion_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
  pAllocator: *const VkAllocationCallbacks,
  pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult;

/// Nullable pointer to [vkCreateWaylandSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWaylandSurfaceKHR.html)
pub type PFN_vkCreateWaylandSurfaceKHR = Option<vkCreateWaylandSurfaceKHR_t>;
/// Non-nullable pointer to [vkCreateWaylandSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWaylandSurfaceKHR.html)
pub type vkCreateWaylandSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateWin32SurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWin32SurfaceKHR.html)
pub type PFN_vkCreateWin32SurfaceKHR = Option<vkCreateWin32SurfaceKHR_t>;
/// Non-nullable pointer to [vkCreateWin32SurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateWin32SurfaceKHR.html)
pub type vkCreateWin32SurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateXcbSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXcbSurfaceKHR.html)
pub type PFN_vkCreateXcbSurfaceKHR = Option<vkCreateXcbSurfaceKHR_t>;
/// Non-nullable pointer to [vkCreateXcbSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXcbSurfaceKHR.html)
pub type vkCreateXcbSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateXlibSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXlibSurfaceKHR.html)
pub type PFN_vkCreateXlibSurfaceKHR = Option<vkCreateXlibSurfaceKHR_t>;
/// Non-nullable pointer to [vkCreateXlibSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXlibSurfaceKHR.html)
pub type vkCreateXlibSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkDebugMarkerSetObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html)
pub type PFN_vkDebugMarkerSetObjectNameEXT = Option<vkDebugMarkerSetObjectNameEXT_t>;
/// Non-nullable pointer to [vkDebugMarkerSetObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html)
pub type vkDebugMarkerSetObjectNameEXT_t = unsafe extern "system" fn(device: VkDevice, pNameInfo: *const VkDebugMarkerObjectNameInfoEXT) -> VkResult;

/// Nullable pointer to [vkDebugMarkerSetObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html)
pub type PFN_vkDebugMarkerSetObjectTagEXT = Option<vkDebugMarkerSetObjectTagEXT_t>;
/// Non-nullable pointer to [vkDebugMarkerSetObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html)
pub type vkDebugMarkerSetObjectTagEXT_t = unsafe extern "system" fn(device: VkDevice, pTagInfo: *const VkDebugMarkerObjectTagInfoEXT) -> VkResult;

/// Nullable pointer to [vkGetAccelerationStructureHandleNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureHandleNV.html)
pub type PFN_vkGetAccelerationStructureHandleNV = Option<vkGetAccelerationStructureHandleNV_t>;
/// Non-nullable pointer to [vkGetAccelerationStructureHandleNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureHandleNV.html)
pub type vkGetAccelerationStructureHandleNV_t =
  unsafe extern "system" fn(device: VkDevice, accelerationStructure: VkAccelerationStructureNV, dataSize: size_t, pData: *mut c_void) -> VkResult;

/// Nullable pointer to [vkGetCalibratedTimestampsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetCalibratedTimestampsEXT.html)
pub type PFN_vkGetCalibratedTimestampsEXT = Option<vkGetCalibratedTimestampsEXT_t>;
/// Non-nullable pointer to [vkGetCalibratedTimestampsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetCalibratedTimestampsEXT.html)
pub type vkGetCalibratedTimestampsEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  timestampCount: uint32_t,
  pTimestampInfos: *const VkCalibratedTimestampInfoEXT,
  pTimestamps: *mut uint64_t,
  pMaxDeviation: *mut uint64_t,
) -> VkResult;

/// Nullable pointer to [vkGetDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html)
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = Option<vkGetDeviceGroupPresentCapabilitiesKHR_t>;
/// Non-nullable pointer to [vkGetDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html)
pub type vkGetDeviceGroupPresentCapabilitiesKHR_t =
  unsafe extern "system" fn(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR) -> VkResult;

/// Nullable pointer to [vkGetDisplayPlaneCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html)
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = Option<vkGetDisplayPlaneCapabilities2KHR_t>;
/// Non-nullable pointer to [vkGetDisplayPlaneCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html)
pub type vkGetDisplayPlaneCapabilities2KHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR,
  pCapabilities: *mut VkDisplayPlaneCapabilities2KHR,
) -> VkResult;

/// Nullable pointer to [vkGetDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html)
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = Option<vkGetDisplayPlaneCapabilitiesKHR_t>;
/// Non-nullable pointer to [vkGetDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html)
pub type vkGetDisplayPlaneCapabilitiesKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  mode: VkDisplayModeKHR,
  planeIndex: uint32_t,
  pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR,
) -> VkResult;

/// Nullable pointer to [vkGetRayTracingCaptureReplayShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR = Option<vkGetRayTracingCaptureReplayShaderGroupHandlesKHR_t>;
/// Non-nullable pointer to [vkGetRayTracingCaptureReplayShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)
pub type vkGetRayTracingCaptureReplayShaderGroupHandlesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline: VkPipeline,
  firstGroup: uint32_t,
  groupCount: uint32_t,
  dataSize: size_t,
  pData: *mut c_void,
) -> VkResult;

/// Nullable pointer to [vkGetRayTracingShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html)
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = Option<vkGetRayTracingShaderGroupHandlesKHR_t>;
/// Non-nullable pointer to [vkGetRayTracingShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html)
pub type vkGetRayTracingShaderGroupHandlesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline: VkPipeline,
  firstGroup: uint32_t,
  groupCount: uint32_t,
  dataSize: size_t,
  pData: *mut c_void,
) -> VkResult;

/// Nullable pointer to [vkMergeValidationCachesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergeValidationCachesEXT.html)
pub type PFN_vkMergeValidationCachesEXT = Option<vkMergeValidationCachesEXT_t>;
/// Non-nullable pointer to [vkMergeValidationCachesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergeValidationCachesEXT.html)
pub type vkMergeValidationCachesEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  dstCache: VkValidationCacheEXT,
  srcCacheCount: uint32_t,
  pSrcCaches: *const VkValidationCacheEXT,
) -> VkResult;

/// Nullable pointer to [vkSetDebugUtilsObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html)
pub type PFN_vkSetDebugUtilsObjectNameEXT = Option<vkSetDebugUtilsObjectNameEXT_t>;
/// Non-nullable pointer to [vkSetDebugUtilsObjectNameEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html)
pub type vkSetDebugUtilsObjectNameEXT_t = unsafe extern "system" fn(device: VkDevice, pNameInfo: *const VkDebugUtilsObjectNameInfoEXT) -> VkResult;

/// Nullable pointer to [vkSetDebugUtilsObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html)
pub type PFN_vkSetDebugUtilsObjectTagEXT = Option<vkSetDebugUtilsObjectTagEXT_t>;
/// Non-nullable pointer to [vkSetDebugUtilsObjectTagEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html)
pub type vkSetDebugUtilsObjectTagEXT_t = unsafe extern "system" fn(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT) -> VkResult;

/// Nullable pointer to [vkSignalSemaphore](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphore.html)
pub type PFN_vkSignalSemaphore = Option<vkSignalSemaphore_t>;
/// Non-nullable pointer to [vkSignalSemaphore](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphore.html)
pub type vkSignalSemaphore_t = unsafe extern "system" fn(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;

/// Nullable pointer to [vkWriteAccelerationStructuresPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html)
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = Option<vkWriteAccelerationStructuresPropertiesKHR_t>;
/// Non-nullable pointer to [vkWriteAccelerationStructuresPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html)
pub type vkWriteAccelerationStructuresPropertiesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  accelerationStructureCount: uint32_t,
  pAccelerationStructures: *const VkAccelerationStructureKHR,
  queryType: VkQueryType,
  dataSize: size_t,
  pData: *mut c_void,
  stride: size_t,
) -> VkResult;

/// Nullable pointer to [vkGetSemaphoreCounterValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValue.html)
pub type PFN_vkGetSemaphoreCounterValue = Option<vkGetSemaphoreCounterValue_t>;
/// Non-nullable pointer to [vkGetSemaphoreCounterValue](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValue.html)
pub type vkGetSemaphoreCounterValue_t = unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut uint64_t) -> VkResult;

/// Nullable pointer to [vkCreateSwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSwapchainKHR.html)
pub type PFN_vkCreateSwapchainKHR = Option<vkCreateSwapchainKHR_t>;
/// Non-nullable pointer to [vkCreateSwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSwapchainKHR.html)
pub type vkCreateSwapchainKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkSwapchainCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pSwapchain: *mut VkSwapchainKHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceExternalImageFormatPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = Option<vkGetPhysicalDeviceExternalImageFormatPropertiesNV_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceExternalImageFormatPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)
pub type vkGetPhysicalDeviceExternalImageFormatPropertiesNV_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  format: VkFormat,
  type_: VkImageType,
  tiling: VkImageTiling,
  usage: VkImageUsageFlags,
  flags: VkImageCreateFlags,
  externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
  pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html)
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = Option<vkGetPhysicalDeviceImageFormatProperties2_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html)
pub type vkGetPhysicalDeviceImageFormatProperties2_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
  pImageFormatProperties: *mut VkImageFormatProperties2,
) -> VkResult;

/// Nullable pointer to [vkCreateSharedSwapchainsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSharedSwapchainsKHR.html)
pub type PFN_vkCreateSharedSwapchainsKHR = Option<vkCreateSharedSwapchainsKHR_t>;
/// Non-nullable pointer to [vkCreateSharedSwapchainsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSharedSwapchainsKHR.html)
pub type vkCreateSharedSwapchainsKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchainCount: uint32_t,
  pCreateInfos: *const VkSwapchainCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pSwapchains: *mut VkSwapchainKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateDisplayModeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayModeKHR.html)
pub type PFN_vkCreateDisplayModeKHR = Option<vkCreateDisplayModeKHR_t>;
/// Non-nullable pointer to [vkCreateDisplayModeKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayModeKHR.html)
pub type vkCreateDisplayModeKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  display: VkDisplayKHR,
  pCreateInfo: *const VkDisplayModeCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pMode: *mut VkDisplayModeKHR,
) -> VkResult;

/// Nullable pointer to [vkAcquireFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html)
pub type PFN_vkAcquireFullScreenExclusiveModeEXT = Option<vkAcquireFullScreenExclusiveModeEXT_t>;
/// Non-nullable pointer to [vkAcquireFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html)
pub type vkAcquireFullScreenExclusiveModeEXT_t = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;

/// Nullable pointer to [vkBindBufferMemory2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2.html)
pub type PFN_vkBindBufferMemory2 = Option<vkBindBufferMemory2_t>;
/// Non-nullable pointer to [vkBindBufferMemory2](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2.html)
pub type vkBindBufferMemory2_t =
  unsafe extern "system" fn(device: VkDevice, bindInfoCount: uint32_t, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult;

/// Nullable pointer to [vkCreateAndroidSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAndroidSurfaceKHR.html)
pub type PFN_vkCreateAndroidSurfaceKHR = Option<vkCreateAndroidSurfaceKHR_t>;
/// Non-nullable pointer to [vkCreateAndroidSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAndroidSurfaceKHR.html)
pub type vkCreateAndroidSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateIOSSurfaceMVK](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIOSSurfaceMVK.html)
pub type PFN_vkCreateIOSSurfaceMVK = Option<vkCreateIOSSurfaceMVK_t>;
/// Non-nullable pointer to [vkCreateIOSSurfaceMVK](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIOSSurfaceMVK.html)
pub type vkCreateIOSSurfaceMVK_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkIOSSurfaceCreateInfoMVK,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateMacOSSurfaceMVK](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMacOSSurfaceMVK.html)
pub type PFN_vkCreateMacOSSurfaceMVK = Option<vkCreateMacOSSurfaceMVK_t>;
/// Non-nullable pointer to [vkCreateMacOSSurfaceMVK](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMacOSSurfaceMVK.html)
pub type vkCreateMacOSSurfaceMVK_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateMetalSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMetalSurfaceEXT.html)
pub type PFN_vkCreateMetalSurfaceEXT = Option<vkCreateMetalSurfaceEXT_t>;
/// Non-nullable pointer to [vkCreateMetalSurfaceEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateMetalSurfaceEXT.html)
pub type vkCreateMetalSurfaceEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkMetalSurfaceCreateInfoEXT,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

#[cfg(feature = "google_games_platform")]
/// Nullable pointer to [vkCreateStreamDescriptorSurfaceGGP](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html)
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = Option<vkCreateStreamDescriptorSurfaceGGP_t>;
#[cfg(feature = "google_games_platform")]
/// Non-nullable pointer to [vkCreateStreamDescriptorSurfaceGGP](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html)
pub type vkCreateStreamDescriptorSurfaceGGP_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkStreamDescriptorSurfaceCreateInfoGGP,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateViSurfaceNN](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateViSurfaceNN.html)
pub type PFN_vkCreateViSurfaceNN = Option<vkCreateViSurfaceNN_t>;
/// Non-nullable pointer to [vkCreateViSurfaceNN](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateViSurfaceNN.html)
pub type vkCreateViSurfaceNN_t = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkViSurfaceCreateInfoNN,
  pAllocator: *const VkAllocationCallbacks,
  pSurface: *mut VkSurfaceKHR,
) -> VkResult;

/// Nullable pointer to [vkGetDeviceGroupSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html)
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = Option<vkGetDeviceGroupSurfacePresentModes2EXT_t>;
/// Non-nullable pointer to [vkGetDeviceGroupSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html)
pub type vkGetDeviceGroupSurfacePresentModes2EXT_t = unsafe extern "system" fn(
  device: VkDevice,
  pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
  pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult;

/// Nullable pointer to [vkGetDeviceGroupSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html)
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = Option<vkGetDeviceGroupSurfacePresentModesKHR_t>;
/// Non-nullable pointer to [vkGetDeviceGroupSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html)
pub type vkGetDeviceGroupSurfacePresentModesKHR_t =
  unsafe extern "system" fn(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceSurfaceCapabilities2EXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = Option<vkGetPhysicalDeviceSurfaceCapabilities2EXT_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSurfaceCapabilities2EXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)
pub type vkGetPhysicalDeviceSurfaceCapabilities2EXT_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  surface: VkSurfaceKHR,
  pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceSurfaceCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = Option<vkGetPhysicalDeviceSurfaceCapabilities2KHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSurfaceCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)
pub type vkGetPhysicalDeviceSurfaceCapabilities2KHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
  pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = Option<vkGetPhysicalDeviceSurfaceCapabilitiesKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
pub type vkGetPhysicalDeviceSurfaceCapabilitiesKHR_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceSurfaceSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = Option<vkGetPhysicalDeviceSurfaceSupportKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSurfaceSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
pub type vkGetPhysicalDeviceSurfaceSupportKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: uint32_t,
  surface: VkSurfaceKHR,
  pSupported: *mut VkBool32,
) -> VkResult;

/// Nullable pointer to [vkReleaseFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html)
pub type PFN_vkReleaseFullScreenExclusiveModeEXT = Option<vkReleaseFullScreenExclusiveModeEXT_t>;
/// Non-nullable pointer to [vkReleaseFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html)
pub type vkReleaseFullScreenExclusiveModeEXT_t = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;

/// Nullable pointer to [vkGetImageViewAddressNVX](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewAddressNVX.html)
pub type PFN_vkGetImageViewAddressNVX = Option<vkGetImageViewAddressNVX_t>;
/// Non-nullable pointer to [vkGetImageViewAddressNVX](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageViewAddressNVX.html)
pub type vkGetImageViewAddressNVX_t =
  unsafe extern "system" fn(device: VkDevice, imageView: VkImageView, pProperties: *mut VkImageViewAddressPropertiesNVX) -> VkResult;

/// Nullable pointer to [vkAcquireProfilingLockKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireProfilingLockKHR.html)
pub type PFN_vkAcquireProfilingLockKHR = Option<vkAcquireProfilingLockKHR_t>;
/// Non-nullable pointer to [vkAcquireProfilingLockKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireProfilingLockKHR.html)
pub type vkAcquireProfilingLockKHR_t = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkAcquireProfilingLockInfoKHR) -> VkResult;

/// Nullable pointer to [vkAcquirePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html)
pub type PFN_vkAcquirePerformanceConfigurationINTEL = Option<vkAcquirePerformanceConfigurationINTEL_t>;
/// Non-nullable pointer to [vkAcquirePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html)
pub type vkAcquirePerformanceConfigurationINTEL_t = unsafe extern "system" fn(
  device: VkDevice,
  pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL,
  pConfiguration: *mut VkPerformanceConfigurationINTEL,
) -> VkResult;

/// Nullable pointer to [vkGetFenceFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceFdKHR.html)
pub type PFN_vkGetFenceFdKHR = Option<vkGetFenceFdKHR_t>;
/// Non-nullable pointer to [vkGetFenceFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceFdKHR.html)
pub type vkGetFenceFdKHR_t = unsafe extern "system" fn(device: VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut int) -> VkResult;

/// Nullable pointer to [vkGetFenceWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceWin32HandleKHR.html)
pub type PFN_vkGetFenceWin32HandleKHR = Option<vkGetFenceWin32HandleKHR_t>;
/// Non-nullable pointer to [vkGetFenceWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceWin32HandleKHR.html)
pub type vkGetFenceWin32HandleKHR_t =
  unsafe extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;

/// Nullable pointer to [vkGetMemoryAndroidHardwareBufferANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html)
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = Option<vkGetMemoryAndroidHardwareBufferANDROID_t>;
/// Non-nullable pointer to [vkGetMemoryAndroidHardwareBufferANDROID](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html)
pub type vkGetMemoryAndroidHardwareBufferANDROID_t =
  unsafe extern "system" fn(device: VkDevice, pInfo: *const VkMemoryGetAndroidHardwareBufferInfoANDROID, pBuffer: *mut AHardwareBuffer) -> VkResult;

/// Nullable pointer to [vkGetMemoryFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdKHR.html)
pub type PFN_vkGetMemoryFdKHR = Option<vkGetMemoryFdKHR_t>;
/// Non-nullable pointer to [vkGetMemoryFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdKHR.html)
pub type vkGetMemoryFdKHR_t = unsafe extern "system" fn(device: VkDevice, pGetFdInfo: *const VkMemoryGetFdInfoKHR, pFd: *mut int) -> VkResult;

/// Nullable pointer to [vkGetMemoryWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleKHR.html)
pub type PFN_vkGetMemoryWin32HandleKHR = Option<vkGetMemoryWin32HandleKHR_t>;
/// Non-nullable pointer to [vkGetMemoryWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleKHR.html)
pub type vkGetMemoryWin32HandleKHR_t =
  unsafe extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;

/// Nullable pointer to [vkGetMemoryWin32HandleNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleNV.html)
pub type PFN_vkGetMemoryWin32HandleNV = Option<vkGetMemoryWin32HandleNV_t>;
/// Non-nullable pointer to [vkGetMemoryWin32HandleNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryWin32HandleNV.html)
pub type vkGetMemoryWin32HandleNV_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
  handleType: VkExternalMemoryHandleTypeFlagsNV,
  pHandle: *mut HANDLE,
) -> VkResult;

/// Nullable pointer to [vkGetPerformanceParameterINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPerformanceParameterINTEL.html)
pub type PFN_vkGetPerformanceParameterINTEL = Option<vkGetPerformanceParameterINTEL_t>;
/// Non-nullable pointer to [vkGetPerformanceParameterINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPerformanceParameterINTEL.html)
pub type vkGetPerformanceParameterINTEL_t =
  unsafe extern "system" fn(device: VkDevice, parameter: VkPerformanceParameterTypeINTEL, pValue: *mut VkPerformanceValueINTEL) -> VkResult;

/// Nullable pointer to [vkGetSemaphoreFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreFdKHR.html)
pub type PFN_vkGetSemaphoreFdKHR = Option<vkGetSemaphoreFdKHR_t>;
/// Non-nullable pointer to [vkGetSemaphoreFdKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreFdKHR.html)
pub type vkGetSemaphoreFdKHR_t = unsafe extern "system" fn(device: VkDevice, pGetFdInfo: *const VkSemaphoreGetFdInfoKHR, pFd: *mut int) -> VkResult;

/// Nullable pointer to [vkGetSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html)
pub type PFN_vkGetSemaphoreWin32HandleKHR = Option<vkGetSemaphoreWin32HandleKHR_t>;
/// Non-nullable pointer to [vkGetSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html)
pub type vkGetSemaphoreWin32HandleKHR_t =
  unsafe extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;

/// Nullable pointer to [vkInitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInitializePerformanceApiINTEL.html)
pub type PFN_vkInitializePerformanceApiINTEL = Option<vkInitializePerformanceApiINTEL_t>;
/// Non-nullable pointer to [vkInitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInitializePerformanceApiINTEL.html)
pub type vkInitializePerformanceApiINTEL_t =
  unsafe extern "system" fn(device: VkDevice, pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL) -> VkResult;

/// Nullable pointer to [vkQueueSetPerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html)
pub type PFN_vkQueueSetPerformanceConfigurationINTEL = Option<vkQueueSetPerformanceConfigurationINTEL_t>;
/// Non-nullable pointer to [vkQueueSetPerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html)
pub type vkQueueSetPerformanceConfigurationINTEL_t =
  unsafe extern "system" fn(queue: VkQueue, configuration: VkPerformanceConfigurationINTEL) -> VkResult;

/// Nullable pointer to [vkReleasePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html)
pub type PFN_vkReleasePerformanceConfigurationINTEL = Option<vkReleasePerformanceConfigurationINTEL_t>;
/// Non-nullable pointer to [vkReleasePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html)
pub type vkReleasePerformanceConfigurationINTEL_t =
  unsafe extern "system" fn(device: VkDevice, configuration: VkPerformanceConfigurationINTEL) -> VkResult;

/// Nullable pointer to [vkCmdSetPerformanceMarkerINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html)
pub type PFN_vkCmdSetPerformanceMarkerINTEL = Option<vkCmdSetPerformanceMarkerINTEL_t>;
/// Non-nullable pointer to [vkCmdSetPerformanceMarkerINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html)
pub type vkCmdSetPerformanceMarkerINTEL_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkPerformanceMarkerInfoINTEL) -> VkResult;

/// Nullable pointer to [vkCmdSetPerformanceOverrideINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html)
pub type PFN_vkCmdSetPerformanceOverrideINTEL = Option<vkCmdSetPerformanceOverrideINTEL_t>;
/// Non-nullable pointer to [vkCmdSetPerformanceOverrideINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html)
pub type vkCmdSetPerformanceOverrideINTEL_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pOverrideInfo: *const VkPerformanceOverrideInfoINTEL) -> VkResult;

/// Nullable pointer to [vkCmdSetPerformanceStreamMarkerINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html)
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = Option<vkCmdSetPerformanceStreamMarkerINTEL_t>;
/// Non-nullable pointer to [vkCmdSetPerformanceStreamMarkerINTEL](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html)
pub type vkCmdSetPerformanceStreamMarkerINTEL_t =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL) -> VkResult;

/// Nullable pointer to [vkGetShaderInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetShaderInfoAMD.html)
pub type PFN_vkGetShaderInfoAMD = Option<vkGetShaderInfoAMD_t>;
/// Non-nullable pointer to [vkGetShaderInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetShaderInfoAMD.html)
pub type vkGetShaderInfoAMD_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline: VkPipeline,
  shaderStage: VkShaderStageFlagBits,
  infoType: VkShaderInfoTypeAMD,
  pInfoSize: *mut size_t,
  pInfo: *mut c_void,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceFragmentShadingRatesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = Option<vkGetPhysicalDeviceFragmentShadingRatesKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceFragmentShadingRatesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)
pub type vkGetPhysicalDeviceFragmentShadingRatesKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pFragmentShadingRateCount: *mut uint32_t,
  pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceToolPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html)
pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = Option<vkGetPhysicalDeviceToolPropertiesEXT_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceToolPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html)
pub type vkGetPhysicalDeviceToolPropertiesEXT_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pToolCount: *mut uint32_t,
  pToolProperties: *mut VkPhysicalDeviceToolPropertiesEXT,
) -> VkResult;

/// Nullable pointer to [vkGetPastPresentationTimingGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html)
pub type PFN_vkGetPastPresentationTimingGOOGLE = Option<vkGetPastPresentationTimingGOOGLE_t>;
/// Non-nullable pointer to [vkGetPastPresentationTimingGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html)
pub type vkGetPastPresentationTimingGOOGLE_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  pPresentationTimingCount: *mut uint32_t,
  pPresentationTimings: *mut VkPastPresentationTimingGOOGLE,
) -> VkResult;

/// Nullable pointer to [vkGetDisplayModeProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModeProperties2KHR.html)
pub type PFN_vkGetDisplayModeProperties2KHR = Option<vkGetDisplayModeProperties2KHR_t>;
/// Non-nullable pointer to [vkGetDisplayModeProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModeProperties2KHR.html)
pub type vkGetDisplayModeProperties2KHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  display: VkDisplayKHR,
  pPropertyCount: *mut uint32_t,
  pProperties: *mut VkDisplayModeProperties2KHR,
) -> VkResult;

/// Nullable pointer to [vkGetDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModePropertiesKHR.html)
pub type PFN_vkGetDisplayModePropertiesKHR = Option<vkGetDisplayModePropertiesKHR_t>;
/// Non-nullable pointer to [vkGetDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModePropertiesKHR.html)
pub type vkGetDisplayModePropertiesKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  display: VkDisplayKHR,
  pPropertyCount: *mut uint32_t,
  pProperties: *mut VkDisplayModePropertiesKHR,
) -> VkResult;

/// Nullable pointer to [vkGetDisplayPlaneSupportedDisplaysKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html)
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = Option<vkGetDisplayPlaneSupportedDisplaysKHR_t>;
/// Non-nullable pointer to [vkGetDisplayPlaneSupportedDisplaysKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html)
pub type vkGetDisplayPlaneSupportedDisplaysKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  planeIndex: uint32_t,
  pDisplayCount: *mut uint32_t,
  pDisplays: *mut VkDisplayKHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceCalibrateableTimeDomainsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = Option<vkGetPhysicalDeviceCalibrateableTimeDomainsEXT_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceCalibrateableTimeDomainsEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)
pub type vkGetPhysicalDeviceCalibrateableTimeDomainsEXT_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pTimeDomainCount: *mut uint32_t, pTimeDomains: *mut VkTimeDomainEXT) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceCooperativeMatrixPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = Option<vkGetPhysicalDeviceCooperativeMatrixPropertiesNV_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceCooperativeMatrixPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)
pub type vkGetPhysicalDeviceCooperativeMatrixPropertiesNV_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pPropertyCount: *mut uint32_t,
  pProperties: *mut VkCooperativeMatrixPropertiesNV,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceDisplayPlaneProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = Option<vkGetPhysicalDeviceDisplayPlaneProperties2KHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceDisplayPlaneProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)
pub type vkGetPhysicalDeviceDisplayPlaneProperties2KHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pPropertyCount: *mut uint32_t,
  pProperties: *mut VkDisplayPlaneProperties2KHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = Option<vkGetPhysicalDeviceDisplayPlanePropertiesKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)
pub type vkGetPhysicalDeviceDisplayPlanePropertiesKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pPropertyCount: *mut uint32_t,
  pProperties: *mut VkDisplayPlanePropertiesKHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceDisplayProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html)
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = Option<vkGetPhysicalDeviceDisplayProperties2KHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceDisplayProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html)
pub type vkGetPhysicalDeviceDisplayProperties2KHR_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut uint32_t, pProperties: *mut VkDisplayProperties2KHR) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html)
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = Option<vkGetPhysicalDeviceDisplayPropertiesKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html)
pub type vkGetPhysicalDeviceDisplayPropertiesKHR_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut uint32_t, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDevicePresentRectanglesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html)
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = Option<vkGetPhysicalDevicePresentRectanglesKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDevicePresentRectanglesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html)
pub type vkGetPhysicalDevicePresentRectanglesKHR_t =
  unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut uint32_t, pRects: *mut VkRect2D) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
  Option<vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)
pub type vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pCombinationCount: *mut uint32_t,
  pCombinations: *mut VkFramebufferMixedSamplesCombinationNV,
) -> VkResult;

/// Nullable pointer to [vkGetPipelineExecutableInternalRepresentationsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html)
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = Option<vkGetPipelineExecutableInternalRepresentationsKHR_t>;
/// Non-nullable pointer to [vkGetPipelineExecutableInternalRepresentationsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html)
pub type vkGetPipelineExecutableInternalRepresentationsKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pExecutableInfo: *const VkPipelineExecutableInfoKHR,
  pInternalRepresentationCount: *mut uint32_t,
  pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR,
) -> VkResult;

/// Nullable pointer to [vkGetPipelineExecutablePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html)
pub type PFN_vkGetPipelineExecutablePropertiesKHR = Option<vkGetPipelineExecutablePropertiesKHR_t>;
/// Non-nullable pointer to [vkGetPipelineExecutablePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html)
pub type vkGetPipelineExecutablePropertiesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pPipelineInfo: *const VkPipelineInfoKHR,
  pExecutableCount: *mut uint32_t,
  pProperties: *mut VkPipelineExecutablePropertiesKHR,
) -> VkResult;

/// Nullable pointer to [vkGetPipelineExecutableStatisticsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html)
pub type PFN_vkGetPipelineExecutableStatisticsKHR = Option<vkGetPipelineExecutableStatisticsKHR_t>;
/// Non-nullable pointer to [vkGetPipelineExecutableStatisticsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html)
pub type vkGetPipelineExecutableStatisticsKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pExecutableInfo: *const VkPipelineExecutableInfoKHR,
  pStatisticCount: *mut uint32_t,
  pStatistics: *mut VkPipelineExecutableStatisticKHR,
) -> VkResult;

/// Nullable pointer to [vkGetSwapchainImagesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainImagesKHR.html)
pub type PFN_vkGetSwapchainImagesKHR = Option<vkGetSwapchainImagesKHR_t>;
/// Non-nullable pointer to [vkGetSwapchainImagesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainImagesKHR.html)
pub type vkGetSwapchainImagesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  pSwapchainImageCount: *mut uint32_t,
  pSwapchainImages: *mut VkImage,
) -> VkResult;

/// Nullable pointer to [vkGetValidationCacheDataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetValidationCacheDataEXT.html)
pub type PFN_vkGetValidationCacheDataEXT = Option<vkGetValidationCacheDataEXT_t>;
/// Non-nullable pointer to [vkGetValidationCacheDataEXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetValidationCacheDataEXT.html)
pub type vkGetValidationCacheDataEXT_t =
  unsafe extern "system" fn(device: VkDevice, validationCache: VkValidationCacheEXT, pDataSize: *mut size_t, pData: *mut c_void) -> VkResult;

/// Nullable pointer to [vkEnumeratePhysicalDeviceGroups](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html)
pub type PFN_vkEnumeratePhysicalDeviceGroups = Option<vkEnumeratePhysicalDeviceGroups_t>;
/// Non-nullable pointer to [vkEnumeratePhysicalDeviceGroups](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html)
pub type vkEnumeratePhysicalDeviceGroups_t = unsafe extern "system" fn(
  instance: VkInstance,
  pPhysicalDeviceGroupCount: *mut uint32_t,
  pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
) -> VkResult;

/// Nullable pointer to [vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
  Option<vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR_t>;
/// Non-nullable pointer to [vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)
pub type vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: uint32_t,
  pCounterCount: *mut uint32_t,
  pCounters: *mut VkPerformanceCounterKHR,
  pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceSurfaceFormats2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html)
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = Option<vkGetPhysicalDeviceSurfaceFormats2KHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSurfaceFormats2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html)
pub type vkGetPhysicalDeviceSurfaceFormats2KHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
  pSurfaceFormatCount: *mut uint32_t,
  pSurfaceFormats: *mut VkSurfaceFormat2KHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceSurfaceFormatsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = Option<vkGetPhysicalDeviceSurfaceFormatsKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSurfaceFormatsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
pub type vkGetPhysicalDeviceSurfaceFormatsKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  surface: VkSurfaceKHR,
  pSurfaceFormatCount: *mut uint32_t,
  pSurfaceFormats: *mut VkSurfaceFormatKHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = Option<vkGetPhysicalDeviceSurfacePresentModes2EXT_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)
pub type vkGetPhysicalDeviceSurfacePresentModes2EXT_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
  pPresentModeCount: *mut uint32_t,
  pPresentModes: *mut VkPresentModeKHR,
) -> VkResult;

/// Nullable pointer to [vkGetPhysicalDeviceSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = Option<vkGetPhysicalDeviceSurfacePresentModesKHR_t>;
/// Non-nullable pointer to [vkGetPhysicalDeviceSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
pub type vkGetPhysicalDeviceSurfacePresentModesKHR_t = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  surface: VkSurfaceKHR,
  pPresentModeCount: *mut uint32_t,
  pPresentModes: *mut VkPresentModeKHR,
) -> VkResult;

/// Nullable pointer to [vkGetDeferredOperationResultKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationResultKHR.html)
pub type PFN_vkGetDeferredOperationResultKHR = Option<vkGetDeferredOperationResultKHR_t>;
/// Non-nullable pointer to [vkGetDeferredOperationResultKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationResultKHR.html)
pub type vkGetDeferredOperationResultKHR_t = unsafe extern "system" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;

/// Nullable pointer to [vkBuildAccelerationStructuresKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBuildAccelerationStructuresKHR.html)
pub type PFN_vkBuildAccelerationStructuresKHR = Option<vkBuildAccelerationStructuresKHR_t>;
/// Non-nullable pointer to [vkBuildAccelerationStructuresKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBuildAccelerationStructuresKHR.html)
pub type vkBuildAccelerationStructuresKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  infoCount: uint32_t,
  pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
  ppBuildRangeInfos: *mut VkAccelerationStructureBuildRangeInfoKHR,
) -> VkResult;

/// Nullable pointer to [vkCopyAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureKHR.html)
pub type PFN_vkCopyAccelerationStructureKHR = Option<vkCopyAccelerationStructureKHR_t>;
/// Non-nullable pointer to [vkCopyAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureKHR.html)
pub type vkCopyAccelerationStructureKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pInfo: *const VkCopyAccelerationStructureInfoKHR,
) -> VkResult;

/// Nullable pointer to [vkCopyAccelerationStructureToMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html)
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = Option<vkCopyAccelerationStructureToMemoryKHR_t>;
/// Non-nullable pointer to [vkCopyAccelerationStructureToMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html)
pub type vkCopyAccelerationStructureToMemoryKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
) -> VkResult;

/// Nullable pointer to [vkCopyMemoryToAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html)
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = Option<vkCopyMemoryToAccelerationStructureKHR_t>;
/// Non-nullable pointer to [vkCopyMemoryToAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html)
pub type vkCopyMemoryToAccelerationStructureKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
) -> VkResult;

/// Nullable pointer to [vkCreateRayTracingPipelinesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesKHR.html)
pub type PFN_vkCreateRayTracingPipelinesKHR = Option<vkCreateRayTracingPipelinesKHR_t>;
/// Non-nullable pointer to [vkCreateRayTracingPipelinesKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesKHR.html)
pub type vkCreateRayTracingPipelinesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pipelineCache: VkPipelineCache,
  createInfoCount: uint32_t,
  pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
  pAllocator: *const VkAllocationCallbacks,
  pPipelines: *mut VkPipeline,
) -> VkResult;

/// Nullable pointer to [vkCreateRayTracingPipelinesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesNV.html)
pub type PFN_vkCreateRayTracingPipelinesNV = Option<vkCreateRayTracingPipelinesNV_t>;
/// Non-nullable pointer to [vkCreateRayTracingPipelinesNV](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesNV.html)
pub type vkCreateRayTracingPipelinesNV_t = unsafe extern "system" fn(
  device: VkDevice,
  pipelineCache: VkPipelineCache,
  createInfoCount: uint32_t,
  pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
  pAllocator: *const VkAllocationCallbacks,
  pPipelines: *mut VkPipeline,
) -> VkResult;

/// Nullable pointer to [vkGetSwapchainStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainStatusKHR.html)
pub type PFN_vkGetSwapchainStatusKHR = Option<vkGetSwapchainStatusKHR_t>;
/// Non-nullable pointer to [vkGetSwapchainStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSwapchainStatusKHR.html)
pub type vkGetSwapchainStatusKHR_t = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;

/// Nullable pointer to [vkQueuePresentKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueuePresentKHR.html)
pub type PFN_vkQueuePresentKHR = Option<vkQueuePresentKHR_t>;
/// Non-nullable pointer to [vkQueuePresentKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueuePresentKHR.html)
pub type vkQueuePresentKHR_t = unsafe extern "system" fn(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;

/// Nullable pointer to [vkDeferredOperationJoinKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeferredOperationJoinKHR.html)
pub type PFN_vkDeferredOperationJoinKHR = Option<vkDeferredOperationJoinKHR_t>;
/// Non-nullable pointer to [vkDeferredOperationJoinKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeferredOperationJoinKHR.html)
pub type vkDeferredOperationJoinKHR_t = unsafe extern "system" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;

/// Nullable pointer to [vkWaitSemaphores](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphores.html)
pub type PFN_vkWaitSemaphores = Option<vkWaitSemaphores_t>;
/// Non-nullable pointer to [vkWaitSemaphores](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphores.html)
pub type vkWaitSemaphores_t = unsafe extern "system" fn(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfo, timeout: uint64_t) -> VkResult;

/// Nullable pointer to [vkAcquireNextImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImage2KHR.html)
pub type PFN_vkAcquireNextImage2KHR = Option<vkAcquireNextImage2KHR_t>;
/// Non-nullable pointer to [vkAcquireNextImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImage2KHR.html)
pub type vkAcquireNextImage2KHR_t =
  unsafe extern "system" fn(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHR, pImageIndex: *mut uint32_t) -> VkResult;

/// Nullable pointer to [vkAcquireNextImageKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImageKHR.html)
pub type PFN_vkAcquireNextImageKHR = Option<vkAcquireNextImageKHR_t>;
/// Non-nullable pointer to [vkAcquireNextImageKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireNextImageKHR.html)
pub type vkAcquireNextImageKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  timeout: uint64_t,
  semaphore: VkSemaphore,
  fence: VkFence,
  pImageIndex: *mut uint32_t,
) -> VkResult;
