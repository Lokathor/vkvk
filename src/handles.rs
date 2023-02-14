/// Khronos: [VkInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstance.html) (handle)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkInstance(*mut core::ffi::c_void);
unsafe impl Send for VkInstance {}
unsafe impl Sync for VkInstance {}
impl Default for VkInstance {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkInstance {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}

/// Khronos: [VkPhysicalDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice.html) (handle)
/// * Parent: [VkInstance]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPhysicalDevice(*mut core::ffi::c_void);
unsafe impl Send for VkPhysicalDevice {}
unsafe impl Sync for VkPhysicalDevice {}
impl Default for VkPhysicalDevice {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkPhysicalDevice {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}

/// Khronos: [VkDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDevice.html) (handle)
/// * Parent: [VkPhysicalDevice]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDevice(*mut core::ffi::c_void);
unsafe impl Send for VkDevice {}
unsafe impl Sync for VkDevice {}
impl Default for VkDevice {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkDevice {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}

/// Khronos: [VkQueue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueue.html) (handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkQueue(*mut core::ffi::c_void);
unsafe impl Send for VkQueue {}
unsafe impl Sync for VkQueue {}
impl Default for VkQueue {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkQueue {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}

/// Khronos: [VkCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBuffer.html) (handle)
/// * Parent: [VkCommandPool]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkCommandBuffer(*mut core::ffi::c_void);
unsafe impl Send for VkCommandBuffer {}
unsafe impl Sync for VkCommandBuffer {}
impl Default for VkCommandBuffer {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkCommandBuffer {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}

/// Khronos: [VkDeviceMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemory.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDeviceMemory(u64);
impl VkDeviceMemory {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPool.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkCommandPool(u64);
impl VkCommandPool {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuffer.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkBuffer(u64);
impl VkBuffer {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferView.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkBufferView(u64);
impl VkBufferView {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImage.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkImage(u64);
impl VkImage {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageView.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkImageView(u64);
impl VkImageView {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModule.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkShaderModule(u64);
impl VkShaderModule {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipeline.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPipeline(u64);
impl VkPipeline {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkPipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayout.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPipelineLayout(u64);
impl VkPipelineLayout {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkSampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampler.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSampler(u64);
impl VkSampler {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSet.html) (non-dispatchable handle)
/// * Parent: [VkDescriptorPool]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDescriptorSet(u64);
impl VkDescriptorSet {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayout.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDescriptorSetLayout(u64);
impl VkDescriptorSetLayout {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPool.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDescriptorPool(u64);
impl VkDescriptorPool {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFence.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkFence(u64);
impl VkFence {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphore.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSemaphore(u64);
impl VkSemaphore {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEvent.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkEvent(u64);
impl VkEvent {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPool.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkQueryPool(u64);
impl VkQueryPool {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebuffer.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkFramebuffer(u64);
impl VkFramebuffer {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPass.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkRenderPass(u64);
impl VkRenderPass {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkPipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCache.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPipelineCache(u64);
impl VkPipelineCache {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutNV.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkIndirectCommandsLayoutNV(u64);
impl VkIndirectCommandsLayoutNV {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplate.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDescriptorUpdateTemplate(u64);
impl VkDescriptorUpdateTemplate {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkValidationCacheEXT.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkValidationCacheEXT(u64);
impl VkValidationCacheEXT {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureKHR.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkAccelerationStructureKHR(u64);
impl VkAccelerationStructureKHR {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureNV.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkAccelerationStructureNV(u64);
impl VkAccelerationStructureNV {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkPerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationINTEL.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPerformanceConfigurationINTEL(u64);
impl VkPerformanceConfigurationINTEL {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkBufferCollectionFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionFUCHSIA.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkBufferCollectionFUCHSIA(u64);
impl VkBufferCollectionFUCHSIA {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeferredOperationKHR.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDeferredOperationKHR(u64);
impl VkDeferredOperationKHR {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkPrivateDataSlot](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlot.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPrivateDataSlot(u64);
impl VkPrivateDataSlot {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuModuleNVX.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkCuModuleNVX(u64);
impl VkCuModuleNVX {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCuFunctionNVX.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkCuFunctionNVX(u64);
impl VkCuFunctionNVX {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkOpticalFlowSessionNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionNV.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkOpticalFlowSessionNV(u64);
impl VkOpticalFlowSessionNV {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMicromapEXT.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkMicromapEXT(u64);
impl VkMicromapEXT {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkDisplayKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayKHR.html) (non-dispatchable handle)
/// * Parent: [VkPhysicalDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDisplayKHR(u64);
impl VkDisplayKHR {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkDisplayModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeKHR.html) (non-dispatchable handle)
/// * Parent: [VkDisplayKHR]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDisplayModeKHR(u64);
impl VkDisplayModeKHR {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html) (non-dispatchable handle)
/// * Parent: [VkInstance]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSurfaceKHR(u64);
impl VkSurfaceKHR {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkSwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainKHR.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSwapchainKHR(u64);
impl VkSwapchainKHR {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackEXT.html) (non-dispatchable handle)
/// * Parent: [VkInstance]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDebugReportCallbackEXT(u64);
impl VkDebugReportCallbackEXT {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}

/// Khronos: [VkDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerEXT.html) (non-dispatchable handle)
/// * Parent: [VkInstance]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDebugUtilsMessengerEXT(u64);
impl VkDebugUtilsMessengerEXT {
  pub const ZERO: Self = Self::zero();
  #[inline]
  #[must_use]
  pub const fn zero() -> Self {
    Self(0)
  }
}
