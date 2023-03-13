#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use crate::prelude::*;

/// Khronos: [vkAcquireDrmDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html) (non-nullable)
pub type vkAcquireDrmDisplayEXT_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  drm_fd: i32,
  display: VkDisplayKHR,
) -> VkResult;
/// Khronos: [vkAcquireDrmDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html) (nullable)
pub type PFN_vkAcquireDrmDisplayEXT = Option<vkAcquireDrmDisplayEXT_t>;
const vkAcquireDrmDisplayEXT_NAME: &str = "vkAcquireDrmDisplayEXT\0";

/// Khronos: [vkAcquireFullScreenExclusiveModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) (non-nullable)
pub type vkAcquireFullScreenExclusiveModeEXT_t =
  unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
/// Khronos: [vkAcquireFullScreenExclusiveModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) (nullable)
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
  Option<vkAcquireFullScreenExclusiveModeEXT_t>;
const vkAcquireFullScreenExclusiveModeEXT_NAME: &str =
  "vkAcquireFullScreenExclusiveModeEXT\0";

/// Khronos: [vkAcquireImageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireImageANDROID.html) (non-nullable)
pub type vkAcquireImageANDROID_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  native_fence_fd: c_int,
  semaphore: VkSemaphore,
  fence: VkFence,
) -> VkResult;
/// Khronos: [vkAcquireImageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireImageANDROID.html) (nullable)
pub type PFN_vkAcquireImageANDROID = Option<vkAcquireImageANDROID_t>;
const vkAcquireImageANDROID_NAME: &str = "vkAcquireImageANDROID\0";

/// Khronos: [vkAcquireNextImage2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html) (non-nullable)
pub type vkAcquireNextImage2KHR_t = unsafe extern "system" fn(
  device: VkDevice,
  acquire_info: *const VkAcquireNextImageInfoKHR,
  image_index: *mut u32,
) -> VkResult;
/// Khronos: [vkAcquireNextImage2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html) (nullable)
pub type PFN_vkAcquireNextImage2KHR = Option<vkAcquireNextImage2KHR_t>;
const vkAcquireNextImage2KHR_NAME: &str = "vkAcquireNextImage2KHR\0";

/// Khronos: [vkAcquireNextImageKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html) (non-nullable)
pub type vkAcquireNextImageKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  timeout: u64,
  semaphore: VkSemaphore,
  fence: VkFence,
  image_index: *mut u32,
) -> VkResult;
/// Khronos: [vkAcquireNextImageKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html) (nullable)
pub type PFN_vkAcquireNextImageKHR = Option<vkAcquireNextImageKHR_t>;
const vkAcquireNextImageKHR_NAME: &str = "vkAcquireNextImageKHR\0";

/// Khronos: [vkAcquirePerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) (non-nullable)
pub type vkAcquirePerformanceConfigurationINTEL_t = unsafe extern "system" fn(
  device: VkDevice,
  acquire_info: *const VkPerformanceConfigurationAcquireInfoINTEL,
  configuration: *mut VkPerformanceConfigurationINTEL,
)
  -> VkResult;
/// Khronos: [vkAcquirePerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) (nullable)
pub type PFN_vkAcquirePerformanceConfigurationINTEL =
  Option<vkAcquirePerformanceConfigurationINTEL_t>;
const vkAcquirePerformanceConfigurationINTEL_NAME: &str =
  "vkAcquirePerformanceConfigurationINTEL\0";

/// Khronos: [vkAcquireProfilingLockKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html) (non-nullable)
pub type vkAcquireProfilingLockKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkAcquireProfilingLockInfoKHR,
) -> VkResult;
/// Khronos: [vkAcquireProfilingLockKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html) (nullable)
pub type PFN_vkAcquireProfilingLockKHR = Option<vkAcquireProfilingLockKHR_t>;
const vkAcquireProfilingLockKHR_NAME: &str = "vkAcquireProfilingLockKHR\0";

/// Khronos: [vkAcquireWinrtDisplayNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html) (non-nullable)
pub type vkAcquireWinrtDisplayNV_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  display: VkDisplayKHR,
) -> VkResult;
/// Khronos: [vkAcquireWinrtDisplayNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html) (nullable)
pub type PFN_vkAcquireWinrtDisplayNV = Option<vkAcquireWinrtDisplayNV_t>;
const vkAcquireWinrtDisplayNV_NAME: &str = "vkAcquireWinrtDisplayNV\0";

/// Khronos: [vkAcquireXlibDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html) (non-nullable)
pub type vkAcquireXlibDisplayEXT_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  dpy: *mut XlibDisplay,
  display: VkDisplayKHR,
) -> VkResult;
/// Khronos: [vkAcquireXlibDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html) (nullable)
pub type PFN_vkAcquireXlibDisplayEXT = Option<vkAcquireXlibDisplayEXT_t>;
const vkAcquireXlibDisplayEXT_NAME: &str = "vkAcquireXlibDisplayEXT\0";

/// Khronos: [vkAllocateCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html) (non-nullable)
pub type vkAllocateCommandBuffers_t = unsafe extern "system" fn(
  device: VkDevice,
  allocate_info: *const VkCommandBufferAllocateInfo,
  command_buffers: *mut VkCommandBuffer,
) -> VkResult;
/// Khronos: [vkAllocateCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html) (nullable)
pub type PFN_vkAllocateCommandBuffers = Option<vkAllocateCommandBuffers_t>;
const vkAllocateCommandBuffers_NAME: &str = "vkAllocateCommandBuffers\0";

/// Khronos: [vkAllocateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html) (non-nullable)
pub type vkAllocateDescriptorSets_t = unsafe extern "system" fn(
  device: VkDevice,
  allocate_info: *const VkDescriptorSetAllocateInfo,
  descriptor_sets: *mut VkDescriptorSet,
) -> VkResult;
/// Khronos: [vkAllocateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html) (nullable)
pub type PFN_vkAllocateDescriptorSets = Option<vkAllocateDescriptorSets_t>;
const vkAllocateDescriptorSets_NAME: &str = "vkAllocateDescriptorSets\0";

/// Khronos: [vkAllocateMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html) (non-nullable)
pub type vkAllocateMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  allocate_info: *const VkMemoryAllocateInfo,
  allocator: *const VkAllocationCallbacks,
  memory: *mut VkDeviceMemory,
) -> VkResult;
/// Khronos: [vkAllocateMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html) (nullable)
pub type PFN_vkAllocateMemory = Option<vkAllocateMemory_t>;
const vkAllocateMemory_NAME: &str = "vkAllocateMemory\0";

/// Khronos: [vkBeginCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html) (non-nullable)
pub type vkBeginCommandBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  begin_info: *const VkCommandBufferBeginInfo,
) -> VkResult;
/// Khronos: [vkBeginCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html) (nullable)
pub type PFN_vkBeginCommandBuffer = Option<vkBeginCommandBuffer_t>;
const vkBeginCommandBuffer_NAME: &str = "vkBeginCommandBuffer\0";

/// Khronos: [vkBindAccelerationStructureMemoryNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) (non-nullable)
pub type vkBindAccelerationStructureMemoryNV_t = unsafe extern "system" fn(
  device: VkDevice,
  bind_info_count: u32,
  bind_infos: *const VkBindAccelerationStructureMemoryInfoNV,
) -> VkResult;
/// Khronos: [vkBindAccelerationStructureMemoryNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) (nullable)
pub type PFN_vkBindAccelerationStructureMemoryNV =
  Option<vkBindAccelerationStructureMemoryNV_t>;
const vkBindAccelerationStructureMemoryNV_NAME: &str =
  "vkBindAccelerationStructureMemoryNV\0";

/// Khronos: [vkBindBufferMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html) (non-nullable)
pub type vkBindBufferMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  buffer: VkBuffer,
  memory: VkDeviceMemory,
  memory_offset: VkDeviceSize,
) -> VkResult;
/// Khronos: [vkBindBufferMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html) (nullable)
pub type PFN_vkBindBufferMemory = Option<vkBindBufferMemory_t>;
const vkBindBufferMemory_NAME: &str = "vkBindBufferMemory\0";

/// Khronos: [vkBindBufferMemory2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html) (non-nullable)
pub type vkBindBufferMemory2_t = unsafe extern "system" fn(
  device: VkDevice,
  bind_info_count: u32,
  bind_infos: *const VkBindBufferMemoryInfo,
) -> VkResult;
/// Khronos: [vkBindBufferMemory2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html) (nullable)
pub type PFN_vkBindBufferMemory2 = Option<vkBindBufferMemory2_t>;
const vkBindBufferMemory2_NAME: &str = "vkBindBufferMemory2\0";

/// Khronos: [vkBindImageMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html) (non-nullable)
pub type vkBindImageMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  memory: VkDeviceMemory,
  memory_offset: VkDeviceSize,
) -> VkResult;
/// Khronos: [vkBindImageMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html) (nullable)
pub type PFN_vkBindImageMemory = Option<vkBindImageMemory_t>;
const vkBindImageMemory_NAME: &str = "vkBindImageMemory\0";

/// Khronos: [vkBindImageMemory2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html) (non-nullable)
pub type vkBindImageMemory2_t = unsafe extern "system" fn(
  device: VkDevice,
  bind_info_count: u32,
  bind_infos: *const VkBindImageMemoryInfo,
) -> VkResult;
/// Khronos: [vkBindImageMemory2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html) (nullable)
pub type PFN_vkBindImageMemory2 = Option<vkBindImageMemory2_t>;
const vkBindImageMemory2_NAME: &str = "vkBindImageMemory2\0";

/// Khronos: [vkBindOpticalFlowSessionImageNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindOpticalFlowSessionImageNV.html) (non-nullable)
pub type vkBindOpticalFlowSessionImageNV_t = unsafe extern "system" fn(
  device: VkDevice,
  session: VkOpticalFlowSessionNV,
  binding_point: VkOpticalFlowSessionBindingPointNV,
  view: VkImageView,
  layout: VkImageLayout,
) -> VkResult;
/// Khronos: [vkBindOpticalFlowSessionImageNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindOpticalFlowSessionImageNV.html) (nullable)
pub type PFN_vkBindOpticalFlowSessionImageNV = Option<vkBindOpticalFlowSessionImageNV_t>;
const vkBindOpticalFlowSessionImageNV_NAME: &str = "vkBindOpticalFlowSessionImageNV\0";

/// Khronos: [vkBindVideoSessionMemoryKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html) (non-nullable)
pub type vkBindVideoSessionMemoryKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  video_session: VkVideoSessionKHR,
  bind_session_memory_info_count: u32,
  bind_session_memory_infos: *const VkBindVideoSessionMemoryInfoKHR,
) -> VkResult;
/// Khronos: [vkBindVideoSessionMemoryKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html) (nullable)
pub type PFN_vkBindVideoSessionMemoryKHR = Option<vkBindVideoSessionMemoryKHR_t>;
const vkBindVideoSessionMemoryKHR_NAME: &str = "vkBindVideoSessionMemoryKHR\0";

/// Khronos: [vkCmdBeginConditionalRenderingEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) (non-nullable)
pub type vkCmdBeginConditionalRenderingEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  conditional_rendering_begin: *const VkConditionalRenderingBeginInfoEXT,
);
/// Khronos: [vkCmdBeginConditionalRenderingEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) (nullable)
pub type PFN_vkCmdBeginConditionalRenderingEXT =
  Option<vkCmdBeginConditionalRenderingEXT_t>;
const vkCmdBeginConditionalRenderingEXT_NAME: &str =
  "vkCmdBeginConditionalRenderingEXT\0";

/// Khronos: [vkCmdBeginDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html) (non-nullable)
pub type vkCmdBeginDebugUtilsLabelEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  label_info: *const VkDebugUtilsLabelEXT,
);
/// Khronos: [vkCmdBeginDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html) (nullable)
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = Option<vkCmdBeginDebugUtilsLabelEXT_t>;
const vkCmdBeginDebugUtilsLabelEXT_NAME: &str = "vkCmdBeginDebugUtilsLabelEXT\0";

/// Khronos: [vkCmdBeginQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html) (non-nullable)
pub type vkCmdBeginQuery_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  query: u32,
  flags: VkQueryControlFlags,
);
/// Khronos: [vkCmdBeginQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html) (nullable)
pub type PFN_vkCmdBeginQuery = Option<vkCmdBeginQuery_t>;
const vkCmdBeginQuery_NAME: &str = "vkCmdBeginQuery\0";

/// Khronos: [vkCmdBeginQueryIndexedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html) (non-nullable)
pub type vkCmdBeginQueryIndexedEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  query: u32,
  flags: VkQueryControlFlags,
  index: u32,
);
/// Khronos: [vkCmdBeginQueryIndexedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html) (nullable)
pub type PFN_vkCmdBeginQueryIndexedEXT = Option<vkCmdBeginQueryIndexedEXT_t>;
const vkCmdBeginQueryIndexedEXT_NAME: &str = "vkCmdBeginQueryIndexedEXT\0";

/// Khronos: [vkCmdBeginRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html) (non-nullable)
pub type vkCmdBeginRenderPass_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  render_pass_begin: *const VkRenderPassBeginInfo,
  contents: VkSubpassContents,
);
/// Khronos: [vkCmdBeginRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html) (nullable)
pub type PFN_vkCmdBeginRenderPass = Option<vkCmdBeginRenderPass_t>;
const vkCmdBeginRenderPass_NAME: &str = "vkCmdBeginRenderPass\0";

/// Khronos: [vkCmdBeginRenderPass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html) (non-nullable)
pub type vkCmdBeginRenderPass2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  render_pass_begin: *const VkRenderPassBeginInfo,
  subpass_begin_info: *const VkSubpassBeginInfo,
);
/// Khronos: [vkCmdBeginRenderPass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html) (nullable)
pub type PFN_vkCmdBeginRenderPass2 = Option<vkCmdBeginRenderPass2_t>;
const vkCmdBeginRenderPass2_NAME: &str = "vkCmdBeginRenderPass2\0";

/// Khronos: [vkCmdBeginRendering](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html) (non-nullable)
pub type vkCmdBeginRendering_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  rendering_info: *const VkRenderingInfo,
);
/// Khronos: [vkCmdBeginRendering](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html) (nullable)
pub type PFN_vkCmdBeginRendering = Option<vkCmdBeginRendering_t>;
const vkCmdBeginRendering_NAME: &str = "vkCmdBeginRendering\0";

/// Khronos: [vkCmdBeginTransformFeedbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html) (non-nullable)
pub type vkCmdBeginTransformFeedbackEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_counter_buffer: u32,
  counter_buffer_count: u32,
  counter_buffers: *const VkBuffer,
  counter_buffer_offsets: *const VkDeviceSize,
);
/// Khronos: [vkCmdBeginTransformFeedbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html) (nullable)
pub type PFN_vkCmdBeginTransformFeedbackEXT = Option<vkCmdBeginTransformFeedbackEXT_t>;
const vkCmdBeginTransformFeedbackEXT_NAME: &str = "vkCmdBeginTransformFeedbackEXT\0";

/// Khronos: [vkCmdBeginVideoCodingKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html) (non-nullable)
pub type vkCmdBeginVideoCodingKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  begin_info: *const VkVideoBeginCodingInfoKHR,
);
/// Khronos: [vkCmdBeginVideoCodingKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html) (nullable)
pub type PFN_vkCmdBeginVideoCodingKHR = Option<vkCmdBeginVideoCodingKHR_t>;
const vkCmdBeginVideoCodingKHR_NAME: &str = "vkCmdBeginVideoCodingKHR\0";

/// Khronos: [vkCmdBindDescriptorBufferEmbeddedSamplersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html) (non-nullable)
pub type vkCmdBindDescriptorBufferEmbeddedSamplersEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_bind_point: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  set: u32,
);
/// Khronos: [vkCmdBindDescriptorBufferEmbeddedSamplersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html) (nullable)
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT =
  Option<vkCmdBindDescriptorBufferEmbeddedSamplersEXT_t>;
const vkCmdBindDescriptorBufferEmbeddedSamplersEXT_NAME: &str =
  "vkCmdBindDescriptorBufferEmbeddedSamplersEXT\0";

/// Khronos: [vkCmdBindDescriptorBuffersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBuffersEXT.html) (non-nullable)
pub type vkCmdBindDescriptorBuffersEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer_count: u32,
  binding_infos: *const VkDescriptorBufferBindingInfoEXT,
);
/// Khronos: [vkCmdBindDescriptorBuffersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBuffersEXT.html) (nullable)
pub type PFN_vkCmdBindDescriptorBuffersEXT = Option<vkCmdBindDescriptorBuffersEXT_t>;
const vkCmdBindDescriptorBuffersEXT_NAME: &str = "vkCmdBindDescriptorBuffersEXT\0";

/// Khronos: [vkCmdBindDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html) (non-nullable)
pub type vkCmdBindDescriptorSets_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_bind_point: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  first_set: u32,
  descriptor_set_count: u32,
  descriptor_sets: *const VkDescriptorSet,
  dynamic_offset_count: u32,
  dynamic_offsets: *const u32,
);
/// Khronos: [vkCmdBindDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html) (nullable)
pub type PFN_vkCmdBindDescriptorSets = Option<vkCmdBindDescriptorSets_t>;
const vkCmdBindDescriptorSets_NAME: &str = "vkCmdBindDescriptorSets\0";

/// Khronos: [vkCmdBindIndexBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html) (non-nullable)
pub type vkCmdBindIndexBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  index_type: VkIndexType,
);
/// Khronos: [vkCmdBindIndexBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html) (nullable)
pub type PFN_vkCmdBindIndexBuffer = Option<vkCmdBindIndexBuffer_t>;
const vkCmdBindIndexBuffer_NAME: &str = "vkCmdBindIndexBuffer\0";

/// Khronos: [vkCmdBindInvocationMaskHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html) (non-nullable)
pub type vkCmdBindInvocationMaskHUAWEI_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  image_view: VkImageView,
  image_layout: VkImageLayout,
);
/// Khronos: [vkCmdBindInvocationMaskHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html) (nullable)
pub type PFN_vkCmdBindInvocationMaskHUAWEI = Option<vkCmdBindInvocationMaskHUAWEI_t>;
const vkCmdBindInvocationMaskHUAWEI_NAME: &str = "vkCmdBindInvocationMaskHUAWEI\0";

/// Khronos: [vkCmdBindPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html) (non-nullable)
pub type vkCmdBindPipeline_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_bind_point: VkPipelineBindPoint,
  pipeline: VkPipeline,
);
/// Khronos: [vkCmdBindPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html) (nullable)
pub type PFN_vkCmdBindPipeline = Option<vkCmdBindPipeline_t>;
const vkCmdBindPipeline_NAME: &str = "vkCmdBindPipeline\0";

/// Khronos: [vkCmdBindPipelineShaderGroupNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html) (non-nullable)
pub type vkCmdBindPipelineShaderGroupNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_bind_point: VkPipelineBindPoint,
  pipeline: VkPipeline,
  group_index: u32,
);
/// Khronos: [vkCmdBindPipelineShaderGroupNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html) (nullable)
pub type PFN_vkCmdBindPipelineShaderGroupNV = Option<vkCmdBindPipelineShaderGroupNV_t>;
const vkCmdBindPipelineShaderGroupNV_NAME: &str = "vkCmdBindPipelineShaderGroupNV\0";

/// Khronos: [vkCmdBindShadingRateImageNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadingRateImageNV.html) (non-nullable)
pub type vkCmdBindShadingRateImageNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  image_view: VkImageView,
  image_layout: VkImageLayout,
);
/// Khronos: [vkCmdBindShadingRateImageNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadingRateImageNV.html) (nullable)
pub type PFN_vkCmdBindShadingRateImageNV = Option<vkCmdBindShadingRateImageNV_t>;
const vkCmdBindShadingRateImageNV_NAME: &str = "vkCmdBindShadingRateImageNV\0";

/// Khronos: [vkCmdBindTransformFeedbackBuffersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html) (non-nullable)
pub type vkCmdBindTransformFeedbackBuffersEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_binding: u32,
  binding_count: u32,
  buffers: *const VkBuffer,
  offsets: *const VkDeviceSize,
  sizes: *const VkDeviceSize,
);
/// Khronos: [vkCmdBindTransformFeedbackBuffersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html) (nullable)
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT =
  Option<vkCmdBindTransformFeedbackBuffersEXT_t>;
const vkCmdBindTransformFeedbackBuffersEXT_NAME: &str =
  "vkCmdBindTransformFeedbackBuffersEXT\0";

/// Khronos: [vkCmdBindVertexBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html) (non-nullable)
pub type vkCmdBindVertexBuffers_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_binding: u32,
  binding_count: u32,
  buffers: *const VkBuffer,
  offsets: *const VkDeviceSize,
);
/// Khronos: [vkCmdBindVertexBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html) (nullable)
pub type PFN_vkCmdBindVertexBuffers = Option<vkCmdBindVertexBuffers_t>;
const vkCmdBindVertexBuffers_NAME: &str = "vkCmdBindVertexBuffers\0";

/// Khronos: [vkCmdBindVertexBuffers2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2.html) (non-nullable)
pub type vkCmdBindVertexBuffers2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_binding: u32,
  binding_count: u32,
  buffers: *const VkBuffer,
  offsets: *const VkDeviceSize,
  sizes: *const VkDeviceSize,
  strides: *const VkDeviceSize,
);
/// Khronos: [vkCmdBindVertexBuffers2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2.html) (nullable)
pub type PFN_vkCmdBindVertexBuffers2 = Option<vkCmdBindVertexBuffers2_t>;
const vkCmdBindVertexBuffers2_NAME: &str = "vkCmdBindVertexBuffers2\0";

/// Khronos: [vkCmdBlitImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html) (non-nullable)
pub type vkCmdBlitImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_image: VkImage,
  src_image_layout: VkImageLayout,
  dst_image: VkImage,
  dst_image_layout: VkImageLayout,
  region_count: u32,
  regions: *const VkImageBlit,
  filter: VkFilter,
);
/// Khronos: [vkCmdBlitImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html) (nullable)
pub type PFN_vkCmdBlitImage = Option<vkCmdBlitImage_t>;
const vkCmdBlitImage_NAME: &str = "vkCmdBlitImage\0";

/// Khronos: [vkCmdBlitImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2.html) (non-nullable)
pub type vkCmdBlitImage2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  blit_image_info: *const VkBlitImageInfo2,
);
/// Khronos: [vkCmdBlitImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2.html) (nullable)
pub type PFN_vkCmdBlitImage2 = Option<vkCmdBlitImage2_t>;
const vkCmdBlitImage2_NAME: &str = "vkCmdBlitImage2\0";

/// Khronos: [vkCmdBuildAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) (non-nullable)
pub type vkCmdBuildAccelerationStructureNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  info: *const VkAccelerationStructureInfoNV,
  instance_data: VkBuffer,
  instance_offset: VkDeviceSize,
  update: VkBool32,
  dst: VkAccelerationStructureNV,
  src: VkAccelerationStructureNV,
  scratch: VkBuffer,
  scratch_offset: VkDeviceSize,
);
/// Khronos: [vkCmdBuildAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) (nullable)
pub type PFN_vkCmdBuildAccelerationStructureNV =
  Option<vkCmdBuildAccelerationStructureNV_t>;
const vkCmdBuildAccelerationStructureNV_NAME: &str =
  "vkCmdBuildAccelerationStructureNV\0";

/// Khronos: [vkCmdClearAttachments](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html) (non-nullable)
pub type vkCmdClearAttachments_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  attachment_count: u32,
  attachments: *const VkClearAttachment,
  rect_count: u32,
  rects: *const VkClearRect,
);
/// Khronos: [vkCmdClearAttachments](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html) (nullable)
pub type PFN_vkCmdClearAttachments = Option<vkCmdClearAttachments_t>;
const vkCmdClearAttachments_NAME: &str = "vkCmdClearAttachments\0";

/// Khronos: [vkCmdClearColorImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html) (non-nullable)
pub type vkCmdClearColorImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  image: VkImage,
  image_layout: VkImageLayout,
  color: *const VkClearColorValue,
  range_count: u32,
  ranges: *const VkImageSubresourceRange,
);
/// Khronos: [vkCmdClearColorImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html) (nullable)
pub type PFN_vkCmdClearColorImage = Option<vkCmdClearColorImage_t>;
const vkCmdClearColorImage_NAME: &str = "vkCmdClearColorImage\0";

/// Khronos: [vkCmdClearDepthStencilImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html) (non-nullable)
pub type vkCmdClearDepthStencilImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  image: VkImage,
  image_layout: VkImageLayout,
  depth_stencil: *const VkClearDepthStencilValue,
  range_count: u32,
  ranges: *const VkImageSubresourceRange,
);
/// Khronos: [vkCmdClearDepthStencilImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html) (nullable)
pub type PFN_vkCmdClearDepthStencilImage = Option<vkCmdClearDepthStencilImage_t>;
const vkCmdClearDepthStencilImage_NAME: &str = "vkCmdClearDepthStencilImage\0";

/// Khronos: [vkCmdControlVideoCodingKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html) (non-nullable)
pub type vkCmdControlVideoCodingKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  coding_control_info: *const VkVideoCodingControlInfoKHR,
);
/// Khronos: [vkCmdControlVideoCodingKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html) (nullable)
pub type PFN_vkCmdControlVideoCodingKHR = Option<vkCmdControlVideoCodingKHR_t>;
const vkCmdControlVideoCodingKHR_NAME: &str = "vkCmdControlVideoCodingKHR\0";

/// Khronos: [vkCmdCopyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html) (non-nullable)
pub type vkCmdCopyAccelerationStructureKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  info: *const VkCopyAccelerationStructureInfoKHR,
);
/// Khronos: [vkCmdCopyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html) (nullable)
pub type PFN_vkCmdCopyAccelerationStructureKHR =
  Option<vkCmdCopyAccelerationStructureKHR_t>;
const vkCmdCopyAccelerationStructureKHR_NAME: &str =
  "vkCmdCopyAccelerationStructureKHR\0";

/// Khronos: [vkCmdCopyAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) (non-nullable)
pub type vkCmdCopyAccelerationStructureNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  dst: VkAccelerationStructureNV,
  src: VkAccelerationStructureNV,
  mode: VkCopyAccelerationStructureModeKHR,
);
/// Khronos: [vkCmdCopyAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) (nullable)
pub type PFN_vkCmdCopyAccelerationStructureNV =
  Option<vkCmdCopyAccelerationStructureNV_t>;
const vkCmdCopyAccelerationStructureNV_NAME: &str = "vkCmdCopyAccelerationStructureNV\0";

/// Khronos: [vkCmdCopyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html) (non-nullable)
pub type vkCmdCopyBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_buffer: VkBuffer,
  dst_buffer: VkBuffer,
  region_count: u32,
  regions: *const VkBufferCopy,
);
/// Khronos: [vkCmdCopyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html) (nullable)
pub type PFN_vkCmdCopyBuffer = Option<vkCmdCopyBuffer_t>;
const vkCmdCopyBuffer_NAME: &str = "vkCmdCopyBuffer\0";

/// Khronos: [vkCmdCopyBuffer2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html) (non-nullable)
pub type vkCmdCopyBuffer2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  copy_buffer_info: *const VkCopyBufferInfo2,
);
/// Khronos: [vkCmdCopyBuffer2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html) (nullable)
pub type PFN_vkCmdCopyBuffer2 = Option<vkCmdCopyBuffer2_t>;
const vkCmdCopyBuffer2_NAME: &str = "vkCmdCopyBuffer2\0";

/// Khronos: [vkCmdCopyBufferToImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html) (non-nullable)
pub type vkCmdCopyBufferToImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_buffer: VkBuffer,
  dst_image: VkImage,
  dst_image_layout: VkImageLayout,
  region_count: u32,
  regions: *const VkBufferImageCopy,
);
/// Khronos: [vkCmdCopyBufferToImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html) (nullable)
pub type PFN_vkCmdCopyBufferToImage = Option<vkCmdCopyBufferToImage_t>;
const vkCmdCopyBufferToImage_NAME: &str = "vkCmdCopyBufferToImage\0";

/// Khronos: [vkCmdCopyBufferToImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html) (non-nullable)
pub type vkCmdCopyBufferToImage2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  copy_buffer_to_image_info: *const VkCopyBufferToImageInfo2,
);
/// Khronos: [vkCmdCopyBufferToImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html) (nullable)
pub type PFN_vkCmdCopyBufferToImage2 = Option<vkCmdCopyBufferToImage2_t>;
const vkCmdCopyBufferToImage2_NAME: &str = "vkCmdCopyBufferToImage2\0";

/// Khronos: [vkCmdCopyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html) (non-nullable)
pub type vkCmdCopyImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_image: VkImage,
  src_image_layout: VkImageLayout,
  dst_image: VkImage,
  dst_image_layout: VkImageLayout,
  region_count: u32,
  regions: *const VkImageCopy,
);
/// Khronos: [vkCmdCopyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html) (nullable)
pub type PFN_vkCmdCopyImage = Option<vkCmdCopyImage_t>;
const vkCmdCopyImage_NAME: &str = "vkCmdCopyImage\0";

/// Khronos: [vkCmdCopyImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html) (non-nullable)
pub type vkCmdCopyImage2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  copy_image_info: *const VkCopyImageInfo2,
);
/// Khronos: [vkCmdCopyImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html) (nullable)
pub type PFN_vkCmdCopyImage2 = Option<vkCmdCopyImage2_t>;
const vkCmdCopyImage2_NAME: &str = "vkCmdCopyImage2\0";

/// Khronos: [vkCmdCopyImageToBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html) (non-nullable)
pub type vkCmdCopyImageToBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_image: VkImage,
  src_image_layout: VkImageLayout,
  dst_buffer: VkBuffer,
  region_count: u32,
  regions: *const VkBufferImageCopy,
);
/// Khronos: [vkCmdCopyImageToBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html) (nullable)
pub type PFN_vkCmdCopyImageToBuffer = Option<vkCmdCopyImageToBuffer_t>;
const vkCmdCopyImageToBuffer_NAME: &str = "vkCmdCopyImageToBuffer\0";

/// Khronos: [vkCmdCopyImageToBuffer2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html) (non-nullable)
pub type vkCmdCopyImageToBuffer2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  copy_image_to_buffer_info: *const VkCopyImageToBufferInfo2,
);
/// Khronos: [vkCmdCopyImageToBuffer2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html) (nullable)
pub type PFN_vkCmdCopyImageToBuffer2 = Option<vkCmdCopyImageToBuffer2_t>;
const vkCmdCopyImageToBuffer2_NAME: &str = "vkCmdCopyImageToBuffer2\0";

/// Khronos: [vkCmdCopyMemoryIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryIndirectNV.html) (non-nullable)
pub type vkCmdCopyMemoryIndirectNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  copy_buffer_address: VkDeviceAddress,
  copy_count: u32,
  stride: u32,
);
/// Khronos: [vkCmdCopyMemoryIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryIndirectNV.html) (nullable)
pub type PFN_vkCmdCopyMemoryIndirectNV = Option<vkCmdCopyMemoryIndirectNV_t>;
const vkCmdCopyMemoryIndirectNV_NAME: &str = "vkCmdCopyMemoryIndirectNV\0";

/// Khronos: [vkCmdCopyMemoryToImageIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToImageIndirectNV.html) (non-nullable)
pub type vkCmdCopyMemoryToImageIndirectNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  copy_buffer_address: VkDeviceAddress,
  copy_count: u32,
  stride: u32,
  dst_image: VkImage,
  dst_image_layout: VkImageLayout,
  image_subresources: *const VkImageSubresourceLayers,
);
/// Khronos: [vkCmdCopyMemoryToImageIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToImageIndirectNV.html) (nullable)
pub type PFN_vkCmdCopyMemoryToImageIndirectNV =
  Option<vkCmdCopyMemoryToImageIndirectNV_t>;
const vkCmdCopyMemoryToImageIndirectNV_NAME: &str = "vkCmdCopyMemoryToImageIndirectNV\0";

/// Khronos: [vkCmdCopyMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapEXT.html) (non-nullable)
pub type vkCmdCopyMicromapEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  info: *const VkCopyMicromapInfoEXT,
);
/// Khronos: [vkCmdCopyMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapEXT.html) (nullable)
pub type PFN_vkCmdCopyMicromapEXT = Option<vkCmdCopyMicromapEXT_t>;
const vkCmdCopyMicromapEXT_NAME: &str = "vkCmdCopyMicromapEXT\0";

/// Khronos: [vkCmdCopyQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html) (non-nullable)
pub type vkCmdCopyQueryPoolResults_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  first_query: u32,
  query_count: u32,
  dst_buffer: VkBuffer,
  dst_offset: VkDeviceSize,
  stride: VkDeviceSize,
  flags: VkQueryResultFlags,
);
/// Khronos: [vkCmdCopyQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html) (nullable)
pub type PFN_vkCmdCopyQueryPoolResults = Option<vkCmdCopyQueryPoolResults_t>;
const vkCmdCopyQueryPoolResults_NAME: &str = "vkCmdCopyQueryPoolResults\0";

/// Khronos: [vkCmdCuLaunchKernelNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html) (non-nullable)
pub type vkCmdCuLaunchKernelNVX_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  launch_info: *const VkCuLaunchInfoNVX,
);
/// Khronos: [vkCmdCuLaunchKernelNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html) (nullable)
pub type PFN_vkCmdCuLaunchKernelNVX = Option<vkCmdCuLaunchKernelNVX_t>;
const vkCmdCuLaunchKernelNVX_NAME: &str = "vkCmdCuLaunchKernelNVX\0";

/// Khronos: [vkCmdDebugMarkerBeginEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html) (non-nullable)
pub type vkCmdDebugMarkerBeginEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  marker_info: *const VkDebugMarkerMarkerInfoEXT,
);
/// Khronos: [vkCmdDebugMarkerBeginEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html) (nullable)
pub type PFN_vkCmdDebugMarkerBeginEXT = Option<vkCmdDebugMarkerBeginEXT_t>;
const vkCmdDebugMarkerBeginEXT_NAME: &str = "vkCmdDebugMarkerBeginEXT\0";

/// Khronos: [vkCmdDebugMarkerEndEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html) (non-nullable)
pub type vkCmdDebugMarkerEndEXT_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer);
/// Khronos: [vkCmdDebugMarkerEndEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html) (nullable)
pub type PFN_vkCmdDebugMarkerEndEXT = Option<vkCmdDebugMarkerEndEXT_t>;
const vkCmdDebugMarkerEndEXT_NAME: &str = "vkCmdDebugMarkerEndEXT\0";

/// Khronos: [vkCmdDebugMarkerInsertEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) (non-nullable)
pub type vkCmdDebugMarkerInsertEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  marker_info: *const VkDebugMarkerMarkerInfoEXT,
);
/// Khronos: [vkCmdDebugMarkerInsertEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) (nullable)
pub type PFN_vkCmdDebugMarkerInsertEXT = Option<vkCmdDebugMarkerInsertEXT_t>;
const vkCmdDebugMarkerInsertEXT_NAME: &str = "vkCmdDebugMarkerInsertEXT\0";

/// Khronos: [vkCmdDecodeVideoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecodeVideoKHR.html) (non-nullable)
pub type vkCmdDecodeVideoKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  decode_info: *const VkVideoDecodeInfoKHR,
);
/// Khronos: [vkCmdDecodeVideoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecodeVideoKHR.html) (nullable)
pub type PFN_vkCmdDecodeVideoKHR = Option<vkCmdDecodeVideoKHR_t>;
const vkCmdDecodeVideoKHR_NAME: &str = "vkCmdDecodeVideoKHR\0";

/// Khronos: [vkCmdDecompressMemoryIndirectCountNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryIndirectCountNV.html) (non-nullable)
pub type vkCmdDecompressMemoryIndirectCountNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  indirect_commands_address: VkDeviceAddress,
  indirect_commands_count_address: VkDeviceAddress,
  stride: u32,
);
/// Khronos: [vkCmdDecompressMemoryIndirectCountNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryIndirectCountNV.html) (nullable)
pub type PFN_vkCmdDecompressMemoryIndirectCountNV =
  Option<vkCmdDecompressMemoryIndirectCountNV_t>;
const vkCmdDecompressMemoryIndirectCountNV_NAME: &str =
  "vkCmdDecompressMemoryIndirectCountNV\0";

/// Khronos: [vkCmdDecompressMemoryNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryNV.html) (non-nullable)
pub type vkCmdDecompressMemoryNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  decompress_region_count: u32,
  decompress_memory_regions: *const VkDecompressMemoryRegionNV,
);
/// Khronos: [vkCmdDecompressMemoryNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryNV.html) (nullable)
pub type PFN_vkCmdDecompressMemoryNV = Option<vkCmdDecompressMemoryNV_t>;
const vkCmdDecompressMemoryNV_NAME: &str = "vkCmdDecompressMemoryNV\0";

/// Khronos: [vkCmdDispatch](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html) (non-nullable)
pub type vkCmdDispatch_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  group_count_x: u32,
  group_count_y: u32,
  group_count_z: u32,
);
/// Khronos: [vkCmdDispatch](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html) (nullable)
pub type PFN_vkCmdDispatch = Option<vkCmdDispatch_t>;
const vkCmdDispatch_NAME: &str = "vkCmdDispatch\0";

/// Khronos: [vkCmdDispatchBase](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html) (non-nullable)
pub type vkCmdDispatchBase_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  base_group_x: u32,
  base_group_y: u32,
  base_group_z: u32,
  group_count_x: u32,
  group_count_y: u32,
  group_count_z: u32,
);
/// Khronos: [vkCmdDispatchBase](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html) (nullable)
pub type PFN_vkCmdDispatchBase = Option<vkCmdDispatchBase_t>;
const vkCmdDispatchBase_NAME: &str = "vkCmdDispatchBase\0";

/// Khronos: [vkCmdDispatchIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html) (non-nullable)
pub type vkCmdDispatchIndirect_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
);
/// Khronos: [vkCmdDispatchIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html) (nullable)
pub type PFN_vkCmdDispatchIndirect = Option<vkCmdDispatchIndirect_t>;
const vkCmdDispatchIndirect_NAME: &str = "vkCmdDispatchIndirect\0";

/// Khronos: [vkCmdDraw](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html) (non-nullable)
pub type vkCmdDraw_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  vertex_count: u32,
  instance_count: u32,
  first_vertex: u32,
  first_instance: u32,
);
/// Khronos: [vkCmdDraw](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html) (nullable)
pub type PFN_vkCmdDraw = Option<vkCmdDraw_t>;
const vkCmdDraw_NAME: &str = "vkCmdDraw\0";

/// Khronos: [vkCmdDrawClusterHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterHUAWEI.html) (non-nullable)
pub type vkCmdDrawClusterHUAWEI_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  group_count_x: u32,
  group_count_y: u32,
  group_count_z: u32,
);
/// Khronos: [vkCmdDrawClusterHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterHUAWEI.html) (nullable)
pub type PFN_vkCmdDrawClusterHUAWEI = Option<vkCmdDrawClusterHUAWEI_t>;
const vkCmdDrawClusterHUAWEI_NAME: &str = "vkCmdDrawClusterHUAWEI\0";

/// Khronos: [vkCmdDrawClusterIndirectHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterIndirectHUAWEI.html) (non-nullable)
pub type vkCmdDrawClusterIndirectHUAWEI_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
);
/// Khronos: [vkCmdDrawClusterIndirectHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterIndirectHUAWEI.html) (nullable)
pub type PFN_vkCmdDrawClusterIndirectHUAWEI = Option<vkCmdDrawClusterIndirectHUAWEI_t>;
const vkCmdDrawClusterIndirectHUAWEI_NAME: &str = "vkCmdDrawClusterIndirectHUAWEI\0";

/// Khronos: [vkCmdDrawIndexed](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html) (non-nullable)
pub type vkCmdDrawIndexed_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  index_count: u32,
  instance_count: u32,
  first_index: u32,
  vertex_offset: i32,
  first_instance: u32,
);
/// Khronos: [vkCmdDrawIndexed](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html) (nullable)
pub type PFN_vkCmdDrawIndexed = Option<vkCmdDrawIndexed_t>;
const vkCmdDrawIndexed_NAME: &str = "vkCmdDrawIndexed\0";

/// Khronos: [vkCmdDrawIndexedIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html) (non-nullable)
pub type vkCmdDrawIndexedIndirect_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  draw_count: u32,
  stride: u32,
);
/// Khronos: [vkCmdDrawIndexedIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html) (nullable)
pub type PFN_vkCmdDrawIndexedIndirect = Option<vkCmdDrawIndexedIndirect_t>;
const vkCmdDrawIndexedIndirect_NAME: &str = "vkCmdDrawIndexedIndirect\0";

/// Khronos: [vkCmdDrawIndexedIndirectCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCount.html) (non-nullable)
pub type vkCmdDrawIndexedIndirectCount_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  count_buffer: VkBuffer,
  count_buffer_offset: VkDeviceSize,
  max_draw_count: u32,
  stride: u32,
);
/// Khronos: [vkCmdDrawIndexedIndirectCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCount.html) (nullable)
pub type PFN_vkCmdDrawIndexedIndirectCount = Option<vkCmdDrawIndexedIndirectCount_t>;
const vkCmdDrawIndexedIndirectCount_NAME: &str = "vkCmdDrawIndexedIndirectCount\0";

/// Khronos: [vkCmdDrawIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html) (non-nullable)
pub type vkCmdDrawIndirect_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  draw_count: u32,
  stride: u32,
);
/// Khronos: [vkCmdDrawIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html) (nullable)
pub type PFN_vkCmdDrawIndirect = Option<vkCmdDrawIndirect_t>;
const vkCmdDrawIndirect_NAME: &str = "vkCmdDrawIndirect\0";

/// Khronos: [vkCmdDrawIndirectByteCountEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html) (non-nullable)
pub type vkCmdDrawIndirectByteCountEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  instance_count: u32,
  first_instance: u32,
  counter_buffer: VkBuffer,
  counter_buffer_offset: VkDeviceSize,
  counter_offset: u32,
  vertex_stride: u32,
);
/// Khronos: [vkCmdDrawIndirectByteCountEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html) (nullable)
pub type PFN_vkCmdDrawIndirectByteCountEXT = Option<vkCmdDrawIndirectByteCountEXT_t>;
const vkCmdDrawIndirectByteCountEXT_NAME: &str = "vkCmdDrawIndirectByteCountEXT\0";

/// Khronos: [vkCmdDrawIndirectCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCount.html) (non-nullable)
pub type vkCmdDrawIndirectCount_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  count_buffer: VkBuffer,
  count_buffer_offset: VkDeviceSize,
  max_draw_count: u32,
  stride: u32,
);
/// Khronos: [vkCmdDrawIndirectCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCount.html) (nullable)
pub type PFN_vkCmdDrawIndirectCount = Option<vkCmdDrawIndirectCount_t>;
const vkCmdDrawIndirectCount_NAME: &str = "vkCmdDrawIndirectCount\0";

/// Khronos: [vkCmdDrawMeshTasksEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html) (non-nullable)
pub type vkCmdDrawMeshTasksEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  group_count_x: u32,
  group_count_y: u32,
  group_count_z: u32,
);
/// Khronos: [vkCmdDrawMeshTasksEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html) (nullable)
pub type PFN_vkCmdDrawMeshTasksEXT = Option<vkCmdDrawMeshTasksEXT_t>;
const vkCmdDrawMeshTasksEXT_NAME: &str = "vkCmdDrawMeshTasksEXT\0";

/// Khronos: [vkCmdDrawMeshTasksIndirectCountEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html) (non-nullable)
pub type vkCmdDrawMeshTasksIndirectCountEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  count_buffer: VkBuffer,
  count_buffer_offset: VkDeviceSize,
  max_draw_count: u32,
  stride: u32,
);
/// Khronos: [vkCmdDrawMeshTasksIndirectCountEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html) (nullable)
pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT =
  Option<vkCmdDrawMeshTasksIndirectCountEXT_t>;
const vkCmdDrawMeshTasksIndirectCountEXT_NAME: &str =
  "vkCmdDrawMeshTasksIndirectCountEXT\0";

/// Khronos: [vkCmdDrawMeshTasksIndirectCountNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) (non-nullable)
pub type vkCmdDrawMeshTasksIndirectCountNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  count_buffer: VkBuffer,
  count_buffer_offset: VkDeviceSize,
  max_draw_count: u32,
  stride: u32,
);
/// Khronos: [vkCmdDrawMeshTasksIndirectCountNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html) (nullable)
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV =
  Option<vkCmdDrawMeshTasksIndirectCountNV_t>;
const vkCmdDrawMeshTasksIndirectCountNV_NAME: &str =
  "vkCmdDrawMeshTasksIndirectCountNV\0";

/// Khronos: [vkCmdDrawMeshTasksIndirectEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectEXT.html) (non-nullable)
pub type vkCmdDrawMeshTasksIndirectEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  draw_count: u32,
  stride: u32,
);
/// Khronos: [vkCmdDrawMeshTasksIndirectEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectEXT.html) (nullable)
pub type PFN_vkCmdDrawMeshTasksIndirectEXT = Option<vkCmdDrawMeshTasksIndirectEXT_t>;
const vkCmdDrawMeshTasksIndirectEXT_NAME: &str = "vkCmdDrawMeshTasksIndirectEXT\0";

/// Khronos: [vkCmdDrawMeshTasksIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html) (non-nullable)
pub type vkCmdDrawMeshTasksIndirectNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  draw_count: u32,
  stride: u32,
);
/// Khronos: [vkCmdDrawMeshTasksIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html) (nullable)
pub type PFN_vkCmdDrawMeshTasksIndirectNV = Option<vkCmdDrawMeshTasksIndirectNV_t>;
const vkCmdDrawMeshTasksIndirectNV_NAME: &str = "vkCmdDrawMeshTasksIndirectNV\0";

/// Khronos: [vkCmdDrawMeshTasksNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html) (non-nullable)
pub type vkCmdDrawMeshTasksNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  task_count: u32,
  first_task: u32,
);
/// Khronos: [vkCmdDrawMeshTasksNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html) (nullable)
pub type PFN_vkCmdDrawMeshTasksNV = Option<vkCmdDrawMeshTasksNV_t>;
const vkCmdDrawMeshTasksNV_NAME: &str = "vkCmdDrawMeshTasksNV\0";

/// Khronos: [vkCmdDrawMultiEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html) (non-nullable)
pub type vkCmdDrawMultiEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  draw_count: u32,
  vertex_info: *const VkMultiDrawInfoEXT,
  instance_count: u32,
  first_instance: u32,
  stride: u32,
);
/// Khronos: [vkCmdDrawMultiEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html) (nullable)
pub type PFN_vkCmdDrawMultiEXT = Option<vkCmdDrawMultiEXT_t>;
const vkCmdDrawMultiEXT_NAME: &str = "vkCmdDrawMultiEXT\0";

/// Khronos: [vkCmdDrawMultiIndexedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html) (non-nullable)
pub type vkCmdDrawMultiIndexedEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  draw_count: u32,
  index_info: *const VkMultiDrawIndexedInfoEXT,
  instance_count: u32,
  first_instance: u32,
  stride: u32,
  vertex_offset: *const i32,
);
/// Khronos: [vkCmdDrawMultiIndexedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html) (nullable)
pub type PFN_vkCmdDrawMultiIndexedEXT = Option<vkCmdDrawMultiIndexedEXT_t>;
const vkCmdDrawMultiIndexedEXT_NAME: &str = "vkCmdDrawMultiIndexedEXT\0";

/// Khronos: [vkCmdEndConditionalRenderingEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) (non-nullable)
pub type vkCmdEndConditionalRenderingEXT_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer);
/// Khronos: [vkCmdEndConditionalRenderingEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) (nullable)
pub type PFN_vkCmdEndConditionalRenderingEXT = Option<vkCmdEndConditionalRenderingEXT_t>;
const vkCmdEndConditionalRenderingEXT_NAME: &str = "vkCmdEndConditionalRenderingEXT\0";

/// Khronos: [vkCmdEndDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) (non-nullable)
pub type vkCmdEndDebugUtilsLabelEXT_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer);
/// Khronos: [vkCmdEndDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) (nullable)
pub type PFN_vkCmdEndDebugUtilsLabelEXT = Option<vkCmdEndDebugUtilsLabelEXT_t>;
const vkCmdEndDebugUtilsLabelEXT_NAME: &str = "vkCmdEndDebugUtilsLabelEXT\0";

/// Khronos: [vkCmdEndQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html) (non-nullable)
pub type vkCmdEndQuery_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  query: u32,
);
/// Khronos: [vkCmdEndQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html) (nullable)
pub type PFN_vkCmdEndQuery = Option<vkCmdEndQuery_t>;
const vkCmdEndQuery_NAME: &str = "vkCmdEndQuery\0";

/// Khronos: [vkCmdEndQueryIndexedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html) (non-nullable)
pub type vkCmdEndQueryIndexedEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  query: u32,
  index: u32,
);
/// Khronos: [vkCmdEndQueryIndexedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html) (nullable)
pub type PFN_vkCmdEndQueryIndexedEXT = Option<vkCmdEndQueryIndexedEXT_t>;
const vkCmdEndQueryIndexedEXT_NAME: &str = "vkCmdEndQueryIndexedEXT\0";

/// Khronos: [vkCmdEndRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html) (non-nullable)
pub type vkCmdEndRenderPass_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer);
/// Khronos: [vkCmdEndRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html) (nullable)
pub type PFN_vkCmdEndRenderPass = Option<vkCmdEndRenderPass_t>;
const vkCmdEndRenderPass_NAME: &str = "vkCmdEndRenderPass\0";

/// Khronos: [vkCmdEndRenderPass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html) (non-nullable)
pub type vkCmdEndRenderPass2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  subpass_end_info: *const VkSubpassEndInfo,
);
/// Khronos: [vkCmdEndRenderPass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html) (nullable)
pub type PFN_vkCmdEndRenderPass2 = Option<vkCmdEndRenderPass2_t>;
const vkCmdEndRenderPass2_NAME: &str = "vkCmdEndRenderPass2\0";

/// Khronos: [vkCmdEndRendering](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html) (non-nullable)
pub type vkCmdEndRendering_t = unsafe extern "system" fn(command_buffer: VkCommandBuffer);
/// Khronos: [vkCmdEndRendering](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html) (nullable)
pub type PFN_vkCmdEndRendering = Option<vkCmdEndRendering_t>;
const vkCmdEndRendering_NAME: &str = "vkCmdEndRendering\0";

/// Khronos: [vkCmdEndTransformFeedbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html) (non-nullable)
pub type vkCmdEndTransformFeedbackEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_counter_buffer: u32,
  counter_buffer_count: u32,
  counter_buffers: *const VkBuffer,
  counter_buffer_offsets: *const VkDeviceSize,
);
/// Khronos: [vkCmdEndTransformFeedbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html) (nullable)
pub type PFN_vkCmdEndTransformFeedbackEXT = Option<vkCmdEndTransformFeedbackEXT_t>;
const vkCmdEndTransformFeedbackEXT_NAME: &str = "vkCmdEndTransformFeedbackEXT\0";

/// Khronos: [vkCmdEndVideoCodingKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html) (non-nullable)
pub type vkCmdEndVideoCodingKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  end_coding_info: *const VkVideoEndCodingInfoKHR,
);
/// Khronos: [vkCmdEndVideoCodingKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html) (nullable)
pub type PFN_vkCmdEndVideoCodingKHR = Option<vkCmdEndVideoCodingKHR_t>;
const vkCmdEndVideoCodingKHR_NAME: &str = "vkCmdEndVideoCodingKHR\0";

/// Khronos: [vkCmdExecuteCommands](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html) (non-nullable)
pub type vkCmdExecuteCommands_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  command_buffer_count: u32,
  command_buffers: *const VkCommandBuffer,
);
/// Khronos: [vkCmdExecuteCommands](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html) (nullable)
pub type PFN_vkCmdExecuteCommands = Option<vkCmdExecuteCommands_t>;
const vkCmdExecuteCommands_NAME: &str = "vkCmdExecuteCommands\0";

/// Khronos: [vkCmdExecuteGeneratedCommandsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html) (non-nullable)
pub type vkCmdExecuteGeneratedCommandsNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  is_preprocessed: VkBool32,
  generated_commands_info: *const VkGeneratedCommandsInfoNV,
);
/// Khronos: [vkCmdExecuteGeneratedCommandsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html) (nullable)
pub type PFN_vkCmdExecuteGeneratedCommandsNV = Option<vkCmdExecuteGeneratedCommandsNV_t>;
const vkCmdExecuteGeneratedCommandsNV_NAME: &str = "vkCmdExecuteGeneratedCommandsNV\0";

/// Khronos: [vkCmdFillBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html) (non-nullable)
pub type vkCmdFillBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  dst_buffer: VkBuffer,
  dst_offset: VkDeviceSize,
  size: VkDeviceSize,
  data: u32,
);
/// Khronos: [vkCmdFillBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html) (nullable)
pub type PFN_vkCmdFillBuffer = Option<vkCmdFillBuffer_t>;
const vkCmdFillBuffer_NAME: &str = "vkCmdFillBuffer\0";

/// Khronos: [vkCmdInsertDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html) (non-nullable)
pub type vkCmdInsertDebugUtilsLabelEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  label_info: *const VkDebugUtilsLabelEXT,
);
/// Khronos: [vkCmdInsertDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html) (nullable)
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = Option<vkCmdInsertDebugUtilsLabelEXT_t>;
const vkCmdInsertDebugUtilsLabelEXT_NAME: &str = "vkCmdInsertDebugUtilsLabelEXT\0";

/// Khronos: [vkCmdNextSubpass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html) (non-nullable)
pub type vkCmdNextSubpass_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, contents: VkSubpassContents);
/// Khronos: [vkCmdNextSubpass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html) (nullable)
pub type PFN_vkCmdNextSubpass = Option<vkCmdNextSubpass_t>;
const vkCmdNextSubpass_NAME: &str = "vkCmdNextSubpass\0";

/// Khronos: [vkCmdNextSubpass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html) (non-nullable)
pub type vkCmdNextSubpass2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  subpass_begin_info: *const VkSubpassBeginInfo,
  subpass_end_info: *const VkSubpassEndInfo,
);
/// Khronos: [vkCmdNextSubpass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html) (nullable)
pub type PFN_vkCmdNextSubpass2 = Option<vkCmdNextSubpass2_t>;
const vkCmdNextSubpass2_NAME: &str = "vkCmdNextSubpass2\0";

/// Khronos: [vkCmdOpticalFlowExecuteNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdOpticalFlowExecuteNV.html) (non-nullable)
pub type vkCmdOpticalFlowExecuteNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  session: VkOpticalFlowSessionNV,
  execute_info: *const VkOpticalFlowExecuteInfoNV,
);
/// Khronos: [vkCmdOpticalFlowExecuteNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdOpticalFlowExecuteNV.html) (nullable)
pub type PFN_vkCmdOpticalFlowExecuteNV = Option<vkCmdOpticalFlowExecuteNV_t>;
const vkCmdOpticalFlowExecuteNV_NAME: &str = "vkCmdOpticalFlowExecuteNV\0";

/// Khronos: [vkCmdPipelineBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html) (non-nullable)
pub type vkCmdPipelineBarrier_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_stage_mask: VkPipelineStageFlags,
  dst_stage_mask: VkPipelineStageFlags,
  dependency_flags: VkDependencyFlags,
  memory_barrier_count: u32,
  memory_barriers: *const VkMemoryBarrier,
  buffer_memory_barrier_count: u32,
  buffer_memory_barriers: *const VkBufferMemoryBarrier,
  image_memory_barrier_count: u32,
  image_memory_barriers: *const VkImageMemoryBarrier,
);
/// Khronos: [vkCmdPipelineBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html) (nullable)
pub type PFN_vkCmdPipelineBarrier = Option<vkCmdPipelineBarrier_t>;
const vkCmdPipelineBarrier_NAME: &str = "vkCmdPipelineBarrier\0";

/// Khronos: [vkCmdPipelineBarrier2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html) (non-nullable)
pub type vkCmdPipelineBarrier2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  dependency_info: *const VkDependencyInfo,
);
/// Khronos: [vkCmdPipelineBarrier2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html) (nullable)
pub type PFN_vkCmdPipelineBarrier2 = Option<vkCmdPipelineBarrier2_t>;
const vkCmdPipelineBarrier2_NAME: &str = "vkCmdPipelineBarrier2\0";

/// Khronos: [vkCmdPreprocessGeneratedCommandsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html) (non-nullable)
pub type vkCmdPreprocessGeneratedCommandsNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  generated_commands_info: *const VkGeneratedCommandsInfoNV,
);
/// Khronos: [vkCmdPreprocessGeneratedCommandsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html) (nullable)
pub type PFN_vkCmdPreprocessGeneratedCommandsNV =
  Option<vkCmdPreprocessGeneratedCommandsNV_t>;
const vkCmdPreprocessGeneratedCommandsNV_NAME: &str =
  "vkCmdPreprocessGeneratedCommandsNV\0";

/// Khronos: [vkCmdPushConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html) (non-nullable)
pub type vkCmdPushConstants_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  layout: VkPipelineLayout,
  stage_flags: VkShaderStageFlags,
  offset: u32,
  size: u32,
  values: *const c_void,
);
/// Khronos: [vkCmdPushConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html) (nullable)
pub type PFN_vkCmdPushConstants = Option<vkCmdPushConstants_t>;
const vkCmdPushConstants_NAME: &str = "vkCmdPushConstants\0";

/// Khronos: [vkCmdPushDescriptorSetKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html) (non-nullable)
pub type vkCmdPushDescriptorSetKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_bind_point: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  set: u32,
  descriptor_write_count: u32,
  descriptor_writes: *const VkWriteDescriptorSet,
);
/// Khronos: [vkCmdPushDescriptorSetKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html) (nullable)
pub type PFN_vkCmdPushDescriptorSetKHR = Option<vkCmdPushDescriptorSetKHR_t>;
const vkCmdPushDescriptorSetKHR_NAME: &str = "vkCmdPushDescriptorSetKHR\0";

/// Khronos: [vkCmdPushDescriptorSetWithTemplateKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) (non-nullable)
pub type vkCmdPushDescriptorSetWithTemplateKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  descriptor_update_template: VkDescriptorUpdateTemplate,
  layout: VkPipelineLayout,
  set: u32,
  data: *const c_void,
);
/// Khronos: [vkCmdPushDescriptorSetWithTemplateKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) (nullable)
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR =
  Option<vkCmdPushDescriptorSetWithTemplateKHR_t>;
const vkCmdPushDescriptorSetWithTemplateKHR_NAME: &str =
  "vkCmdPushDescriptorSetWithTemplateKHR\0";

/// Khronos: [vkCmdRefreshObjectsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdRefreshObjectsKHR.html) (non-nullable)
pub type vkCmdRefreshObjectsKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  refresh_objects: *const VkRefreshObjectListKHR,
);
/// Khronos: [vkCmdRefreshObjectsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdRefreshObjectsKHR.html) (nullable)
pub type PFN_vkCmdRefreshObjectsKHR = Option<vkCmdRefreshObjectsKHR_t>;
const vkCmdRefreshObjectsKHR_NAME: &str = "vkCmdRefreshObjectsKHR\0";

/// Khronos: [vkCmdResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html) (non-nullable)
pub type vkCmdResetEvent_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  event: VkEvent,
  stage_mask: VkPipelineStageFlags,
);
/// Khronos: [vkCmdResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html) (nullable)
pub type PFN_vkCmdResetEvent = Option<vkCmdResetEvent_t>;
const vkCmdResetEvent_NAME: &str = "vkCmdResetEvent\0";

/// Khronos: [vkCmdResetEvent2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html) (non-nullable)
pub type vkCmdResetEvent2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  event: VkEvent,
  stage_mask: VkPipelineStageFlags2,
);
/// Khronos: [vkCmdResetEvent2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html) (nullable)
pub type PFN_vkCmdResetEvent2 = Option<vkCmdResetEvent2_t>;
const vkCmdResetEvent2_NAME: &str = "vkCmdResetEvent2\0";

/// Khronos: [vkCmdResetQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html) (non-nullable)
pub type vkCmdResetQueryPool_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  first_query: u32,
  query_count: u32,
);
/// Khronos: [vkCmdResetQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html) (nullable)
pub type PFN_vkCmdResetQueryPool = Option<vkCmdResetQueryPool_t>;
const vkCmdResetQueryPool_NAME: &str = "vkCmdResetQueryPool\0";

/// Khronos: [vkCmdResolveImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html) (non-nullable)
pub type vkCmdResolveImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_image: VkImage,
  src_image_layout: VkImageLayout,
  dst_image: VkImage,
  dst_image_layout: VkImageLayout,
  region_count: u32,
  regions: *const VkImageResolve,
);
/// Khronos: [vkCmdResolveImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html) (nullable)
pub type PFN_vkCmdResolveImage = Option<vkCmdResolveImage_t>;
const vkCmdResolveImage_NAME: &str = "vkCmdResolveImage\0";

/// Khronos: [vkCmdResolveImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html) (non-nullable)
pub type vkCmdResolveImage2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  resolve_image_info: *const VkResolveImageInfo2,
);
/// Khronos: [vkCmdResolveImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html) (nullable)
pub type PFN_vkCmdResolveImage2 = Option<vkCmdResolveImage2_t>;
const vkCmdResolveImage2_NAME: &str = "vkCmdResolveImage2\0";

/// Khronos: [vkCmdSetAlphaToCoverageEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html) (non-nullable)
pub type vkCmdSetAlphaToCoverageEnableEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  alpha_to_coverage_enable: VkBool32,
);
/// Khronos: [vkCmdSetAlphaToCoverageEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html) (nullable)
pub type PFN_vkCmdSetAlphaToCoverageEnableEXT =
  Option<vkCmdSetAlphaToCoverageEnableEXT_t>;
const vkCmdSetAlphaToCoverageEnableEXT_NAME: &str = "vkCmdSetAlphaToCoverageEnableEXT\0";

/// Khronos: [vkCmdSetAlphaToOneEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html) (non-nullable)
pub type vkCmdSetAlphaToOneEnableEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  alpha_to_one_enable: VkBool32,
);
/// Khronos: [vkCmdSetAlphaToOneEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html) (nullable)
pub type PFN_vkCmdSetAlphaToOneEnableEXT = Option<vkCmdSetAlphaToOneEnableEXT_t>;
const vkCmdSetAlphaToOneEnableEXT_NAME: &str = "vkCmdSetAlphaToOneEnableEXT\0";

/// Khronos: [vkCmdSetBlendConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html) (non-nullable)
pub type vkCmdSetBlendConstants_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  blend_constants: *const [c_float; 4],
);
/// Khronos: [vkCmdSetBlendConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html) (nullable)
pub type PFN_vkCmdSetBlendConstants = Option<vkCmdSetBlendConstants_t>;
const vkCmdSetBlendConstants_NAME: &str = "vkCmdSetBlendConstants\0";

/// Khronos: [vkCmdSetCheckpointNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html) (non-nullable)
pub type vkCmdSetCheckpointNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  checkpoint_marker: *const c_void,
);
/// Khronos: [vkCmdSetCheckpointNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html) (nullable)
pub type PFN_vkCmdSetCheckpointNV = Option<vkCmdSetCheckpointNV_t>;
const vkCmdSetCheckpointNV_NAME: &str = "vkCmdSetCheckpointNV\0";

/// Khronos: [vkCmdSetCoarseSampleOrderNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html) (non-nullable)
pub type vkCmdSetCoarseSampleOrderNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  sample_order_type: VkCoarseSampleOrderTypeNV,
  custom_sample_order_count: u32,
  custom_sample_orders: *const VkCoarseSampleOrderCustomNV,
);
/// Khronos: [vkCmdSetCoarseSampleOrderNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html) (nullable)
pub type PFN_vkCmdSetCoarseSampleOrderNV = Option<vkCmdSetCoarseSampleOrderNV_t>;
const vkCmdSetCoarseSampleOrderNV_NAME: &str = "vkCmdSetCoarseSampleOrderNV\0";

/// Khronos: [vkCmdSetColorBlendAdvancedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html) (non-nullable)
pub type vkCmdSetColorBlendAdvancedEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_attachment: u32,
  attachment_count: u32,
  color_blend_advanced: *const VkColorBlendAdvancedEXT,
);
/// Khronos: [vkCmdSetColorBlendAdvancedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html) (nullable)
pub type PFN_vkCmdSetColorBlendAdvancedEXT = Option<vkCmdSetColorBlendAdvancedEXT_t>;
const vkCmdSetColorBlendAdvancedEXT_NAME: &str = "vkCmdSetColorBlendAdvancedEXT\0";

/// Khronos: [vkCmdSetColorBlendEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html) (non-nullable)
pub type vkCmdSetColorBlendEnableEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_attachment: u32,
  attachment_count: u32,
  color_blend_enables: *const VkBool32,
);
/// Khronos: [vkCmdSetColorBlendEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html) (nullable)
pub type PFN_vkCmdSetColorBlendEnableEXT = Option<vkCmdSetColorBlendEnableEXT_t>;
const vkCmdSetColorBlendEnableEXT_NAME: &str = "vkCmdSetColorBlendEnableEXT\0";

/// Khronos: [vkCmdSetColorBlendEquationEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html) (non-nullable)
pub type vkCmdSetColorBlendEquationEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_attachment: u32,
  attachment_count: u32,
  color_blend_equations: *const VkColorBlendEquationEXT,
);
/// Khronos: [vkCmdSetColorBlendEquationEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html) (nullable)
pub type PFN_vkCmdSetColorBlendEquationEXT = Option<vkCmdSetColorBlendEquationEXT_t>;
const vkCmdSetColorBlendEquationEXT_NAME: &str = "vkCmdSetColorBlendEquationEXT\0";

/// Khronos: [vkCmdSetColorWriteEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html) (non-nullable)
pub type vkCmdSetColorWriteEnableEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  attachment_count: u32,
  color_write_enables: *const VkBool32,
);
/// Khronos: [vkCmdSetColorWriteEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html) (nullable)
pub type PFN_vkCmdSetColorWriteEnableEXT = Option<vkCmdSetColorWriteEnableEXT_t>;
const vkCmdSetColorWriteEnableEXT_NAME: &str = "vkCmdSetColorWriteEnableEXT\0";

/// Khronos: [vkCmdSetColorWriteMaskEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html) (non-nullable)
pub type vkCmdSetColorWriteMaskEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_attachment: u32,
  attachment_count: u32,
  color_write_masks: *const VkColorComponentFlags,
);
/// Khronos: [vkCmdSetColorWriteMaskEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html) (nullable)
pub type PFN_vkCmdSetColorWriteMaskEXT = Option<vkCmdSetColorWriteMaskEXT_t>;
const vkCmdSetColorWriteMaskEXT_NAME: &str = "vkCmdSetColorWriteMaskEXT\0";

/// Khronos: [vkCmdSetConservativeRasterizationModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html) (non-nullable)
pub type vkCmdSetConservativeRasterizationModeEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  conservative_rasterization_mode: VkConservativeRasterizationModeEXT,
);
/// Khronos: [vkCmdSetConservativeRasterizationModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html) (nullable)
pub type PFN_vkCmdSetConservativeRasterizationModeEXT =
  Option<vkCmdSetConservativeRasterizationModeEXT_t>;
const vkCmdSetConservativeRasterizationModeEXT_NAME: &str =
  "vkCmdSetConservativeRasterizationModeEXT\0";

/// Khronos: [vkCmdSetCoverageModulationModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html) (non-nullable)
pub type vkCmdSetCoverageModulationModeNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  coverage_modulation_mode: VkCoverageModulationModeNV,
);
/// Khronos: [vkCmdSetCoverageModulationModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html) (nullable)
pub type PFN_vkCmdSetCoverageModulationModeNV =
  Option<vkCmdSetCoverageModulationModeNV_t>;
const vkCmdSetCoverageModulationModeNV_NAME: &str = "vkCmdSetCoverageModulationModeNV\0";

/// Khronos: [vkCmdSetCoverageModulationTableEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html) (non-nullable)
pub type vkCmdSetCoverageModulationTableEnableNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  coverage_modulation_table_enable: VkBool32,
);
/// Khronos: [vkCmdSetCoverageModulationTableEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html) (nullable)
pub type PFN_vkCmdSetCoverageModulationTableEnableNV =
  Option<vkCmdSetCoverageModulationTableEnableNV_t>;
const vkCmdSetCoverageModulationTableEnableNV_NAME: &str =
  "vkCmdSetCoverageModulationTableEnableNV\0";

/// Khronos: [vkCmdSetCoverageModulationTableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html) (non-nullable)
pub type vkCmdSetCoverageModulationTableNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  coverage_modulation_table_count: u32,
  coverage_modulation_table: *const c_float,
);
/// Khronos: [vkCmdSetCoverageModulationTableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html) (nullable)
pub type PFN_vkCmdSetCoverageModulationTableNV =
  Option<vkCmdSetCoverageModulationTableNV_t>;
const vkCmdSetCoverageModulationTableNV_NAME: &str =
  "vkCmdSetCoverageModulationTableNV\0";

/// Khronos: [vkCmdSetCoverageReductionModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html) (non-nullable)
pub type vkCmdSetCoverageReductionModeNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  coverage_reduction_mode: VkCoverageReductionModeNV,
);
/// Khronos: [vkCmdSetCoverageReductionModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html) (nullable)
pub type PFN_vkCmdSetCoverageReductionModeNV = Option<vkCmdSetCoverageReductionModeNV_t>;
const vkCmdSetCoverageReductionModeNV_NAME: &str = "vkCmdSetCoverageReductionModeNV\0";

/// Khronos: [vkCmdSetCoverageToColorEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html) (non-nullable)
pub type vkCmdSetCoverageToColorEnableNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  coverage_to_color_enable: VkBool32,
);
/// Khronos: [vkCmdSetCoverageToColorEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html) (nullable)
pub type PFN_vkCmdSetCoverageToColorEnableNV = Option<vkCmdSetCoverageToColorEnableNV_t>;
const vkCmdSetCoverageToColorEnableNV_NAME: &str = "vkCmdSetCoverageToColorEnableNV\0";

/// Khronos: [vkCmdSetCoverageToColorLocationNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html) (non-nullable)
pub type vkCmdSetCoverageToColorLocationNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  coverage_to_color_location: u32,
);
/// Khronos: [vkCmdSetCoverageToColorLocationNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html) (nullable)
pub type PFN_vkCmdSetCoverageToColorLocationNV =
  Option<vkCmdSetCoverageToColorLocationNV_t>;
const vkCmdSetCoverageToColorLocationNV_NAME: &str =
  "vkCmdSetCoverageToColorLocationNV\0";

/// Khronos: [vkCmdSetCullMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html) (non-nullable)
pub type vkCmdSetCullMode_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, cull_mode: VkCullModeFlags);
/// Khronos: [vkCmdSetCullMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html) (nullable)
pub type PFN_vkCmdSetCullMode = Option<vkCmdSetCullMode_t>;
const vkCmdSetCullMode_NAME: &str = "vkCmdSetCullMode\0";

/// Khronos: [vkCmdSetDepthBias](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html) (non-nullable)
pub type vkCmdSetDepthBias_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  depth_bias_constant_factor: c_float,
  depth_bias_clamp: c_float,
  depth_bias_slope_factor: c_float,
);
/// Khronos: [vkCmdSetDepthBias](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html) (nullable)
pub type PFN_vkCmdSetDepthBias = Option<vkCmdSetDepthBias_t>;
const vkCmdSetDepthBias_NAME: &str = "vkCmdSetDepthBias\0";

/// Khronos: [vkCmdSetDepthBiasEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnable.html) (non-nullable)
pub type vkCmdSetDepthBiasEnable_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, depth_bias_enable: VkBool32);
/// Khronos: [vkCmdSetDepthBiasEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnable.html) (nullable)
pub type PFN_vkCmdSetDepthBiasEnable = Option<vkCmdSetDepthBiasEnable_t>;
const vkCmdSetDepthBiasEnable_NAME: &str = "vkCmdSetDepthBiasEnable\0";

/// Khronos: [vkCmdSetDepthBounds](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html) (non-nullable)
pub type vkCmdSetDepthBounds_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  min_depth_bounds: c_float,
  max_depth_bounds: c_float,
);
/// Khronos: [vkCmdSetDepthBounds](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html) (nullable)
pub type PFN_vkCmdSetDepthBounds = Option<vkCmdSetDepthBounds_t>;
const vkCmdSetDepthBounds_NAME: &str = "vkCmdSetDepthBounds\0";

/// Khronos: [vkCmdSetDepthBoundsTestEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html) (non-nullable)
pub type vkCmdSetDepthBoundsTestEnable_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  depth_bounds_test_enable: VkBool32,
);
/// Khronos: [vkCmdSetDepthBoundsTestEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html) (nullable)
pub type PFN_vkCmdSetDepthBoundsTestEnable = Option<vkCmdSetDepthBoundsTestEnable_t>;
const vkCmdSetDepthBoundsTestEnable_NAME: &str = "vkCmdSetDepthBoundsTestEnable\0";

/// Khronos: [vkCmdSetDepthClampEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html) (non-nullable)
pub type vkCmdSetDepthClampEnableEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  depth_clamp_enable: VkBool32,
);
/// Khronos: [vkCmdSetDepthClampEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html) (nullable)
pub type PFN_vkCmdSetDepthClampEnableEXT = Option<vkCmdSetDepthClampEnableEXT_t>;
const vkCmdSetDepthClampEnableEXT_NAME: &str = "vkCmdSetDepthClampEnableEXT\0";

/// Khronos: [vkCmdSetDepthClipEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html) (non-nullable)
pub type vkCmdSetDepthClipEnableEXT_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, depth_clip_enable: VkBool32);
/// Khronos: [vkCmdSetDepthClipEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html) (nullable)
pub type PFN_vkCmdSetDepthClipEnableEXT = Option<vkCmdSetDepthClipEnableEXT_t>;
const vkCmdSetDepthClipEnableEXT_NAME: &str = "vkCmdSetDepthClipEnableEXT\0";

/// Khronos: [vkCmdSetDepthClipNegativeOneToOneEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html) (non-nullable)
pub type vkCmdSetDepthClipNegativeOneToOneEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  negative_one_to_one: VkBool32,
);
/// Khronos: [vkCmdSetDepthClipNegativeOneToOneEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html) (nullable)
pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT =
  Option<vkCmdSetDepthClipNegativeOneToOneEXT_t>;
const vkCmdSetDepthClipNegativeOneToOneEXT_NAME: &str =
  "vkCmdSetDepthClipNegativeOneToOneEXT\0";

/// Khronos: [vkCmdSetDepthCompareOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html) (non-nullable)
pub type vkCmdSetDepthCompareOp_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  depth_compare_op: VkCompareOp,
);
/// Khronos: [vkCmdSetDepthCompareOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html) (nullable)
pub type PFN_vkCmdSetDepthCompareOp = Option<vkCmdSetDepthCompareOp_t>;
const vkCmdSetDepthCompareOp_NAME: &str = "vkCmdSetDepthCompareOp\0";

/// Khronos: [vkCmdSetDepthTestEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html) (non-nullable)
pub type vkCmdSetDepthTestEnable_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, depth_test_enable: VkBool32);
/// Khronos: [vkCmdSetDepthTestEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html) (nullable)
pub type PFN_vkCmdSetDepthTestEnable = Option<vkCmdSetDepthTestEnable_t>;
const vkCmdSetDepthTestEnable_NAME: &str = "vkCmdSetDepthTestEnable\0";

/// Khronos: [vkCmdSetDepthWriteEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html) (non-nullable)
pub type vkCmdSetDepthWriteEnable_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  depth_write_enable: VkBool32,
);
/// Khronos: [vkCmdSetDepthWriteEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html) (nullable)
pub type PFN_vkCmdSetDepthWriteEnable = Option<vkCmdSetDepthWriteEnable_t>;
const vkCmdSetDepthWriteEnable_NAME: &str = "vkCmdSetDepthWriteEnable\0";

/// Khronos: [vkCmdSetDescriptorBufferOffsetsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html) (non-nullable)
pub type vkCmdSetDescriptorBufferOffsetsEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_bind_point: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  first_set: u32,
  set_count: u32,
  buffer_indices: *const u32,
  offsets: *const VkDeviceSize,
);
/// Khronos: [vkCmdSetDescriptorBufferOffsetsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html) (nullable)
pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT =
  Option<vkCmdSetDescriptorBufferOffsetsEXT_t>;
const vkCmdSetDescriptorBufferOffsetsEXT_NAME: &str =
  "vkCmdSetDescriptorBufferOffsetsEXT\0";

/// Khronos: [vkCmdSetDeviceMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html) (non-nullable)
pub type vkCmdSetDeviceMask_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, device_mask: u32);
/// Khronos: [vkCmdSetDeviceMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html) (nullable)
pub type PFN_vkCmdSetDeviceMask = Option<vkCmdSetDeviceMask_t>;
const vkCmdSetDeviceMask_NAME: &str = "vkCmdSetDeviceMask\0";

/// Khronos: [vkCmdSetDiscardRectangleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) (non-nullable)
pub type vkCmdSetDiscardRectangleEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_discard_rectangle: u32,
  discard_rectangle_count: u32,
  discard_rectangles: *const VkRect2D,
);
/// Khronos: [vkCmdSetDiscardRectangleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) (nullable)
pub type PFN_vkCmdSetDiscardRectangleEXT = Option<vkCmdSetDiscardRectangleEXT_t>;
const vkCmdSetDiscardRectangleEXT_NAME: &str = "vkCmdSetDiscardRectangleEXT\0";

/// Khronos: [vkCmdSetDiscardRectangleEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEnableEXT.html) (non-nullable)
pub type vkCmdSetDiscardRectangleEnableEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  discard_rectangle_enable: VkBool32,
);
/// Khronos: [vkCmdSetDiscardRectangleEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEnableEXT.html) (nullable)
pub type PFN_vkCmdSetDiscardRectangleEnableEXT =
  Option<vkCmdSetDiscardRectangleEnableEXT_t>;
const vkCmdSetDiscardRectangleEnableEXT_NAME: &str =
  "vkCmdSetDiscardRectangleEnableEXT\0";

/// Khronos: [vkCmdSetDiscardRectangleModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleModeEXT.html) (non-nullable)
pub type vkCmdSetDiscardRectangleModeEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  discard_rectangle_mode: VkDiscardRectangleModeEXT,
);
/// Khronos: [vkCmdSetDiscardRectangleModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleModeEXT.html) (nullable)
pub type PFN_vkCmdSetDiscardRectangleModeEXT = Option<vkCmdSetDiscardRectangleModeEXT_t>;
const vkCmdSetDiscardRectangleModeEXT_NAME: &str = "vkCmdSetDiscardRectangleModeEXT\0";

/// Khronos: [vkCmdSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html) (non-nullable)
pub type vkCmdSetEvent_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  event: VkEvent,
  stage_mask: VkPipelineStageFlags,
);
/// Khronos: [vkCmdSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html) (nullable)
pub type PFN_vkCmdSetEvent = Option<vkCmdSetEvent_t>;
const vkCmdSetEvent_NAME: &str = "vkCmdSetEvent\0";

/// Khronos: [vkCmdSetEvent2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html) (non-nullable)
pub type vkCmdSetEvent2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  event: VkEvent,
  dependency_info: *const VkDependencyInfo,
);
/// Khronos: [vkCmdSetEvent2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html) (nullable)
pub type PFN_vkCmdSetEvent2 = Option<vkCmdSetEvent2_t>;
const vkCmdSetEvent2_NAME: &str = "vkCmdSetEvent2\0";

/// Khronos: [vkCmdSetExclusiveScissorEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorEnableNV.html) (non-nullable)
pub type vkCmdSetExclusiveScissorEnableNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_exclusive_scissor: u32,
  exclusive_scissor_count: u32,
  exclusive_scissor_enables: *const VkBool32,
);
/// Khronos: [vkCmdSetExclusiveScissorEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorEnableNV.html) (nullable)
pub type PFN_vkCmdSetExclusiveScissorEnableNV =
  Option<vkCmdSetExclusiveScissorEnableNV_t>;
const vkCmdSetExclusiveScissorEnableNV_NAME: &str = "vkCmdSetExclusiveScissorEnableNV\0";

/// Khronos: [vkCmdSetExclusiveScissorNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html) (non-nullable)
pub type vkCmdSetExclusiveScissorNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_exclusive_scissor: u32,
  exclusive_scissor_count: u32,
  exclusive_scissors: *const VkRect2D,
);
/// Khronos: [vkCmdSetExclusiveScissorNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html) (nullable)
pub type PFN_vkCmdSetExclusiveScissorNV = Option<vkCmdSetExclusiveScissorNV_t>;
const vkCmdSetExclusiveScissorNV_NAME: &str = "vkCmdSetExclusiveScissorNV\0";

/// Khronos: [vkCmdSetExtraPrimitiveOverestimationSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html) (non-nullable)
pub type vkCmdSetExtraPrimitiveOverestimationSizeEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  extra_primitive_overestimation_size: c_float,
);
/// Khronos: [vkCmdSetExtraPrimitiveOverestimationSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html) (nullable)
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT =
  Option<vkCmdSetExtraPrimitiveOverestimationSizeEXT_t>;
const vkCmdSetExtraPrimitiveOverestimationSizeEXT_NAME: &str =
  "vkCmdSetExtraPrimitiveOverestimationSizeEXT\0";

/// Khronos: [vkCmdSetFragmentShadingRateEnumNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html) (non-nullable)
pub type vkCmdSetFragmentShadingRateEnumNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  shading_rate: VkFragmentShadingRateNV,
  combiner_ops: *const [VkFragmentShadingRateCombinerOpKHR; 2],
);
/// Khronos: [vkCmdSetFragmentShadingRateEnumNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html) (nullable)
pub type PFN_vkCmdSetFragmentShadingRateEnumNV =
  Option<vkCmdSetFragmentShadingRateEnumNV_t>;
const vkCmdSetFragmentShadingRateEnumNV_NAME: &str =
  "vkCmdSetFragmentShadingRateEnumNV\0";

/// Khronos: [vkCmdSetFragmentShadingRateKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html) (non-nullable)
pub type vkCmdSetFragmentShadingRateKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  fragment_size: *const VkExtent2D,
  combiner_ops: *const [VkFragmentShadingRateCombinerOpKHR; 2],
);
/// Khronos: [vkCmdSetFragmentShadingRateKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html) (nullable)
pub type PFN_vkCmdSetFragmentShadingRateKHR = Option<vkCmdSetFragmentShadingRateKHR_t>;
const vkCmdSetFragmentShadingRateKHR_NAME: &str = "vkCmdSetFragmentShadingRateKHR\0";

/// Khronos: [vkCmdSetFrontFace](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html) (non-nullable)
pub type vkCmdSetFrontFace_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, front_face: VkFrontFace);
/// Khronos: [vkCmdSetFrontFace](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html) (nullable)
pub type PFN_vkCmdSetFrontFace = Option<vkCmdSetFrontFace_t>;
const vkCmdSetFrontFace_NAME: &str = "vkCmdSetFrontFace\0";

/// Khronos: [vkCmdSetLineRasterizationModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html) (non-nullable)
pub type vkCmdSetLineRasterizationModeEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  line_rasterization_mode: VkLineRasterizationModeEXT,
);
/// Khronos: [vkCmdSetLineRasterizationModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html) (nullable)
pub type PFN_vkCmdSetLineRasterizationModeEXT =
  Option<vkCmdSetLineRasterizationModeEXT_t>;
const vkCmdSetLineRasterizationModeEXT_NAME: &str = "vkCmdSetLineRasterizationModeEXT\0";

/// Khronos: [vkCmdSetLineStippleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEXT.html) (non-nullable)
pub type vkCmdSetLineStippleEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  line_stipple_factor: u32,
  line_stipple_pattern: u16,
);
/// Khronos: [vkCmdSetLineStippleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEXT.html) (nullable)
pub type PFN_vkCmdSetLineStippleEXT = Option<vkCmdSetLineStippleEXT_t>;
const vkCmdSetLineStippleEXT_NAME: &str = "vkCmdSetLineStippleEXT\0";

/// Khronos: [vkCmdSetLineStippleEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html) (non-nullable)
pub type vkCmdSetLineStippleEnableEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  stippled_line_enable: VkBool32,
);
/// Khronos: [vkCmdSetLineStippleEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html) (nullable)
pub type PFN_vkCmdSetLineStippleEnableEXT = Option<vkCmdSetLineStippleEnableEXT_t>;
const vkCmdSetLineStippleEnableEXT_NAME: &str = "vkCmdSetLineStippleEnableEXT\0";

/// Khronos: [vkCmdSetLineWidth](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html) (non-nullable)
pub type vkCmdSetLineWidth_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, line_width: c_float);
/// Khronos: [vkCmdSetLineWidth](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html) (nullable)
pub type PFN_vkCmdSetLineWidth = Option<vkCmdSetLineWidth_t>;
const vkCmdSetLineWidth_NAME: &str = "vkCmdSetLineWidth\0";

/// Khronos: [vkCmdSetLogicOpEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html) (non-nullable)
pub type vkCmdSetLogicOpEXT_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, logic_op: VkLogicOp);
/// Khronos: [vkCmdSetLogicOpEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html) (nullable)
pub type PFN_vkCmdSetLogicOpEXT = Option<vkCmdSetLogicOpEXT_t>;
const vkCmdSetLogicOpEXT_NAME: &str = "vkCmdSetLogicOpEXT\0";

/// Khronos: [vkCmdSetLogicOpEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html) (non-nullable)
pub type vkCmdSetLogicOpEnableEXT_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, logic_op_enable: VkBool32);
/// Khronos: [vkCmdSetLogicOpEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html) (nullable)
pub type PFN_vkCmdSetLogicOpEnableEXT = Option<vkCmdSetLogicOpEnableEXT_t>;
const vkCmdSetLogicOpEnableEXT_NAME: &str = "vkCmdSetLogicOpEnableEXT\0";

/// Khronos: [vkCmdSetPatchControlPointsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html) (non-nullable)
pub type vkCmdSetPatchControlPointsEXT_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, patch_control_points: u32);
/// Khronos: [vkCmdSetPatchControlPointsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html) (nullable)
pub type PFN_vkCmdSetPatchControlPointsEXT = Option<vkCmdSetPatchControlPointsEXT_t>;
const vkCmdSetPatchControlPointsEXT_NAME: &str = "vkCmdSetPatchControlPointsEXT\0";

/// Khronos: [vkCmdSetPerformanceMarkerINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html) (non-nullable)
pub type vkCmdSetPerformanceMarkerINTEL_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  marker_info: *const VkPerformanceMarkerInfoINTEL,
) -> VkResult;
/// Khronos: [vkCmdSetPerformanceMarkerINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html) (nullable)
pub type PFN_vkCmdSetPerformanceMarkerINTEL = Option<vkCmdSetPerformanceMarkerINTEL_t>;
const vkCmdSetPerformanceMarkerINTEL_NAME: &str = "vkCmdSetPerformanceMarkerINTEL\0";

/// Khronos: [vkCmdSetPerformanceOverrideINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) (non-nullable)
pub type vkCmdSetPerformanceOverrideINTEL_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  override_info: *const VkPerformanceOverrideInfoINTEL,
) -> VkResult;
/// Khronos: [vkCmdSetPerformanceOverrideINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) (nullable)
pub type PFN_vkCmdSetPerformanceOverrideINTEL =
  Option<vkCmdSetPerformanceOverrideINTEL_t>;
const vkCmdSetPerformanceOverrideINTEL_NAME: &str = "vkCmdSetPerformanceOverrideINTEL\0";

/// Khronos: [vkCmdSetPerformanceStreamMarkerINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) (non-nullable)
pub type vkCmdSetPerformanceStreamMarkerINTEL_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  marker_info: *const VkPerformanceStreamMarkerInfoINTEL,
) -> VkResult;
/// Khronos: [vkCmdSetPerformanceStreamMarkerINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) (nullable)
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL =
  Option<vkCmdSetPerformanceStreamMarkerINTEL_t>;
const vkCmdSetPerformanceStreamMarkerINTEL_NAME: &str =
  "vkCmdSetPerformanceStreamMarkerINTEL\0";

/// Khronos: [vkCmdSetPolygonModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html) (non-nullable)
pub type vkCmdSetPolygonModeEXT_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, polygon_mode: VkPolygonMode);
/// Khronos: [vkCmdSetPolygonModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html) (nullable)
pub type PFN_vkCmdSetPolygonModeEXT = Option<vkCmdSetPolygonModeEXT_t>;
const vkCmdSetPolygonModeEXT_NAME: &str = "vkCmdSetPolygonModeEXT\0";

/// Khronos: [vkCmdSetPrimitiveRestartEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html) (non-nullable)
pub type vkCmdSetPrimitiveRestartEnable_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  primitive_restart_enable: VkBool32,
);
/// Khronos: [vkCmdSetPrimitiveRestartEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html) (nullable)
pub type PFN_vkCmdSetPrimitiveRestartEnable = Option<vkCmdSetPrimitiveRestartEnable_t>;
const vkCmdSetPrimitiveRestartEnable_NAME: &str = "vkCmdSetPrimitiveRestartEnable\0";

/// Khronos: [vkCmdSetPrimitiveTopology](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html) (non-nullable)
pub type vkCmdSetPrimitiveTopology_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  primitive_topology: VkPrimitiveTopology,
);
/// Khronos: [vkCmdSetPrimitiveTopology](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html) (nullable)
pub type PFN_vkCmdSetPrimitiveTopology = Option<vkCmdSetPrimitiveTopology_t>;
const vkCmdSetPrimitiveTopology_NAME: &str = "vkCmdSetPrimitiveTopology\0";

/// Khronos: [vkCmdSetProvokingVertexModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html) (non-nullable)
pub type vkCmdSetProvokingVertexModeEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  provoking_vertex_mode: VkProvokingVertexModeEXT,
);
/// Khronos: [vkCmdSetProvokingVertexModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html) (nullable)
pub type PFN_vkCmdSetProvokingVertexModeEXT = Option<vkCmdSetProvokingVertexModeEXT_t>;
const vkCmdSetProvokingVertexModeEXT_NAME: &str = "vkCmdSetProvokingVertexModeEXT\0";

/// Khronos: [vkCmdSetRasterizationSamplesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html) (non-nullable)
pub type vkCmdSetRasterizationSamplesEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  rasterization_samples: VkSampleCountFlagBits,
);
/// Khronos: [vkCmdSetRasterizationSamplesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html) (nullable)
pub type PFN_vkCmdSetRasterizationSamplesEXT = Option<vkCmdSetRasterizationSamplesEXT_t>;
const vkCmdSetRasterizationSamplesEXT_NAME: &str = "vkCmdSetRasterizationSamplesEXT\0";

/// Khronos: [vkCmdSetRasterizationStreamEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html) (non-nullable)
pub type vkCmdSetRasterizationStreamEXT_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, rasterization_stream: u32);
/// Khronos: [vkCmdSetRasterizationStreamEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html) (nullable)
pub type PFN_vkCmdSetRasterizationStreamEXT = Option<vkCmdSetRasterizationStreamEXT_t>;
const vkCmdSetRasterizationStreamEXT_NAME: &str = "vkCmdSetRasterizationStreamEXT\0";

/// Khronos: [vkCmdSetRasterizerDiscardEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html) (non-nullable)
pub type vkCmdSetRasterizerDiscardEnable_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  rasterizer_discard_enable: VkBool32,
);
/// Khronos: [vkCmdSetRasterizerDiscardEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html) (nullable)
pub type PFN_vkCmdSetRasterizerDiscardEnable = Option<vkCmdSetRasterizerDiscardEnable_t>;
const vkCmdSetRasterizerDiscardEnable_NAME: &str = "vkCmdSetRasterizerDiscardEnable\0";

/// Khronos: [vkCmdSetRayTracingPipelineStackSizeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html) (non-nullable)
pub type vkCmdSetRayTracingPipelineStackSizeKHR_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer, pipeline_stack_size: u32);
/// Khronos: [vkCmdSetRayTracingPipelineStackSizeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html) (nullable)
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
  Option<vkCmdSetRayTracingPipelineStackSizeKHR_t>;
const vkCmdSetRayTracingPipelineStackSizeKHR_NAME: &str =
  "vkCmdSetRayTracingPipelineStackSizeKHR\0";

/// Khronos: [vkCmdSetRepresentativeFragmentTestEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html) (non-nullable)
pub type vkCmdSetRepresentativeFragmentTestEnableNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  representative_fragment_test_enable: VkBool32,
);
/// Khronos: [vkCmdSetRepresentativeFragmentTestEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html) (nullable)
pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV =
  Option<vkCmdSetRepresentativeFragmentTestEnableNV_t>;
const vkCmdSetRepresentativeFragmentTestEnableNV_NAME: &str =
  "vkCmdSetRepresentativeFragmentTestEnableNV\0";

/// Khronos: [vkCmdSetSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html) (non-nullable)
pub type vkCmdSetSampleLocationsEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  sample_locations_info: *const VkSampleLocationsInfoEXT,
);
/// Khronos: [vkCmdSetSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html) (nullable)
pub type PFN_vkCmdSetSampleLocationsEXT = Option<vkCmdSetSampleLocationsEXT_t>;
const vkCmdSetSampleLocationsEXT_NAME: &str = "vkCmdSetSampleLocationsEXT\0";

/// Khronos: [vkCmdSetSampleLocationsEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html) (non-nullable)
pub type vkCmdSetSampleLocationsEnableEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  sample_locations_enable: VkBool32,
);
/// Khronos: [vkCmdSetSampleLocationsEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html) (nullable)
pub type PFN_vkCmdSetSampleLocationsEnableEXT =
  Option<vkCmdSetSampleLocationsEnableEXT_t>;
const vkCmdSetSampleLocationsEnableEXT_NAME: &str = "vkCmdSetSampleLocationsEnableEXT\0";

/// Khronos: [vkCmdSetSampleMaskEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html) (non-nullable)
pub type vkCmdSetSampleMaskEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  samples: VkSampleCountFlagBits,
  sample_mask: *const VkSampleMask,
);
/// Khronos: [vkCmdSetSampleMaskEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html) (nullable)
pub type PFN_vkCmdSetSampleMaskEXT = Option<vkCmdSetSampleMaskEXT_t>;
const vkCmdSetSampleMaskEXT_NAME: &str = "vkCmdSetSampleMaskEXT\0";

/// Khronos: [vkCmdSetScissor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html) (non-nullable)
pub type vkCmdSetScissor_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_scissor: u32,
  scissor_count: u32,
  scissors: *const VkRect2D,
);
/// Khronos: [vkCmdSetScissor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html) (nullable)
pub type PFN_vkCmdSetScissor = Option<vkCmdSetScissor_t>;
const vkCmdSetScissor_NAME: &str = "vkCmdSetScissor\0";

/// Khronos: [vkCmdSetScissorWithCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html) (non-nullable)
pub type vkCmdSetScissorWithCount_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  scissor_count: u32,
  scissors: *const VkRect2D,
);
/// Khronos: [vkCmdSetScissorWithCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html) (nullable)
pub type PFN_vkCmdSetScissorWithCount = Option<vkCmdSetScissorWithCount_t>;
const vkCmdSetScissorWithCount_NAME: &str = "vkCmdSetScissorWithCount\0";

/// Khronos: [vkCmdSetShadingRateImageEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html) (non-nullable)
pub type vkCmdSetShadingRateImageEnableNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  shading_rate_image_enable: VkBool32,
);
/// Khronos: [vkCmdSetShadingRateImageEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html) (nullable)
pub type PFN_vkCmdSetShadingRateImageEnableNV =
  Option<vkCmdSetShadingRateImageEnableNV_t>;
const vkCmdSetShadingRateImageEnableNV_NAME: &str = "vkCmdSetShadingRateImageEnableNV\0";

/// Khronos: [vkCmdSetStencilCompareMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html) (non-nullable)
pub type vkCmdSetStencilCompareMask_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  face_mask: VkStencilFaceFlags,
  compare_mask: u32,
);
/// Khronos: [vkCmdSetStencilCompareMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html) (nullable)
pub type PFN_vkCmdSetStencilCompareMask = Option<vkCmdSetStencilCompareMask_t>;
const vkCmdSetStencilCompareMask_NAME: &str = "vkCmdSetStencilCompareMask\0";

/// Khronos: [vkCmdSetStencilOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOp.html) (non-nullable)
pub type vkCmdSetStencilOp_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  face_mask: VkStencilFaceFlags,
  fail_op: VkStencilOp,
  pass_op: VkStencilOp,
  depth_fail_op: VkStencilOp,
  compare_op: VkCompareOp,
);
/// Khronos: [vkCmdSetStencilOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOp.html) (nullable)
pub type PFN_vkCmdSetStencilOp = Option<vkCmdSetStencilOp_t>;
const vkCmdSetStencilOp_NAME: &str = "vkCmdSetStencilOp\0";

/// Khronos: [vkCmdSetStencilReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html) (non-nullable)
pub type vkCmdSetStencilReference_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  face_mask: VkStencilFaceFlags,
  reference: u32,
);
/// Khronos: [vkCmdSetStencilReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html) (nullable)
pub type PFN_vkCmdSetStencilReference = Option<vkCmdSetStencilReference_t>;
const vkCmdSetStencilReference_NAME: &str = "vkCmdSetStencilReference\0";

/// Khronos: [vkCmdSetStencilTestEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnable.html) (non-nullable)
pub type vkCmdSetStencilTestEnable_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  stencil_test_enable: VkBool32,
);
/// Khronos: [vkCmdSetStencilTestEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnable.html) (nullable)
pub type PFN_vkCmdSetStencilTestEnable = Option<vkCmdSetStencilTestEnable_t>;
const vkCmdSetStencilTestEnable_NAME: &str = "vkCmdSetStencilTestEnable\0";

/// Khronos: [vkCmdSetStencilWriteMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html) (non-nullable)
pub type vkCmdSetStencilWriteMask_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  face_mask: VkStencilFaceFlags,
  write_mask: u32,
);
/// Khronos: [vkCmdSetStencilWriteMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html) (nullable)
pub type PFN_vkCmdSetStencilWriteMask = Option<vkCmdSetStencilWriteMask_t>;
const vkCmdSetStencilWriteMask_NAME: &str = "vkCmdSetStencilWriteMask\0";

/// Khronos: [vkCmdSetTessellationDomainOriginEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html) (non-nullable)
pub type vkCmdSetTessellationDomainOriginEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  domain_origin: VkTessellationDomainOrigin,
);
/// Khronos: [vkCmdSetTessellationDomainOriginEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html) (nullable)
pub type PFN_vkCmdSetTessellationDomainOriginEXT =
  Option<vkCmdSetTessellationDomainOriginEXT_t>;
const vkCmdSetTessellationDomainOriginEXT_NAME: &str =
  "vkCmdSetTessellationDomainOriginEXT\0";

/// Khronos: [vkCmdSetVertexInputEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html) (non-nullable)
pub type vkCmdSetVertexInputEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  vertex_binding_description_count: u32,
  vertex_binding_descriptions: *const VkVertexInputBindingDescription2EXT,
  vertex_attribute_description_count: u32,
  vertex_attribute_descriptions: *const VkVertexInputAttributeDescription2EXT,
);
/// Khronos: [vkCmdSetVertexInputEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html) (nullable)
pub type PFN_vkCmdSetVertexInputEXT = Option<vkCmdSetVertexInputEXT_t>;
const vkCmdSetVertexInputEXT_NAME: &str = "vkCmdSetVertexInputEXT\0";

/// Khronos: [vkCmdSetViewport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html) (non-nullable)
pub type vkCmdSetViewport_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_viewport: u32,
  viewport_count: u32,
  viewports: *const VkViewport,
);
/// Khronos: [vkCmdSetViewport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html) (nullable)
pub type PFN_vkCmdSetViewport = Option<vkCmdSetViewport_t>;
const vkCmdSetViewport_NAME: &str = "vkCmdSetViewport\0";

/// Khronos: [vkCmdSetViewportShadingRatePaletteNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html) (non-nullable)
pub type vkCmdSetViewportShadingRatePaletteNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_viewport: u32,
  viewport_count: u32,
  shading_rate_palettes: *const VkShadingRatePaletteNV,
);
/// Khronos: [vkCmdSetViewportShadingRatePaletteNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html) (nullable)
pub type PFN_vkCmdSetViewportShadingRatePaletteNV =
  Option<vkCmdSetViewportShadingRatePaletteNV_t>;
const vkCmdSetViewportShadingRatePaletteNV_NAME: &str =
  "vkCmdSetViewportShadingRatePaletteNV\0";

/// Khronos: [vkCmdSetViewportSwizzleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html) (non-nullable)
pub type vkCmdSetViewportSwizzleNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_viewport: u32,
  viewport_count: u32,
  viewport_swizzles: *const VkViewportSwizzleNV,
);
/// Khronos: [vkCmdSetViewportSwizzleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html) (nullable)
pub type PFN_vkCmdSetViewportSwizzleNV = Option<vkCmdSetViewportSwizzleNV_t>;
const vkCmdSetViewportSwizzleNV_NAME: &str = "vkCmdSetViewportSwizzleNV\0";

/// Khronos: [vkCmdSetViewportWScalingEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html) (non-nullable)
pub type vkCmdSetViewportWScalingEnableNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  viewport_w_scaling_enable: VkBool32,
);
/// Khronos: [vkCmdSetViewportWScalingEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html) (nullable)
pub type PFN_vkCmdSetViewportWScalingEnableNV =
  Option<vkCmdSetViewportWScalingEnableNV_t>;
const vkCmdSetViewportWScalingEnableNV_NAME: &str = "vkCmdSetViewportWScalingEnableNV\0";

/// Khronos: [vkCmdSetViewportWScalingNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html) (non-nullable)
pub type vkCmdSetViewportWScalingNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_viewport: u32,
  viewport_count: u32,
  viewport_w_scalings: *const VkViewportWScalingNV,
);
/// Khronos: [vkCmdSetViewportWScalingNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html) (nullable)
pub type PFN_vkCmdSetViewportWScalingNV = Option<vkCmdSetViewportWScalingNV_t>;
const vkCmdSetViewportWScalingNV_NAME: &str = "vkCmdSetViewportWScalingNV\0";

/// Khronos: [vkCmdSetViewportWithCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html) (non-nullable)
pub type vkCmdSetViewportWithCount_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  viewport_count: u32,
  viewports: *const VkViewport,
);
/// Khronos: [vkCmdSetViewportWithCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html) (nullable)
pub type PFN_vkCmdSetViewportWithCount = Option<vkCmdSetViewportWithCount_t>;
const vkCmdSetViewportWithCount_NAME: &str = "vkCmdSetViewportWithCount\0";

/// Khronos: [vkCmdSubpassShadingHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html) (non-nullable)
pub type vkCmdSubpassShadingHUAWEI_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer);
/// Khronos: [vkCmdSubpassShadingHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html) (nullable)
pub type PFN_vkCmdSubpassShadingHUAWEI = Option<vkCmdSubpassShadingHUAWEI_t>;
const vkCmdSubpassShadingHUAWEI_NAME: &str = "vkCmdSubpassShadingHUAWEI\0";

/// Khronos: [vkCmdTraceRaysIndirect2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html) (non-nullable)
pub type vkCmdTraceRaysIndirect2KHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  indirect_device_address: VkDeviceAddress,
);
/// Khronos: [vkCmdTraceRaysIndirect2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html) (nullable)
pub type PFN_vkCmdTraceRaysIndirect2KHR = Option<vkCmdTraceRaysIndirect2KHR_t>;
const vkCmdTraceRaysIndirect2KHR_NAME: &str = "vkCmdTraceRaysIndirect2KHR\0";

/// Khronos: [vkCmdTraceRaysIndirectKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html) (non-nullable)
pub type vkCmdTraceRaysIndirectKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  raygen_shader_binding_table: *const VkStridedDeviceAddressRegionKHR,
  miss_shader_binding_table: *const VkStridedDeviceAddressRegionKHR,
  hit_shader_binding_table: *const VkStridedDeviceAddressRegionKHR,
  callable_shader_binding_table: *const VkStridedDeviceAddressRegionKHR,
  indirect_device_address: VkDeviceAddress,
);
/// Khronos: [vkCmdTraceRaysIndirectKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html) (nullable)
pub type PFN_vkCmdTraceRaysIndirectKHR = Option<vkCmdTraceRaysIndirectKHR_t>;
const vkCmdTraceRaysIndirectKHR_NAME: &str = "vkCmdTraceRaysIndirectKHR\0";

/// Khronos: [vkCmdTraceRaysKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html) (non-nullable)
pub type vkCmdTraceRaysKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  raygen_shader_binding_table: *const VkStridedDeviceAddressRegionKHR,
  miss_shader_binding_table: *const VkStridedDeviceAddressRegionKHR,
  hit_shader_binding_table: *const VkStridedDeviceAddressRegionKHR,
  callable_shader_binding_table: *const VkStridedDeviceAddressRegionKHR,
  width: u32,
  height: u32,
  depth: u32,
);
/// Khronos: [vkCmdTraceRaysKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html) (nullable)
pub type PFN_vkCmdTraceRaysKHR = Option<vkCmdTraceRaysKHR_t>;
const vkCmdTraceRaysKHR_NAME: &str = "vkCmdTraceRaysKHR\0";

/// Khronos: [vkCmdTraceRaysNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html) (non-nullable)
pub type vkCmdTraceRaysNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  raygen_shader_binding_table_buffer: VkBuffer,
  raygen_shader_binding_offset: VkDeviceSize,
  miss_shader_binding_table_buffer: VkBuffer,
  miss_shader_binding_offset: VkDeviceSize,
  miss_shader_binding_stride: VkDeviceSize,
  hit_shader_binding_table_buffer: VkBuffer,
  hit_shader_binding_offset: VkDeviceSize,
  hit_shader_binding_stride: VkDeviceSize,
  callable_shader_binding_table_buffer: VkBuffer,
  callable_shader_binding_offset: VkDeviceSize,
  callable_shader_binding_stride: VkDeviceSize,
  width: u32,
  height: u32,
  depth: u32,
);
/// Khronos: [vkCmdTraceRaysNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html) (nullable)
pub type PFN_vkCmdTraceRaysNV = Option<vkCmdTraceRaysNV_t>;
const vkCmdTraceRaysNV_NAME: &str = "vkCmdTraceRaysNV\0";

/// Khronos: [vkCmdUpdateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html) (non-nullable)
pub type vkCmdUpdateBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  dst_buffer: VkBuffer,
  dst_offset: VkDeviceSize,
  data_size: VkDeviceSize,
  data: *const c_void,
);
/// Khronos: [vkCmdUpdateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html) (nullable)
pub type PFN_vkCmdUpdateBuffer = Option<vkCmdUpdateBuffer_t>;
const vkCmdUpdateBuffer_NAME: &str = "vkCmdUpdateBuffer\0";

/// Khronos: [vkCmdWaitEvents](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html) (non-nullable)
pub type vkCmdWaitEvents_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  event_count: u32,
  events: *const VkEvent,
  src_stage_mask: VkPipelineStageFlags,
  dst_stage_mask: VkPipelineStageFlags,
  memory_barrier_count: u32,
  memory_barriers: *const VkMemoryBarrier,
  buffer_memory_barrier_count: u32,
  buffer_memory_barriers: *const VkBufferMemoryBarrier,
  image_memory_barrier_count: u32,
  image_memory_barriers: *const VkImageMemoryBarrier,
);
/// Khronos: [vkCmdWaitEvents](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html) (nullable)
pub type PFN_vkCmdWaitEvents = Option<vkCmdWaitEvents_t>;
const vkCmdWaitEvents_NAME: &str = "vkCmdWaitEvents\0";

/// Khronos: [vkCmdWaitEvents2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html) (non-nullable)
pub type vkCmdWaitEvents2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  event_count: u32,
  events: *const VkEvent,
  dependency_infos: *const VkDependencyInfo,
);
/// Khronos: [vkCmdWaitEvents2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html) (nullable)
pub type PFN_vkCmdWaitEvents2 = Option<vkCmdWaitEvents2_t>;
const vkCmdWaitEvents2_NAME: &str = "vkCmdWaitEvents2\0";

/// Khronos: [vkCmdWriteAccelerationStructuresPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html) (non-nullable)
pub type vkCmdWriteAccelerationStructuresPropertiesKHR_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  acceleration_structure_count: u32,
  acceleration_structures: *const VkAccelerationStructureKHR,
  query_type: VkQueryType,
  query_pool: VkQueryPool,
  first_query: u32,
);
/// Khronos: [vkCmdWriteAccelerationStructuresPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html) (nullable)
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR =
  Option<vkCmdWriteAccelerationStructuresPropertiesKHR_t>;
const vkCmdWriteAccelerationStructuresPropertiesKHR_NAME: &str =
  "vkCmdWriteAccelerationStructuresPropertiesKHR\0";

/// Khronos: [vkCmdWriteAccelerationStructuresPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) (non-nullable)
pub type vkCmdWriteAccelerationStructuresPropertiesNV_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  acceleration_structure_count: u32,
  acceleration_structures: *const VkAccelerationStructureNV,
  query_type: VkQueryType,
  query_pool: VkQueryPool,
  first_query: u32,
);
/// Khronos: [vkCmdWriteAccelerationStructuresPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) (nullable)
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV =
  Option<vkCmdWriteAccelerationStructuresPropertiesNV_t>;
const vkCmdWriteAccelerationStructuresPropertiesNV_NAME: &str =
  "vkCmdWriteAccelerationStructuresPropertiesNV\0";

/// Khronos: [vkCmdWriteBufferMarker2AMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html) (non-nullable)
pub type vkCmdWriteBufferMarker2AMD_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  stage: VkPipelineStageFlags2,
  dst_buffer: VkBuffer,
  dst_offset: VkDeviceSize,
  marker: u32,
);
/// Khronos: [vkCmdWriteBufferMarker2AMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html) (nullable)
pub type PFN_vkCmdWriteBufferMarker2AMD = Option<vkCmdWriteBufferMarker2AMD_t>;
const vkCmdWriteBufferMarker2AMD_NAME: &str = "vkCmdWriteBufferMarker2AMD\0";

/// Khronos: [vkCmdWriteBufferMarkerAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html) (non-nullable)
pub type vkCmdWriteBufferMarkerAMD_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_stage: VkPipelineStageFlagBits,
  dst_buffer: VkBuffer,
  dst_offset: VkDeviceSize,
  marker: u32,
);
/// Khronos: [vkCmdWriteBufferMarkerAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html) (nullable)
pub type PFN_vkCmdWriteBufferMarkerAMD = Option<vkCmdWriteBufferMarkerAMD_t>;
const vkCmdWriteBufferMarkerAMD_NAME: &str = "vkCmdWriteBufferMarkerAMD\0";

/// Khronos: [vkCmdWriteMicromapsPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteMicromapsPropertiesEXT.html) (non-nullable)
pub type vkCmdWriteMicromapsPropertiesEXT_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  micromap_count: u32,
  micromaps: *const VkMicromapEXT,
  query_type: VkQueryType,
  query_pool: VkQueryPool,
  first_query: u32,
);
/// Khronos: [vkCmdWriteMicromapsPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteMicromapsPropertiesEXT.html) (nullable)
pub type PFN_vkCmdWriteMicromapsPropertiesEXT =
  Option<vkCmdWriteMicromapsPropertiesEXT_t>;
const vkCmdWriteMicromapsPropertiesEXT_NAME: &str = "vkCmdWriteMicromapsPropertiesEXT\0";

/// Khronos: [vkCmdWriteTimestamp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html) (non-nullable)
pub type vkCmdWriteTimestamp_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_stage: VkPipelineStageFlagBits,
  query_pool: VkQueryPool,
  query: u32,
);
/// Khronos: [vkCmdWriteTimestamp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html) (nullable)
pub type PFN_vkCmdWriteTimestamp = Option<vkCmdWriteTimestamp_t>;
const vkCmdWriteTimestamp_NAME: &str = "vkCmdWriteTimestamp\0";

/// Khronos: [vkCmdWriteTimestamp2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html) (non-nullable)
pub type vkCmdWriteTimestamp2_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  stage: VkPipelineStageFlags2,
  query_pool: VkQueryPool,
  query: u32,
);
/// Khronos: [vkCmdWriteTimestamp2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html) (nullable)
pub type PFN_vkCmdWriteTimestamp2 = Option<vkCmdWriteTimestamp2_t>;
const vkCmdWriteTimestamp2_NAME: &str = "vkCmdWriteTimestamp2\0";

/// Khronos: [vkCompileDeferredNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html) (non-nullable)
pub type vkCompileDeferredNV_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline: VkPipeline,
  shader: u32,
) -> VkResult;
/// Khronos: [vkCompileDeferredNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html) (nullable)
pub type PFN_vkCompileDeferredNV = Option<vkCompileDeferredNV_t>;
const vkCompileDeferredNV_NAME: &str = "vkCompileDeferredNV\0";

/// Khronos: [vkCopyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html) (non-nullable)
pub type vkCopyAccelerationStructureKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  deferred_operation: VkDeferredOperationKHR,
  info: *const VkCopyAccelerationStructureInfoKHR,
) -> VkResult;
/// Khronos: [vkCopyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html) (nullable)
pub type PFN_vkCopyAccelerationStructureKHR = Option<vkCopyAccelerationStructureKHR_t>;
const vkCopyAccelerationStructureKHR_NAME: &str = "vkCopyAccelerationStructureKHR\0";

/// Khronos: [vkCopyMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapEXT.html) (non-nullable)
pub type vkCopyMicromapEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  deferred_operation: VkDeferredOperationKHR,
  info: *const VkCopyMicromapInfoEXT,
) -> VkResult;
/// Khronos: [vkCopyMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapEXT.html) (nullable)
pub type PFN_vkCopyMicromapEXT = Option<vkCopyMicromapEXT_t>;
const vkCopyMicromapEXT_NAME: &str = "vkCopyMicromapEXT\0";

/// Khronos: [vkCreateAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html) (non-nullable)
pub type vkCreateAccelerationStructureKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkAccelerationStructureCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  acceleration_structure: *mut VkAccelerationStructureKHR,
) -> VkResult;
/// Khronos: [vkCreateAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html) (nullable)
pub type PFN_vkCreateAccelerationStructureKHR =
  Option<vkCreateAccelerationStructureKHR_t>;
const vkCreateAccelerationStructureKHR_NAME: &str = "vkCreateAccelerationStructureKHR\0";

/// Khronos: [vkCreateAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html) (non-nullable)
pub type vkCreateAccelerationStructureNV_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkAccelerationStructureCreateInfoNV,
  allocator: *const VkAllocationCallbacks,
  acceleration_structure: *mut VkAccelerationStructureNV,
) -> VkResult;
/// Khronos: [vkCreateAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html) (nullable)
pub type PFN_vkCreateAccelerationStructureNV = Option<vkCreateAccelerationStructureNV_t>;
const vkCreateAccelerationStructureNV_NAME: &str = "vkCreateAccelerationStructureNV\0";

/// Khronos: [vkCreateAndroidSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html) (non-nullable)
pub type vkCreateAndroidSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkAndroidSurfaceCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateAndroidSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html) (nullable)
pub type PFN_vkCreateAndroidSurfaceKHR = Option<vkCreateAndroidSurfaceKHR_t>;
const vkCreateAndroidSurfaceKHR_NAME: &str = "vkCreateAndroidSurfaceKHR\0";

/// Khronos: [vkCreateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html) (non-nullable)
pub type vkCreateBuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkBufferCreateInfo,
  allocator: *const VkAllocationCallbacks,
  buffer: *mut VkBuffer,
) -> VkResult;
/// Khronos: [vkCreateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html) (nullable)
pub type PFN_vkCreateBuffer = Option<vkCreateBuffer_t>;
const vkCreateBuffer_NAME: &str = "vkCreateBuffer\0";

/// Khronos: [vkCreateBufferCollectionFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html) (non-nullable)
pub type vkCreateBufferCollectionFUCHSIA_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkBufferCollectionCreateInfoFUCHSIA,
  allocator: *const VkAllocationCallbacks,
  collection: *mut VkBufferCollectionFUCHSIA,
) -> VkResult;
/// Khronos: [vkCreateBufferCollectionFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html) (nullable)
pub type PFN_vkCreateBufferCollectionFUCHSIA = Option<vkCreateBufferCollectionFUCHSIA_t>;
const vkCreateBufferCollectionFUCHSIA_NAME: &str = "vkCreateBufferCollectionFUCHSIA\0";

/// Khronos: [vkCreateBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html) (non-nullable)
pub type vkCreateBufferView_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkBufferViewCreateInfo,
  allocator: *const VkAllocationCallbacks,
  view: *mut VkBufferView,
) -> VkResult;
/// Khronos: [vkCreateBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html) (nullable)
pub type PFN_vkCreateBufferView = Option<vkCreateBufferView_t>;
const vkCreateBufferView_NAME: &str = "vkCreateBufferView\0";

/// Khronos: [vkCreateCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html) (non-nullable)
pub type vkCreateCommandPool_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkCommandPoolCreateInfo,
  allocator: *const VkAllocationCallbacks,
  command_pool: *mut VkCommandPool,
) -> VkResult;
/// Khronos: [vkCreateCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html) (nullable)
pub type PFN_vkCreateCommandPool = Option<vkCreateCommandPool_t>;
const vkCreateCommandPool_NAME: &str = "vkCreateCommandPool\0";

/// Khronos: [vkCreateComputePipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html) (non-nullable)
pub type vkCreateComputePipelines_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_cache: VkPipelineCache,
  create_info_count: u32,
  create_infos: *const VkComputePipelineCreateInfo,
  allocator: *const VkAllocationCallbacks,
  pipelines: *mut VkPipeline,
) -> VkResult;
/// Khronos: [vkCreateComputePipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html) (nullable)
pub type PFN_vkCreateComputePipelines = Option<vkCreateComputePipelines_t>;
const vkCreateComputePipelines_NAME: &str = "vkCreateComputePipelines\0";

/// Khronos: [vkCreateCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html) (non-nullable)
pub type vkCreateCuFunctionNVX_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkCuFunctionCreateInfoNVX,
  allocator: *const VkAllocationCallbacks,
  function: *mut VkCuFunctionNVX,
) -> VkResult;
/// Khronos: [vkCreateCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html) (nullable)
pub type PFN_vkCreateCuFunctionNVX = Option<vkCreateCuFunctionNVX_t>;
const vkCreateCuFunctionNVX_NAME: &str = "vkCreateCuFunctionNVX\0";

/// Khronos: [vkCreateCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html) (non-nullable)
pub type vkCreateCuModuleNVX_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkCuModuleCreateInfoNVX,
  allocator: *const VkAllocationCallbacks,
  module: *mut VkCuModuleNVX,
) -> VkResult;
/// Khronos: [vkCreateCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html) (nullable)
pub type PFN_vkCreateCuModuleNVX = Option<vkCreateCuModuleNVX_t>;
const vkCreateCuModuleNVX_NAME: &str = "vkCreateCuModuleNVX\0";

/// Khronos: [vkCreateDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html) (non-nullable)
pub type vkCreateDebugReportCallbackEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkDebugReportCallbackCreateInfoEXT,
  allocator: *const VkAllocationCallbacks,
  callback: *mut VkDebugReportCallbackEXT,
) -> VkResult;
/// Khronos: [vkCreateDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html) (nullable)
pub type PFN_vkCreateDebugReportCallbackEXT = Option<vkCreateDebugReportCallbackEXT_t>;
const vkCreateDebugReportCallbackEXT_NAME: &str = "vkCreateDebugReportCallbackEXT\0";

/// Khronos: [vkCreateDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) (non-nullable)
pub type vkCreateDebugUtilsMessengerEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkDebugUtilsMessengerCreateInfoEXT,
  allocator: *const VkAllocationCallbacks,
  messenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult;
/// Khronos: [vkCreateDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) (nullable)
pub type PFN_vkCreateDebugUtilsMessengerEXT = Option<vkCreateDebugUtilsMessengerEXT_t>;
const vkCreateDebugUtilsMessengerEXT_NAME: &str = "vkCreateDebugUtilsMessengerEXT\0";

/// Khronos: [vkCreateDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html) (non-nullable)
pub type vkCreateDeferredOperationKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  allocator: *const VkAllocationCallbacks,
  deferred_operation: *mut VkDeferredOperationKHR,
) -> VkResult;
/// Khronos: [vkCreateDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html) (nullable)
pub type PFN_vkCreateDeferredOperationKHR = Option<vkCreateDeferredOperationKHR_t>;
const vkCreateDeferredOperationKHR_NAME: &str = "vkCreateDeferredOperationKHR\0";

/// Khronos: [vkCreateDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html) (non-nullable)
pub type vkCreateDescriptorPool_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkDescriptorPoolCreateInfo,
  allocator: *const VkAllocationCallbacks,
  descriptor_pool: *mut VkDescriptorPool,
) -> VkResult;
/// Khronos: [vkCreateDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html) (nullable)
pub type PFN_vkCreateDescriptorPool = Option<vkCreateDescriptorPool_t>;
const vkCreateDescriptorPool_NAME: &str = "vkCreateDescriptorPool\0";

/// Khronos: [vkCreateDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html) (non-nullable)
pub type vkCreateDescriptorSetLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkDescriptorSetLayoutCreateInfo,
  allocator: *const VkAllocationCallbacks,
  set_layout: *mut VkDescriptorSetLayout,
) -> VkResult;
/// Khronos: [vkCreateDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html) (nullable)
pub type PFN_vkCreateDescriptorSetLayout = Option<vkCreateDescriptorSetLayout_t>;
const vkCreateDescriptorSetLayout_NAME: &str = "vkCreateDescriptorSetLayout\0";

/// Khronos: [vkCreateDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html) (non-nullable)
pub type vkCreateDescriptorUpdateTemplate_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkDescriptorUpdateTemplateCreateInfo,
  allocator: *const VkAllocationCallbacks,
  descriptor_update_template: *mut VkDescriptorUpdateTemplate,
) -> VkResult;
/// Khronos: [vkCreateDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html) (nullable)
pub type PFN_vkCreateDescriptorUpdateTemplate =
  Option<vkCreateDescriptorUpdateTemplate_t>;
const vkCreateDescriptorUpdateTemplate_NAME: &str = "vkCreateDescriptorUpdateTemplate\0";

/// Khronos: [vkCreateDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html) (non-nullable)
pub type vkCreateDevice_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  create_info: *const VkDeviceCreateInfo,
  allocator: *const VkAllocationCallbacks,
  device: *mut VkDevice,
) -> VkResult;
/// Khronos: [vkCreateDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html) (nullable)
pub type PFN_vkCreateDevice = Option<vkCreateDevice_t>;
const vkCreateDevice_NAME: &str = "vkCreateDevice\0";

/// Khronos: [vkCreateDirectFBSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html) (non-nullable)
pub type vkCreateDirectFBSurfaceEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkDirectFBSurfaceCreateInfoEXT,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateDirectFBSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html) (nullable)
pub type PFN_vkCreateDirectFBSurfaceEXT = Option<vkCreateDirectFBSurfaceEXT_t>;
const vkCreateDirectFBSurfaceEXT_NAME: &str = "vkCreateDirectFBSurfaceEXT\0";

/// Khronos: [vkCreateDisplayModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html) (non-nullable)
pub type vkCreateDisplayModeKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  display: VkDisplayKHR,
  create_info: *const VkDisplayModeCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  mode: *mut VkDisplayModeKHR,
) -> VkResult;
/// Khronos: [vkCreateDisplayModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html) (nullable)
pub type PFN_vkCreateDisplayModeKHR = Option<vkCreateDisplayModeKHR_t>;
const vkCreateDisplayModeKHR_NAME: &str = "vkCreateDisplayModeKHR\0";

/// Khronos: [vkCreateDisplayPlaneSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html) (non-nullable)
pub type vkCreateDisplayPlaneSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkDisplaySurfaceCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateDisplayPlaneSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html) (nullable)
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = Option<vkCreateDisplayPlaneSurfaceKHR_t>;
const vkCreateDisplayPlaneSurfaceKHR_NAME: &str = "vkCreateDisplayPlaneSurfaceKHR\0";

/// Khronos: [vkCreateEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html) (non-nullable)
pub type vkCreateEvent_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkEventCreateInfo,
  allocator: *const VkAllocationCallbacks,
  event: *mut VkEvent,
) -> VkResult;
/// Khronos: [vkCreateEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html) (nullable)
pub type PFN_vkCreateEvent = Option<vkCreateEvent_t>;
const vkCreateEvent_NAME: &str = "vkCreateEvent\0";

/// Khronos: [vkCreateFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html) (non-nullable)
pub type vkCreateFence_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkFenceCreateInfo,
  allocator: *const VkAllocationCallbacks,
  fence: *mut VkFence,
) -> VkResult;
/// Khronos: [vkCreateFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html) (nullable)
pub type PFN_vkCreateFence = Option<vkCreateFence_t>;
const vkCreateFence_NAME: &str = "vkCreateFence\0";

/// Khronos: [vkCreateFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html) (non-nullable)
pub type vkCreateFramebuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkFramebufferCreateInfo,
  allocator: *const VkAllocationCallbacks,
  framebuffer: *mut VkFramebuffer,
) -> VkResult;
/// Khronos: [vkCreateFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html) (nullable)
pub type PFN_vkCreateFramebuffer = Option<vkCreateFramebuffer_t>;
const vkCreateFramebuffer_NAME: &str = "vkCreateFramebuffer\0";

/// Khronos: [vkCreateGraphicsPipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html) (non-nullable)
pub type vkCreateGraphicsPipelines_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_cache: VkPipelineCache,
  create_info_count: u32,
  create_infos: *const VkGraphicsPipelineCreateInfo,
  allocator: *const VkAllocationCallbacks,
  pipelines: *mut VkPipeline,
) -> VkResult;
/// Khronos: [vkCreateGraphicsPipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html) (nullable)
pub type PFN_vkCreateGraphicsPipelines = Option<vkCreateGraphicsPipelines_t>;
const vkCreateGraphicsPipelines_NAME: &str = "vkCreateGraphicsPipelines\0";

/// Khronos: [vkCreateHeadlessSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html) (non-nullable)
pub type vkCreateHeadlessSurfaceEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkHeadlessSurfaceCreateInfoEXT,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateHeadlessSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html) (nullable)
pub type PFN_vkCreateHeadlessSurfaceEXT = Option<vkCreateHeadlessSurfaceEXT_t>;
const vkCreateHeadlessSurfaceEXT_NAME: &str = "vkCreateHeadlessSurfaceEXT\0";

/// Khronos: [vkCreateIOSSurfaceMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html) (non-nullable)
pub type vkCreateIOSSurfaceMVK_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkIOSSurfaceCreateInfoMVK,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateIOSSurfaceMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html) (nullable)
pub type PFN_vkCreateIOSSurfaceMVK = Option<vkCreateIOSSurfaceMVK_t>;
const vkCreateIOSSurfaceMVK_NAME: &str = "vkCreateIOSSurfaceMVK\0";

/// Khronos: [vkCreateImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html) (non-nullable)
pub type vkCreateImage_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkImageCreateInfo,
  allocator: *const VkAllocationCallbacks,
  image: *mut VkImage,
) -> VkResult;
/// Khronos: [vkCreateImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html) (nullable)
pub type PFN_vkCreateImage = Option<vkCreateImage_t>;
const vkCreateImage_NAME: &str = "vkCreateImage\0";

/// Khronos: [vkCreateImagePipeSurfaceFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html) (non-nullable)
pub type vkCreateImagePipeSurfaceFUCHSIA_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkImagePipeSurfaceCreateInfoFUCHSIA,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateImagePipeSurfaceFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html) (nullable)
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = Option<vkCreateImagePipeSurfaceFUCHSIA_t>;
const vkCreateImagePipeSurfaceFUCHSIA_NAME: &str = "vkCreateImagePipeSurfaceFUCHSIA\0";

/// Khronos: [vkCreateImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html) (non-nullable)
pub type vkCreateImageView_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkImageViewCreateInfo,
  allocator: *const VkAllocationCallbacks,
  view: *mut VkImageView,
) -> VkResult;
/// Khronos: [vkCreateImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html) (nullable)
pub type PFN_vkCreateImageView = Option<vkCreateImageView_t>;
const vkCreateImageView_NAME: &str = "vkCreateImageView\0";

/// Khronos: [vkCreateIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html) (non-nullable)
pub type vkCreateIndirectCommandsLayoutNV_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkIndirectCommandsLayoutCreateInfoNV,
  allocator: *const VkAllocationCallbacks,
  indirect_commands_layout: *mut VkIndirectCommandsLayoutNV,
) -> VkResult;
/// Khronos: [vkCreateIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html) (nullable)
pub type PFN_vkCreateIndirectCommandsLayoutNV =
  Option<vkCreateIndirectCommandsLayoutNV_t>;
const vkCreateIndirectCommandsLayoutNV_NAME: &str = "vkCreateIndirectCommandsLayoutNV\0";

/// Khronos: [vkCreateInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html) (non-nullable)
pub type vkCreateInstance_t = unsafe extern "system" fn(
  create_info: *const VkInstanceCreateInfo,
  allocator: *const VkAllocationCallbacks,
  instance: *mut VkInstance,
) -> VkResult;
/// Khronos: [vkCreateInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html) (nullable)
pub type PFN_vkCreateInstance = Option<vkCreateInstance_t>;
const vkCreateInstance_NAME: &str = "vkCreateInstance\0";

/// Khronos: [vkCreateMacOSSurfaceMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html) (non-nullable)
pub type vkCreateMacOSSurfaceMVK_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkMacOSSurfaceCreateInfoMVK,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateMacOSSurfaceMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html) (nullable)
pub type PFN_vkCreateMacOSSurfaceMVK = Option<vkCreateMacOSSurfaceMVK_t>;
const vkCreateMacOSSurfaceMVK_NAME: &str = "vkCreateMacOSSurfaceMVK\0";

/// Khronos: [vkCreateMetalSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html) (non-nullable)
pub type vkCreateMetalSurfaceEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkMetalSurfaceCreateInfoEXT,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateMetalSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html) (nullable)
pub type PFN_vkCreateMetalSurfaceEXT = Option<vkCreateMetalSurfaceEXT_t>;
const vkCreateMetalSurfaceEXT_NAME: &str = "vkCreateMetalSurfaceEXT\0";

/// Khronos: [vkCreateMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMicromapEXT.html) (non-nullable)
pub type vkCreateMicromapEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkMicromapCreateInfoEXT,
  allocator: *const VkAllocationCallbacks,
  micromap: *mut VkMicromapEXT,
) -> VkResult;
/// Khronos: [vkCreateMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMicromapEXT.html) (nullable)
pub type PFN_vkCreateMicromapEXT = Option<vkCreateMicromapEXT_t>;
const vkCreateMicromapEXT_NAME: &str = "vkCreateMicromapEXT\0";

/// Khronos: [vkCreateOpticalFlowSessionNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateOpticalFlowSessionNV.html) (non-nullable)
pub type vkCreateOpticalFlowSessionNV_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkOpticalFlowSessionCreateInfoNV,
  allocator: *const VkAllocationCallbacks,
  session: *mut VkOpticalFlowSessionNV,
) -> VkResult;
/// Khronos: [vkCreateOpticalFlowSessionNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateOpticalFlowSessionNV.html) (nullable)
pub type PFN_vkCreateOpticalFlowSessionNV = Option<vkCreateOpticalFlowSessionNV_t>;
const vkCreateOpticalFlowSessionNV_NAME: &str = "vkCreateOpticalFlowSessionNV\0";

/// Khronos: [vkCreatePipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html) (non-nullable)
pub type vkCreatePipelineCache_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkPipelineCacheCreateInfo,
  allocator: *const VkAllocationCallbacks,
  pipeline_cache: *mut VkPipelineCache,
) -> VkResult;
/// Khronos: [vkCreatePipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html) (nullable)
pub type PFN_vkCreatePipelineCache = Option<vkCreatePipelineCache_t>;
const vkCreatePipelineCache_NAME: &str = "vkCreatePipelineCache\0";

/// Khronos: [vkCreatePipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html) (non-nullable)
pub type vkCreatePipelineLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkPipelineLayoutCreateInfo,
  allocator: *const VkAllocationCallbacks,
  pipeline_layout: *mut VkPipelineLayout,
) -> VkResult;
/// Khronos: [vkCreatePipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html) (nullable)
pub type PFN_vkCreatePipelineLayout = Option<vkCreatePipelineLayout_t>;
const vkCreatePipelineLayout_NAME: &str = "vkCreatePipelineLayout\0";

/// Khronos: [vkCreatePrivateDataSlot](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html) (non-nullable)
pub type vkCreatePrivateDataSlot_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkPrivateDataSlotCreateInfo,
  allocator: *const VkAllocationCallbacks,
  private_data_slot: *mut VkPrivateDataSlot,
) -> VkResult;
/// Khronos: [vkCreatePrivateDataSlot](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html) (nullable)
pub type PFN_vkCreatePrivateDataSlot = Option<vkCreatePrivateDataSlot_t>;
const vkCreatePrivateDataSlot_NAME: &str = "vkCreatePrivateDataSlot\0";

/// Khronos: [vkCreateQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html) (non-nullable)
pub type vkCreateQueryPool_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkQueryPoolCreateInfo,
  allocator: *const VkAllocationCallbacks,
  query_pool: *mut VkQueryPool,
) -> VkResult;
/// Khronos: [vkCreateQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html) (nullable)
pub type PFN_vkCreateQueryPool = Option<vkCreateQueryPool_t>;
const vkCreateQueryPool_NAME: &str = "vkCreateQueryPool\0";

/// Khronos: [vkCreateRayTracingPipelinesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html) (non-nullable)
pub type vkCreateRayTracingPipelinesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  deferred_operation: VkDeferredOperationKHR,
  pipeline_cache: VkPipelineCache,
  create_info_count: u32,
  create_infos: *const VkRayTracingPipelineCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  pipelines: *mut VkPipeline,
) -> VkResult;
/// Khronos: [vkCreateRayTracingPipelinesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html) (nullable)
pub type PFN_vkCreateRayTracingPipelinesKHR = Option<vkCreateRayTracingPipelinesKHR_t>;
const vkCreateRayTracingPipelinesKHR_NAME: &str = "vkCreateRayTracingPipelinesKHR\0";

/// Khronos: [vkCreateRayTracingPipelinesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html) (non-nullable)
pub type vkCreateRayTracingPipelinesNV_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_cache: VkPipelineCache,
  create_info_count: u32,
  create_infos: *const VkRayTracingPipelineCreateInfoNV,
  allocator: *const VkAllocationCallbacks,
  pipelines: *mut VkPipeline,
) -> VkResult;
/// Khronos: [vkCreateRayTracingPipelinesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html) (nullable)
pub type PFN_vkCreateRayTracingPipelinesNV = Option<vkCreateRayTracingPipelinesNV_t>;
const vkCreateRayTracingPipelinesNV_NAME: &str = "vkCreateRayTracingPipelinesNV\0";

/// Khronos: [vkCreateRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html) (non-nullable)
pub type vkCreateRenderPass_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkRenderPassCreateInfo,
  allocator: *const VkAllocationCallbacks,
  render_pass: *mut VkRenderPass,
) -> VkResult;
/// Khronos: [vkCreateRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html) (nullable)
pub type PFN_vkCreateRenderPass = Option<vkCreateRenderPass_t>;
const vkCreateRenderPass_NAME: &str = "vkCreateRenderPass\0";

/// Khronos: [vkCreateRenderPass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html) (non-nullable)
pub type vkCreateRenderPass2_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkRenderPassCreateInfo2,
  allocator: *const VkAllocationCallbacks,
  render_pass: *mut VkRenderPass,
) -> VkResult;
/// Khronos: [vkCreateRenderPass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html) (nullable)
pub type PFN_vkCreateRenderPass2 = Option<vkCreateRenderPass2_t>;
const vkCreateRenderPass2_NAME: &str = "vkCreateRenderPass2\0";

/// Khronos: [vkCreateSampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html) (non-nullable)
pub type vkCreateSampler_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkSamplerCreateInfo,
  allocator: *const VkAllocationCallbacks,
  sampler: *mut VkSampler,
) -> VkResult;
/// Khronos: [vkCreateSampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html) (nullable)
pub type PFN_vkCreateSampler = Option<vkCreateSampler_t>;
const vkCreateSampler_NAME: &str = "vkCreateSampler\0";

/// Khronos: [vkCreateSamplerYcbcrConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html) (non-nullable)
pub type vkCreateSamplerYcbcrConversion_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkSamplerYcbcrConversionCreateInfo,
  allocator: *const VkAllocationCallbacks,
  ycbcr_conversion: *mut VkSamplerYcbcrConversion,
) -> VkResult;
/// Khronos: [vkCreateSamplerYcbcrConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html) (nullable)
pub type PFN_vkCreateSamplerYcbcrConversion = Option<vkCreateSamplerYcbcrConversion_t>;
const vkCreateSamplerYcbcrConversion_NAME: &str = "vkCreateSamplerYcbcrConversion\0";

/// Khronos: [vkCreateScreenSurfaceQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html) (non-nullable)
pub type vkCreateScreenSurfaceQNX_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkScreenSurfaceCreateInfoQNX,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateScreenSurfaceQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html) (nullable)
pub type PFN_vkCreateScreenSurfaceQNX = Option<vkCreateScreenSurfaceQNX_t>;
const vkCreateScreenSurfaceQNX_NAME: &str = "vkCreateScreenSurfaceQNX\0";

/// Khronos: [vkCreateSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html) (non-nullable)
pub type vkCreateSemaphore_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkSemaphoreCreateInfo,
  allocator: *const VkAllocationCallbacks,
  semaphore: *mut VkSemaphore,
) -> VkResult;
/// Khronos: [vkCreateSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html) (nullable)
pub type PFN_vkCreateSemaphore = Option<vkCreateSemaphore_t>;
const vkCreateSemaphore_NAME: &str = "vkCreateSemaphore\0";

/// Khronos: [vkCreateShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html) (non-nullable)
pub type vkCreateShaderModule_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkShaderModuleCreateInfo,
  allocator: *const VkAllocationCallbacks,
  shader_module: *mut VkShaderModule,
) -> VkResult;
/// Khronos: [vkCreateShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html) (nullable)
pub type PFN_vkCreateShaderModule = Option<vkCreateShaderModule_t>;
const vkCreateShaderModule_NAME: &str = "vkCreateShaderModule\0";

/// Khronos: [vkCreateSharedSwapchainsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html) (non-nullable)
pub type vkCreateSharedSwapchainsKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain_count: u32,
  create_infos: *const VkSwapchainCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  swapchains: *mut VkSwapchainKHR,
) -> VkResult;
/// Khronos: [vkCreateSharedSwapchainsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html) (nullable)
pub type PFN_vkCreateSharedSwapchainsKHR = Option<vkCreateSharedSwapchainsKHR_t>;
const vkCreateSharedSwapchainsKHR_NAME: &str = "vkCreateSharedSwapchainsKHR\0";

/// Khronos: [vkCreateSwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html) (non-nullable)
pub type vkCreateSwapchainKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkSwapchainCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  swapchain: *mut VkSwapchainKHR,
) -> VkResult;
/// Khronos: [vkCreateSwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html) (nullable)
pub type PFN_vkCreateSwapchainKHR = Option<vkCreateSwapchainKHR_t>;
const vkCreateSwapchainKHR_NAME: &str = "vkCreateSwapchainKHR\0";

/// Khronos: [vkCreateValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html) (non-nullable)
pub type vkCreateValidationCacheEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkValidationCacheCreateInfoEXT,
  allocator: *const VkAllocationCallbacks,
  validation_cache: *mut VkValidationCacheEXT,
) -> VkResult;
/// Khronos: [vkCreateValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html) (nullable)
pub type PFN_vkCreateValidationCacheEXT = Option<vkCreateValidationCacheEXT_t>;
const vkCreateValidationCacheEXT_NAME: &str = "vkCreateValidationCacheEXT\0";

/// Khronos: [vkCreateViSurfaceNN](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html) (non-nullable)
pub type vkCreateViSurfaceNN_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkViSurfaceCreateInfoNN,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateViSurfaceNN](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html) (nullable)
pub type PFN_vkCreateViSurfaceNN = Option<vkCreateViSurfaceNN_t>;
const vkCreateViSurfaceNN_NAME: &str = "vkCreateViSurfaceNN\0";

/// Khronos: [vkCreateVideoSessionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html) (non-nullable)
pub type vkCreateVideoSessionKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkVideoSessionCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  video_session: *mut VkVideoSessionKHR,
) -> VkResult;
/// Khronos: [vkCreateVideoSessionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html) (nullable)
pub type PFN_vkCreateVideoSessionKHR = Option<vkCreateVideoSessionKHR_t>;
const vkCreateVideoSessionKHR_NAME: &str = "vkCreateVideoSessionKHR\0";

/// Khronos: [vkCreateVideoSessionParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html) (non-nullable)
pub type vkCreateVideoSessionParametersKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkVideoSessionParametersCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  video_session_parameters: *mut VkVideoSessionParametersKHR,
) -> VkResult;
/// Khronos: [vkCreateVideoSessionParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html) (nullable)
pub type PFN_vkCreateVideoSessionParametersKHR =
  Option<vkCreateVideoSessionParametersKHR_t>;
const vkCreateVideoSessionParametersKHR_NAME: &str =
  "vkCreateVideoSessionParametersKHR\0";

/// Khronos: [vkCreateWaylandSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html) (non-nullable)
pub type vkCreateWaylandSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkWaylandSurfaceCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateWaylandSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html) (nullable)
pub type PFN_vkCreateWaylandSurfaceKHR = Option<vkCreateWaylandSurfaceKHR_t>;
const vkCreateWaylandSurfaceKHR_NAME: &str = "vkCreateWaylandSurfaceKHR\0";

/// Khronos: [vkCreateWin32SurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html) (non-nullable)
pub type vkCreateWin32SurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkWin32SurfaceCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateWin32SurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html) (nullable)
pub type PFN_vkCreateWin32SurfaceKHR = Option<vkCreateWin32SurfaceKHR_t>;
const vkCreateWin32SurfaceKHR_NAME: &str = "vkCreateWin32SurfaceKHR\0";

/// Khronos: [vkCreateXcbSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html) (non-nullable)
pub type vkCreateXcbSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkXcbSurfaceCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateXcbSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html) (nullable)
pub type PFN_vkCreateXcbSurfaceKHR = Option<vkCreateXcbSurfaceKHR_t>;
const vkCreateXcbSurfaceKHR_NAME: &str = "vkCreateXcbSurfaceKHR\0";

/// Khronos: [vkCreateXlibSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html) (non-nullable)
pub type vkCreateXlibSurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  create_info: *const VkXlibSurfaceCreateInfoKHR,
  allocator: *const VkAllocationCallbacks,
  surface: *mut VkSurfaceKHR,
) -> VkResult;
/// Khronos: [vkCreateXlibSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html) (nullable)
pub type PFN_vkCreateXlibSurfaceKHR = Option<vkCreateXlibSurfaceKHR_t>;
const vkCreateXlibSurfaceKHR_NAME: &str = "vkCreateXlibSurfaceKHR\0";

/// Khronos: [vkDebugMarkerSetObjectNameEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) (non-nullable)
pub type vkDebugMarkerSetObjectNameEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  name_info: *const VkDebugMarkerObjectNameInfoEXT,
) -> VkResult;
/// Khronos: [vkDebugMarkerSetObjectNameEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) (nullable)
pub type PFN_vkDebugMarkerSetObjectNameEXT = Option<vkDebugMarkerSetObjectNameEXT_t>;
const vkDebugMarkerSetObjectNameEXT_NAME: &str = "vkDebugMarkerSetObjectNameEXT\0";

/// Khronos: [vkDebugMarkerSetObjectTagEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) (non-nullable)
pub type vkDebugMarkerSetObjectTagEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  tag_info: *const VkDebugMarkerObjectTagInfoEXT,
) -> VkResult;
/// Khronos: [vkDebugMarkerSetObjectTagEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) (nullable)
pub type PFN_vkDebugMarkerSetObjectTagEXT = Option<vkDebugMarkerSetObjectTagEXT_t>;
const vkDebugMarkerSetObjectTagEXT_NAME: &str = "vkDebugMarkerSetObjectTagEXT\0";

/// Khronos: [vkDebugReportMessageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html) (non-nullable)
pub type vkDebugReportMessageEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  flags: VkDebugReportFlagsEXT,
  object_type: VkDebugReportObjectTypeEXT,
  object: u64,
  location: c_size_t,
  message_code: i32,
  layer_prefix: *const u8,
  message: *const u8,
);
/// Khronos: [vkDebugReportMessageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html) (nullable)
pub type PFN_vkDebugReportMessageEXT = Option<vkDebugReportMessageEXT_t>;
const vkDebugReportMessageEXT_NAME: &str = "vkDebugReportMessageEXT\0";

/// Khronos: [vkDeferredOperationJoinKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html) (non-nullable)
pub type vkDeferredOperationJoinKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  operation: VkDeferredOperationKHR,
) -> VkResult;
/// Khronos: [vkDeferredOperationJoinKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html) (nullable)
pub type PFN_vkDeferredOperationJoinKHR = Option<vkDeferredOperationJoinKHR_t>;
const vkDeferredOperationJoinKHR_NAME: &str = "vkDeferredOperationJoinKHR\0";

/// Khronos: [vkDestroyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html) (non-nullable)
pub type vkDestroyAccelerationStructureKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  acceleration_structure: VkAccelerationStructureKHR,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html) (nullable)
pub type PFN_vkDestroyAccelerationStructureKHR =
  Option<vkDestroyAccelerationStructureKHR_t>;
const vkDestroyAccelerationStructureKHR_NAME: &str =
  "vkDestroyAccelerationStructureKHR\0";

/// Khronos: [vkDestroyAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html) (non-nullable)
pub type vkDestroyAccelerationStructureNV_t = unsafe extern "system" fn(
  device: VkDevice,
  acceleration_structure: VkAccelerationStructureNV,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html) (nullable)
pub type PFN_vkDestroyAccelerationStructureNV =
  Option<vkDestroyAccelerationStructureNV_t>;
const vkDestroyAccelerationStructureNV_NAME: &str = "vkDestroyAccelerationStructureNV\0";

/// Khronos: [vkDestroyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html) (non-nullable)
pub type vkDestroyBuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  buffer: VkBuffer,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html) (nullable)
pub type PFN_vkDestroyBuffer = Option<vkDestroyBuffer_t>;
const vkDestroyBuffer_NAME: &str = "vkDestroyBuffer\0";

/// Khronos: [vkDestroyBufferCollectionFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html) (non-nullable)
pub type vkDestroyBufferCollectionFUCHSIA_t = unsafe extern "system" fn(
  device: VkDevice,
  collection: VkBufferCollectionFUCHSIA,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyBufferCollectionFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html) (nullable)
pub type PFN_vkDestroyBufferCollectionFUCHSIA =
  Option<vkDestroyBufferCollectionFUCHSIA_t>;
const vkDestroyBufferCollectionFUCHSIA_NAME: &str = "vkDestroyBufferCollectionFUCHSIA\0";

/// Khronos: [vkDestroyBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html) (non-nullable)
pub type vkDestroyBufferView_t = unsafe extern "system" fn(
  device: VkDevice,
  buffer_view: VkBufferView,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html) (nullable)
pub type PFN_vkDestroyBufferView = Option<vkDestroyBufferView_t>;
const vkDestroyBufferView_NAME: &str = "vkDestroyBufferView\0";

/// Khronos: [vkDestroyCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html) (non-nullable)
pub type vkDestroyCommandPool_t = unsafe extern "system" fn(
  device: VkDevice,
  command_pool: VkCommandPool,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html) (nullable)
pub type PFN_vkDestroyCommandPool = Option<vkDestroyCommandPool_t>;
const vkDestroyCommandPool_NAME: &str = "vkDestroyCommandPool\0";

/// Khronos: [vkDestroyCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html) (non-nullable)
pub type vkDestroyCuFunctionNVX_t = unsafe extern "system" fn(
  device: VkDevice,
  function: VkCuFunctionNVX,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html) (nullable)
pub type PFN_vkDestroyCuFunctionNVX = Option<vkDestroyCuFunctionNVX_t>;
const vkDestroyCuFunctionNVX_NAME: &str = "vkDestroyCuFunctionNVX\0";

/// Khronos: [vkDestroyCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html) (non-nullable)
pub type vkDestroyCuModuleNVX_t = unsafe extern "system" fn(
  device: VkDevice,
  module: VkCuModuleNVX,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html) (nullable)
pub type PFN_vkDestroyCuModuleNVX = Option<vkDestroyCuModuleNVX_t>;
const vkDestroyCuModuleNVX_NAME: &str = "vkDestroyCuModuleNVX\0";

/// Khronos: [vkDestroyDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html) (non-nullable)
pub type vkDestroyDebugReportCallbackEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  callback: VkDebugReportCallbackEXT,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html) (nullable)
pub type PFN_vkDestroyDebugReportCallbackEXT = Option<vkDestroyDebugReportCallbackEXT_t>;
const vkDestroyDebugReportCallbackEXT_NAME: &str = "vkDestroyDebugReportCallbackEXT\0";

/// Khronos: [vkDestroyDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) (non-nullable)
pub type vkDestroyDebugUtilsMessengerEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  messenger: VkDebugUtilsMessengerEXT,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) (nullable)
pub type PFN_vkDestroyDebugUtilsMessengerEXT = Option<vkDestroyDebugUtilsMessengerEXT_t>;
const vkDestroyDebugUtilsMessengerEXT_NAME: &str = "vkDestroyDebugUtilsMessengerEXT\0";

/// Khronos: [vkDestroyDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html) (non-nullable)
pub type vkDestroyDeferredOperationKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  operation: VkDeferredOperationKHR,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html) (nullable)
pub type PFN_vkDestroyDeferredOperationKHR = Option<vkDestroyDeferredOperationKHR_t>;
const vkDestroyDeferredOperationKHR_NAME: &str = "vkDestroyDeferredOperationKHR\0";

/// Khronos: [vkDestroyDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html) (non-nullable)
pub type vkDestroyDescriptorPool_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_pool: VkDescriptorPool,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html) (nullable)
pub type PFN_vkDestroyDescriptorPool = Option<vkDestroyDescriptorPool_t>;
const vkDestroyDescriptorPool_NAME: &str = "vkDestroyDescriptorPool\0";

/// Khronos: [vkDestroyDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html) (non-nullable)
pub type vkDestroyDescriptorSetLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_set_layout: VkDescriptorSetLayout,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html) (nullable)
pub type PFN_vkDestroyDescriptorSetLayout = Option<vkDestroyDescriptorSetLayout_t>;
const vkDestroyDescriptorSetLayout_NAME: &str = "vkDestroyDescriptorSetLayout\0";

/// Khronos: [vkDestroyDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html) (non-nullable)
pub type vkDestroyDescriptorUpdateTemplate_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_update_template: VkDescriptorUpdateTemplate,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html) (nullable)
pub type PFN_vkDestroyDescriptorUpdateTemplate =
  Option<vkDestroyDescriptorUpdateTemplate_t>;
const vkDestroyDescriptorUpdateTemplate_NAME: &str =
  "vkDestroyDescriptorUpdateTemplate\0";

/// Khronos: [vkDestroyDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html) (non-nullable)
pub type vkDestroyDevice_t =
  unsafe extern "system" fn(device: VkDevice, allocator: *const VkAllocationCallbacks);
/// Khronos: [vkDestroyDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html) (nullable)
pub type PFN_vkDestroyDevice = Option<vkDestroyDevice_t>;
const vkDestroyDevice_NAME: &str = "vkDestroyDevice\0";

/// Khronos: [vkDestroyEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html) (non-nullable)
pub type vkDestroyEvent_t = unsafe extern "system" fn(
  device: VkDevice,
  event: VkEvent,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html) (nullable)
pub type PFN_vkDestroyEvent = Option<vkDestroyEvent_t>;
const vkDestroyEvent_NAME: &str = "vkDestroyEvent\0";

/// Khronos: [vkDestroyFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html) (non-nullable)
pub type vkDestroyFence_t = unsafe extern "system" fn(
  device: VkDevice,
  fence: VkFence,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html) (nullable)
pub type PFN_vkDestroyFence = Option<vkDestroyFence_t>;
const vkDestroyFence_NAME: &str = "vkDestroyFence\0";

/// Khronos: [vkDestroyFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html) (non-nullable)
pub type vkDestroyFramebuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  framebuffer: VkFramebuffer,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html) (nullable)
pub type PFN_vkDestroyFramebuffer = Option<vkDestroyFramebuffer_t>;
const vkDestroyFramebuffer_NAME: &str = "vkDestroyFramebuffer\0";

/// Khronos: [vkDestroyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html) (non-nullable)
pub type vkDestroyImage_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html) (nullable)
pub type PFN_vkDestroyImage = Option<vkDestroyImage_t>;
const vkDestroyImage_NAME: &str = "vkDestroyImage\0";

/// Khronos: [vkDestroyImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html) (non-nullable)
pub type vkDestroyImageView_t = unsafe extern "system" fn(
  device: VkDevice,
  image_view: VkImageView,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html) (nullable)
pub type PFN_vkDestroyImageView = Option<vkDestroyImageView_t>;
const vkDestroyImageView_NAME: &str = "vkDestroyImageView\0";

/// Khronos: [vkDestroyIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html) (non-nullable)
pub type vkDestroyIndirectCommandsLayoutNV_t = unsafe extern "system" fn(
  device: VkDevice,
  indirect_commands_layout: VkIndirectCommandsLayoutNV,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html) (nullable)
pub type PFN_vkDestroyIndirectCommandsLayoutNV =
  Option<vkDestroyIndirectCommandsLayoutNV_t>;
const vkDestroyIndirectCommandsLayoutNV_NAME: &str =
  "vkDestroyIndirectCommandsLayoutNV\0";

/// Khronos: [vkDestroyInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html) (non-nullable)
pub type vkDestroyInstance_t = unsafe extern "system" fn(
  instance: VkInstance,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html) (nullable)
pub type PFN_vkDestroyInstance = Option<vkDestroyInstance_t>;
const vkDestroyInstance_NAME: &str = "vkDestroyInstance\0";

/// Khronos: [vkDestroyMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyMicromapEXT.html) (non-nullable)
pub type vkDestroyMicromapEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  micromap: VkMicromapEXT,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyMicromapEXT.html) (nullable)
pub type PFN_vkDestroyMicromapEXT = Option<vkDestroyMicromapEXT_t>;
const vkDestroyMicromapEXT_NAME: &str = "vkDestroyMicromapEXT\0";

/// Khronos: [vkDestroyOpticalFlowSessionNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyOpticalFlowSessionNV.html) (non-nullable)
pub type vkDestroyOpticalFlowSessionNV_t = unsafe extern "system" fn(
  device: VkDevice,
  session: VkOpticalFlowSessionNV,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyOpticalFlowSessionNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyOpticalFlowSessionNV.html) (nullable)
pub type PFN_vkDestroyOpticalFlowSessionNV = Option<vkDestroyOpticalFlowSessionNV_t>;
const vkDestroyOpticalFlowSessionNV_NAME: &str = "vkDestroyOpticalFlowSessionNV\0";

/// Khronos: [vkDestroyPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html) (non-nullable)
pub type vkDestroyPipeline_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline: VkPipeline,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html) (nullable)
pub type PFN_vkDestroyPipeline = Option<vkDestroyPipeline_t>;
const vkDestroyPipeline_NAME: &str = "vkDestroyPipeline\0";

/// Khronos: [vkDestroyPipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html) (non-nullable)
pub type vkDestroyPipelineCache_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_cache: VkPipelineCache,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyPipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html) (nullable)
pub type PFN_vkDestroyPipelineCache = Option<vkDestroyPipelineCache_t>;
const vkDestroyPipelineCache_NAME: &str = "vkDestroyPipelineCache\0";

/// Khronos: [vkDestroyPipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html) (non-nullable)
pub type vkDestroyPipelineLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_layout: VkPipelineLayout,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyPipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html) (nullable)
pub type PFN_vkDestroyPipelineLayout = Option<vkDestroyPipelineLayout_t>;
const vkDestroyPipelineLayout_NAME: &str = "vkDestroyPipelineLayout\0";

/// Khronos: [vkDestroyPrivateDataSlot](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html) (non-nullable)
pub type vkDestroyPrivateDataSlot_t = unsafe extern "system" fn(
  device: VkDevice,
  private_data_slot: VkPrivateDataSlot,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyPrivateDataSlot](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html) (nullable)
pub type PFN_vkDestroyPrivateDataSlot = Option<vkDestroyPrivateDataSlot_t>;
const vkDestroyPrivateDataSlot_NAME: &str = "vkDestroyPrivateDataSlot\0";

/// Khronos: [vkDestroyQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html) (non-nullable)
pub type vkDestroyQueryPool_t = unsafe extern "system" fn(
  device: VkDevice,
  query_pool: VkQueryPool,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html) (nullable)
pub type PFN_vkDestroyQueryPool = Option<vkDestroyQueryPool_t>;
const vkDestroyQueryPool_NAME: &str = "vkDestroyQueryPool\0";

/// Khronos: [vkDestroyRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html) (non-nullable)
pub type vkDestroyRenderPass_t = unsafe extern "system" fn(
  device: VkDevice,
  render_pass: VkRenderPass,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html) (nullable)
pub type PFN_vkDestroyRenderPass = Option<vkDestroyRenderPass_t>;
const vkDestroyRenderPass_NAME: &str = "vkDestroyRenderPass\0";

/// Khronos: [vkDestroySampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html) (non-nullable)
pub type vkDestroySampler_t = unsafe extern "system" fn(
  device: VkDevice,
  sampler: VkSampler,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroySampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html) (nullable)
pub type PFN_vkDestroySampler = Option<vkDestroySampler_t>;
const vkDestroySampler_NAME: &str = "vkDestroySampler\0";

/// Khronos: [vkDestroySamplerYcbcrConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html) (non-nullable)
pub type vkDestroySamplerYcbcrConversion_t = unsafe extern "system" fn(
  device: VkDevice,
  ycbcr_conversion: VkSamplerYcbcrConversion,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroySamplerYcbcrConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html) (nullable)
pub type PFN_vkDestroySamplerYcbcrConversion = Option<vkDestroySamplerYcbcrConversion_t>;
const vkDestroySamplerYcbcrConversion_NAME: &str = "vkDestroySamplerYcbcrConversion\0";

/// Khronos: [vkDestroySemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html) (non-nullable)
pub type vkDestroySemaphore_t = unsafe extern "system" fn(
  device: VkDevice,
  semaphore: VkSemaphore,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroySemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html) (nullable)
pub type PFN_vkDestroySemaphore = Option<vkDestroySemaphore_t>;
const vkDestroySemaphore_NAME: &str = "vkDestroySemaphore\0";

/// Khronos: [vkDestroySemaphoreSciSyncPoolNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphoreSciSyncPoolNV.html) (non-nullable)
pub type vkDestroySemaphoreSciSyncPoolNV_t = unsafe extern "system" fn(
  device: VkDevice,
  semaphore_pool: VkSemaphoreSciSyncPoolNV,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroySemaphoreSciSyncPoolNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphoreSciSyncPoolNV.html) (nullable)
pub type PFN_vkDestroySemaphoreSciSyncPoolNV = Option<vkDestroySemaphoreSciSyncPoolNV_t>;
const vkDestroySemaphoreSciSyncPoolNV_NAME: &str = "vkDestroySemaphoreSciSyncPoolNV\0";

/// Khronos: [vkDestroyShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html) (non-nullable)
pub type vkDestroyShaderModule_t = unsafe extern "system" fn(
  device: VkDevice,
  shader_module: VkShaderModule,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html) (nullable)
pub type PFN_vkDestroyShaderModule = Option<vkDestroyShaderModule_t>;
const vkDestroyShaderModule_NAME: &str = "vkDestroyShaderModule\0";

/// Khronos: [vkDestroySurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html) (non-nullable)
pub type vkDestroySurfaceKHR_t = unsafe extern "system" fn(
  instance: VkInstance,
  surface: VkSurfaceKHR,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroySurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html) (nullable)
pub type PFN_vkDestroySurfaceKHR = Option<vkDestroySurfaceKHR_t>;
const vkDestroySurfaceKHR_NAME: &str = "vkDestroySurfaceKHR\0";

/// Khronos: [vkDestroySwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html) (non-nullable)
pub type vkDestroySwapchainKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroySwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html) (nullable)
pub type PFN_vkDestroySwapchainKHR = Option<vkDestroySwapchainKHR_t>;
const vkDestroySwapchainKHR_NAME: &str = "vkDestroySwapchainKHR\0";

/// Khronos: [vkDestroyValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html) (non-nullable)
pub type vkDestroyValidationCacheEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  validation_cache: VkValidationCacheEXT,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html) (nullable)
pub type PFN_vkDestroyValidationCacheEXT = Option<vkDestroyValidationCacheEXT_t>;
const vkDestroyValidationCacheEXT_NAME: &str = "vkDestroyValidationCacheEXT\0";

/// Khronos: [vkDestroyVideoSessionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html) (non-nullable)
pub type vkDestroyVideoSessionKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  video_session: VkVideoSessionKHR,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyVideoSessionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html) (nullable)
pub type PFN_vkDestroyVideoSessionKHR = Option<vkDestroyVideoSessionKHR_t>;
const vkDestroyVideoSessionKHR_NAME: &str = "vkDestroyVideoSessionKHR\0";

/// Khronos: [vkDestroyVideoSessionParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionParametersKHR.html) (non-nullable)
pub type vkDestroyVideoSessionParametersKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  video_session_parameters: VkVideoSessionParametersKHR,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkDestroyVideoSessionParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionParametersKHR.html) (nullable)
pub type PFN_vkDestroyVideoSessionParametersKHR =
  Option<vkDestroyVideoSessionParametersKHR_t>;
const vkDestroyVideoSessionParametersKHR_NAME: &str =
  "vkDestroyVideoSessionParametersKHR\0";

/// Khronos: [vkDeviceWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html) (non-nullable)
pub type vkDeviceWaitIdle_t = unsafe extern "system" fn(device: VkDevice) -> VkResult;
/// Khronos: [vkDeviceWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html) (nullable)
pub type PFN_vkDeviceWaitIdle = Option<vkDeviceWaitIdle_t>;
const vkDeviceWaitIdle_NAME: &str = "vkDeviceWaitIdle\0";

/// Khronos: [vkDisplayPowerControlEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html) (non-nullable)
pub type vkDisplayPowerControlEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  display: VkDisplayKHR,
  display_power_info: *const VkDisplayPowerInfoEXT,
) -> VkResult;
/// Khronos: [vkDisplayPowerControlEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html) (nullable)
pub type PFN_vkDisplayPowerControlEXT = Option<vkDisplayPowerControlEXT_t>;
const vkDisplayPowerControlEXT_NAME: &str = "vkDisplayPowerControlEXT\0";

/// Khronos: [vkEndCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html) (non-nullable)
pub type vkEndCommandBuffer_t =
  unsafe extern "system" fn(command_buffer: VkCommandBuffer) -> VkResult;
/// Khronos: [vkEndCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html) (nullable)
pub type PFN_vkEndCommandBuffer = Option<vkEndCommandBuffer_t>;
const vkEndCommandBuffer_NAME: &str = "vkEndCommandBuffer\0";

/// Khronos: [vkEnumerateDeviceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html) (non-nullable)
pub type vkEnumerateDeviceExtensionProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  layer_name: *const u8,
  property_count: *mut u32,
  properties: *mut VkExtensionProperties,
) -> VkResult;
/// Khronos: [vkEnumerateDeviceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html) (nullable)
pub type PFN_vkEnumerateDeviceExtensionProperties =
  Option<vkEnumerateDeviceExtensionProperties_t>;
const vkEnumerateDeviceExtensionProperties_NAME: &str =
  "vkEnumerateDeviceExtensionProperties\0";

/// Khronos: [vkEnumerateDeviceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html) (non-nullable)
pub type vkEnumerateDeviceLayerProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  property_count: *mut u32,
  properties: *mut VkLayerProperties,
) -> VkResult;
/// Khronos: [vkEnumerateDeviceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html) (nullable)
pub type PFN_vkEnumerateDeviceLayerProperties =
  Option<vkEnumerateDeviceLayerProperties_t>;
const vkEnumerateDeviceLayerProperties_NAME: &str = "vkEnumerateDeviceLayerProperties\0";

/// Khronos: [vkEnumerateInstanceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html) (non-nullable)
pub type vkEnumerateInstanceExtensionProperties_t = unsafe extern "system" fn(
  layer_name: *const u8,
  property_count: *mut u32,
  properties: *mut VkExtensionProperties,
)
  -> VkResult;
/// Khronos: [vkEnumerateInstanceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html) (nullable)
pub type PFN_vkEnumerateInstanceExtensionProperties =
  Option<vkEnumerateInstanceExtensionProperties_t>;
const vkEnumerateInstanceExtensionProperties_NAME: &str =
  "vkEnumerateInstanceExtensionProperties\0";

/// Khronos: [vkEnumerateInstanceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html) (non-nullable)
pub type vkEnumerateInstanceLayerProperties_t = unsafe extern "system" fn(
  property_count: *mut u32,
  properties: *mut VkLayerProperties,
) -> VkResult;
/// Khronos: [vkEnumerateInstanceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html) (nullable)
pub type PFN_vkEnumerateInstanceLayerProperties =
  Option<vkEnumerateInstanceLayerProperties_t>;
const vkEnumerateInstanceLayerProperties_NAME: &str =
  "vkEnumerateInstanceLayerProperties\0";

/// Khronos: [vkEnumerateInstanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html) (non-nullable)
pub type vkEnumerateInstanceVersion_t =
  unsafe extern "system" fn(api_version: *mut u32) -> VkResult;
/// Khronos: [vkEnumerateInstanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html) (nullable)
pub type PFN_vkEnumerateInstanceVersion = Option<vkEnumerateInstanceVersion_t>;
const vkEnumerateInstanceVersion_NAME: &str = "vkEnumerateInstanceVersion\0";

/// Khronos: [vkEnumeratePhysicalDeviceGroups](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html) (non-nullable)
pub type vkEnumeratePhysicalDeviceGroups_t = unsafe extern "system" fn(
  instance: VkInstance,
  physical_device_group_count: *mut u32,
  physical_device_group_properties: *mut VkPhysicalDeviceGroupProperties,
) -> VkResult;
/// Khronos: [vkEnumeratePhysicalDeviceGroups](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html) (nullable)
pub type PFN_vkEnumeratePhysicalDeviceGroups = Option<vkEnumeratePhysicalDeviceGroups_t>;
const vkEnumeratePhysicalDeviceGroups_NAME: &str = "vkEnumeratePhysicalDeviceGroups\0";

/// Khronos: [vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html) (non-nullable)
pub type vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    counter_count: *mut u32,
    counters: *mut VkPerformanceCounterKHR,
    counter_descriptions: *mut VkPerformanceCounterDescriptionKHR,
  ) -> VkResult;
/// Khronos: [vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html) (nullable)
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
  Option<vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR_t>;
const vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR_NAME: &str =
  "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR\0";

/// Khronos: [vkEnumeratePhysicalDevices](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html) (non-nullable)
pub type vkEnumeratePhysicalDevices_t = unsafe extern "system" fn(
  instance: VkInstance,
  physical_device_count: *mut u32,
  physical_devices: *mut VkPhysicalDevice,
) -> VkResult;
/// Khronos: [vkEnumeratePhysicalDevices](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html) (nullable)
pub type PFN_vkEnumeratePhysicalDevices = Option<vkEnumeratePhysicalDevices_t>;
const vkEnumeratePhysicalDevices_NAME: &str = "vkEnumeratePhysicalDevices\0";

/// Khronos: [vkExportMetalObjectsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkExportMetalObjectsEXT.html) (non-nullable)
pub type vkExportMetalObjectsEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  metal_objects_info: *mut VkExportMetalObjectsInfoEXT,
);
/// Khronos: [vkExportMetalObjectsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkExportMetalObjectsEXT.html) (nullable)
pub type PFN_vkExportMetalObjectsEXT = Option<vkExportMetalObjectsEXT_t>;
const vkExportMetalObjectsEXT_NAME: &str = "vkExportMetalObjectsEXT\0";

/// Khronos: [vkFlushMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html) (non-nullable)
pub type vkFlushMappedMemoryRanges_t = unsafe extern "system" fn(
  device: VkDevice,
  memory_range_count: u32,
  memory_ranges: *const VkMappedMemoryRange,
) -> VkResult;
/// Khronos: [vkFlushMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html) (nullable)
pub type PFN_vkFlushMappedMemoryRanges = Option<vkFlushMappedMemoryRanges_t>;
const vkFlushMappedMemoryRanges_NAME: &str = "vkFlushMappedMemoryRanges\0";

/// Khronos: [vkFreeCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html) (non-nullable)
pub type vkFreeCommandBuffers_t = unsafe extern "system" fn(
  device: VkDevice,
  command_pool: VkCommandPool,
  command_buffer_count: u32,
  command_buffers: *const VkCommandBuffer,
);
/// Khronos: [vkFreeCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html) (nullable)
pub type PFN_vkFreeCommandBuffers = Option<vkFreeCommandBuffers_t>;
const vkFreeCommandBuffers_NAME: &str = "vkFreeCommandBuffers\0";

/// Khronos: [vkFreeDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html) (non-nullable)
pub type vkFreeDescriptorSets_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_pool: VkDescriptorPool,
  descriptor_set_count: u32,
  descriptor_sets: *const VkDescriptorSet,
) -> VkResult;
/// Khronos: [vkFreeDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html) (nullable)
pub type PFN_vkFreeDescriptorSets = Option<vkFreeDescriptorSets_t>;
const vkFreeDescriptorSets_NAME: &str = "vkFreeDescriptorSets\0";

/// Khronos: [vkFreeMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html) (non-nullable)
pub type vkFreeMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
  allocator: *const VkAllocationCallbacks,
);
/// Khronos: [vkFreeMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html) (nullable)
pub type PFN_vkFreeMemory = Option<vkFreeMemory_t>;
const vkFreeMemory_NAME: &str = "vkFreeMemory\0";

/// Khronos: [vkGetAccelerationStructureDeviceAddressKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html) (non-nullable)
pub type vkGetAccelerationStructureDeviceAddressKHR_t =
  unsafe extern "system" fn(
    device: VkDevice,
    info: *const VkAccelerationStructureDeviceAddressInfoKHR,
  ) -> VkDeviceAddress;
/// Khronos: [vkGetAccelerationStructureDeviceAddressKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html) (nullable)
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR =
  Option<vkGetAccelerationStructureDeviceAddressKHR_t>;
const vkGetAccelerationStructureDeviceAddressKHR_NAME: &str =
  "vkGetAccelerationStructureDeviceAddressKHR\0";

/// Khronos: [vkGetAccelerationStructureHandleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html) (non-nullable)
pub type vkGetAccelerationStructureHandleNV_t = unsafe extern "system" fn(
  device: VkDevice,
  acceleration_structure: VkAccelerationStructureNV,
  data_size: c_size_t,
  data: *mut c_void,
) -> VkResult;
/// Khronos: [vkGetAccelerationStructureHandleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html) (nullable)
pub type PFN_vkGetAccelerationStructureHandleNV =
  Option<vkGetAccelerationStructureHandleNV_t>;
const vkGetAccelerationStructureHandleNV_NAME: &str =
  "vkGetAccelerationStructureHandleNV\0";

/// Khronos: [vkGetAccelerationStructureMemoryRequirementsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) (non-nullable)
pub type vkGetAccelerationStructureMemoryRequirementsNV_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkAccelerationStructureMemoryRequirementsInfoNV,
  memory_requirements: *mut VkMemoryRequirements2KHR,
);
/// Khronos: [vkGetAccelerationStructureMemoryRequirementsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) (nullable)
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV =
  Option<vkGetAccelerationStructureMemoryRequirementsNV_t>;
const vkGetAccelerationStructureMemoryRequirementsNV_NAME: &str =
  "vkGetAccelerationStructureMemoryRequirementsNV\0";

/// Khronos: [vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html) (non-nullable)
pub type vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT_t =
  unsafe extern "system" fn(
    device: VkDevice,
    info: *const VkAccelerationStructureCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
  ) -> VkResult;
/// Khronos: [vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html) (nullable)
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
  Option<vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT_t>;
const vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT_NAME: &str =
  "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT\0";

/// Khronos: [vkGetAndroidHardwareBufferPropertiesANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) (non-nullable)
pub type vkGetAndroidHardwareBufferPropertiesANDROID_t =
  unsafe extern "system" fn(
    device: VkDevice,
    buffer: *const AHardwareBuffer,
    properties: *mut VkAndroidHardwareBufferPropertiesANDROID,
  ) -> VkResult;
/// Khronos: [vkGetAndroidHardwareBufferPropertiesANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html) (nullable)
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID =
  Option<vkGetAndroidHardwareBufferPropertiesANDROID_t>;
const vkGetAndroidHardwareBufferPropertiesANDROID_NAME: &str =
  "vkGetAndroidHardwareBufferPropertiesANDROID\0";

/// Khronos: [vkGetBufferCollectionPropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html) (non-nullable)
pub type vkGetBufferCollectionPropertiesFUCHSIA_t = unsafe extern "system" fn(
  device: VkDevice,
  collection: VkBufferCollectionFUCHSIA,
  properties: *mut VkBufferCollectionPropertiesFUCHSIA,
)
  -> VkResult;
/// Khronos: [vkGetBufferCollectionPropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html) (nullable)
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA =
  Option<vkGetBufferCollectionPropertiesFUCHSIA_t>;
const vkGetBufferCollectionPropertiesFUCHSIA_NAME: &str =
  "vkGetBufferCollectionPropertiesFUCHSIA\0";

/// Khronos: [vkGetBufferDeviceAddress](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html) (non-nullable)
pub type vkGetBufferDeviceAddress_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress;
/// Khronos: [vkGetBufferDeviceAddress](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html) (nullable)
pub type PFN_vkGetBufferDeviceAddress = Option<vkGetBufferDeviceAddress_t>;
const vkGetBufferDeviceAddress_NAME: &str = "vkGetBufferDeviceAddress\0";

/// Khronos: [vkGetBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html) (non-nullable)
pub type vkGetBufferMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  buffer: VkBuffer,
  memory_requirements: *mut VkMemoryRequirements,
);
/// Khronos: [vkGetBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html) (nullable)
pub type PFN_vkGetBufferMemoryRequirements = Option<vkGetBufferMemoryRequirements_t>;
const vkGetBufferMemoryRequirements_NAME: &str = "vkGetBufferMemoryRequirements\0";

/// Khronos: [vkGetBufferMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html) (non-nullable)
pub type vkGetBufferMemoryRequirements2_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkBufferMemoryRequirementsInfo2,
  memory_requirements: *mut VkMemoryRequirements2,
);
/// Khronos: [vkGetBufferMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html) (nullable)
pub type PFN_vkGetBufferMemoryRequirements2 = Option<vkGetBufferMemoryRequirements2_t>;
const vkGetBufferMemoryRequirements2_NAME: &str = "vkGetBufferMemoryRequirements2\0";

/// Khronos: [vkGetBufferOpaqueCaptureAddress](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html) (non-nullable)
pub type vkGetBufferOpaqueCaptureAddress_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkBufferDeviceAddressInfo,
) -> u64;
/// Khronos: [vkGetBufferOpaqueCaptureAddress](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html) (nullable)
pub type PFN_vkGetBufferOpaqueCaptureAddress = Option<vkGetBufferOpaqueCaptureAddress_t>;
const vkGetBufferOpaqueCaptureAddress_NAME: &str = "vkGetBufferOpaqueCaptureAddress\0";

/// Khronos: [vkGetBufferOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureDescriptorDataEXT.html) (non-nullable)
pub type vkGetBufferOpaqueCaptureDescriptorDataEXT_t =
  unsafe extern "system" fn(
    device: VkDevice,
    info: *const VkBufferCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
  ) -> VkResult;
/// Khronos: [vkGetBufferOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureDescriptorDataEXT.html) (nullable)
pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT =
  Option<vkGetBufferOpaqueCaptureDescriptorDataEXT_t>;
const vkGetBufferOpaqueCaptureDescriptorDataEXT_NAME: &str =
  "vkGetBufferOpaqueCaptureDescriptorDataEXT\0";

/// Khronos: [vkGetCalibratedTimestampsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html) (non-nullable)
pub type vkGetCalibratedTimestampsEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  timestamp_count: u32,
  timestamp_infos: *const VkCalibratedTimestampInfoEXT,
  timestamps: *mut u64,
  max_deviation: *mut u64,
) -> VkResult;
/// Khronos: [vkGetCalibratedTimestampsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html) (nullable)
pub type PFN_vkGetCalibratedTimestampsEXT = Option<vkGetCalibratedTimestampsEXT_t>;
const vkGetCalibratedTimestampsEXT_NAME: &str = "vkGetCalibratedTimestampsEXT\0";

/// Khronos: [vkGetDeferredOperationMaxConcurrencyKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) (non-nullable)
pub type vkGetDeferredOperationMaxConcurrencyKHR_t =
  unsafe extern "system" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> u32;
/// Khronos: [vkGetDeferredOperationMaxConcurrencyKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) (nullable)
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
  Option<vkGetDeferredOperationMaxConcurrencyKHR_t>;
const vkGetDeferredOperationMaxConcurrencyKHR_NAME: &str =
  "vkGetDeferredOperationMaxConcurrencyKHR\0";

/// Khronos: [vkGetDeferredOperationResultKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html) (non-nullable)
pub type vkGetDeferredOperationResultKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  operation: VkDeferredOperationKHR,
) -> VkResult;
/// Khronos: [vkGetDeferredOperationResultKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html) (nullable)
pub type PFN_vkGetDeferredOperationResultKHR = Option<vkGetDeferredOperationResultKHR_t>;
const vkGetDeferredOperationResultKHR_NAME: &str = "vkGetDeferredOperationResultKHR\0";

/// Khronos: [vkGetDescriptorSetHostMappingVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetHostMappingVALVE.html) (non-nullable)
pub type vkGetDescriptorSetHostMappingVALVE_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_set: VkDescriptorSet,
  data: *mut *mut c_void,
);
/// Khronos: [vkGetDescriptorSetHostMappingVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetHostMappingVALVE.html) (nullable)
pub type PFN_vkGetDescriptorSetHostMappingVALVE =
  Option<vkGetDescriptorSetHostMappingVALVE_t>;
const vkGetDescriptorSetHostMappingVALVE_NAME: &str =
  "vkGetDescriptorSetHostMappingVALVE\0";

/// Khronos: [vkGetDescriptorSetLayoutBindingOffsetEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html) (non-nullable)
pub type vkGetDescriptorSetLayoutBindingOffsetEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  layout: VkDescriptorSetLayout,
  binding: u32,
  offset: *mut VkDeviceSize,
);
/// Khronos: [vkGetDescriptorSetLayoutBindingOffsetEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html) (nullable)
pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT =
  Option<vkGetDescriptorSetLayoutBindingOffsetEXT_t>;
const vkGetDescriptorSetLayoutBindingOffsetEXT_NAME: &str =
  "vkGetDescriptorSetLayoutBindingOffsetEXT\0";

/// Khronos: [vkGetDescriptorSetLayoutHostMappingInfoVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html) (non-nullable)
pub type vkGetDescriptorSetLayoutHostMappingInfoVALVE_t = unsafe extern "system" fn(
  device: VkDevice,
  binding_reference: *const VkDescriptorSetBindingReferenceVALVE,
  host_mapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE,
);
/// Khronos: [vkGetDescriptorSetLayoutHostMappingInfoVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html) (nullable)
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE =
  Option<vkGetDescriptorSetLayoutHostMappingInfoVALVE_t>;
const vkGetDescriptorSetLayoutHostMappingInfoVALVE_NAME: &str =
  "vkGetDescriptorSetLayoutHostMappingInfoVALVE\0";

/// Khronos: [vkGetDescriptorSetLayoutSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSizeEXT.html) (non-nullable)
pub type vkGetDescriptorSetLayoutSizeEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  layout: VkDescriptorSetLayout,
  layout_size_in_bytes: *mut VkDeviceSize,
);
/// Khronos: [vkGetDescriptorSetLayoutSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSizeEXT.html) (nullable)
pub type PFN_vkGetDescriptorSetLayoutSizeEXT = Option<vkGetDescriptorSetLayoutSizeEXT_t>;
const vkGetDescriptorSetLayoutSizeEXT_NAME: &str = "vkGetDescriptorSetLayoutSizeEXT\0";

/// Khronos: [vkGetDescriptorSetLayoutSupport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html) (non-nullable)
pub type vkGetDescriptorSetLayoutSupport_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkDescriptorSetLayoutCreateInfo,
  support: *mut VkDescriptorSetLayoutSupport,
);
/// Khronos: [vkGetDescriptorSetLayoutSupport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html) (nullable)
pub type PFN_vkGetDescriptorSetLayoutSupport = Option<vkGetDescriptorSetLayoutSupport_t>;
const vkGetDescriptorSetLayoutSupport_NAME: &str = "vkGetDescriptorSetLayoutSupport\0";

/// Khronos: [vkGetDeviceAccelerationStructureCompatibilityKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html) (non-nullable)
pub type vkGetDeviceAccelerationStructureCompatibilityKHR_t =
  unsafe extern "system" fn(
    device: VkDevice,
    version_info: *const VkAccelerationStructureVersionInfoKHR,
    compatibility: *mut VkAccelerationStructureCompatibilityKHR,
  );
/// Khronos: [vkGetDeviceAccelerationStructureCompatibilityKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html) (nullable)
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR =
  Option<vkGetDeviceAccelerationStructureCompatibilityKHR_t>;
const vkGetDeviceAccelerationStructureCompatibilityKHR_NAME: &str =
  "vkGetDeviceAccelerationStructureCompatibilityKHR\0";

/// Khronos: [vkGetDeviceBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html) (non-nullable)
pub type vkGetDeviceBufferMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkDeviceBufferMemoryRequirements,
  memory_requirements: *mut VkMemoryRequirements2,
);
/// Khronos: [vkGetDeviceBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html) (nullable)
pub type PFN_vkGetDeviceBufferMemoryRequirements =
  Option<vkGetDeviceBufferMemoryRequirements_t>;
const vkGetDeviceBufferMemoryRequirements_NAME: &str =
  "vkGetDeviceBufferMemoryRequirements\0";

/// Khronos: [vkGetDeviceFaultInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceFaultInfoEXT.html) (non-nullable)
pub type vkGetDeviceFaultInfoEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  fault_counts: *mut VkDeviceFaultCountsEXT,
  fault_info: *mut VkDeviceFaultInfoEXT,
) -> VkResult;
/// Khronos: [vkGetDeviceFaultInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceFaultInfoEXT.html) (nullable)
pub type PFN_vkGetDeviceFaultInfoEXT = Option<vkGetDeviceFaultInfoEXT_t>;
const vkGetDeviceFaultInfoEXT_NAME: &str = "vkGetDeviceFaultInfoEXT\0";

/// Khronos: [vkGetDeviceGroupPeerMemoryFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html) (non-nullable)
pub type vkGetDeviceGroupPeerMemoryFeatures_t = unsafe extern "system" fn(
  device: VkDevice,
  heap_index: u32,
  local_device_index: u32,
  remote_device_index: u32,
  peer_memory_features: *mut VkPeerMemoryFeatureFlags,
);
/// Khronos: [vkGetDeviceGroupPeerMemoryFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html) (nullable)
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures =
  Option<vkGetDeviceGroupPeerMemoryFeatures_t>;
const vkGetDeviceGroupPeerMemoryFeatures_NAME: &str =
  "vkGetDeviceGroupPeerMemoryFeatures\0";

/// Khronos: [vkGetDeviceGroupPresentCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) (non-nullable)
pub type vkGetDeviceGroupPresentCapabilitiesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  device_group_present_capabilities: *mut VkDeviceGroupPresentCapabilitiesKHR,
)
  -> VkResult;
/// Khronos: [vkGetDeviceGroupPresentCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) (nullable)
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR =
  Option<vkGetDeviceGroupPresentCapabilitiesKHR_t>;
const vkGetDeviceGroupPresentCapabilitiesKHR_NAME: &str =
  "vkGetDeviceGroupPresentCapabilitiesKHR\0";

/// Khronos: [vkGetDeviceGroupSurfacePresentModes2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) (non-nullable)
pub type vkGetDeviceGroupSurfacePresentModes2EXT_t =
  unsafe extern "system" fn(
    device: VkDevice,
    surface_info: *const VkPhysicalDeviceSurfaceInfo2KHR,
    modes: *mut VkDeviceGroupPresentModeFlagsKHR,
  ) -> VkResult;
/// Khronos: [vkGetDeviceGroupSurfacePresentModes2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html) (nullable)
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT =
  Option<vkGetDeviceGroupSurfacePresentModes2EXT_t>;
const vkGetDeviceGroupSurfacePresentModes2EXT_NAME: &str =
  "vkGetDeviceGroupSurfacePresentModes2EXT\0";

/// Khronos: [vkGetDeviceGroupSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) (non-nullable)
pub type vkGetDeviceGroupSurfacePresentModesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  surface: VkSurfaceKHR,
  modes: *mut VkDeviceGroupPresentModeFlagsKHR,
)
  -> VkResult;
/// Khronos: [vkGetDeviceGroupSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) (nullable)
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR =
  Option<vkGetDeviceGroupSurfacePresentModesKHR_t>;
const vkGetDeviceGroupSurfacePresentModesKHR_NAME: &str =
  "vkGetDeviceGroupSurfacePresentModesKHR\0";

/// Khronos: [vkGetDeviceImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html) (non-nullable)
pub type vkGetDeviceImageMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkDeviceImageMemoryRequirements,
  memory_requirements: *mut VkMemoryRequirements2,
);
/// Khronos: [vkGetDeviceImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html) (nullable)
pub type PFN_vkGetDeviceImageMemoryRequirements =
  Option<vkGetDeviceImageMemoryRequirements_t>;
const vkGetDeviceImageMemoryRequirements_NAME: &str =
  "vkGetDeviceImageMemoryRequirements\0";

/// Khronos: [vkGetDeviceImageSparseMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html) (non-nullable)
pub type vkGetDeviceImageSparseMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkDeviceImageMemoryRequirements,
  sparse_memory_requirement_count: *mut u32,
  sparse_memory_requirements: *mut VkSparseImageMemoryRequirements2,
);
/// Khronos: [vkGetDeviceImageSparseMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html) (nullable)
pub type PFN_vkGetDeviceImageSparseMemoryRequirements =
  Option<vkGetDeviceImageSparseMemoryRequirements_t>;
const vkGetDeviceImageSparseMemoryRequirements_NAME: &str =
  "vkGetDeviceImageSparseMemoryRequirements\0";

/// Khronos: [vkGetDeviceMemoryCommitment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html) (non-nullable)
pub type vkGetDeviceMemoryCommitment_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
  committed_memory_in_bytes: *mut VkDeviceSize,
);
/// Khronos: [vkGetDeviceMemoryCommitment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html) (nullable)
pub type PFN_vkGetDeviceMemoryCommitment = Option<vkGetDeviceMemoryCommitment_t>;
const vkGetDeviceMemoryCommitment_NAME: &str = "vkGetDeviceMemoryCommitment\0";

/// Khronos: [vkGetDeviceMemoryOpaqueCaptureAddress](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html) (non-nullable)
pub type vkGetDeviceMemoryOpaqueCaptureAddress_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkDeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;
/// Khronos: [vkGetDeviceMemoryOpaqueCaptureAddress](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html) (nullable)
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress =
  Option<vkGetDeviceMemoryOpaqueCaptureAddress_t>;
const vkGetDeviceMemoryOpaqueCaptureAddress_NAME: &str =
  "vkGetDeviceMemoryOpaqueCaptureAddress\0";

/// Khronos: [vkGetDeviceMicromapCompatibilityEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMicromapCompatibilityEXT.html) (non-nullable)
pub type vkGetDeviceMicromapCompatibilityEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  version_info: *const VkMicromapVersionInfoEXT,
  compatibility: *mut VkAccelerationStructureCompatibilityKHR,
);
/// Khronos: [vkGetDeviceMicromapCompatibilityEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMicromapCompatibilityEXT.html) (nullable)
pub type PFN_vkGetDeviceMicromapCompatibilityEXT =
  Option<vkGetDeviceMicromapCompatibilityEXT_t>;
const vkGetDeviceMicromapCompatibilityEXT_NAME: &str =
  "vkGetDeviceMicromapCompatibilityEXT\0";

/// Khronos: [vkGetDeviceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html) (non-nullable)
pub type vkGetDeviceProcAddr_t =
  unsafe extern "system" fn(device: VkDevice, name: *const u8) -> PFN_vkVoidFunction;
/// Khronos: [vkGetDeviceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html) (nullable)
pub type PFN_vkGetDeviceProcAddr = Option<vkGetDeviceProcAddr_t>;
const vkGetDeviceProcAddr_NAME: &str = "vkGetDeviceProcAddr\0";

/// Khronos: [vkGetDeviceQueue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html) (non-nullable)
pub type vkGetDeviceQueue_t = unsafe extern "system" fn(
  device: VkDevice,
  queue_family_index: u32,
  queue_index: u32,
  queue: *mut VkQueue,
);
/// Khronos: [vkGetDeviceQueue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html) (nullable)
pub type PFN_vkGetDeviceQueue = Option<vkGetDeviceQueue_t>;
const vkGetDeviceQueue_NAME: &str = "vkGetDeviceQueue\0";

/// Khronos: [vkGetDeviceQueue2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html) (non-nullable)
pub type vkGetDeviceQueue2_t = unsafe extern "system" fn(
  device: VkDevice,
  queue_info: *const VkDeviceQueueInfo2,
  queue: *mut VkQueue,
);
/// Khronos: [vkGetDeviceQueue2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html) (nullable)
pub type PFN_vkGetDeviceQueue2 = Option<vkGetDeviceQueue2_t>;
const vkGetDeviceQueue2_NAME: &str = "vkGetDeviceQueue2\0";

/// Khronos: [vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html) (non-nullable)
pub type vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI_t =
  unsafe extern "system" fn(
    device: VkDevice,
    renderpass: VkRenderPass,
    max_workgroup_size: *mut VkExtent2D,
  ) -> VkResult;
/// Khronos: [vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html) (nullable)
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI =
  Option<vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI_t>;
const vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI_NAME: &str =
  "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI\0";

/// Khronos: [vkGetDisplayModeProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html) (non-nullable)
pub type vkGetDisplayModeProperties2KHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  display: VkDisplayKHR,
  property_count: *mut u32,
  properties: *mut VkDisplayModeProperties2KHR,
) -> VkResult;
/// Khronos: [vkGetDisplayModeProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html) (nullable)
pub type PFN_vkGetDisplayModeProperties2KHR = Option<vkGetDisplayModeProperties2KHR_t>;
const vkGetDisplayModeProperties2KHR_NAME: &str = "vkGetDisplayModeProperties2KHR\0";

/// Khronos: [vkGetDisplayModePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html) (non-nullable)
pub type vkGetDisplayModePropertiesKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  display: VkDisplayKHR,
  property_count: *mut u32,
  properties: *mut VkDisplayModePropertiesKHR,
) -> VkResult;
/// Khronos: [vkGetDisplayModePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html) (nullable)
pub type PFN_vkGetDisplayModePropertiesKHR = Option<vkGetDisplayModePropertiesKHR_t>;
const vkGetDisplayModePropertiesKHR_NAME: &str = "vkGetDisplayModePropertiesKHR\0";

/// Khronos: [vkGetDisplayPlaneCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) (non-nullable)
pub type vkGetDisplayPlaneCapabilities2KHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  display_plane_info: *const VkDisplayPlaneInfo2KHR,
  capabilities: *mut VkDisplayPlaneCapabilities2KHR,
) -> VkResult;
/// Khronos: [vkGetDisplayPlaneCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html) (nullable)
pub type PFN_vkGetDisplayPlaneCapabilities2KHR =
  Option<vkGetDisplayPlaneCapabilities2KHR_t>;
const vkGetDisplayPlaneCapabilities2KHR_NAME: &str =
  "vkGetDisplayPlaneCapabilities2KHR\0";

/// Khronos: [vkGetDisplayPlaneCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html) (non-nullable)
pub type vkGetDisplayPlaneCapabilitiesKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  mode: VkDisplayModeKHR,
  plane_index: u32,
  capabilities: *mut VkDisplayPlaneCapabilitiesKHR,
) -> VkResult;
/// Khronos: [vkGetDisplayPlaneCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html) (nullable)
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR =
  Option<vkGetDisplayPlaneCapabilitiesKHR_t>;
const vkGetDisplayPlaneCapabilitiesKHR_NAME: &str = "vkGetDisplayPlaneCapabilitiesKHR\0";

/// Khronos: [vkGetDisplayPlaneSupportedDisplaysKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html) (non-nullable)
pub type vkGetDisplayPlaneSupportedDisplaysKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  plane_index: u32,
  display_count: *mut u32,
  displays: *mut VkDisplayKHR,
) -> VkResult;
/// Khronos: [vkGetDisplayPlaneSupportedDisplaysKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html) (nullable)
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR =
  Option<vkGetDisplayPlaneSupportedDisplaysKHR_t>;
const vkGetDisplayPlaneSupportedDisplaysKHR_NAME: &str =
  "vkGetDisplayPlaneSupportedDisplaysKHR\0";

/// Khronos: [vkGetDrmDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html) (non-nullable)
pub type vkGetDrmDisplayEXT_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  drm_fd: i32,
  connector_id: u32,
  display: *mut VkDisplayKHR,
) -> VkResult;
/// Khronos: [vkGetDrmDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html) (nullable)
pub type PFN_vkGetDrmDisplayEXT = Option<vkGetDrmDisplayEXT_t>;
const vkGetDrmDisplayEXT_NAME: &str = "vkGetDrmDisplayEXT\0";

/// Khronos: [vkGetDynamicRenderingTilePropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html) (non-nullable)
pub type vkGetDynamicRenderingTilePropertiesQCOM_t =
  unsafe extern "system" fn(
    device: VkDevice,
    rendering_info: *const VkRenderingInfo,
    properties: *mut VkTilePropertiesQCOM,
  ) -> VkResult;
/// Khronos: [vkGetDynamicRenderingTilePropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html) (nullable)
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM =
  Option<vkGetDynamicRenderingTilePropertiesQCOM_t>;
const vkGetDynamicRenderingTilePropertiesQCOM_NAME: &str =
  "vkGetDynamicRenderingTilePropertiesQCOM\0";

/// Khronos: [vkGetEventStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html) (non-nullable)
pub type vkGetEventStatus_t =
  unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
/// Khronos: [vkGetEventStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html) (nullable)
pub type PFN_vkGetEventStatus = Option<vkGetEventStatus_t>;
const vkGetEventStatus_NAME: &str = "vkGetEventStatus\0";

/// Khronos: [vkGetFenceFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html) (non-nullable)
pub type vkGetFenceFdKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  get_fd_info: *const VkFenceGetFdInfoKHR,
  fd: *mut c_int,
) -> VkResult;
/// Khronos: [vkGetFenceFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html) (nullable)
pub type PFN_vkGetFenceFdKHR = Option<vkGetFenceFdKHR_t>;
const vkGetFenceFdKHR_NAME: &str = "vkGetFenceFdKHR\0";

/// Khronos: [vkGetFenceSciSyncFenceNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceSciSyncFenceNV.html) (non-nullable)
pub type vkGetFenceSciSyncFenceNV_t = unsafe extern "system" fn(
  device: VkDevice,
  get_sci_sync_handle_info: *const VkFenceGetSciSyncInfoNV,
  handle: *mut c_void,
) -> VkResult;
/// Khronos: [vkGetFenceSciSyncFenceNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceSciSyncFenceNV.html) (nullable)
pub type PFN_vkGetFenceSciSyncFenceNV = Option<vkGetFenceSciSyncFenceNV_t>;
const vkGetFenceSciSyncFenceNV_NAME: &str = "vkGetFenceSciSyncFenceNV\0";

/// Khronos: [vkGetFenceSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceSciSyncObjNV.html) (non-nullable)
pub type vkGetFenceSciSyncObjNV_t = unsafe extern "system" fn(
  device: VkDevice,
  get_sci_sync_handle_info: *const VkFenceGetSciSyncInfoNV,
  handle: *mut c_void,
) -> VkResult;
/// Khronos: [vkGetFenceSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceSciSyncObjNV.html) (nullable)
pub type PFN_vkGetFenceSciSyncObjNV = Option<vkGetFenceSciSyncObjNV_t>;
const vkGetFenceSciSyncObjNV_NAME: &str = "vkGetFenceSciSyncObjNV\0";

/// Khronos: [vkGetFenceStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html) (non-nullable)
pub type vkGetFenceStatus_t =
  unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult;
/// Khronos: [vkGetFenceStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html) (nullable)
pub type PFN_vkGetFenceStatus = Option<vkGetFenceStatus_t>;
const vkGetFenceStatus_NAME: &str = "vkGetFenceStatus\0";

/// Khronos: [vkGetFenceWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html) (non-nullable)
pub type vkGetFenceWin32HandleKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  get_win_32_handle_info: *const VkFenceGetWin32HandleInfoKHR,
  handle: *mut HANDLE,
) -> VkResult;
/// Khronos: [vkGetFenceWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html) (nullable)
pub type PFN_vkGetFenceWin32HandleKHR = Option<vkGetFenceWin32HandleKHR_t>;
const vkGetFenceWin32HandleKHR_NAME: &str = "vkGetFenceWin32HandleKHR\0";

/// Khronos: [vkGetFramebufferTilePropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFramebufferTilePropertiesQCOM.html) (non-nullable)
pub type vkGetFramebufferTilePropertiesQCOM_t = unsafe extern "system" fn(
  device: VkDevice,
  framebuffer: VkFramebuffer,
  properties_count: *mut u32,
  properties: *mut VkTilePropertiesQCOM,
) -> VkResult;
/// Khronos: [vkGetFramebufferTilePropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFramebufferTilePropertiesQCOM.html) (nullable)
pub type PFN_vkGetFramebufferTilePropertiesQCOM =
  Option<vkGetFramebufferTilePropertiesQCOM_t>;
const vkGetFramebufferTilePropertiesQCOM_NAME: &str =
  "vkGetFramebufferTilePropertiesQCOM\0";

/// Khronos: [vkGetGeneratedCommandsMemoryRequirementsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html) (non-nullable)
pub type vkGetGeneratedCommandsMemoryRequirementsNV_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkGeneratedCommandsMemoryRequirementsInfoNV,
  memory_requirements: *mut VkMemoryRequirements2,
);
/// Khronos: [vkGetGeneratedCommandsMemoryRequirementsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html) (nullable)
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV =
  Option<vkGetGeneratedCommandsMemoryRequirementsNV_t>;
const vkGetGeneratedCommandsMemoryRequirementsNV_NAME: &str =
  "vkGetGeneratedCommandsMemoryRequirementsNV\0";

/// Khronos: [vkGetImageDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html) (non-nullable)
pub type vkGetImageDrmFormatModifierPropertiesEXT_t =
  unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    properties: *mut VkImageDrmFormatModifierPropertiesEXT,
  ) -> VkResult;
/// Khronos: [vkGetImageDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html) (nullable)
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT =
  Option<vkGetImageDrmFormatModifierPropertiesEXT_t>;
const vkGetImageDrmFormatModifierPropertiesEXT_NAME: &str =
  "vkGetImageDrmFormatModifierPropertiesEXT\0";

/// Khronos: [vkGetImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html) (non-nullable)
pub type vkGetImageMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  memory_requirements: *mut VkMemoryRequirements,
);
/// Khronos: [vkGetImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html) (nullable)
pub type PFN_vkGetImageMemoryRequirements = Option<vkGetImageMemoryRequirements_t>;
const vkGetImageMemoryRequirements_NAME: &str = "vkGetImageMemoryRequirements\0";

/// Khronos: [vkGetImageMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html) (non-nullable)
pub type vkGetImageMemoryRequirements2_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkImageMemoryRequirementsInfo2,
  memory_requirements: *mut VkMemoryRequirements2,
);
/// Khronos: [vkGetImageMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html) (nullable)
pub type PFN_vkGetImageMemoryRequirements2 = Option<vkGetImageMemoryRequirements2_t>;
const vkGetImageMemoryRequirements2_NAME: &str = "vkGetImageMemoryRequirements2\0";

/// Khronos: [vkGetImageOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageOpaqueCaptureDescriptorDataEXT.html) (non-nullable)
pub type vkGetImageOpaqueCaptureDescriptorDataEXT_t =
  unsafe extern "system" fn(
    device: VkDevice,
    info: *const VkImageCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
  ) -> VkResult;
/// Khronos: [vkGetImageOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageOpaqueCaptureDescriptorDataEXT.html) (nullable)
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT =
  Option<vkGetImageOpaqueCaptureDescriptorDataEXT_t>;
const vkGetImageOpaqueCaptureDescriptorDataEXT_NAME: &str =
  "vkGetImageOpaqueCaptureDescriptorDataEXT\0";

/// Khronos: [vkGetImageSparseMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html) (non-nullable)
pub type vkGetImageSparseMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  sparse_memory_requirement_count: *mut u32,
  sparse_memory_requirements: *mut VkSparseImageMemoryRequirements,
);
/// Khronos: [vkGetImageSparseMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html) (nullable)
pub type PFN_vkGetImageSparseMemoryRequirements =
  Option<vkGetImageSparseMemoryRequirements_t>;
const vkGetImageSparseMemoryRequirements_NAME: &str =
  "vkGetImageSparseMemoryRequirements\0";

/// Khronos: [vkGetImageSparseMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html) (non-nullable)
pub type vkGetImageSparseMemoryRequirements2_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkImageSparseMemoryRequirementsInfo2,
  sparse_memory_requirement_count: *mut u32,
  sparse_memory_requirements: *mut VkSparseImageMemoryRequirements2,
);
/// Khronos: [vkGetImageSparseMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html) (nullable)
pub type PFN_vkGetImageSparseMemoryRequirements2 =
  Option<vkGetImageSparseMemoryRequirements2_t>;
const vkGetImageSparseMemoryRequirements2_NAME: &str =
  "vkGetImageSparseMemoryRequirements2\0";

/// Khronos: [vkGetImageSubresourceLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html) (non-nullable)
pub type vkGetImageSubresourceLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  subresource: *const VkImageSubresource,
  layout: *mut VkSubresourceLayout,
);
/// Khronos: [vkGetImageSubresourceLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html) (nullable)
pub type PFN_vkGetImageSubresourceLayout = Option<vkGetImageSubresourceLayout_t>;
const vkGetImageSubresourceLayout_NAME: &str = "vkGetImageSubresourceLayout\0";

/// Khronos: [vkGetImageSubresourceLayout2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html) (non-nullable)
pub type vkGetImageSubresourceLayout2EXT_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  subresource: *const VkImageSubresource2EXT,
  layout: *mut VkSubresourceLayout2EXT,
);
/// Khronos: [vkGetImageSubresourceLayout2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html) (nullable)
pub type PFN_vkGetImageSubresourceLayout2EXT = Option<vkGetImageSubresourceLayout2EXT_t>;
const vkGetImageSubresourceLayout2EXT_NAME: &str = "vkGetImageSubresourceLayout2EXT\0";

/// Khronos: [vkGetImageViewAddressNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html) (non-nullable)
pub type vkGetImageViewAddressNVX_t = unsafe extern "system" fn(
  device: VkDevice,
  image_view: VkImageView,
  properties: *mut VkImageViewAddressPropertiesNVX,
) -> VkResult;
/// Khronos: [vkGetImageViewAddressNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html) (nullable)
pub type PFN_vkGetImageViewAddressNVX = Option<vkGetImageViewAddressNVX_t>;
const vkGetImageViewAddressNVX_NAME: &str = "vkGetImageViewAddressNVX\0";

/// Khronos: [vkGetImageViewHandleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html) (non-nullable)
pub type vkGetImageViewHandleNVX_t = unsafe extern "system" fn(
  device: VkDevice,
  info: *const VkImageViewHandleInfoNVX,
) -> u32;
/// Khronos: [vkGetImageViewHandleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html) (nullable)
pub type PFN_vkGetImageViewHandleNVX = Option<vkGetImageViewHandleNVX_t>;
const vkGetImageViewHandleNVX_NAME: &str = "vkGetImageViewHandleNVX\0";

/// Khronos: [vkGetImageViewOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html) (non-nullable)
pub type vkGetImageViewOpaqueCaptureDescriptorDataEXT_t =
  unsafe extern "system" fn(
    device: VkDevice,
    info: *const VkImageViewCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
  ) -> VkResult;
/// Khronos: [vkGetImageViewOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html) (nullable)
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT =
  Option<vkGetImageViewOpaqueCaptureDescriptorDataEXT_t>;
const vkGetImageViewOpaqueCaptureDescriptorDataEXT_NAME: &str =
  "vkGetImageViewOpaqueCaptureDescriptorDataEXT\0";

/// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html) (non-nullable)
pub type vkGetInstanceProcAddr_t =
  unsafe extern "system" fn(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
/// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html) (nullable)
pub type PFN_vkGetInstanceProcAddr = Option<vkGetInstanceProcAddr_t>;
const vkGetInstanceProcAddr_NAME: &str = "vkGetInstanceProcAddr\0";

/// Khronos: [vkGetMemoryAndroidHardwareBufferANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) (non-nullable)
pub type vkGetMemoryAndroidHardwareBufferANDROID_t =
  unsafe extern "system" fn(
    device: VkDevice,
    info: *const VkMemoryGetAndroidHardwareBufferInfoANDROID,
    buffer: *mut *mut AHardwareBuffer,
  ) -> VkResult;
/// Khronos: [vkGetMemoryAndroidHardwareBufferANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html) (nullable)
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID =
  Option<vkGetMemoryAndroidHardwareBufferANDROID_t>;
const vkGetMemoryAndroidHardwareBufferANDROID_NAME: &str =
  "vkGetMemoryAndroidHardwareBufferANDROID\0";

/// Khronos: [vkGetMemoryFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html) (non-nullable)
pub type vkGetMemoryFdKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  get_fd_info: *const VkMemoryGetFdInfoKHR,
  fd: *mut c_int,
) -> VkResult;
/// Khronos: [vkGetMemoryFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html) (nullable)
pub type PFN_vkGetMemoryFdKHR = Option<vkGetMemoryFdKHR_t>;
const vkGetMemoryFdKHR_NAME: &str = "vkGetMemoryFdKHR\0";

/// Khronos: [vkGetMemoryFdPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) (non-nullable)
pub type vkGetMemoryFdPropertiesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  handle_type: VkExternalMemoryHandleTypeFlagBits,
  fd: c_int,
  memory_fd_properties: *mut VkMemoryFdPropertiesKHR,
) -> VkResult;
/// Khronos: [vkGetMemoryFdPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) (nullable)
pub type PFN_vkGetMemoryFdPropertiesKHR = Option<vkGetMemoryFdPropertiesKHR_t>;
const vkGetMemoryFdPropertiesKHR_NAME: &str = "vkGetMemoryFdPropertiesKHR\0";

/// Khronos: [vkGetMemoryHostPointerPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) (non-nullable)
pub type vkGetMemoryHostPointerPropertiesEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  handle_type: VkExternalMemoryHandleTypeFlagBits,
  host_pointer: *const c_void,
  memory_host_pointer_properties: *mut VkMemoryHostPointerPropertiesEXT,
) -> VkResult;
/// Khronos: [vkGetMemoryHostPointerPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) (nullable)
pub type PFN_vkGetMemoryHostPointerPropertiesEXT =
  Option<vkGetMemoryHostPointerPropertiesEXT_t>;
const vkGetMemoryHostPointerPropertiesEXT_NAME: &str =
  "vkGetMemoryHostPointerPropertiesEXT\0";

/// Khronos: [vkGetMemoryRemoteAddressNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html) (non-nullable)
pub type vkGetMemoryRemoteAddressNV_t = unsafe extern "system" fn(
  device: VkDevice,
  memory_get_remote_address_info: *const VkMemoryGetRemoteAddressInfoNV,
  address: *mut VkRemoteAddressNV,
) -> VkResult;
/// Khronos: [vkGetMemoryRemoteAddressNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html) (nullable)
pub type PFN_vkGetMemoryRemoteAddressNV = Option<vkGetMemoryRemoteAddressNV_t>;
const vkGetMemoryRemoteAddressNV_NAME: &str = "vkGetMemoryRemoteAddressNV\0";

/// Khronos: [vkGetMemoryWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html) (non-nullable)
pub type vkGetMemoryWin32HandleKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  get_win_32_handle_info: *const VkMemoryGetWin32HandleInfoKHR,
  handle: *mut HANDLE,
) -> VkResult;
/// Khronos: [vkGetMemoryWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html) (nullable)
pub type PFN_vkGetMemoryWin32HandleKHR = Option<vkGetMemoryWin32HandleKHR_t>;
const vkGetMemoryWin32HandleKHR_NAME: &str = "vkGetMemoryWin32HandleKHR\0";

/// Khronos: [vkGetMemoryWin32HandleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html) (non-nullable)
pub type vkGetMemoryWin32HandleNV_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
  handle_type: VkExternalMemoryHandleTypeFlagsNV,
  handle: *mut HANDLE,
) -> VkResult;
/// Khronos: [vkGetMemoryWin32HandleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html) (nullable)
pub type PFN_vkGetMemoryWin32HandleNV = Option<vkGetMemoryWin32HandleNV_t>;
const vkGetMemoryWin32HandleNV_NAME: &str = "vkGetMemoryWin32HandleNV\0";

/// Khronos: [vkGetMemoryWin32HandlePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html) (non-nullable)
pub type vkGetMemoryWin32HandlePropertiesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  handle_type: VkExternalMemoryHandleTypeFlagBits,
  handle: HANDLE,
  memory_win_32_handle_properties: *mut VkMemoryWin32HandlePropertiesKHR,
) -> VkResult;
/// Khronos: [vkGetMemoryWin32HandlePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html) (nullable)
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR =
  Option<vkGetMemoryWin32HandlePropertiesKHR_t>;
const vkGetMemoryWin32HandlePropertiesKHR_NAME: &str =
  "vkGetMemoryWin32HandlePropertiesKHR\0";

/// Khronos: [vkGetMemoryZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html) (non-nullable)
pub type vkGetMemoryZirconHandleFUCHSIA_t = unsafe extern "system" fn(
  device: VkDevice,
  get_zircon_handle_info: *const VkMemoryGetZirconHandleInfoFUCHSIA,
  zircon_handle: *mut zx_handle_t,
) -> VkResult;
/// Khronos: [vkGetMemoryZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html) (nullable)
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = Option<vkGetMemoryZirconHandleFUCHSIA_t>;
const vkGetMemoryZirconHandleFUCHSIA_NAME: &str = "vkGetMemoryZirconHandleFUCHSIA\0";

/// Khronos: [vkGetMemoryZirconHandlePropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html) (non-nullable)
pub type vkGetMemoryZirconHandlePropertiesFUCHSIA_t =
  unsafe extern "system" fn(
    device: VkDevice,
    handle_type: VkExternalMemoryHandleTypeFlagBits,
    zircon_handle: zx_handle_t,
    memory_zircon_handle_properties: *mut VkMemoryZirconHandlePropertiesFUCHSIA,
  ) -> VkResult;
/// Khronos: [vkGetMemoryZirconHandlePropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html) (nullable)
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA =
  Option<vkGetMemoryZirconHandlePropertiesFUCHSIA_t>;
const vkGetMemoryZirconHandlePropertiesFUCHSIA_NAME: &str =
  "vkGetMemoryZirconHandlePropertiesFUCHSIA\0";

/// Khronos: [vkGetPastPresentationTimingGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) (non-nullable)
pub type vkGetPastPresentationTimingGOOGLE_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  presentation_timing_count: *mut u32,
  presentation_timings: *mut VkPastPresentationTimingGOOGLE,
) -> VkResult;
/// Khronos: [vkGetPastPresentationTimingGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) (nullable)
pub type PFN_vkGetPastPresentationTimingGOOGLE =
  Option<vkGetPastPresentationTimingGOOGLE_t>;
const vkGetPastPresentationTimingGOOGLE_NAME: &str =
  "vkGetPastPresentationTimingGOOGLE\0";

/// Khronos: [vkGetPhysicalDeviceCalibrateableTimeDomainsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) (non-nullable)
pub type vkGetPhysicalDeviceCalibrateableTimeDomainsEXT_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    time_domain_count: *mut u32,
    time_domains: *mut VkTimeDomainEXT,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceCalibrateableTimeDomainsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) (nullable)
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT =
  Option<vkGetPhysicalDeviceCalibrateableTimeDomainsEXT_t>;
const vkGetPhysicalDeviceCalibrateableTimeDomainsEXT_NAME: &str =
  "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0";

/// Khronos: [vkGetPhysicalDeviceCooperativeMatrixPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html) (non-nullable)
pub type vkGetPhysicalDeviceCooperativeMatrixPropertiesNV_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut VkCooperativeMatrixPropertiesNV,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceCooperativeMatrixPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html) (nullable)
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV =
  Option<vkGetPhysicalDeviceCooperativeMatrixPropertiesNV_t>;
const vkGetPhysicalDeviceCooperativeMatrixPropertiesNV_NAME: &str =
  "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV\0";

/// Khronos: [vkGetPhysicalDeviceDirectFBPresentationSupportEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html) (non-nullable)
pub type vkGetPhysicalDeviceDirectFBPresentationSupportEXT_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    dfb: *mut IDirectFB,
  ) -> VkBool32;
/// Khronos: [vkGetPhysicalDeviceDirectFBPresentationSupportEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html) (nullable)
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT =
  Option<vkGetPhysicalDeviceDirectFBPresentationSupportEXT_t>;
const vkGetPhysicalDeviceDirectFBPresentationSupportEXT_NAME: &str =
  "vkGetPhysicalDeviceDirectFBPresentationSupportEXT\0";

/// Khronos: [vkGetPhysicalDeviceDisplayPlaneProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html) (non-nullable)
pub type vkGetPhysicalDeviceDisplayPlaneProperties2KHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut VkDisplayPlaneProperties2KHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceDisplayPlaneProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR =
  Option<vkGetPhysicalDeviceDisplayPlaneProperties2KHR_t>;
const vkGetPhysicalDeviceDisplayPlaneProperties2KHR_NAME: &str =
  "vkGetPhysicalDeviceDisplayPlaneProperties2KHR\0";

/// Khronos: [vkGetPhysicalDeviceDisplayPlanePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceDisplayPlanePropertiesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut VkDisplayPlanePropertiesKHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceDisplayPlanePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR =
  Option<vkGetPhysicalDeviceDisplayPlanePropertiesKHR_t>;
const vkGetPhysicalDeviceDisplayPlanePropertiesKHR_NAME: &str =
  "vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0";

/// Khronos: [vkGetPhysicalDeviceDisplayProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html) (non-nullable)
pub type vkGetPhysicalDeviceDisplayProperties2KHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut VkDisplayProperties2KHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceDisplayProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR =
  Option<vkGetPhysicalDeviceDisplayProperties2KHR_t>;
const vkGetPhysicalDeviceDisplayProperties2KHR_NAME: &str =
  "vkGetPhysicalDeviceDisplayProperties2KHR\0";

/// Khronos: [vkGetPhysicalDeviceDisplayPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceDisplayPropertiesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut VkDisplayPropertiesKHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceDisplayPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR =
  Option<vkGetPhysicalDeviceDisplayPropertiesKHR_t>;
const vkGetPhysicalDeviceDisplayPropertiesKHR_NAME: &str =
  "vkGetPhysicalDeviceDisplayPropertiesKHR\0";

/// Khronos: [vkGetPhysicalDeviceExternalBufferProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceExternalBufferProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  external_buffer_info: *const VkPhysicalDeviceExternalBufferInfo,
  external_buffer_properties: *mut VkExternalBufferProperties,
);
/// Khronos: [vkGetPhysicalDeviceExternalBufferProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties =
  Option<vkGetPhysicalDeviceExternalBufferProperties_t>;
const vkGetPhysicalDeviceExternalBufferProperties_NAME: &str =
  "vkGetPhysicalDeviceExternalBufferProperties\0";

/// Khronos: [vkGetPhysicalDeviceExternalFenceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceExternalFenceProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  external_fence_info: *const VkPhysicalDeviceExternalFenceInfo,
  external_fence_properties: *mut VkExternalFenceProperties,
);
/// Khronos: [vkGetPhysicalDeviceExternalFenceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties =
  Option<vkGetPhysicalDeviceExternalFenceProperties_t>;
const vkGetPhysicalDeviceExternalFenceProperties_NAME: &str =
  "vkGetPhysicalDeviceExternalFenceProperties\0";

/// Khronos: [vkGetPhysicalDeviceExternalImageFormatPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html) (non-nullable)
pub type vkGetPhysicalDeviceExternalImageFormatPropertiesNV_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    format: VkFormat,
    ty: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    external_handle_type: VkExternalMemoryHandleTypeFlagsNV,
    external_image_format_properties: *mut VkExternalImageFormatPropertiesNV,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceExternalImageFormatPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html) (nullable)
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV =
  Option<vkGetPhysicalDeviceExternalImageFormatPropertiesNV_t>;
const vkGetPhysicalDeviceExternalImageFormatPropertiesNV_NAME: &str =
  "vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0";

/// Khronos: [vkGetPhysicalDeviceExternalSemaphoreProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceExternalSemaphoreProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  external_semaphore_info: *const VkPhysicalDeviceExternalSemaphoreInfo,
  external_semaphore_properties: *mut VkExternalSemaphoreProperties,
);
/// Khronos: [vkGetPhysicalDeviceExternalSemaphoreProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties =
  Option<vkGetPhysicalDeviceExternalSemaphoreProperties_t>;
const vkGetPhysicalDeviceExternalSemaphoreProperties_NAME: &str =
  "vkGetPhysicalDeviceExternalSemaphoreProperties\0";

/// Khronos: [vkGetPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html) (non-nullable)
pub type vkGetPhysicalDeviceFeatures_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  features: *mut VkPhysicalDeviceFeatures,
);
/// Khronos: [vkGetPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html) (nullable)
pub type PFN_vkGetPhysicalDeviceFeatures = Option<vkGetPhysicalDeviceFeatures_t>;
const vkGetPhysicalDeviceFeatures_NAME: &str = "vkGetPhysicalDeviceFeatures\0";

/// Khronos: [vkGetPhysicalDeviceFeatures2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html) (non-nullable)
pub type vkGetPhysicalDeviceFeatures2_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  features: *mut VkPhysicalDeviceFeatures2,
);
/// Khronos: [vkGetPhysicalDeviceFeatures2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html) (nullable)
pub type PFN_vkGetPhysicalDeviceFeatures2 = Option<vkGetPhysicalDeviceFeatures2_t>;
const vkGetPhysicalDeviceFeatures2_NAME: &str = "vkGetPhysicalDeviceFeatures2\0";

/// Khronos: [vkGetPhysicalDeviceFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceFormatProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  format: VkFormat,
  format_properties: *mut VkFormatProperties,
);
/// Khronos: [vkGetPhysicalDeviceFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceFormatProperties =
  Option<vkGetPhysicalDeviceFormatProperties_t>;
const vkGetPhysicalDeviceFormatProperties_NAME: &str =
  "vkGetPhysicalDeviceFormatProperties\0";

/// Khronos: [vkGetPhysicalDeviceFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html) (non-nullable)
pub type vkGetPhysicalDeviceFormatProperties2_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  format: VkFormat,
  format_properties: *mut VkFormatProperties2,
);
/// Khronos: [vkGetPhysicalDeviceFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html) (nullable)
pub type PFN_vkGetPhysicalDeviceFormatProperties2 =
  Option<vkGetPhysicalDeviceFormatProperties2_t>;
const vkGetPhysicalDeviceFormatProperties2_NAME: &str =
  "vkGetPhysicalDeviceFormatProperties2\0";

/// Khronos: [vkGetPhysicalDeviceFragmentShadingRatesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceFragmentShadingRatesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    fragment_shading_rate_count: *mut u32,
    fragment_shading_rates: *mut VkPhysicalDeviceFragmentShadingRateKHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceFragmentShadingRatesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR =
  Option<vkGetPhysicalDeviceFragmentShadingRatesKHR_t>;
const vkGetPhysicalDeviceFragmentShadingRatesKHR_NAME: &str =
  "vkGetPhysicalDeviceFragmentShadingRatesKHR\0";

/// Khronos: [vkGetPhysicalDeviceImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceImageFormatProperties_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    format: VkFormat,
    ty: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    image_format_properties: *mut VkImageFormatProperties,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceImageFormatProperties =
  Option<vkGetPhysicalDeviceImageFormatProperties_t>;
const vkGetPhysicalDeviceImageFormatProperties_NAME: &str =
  "vkGetPhysicalDeviceImageFormatProperties\0";

/// Khronos: [vkGetPhysicalDeviceImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html) (non-nullable)
pub type vkGetPhysicalDeviceImageFormatProperties2_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    image_format_info: *const VkPhysicalDeviceImageFormatInfo2,
    image_format_properties: *mut VkImageFormatProperties2,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html) (nullable)
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 =
  Option<vkGetPhysicalDeviceImageFormatProperties2_t>;
const vkGetPhysicalDeviceImageFormatProperties2_NAME: &str =
  "vkGetPhysicalDeviceImageFormatProperties2\0";

/// Khronos: [vkGetPhysicalDeviceMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceMemoryProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  memory_properties: *mut VkPhysicalDeviceMemoryProperties,
);
/// Khronos: [vkGetPhysicalDeviceMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceMemoryProperties =
  Option<vkGetPhysicalDeviceMemoryProperties_t>;
const vkGetPhysicalDeviceMemoryProperties_NAME: &str =
  "vkGetPhysicalDeviceMemoryProperties\0";

/// Khronos: [vkGetPhysicalDeviceMemoryProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html) (non-nullable)
pub type vkGetPhysicalDeviceMemoryProperties2_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  memory_properties: *mut VkPhysicalDeviceMemoryProperties2,
);
/// Khronos: [vkGetPhysicalDeviceMemoryProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html) (nullable)
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 =
  Option<vkGetPhysicalDeviceMemoryProperties2_t>;
const vkGetPhysicalDeviceMemoryProperties2_NAME: &str =
  "vkGetPhysicalDeviceMemoryProperties2\0";

/// Khronos: [vkGetPhysicalDeviceMultisamplePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) (non-nullable)
pub type vkGetPhysicalDeviceMultisamplePropertiesEXT_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  samples: VkSampleCountFlagBits,
  multisample_properties: *mut VkMultisamplePropertiesEXT,
);
/// Khronos: [vkGetPhysicalDeviceMultisamplePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) (nullable)
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT =
  Option<vkGetPhysicalDeviceMultisamplePropertiesEXT_t>;
const vkGetPhysicalDeviceMultisamplePropertiesEXT_NAME: &str =
  "vkGetPhysicalDeviceMultisamplePropertiesEXT\0";

/// Khronos: [vkGetPhysicalDeviceOpticalFlowImageFormatsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html) (non-nullable)
pub type vkGetPhysicalDeviceOpticalFlowImageFormatsNV_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    optical_flow_image_format_info: *const VkOpticalFlowImageFormatInfoNV,
    format_count: *mut u32,
    image_format_properties: *mut VkOpticalFlowImageFormatPropertiesNV,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceOpticalFlowImageFormatsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html) (nullable)
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV =
  Option<vkGetPhysicalDeviceOpticalFlowImageFormatsNV_t>;
const vkGetPhysicalDeviceOpticalFlowImageFormatsNV_NAME: &str =
  "vkGetPhysicalDeviceOpticalFlowImageFormatsNV\0";

/// Khronos: [vkGetPhysicalDevicePresentRectanglesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) (non-nullable)
pub type vkGetPhysicalDevicePresentRectanglesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    rect_count: *mut u32,
    rects: *mut VkRect2D,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDevicePresentRectanglesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR =
  Option<vkGetPhysicalDevicePresentRectanglesKHR_t>;
const vkGetPhysicalDevicePresentRectanglesKHR_NAME: &str =
  "vkGetPhysicalDevicePresentRectanglesKHR\0";

/// Khronos: [vkGetPhysicalDeviceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  properties: *mut VkPhysicalDeviceProperties,
);
/// Khronos: [vkGetPhysicalDeviceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceProperties = Option<vkGetPhysicalDeviceProperties_t>;
const vkGetPhysicalDeviceProperties_NAME: &str = "vkGetPhysicalDeviceProperties\0";

/// Khronos: [vkGetPhysicalDeviceProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html) (non-nullable)
pub type vkGetPhysicalDeviceProperties2_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  properties: *mut VkPhysicalDeviceProperties2,
);
/// Khronos: [vkGetPhysicalDeviceProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html) (nullable)
pub type PFN_vkGetPhysicalDeviceProperties2 = Option<vkGetPhysicalDeviceProperties2_t>;
const vkGetPhysicalDeviceProperties2_NAME: &str = "vkGetPhysicalDeviceProperties2\0";

/// Khronos: [vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    performance_query_create_info: *const VkQueryPoolPerformanceCreateInfoKHR,
    num_passes: *mut u32,
  );
/// Khronos: [vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR =
  Option<vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR_t>;
const vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR_NAME: &str =
  "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR\0";

/// Khronos: [vkGetPhysicalDeviceQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceQueueFamilyProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  queue_family_property_count: *mut u32,
  queue_family_properties: *mut VkQueueFamilyProperties,
);
/// Khronos: [vkGetPhysicalDeviceQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties =
  Option<vkGetPhysicalDeviceQueueFamilyProperties_t>;
const vkGetPhysicalDeviceQueueFamilyProperties_NAME: &str =
  "vkGetPhysicalDeviceQueueFamilyProperties\0";

/// Khronos: [vkGetPhysicalDeviceQueueFamilyProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html) (non-nullable)
pub type vkGetPhysicalDeviceQueueFamilyProperties2_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  queue_family_property_count: *mut u32,
  queue_family_properties: *mut VkQueueFamilyProperties2,
);
/// Khronos: [vkGetPhysicalDeviceQueueFamilyProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html) (nullable)
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 =
  Option<vkGetPhysicalDeviceQueueFamilyProperties2_t>;
const vkGetPhysicalDeviceQueueFamilyProperties2_NAME: &str =
  "vkGetPhysicalDeviceQueueFamilyProperties2\0";

/// Khronos: [vkGetPhysicalDeviceRefreshableObjectTypesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceRefreshableObjectTypesKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceRefreshableObjectTypesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    refreshable_object_type_count: *mut u32,
    refreshable_object_types: *mut VkObjectType,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceRefreshableObjectTypesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceRefreshableObjectTypesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceRefreshableObjectTypesKHR =
  Option<vkGetPhysicalDeviceRefreshableObjectTypesKHR_t>;
const vkGetPhysicalDeviceRefreshableObjectTypesKHR_NAME: &str =
  "vkGetPhysicalDeviceRefreshableObjectTypesKHR\0";

/// Khronos: [vkGetPhysicalDeviceScreenPresentationSupportQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html) (non-nullable)
pub type vkGetPhysicalDeviceScreenPresentationSupportQNX_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    window: *mut _screen_window,
  ) -> VkBool32;
/// Khronos: [vkGetPhysicalDeviceScreenPresentationSupportQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html) (nullable)
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX =
  Option<vkGetPhysicalDeviceScreenPresentationSupportQNX_t>;
const vkGetPhysicalDeviceScreenPresentationSupportQNX_NAME: &str =
  "vkGetPhysicalDeviceScreenPresentationSupportQNX\0";

/// Khronos: [vkGetPhysicalDeviceSparseImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceSparseImageFormatProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  format: VkFormat,
  ty: VkImageType,
  samples: VkSampleCountFlagBits,
  usage: VkImageUsageFlags,
  tiling: VkImageTiling,
  property_count: *mut u32,
  properties: *mut VkSparseImageFormatProperties,
);
/// Khronos: [vkGetPhysicalDeviceSparseImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties =
  Option<vkGetPhysicalDeviceSparseImageFormatProperties_t>;
const vkGetPhysicalDeviceSparseImageFormatProperties_NAME: &str =
  "vkGetPhysicalDeviceSparseImageFormatProperties\0";

/// Khronos: [vkGetPhysicalDeviceSparseImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html) (non-nullable)
pub type vkGetPhysicalDeviceSparseImageFormatProperties2_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    format_info: *const VkPhysicalDeviceSparseImageFormatInfo2,
    property_count: *mut u32,
    properties: *mut VkSparseImageFormatProperties2,
  );
/// Khronos: [vkGetPhysicalDeviceSparseImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 =
  Option<vkGetPhysicalDeviceSparseImageFormatProperties2_t>;
const vkGetPhysicalDeviceSparseImageFormatProperties2_NAME: &str =
  "vkGetPhysicalDeviceSparseImageFormatProperties2\0";

/// Khronos: [vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html) (non-nullable)
pub type vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    combination_count: *mut u32,
    combinations: *mut VkFramebufferMixedSamplesCombinationNV,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
  Option<vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV_t>;
const vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV_NAME: &str =
  "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV\0";

/// Khronos: [vkGetPhysicalDeviceSurfaceCapabilities2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html) (non-nullable)
pub type vkGetPhysicalDeviceSurfaceCapabilities2EXT_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    surface_capabilities: *mut VkSurfaceCapabilities2EXT,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceSurfaceCapabilities2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT =
  Option<vkGetPhysicalDeviceSurfaceCapabilities2EXT_t>;
const vkGetPhysicalDeviceSurfaceCapabilities2EXT_NAME: &str =
  "vkGetPhysicalDeviceSurfaceCapabilities2EXT\0";

/// Khronos: [vkGetPhysicalDeviceSurfaceCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html) (non-nullable)
pub type vkGetPhysicalDeviceSurfaceCapabilities2KHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface_info: *const VkPhysicalDeviceSurfaceInfo2KHR,
    surface_capabilities: *mut VkSurfaceCapabilities2KHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceSurfaceCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR =
  Option<vkGetPhysicalDeviceSurfaceCapabilities2KHR_t>;
const vkGetPhysicalDeviceSurfaceCapabilities2KHR_NAME: &str =
  "vkGetPhysicalDeviceSurfaceCapabilities2KHR\0";

/// Khronos: [vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceSurfaceCapabilitiesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    surface_capabilities: *mut VkSurfaceCapabilitiesKHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR =
  Option<vkGetPhysicalDeviceSurfaceCapabilitiesKHR_t>;
const vkGetPhysicalDeviceSurfaceCapabilitiesKHR_NAME: &str =
  "vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0";

/// Khronos: [vkGetPhysicalDeviceSurfaceFormats2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html) (non-nullable)
pub type vkGetPhysicalDeviceSurfaceFormats2KHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  surface_info: *const VkPhysicalDeviceSurfaceInfo2KHR,
  surface_format_count: *mut u32,
  surface_formats: *mut VkSurfaceFormat2KHR,
) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceSurfaceFormats2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR =
  Option<vkGetPhysicalDeviceSurfaceFormats2KHR_t>;
const vkGetPhysicalDeviceSurfaceFormats2KHR_NAME: &str =
  "vkGetPhysicalDeviceSurfaceFormats2KHR\0";

/// Khronos: [vkGetPhysicalDeviceSurfaceFormatsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceSurfaceFormatsKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  surface: VkSurfaceKHR,
  surface_format_count: *mut u32,
  surface_formats: *mut VkSurfaceFormatKHR,
) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceSurfaceFormatsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR =
  Option<vkGetPhysicalDeviceSurfaceFormatsKHR_t>;
const vkGetPhysicalDeviceSurfaceFormatsKHR_NAME: &str =
  "vkGetPhysicalDeviceSurfaceFormatsKHR\0";

/// Khronos: [vkGetPhysicalDeviceSurfacePresentModes2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) (non-nullable)
pub type vkGetPhysicalDeviceSurfacePresentModes2EXT_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface_info: *const VkPhysicalDeviceSurfaceInfo2KHR,
    present_mode_count: *mut u32,
    present_modes: *mut VkPresentModeKHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceSurfacePresentModes2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT =
  Option<vkGetPhysicalDeviceSurfacePresentModes2EXT_t>;
const vkGetPhysicalDeviceSurfacePresentModes2EXT_NAME: &str =
  "vkGetPhysicalDeviceSurfacePresentModes2EXT\0";

/// Khronos: [vkGetPhysicalDeviceSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceSurfacePresentModesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    present_mode_count: *mut u32,
    present_modes: *mut VkPresentModeKHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR =
  Option<vkGetPhysicalDeviceSurfacePresentModesKHR_t>;
const vkGetPhysicalDeviceSurfacePresentModesKHR_NAME: &str =
  "vkGetPhysicalDeviceSurfacePresentModesKHR\0";

/// Khronos: [vkGetPhysicalDeviceSurfaceSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceSurfaceSupportKHR_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  queue_family_index: u32,
  surface: VkSurfaceKHR,
  supported: *mut VkBool32,
) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceSurfaceSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR =
  Option<vkGetPhysicalDeviceSurfaceSupportKHR_t>;
const vkGetPhysicalDeviceSurfaceSupportKHR_NAME: &str =
  "vkGetPhysicalDeviceSurfaceSupportKHR\0";

/// Khronos: [vkGetPhysicalDeviceToolProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html) (non-nullable)
pub type vkGetPhysicalDeviceToolProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  tool_count: *mut u32,
  tool_properties: *mut VkPhysicalDeviceToolProperties,
) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceToolProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html) (nullable)
pub type PFN_vkGetPhysicalDeviceToolProperties =
  Option<vkGetPhysicalDeviceToolProperties_t>;
const vkGetPhysicalDeviceToolProperties_NAME: &str =
  "vkGetPhysicalDeviceToolProperties\0";

/// Khronos: [vkGetPhysicalDeviceVideoCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceVideoCapabilitiesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    video_profile: *const VkVideoProfileInfoKHR,
    capabilities: *mut VkVideoCapabilitiesKHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceVideoCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR =
  Option<vkGetPhysicalDeviceVideoCapabilitiesKHR_t>;
const vkGetPhysicalDeviceVideoCapabilitiesKHR_NAME: &str =
  "vkGetPhysicalDeviceVideoCapabilitiesKHR\0";

/// Khronos: [vkGetPhysicalDeviceVideoFormatPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceVideoFormatPropertiesKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    video_format_info: *const VkPhysicalDeviceVideoFormatInfoKHR,
    video_format_property_count: *mut u32,
    video_format_properties: *mut VkVideoFormatPropertiesKHR,
  ) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceVideoFormatPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR =
  Option<vkGetPhysicalDeviceVideoFormatPropertiesKHR_t>;
const vkGetPhysicalDeviceVideoFormatPropertiesKHR_NAME: &str =
  "vkGetPhysicalDeviceVideoFormatPropertiesKHR\0";

/// Khronos: [vkGetPhysicalDeviceWaylandPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceWaylandPresentationSupportKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
  ) -> VkBool32;
/// Khronos: [vkGetPhysicalDeviceWaylandPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR =
  Option<vkGetPhysicalDeviceWaylandPresentationSupportKHR_t>;
const vkGetPhysicalDeviceWaylandPresentationSupportKHR_NAME: &str =
  "vkGetPhysicalDeviceWaylandPresentationSupportKHR\0";

/// Khronos: [vkGetPhysicalDeviceWin32PresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceWin32PresentationSupportKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
  ) -> VkBool32;
/// Khronos: [vkGetPhysicalDeviceWin32PresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
  Option<vkGetPhysicalDeviceWin32PresentationSupportKHR_t>;
const vkGetPhysicalDeviceWin32PresentationSupportKHR_NAME: &str =
  "vkGetPhysicalDeviceWin32PresentationSupportKHR\0";

/// Khronos: [vkGetPhysicalDeviceXcbPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceXcbPresentationSupportKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
  ) -> VkBool32;
/// Khronos: [vkGetPhysicalDeviceXcbPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR =
  Option<vkGetPhysicalDeviceXcbPresentationSupportKHR_t>;
const vkGetPhysicalDeviceXcbPresentationSupportKHR_NAME: &str =
  "vkGetPhysicalDeviceXcbPresentationSupportKHR\0";

/// Khronos: [vkGetPhysicalDeviceXlibPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) (non-nullable)
pub type vkGetPhysicalDeviceXlibPresentationSupportKHR_t =
  unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    dpy: *mut XlibDisplay,
    visual_id: XlibVisualID,
  ) -> VkBool32;
/// Khronos: [vkGetPhysicalDeviceXlibPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) (nullable)
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR =
  Option<vkGetPhysicalDeviceXlibPresentationSupportKHR_t>;
const vkGetPhysicalDeviceXlibPresentationSupportKHR_NAME: &str =
  "vkGetPhysicalDeviceXlibPresentationSupportKHR\0";

/// Khronos: [vkGetPipelineCacheData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html) (non-nullable)
pub type vkGetPipelineCacheData_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_cache: VkPipelineCache,
  data_size: *mut c_size_t,
  data: *mut c_void,
) -> VkResult;
/// Khronos: [vkGetPipelineCacheData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html) (nullable)
pub type PFN_vkGetPipelineCacheData = Option<vkGetPipelineCacheData_t>;
const vkGetPipelineCacheData_NAME: &str = "vkGetPipelineCacheData\0";

/// Khronos: [vkGetPipelineExecutableInternalRepresentationsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) (non-nullable)
pub type vkGetPipelineExecutableInternalRepresentationsKHR_t =
  unsafe extern "system" fn(
    device: VkDevice,
    executable_info: *const VkPipelineExecutableInfoKHR,
    internal_representation_count: *mut u32,
    internal_representations: *mut VkPipelineExecutableInternalRepresentationKHR,
  ) -> VkResult;
/// Khronos: [vkGetPipelineExecutableInternalRepresentationsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) (nullable)
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
  Option<vkGetPipelineExecutableInternalRepresentationsKHR_t>;
const vkGetPipelineExecutableInternalRepresentationsKHR_NAME: &str =
  "vkGetPipelineExecutableInternalRepresentationsKHR\0";

/// Khronos: [vkGetPipelineExecutablePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) (non-nullable)
pub type vkGetPipelineExecutablePropertiesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_info: *const VkPipelineInfoKHR,
  executable_count: *mut u32,
  properties: *mut VkPipelineExecutablePropertiesKHR,
) -> VkResult;
/// Khronos: [vkGetPipelineExecutablePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) (nullable)
pub type PFN_vkGetPipelineExecutablePropertiesKHR =
  Option<vkGetPipelineExecutablePropertiesKHR_t>;
const vkGetPipelineExecutablePropertiesKHR_NAME: &str =
  "vkGetPipelineExecutablePropertiesKHR\0";

/// Khronos: [vkGetPipelinePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html) (non-nullable)
pub type vkGetPipelinePropertiesEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_info: *const VkPipelineInfoEXT,
  pipeline_properties: *mut VkBaseOutStructure,
) -> VkResult;
/// Khronos: [vkGetPipelinePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html) (nullable)
pub type PFN_vkGetPipelinePropertiesEXT = Option<vkGetPipelinePropertiesEXT_t>;
const vkGetPipelinePropertiesEXT_NAME: &str = "vkGetPipelinePropertiesEXT\0";

/// Khronos: [vkGetPrivateData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html) (non-nullable)
pub type vkGetPrivateData_t = unsafe extern "system" fn(
  device: VkDevice,
  object_type: VkObjectType,
  object_handle: u64,
  private_data_slot: VkPrivateDataSlot,
  data: *mut u64,
);
/// Khronos: [vkGetPrivateData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html) (nullable)
pub type PFN_vkGetPrivateData = Option<vkGetPrivateData_t>;
const vkGetPrivateData_NAME: &str = "vkGetPrivateData\0";

/// Khronos: [vkGetQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html) (non-nullable)
pub type vkGetQueryPoolResults_t = unsafe extern "system" fn(
  device: VkDevice,
  query_pool: VkQueryPool,
  first_query: u32,
  query_count: u32,
  data_size: c_size_t,
  data: *mut c_void,
  stride: VkDeviceSize,
  flags: VkQueryResultFlags,
) -> VkResult;
/// Khronos: [vkGetQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html) (nullable)
pub type PFN_vkGetQueryPoolResults = Option<vkGetQueryPoolResults_t>;
const vkGetQueryPoolResults_NAME: &str = "vkGetQueryPoolResults\0";

/// Khronos: [vkGetQueueCheckpointData2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html) (non-nullable)
pub type vkGetQueueCheckpointData2NV_t = unsafe extern "system" fn(
  queue: VkQueue,
  checkpoint_data_count: *mut u32,
  checkpoint_data: *mut VkCheckpointData2NV,
);
/// Khronos: [vkGetQueueCheckpointData2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html) (nullable)
pub type PFN_vkGetQueueCheckpointData2NV = Option<vkGetQueueCheckpointData2NV_t>;
const vkGetQueueCheckpointData2NV_NAME: &str = "vkGetQueueCheckpointData2NV\0";

/// Khronos: [vkGetQueueCheckpointDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html) (non-nullable)
pub type vkGetQueueCheckpointDataNV_t = unsafe extern "system" fn(
  queue: VkQueue,
  checkpoint_data_count: *mut u32,
  checkpoint_data: *mut VkCheckpointDataNV,
);
/// Khronos: [vkGetQueueCheckpointDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html) (nullable)
pub type PFN_vkGetQueueCheckpointDataNV = Option<vkGetQueueCheckpointDataNV_t>;
const vkGetQueueCheckpointDataNV_NAME: &str = "vkGetQueueCheckpointDataNV\0";

/// Khronos: [vkGetRayTracingCaptureReplayShaderGroupHandlesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) (non-nullable)
pub type vkGetRayTracingCaptureReplayShaderGroupHandlesKHR_t =
  unsafe extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    first_group: u32,
    group_count: u32,
    data_size: c_size_t,
    data: *mut c_void,
  ) -> VkResult;
/// Khronos: [vkGetRayTracingCaptureReplayShaderGroupHandlesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) (nullable)
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
  Option<vkGetRayTracingCaptureReplayShaderGroupHandlesKHR_t>;
const vkGetRayTracingCaptureReplayShaderGroupHandlesKHR_NAME: &str =
  "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0";

/// Khronos: [vkGetRayTracingShaderGroupHandlesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html) (non-nullable)
pub type vkGetRayTracingShaderGroupHandlesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline: VkPipeline,
  first_group: u32,
  group_count: u32,
  data_size: c_size_t,
  data: *mut c_void,
) -> VkResult;
/// Khronos: [vkGetRayTracingShaderGroupHandlesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html) (nullable)
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR =
  Option<vkGetRayTracingShaderGroupHandlesKHR_t>;
const vkGetRayTracingShaderGroupHandlesKHR_NAME: &str =
  "vkGetRayTracingShaderGroupHandlesKHR\0";

/// Khronos: [vkGetRayTracingShaderGroupStackSizeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html) (non-nullable)
pub type vkGetRayTracingShaderGroupStackSizeKHR_t =
  unsafe extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    group: u32,
    group_shader: VkShaderGroupShaderKHR,
  ) -> VkDeviceSize;
/// Khronos: [vkGetRayTracingShaderGroupStackSizeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html) (nullable)
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR =
  Option<vkGetRayTracingShaderGroupStackSizeKHR_t>;
const vkGetRayTracingShaderGroupStackSizeKHR_NAME: &str =
  "vkGetRayTracingShaderGroupStackSizeKHR\0";

/// Khronos: [vkGetRefreshCycleDurationGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) (non-nullable)
pub type vkGetRefreshCycleDurationGOOGLE_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  display_timing_properties: *mut VkRefreshCycleDurationGOOGLE,
) -> VkResult;
/// Khronos: [vkGetRefreshCycleDurationGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) (nullable)
pub type PFN_vkGetRefreshCycleDurationGOOGLE = Option<vkGetRefreshCycleDurationGOOGLE_t>;
const vkGetRefreshCycleDurationGOOGLE_NAME: &str = "vkGetRefreshCycleDurationGOOGLE\0";

/// Khronos: [vkGetRenderAreaGranularity](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html) (non-nullable)
pub type vkGetRenderAreaGranularity_t = unsafe extern "system" fn(
  device: VkDevice,
  render_pass: VkRenderPass,
  granularity: *mut VkExtent2D,
);
/// Khronos: [vkGetRenderAreaGranularity](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html) (nullable)
pub type PFN_vkGetRenderAreaGranularity = Option<vkGetRenderAreaGranularity_t>;
const vkGetRenderAreaGranularity_NAME: &str = "vkGetRenderAreaGranularity\0";

/// Khronos: [vkGetSamplerOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html) (non-nullable)
pub type vkGetSamplerOpaqueCaptureDescriptorDataEXT_t =
  unsafe extern "system" fn(
    device: VkDevice,
    info: *const VkSamplerCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
  ) -> VkResult;
/// Khronos: [vkGetSamplerOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html) (nullable)
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT =
  Option<vkGetSamplerOpaqueCaptureDescriptorDataEXT_t>;
const vkGetSamplerOpaqueCaptureDescriptorDataEXT_NAME: &str =
  "vkGetSamplerOpaqueCaptureDescriptorDataEXT\0";

/// Khronos: [vkGetSemaphoreCounterValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html) (non-nullable)
pub type vkGetSemaphoreCounterValue_t = unsafe extern "system" fn(
  device: VkDevice,
  semaphore: VkSemaphore,
  value: *mut u64,
) -> VkResult;
/// Khronos: [vkGetSemaphoreCounterValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html) (nullable)
pub type PFN_vkGetSemaphoreCounterValue = Option<vkGetSemaphoreCounterValue_t>;
const vkGetSemaphoreCounterValue_NAME: &str = "vkGetSemaphoreCounterValue\0";

/// Khronos: [vkGetSemaphoreFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html) (non-nullable)
pub type vkGetSemaphoreFdKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  get_fd_info: *const VkSemaphoreGetFdInfoKHR,
  fd: *mut c_int,
) -> VkResult;
/// Khronos: [vkGetSemaphoreFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html) (nullable)
pub type PFN_vkGetSemaphoreFdKHR = Option<vkGetSemaphoreFdKHR_t>;
const vkGetSemaphoreFdKHR_NAME: &str = "vkGetSemaphoreFdKHR\0";

/// Khronos: [vkGetSemaphoreSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreSciSyncObjNV.html) (non-nullable)
pub type vkGetSemaphoreSciSyncObjNV_t = unsafe extern "system" fn(
  device: VkDevice,
  get_sci_sync_info: *const VkSemaphoreGetSciSyncInfoNV,
  handle: *mut c_void,
) -> VkResult;
/// Khronos: [vkGetSemaphoreSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreSciSyncObjNV.html) (nullable)
pub type PFN_vkGetSemaphoreSciSyncObjNV = Option<vkGetSemaphoreSciSyncObjNV_t>;
const vkGetSemaphoreSciSyncObjNV_NAME: &str = "vkGetSemaphoreSciSyncObjNV\0";

/// Khronos: [vkGetSemaphoreWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) (non-nullable)
pub type vkGetSemaphoreWin32HandleKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  get_win_32_handle_info: *const VkSemaphoreGetWin32HandleInfoKHR,
  handle: *mut HANDLE,
) -> VkResult;
/// Khronos: [vkGetSemaphoreWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) (nullable)
pub type PFN_vkGetSemaphoreWin32HandleKHR = Option<vkGetSemaphoreWin32HandleKHR_t>;
const vkGetSemaphoreWin32HandleKHR_NAME: &str = "vkGetSemaphoreWin32HandleKHR\0";

/// Khronos: [vkGetSemaphoreZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html) (non-nullable)
pub type vkGetSemaphoreZirconHandleFUCHSIA_t = unsafe extern "system" fn(
  device: VkDevice,
  get_zircon_handle_info: *const VkSemaphoreGetZirconHandleInfoFUCHSIA,
  zircon_handle: *mut zx_handle_t,
) -> VkResult;
/// Khronos: [vkGetSemaphoreZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html) (nullable)
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA =
  Option<vkGetSemaphoreZirconHandleFUCHSIA_t>;
const vkGetSemaphoreZirconHandleFUCHSIA_NAME: &str =
  "vkGetSemaphoreZirconHandleFUCHSIA\0";

/// Khronos: [vkGetShaderInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html) (non-nullable)
pub type vkGetShaderInfoAMD_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline: VkPipeline,
  shader_stage: VkShaderStageFlagBits,
  info_type: VkShaderInfoTypeAMD,
  info_size: *mut c_size_t,
  info: *mut c_void,
) -> VkResult;
/// Khronos: [vkGetShaderInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html) (nullable)
pub type PFN_vkGetShaderInfoAMD = Option<vkGetShaderInfoAMD_t>;
const vkGetShaderInfoAMD_NAME: &str = "vkGetShaderInfoAMD\0";

/// Khronos: [vkGetShaderModuleCreateInfoIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleCreateInfoIdentifierEXT.html) (non-nullable)
pub type vkGetShaderModuleCreateInfoIdentifierEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkShaderModuleCreateInfo,
  identifier: *mut VkShaderModuleIdentifierEXT,
);
/// Khronos: [vkGetShaderModuleCreateInfoIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleCreateInfoIdentifierEXT.html) (nullable)
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT =
  Option<vkGetShaderModuleCreateInfoIdentifierEXT_t>;
const vkGetShaderModuleCreateInfoIdentifierEXT_NAME: &str =
  "vkGetShaderModuleCreateInfoIdentifierEXT\0";

/// Khronos: [vkGetShaderModuleIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleIdentifierEXT.html) (non-nullable)
pub type vkGetShaderModuleIdentifierEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  shader_module: VkShaderModule,
  identifier: *mut VkShaderModuleIdentifierEXT,
);
/// Khronos: [vkGetShaderModuleIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleIdentifierEXT.html) (nullable)
pub type PFN_vkGetShaderModuleIdentifierEXT = Option<vkGetShaderModuleIdentifierEXT_t>;
const vkGetShaderModuleIdentifierEXT_NAME: &str = "vkGetShaderModuleIdentifierEXT\0";

/// Khronos: [vkGetSwapchainCounterEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html) (non-nullable)
pub type vkGetSwapchainCounterEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  counter: VkSurfaceCounterFlagBitsEXT,
  counter_value: *mut u64,
) -> VkResult;
/// Khronos: [vkGetSwapchainCounterEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html) (nullable)
pub type PFN_vkGetSwapchainCounterEXT = Option<vkGetSwapchainCounterEXT_t>;
const vkGetSwapchainCounterEXT_NAME: &str = "vkGetSwapchainCounterEXT\0";

/// Khronos: [vkGetSwapchainGrallocUsage2ANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainGrallocUsage2ANDROID.html) (non-nullable)
pub type vkGetSwapchainGrallocUsage2ANDROID_t = unsafe extern "system" fn(
  device: VkDevice,
  format: VkFormat,
  image_usage: VkImageUsageFlags,
  swapchain_image_usage: VkSwapchainImageUsageFlagsANDROID,
  gralloc_consumer_usage: *mut u64,
  gralloc_producer_usage: *mut u64,
) -> VkResult;
/// Khronos: [vkGetSwapchainGrallocUsage2ANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainGrallocUsage2ANDROID.html) (nullable)
pub type PFN_vkGetSwapchainGrallocUsage2ANDROID =
  Option<vkGetSwapchainGrallocUsage2ANDROID_t>;
const vkGetSwapchainGrallocUsage2ANDROID_NAME: &str =
  "vkGetSwapchainGrallocUsage2ANDROID\0";

/// Khronos: [vkGetSwapchainGrallocUsageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainGrallocUsageANDROID.html) (non-nullable)
pub type vkGetSwapchainGrallocUsageANDROID_t = unsafe extern "system" fn(
  device: VkDevice,
  format: VkFormat,
  image_usage: VkImageUsageFlags,
  gralloc_usage: *mut c_int,
) -> VkResult;
/// Khronos: [vkGetSwapchainGrallocUsageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainGrallocUsageANDROID.html) (nullable)
pub type PFN_vkGetSwapchainGrallocUsageANDROID =
  Option<vkGetSwapchainGrallocUsageANDROID_t>;
const vkGetSwapchainGrallocUsageANDROID_NAME: &str =
  "vkGetSwapchainGrallocUsageANDROID\0";

/// Khronos: [vkGetSwapchainImagesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html) (non-nullable)
pub type vkGetSwapchainImagesKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  swapchain_image_count: *mut u32,
  swapchain_images: *mut VkImage,
) -> VkResult;
/// Khronos: [vkGetSwapchainImagesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html) (nullable)
pub type PFN_vkGetSwapchainImagesKHR = Option<vkGetSwapchainImagesKHR_t>;
const vkGetSwapchainImagesKHR_NAME: &str = "vkGetSwapchainImagesKHR\0";

/// Khronos: [vkGetSwapchainStatusKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html) (non-nullable)
pub type vkGetSwapchainStatusKHR_t =
  unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
/// Khronos: [vkGetSwapchainStatusKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html) (nullable)
pub type PFN_vkGetSwapchainStatusKHR = Option<vkGetSwapchainStatusKHR_t>;
const vkGetSwapchainStatusKHR_NAME: &str = "vkGetSwapchainStatusKHR\0";

/// Khronos: [vkGetValidationCacheDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html) (non-nullable)
pub type vkGetValidationCacheDataEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  validation_cache: VkValidationCacheEXT,
  data_size: *mut c_size_t,
  data: *mut c_void,
) -> VkResult;
/// Khronos: [vkGetValidationCacheDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html) (nullable)
pub type PFN_vkGetValidationCacheDataEXT = Option<vkGetValidationCacheDataEXT_t>;
const vkGetValidationCacheDataEXT_NAME: &str = "vkGetValidationCacheDataEXT\0";

/// Khronos: [vkGetVideoSessionMemoryRequirementsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html) (non-nullable)
pub type vkGetVideoSessionMemoryRequirementsKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  video_session: VkVideoSessionKHR,
  memory_requirements_count: *mut u32,
  memory_requirements: *mut VkVideoSessionMemoryRequirementsKHR,
)
  -> VkResult;
/// Khronos: [vkGetVideoSessionMemoryRequirementsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html) (nullable)
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR =
  Option<vkGetVideoSessionMemoryRequirementsKHR_t>;
const vkGetVideoSessionMemoryRequirementsKHR_NAME: &str =
  "vkGetVideoSessionMemoryRequirementsKHR\0";

/// Khronos: [vkGetWinrtDisplayNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html) (non-nullable)
pub type vkGetWinrtDisplayNV_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  device_relative_id: u32,
  display: *mut VkDisplayKHR,
) -> VkResult;
/// Khronos: [vkGetWinrtDisplayNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html) (nullable)
pub type PFN_vkGetWinrtDisplayNV = Option<vkGetWinrtDisplayNV_t>;
const vkGetWinrtDisplayNV_NAME: &str = "vkGetWinrtDisplayNV\0";

/// Khronos: [vkImportFenceFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html) (non-nullable)
pub type vkImportFenceFdKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  import_fence_fd_info: *const VkImportFenceFdInfoKHR,
) -> VkResult;
/// Khronos: [vkImportFenceFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html) (nullable)
pub type PFN_vkImportFenceFdKHR = Option<vkImportFenceFdKHR_t>;
const vkImportFenceFdKHR_NAME: &str = "vkImportFenceFdKHR\0";

/// Khronos: [vkImportFenceSciSyncFenceNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceSciSyncFenceNV.html) (non-nullable)
pub type vkImportFenceSciSyncFenceNV_t = unsafe extern "system" fn(
  device: VkDevice,
  import_fence_sci_sync_info: *const VkImportFenceSciSyncInfoNV,
) -> VkResult;
/// Khronos: [vkImportFenceSciSyncFenceNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceSciSyncFenceNV.html) (nullable)
pub type PFN_vkImportFenceSciSyncFenceNV = Option<vkImportFenceSciSyncFenceNV_t>;
const vkImportFenceSciSyncFenceNV_NAME: &str = "vkImportFenceSciSyncFenceNV\0";

/// Khronos: [vkImportFenceSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceSciSyncObjNV.html) (non-nullable)
pub type vkImportFenceSciSyncObjNV_t = unsafe extern "system" fn(
  device: VkDevice,
  import_fence_sci_sync_info: *const VkImportFenceSciSyncInfoNV,
) -> VkResult;
/// Khronos: [vkImportFenceSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceSciSyncObjNV.html) (nullable)
pub type PFN_vkImportFenceSciSyncObjNV = Option<vkImportFenceSciSyncObjNV_t>;
const vkImportFenceSciSyncObjNV_NAME: &str = "vkImportFenceSciSyncObjNV\0";

/// Khronos: [vkImportFenceWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html) (non-nullable)
pub type vkImportFenceWin32HandleKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  import_fence_win_32_handle_info: *const VkImportFenceWin32HandleInfoKHR,
) -> VkResult;
/// Khronos: [vkImportFenceWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html) (nullable)
pub type PFN_vkImportFenceWin32HandleKHR = Option<vkImportFenceWin32HandleKHR_t>;
const vkImportFenceWin32HandleKHR_NAME: &str = "vkImportFenceWin32HandleKHR\0";

/// Khronos: [vkImportSemaphoreFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html) (non-nullable)
pub type vkImportSemaphoreFdKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  import_semaphore_fd_info: *const VkImportSemaphoreFdInfoKHR,
) -> VkResult;
/// Khronos: [vkImportSemaphoreFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html) (nullable)
pub type PFN_vkImportSemaphoreFdKHR = Option<vkImportSemaphoreFdKHR_t>;
const vkImportSemaphoreFdKHR_NAME: &str = "vkImportSemaphoreFdKHR\0";

/// Khronos: [vkImportSemaphoreSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreSciSyncObjNV.html) (non-nullable)
pub type vkImportSemaphoreSciSyncObjNV_t = unsafe extern "system" fn(
  device: VkDevice,
  import_semaphore_sci_sync_info: *const VkImportSemaphoreSciSyncInfoNV,
) -> VkResult;
/// Khronos: [vkImportSemaphoreSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreSciSyncObjNV.html) (nullable)
pub type PFN_vkImportSemaphoreSciSyncObjNV = Option<vkImportSemaphoreSciSyncObjNV_t>;
const vkImportSemaphoreSciSyncObjNV_NAME: &str = "vkImportSemaphoreSciSyncObjNV\0";

/// Khronos: [vkImportSemaphoreWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) (non-nullable)
pub type vkImportSemaphoreWin32HandleKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  import_semaphore_win_32_handle_info: *const VkImportSemaphoreWin32HandleInfoKHR,
) -> VkResult;
/// Khronos: [vkImportSemaphoreWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) (nullable)
pub type PFN_vkImportSemaphoreWin32HandleKHR = Option<vkImportSemaphoreWin32HandleKHR_t>;
const vkImportSemaphoreWin32HandleKHR_NAME: &str = "vkImportSemaphoreWin32HandleKHR\0";

/// Khronos: [vkImportSemaphoreZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html) (non-nullable)
pub type vkImportSemaphoreZirconHandleFUCHSIA_t = unsafe extern "system" fn(
  device: VkDevice,
  import_semaphore_zircon_handle_info: *const VkImportSemaphoreZirconHandleInfoFUCHSIA,
) -> VkResult;
/// Khronos: [vkImportSemaphoreZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html) (nullable)
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA =
  Option<vkImportSemaphoreZirconHandleFUCHSIA_t>;
const vkImportSemaphoreZirconHandleFUCHSIA_NAME: &str =
  "vkImportSemaphoreZirconHandleFUCHSIA\0";

/// Khronos: [vkInitializePerformanceApiINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html) (non-nullable)
pub type vkInitializePerformanceApiINTEL_t = unsafe extern "system" fn(
  device: VkDevice,
  initialize_info: *const VkInitializePerformanceApiInfoINTEL,
) -> VkResult;
/// Khronos: [vkInitializePerformanceApiINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html) (nullable)
pub type PFN_vkInitializePerformanceApiINTEL = Option<vkInitializePerformanceApiINTEL_t>;
const vkInitializePerformanceApiINTEL_NAME: &str = "vkInitializePerformanceApiINTEL\0";

/// Khronos: [vkInvalidateMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html) (non-nullable)
pub type vkInvalidateMappedMemoryRanges_t = unsafe extern "system" fn(
  device: VkDevice,
  memory_range_count: u32,
  memory_ranges: *const VkMappedMemoryRange,
) -> VkResult;
/// Khronos: [vkInvalidateMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html) (nullable)
pub type PFN_vkInvalidateMappedMemoryRanges = Option<vkInvalidateMappedMemoryRanges_t>;
const vkInvalidateMappedMemoryRanges_NAME: &str = "vkInvalidateMappedMemoryRanges\0";

/// Khronos: [vkMapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html) (non-nullable)
pub type vkMapMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
  offset: VkDeviceSize,
  size: VkDeviceSize,
  flags: VkMemoryMapFlags,
  data: *mut *mut c_void,
) -> VkResult;
/// Khronos: [vkMapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html) (nullable)
pub type PFN_vkMapMemory = Option<vkMapMemory_t>;
const vkMapMemory_NAME: &str = "vkMapMemory\0";

/// Khronos: [vkMergePipelineCaches](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html) (non-nullable)
pub type vkMergePipelineCaches_t = unsafe extern "system" fn(
  device: VkDevice,
  dst_cache: VkPipelineCache,
  src_cache_count: u32,
  src_caches: *const VkPipelineCache,
) -> VkResult;
/// Khronos: [vkMergePipelineCaches](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html) (nullable)
pub type PFN_vkMergePipelineCaches = Option<vkMergePipelineCaches_t>;
const vkMergePipelineCaches_NAME: &str = "vkMergePipelineCaches\0";

/// Khronos: [vkMergeValidationCachesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html) (non-nullable)
pub type vkMergeValidationCachesEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  dst_cache: VkValidationCacheEXT,
  src_cache_count: u32,
  src_caches: *const VkValidationCacheEXT,
) -> VkResult;
/// Khronos: [vkMergeValidationCachesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html) (nullable)
pub type PFN_vkMergeValidationCachesEXT = Option<vkMergeValidationCachesEXT_t>;
const vkMergeValidationCachesEXT_NAME: &str = "vkMergeValidationCachesEXT\0";

/// Khronos: [vkQueueBeginDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html) (non-nullable)
pub type vkQueueBeginDebugUtilsLabelEXT_t =
  unsafe extern "system" fn(queue: VkQueue, label_info: *const VkDebugUtilsLabelEXT);
/// Khronos: [vkQueueBeginDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html) (nullable)
pub type PFN_vkQueueBeginDebugUtilsLabelEXT = Option<vkQueueBeginDebugUtilsLabelEXT_t>;
const vkQueueBeginDebugUtilsLabelEXT_NAME: &str = "vkQueueBeginDebugUtilsLabelEXT\0";

/// Khronos: [vkQueueBindSparse](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html) (non-nullable)
pub type vkQueueBindSparse_t = unsafe extern "system" fn(
  queue: VkQueue,
  bind_info_count: u32,
  bind_info: *const VkBindSparseInfo,
  fence: VkFence,
) -> VkResult;
/// Khronos: [vkQueueBindSparse](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html) (nullable)
pub type PFN_vkQueueBindSparse = Option<vkQueueBindSparse_t>;
const vkQueueBindSparse_NAME: &str = "vkQueueBindSparse\0";

/// Khronos: [vkQueueEndDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) (non-nullable)
pub type vkQueueEndDebugUtilsLabelEXT_t = unsafe extern "system" fn(queue: VkQueue);
/// Khronos: [vkQueueEndDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) (nullable)
pub type PFN_vkQueueEndDebugUtilsLabelEXT = Option<vkQueueEndDebugUtilsLabelEXT_t>;
const vkQueueEndDebugUtilsLabelEXT_NAME: &str = "vkQueueEndDebugUtilsLabelEXT\0";

/// Khronos: [vkQueueInsertDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) (non-nullable)
pub type vkQueueInsertDebugUtilsLabelEXT_t =
  unsafe extern "system" fn(queue: VkQueue, label_info: *const VkDebugUtilsLabelEXT);
/// Khronos: [vkQueueInsertDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) (nullable)
pub type PFN_vkQueueInsertDebugUtilsLabelEXT = Option<vkQueueInsertDebugUtilsLabelEXT_t>;
const vkQueueInsertDebugUtilsLabelEXT_NAME: &str = "vkQueueInsertDebugUtilsLabelEXT\0";

/// Khronos: [vkQueuePresentKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html) (non-nullable)
pub type vkQueuePresentKHR_t = unsafe extern "system" fn(
  queue: VkQueue,
  present_info: *const VkPresentInfoKHR,
) -> VkResult;
/// Khronos: [vkQueuePresentKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html) (nullable)
pub type PFN_vkQueuePresentKHR = Option<vkQueuePresentKHR_t>;
const vkQueuePresentKHR_NAME: &str = "vkQueuePresentKHR\0";

/// Khronos: [vkQueueSetPerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) (non-nullable)
pub type vkQueueSetPerformanceConfigurationINTEL_t =
  unsafe extern "system" fn(
    queue: VkQueue,
    configuration: VkPerformanceConfigurationINTEL,
  ) -> VkResult;
/// Khronos: [vkQueueSetPerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) (nullable)
pub type PFN_vkQueueSetPerformanceConfigurationINTEL =
  Option<vkQueueSetPerformanceConfigurationINTEL_t>;
const vkQueueSetPerformanceConfigurationINTEL_NAME: &str =
  "vkQueueSetPerformanceConfigurationINTEL\0";

/// Khronos: [vkQueueSignalReleaseImageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSignalReleaseImageANDROID.html) (non-nullable)
pub type vkQueueSignalReleaseImageANDROID_t = unsafe extern "system" fn(
  queue: VkQueue,
  wait_semaphore_count: u32,
  wait_semaphores: *const VkSemaphore,
  image: VkImage,
  native_fence_fd: *mut c_int,
) -> VkResult;
/// Khronos: [vkQueueSignalReleaseImageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSignalReleaseImageANDROID.html) (nullable)
pub type PFN_vkQueueSignalReleaseImageANDROID =
  Option<vkQueueSignalReleaseImageANDROID_t>;
const vkQueueSignalReleaseImageANDROID_NAME: &str = "vkQueueSignalReleaseImageANDROID\0";

/// Khronos: [vkQueueSubmit](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html) (non-nullable)
pub type vkQueueSubmit_t = unsafe extern "system" fn(
  queue: VkQueue,
  submit_count: u32,
  submits: *const VkSubmitInfo,
  fence: VkFence,
) -> VkResult;
/// Khronos: [vkQueueSubmit](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html) (nullable)
pub type PFN_vkQueueSubmit = Option<vkQueueSubmit_t>;
const vkQueueSubmit_NAME: &str = "vkQueueSubmit\0";

/// Khronos: [vkQueueSubmit2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html) (non-nullable)
pub type vkQueueSubmit2_t = unsafe extern "system" fn(
  queue: VkQueue,
  submit_count: u32,
  submits: *const VkSubmitInfo2,
  fence: VkFence,
) -> VkResult;
/// Khronos: [vkQueueSubmit2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html) (nullable)
pub type PFN_vkQueueSubmit2 = Option<vkQueueSubmit2_t>;
const vkQueueSubmit2_NAME: &str = "vkQueueSubmit2\0";

/// Khronos: [vkQueueWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html) (non-nullable)
pub type vkQueueWaitIdle_t = unsafe extern "system" fn(queue: VkQueue) -> VkResult;
/// Khronos: [vkQueueWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html) (nullable)
pub type PFN_vkQueueWaitIdle = Option<vkQueueWaitIdle_t>;
const vkQueueWaitIdle_NAME: &str = "vkQueueWaitIdle\0";

/// Khronos: [vkRegisterDeviceEventEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkRegisterDeviceEventEXT.html) (non-nullable)
pub type vkRegisterDeviceEventEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  device_event_info: *const VkDeviceEventInfoEXT,
  allocator: *const VkAllocationCallbacks,
  fence: *mut VkFence,
) -> VkResult;
/// Khronos: [vkRegisterDeviceEventEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkRegisterDeviceEventEXT.html) (nullable)
pub type PFN_vkRegisterDeviceEventEXT = Option<vkRegisterDeviceEventEXT_t>;
const vkRegisterDeviceEventEXT_NAME: &str = "vkRegisterDeviceEventEXT\0";

/// Khronos: [vkRegisterDisplayEventEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html) (non-nullable)
pub type vkRegisterDisplayEventEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  display: VkDisplayKHR,
  display_event_info: *const VkDisplayEventInfoEXT,
  allocator: *const VkAllocationCallbacks,
  fence: *mut VkFence,
) -> VkResult;
/// Khronos: [vkRegisterDisplayEventEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html) (nullable)
pub type PFN_vkRegisterDisplayEventEXT = Option<vkRegisterDisplayEventEXT_t>;
const vkRegisterDisplayEventEXT_NAME: &str = "vkRegisterDisplayEventEXT\0";

/// Khronos: [vkReleaseDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html) (non-nullable)
pub type vkReleaseDisplayEXT_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  display: VkDisplayKHR,
) -> VkResult;
/// Khronos: [vkReleaseDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html) (nullable)
pub type PFN_vkReleaseDisplayEXT = Option<vkReleaseDisplayEXT_t>;
const vkReleaseDisplayEXT_NAME: &str = "vkReleaseDisplayEXT\0";

/// Khronos: [vkReleaseFullScreenExclusiveModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) (non-nullable)
pub type vkReleaseFullScreenExclusiveModeEXT_t =
  unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
/// Khronos: [vkReleaseFullScreenExclusiveModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) (nullable)
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
  Option<vkReleaseFullScreenExclusiveModeEXT_t>;
const vkReleaseFullScreenExclusiveModeEXT_NAME: &str =
  "vkReleaseFullScreenExclusiveModeEXT\0";

/// Khronos: [vkReleasePerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) (non-nullable)
pub type vkReleasePerformanceConfigurationINTEL_t = unsafe extern "system" fn(
  device: VkDevice,
  configuration: VkPerformanceConfigurationINTEL,
)
  -> VkResult;
/// Khronos: [vkReleasePerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) (nullable)
pub type PFN_vkReleasePerformanceConfigurationINTEL =
  Option<vkReleasePerformanceConfigurationINTEL_t>;
const vkReleasePerformanceConfigurationINTEL_NAME: &str =
  "vkReleasePerformanceConfigurationINTEL\0";

/// Khronos: [vkReleaseProfilingLockKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html) (non-nullable)
pub type vkReleaseProfilingLockKHR_t = unsafe extern "system" fn(device: VkDevice);
/// Khronos: [vkReleaseProfilingLockKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html) (nullable)
pub type PFN_vkReleaseProfilingLockKHR = Option<vkReleaseProfilingLockKHR_t>;
const vkReleaseProfilingLockKHR_NAME: &str = "vkReleaseProfilingLockKHR\0";

/// Khronos: [vkReleaseSwapchainImagesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseSwapchainImagesEXT.html) (non-nullable)
pub type vkReleaseSwapchainImagesEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  release_info: *const VkReleaseSwapchainImagesInfoEXT,
) -> VkResult;
/// Khronos: [vkReleaseSwapchainImagesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseSwapchainImagesEXT.html) (nullable)
pub type PFN_vkReleaseSwapchainImagesEXT = Option<vkReleaseSwapchainImagesEXT_t>;
const vkReleaseSwapchainImagesEXT_NAME: &str = "vkReleaseSwapchainImagesEXT\0";

/// Khronos: [vkResetCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html) (non-nullable)
pub type vkResetCommandBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  flags: VkCommandBufferResetFlags,
) -> VkResult;
/// Khronos: [vkResetCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html) (nullable)
pub type PFN_vkResetCommandBuffer = Option<vkResetCommandBuffer_t>;
const vkResetCommandBuffer_NAME: &str = "vkResetCommandBuffer\0";

/// Khronos: [vkResetCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html) (non-nullable)
pub type vkResetCommandPool_t = unsafe extern "system" fn(
  device: VkDevice,
  command_pool: VkCommandPool,
  flags: VkCommandPoolResetFlags,
) -> VkResult;
/// Khronos: [vkResetCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html) (nullable)
pub type PFN_vkResetCommandPool = Option<vkResetCommandPool_t>;
const vkResetCommandPool_NAME: &str = "vkResetCommandPool\0";

/// Khronos: [vkResetDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html) (non-nullable)
pub type vkResetDescriptorPool_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_pool: VkDescriptorPool,
  flags: VkDescriptorPoolResetFlags,
) -> VkResult;
/// Khronos: [vkResetDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html) (nullable)
pub type PFN_vkResetDescriptorPool = Option<vkResetDescriptorPool_t>;
const vkResetDescriptorPool_NAME: &str = "vkResetDescriptorPool\0";

/// Khronos: [vkResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html) (non-nullable)
pub type vkResetEvent_t =
  unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
/// Khronos: [vkResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html) (nullable)
pub type PFN_vkResetEvent = Option<vkResetEvent_t>;
const vkResetEvent_NAME: &str = "vkResetEvent\0";

/// Khronos: [vkResetFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetFences.html) (non-nullable)
pub type vkResetFences_t = unsafe extern "system" fn(
  device: VkDevice,
  fence_count: u32,
  fences: *const VkFence,
) -> VkResult;
/// Khronos: [vkResetFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetFences.html) (nullable)
pub type PFN_vkResetFences = Option<vkResetFences_t>;
const vkResetFences_NAME: &str = "vkResetFences\0";

/// Khronos: [vkResetQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html) (non-nullable)
pub type vkResetQueryPool_t = unsafe extern "system" fn(
  device: VkDevice,
  query_pool: VkQueryPool,
  first_query: u32,
  query_count: u32,
);
/// Khronos: [vkResetQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html) (nullable)
pub type PFN_vkResetQueryPool = Option<vkResetQueryPool_t>;
const vkResetQueryPool_NAME: &str = "vkResetQueryPool\0";

/// Khronos: [vkSetBufferCollectionBufferConstraintsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html) (non-nullable)
pub type vkSetBufferCollectionBufferConstraintsFUCHSIA_t =
  unsafe extern "system" fn(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    buffer_constraints_info: *const VkBufferConstraintsInfoFUCHSIA,
  ) -> VkResult;
/// Khronos: [vkSetBufferCollectionBufferConstraintsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html) (nullable)
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA =
  Option<vkSetBufferCollectionBufferConstraintsFUCHSIA_t>;
const vkSetBufferCollectionBufferConstraintsFUCHSIA_NAME: &str =
  "vkSetBufferCollectionBufferConstraintsFUCHSIA\0";

/// Khronos: [vkSetBufferCollectionImageConstraintsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html) (non-nullable)
pub type vkSetBufferCollectionImageConstraintsFUCHSIA_t =
  unsafe extern "system" fn(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    image_constraints_info: *const VkImageConstraintsInfoFUCHSIA,
  ) -> VkResult;
/// Khronos: [vkSetBufferCollectionImageConstraintsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html) (nullable)
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA =
  Option<vkSetBufferCollectionImageConstraintsFUCHSIA_t>;
const vkSetBufferCollectionImageConstraintsFUCHSIA_NAME: &str =
  "vkSetBufferCollectionImageConstraintsFUCHSIA\0";

/// Khronos: [vkSetDebugUtilsObjectNameEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) (non-nullable)
pub type vkSetDebugUtilsObjectNameEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  name_info: *const VkDebugUtilsObjectNameInfoEXT,
) -> VkResult;
/// Khronos: [vkSetDebugUtilsObjectNameEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) (nullable)
pub type PFN_vkSetDebugUtilsObjectNameEXT = Option<vkSetDebugUtilsObjectNameEXT_t>;
const vkSetDebugUtilsObjectNameEXT_NAME: &str = "vkSetDebugUtilsObjectNameEXT\0";

/// Khronos: [vkSetDebugUtilsObjectTagEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) (non-nullable)
pub type vkSetDebugUtilsObjectTagEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  tag_info: *const VkDebugUtilsObjectTagInfoEXT,
) -> VkResult;
/// Khronos: [vkSetDebugUtilsObjectTagEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) (nullable)
pub type PFN_vkSetDebugUtilsObjectTagEXT = Option<vkSetDebugUtilsObjectTagEXT_t>;
const vkSetDebugUtilsObjectTagEXT_NAME: &str = "vkSetDebugUtilsObjectTagEXT\0";

/// Khronos: [vkSetDeviceMemoryPriorityEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html) (non-nullable)
pub type vkSetDeviceMemoryPriorityEXT_t =
  unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, priority: c_float);
/// Khronos: [vkSetDeviceMemoryPriorityEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html) (nullable)
pub type PFN_vkSetDeviceMemoryPriorityEXT = Option<vkSetDeviceMemoryPriorityEXT_t>;
const vkSetDeviceMemoryPriorityEXT_NAME: &str = "vkSetDeviceMemoryPriorityEXT\0";

/// Khronos: [vkSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html) (non-nullable)
pub type vkSetEvent_t =
  unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
/// Khronos: [vkSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html) (nullable)
pub type PFN_vkSetEvent = Option<vkSetEvent_t>;
const vkSetEvent_NAME: &str = "vkSetEvent\0";

/// Khronos: [vkSetHdrMetadataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html) (non-nullable)
pub type vkSetHdrMetadataEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain_count: u32,
  swapchains: *const VkSwapchainKHR,
  metadata: *const VkHdrMetadataEXT,
);
/// Khronos: [vkSetHdrMetadataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html) (nullable)
pub type PFN_vkSetHdrMetadataEXT = Option<vkSetHdrMetadataEXT_t>;
const vkSetHdrMetadataEXT_NAME: &str = "vkSetHdrMetadataEXT\0";

/// Khronos: [vkSetLocalDimmingAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html) (non-nullable)
pub type vkSetLocalDimmingAMD_t = unsafe extern "system" fn(
  device: VkDevice,
  swap_chain: VkSwapchainKHR,
  local_dimming_enable: VkBool32,
);
/// Khronos: [vkSetLocalDimmingAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html) (nullable)
pub type PFN_vkSetLocalDimmingAMD = Option<vkSetLocalDimmingAMD_t>;
const vkSetLocalDimmingAMD_NAME: &str = "vkSetLocalDimmingAMD\0";

/// Khronos: [vkSetPrivateData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html) (non-nullable)
pub type vkSetPrivateData_t = unsafe extern "system" fn(
  device: VkDevice,
  object_type: VkObjectType,
  object_handle: u64,
  private_data_slot: VkPrivateDataSlot,
  data: u64,
) -> VkResult;
/// Khronos: [vkSetPrivateData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html) (nullable)
pub type PFN_vkSetPrivateData = Option<vkSetPrivateData_t>;
const vkSetPrivateData_NAME: &str = "vkSetPrivateData\0";

/// Khronos: [vkSignalSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html) (non-nullable)
pub type vkSignalSemaphore_t = unsafe extern "system" fn(
  device: VkDevice,
  signal_info: *const VkSemaphoreSignalInfo,
) -> VkResult;
/// Khronos: [vkSignalSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html) (nullable)
pub type PFN_vkSignalSemaphore = Option<vkSignalSemaphore_t>;
const vkSignalSemaphore_NAME: &str = "vkSignalSemaphore\0";

/// Khronos: [vkSubmitDebugUtilsMessageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) (non-nullable)
pub type vkSubmitDebugUtilsMessageEXT_t = unsafe extern "system" fn(
  instance: VkInstance,
  message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
  message_types: VkDebugUtilsMessageTypeFlagsEXT,
  callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
);
/// Khronos: [vkSubmitDebugUtilsMessageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) (nullable)
pub type PFN_vkSubmitDebugUtilsMessageEXT = Option<vkSubmitDebugUtilsMessageEXT_t>;
const vkSubmitDebugUtilsMessageEXT_NAME: &str = "vkSubmitDebugUtilsMessageEXT\0";

/// Khronos: [vkTrimCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html) (non-nullable)
pub type vkTrimCommandPool_t = unsafe extern "system" fn(
  device: VkDevice,
  command_pool: VkCommandPool,
  flags: VkCommandPoolTrimFlags,
);
/// Khronos: [vkTrimCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html) (nullable)
pub type PFN_vkTrimCommandPool = Option<vkTrimCommandPool_t>;
const vkTrimCommandPool_NAME: &str = "vkTrimCommandPool\0";

/// Khronos: [vkUninitializePerformanceApiINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html) (non-nullable)
pub type vkUninitializePerformanceApiINTEL_t =
  unsafe extern "system" fn(device: VkDevice);
/// Khronos: [vkUninitializePerformanceApiINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html) (nullable)
pub type PFN_vkUninitializePerformanceApiINTEL =
  Option<vkUninitializePerformanceApiINTEL_t>;
const vkUninitializePerformanceApiINTEL_NAME: &str =
  "vkUninitializePerformanceApiINTEL\0";

/// Khronos: [vkUnmapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html) (non-nullable)
pub type vkUnmapMemory_t =
  unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory);
/// Khronos: [vkUnmapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html) (nullable)
pub type PFN_vkUnmapMemory = Option<vkUnmapMemory_t>;
const vkUnmapMemory_NAME: &str = "vkUnmapMemory\0";

/// Khronos: [vkUpdateDescriptorSetWithTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html) (non-nullable)
pub type vkUpdateDescriptorSetWithTemplate_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_set: VkDescriptorSet,
  descriptor_update_template: VkDescriptorUpdateTemplate,
  data: *const c_void,
);
/// Khronos: [vkUpdateDescriptorSetWithTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html) (nullable)
pub type PFN_vkUpdateDescriptorSetWithTemplate =
  Option<vkUpdateDescriptorSetWithTemplate_t>;
const vkUpdateDescriptorSetWithTemplate_NAME: &str =
  "vkUpdateDescriptorSetWithTemplate\0";

/// Khronos: [vkUpdateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html) (non-nullable)
pub type vkUpdateDescriptorSets_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_write_count: u32,
  descriptor_writes: *const VkWriteDescriptorSet,
  descriptor_copy_count: u32,
  descriptor_copies: *const VkCopyDescriptorSet,
);
/// Khronos: [vkUpdateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html) (nullable)
pub type PFN_vkUpdateDescriptorSets = Option<vkUpdateDescriptorSets_t>;
const vkUpdateDescriptorSets_NAME: &str = "vkUpdateDescriptorSets\0";

/// Khronos: [vkUpdateVideoSessionParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html) (non-nullable)
pub type vkUpdateVideoSessionParametersKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  video_session_parameters: VkVideoSessionParametersKHR,
  update_info: *const VkVideoSessionParametersUpdateInfoKHR,
) -> VkResult;
/// Khronos: [vkUpdateVideoSessionParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html) (nullable)
pub type PFN_vkUpdateVideoSessionParametersKHR =
  Option<vkUpdateVideoSessionParametersKHR_t>;
const vkUpdateVideoSessionParametersKHR_NAME: &str =
  "vkUpdateVideoSessionParametersKHR\0";

/// Khronos: [vkWaitForFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html) (non-nullable)
pub type vkWaitForFences_t = unsafe extern "system" fn(
  device: VkDevice,
  fence_count: u32,
  fences: *const VkFence,
  wait_all: VkBool32,
  timeout: u64,
) -> VkResult;
/// Khronos: [vkWaitForFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html) (nullable)
pub type PFN_vkWaitForFences = Option<vkWaitForFences_t>;
const vkWaitForFences_NAME: &str = "vkWaitForFences\0";

/// Khronos: [vkWaitForPresentKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html) (non-nullable)
pub type vkWaitForPresentKHR_t = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  present_id: u64,
  timeout: u64,
) -> VkResult;
/// Khronos: [vkWaitForPresentKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html) (nullable)
pub type PFN_vkWaitForPresentKHR = Option<vkWaitForPresentKHR_t>;
const vkWaitForPresentKHR_NAME: &str = "vkWaitForPresentKHR\0";

/// Khronos: [vkWaitSemaphores](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html) (non-nullable)
pub type vkWaitSemaphores_t = unsafe extern "system" fn(
  device: VkDevice,
  wait_info: *const VkSemaphoreWaitInfo,
  timeout: u64,
) -> VkResult;
/// Khronos: [vkWaitSemaphores](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html) (nullable)
pub type PFN_vkWaitSemaphores = Option<vkWaitSemaphores_t>;
const vkWaitSemaphores_NAME: &str = "vkWaitSemaphores\0";

/// Khronos: [vkWriteAccelerationStructuresPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html) (non-nullable)
pub type vkWriteAccelerationStructuresPropertiesKHR_t =
  unsafe extern "system" fn(
    device: VkDevice,
    acceleration_structure_count: u32,
    acceleration_structures: *const VkAccelerationStructureKHR,
    query_type: VkQueryType,
    data_size: c_size_t,
    data: *mut c_void,
    stride: c_size_t,
  ) -> VkResult;
/// Khronos: [vkWriteAccelerationStructuresPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html) (nullable)
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR =
  Option<vkWriteAccelerationStructuresPropertiesKHR_t>;
const vkWriteAccelerationStructuresPropertiesKHR_NAME: &str =
  "vkWriteAccelerationStructuresPropertiesKHR\0";

/// Khronos: [vkWriteMicromapsPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWriteMicromapsPropertiesEXT.html) (non-nullable)
pub type vkWriteMicromapsPropertiesEXT_t = unsafe extern "system" fn(
  device: VkDevice,
  micromap_count: u32,
  micromaps: *const VkMicromapEXT,
  query_type: VkQueryType,
  data_size: c_size_t,
  data: *mut c_void,
  stride: c_size_t,
) -> VkResult;
/// Khronos: [vkWriteMicromapsPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWriteMicromapsPropertiesEXT.html) (nullable)
pub type PFN_vkWriteMicromapsPropertiesEXT = Option<vkWriteMicromapsPropertiesEXT_t>;
const vkWriteMicromapsPropertiesEXT_NAME: &str = "vkWriteMicromapsPropertiesEXT\0";

/// pointers for 89 fns.
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct InstanceFns {
  /// Khronos: [vkAcquireDrmDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INITIALIZATION_FAILED`
  /// * `physical_device`
  /// * `drm_fd`
  /// * `display`
  pub AcquireDrmDisplayEXT: PFN_vkAcquireDrmDisplayEXT,

  /// Khronos: [vkAcquireWinrtDisplayNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_DEVICE_LOST`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`
  /// * `physical_device`
  /// * `display`
  pub AcquireWinrtDisplayNV: PFN_vkAcquireWinrtDisplayNV,

  /// Khronos: [vkAcquireXlibDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INITIALIZATION_FAILED`
  /// * `physical_device`
  /// * `dpy`
  /// * `display`
  pub AcquireXlibDisplayEXT: PFN_vkAcquireXlibDisplayEXT,

  /// Khronos: [vkCreateAndroidSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateAndroidSurfaceKHR: PFN_vkCreateAndroidSurfaceKHR,

  /// Khronos: [vkCreateDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `callback`
  pub CreateDebugReportCallbackEXT: PFN_vkCreateDebugReportCallbackEXT,

  /// Khronos: [vkCreateDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `messenger`
  pub CreateDebugUtilsMessengerEXT: PFN_vkCreateDebugUtilsMessengerEXT,

  /// Khronos: [vkCreateDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`, `VK_ERROR_EXTENSION_NOT_PRESENT`,
  ///   `VK_ERROR_FEATURE_NOT_PRESENT`, `VK_ERROR_TOO_MANY_OBJECTS`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `physical_device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `device`
  pub CreateDevice: PFN_vkCreateDevice,

  /// Khronos: [vkCreateDirectFBSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateDirectFBSurfaceEXT: PFN_vkCreateDirectFBSurfaceEXT,

  /// Khronos: [vkCreateDisplayModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`
  /// * `physical_device`
  /// * `display`, Extern Sync: true
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `mode`
  pub CreateDisplayModeKHR: PFN_vkCreateDisplayModeKHR,

  /// Khronos: [vkCreateDisplayPlaneSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateDisplayPlaneSurfaceKHR: PFN_vkCreateDisplayPlaneSurfaceKHR,

  /// Khronos: [vkCreateHeadlessSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateHeadlessSurfaceEXT: PFN_vkCreateHeadlessSurfaceEXT,

  /// Khronos: [vkCreateIOSSurfaceMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateIOSSurfaceMVK: PFN_vkCreateIOSSurfaceMVK,

  /// Khronos: [vkCreateImagePipeSurfaceFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateImagePipeSurfaceFUCHSIA: PFN_vkCreateImagePipeSurfaceFUCHSIA,

  /// Khronos: [vkCreateMacOSSurfaceMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateMacOSSurfaceMVK: PFN_vkCreateMacOSSurfaceMVK,

  /// Khronos: [vkCreateMetalSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateMetalSurfaceEXT: PFN_vkCreateMetalSurfaceEXT,

  /// Khronos: [vkCreateScreenSurfaceQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateScreenSurfaceQNX: PFN_vkCreateScreenSurfaceQNX,

  /// Khronos: [vkCreateViSurfaceNN](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateViSurfaceNN: PFN_vkCreateViSurfaceNN,

  /// Khronos: [vkCreateWaylandSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateWaylandSurfaceKHR: PFN_vkCreateWaylandSurfaceKHR,

  /// Khronos: [vkCreateWin32SurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateWin32SurfaceKHR: PFN_vkCreateWin32SurfaceKHR,

  /// Khronos: [vkCreateXcbSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateXcbSurfaceKHR: PFN_vkCreateXcbSurfaceKHR,

  /// Khronos: [vkCreateXlibSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `instance`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `surface`
  pub CreateXlibSurfaceKHR: PFN_vkCreateXlibSurfaceKHR,

  /// Khronos: [vkDebugReportMessageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html)
  /// * `instance`
  /// * `flags`
  /// * `object_type`
  /// * `object`, Object Type: objectType
  /// * `location`
  /// * `message_code`
  /// * `layer_prefix`, Len: `null_terminated`
  /// * `message`, Len: `null_terminated`
  pub DebugReportMessageEXT: PFN_vkDebugReportMessageEXT,

  /// Khronos: [vkDestroyDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html)
  /// * `instance`
  /// * `callback`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyDebugReportCallbackEXT: PFN_vkDestroyDebugReportCallbackEXT,

  /// Khronos: [vkDestroyDebugUtilsMessengerEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html)
  /// * `instance`
  /// * `messenger`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyDebugUtilsMessengerEXT: PFN_vkDestroyDebugUtilsMessengerEXT,

  /// Khronos: [vkDestroyInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)
  /// * Implicit Extern Sync: all [VkPhysicalDevice] objects enumerated from
  ///   `instance`
  /// * `instance`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyInstance: PFN_vkDestroyInstance,

  /// Khronos: [vkDestroySurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html)
  /// * `instance`
  /// * `surface`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroySurfaceKHR: PFN_vkDestroySurfaceKHR,

  /// Khronos: [vkEnumerateDeviceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_LAYER_NOT_PRESENT`
  /// * `physical_device`
  /// * `layer_name`, Optional: true, Len: `null_terminated`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub EnumerateDeviceExtensionProperties: PFN_vkEnumerateDeviceExtensionProperties,

  /// Khronos: [vkEnumerateDeviceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub EnumerateDeviceLayerProperties: PFN_vkEnumerateDeviceLayerProperties,

  /// Khronos: [vkEnumeratePhysicalDeviceGroups](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`
  /// * `instance`
  /// * `physical_device_group_count`, Optional: false,true
  /// * `physical_device_group_properties`, Optional: true, Len:
  ///   `physical_device_group_count`
  pub EnumeratePhysicalDeviceGroups: PFN_vkEnumeratePhysicalDeviceGroups,

  /// Khronos: [vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`
  /// * `physical_device`
  /// * `queue_family_index`
  /// * `counter_count`, Optional: false,true
  /// * `counters`, Optional: true, Len: `counter_count`
  /// * `counter_descriptions`, Optional: true, Len: `counter_count`
  pub EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR:
    PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,

  /// Khronos: [vkEnumeratePhysicalDevices](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`
  /// * `instance`
  /// * `physical_device_count`, Optional: false,true
  /// * `physical_devices`, Optional: true, Len: `physical_device_count`
  pub EnumeratePhysicalDevices: PFN_vkEnumeratePhysicalDevices,

  /// Khronos: [vkGetDisplayModeProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `display`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub GetDisplayModeProperties2KHR: PFN_vkGetDisplayModeProperties2KHR,

  /// Khronos: [vkGetDisplayModePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `display`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub GetDisplayModePropertiesKHR: PFN_vkGetDisplayModePropertiesKHR,

  /// Khronos: [vkGetDisplayPlaneCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `display_plane_info`
  /// * `capabilities`
  pub GetDisplayPlaneCapabilities2KHR: PFN_vkGetDisplayPlaneCapabilities2KHR,

  /// Khronos: [vkGetDisplayPlaneCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `mode`, Extern Sync: true
  /// * `plane_index`
  /// * `capabilities`
  pub GetDisplayPlaneCapabilitiesKHR: PFN_vkGetDisplayPlaneCapabilitiesKHR,

  /// Khronos: [vkGetDisplayPlaneSupportedDisplaysKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `plane_index`
  /// * `display_count`, Optional: false,true
  /// * `displays`, Optional: true, Len: `display_count`
  pub GetDisplayPlaneSupportedDisplaysKHR: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,

  /// Khronos: [vkGetDrmDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INITIALIZATION_FAILED`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `physical_device`
  /// * `drm_fd`
  /// * `connector_id`
  /// * `display`
  pub GetDrmDisplayEXT: PFN_vkGetDrmDisplayEXT,

  /// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
  /// * `instance`, Optional: true
  /// * `name`, Len: `null_terminated`
  pub GetInstanceProcAddr: PFN_vkGetInstanceProcAddr,

  /// Khronos: [vkGetPhysicalDeviceCalibrateableTimeDomainsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `time_domain_count`, Optional: false,true
  /// * `time_domains`, Optional: true, Len: `time_domain_count`
  pub GetPhysicalDeviceCalibrateableTimeDomainsEXT:
    PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT,

  /// Khronos: [vkGetPhysicalDeviceCooperativeMatrixPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub GetPhysicalDeviceCooperativeMatrixPropertiesNV:
    PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,

  /// Khronos: [vkGetPhysicalDeviceDirectFBPresentationSupportEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)
  /// * `physical_device`
  /// * `queue_family_index`
  /// * `dfb`
  pub GetPhysicalDeviceDirectFBPresentationSupportEXT:
    PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,

  /// Khronos: [vkGetPhysicalDeviceDisplayPlaneProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub GetPhysicalDeviceDisplayPlaneProperties2KHR:
    PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,

  /// Khronos: [vkGetPhysicalDeviceDisplayPlanePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub GetPhysicalDeviceDisplayPlanePropertiesKHR:
    PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,

  /// Khronos: [vkGetPhysicalDeviceDisplayProperties2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub GetPhysicalDeviceDisplayProperties2KHR:
    PFN_vkGetPhysicalDeviceDisplayProperties2KHR,

  /// Khronos: [vkGetPhysicalDeviceDisplayPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub GetPhysicalDeviceDisplayPropertiesKHR: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,

  /// Khronos: [vkGetPhysicalDeviceExternalBufferProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html)
  /// * `physical_device`
  /// * `external_buffer_info`
  /// * `external_buffer_properties`
  pub GetPhysicalDeviceExternalBufferProperties:
    PFN_vkGetPhysicalDeviceExternalBufferProperties,

  /// Khronos: [vkGetPhysicalDeviceExternalFenceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html)
  /// * `physical_device`
  /// * `external_fence_info`
  /// * `external_fence_properties`
  pub GetPhysicalDeviceExternalFenceProperties:
    PFN_vkGetPhysicalDeviceExternalFenceProperties,

  /// Khronos: [vkGetPhysicalDeviceExternalImageFormatPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
  /// * `physical_device`
  /// * `format`
  /// * `ty`
  /// * `tiling`
  /// * `usage`
  /// * `flags`, Optional: true
  /// * `external_handle_type`, Optional: true
  /// * `external_image_format_properties`
  pub GetPhysicalDeviceExternalImageFormatPropertiesNV:
    PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,

  /// Khronos: [vkGetPhysicalDeviceExternalSemaphoreProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
  /// * `physical_device`
  /// * `external_semaphore_info`
  /// * `external_semaphore_properties`
  pub GetPhysicalDeviceExternalSemaphoreProperties:
    PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,

  /// Khronos: [vkGetPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
  /// * `physical_device`
  /// * `features`
  pub GetPhysicalDeviceFeatures: PFN_vkGetPhysicalDeviceFeatures,

  /// Khronos: [vkGetPhysicalDeviceFeatures2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html)
  /// * `physical_device`
  /// * `features`
  pub GetPhysicalDeviceFeatures2: PFN_vkGetPhysicalDeviceFeatures2,

  /// Khronos: [vkGetPhysicalDeviceFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
  /// * `physical_device`
  /// * `format`
  /// * `format_properties`
  pub GetPhysicalDeviceFormatProperties: PFN_vkGetPhysicalDeviceFormatProperties,

  /// Khronos: [vkGetPhysicalDeviceFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html)
  /// * `physical_device`
  /// * `format`
  /// * `format_properties`
  pub GetPhysicalDeviceFormatProperties2: PFN_vkGetPhysicalDeviceFormatProperties2,

  /// Khronos: [vkGetPhysicalDeviceFragmentShadingRatesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `physical_device`
  /// * `fragment_shading_rate_count`, Optional: false,true
  /// * `fragment_shading_rates`, Optional: true, Len:
  ///   `fragment_shading_rate_count`
  pub GetPhysicalDeviceFragmentShadingRatesKHR:
    PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,

  /// Khronos: [vkGetPhysicalDeviceImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
  /// * `physical_device`
  /// * `format`
  /// * `ty`
  /// * `tiling`
  /// * `usage`
  /// * `flags`, Optional: true
  /// * `image_format_properties`
  pub GetPhysicalDeviceImageFormatProperties:
    PFN_vkGetPhysicalDeviceImageFormatProperties,

  /// Khronos: [vkGetPhysicalDeviceImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_FORMAT_NOT_SUPPORTED`,
  ///   `VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`
  /// * `physical_device`
  /// * `image_format_info`
  /// * `image_format_properties`
  pub GetPhysicalDeviceImageFormatProperties2:
    PFN_vkGetPhysicalDeviceImageFormatProperties2,

  /// Khronos: [vkGetPhysicalDeviceMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html)
  /// * `physical_device`
  /// * `memory_properties`
  pub GetPhysicalDeviceMemoryProperties: PFN_vkGetPhysicalDeviceMemoryProperties,

  /// Khronos: [vkGetPhysicalDeviceMemoryProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html)
  /// * `physical_device`
  /// * `memory_properties`
  pub GetPhysicalDeviceMemoryProperties2: PFN_vkGetPhysicalDeviceMemoryProperties2,

  /// Khronos: [vkGetPhysicalDeviceMultisamplePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)
  /// * `physical_device`
  /// * `samples`
  /// * `multisample_properties`
  pub GetPhysicalDeviceMultisamplePropertiesEXT:
    PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,

  /// Khronos: [vkGetPhysicalDeviceOpticalFlowImageFormatsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_EXTENSION_NOT_PRESENT`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`, `VK_ERROR_FORMAT_NOT_SUPPORTED`
  /// * `physical_device`
  /// * `optical_flow_image_format_info`
  /// * `format_count`, Optional: false,true
  /// * `image_format_properties`, Optional: true, Len: `format_count`
  pub GetPhysicalDeviceOpticalFlowImageFormatsNV:
    PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,

  /// Khronos: [vkGetPhysicalDevicePresentRectanglesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `surface`, Extern Sync: true
  /// * `rect_count`, Optional: false,true
  /// * `rects`, Optional: true, Len: `rect_count`
  pub GetPhysicalDevicePresentRectanglesKHR: PFN_vkGetPhysicalDevicePresentRectanglesKHR,

  /// Khronos: [vkGetPhysicalDeviceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html)
  /// * `physical_device`
  /// * `properties`
  pub GetPhysicalDeviceProperties: PFN_vkGetPhysicalDeviceProperties,

  /// Khronos: [vkGetPhysicalDeviceProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html)
  /// * `physical_device`
  /// * `properties`
  pub GetPhysicalDeviceProperties2: PFN_vkGetPhysicalDeviceProperties2,

  /// Khronos: [vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)
  /// * `physical_device`
  /// * `performance_query_create_info`
  /// * `num_passes`
  pub GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR:
    PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,

  /// Khronos: [vkGetPhysicalDeviceQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
  /// * `physical_device`
  /// * `queue_family_property_count`, Optional: false,true
  /// * `queue_family_properties`, Optional: true, Len:
  ///   `queue_family_property_count`
  pub GetPhysicalDeviceQueueFamilyProperties:
    PFN_vkGetPhysicalDeviceQueueFamilyProperties,

  /// Khronos: [vkGetPhysicalDeviceQueueFamilyProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html)
  /// * `physical_device`
  /// * `queue_family_property_count`, Optional: false,true
  /// * `queue_family_properties`, Optional: true, Len:
  ///   `queue_family_property_count`
  pub GetPhysicalDeviceQueueFamilyProperties2:
    PFN_vkGetPhysicalDeviceQueueFamilyProperties2,

  /// Khronos: [vkGetPhysicalDeviceRefreshableObjectTypesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceRefreshableObjectTypesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * `physical_device`
  /// * `refreshable_object_type_count`, Optional: false,true
  /// * `refreshable_object_types`, Optional: true, Len:
  ///   `refreshable_object_type_count`
  pub GetPhysicalDeviceRefreshableObjectTypesKHR:
    PFN_vkGetPhysicalDeviceRefreshableObjectTypesKHR,

  /// Khronos: [vkGetPhysicalDeviceScreenPresentationSupportQNX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html)
  /// * `physical_device`
  /// * `queue_family_index`
  /// * `window`
  pub GetPhysicalDeviceScreenPresentationSupportQNX:
    PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,

  /// Khronos: [vkGetPhysicalDeviceSparseImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html)
  /// * `physical_device`
  /// * `format`
  /// * `ty`
  /// * `samples`
  /// * `usage`
  /// * `tiling`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub GetPhysicalDeviceSparseImageFormatProperties:
    PFN_vkGetPhysicalDeviceSparseImageFormatProperties,

  /// Khronos: [vkGetPhysicalDeviceSparseImageFormatProperties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
  /// * `physical_device`
  /// * `format_info`
  /// * `property_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `property_count`
  pub GetPhysicalDeviceSparseImageFormatProperties2:
    PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,

  /// Khronos: [vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `physical_device`
  /// * `combination_count`, Optional: false,true
  /// * `combinations`, Optional: true, Len: `combination_count`
  pub GetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV:
    PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,

  /// Khronos: [vkGetPhysicalDeviceSurfaceCapabilities2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `physical_device`
  /// * `surface`
  /// * `surface_capabilities`
  pub GetPhysicalDeviceSurfaceCapabilities2EXT:
    PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,

  /// Khronos: [vkGetPhysicalDeviceSurfaceCapabilities2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `physical_device`
  /// * `surface_info`
  /// * `surface_capabilities`
  pub GetPhysicalDeviceSurfaceCapabilities2KHR:
    PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,

  /// Khronos: [vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `physical_device`
  /// * `surface`
  /// * `surface_capabilities`
  pub GetPhysicalDeviceSurfaceCapabilitiesKHR:
    PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,

  /// Khronos: [vkGetPhysicalDeviceSurfaceFormats2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `physical_device`
  /// * `surface_info`
  /// * `surface_format_count`, Optional: false,true
  /// * `surface_formats`, Optional: true, Len: `surface_format_count`
  pub GetPhysicalDeviceSurfaceFormats2KHR: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,

  /// Khronos: [vkGetPhysicalDeviceSurfaceFormatsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `physical_device`
  /// * `surface`, Optional: true
  /// * `surface_format_count`, Optional: false,true
  /// * `surface_formats`, Optional: true, Len: `surface_format_count`
  pub GetPhysicalDeviceSurfaceFormatsKHR: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,

  /// Khronos: [vkGetPhysicalDeviceSurfacePresentModes2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `physical_device`
  /// * `surface_info`
  /// * `present_mode_count`, Optional: false,true
  /// * `present_modes`, Optional: true, Len: `present_mode_count`
  pub GetPhysicalDeviceSurfacePresentModes2EXT:
    PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,

  /// Khronos: [vkGetPhysicalDeviceSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `physical_device`
  /// * `surface`, Optional: true
  /// * `present_mode_count`, Optional: false,true
  /// * `present_modes`, Optional: true, Len: `present_mode_count`
  pub GetPhysicalDeviceSurfacePresentModesKHR:
    PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,

  /// Khronos: [vkGetPhysicalDeviceSurfaceSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `physical_device`
  /// * `queue_family_index`
  /// * `surface`
  /// * `supported`
  pub GetPhysicalDeviceSurfaceSupportKHR: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,

  /// Khronos: [vkGetPhysicalDeviceToolProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `physical_device`
  /// * `tool_count`, Optional: false,true
  /// * `tool_properties`, Optional: true, Len: `tool_count`
  pub GetPhysicalDeviceToolProperties: PFN_vkGetPhysicalDeviceToolProperties,

  /// Khronos: [vkGetPhysicalDeviceVideoCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`
  /// * `physical_device`
  /// * `video_profile`
  /// * `capabilities`
  pub GetPhysicalDeviceVideoCapabilitiesKHR: PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,

  /// Khronos: [vkGetPhysicalDeviceVideoFormatPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`,
  ///   `VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`
  /// * `physical_device`
  /// * `video_format_info`
  /// * `video_format_property_count`, Optional: false,true
  /// * `video_format_properties`, Optional: true, Len:
  ///   `video_format_property_count`
  pub GetPhysicalDeviceVideoFormatPropertiesKHR:
    PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,

  /// Khronos: [vkGetPhysicalDeviceWaylandPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
  /// * `physical_device`
  /// * `queue_family_index`
  /// * `display`
  pub GetPhysicalDeviceWaylandPresentationSupportKHR:
    PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,

  /// Khronos: [vkGetPhysicalDeviceWin32PresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
  /// * `physical_device`
  /// * `queue_family_index`
  pub GetPhysicalDeviceWin32PresentationSupportKHR:
    PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,

  /// Khronos: [vkGetPhysicalDeviceXcbPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
  /// * `physical_device`
  /// * `queue_family_index`
  /// * `connection`
  /// * `visual_id`
  pub GetPhysicalDeviceXcbPresentationSupportKHR:
    PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,

  /// Khronos: [vkGetPhysicalDeviceXlibPresentationSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
  /// * `physical_device`
  /// * `queue_family_index`
  /// * `dpy`
  /// * `visual_id`
  pub GetPhysicalDeviceXlibPresentationSupportKHR:
    PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,

  /// Khronos: [vkGetWinrtDisplayNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_DEVICE_LOST`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`
  /// * `physical_device`
  /// * `device_relative_id`
  /// * `display`
  pub GetWinrtDisplayNV: PFN_vkGetWinrtDisplayNV,

  /// Khronos: [vkReleaseDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * `physical_device`
  /// * `display`
  pub ReleaseDisplayEXT: PFN_vkReleaseDisplayEXT,

  /// Khronos: [vkSubmitDebugUtilsMessageEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html)
  /// * `instance`
  /// * `message_severity`
  /// * `message_types`
  /// * `callback_data`
  pub SubmitDebugUtilsMessageEXT: PFN_vkSubmitDebugUtilsMessageEXT,
}
impl InstanceFns {
  pub unsafe fn load(&mut self, instance: VkInstance, loader: vkGetInstanceProcAddr_t) {
    use core::mem::transmute;
    self.AcquireDrmDisplayEXT =
      transmute(loader(instance, vkAcquireDrmDisplayEXT_NAME.as_ptr()));
    self.AcquireWinrtDisplayNV =
      transmute(loader(instance, vkAcquireWinrtDisplayNV_NAME.as_ptr()));
    self.AcquireXlibDisplayEXT =
      transmute(loader(instance, vkAcquireXlibDisplayEXT_NAME.as_ptr()));
    self.CreateAndroidSurfaceKHR =
      transmute(loader(instance, vkCreateAndroidSurfaceKHR_NAME.as_ptr()));
    self.CreateDebugReportCallbackEXT =
      transmute(loader(instance, vkCreateDebugReportCallbackEXT_NAME.as_ptr()));
    self.CreateDebugUtilsMessengerEXT =
      transmute(loader(instance, vkCreateDebugUtilsMessengerEXT_NAME.as_ptr()));
    self.CreateDevice = transmute(loader(instance, vkCreateDevice_NAME.as_ptr()));
    self.CreateDirectFBSurfaceEXT =
      transmute(loader(instance, vkCreateDirectFBSurfaceEXT_NAME.as_ptr()));
    self.CreateDisplayModeKHR =
      transmute(loader(instance, vkCreateDisplayModeKHR_NAME.as_ptr()));
    self.CreateDisplayPlaneSurfaceKHR =
      transmute(loader(instance, vkCreateDisplayPlaneSurfaceKHR_NAME.as_ptr()));
    self.CreateHeadlessSurfaceEXT =
      transmute(loader(instance, vkCreateHeadlessSurfaceEXT_NAME.as_ptr()));
    self.CreateIOSSurfaceMVK =
      transmute(loader(instance, vkCreateIOSSurfaceMVK_NAME.as_ptr()));
    self.CreateImagePipeSurfaceFUCHSIA =
      transmute(loader(instance, vkCreateImagePipeSurfaceFUCHSIA_NAME.as_ptr()));
    self.CreateMacOSSurfaceMVK =
      transmute(loader(instance, vkCreateMacOSSurfaceMVK_NAME.as_ptr()));
    self.CreateMetalSurfaceEXT =
      transmute(loader(instance, vkCreateMetalSurfaceEXT_NAME.as_ptr()));
    self.CreateScreenSurfaceQNX =
      transmute(loader(instance, vkCreateScreenSurfaceQNX_NAME.as_ptr()));
    self.CreateViSurfaceNN =
      transmute(loader(instance, vkCreateViSurfaceNN_NAME.as_ptr()));
    self.CreateWaylandSurfaceKHR =
      transmute(loader(instance, vkCreateWaylandSurfaceKHR_NAME.as_ptr()));
    self.CreateWin32SurfaceKHR =
      transmute(loader(instance, vkCreateWin32SurfaceKHR_NAME.as_ptr()));
    self.CreateXcbSurfaceKHR =
      transmute(loader(instance, vkCreateXcbSurfaceKHR_NAME.as_ptr()));
    self.CreateXlibSurfaceKHR =
      transmute(loader(instance, vkCreateXlibSurfaceKHR_NAME.as_ptr()));
    self.DebugReportMessageEXT =
      transmute(loader(instance, vkDebugReportMessageEXT_NAME.as_ptr()));
    self.DestroyDebugReportCallbackEXT =
      transmute(loader(instance, vkDestroyDebugReportCallbackEXT_NAME.as_ptr()));
    self.DestroyDebugUtilsMessengerEXT =
      transmute(loader(instance, vkDestroyDebugUtilsMessengerEXT_NAME.as_ptr()));
    self.DestroyInstance = transmute(loader(instance, vkDestroyInstance_NAME.as_ptr()));
    self.DestroySurfaceKHR =
      transmute(loader(instance, vkDestroySurfaceKHR_NAME.as_ptr()));
    self.EnumerateDeviceExtensionProperties =
      transmute(loader(instance, vkEnumerateDeviceExtensionProperties_NAME.as_ptr()));
    self.EnumerateDeviceLayerProperties =
      transmute(loader(instance, vkEnumerateDeviceLayerProperties_NAME.as_ptr()));
    self.EnumeratePhysicalDeviceGroups =
      transmute(loader(instance, vkEnumeratePhysicalDeviceGroups_NAME.as_ptr()));
    self.EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
      transmute(loader(
        instance,
        vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR_NAME.as_ptr(),
      ));
    self.EnumeratePhysicalDevices =
      transmute(loader(instance, vkEnumeratePhysicalDevices_NAME.as_ptr()));
    self.GetDisplayModeProperties2KHR =
      transmute(loader(instance, vkGetDisplayModeProperties2KHR_NAME.as_ptr()));
    self.GetDisplayModePropertiesKHR =
      transmute(loader(instance, vkGetDisplayModePropertiesKHR_NAME.as_ptr()));
    self.GetDisplayPlaneCapabilities2KHR =
      transmute(loader(instance, vkGetDisplayPlaneCapabilities2KHR_NAME.as_ptr()));
    self.GetDisplayPlaneCapabilitiesKHR =
      transmute(loader(instance, vkGetDisplayPlaneCapabilitiesKHR_NAME.as_ptr()));
    self.GetDisplayPlaneSupportedDisplaysKHR =
      transmute(loader(instance, vkGetDisplayPlaneSupportedDisplaysKHR_NAME.as_ptr()));
    self.GetDrmDisplayEXT = transmute(loader(instance, vkGetDrmDisplayEXT_NAME.as_ptr()));
    self.GetInstanceProcAddr =
      transmute(loader(instance, vkGetInstanceProcAddr_NAME.as_ptr()));
    self.GetPhysicalDeviceCalibrateableTimeDomainsEXT = transmute(loader(
      instance,
      vkGetPhysicalDeviceCalibrateableTimeDomainsEXT_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceCooperativeMatrixPropertiesNV = transmute(loader(
      instance,
      vkGetPhysicalDeviceCooperativeMatrixPropertiesNV_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceDirectFBPresentationSupportEXT = transmute(loader(
      instance,
      vkGetPhysicalDeviceDirectFBPresentationSupportEXT_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceDisplayPlaneProperties2KHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceDisplayPlaneProperties2KHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceDisplayPlanePropertiesKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceDisplayPlanePropertiesKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceDisplayProperties2KHR =
      transmute(loader(instance, vkGetPhysicalDeviceDisplayProperties2KHR_NAME.as_ptr()));
    self.GetPhysicalDeviceDisplayPropertiesKHR =
      transmute(loader(instance, vkGetPhysicalDeviceDisplayPropertiesKHR_NAME.as_ptr()));
    self.GetPhysicalDeviceExternalBufferProperties = transmute(loader(
      instance,
      vkGetPhysicalDeviceExternalBufferProperties_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceExternalFenceProperties = transmute(loader(
      instance,
      vkGetPhysicalDeviceExternalFenceProperties_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceExternalImageFormatPropertiesNV = transmute(loader(
      instance,
      vkGetPhysicalDeviceExternalImageFormatPropertiesNV_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceExternalSemaphoreProperties = transmute(loader(
      instance,
      vkGetPhysicalDeviceExternalSemaphoreProperties_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceFeatures =
      transmute(loader(instance, vkGetPhysicalDeviceFeatures_NAME.as_ptr()));
    self.GetPhysicalDeviceFeatures2 =
      transmute(loader(instance, vkGetPhysicalDeviceFeatures2_NAME.as_ptr()));
    self.GetPhysicalDeviceFormatProperties =
      transmute(loader(instance, vkGetPhysicalDeviceFormatProperties_NAME.as_ptr()));
    self.GetPhysicalDeviceFormatProperties2 =
      transmute(loader(instance, vkGetPhysicalDeviceFormatProperties2_NAME.as_ptr()));
    self.GetPhysicalDeviceFragmentShadingRatesKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceFragmentShadingRatesKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceImageFormatProperties =
      transmute(loader(instance, vkGetPhysicalDeviceImageFormatProperties_NAME.as_ptr()));
    self.GetPhysicalDeviceImageFormatProperties2 = transmute(loader(
      instance,
      vkGetPhysicalDeviceImageFormatProperties2_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceMemoryProperties =
      transmute(loader(instance, vkGetPhysicalDeviceMemoryProperties_NAME.as_ptr()));
    self.GetPhysicalDeviceMemoryProperties2 =
      transmute(loader(instance, vkGetPhysicalDeviceMemoryProperties2_NAME.as_ptr()));
    self.GetPhysicalDeviceMultisamplePropertiesEXT = transmute(loader(
      instance,
      vkGetPhysicalDeviceMultisamplePropertiesEXT_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceOpticalFlowImageFormatsNV = transmute(loader(
      instance,
      vkGetPhysicalDeviceOpticalFlowImageFormatsNV_NAME.as_ptr(),
    ));
    self.GetPhysicalDevicePresentRectanglesKHR =
      transmute(loader(instance, vkGetPhysicalDevicePresentRectanglesKHR_NAME.as_ptr()));
    self.GetPhysicalDeviceProperties =
      transmute(loader(instance, vkGetPhysicalDeviceProperties_NAME.as_ptr()));
    self.GetPhysicalDeviceProperties2 =
      transmute(loader(instance, vkGetPhysicalDeviceProperties2_NAME.as_ptr()));
    self.GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceQueueFamilyProperties =
      transmute(loader(instance, vkGetPhysicalDeviceQueueFamilyProperties_NAME.as_ptr()));
    self.GetPhysicalDeviceQueueFamilyProperties2 = transmute(loader(
      instance,
      vkGetPhysicalDeviceQueueFamilyProperties2_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceRefreshableObjectTypesKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceRefreshableObjectTypesKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceScreenPresentationSupportQNX = transmute(loader(
      instance,
      vkGetPhysicalDeviceScreenPresentationSupportQNX_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceSparseImageFormatProperties = transmute(loader(
      instance,
      vkGetPhysicalDeviceSparseImageFormatProperties_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceSparseImageFormatProperties2 = transmute(loader(
      instance,
      vkGetPhysicalDeviceSparseImageFormatProperties2_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
      transmute(loader(
        instance,
        vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV_NAME.as_ptr(),
      ));
    self.GetPhysicalDeviceSurfaceCapabilities2EXT = transmute(loader(
      instance,
      vkGetPhysicalDeviceSurfaceCapabilities2EXT_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceSurfaceCapabilities2KHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceSurfaceCapabilities2KHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceSurfaceCapabilitiesKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceSurfaceCapabilitiesKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceSurfaceFormats2KHR =
      transmute(loader(instance, vkGetPhysicalDeviceSurfaceFormats2KHR_NAME.as_ptr()));
    self.GetPhysicalDeviceSurfaceFormatsKHR =
      transmute(loader(instance, vkGetPhysicalDeviceSurfaceFormatsKHR_NAME.as_ptr()));
    self.GetPhysicalDeviceSurfacePresentModes2EXT = transmute(loader(
      instance,
      vkGetPhysicalDeviceSurfacePresentModes2EXT_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceSurfacePresentModesKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceSurfacePresentModesKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceSurfaceSupportKHR =
      transmute(loader(instance, vkGetPhysicalDeviceSurfaceSupportKHR_NAME.as_ptr()));
    self.GetPhysicalDeviceToolProperties =
      transmute(loader(instance, vkGetPhysicalDeviceToolProperties_NAME.as_ptr()));
    self.GetPhysicalDeviceVideoCapabilitiesKHR =
      transmute(loader(instance, vkGetPhysicalDeviceVideoCapabilitiesKHR_NAME.as_ptr()));
    self.GetPhysicalDeviceVideoFormatPropertiesKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceVideoFormatPropertiesKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceWaylandPresentationSupportKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceWaylandPresentationSupportKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceWin32PresentationSupportKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceWin32PresentationSupportKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceXcbPresentationSupportKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceXcbPresentationSupportKHR_NAME.as_ptr(),
    ));
    self.GetPhysicalDeviceXlibPresentationSupportKHR = transmute(loader(
      instance,
      vkGetPhysicalDeviceXlibPresentationSupportKHR_NAME.as_ptr(),
    ));
    self.GetWinrtDisplayNV =
      transmute(loader(instance, vkGetWinrtDisplayNV_NAME.as_ptr()));
    self.ReleaseDisplayEXT =
      transmute(loader(instance, vkReleaseDisplayEXT_NAME.as_ptr()));
    self.SubmitDebugUtilsMessageEXT =
      transmute(loader(instance, vkSubmitDebugUtilsMessageEXT_NAME.as_ptr()));
  }
}

/// pointers for 447 fns.
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct DeviceFns {
  /// Khronos: [vkAcquireFullScreenExclusiveModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`, `VK_ERROR_SURFACE_LOST_KHR`
  /// * `device`
  /// * `swapchain`
  pub AcquireFullScreenExclusiveModeEXT: PFN_vkAcquireFullScreenExclusiveModeEXT,

  /// Khronos: [vkAcquireImageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireImageANDROID.html)
  /// * `device`
  /// * `image`
  /// * `native_fence_fd`
  /// * `semaphore`
  /// * `fence`
  pub AcquireImageANDROID: PFN_vkAcquireImageANDROID,

  /// Khronos: [vkAcquireNextImage2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html)
  /// * Success: `VK_SUCCESS`, `VK_TIMEOUT`, `VK_NOT_READY`, `VK_SUBOPTIMAL_KHR`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`, `VK_ERROR_OUT_OF_DATE_KHR`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`,
  ///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
  /// * `device`
  /// * `acquire_info`
  /// * `image_index`
  pub AcquireNextImage2KHR: PFN_vkAcquireNextImage2KHR,

  /// Khronos: [vkAcquireNextImageKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_TIMEOUT`, `VK_NOT_READY`, `VK_SUBOPTIMAL_KHR`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`, `VK_ERROR_OUT_OF_DATE_KHR`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`,
  ///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
  /// * `device`
  /// * `swapchain`, Extern Sync: true
  /// * `timeout`
  /// * `semaphore`, Optional: true, Extern Sync: true
  /// * `fence`, Optional: true, Extern Sync: true
  /// * `image_index`
  pub AcquireNextImageKHR: PFN_vkAcquireNextImageKHR,

  /// Khronos: [vkAcquirePerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `acquire_info`
  /// * `configuration`
  pub AcquirePerformanceConfigurationINTEL: PFN_vkAcquirePerformanceConfigurationINTEL,

  /// Khronos: [vkAcquireProfilingLockKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_TIMEOUT`
  /// * `device`
  /// * `info`
  pub AcquireProfilingLockKHR: PFN_vkAcquireProfilingLockKHR,

  /// Khronos: [vkAllocateCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `allocate_info`, Extern Sync: pAllocateInfo-&gt;commandPool
  /// * `command_buffers`, Len: `allocate_info_&gt;command_buffer_count`
  pub AllocateCommandBuffers: PFN_vkAllocateCommandBuffers,

  /// Khronos: [vkAllocateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_FRAGMENTED_POOL`, `VK_ERROR_OUT_OF_POOL_MEMORY`
  /// * `device`
  /// * `allocate_info`, Extern Sync: pAllocateInfo-&gt;descriptorPool
  /// * `descriptor_sets`, Len: `allocate_info_&gt;descriptor_set_count`
  pub AllocateDescriptorSets: PFN_vkAllocateDescriptorSets,

  /// Khronos: [vkAllocateMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_EXTERNAL_HANDLE`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  /// * `device`
  /// * `allocate_info`
  /// * `allocator`, Optional: true
  /// * `memory`
  pub AllocateMemory: PFN_vkAllocateMemory,

  /// Khronos: [vkBeginCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * Implicit Extern Sync: the [VkCommandPool] that `commandBuffer` was
  ///   allocated from
  /// * `command_buffer`, Extern Sync: true
  /// * `begin_info`
  pub BeginCommandBuffer: PFN_vkBeginCommandBuffer,

  /// Khronos: [vkBindAccelerationStructureMemoryNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `bind_info_count`
  /// * `bind_infos`, Len: `bind_info_count`
  pub BindAccelerationStructureMemoryNV: PFN_vkBindAccelerationStructureMemoryNV,

  /// Khronos: [vkBindBufferMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  /// * `device`
  /// * `buffer`, Extern Sync: true
  /// * `memory`
  /// * `memory_offset`
  pub BindBufferMemory: PFN_vkBindBufferMemory,

  /// Khronos: [vkBindBufferMemory2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  /// * `device`
  /// * `bind_info_count`
  /// * `bind_infos`, Len: `bind_info_count`
  pub BindBufferMemory2: PFN_vkBindBufferMemory2,

  /// Khronos: [vkBindImageMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `image`, Extern Sync: true
  /// * `memory`
  /// * `memory_offset`
  pub BindImageMemory: PFN_vkBindImageMemory,

  /// Khronos: [vkBindImageMemory2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `bind_info_count`
  /// * `bind_infos`, Len: `bind_info_count`
  pub BindImageMemory2: PFN_vkBindImageMemory2,

  /// Khronos: [vkBindOpticalFlowSessionImageNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindOpticalFlowSessionImageNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INITIALIZATION_FAILED`
  /// * `device`
  /// * `session`
  /// * `binding_point`
  /// * `view`, Optional: true
  /// * `layout`
  pub BindOpticalFlowSessionImageNV: PFN_vkBindOpticalFlowSessionImageNV,

  /// Khronos: [vkBindVideoSessionMemoryKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `video_session`, Extern Sync: true
  /// * `bind_session_memory_info_count`
  /// * `bind_session_memory_infos`, Len: `bind_session_memory_info_count`
  pub BindVideoSessionMemoryKHR: PFN_vkBindVideoSessionMemoryKHR,

  /// Khronos: [vkCmdBeginConditionalRenderingEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  /// * `conditional_rendering_begin`
  pub CmdBeginConditionalRenderingEXT: PFN_vkCmdBeginConditionalRenderingEXT,

  /// Khronos: [vkCmdBeginDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  /// * `label_info`
  pub CmdBeginDebugUtilsLabelEXT: PFN_vkCmdBeginDebugUtilsLabelEXT,

  /// Khronos: [vkCmdBeginQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `query_pool`
  /// * `query`
  /// * `flags`, Optional: true
  pub CmdBeginQuery: PFN_vkCmdBeginQuery,

  /// Khronos: [vkCmdBeginQueryIndexedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  /// * `query_pool`
  /// * `query`
  /// * `flags`, Optional: true
  /// * `index`
  pub CmdBeginQueryIndexedEXT: PFN_vkCmdBeginQueryIndexedEXT,

  /// Khronos: [vkCmdBeginRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html)
  /// * Queues: graphics
  /// * Render Pass: outside
  /// * Command Buffer Level: primary
  /// * Tasks: action, state, synchronization
  /// * `command_buffer`, Extern Sync: true
  /// * `render_pass_begin`
  /// * `contents`
  pub CmdBeginRenderPass: PFN_vkCmdBeginRenderPass,

  /// Khronos: [vkCmdBeginRenderPass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html)
  /// * Queues: graphics
  /// * Render Pass: outside
  /// * Command Buffer Level: primary
  /// * Tasks: action, state, synchronization
  /// * `command_buffer`, Extern Sync: true
  /// * `render_pass_begin`
  /// * `subpass_begin_info`
  pub CmdBeginRenderPass2: PFN_vkCmdBeginRenderPass2,

  /// Khronos: [vkCmdBeginRendering](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html)
  /// * Queues: graphics
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  /// * `rendering_info`
  pub CmdBeginRendering: PFN_vkCmdBeginRendering,

  /// Khronos: [vkCmdBeginTransformFeedbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_counter_buffer`
  /// * `counter_buffer_count`, Optional: true
  /// * `counter_buffers`, Len: `counter_buffer_count`, true
  /// * `counter_buffer_offsets`, Optional: true, Len: `counter_buffer_count`
  pub CmdBeginTransformFeedbackEXT: PFN_vkCmdBeginTransformFeedbackEXT,

  /// Khronos: [vkCmdBeginVideoCodingKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html)
  /// * Queues: decode, encode
  /// * Render Pass: outside
  /// * Command Buffer Level: primary
  /// * Tasks: action, state
  /// * Video Coding: outside
  /// * `command_buffer`, Extern Sync: true
  /// * `begin_info`
  pub CmdBeginVideoCodingKHR: PFN_vkCmdBeginVideoCodingKHR,

  /// Khronos: [vkCmdBindDescriptorBufferEmbeddedSamplersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `pipeline_bind_point`
  /// * `layout`
  /// * `set`
  pub CmdBindDescriptorBufferEmbeddedSamplersEXT:
    PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT,

  /// Khronos: [vkCmdBindDescriptorBuffersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBuffersEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer_count`
  /// * `binding_infos`, Len: `buffer_count`
  pub CmdBindDescriptorBuffersEXT: PFN_vkCmdBindDescriptorBuffersEXT,

  /// Khronos: [vkCmdBindDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `pipeline_bind_point`
  /// * `layout`
  /// * `first_set`
  /// * `descriptor_set_count`
  /// * `descriptor_sets`, Optional: false,true, Len: `descriptor_set_count`
  /// * `dynamic_offset_count`, Optional: true
  /// * `dynamic_offsets`, Len: `dynamic_offset_count`
  pub CmdBindDescriptorSets: PFN_vkCmdBindDescriptorSets,

  /// Khronos: [vkCmdBindIndexBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  /// * `index_type`
  pub CmdBindIndexBuffer: PFN_vkCmdBindIndexBuffer,

  /// Khronos: [vkCmdBindInvocationMaskHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `image_view`, Optional: true
  /// * `image_layout`
  pub CmdBindInvocationMaskHUAWEI: PFN_vkCmdBindInvocationMaskHUAWEI,

  /// Khronos: [vkCmdBindPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `pipeline_bind_point`
  /// * `pipeline`
  pub CmdBindPipeline: PFN_vkCmdBindPipeline,

  /// Khronos: [vkCmdBindPipelineShaderGroupNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `pipeline_bind_point`
  /// * `pipeline`
  /// * `group_index`
  pub CmdBindPipelineShaderGroupNV: PFN_vkCmdBindPipelineShaderGroupNV,

  /// Khronos: [vkCmdBindShadingRateImageNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadingRateImageNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `image_view`, Optional: true
  /// * `image_layout`
  pub CmdBindShadingRateImageNV: PFN_vkCmdBindShadingRateImageNV,

  /// Khronos: [vkCmdBindTransformFeedbackBuffersEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_binding`
  /// * `binding_count`
  /// * `buffers`, Len: `binding_count`
  /// * `offsets`, Len: `binding_count`
  /// * `sizes`, Optional: true, Len: `binding_count`, true
  pub CmdBindTransformFeedbackBuffersEXT: PFN_vkCmdBindTransformFeedbackBuffersEXT,

  /// Khronos: [vkCmdBindVertexBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_binding`
  /// * `binding_count`
  /// * `buffers`, Optional: false,true, Len: `binding_count`
  /// * `offsets`, Len: `binding_count`
  pub CmdBindVertexBuffers: PFN_vkCmdBindVertexBuffers,

  /// Khronos: [vkCmdBindVertexBuffers2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_binding`
  /// * `binding_count`
  /// * `buffers`, Optional: false,true, Len: `binding_count`
  /// * `offsets`, Len: `binding_count`
  /// * `sizes`, Optional: true, Len: `binding_count`
  /// * `strides`, Optional: true, Len: `binding_count`
  pub CmdBindVertexBuffers2: PFN_vkCmdBindVertexBuffers2,

  /// Khronos: [vkCmdBlitImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html)
  /// * Queues: graphics
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `src_image`
  /// * `src_image_layout`
  /// * `dst_image`
  /// * `dst_image_layout`
  /// * `region_count`
  /// * `regions`, Len: `region_count`
  /// * `filter`
  pub CmdBlitImage: PFN_vkCmdBlitImage,

  /// Khronos: [vkCmdBlitImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2.html)
  /// * Queues: graphics
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `blit_image_info`
  pub CmdBlitImage2: PFN_vkCmdBlitImage2,

  /// Khronos: [vkCmdBuildAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `info`
  /// * `instance_data`, Optional: true
  /// * `instance_offset`
  /// * `update`
  /// * `dst`
  /// * `src`, Optional: true
  /// * `scratch`
  /// * `scratch_offset`
  pub CmdBuildAccelerationStructureNV: PFN_vkCmdBuildAccelerationStructureNV,

  /// Khronos: [vkCmdClearAttachments](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `attachment_count`
  /// * `attachments`, Len: `attachment_count`
  /// * `rect_count`
  /// * `rects`, Len: `rect_count`
  pub CmdClearAttachments: PFN_vkCmdClearAttachments,

  /// Khronos: [vkCmdClearColorImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html)
  /// * Queues: graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `image`
  /// * `image_layout`
  /// * `color`, true
  /// * `range_count`
  /// * `ranges`, Len: `range_count`
  pub CmdClearColorImage: PFN_vkCmdClearColorImage,

  /// Khronos: [vkCmdClearDepthStencilImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html)
  /// * Queues: graphics
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `image`
  /// * `image_layout`
  /// * `depth_stencil`
  /// * `range_count`
  /// * `ranges`, Len: `range_count`
  pub CmdClearDepthStencilImage: PFN_vkCmdClearDepthStencilImage,

  /// Khronos: [vkCmdControlVideoCodingKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html)
  /// * Queues: decode, encode
  /// * Render Pass: outside
  /// * Command Buffer Level: primary
  /// * Tasks: action
  /// * Video Coding: inside
  /// * `command_buffer`, Extern Sync: true
  /// * `coding_control_info`
  pub CmdControlVideoCodingKHR: PFN_vkCmdControlVideoCodingKHR,

  /// Khronos: [vkCmdCopyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `info`
  pub CmdCopyAccelerationStructureKHR: PFN_vkCmdCopyAccelerationStructureKHR,

  /// Khronos: [vkCmdCopyAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `dst`
  /// * `src`
  /// * `mode`
  pub CmdCopyAccelerationStructureNV: PFN_vkCmdCopyAccelerationStructureNV,

  /// Khronos: [vkCmdCopyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `src_buffer`
  /// * `dst_buffer`
  /// * `region_count`
  /// * `regions`, Len: `region_count`
  pub CmdCopyBuffer: PFN_vkCmdCopyBuffer,

  /// Khronos: [vkCmdCopyBuffer2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `copy_buffer_info`
  pub CmdCopyBuffer2: PFN_vkCmdCopyBuffer2,

  /// Khronos: [vkCmdCopyBufferToImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `src_buffer`
  /// * `dst_image`
  /// * `dst_image_layout`
  /// * `region_count`
  /// * `regions`, Len: `region_count`
  pub CmdCopyBufferToImage: PFN_vkCmdCopyBufferToImage,

  /// Khronos: [vkCmdCopyBufferToImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `copy_buffer_to_image_info`
  pub CmdCopyBufferToImage2: PFN_vkCmdCopyBufferToImage2,

  /// Khronos: [vkCmdCopyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `src_image`
  /// * `src_image_layout`
  /// * `dst_image`
  /// * `dst_image_layout`
  /// * `region_count`
  /// * `regions`, Len: `region_count`
  pub CmdCopyImage: PFN_vkCmdCopyImage,

  /// Khronos: [vkCmdCopyImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `copy_image_info`
  pub CmdCopyImage2: PFN_vkCmdCopyImage2,

  /// Khronos: [vkCmdCopyImageToBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `src_image`
  /// * `src_image_layout`
  /// * `dst_buffer`
  /// * `region_count`
  /// * `regions`, Len: `region_count`
  pub CmdCopyImageToBuffer: PFN_vkCmdCopyImageToBuffer,

  /// Khronos: [vkCmdCopyImageToBuffer2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `copy_image_to_buffer_info`
  pub CmdCopyImageToBuffer2: PFN_vkCmdCopyImageToBuffer2,

  /// Khronos: [vkCmdCopyMemoryIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryIndirectNV.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `copy_buffer_address`
  /// * `copy_count`
  /// * `stride`
  pub CmdCopyMemoryIndirectNV: PFN_vkCmdCopyMemoryIndirectNV,

  /// Khronos: [vkCmdCopyMemoryToImageIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToImageIndirectNV.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `copy_buffer_address`
  /// * `copy_count`
  /// * `stride`
  /// * `dst_image`
  /// * `dst_image_layout`
  /// * `image_subresources`, Len: `copy_count`
  pub CmdCopyMemoryToImageIndirectNV: PFN_vkCmdCopyMemoryToImageIndirectNV,

  /// Khronos: [vkCmdCopyMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapEXT.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `info`
  pub CmdCopyMicromapEXT: PFN_vkCmdCopyMicromapEXT,

  /// Khronos: [vkCmdCopyQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html)
  /// * Queues: graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `query_pool`
  /// * `first_query`
  /// * `query_count`
  /// * `dst_buffer`
  /// * `dst_offset`
  /// * `stride`
  /// * `flags`, Optional: true
  pub CmdCopyQueryPoolResults: PFN_vkCmdCopyQueryPoolResults,

  /// Khronos: [vkCmdCuLaunchKernelNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`
  /// * `launch_info`
  pub CmdCuLaunchKernelNVX: PFN_vkCmdCuLaunchKernelNVX,

  /// Khronos: [vkCmdDebugMarkerBeginEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `marker_info`
  pub CmdDebugMarkerBeginEXT: PFN_vkCmdDebugMarkerBeginEXT,

  /// Khronos: [vkCmdDebugMarkerEndEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  pub CmdDebugMarkerEndEXT: PFN_vkCmdDebugMarkerEndEXT,

  /// Khronos: [vkCmdDebugMarkerInsertEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `marker_info`
  pub CmdDebugMarkerInsertEXT: PFN_vkCmdDebugMarkerInsertEXT,

  /// Khronos: [vkCmdDecodeVideoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecodeVideoKHR.html)
  /// * Queues: decode
  /// * Render Pass: outside
  /// * Command Buffer Level: primary
  /// * Tasks: action
  /// * Video Coding: inside
  /// * `command_buffer`, Extern Sync: true
  /// * `decode_info`
  pub CmdDecodeVideoKHR: PFN_vkCmdDecodeVideoKHR,

  /// Khronos: [vkCmdDecompressMemoryIndirectCountNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryIndirectCountNV.html)
  /// * Queues: graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `indirect_commands_address`
  /// * `indirect_commands_count_address`
  /// * `stride`
  pub CmdDecompressMemoryIndirectCountNV: PFN_vkCmdDecompressMemoryIndirectCountNV,

  /// Khronos: [vkCmdDecompressMemoryNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryNV.html)
  /// * Queues: graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `decompress_region_count`
  /// * `decompress_memory_regions`, Len: `decompress_region_count`
  pub CmdDecompressMemoryNV: PFN_vkCmdDecompressMemoryNV,

  /// Khronos: [vkCmdDispatch](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `group_count_x`
  /// * `group_count_y`
  /// * `group_count_z`
  pub CmdDispatch: PFN_vkCmdDispatch,

  /// Khronos: [vkCmdDispatchBase](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `base_group_x`
  /// * `base_group_y`
  /// * `base_group_z`
  /// * `group_count_x`
  /// * `group_count_y`
  /// * `group_count_z`
  pub CmdDispatchBase: PFN_vkCmdDispatchBase,

  /// Khronos: [vkCmdDispatchIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  pub CmdDispatchIndirect: PFN_vkCmdDispatchIndirect,

  /// Khronos: [vkCmdDraw](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `vertex_count`
  /// * `instance_count`
  /// * `first_vertex`
  /// * `first_instance`
  pub CmdDraw: PFN_vkCmdDraw,

  /// Khronos: [vkCmdDrawClusterHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterHUAWEI.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `group_count_x`
  /// * `group_count_y`
  /// * `group_count_z`
  pub CmdDrawClusterHUAWEI: PFN_vkCmdDrawClusterHUAWEI,

  /// Khronos: [vkCmdDrawClusterIndirectHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterIndirectHUAWEI.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  pub CmdDrawClusterIndirectHUAWEI: PFN_vkCmdDrawClusterIndirectHUAWEI,

  /// Khronos: [vkCmdDrawIndexed](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `index_count`
  /// * `instance_count`
  /// * `first_index`
  /// * `vertex_offset`
  /// * `first_instance`
  pub CmdDrawIndexed: PFN_vkCmdDrawIndexed,

  /// Khronos: [vkCmdDrawIndexedIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  /// * `draw_count`
  /// * `stride`
  pub CmdDrawIndexedIndirect: PFN_vkCmdDrawIndexedIndirect,

  /// Khronos: [vkCmdDrawIndexedIndirectCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCount.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  /// * `count_buffer`
  /// * `count_buffer_offset`
  /// * `max_draw_count`
  /// * `stride`
  pub CmdDrawIndexedIndirectCount: PFN_vkCmdDrawIndexedIndirectCount,

  /// Khronos: [vkCmdDrawIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  /// * `draw_count`
  /// * `stride`
  pub CmdDrawIndirect: PFN_vkCmdDrawIndirect,

  /// Khronos: [vkCmdDrawIndirectByteCountEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `instance_count`
  /// * `first_instance`
  /// * `counter_buffer`
  /// * `counter_buffer_offset`
  /// * `counter_offset`
  /// * `vertex_stride`
  pub CmdDrawIndirectByteCountEXT: PFN_vkCmdDrawIndirectByteCountEXT,

  /// Khronos: [vkCmdDrawIndirectCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCount.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  /// * `count_buffer`
  /// * `count_buffer_offset`
  /// * `max_draw_count`
  /// * `stride`
  pub CmdDrawIndirectCount: PFN_vkCmdDrawIndirectCount,

  /// Khronos: [vkCmdDrawMeshTasksEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `group_count_x`
  /// * `group_count_y`
  /// * `group_count_z`
  pub CmdDrawMeshTasksEXT: PFN_vkCmdDrawMeshTasksEXT,

  /// Khronos: [vkCmdDrawMeshTasksIndirectCountEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  /// * `count_buffer`
  /// * `count_buffer_offset`
  /// * `max_draw_count`
  /// * `stride`
  pub CmdDrawMeshTasksIndirectCountEXT: PFN_vkCmdDrawMeshTasksIndirectCountEXT,

  /// Khronos: [vkCmdDrawMeshTasksIndirectCountNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  /// * `count_buffer`
  /// * `count_buffer_offset`
  /// * `max_draw_count`
  /// * `stride`
  pub CmdDrawMeshTasksIndirectCountNV: PFN_vkCmdDrawMeshTasksIndirectCountNV,

  /// Khronos: [vkCmdDrawMeshTasksIndirectEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectEXT.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  /// * `draw_count`
  /// * `stride`
  pub CmdDrawMeshTasksIndirectEXT: PFN_vkCmdDrawMeshTasksIndirectEXT,

  /// Khronos: [vkCmdDrawMeshTasksIndirectNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `buffer`
  /// * `offset`
  /// * `draw_count`
  /// * `stride`
  pub CmdDrawMeshTasksIndirectNV: PFN_vkCmdDrawMeshTasksIndirectNV,

  /// Khronos: [vkCmdDrawMeshTasksNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `task_count`
  /// * `first_task`
  pub CmdDrawMeshTasksNV: PFN_vkCmdDrawMeshTasksNV,

  /// Khronos: [vkCmdDrawMultiEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `draw_count`, Optional: true
  /// * `vertex_info`, Len: `draw_count`, true, Stride: stride
  /// * `instance_count`
  /// * `first_instance`
  /// * `stride`
  pub CmdDrawMultiEXT: PFN_vkCmdDrawMultiEXT,

  /// Khronos: [vkCmdDrawMultiIndexedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `draw_count`, Optional: true
  /// * `index_info`, Len: `draw_count`, true, Stride: stride
  /// * `instance_count`
  /// * `first_instance`
  /// * `stride`
  /// * `vertex_offset`, Optional: true
  pub CmdDrawMultiIndexedEXT: PFN_vkCmdDrawMultiIndexedEXT,

  /// Khronos: [vkCmdEndConditionalRenderingEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  pub CmdEndConditionalRenderingEXT: PFN_vkCmdEndConditionalRenderingEXT,

  /// Khronos: [vkCmdEndDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  pub CmdEndDebugUtilsLabelEXT: PFN_vkCmdEndDebugUtilsLabelEXT,

  /// Khronos: [vkCmdEndQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `query_pool`
  /// * `query`
  pub CmdEndQuery: PFN_vkCmdEndQuery,

  /// Khronos: [vkCmdEndQueryIndexedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  /// * `query_pool`
  /// * `query`
  /// * `index`
  pub CmdEndQueryIndexedEXT: PFN_vkCmdEndQueryIndexedEXT,

  /// Khronos: [vkCmdEndRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary
  /// * Tasks: action, state, synchronization
  /// * `command_buffer`, Extern Sync: true
  pub CmdEndRenderPass: PFN_vkCmdEndRenderPass,

  /// Khronos: [vkCmdEndRenderPass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary
  /// * Tasks: action, state, synchronization
  /// * `command_buffer`, Extern Sync: true
  /// * `subpass_end_info`
  pub CmdEndRenderPass2: PFN_vkCmdEndRenderPass2,

  /// Khronos: [vkCmdEndRendering](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  pub CmdEndRendering: PFN_vkCmdEndRendering,

  /// Khronos: [vkCmdEndTransformFeedbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_counter_buffer`
  /// * `counter_buffer_count`, Optional: true
  /// * `counter_buffers`, Len: `counter_buffer_count`, true
  /// * `counter_buffer_offsets`, Optional: true, Len: `counter_buffer_count`
  pub CmdEndTransformFeedbackEXT: PFN_vkCmdEndTransformFeedbackEXT,

  /// Khronos: [vkCmdEndVideoCodingKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html)
  /// * Queues: decode, encode
  /// * Render Pass: outside
  /// * Command Buffer Level: primary
  /// * Tasks: action, state
  /// * Video Coding: inside
  /// * `command_buffer`, Extern Sync: true
  /// * `end_coding_info`
  pub CmdEndVideoCodingKHR: PFN_vkCmdEndVideoCodingKHR,

  /// Khronos: [vkCmdExecuteCommands](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary
  /// * Tasks: indirection
  /// * `command_buffer`, Extern Sync: true
  /// * `command_buffer_count`
  /// * `command_buffers`, Len: `command_buffer_count`
  pub CmdExecuteCommands: PFN_vkCmdExecuteCommands,

  /// Khronos: [vkCmdExecuteGeneratedCommandsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html)
  /// * Queues: graphics, compute
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, indirection
  /// * `command_buffer`, Extern Sync: true
  /// * `is_preprocessed`
  /// * `generated_commands_info`
  pub CmdExecuteGeneratedCommandsNV: PFN_vkCmdExecuteGeneratedCommandsNV,

  /// Khronos: [vkCmdFillBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html)
  ///
  /// transfer support is only available when VK_KHR_maintenance1 is enabled, as
  /// documented in valid usage language in the specification
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `dst_buffer`
  /// * `dst_offset`
  /// * `size`
  /// * `data`
  pub CmdFillBuffer: PFN_vkCmdFillBuffer,

  /// Khronos: [vkCmdInsertDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `label_info`
  pub CmdInsertDebugUtilsLabelEXT: PFN_vkCmdInsertDebugUtilsLabelEXT,

  /// Khronos: [vkCmdNextSubpass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary
  /// * Tasks: action, state, synchronization
  /// * `command_buffer`, Extern Sync: true
  /// * `contents`
  pub CmdNextSubpass: PFN_vkCmdNextSubpass,

  /// Khronos: [vkCmdNextSubpass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary
  /// * Tasks: action, state, synchronization
  /// * `command_buffer`, Extern Sync: true
  /// * `subpass_begin_info`
  /// * `subpass_end_info`
  pub CmdNextSubpass2: PFN_vkCmdNextSubpass2,

  /// Khronos: [vkCmdOpticalFlowExecuteNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdOpticalFlowExecuteNV.html)
  /// * Queues: opticalflow
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`
  /// * `session`
  /// * `execute_info`
  pub CmdOpticalFlowExecuteNV: PFN_vkCmdOpticalFlowExecuteNV,

  /// Khronos: [vkCmdPipelineBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html)
  /// * Queues: transfer, graphics, compute, decode, encode
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: synchronization
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `src_stage_mask`, Optional: true
  /// * `dst_stage_mask`, Optional: true
  /// * `dependency_flags`, Optional: true
  /// * `memory_barrier_count`, Optional: true
  /// * `memory_barriers`, Len: `memory_barrier_count`
  /// * `buffer_memory_barrier_count`, Optional: true
  /// * `buffer_memory_barriers`, Len: `buffer_memory_barrier_count`
  /// * `image_memory_barrier_count`, Optional: true
  /// * `image_memory_barriers`, Len: `image_memory_barrier_count`
  pub CmdPipelineBarrier: PFN_vkCmdPipelineBarrier,

  /// Khronos: [vkCmdPipelineBarrier2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html)
  /// * Queues: transfer, graphics, compute, decode, encode
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: synchronization
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `dependency_info`
  pub CmdPipelineBarrier2: PFN_vkCmdPipelineBarrier2,

  /// Khronos: [vkCmdPreprocessGeneratedCommandsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html)
  /// * Queues: graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `generated_commands_info`
  pub CmdPreprocessGeneratedCommandsNV: PFN_vkCmdPreprocessGeneratedCommandsNV,

  /// Khronos: [vkCmdPushConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `layout`
  /// * `stage_flags`
  /// * `offset`
  /// * `size`
  /// * `values`, Len: `size`
  pub CmdPushConstants: PFN_vkCmdPushConstants,

  /// Khronos: [vkCmdPushDescriptorSetKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `pipeline_bind_point`
  /// * `layout`
  /// * `set`
  /// * `descriptor_write_count`
  /// * `descriptor_writes`, Len: `descriptor_write_count`
  pub CmdPushDescriptorSetKHR: PFN_vkCmdPushDescriptorSetKHR,

  /// Khronos: [vkCmdPushDescriptorSetWithTemplateKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `descriptor_update_template`
  /// * `layout`
  /// * `set`
  /// * `data`, true
  pub CmdPushDescriptorSetWithTemplateKHR: PFN_vkCmdPushDescriptorSetWithTemplateKHR,

  /// Khronos: [vkCmdRefreshObjectsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdRefreshObjectsKHR.html)
  /// * Queues: graphics, compute, transfer
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `refresh_objects`
  pub CmdRefreshObjectsKHR: PFN_vkCmdRefreshObjectsKHR,

  /// Khronos: [vkCmdResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: synchronization
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `event`
  /// * `stage_mask`, Optional: true
  pub CmdResetEvent: PFN_vkCmdResetEvent,

  /// Khronos: [vkCmdResetEvent2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: synchronization
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `event`
  /// * `stage_mask`, Optional: true
  pub CmdResetEvent2: PFN_vkCmdResetEvent2,

  /// Khronos: [vkCmdResetQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html)
  /// * Queues: graphics, compute, decode, encode, opticalflow
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `query_pool`
  /// * `first_query`
  /// * `query_count`
  pub CmdResetQueryPool: PFN_vkCmdResetQueryPool,

  /// Khronos: [vkCmdResolveImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html)
  /// * Queues: graphics
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `src_image`
  /// * `src_image_layout`
  /// * `dst_image`
  /// * `dst_image_layout`
  /// * `region_count`
  /// * `regions`, Len: `region_count`
  pub CmdResolveImage: PFN_vkCmdResolveImage,

  /// Khronos: [vkCmdResolveImage2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html)
  /// * Queues: graphics
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `resolve_image_info`
  pub CmdResolveImage2: PFN_vkCmdResolveImage2,

  /// Khronos: [vkCmdSetAlphaToCoverageEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `alpha_to_coverage_enable`
  pub CmdSetAlphaToCoverageEnableEXT: PFN_vkCmdSetAlphaToCoverageEnableEXT,

  /// Khronos: [vkCmdSetAlphaToOneEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `alpha_to_one_enable`
  pub CmdSetAlphaToOneEnableEXT: PFN_vkCmdSetAlphaToOneEnableEXT,

  /// Khronos: [vkCmdSetBlendConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `blend_constants`
  pub CmdSetBlendConstants: PFN_vkCmdSetBlendConstants,

  /// Khronos: [vkCmdSetCheckpointNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html)
  /// * Queues: graphics, compute, transfer
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `checkpoint_marker`, true
  pub CmdSetCheckpointNV: PFN_vkCmdSetCheckpointNV,

  /// Khronos: [vkCmdSetCoarseSampleOrderNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `sample_order_type`
  /// * `custom_sample_order_count`, Optional: true
  /// * `custom_sample_orders`, Len: `custom_sample_order_count`
  pub CmdSetCoarseSampleOrderNV: PFN_vkCmdSetCoarseSampleOrderNV,

  /// Khronos: [vkCmdSetColorBlendAdvancedEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_attachment`
  /// * `attachment_count`
  /// * `color_blend_advanced`, Len: `attachment_count`
  pub CmdSetColorBlendAdvancedEXT: PFN_vkCmdSetColorBlendAdvancedEXT,

  /// Khronos: [vkCmdSetColorBlendEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_attachment`
  /// * `attachment_count`
  /// * `color_blend_enables`, Len: `attachment_count`
  pub CmdSetColorBlendEnableEXT: PFN_vkCmdSetColorBlendEnableEXT,

  /// Khronos: [vkCmdSetColorBlendEquationEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_attachment`
  /// * `attachment_count`
  /// * `color_blend_equations`, Len: `attachment_count`
  pub CmdSetColorBlendEquationEXT: PFN_vkCmdSetColorBlendEquationEXT,

  /// Khronos: [vkCmdSetColorWriteEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `attachment_count`
  /// * `color_write_enables`, Len: `attachment_count`
  pub CmdSetColorWriteEnableEXT: PFN_vkCmdSetColorWriteEnableEXT,

  /// Khronos: [vkCmdSetColorWriteMaskEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_attachment`
  /// * `attachment_count`
  /// * `color_write_masks`, Optional: false,true, Len: `attachment_count`
  pub CmdSetColorWriteMaskEXT: PFN_vkCmdSetColorWriteMaskEXT,

  /// Khronos: [vkCmdSetConservativeRasterizationModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `conservative_rasterization_mode`
  pub CmdSetConservativeRasterizationModeEXT:
    PFN_vkCmdSetConservativeRasterizationModeEXT,

  /// Khronos: [vkCmdSetCoverageModulationModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `coverage_modulation_mode`
  pub CmdSetCoverageModulationModeNV: PFN_vkCmdSetCoverageModulationModeNV,

  /// Khronos: [vkCmdSetCoverageModulationTableEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `coverage_modulation_table_enable`
  pub CmdSetCoverageModulationTableEnableNV: PFN_vkCmdSetCoverageModulationTableEnableNV,

  /// Khronos: [vkCmdSetCoverageModulationTableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `coverage_modulation_table_count`
  /// * `coverage_modulation_table`, Len: `coverage_modulation_table_count`
  pub CmdSetCoverageModulationTableNV: PFN_vkCmdSetCoverageModulationTableNV,

  /// Khronos: [vkCmdSetCoverageReductionModeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `coverage_reduction_mode`
  pub CmdSetCoverageReductionModeNV: PFN_vkCmdSetCoverageReductionModeNV,

  /// Khronos: [vkCmdSetCoverageToColorEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `coverage_to_color_enable`
  pub CmdSetCoverageToColorEnableNV: PFN_vkCmdSetCoverageToColorEnableNV,

  /// Khronos: [vkCmdSetCoverageToColorLocationNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `coverage_to_color_location`
  pub CmdSetCoverageToColorLocationNV: PFN_vkCmdSetCoverageToColorLocationNV,

  /// Khronos: [vkCmdSetCullMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `cull_mode`, Optional: true
  pub CmdSetCullMode: PFN_vkCmdSetCullMode,

  /// Khronos: [vkCmdSetDepthBias](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `depth_bias_constant_factor`
  /// * `depth_bias_clamp`
  /// * `depth_bias_slope_factor`
  pub CmdSetDepthBias: PFN_vkCmdSetDepthBias,

  /// Khronos: [vkCmdSetDepthBiasEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnable.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `depth_bias_enable`
  pub CmdSetDepthBiasEnable: PFN_vkCmdSetDepthBiasEnable,

  /// Khronos: [vkCmdSetDepthBounds](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `min_depth_bounds`
  /// * `max_depth_bounds`
  pub CmdSetDepthBounds: PFN_vkCmdSetDepthBounds,

  /// Khronos: [vkCmdSetDepthBoundsTestEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `depth_bounds_test_enable`
  pub CmdSetDepthBoundsTestEnable: PFN_vkCmdSetDepthBoundsTestEnable,

  /// Khronos: [vkCmdSetDepthClampEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `depth_clamp_enable`
  pub CmdSetDepthClampEnableEXT: PFN_vkCmdSetDepthClampEnableEXT,

  /// Khronos: [vkCmdSetDepthClipEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `depth_clip_enable`
  pub CmdSetDepthClipEnableEXT: PFN_vkCmdSetDepthClipEnableEXT,

  /// Khronos: [vkCmdSetDepthClipNegativeOneToOneEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `negative_one_to_one`
  pub CmdSetDepthClipNegativeOneToOneEXT: PFN_vkCmdSetDepthClipNegativeOneToOneEXT,

  /// Khronos: [vkCmdSetDepthCompareOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `depth_compare_op`
  pub CmdSetDepthCompareOp: PFN_vkCmdSetDepthCompareOp,

  /// Khronos: [vkCmdSetDepthTestEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `depth_test_enable`
  pub CmdSetDepthTestEnable: PFN_vkCmdSetDepthTestEnable,

  /// Khronos: [vkCmdSetDepthWriteEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `depth_write_enable`
  pub CmdSetDepthWriteEnable: PFN_vkCmdSetDepthWriteEnable,

  /// Khronos: [vkCmdSetDescriptorBufferOffsetsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html)
  /// * Queues: graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `pipeline_bind_point`
  /// * `layout`
  /// * `first_set`
  /// * `set_count`
  /// * `buffer_indices`, Len: `set_count`
  /// * `offsets`, Len: `set_count`
  pub CmdSetDescriptorBufferOffsetsEXT: PFN_vkCmdSetDescriptorBufferOffsetsEXT,

  /// Khronos: [vkCmdSetDeviceMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html)
  /// * Queues: graphics, compute, transfer
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `device_mask`
  pub CmdSetDeviceMask: PFN_vkCmdSetDeviceMask,

  /// Khronos: [vkCmdSetDiscardRectangleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_discard_rectangle`
  /// * `discard_rectangle_count`
  /// * `discard_rectangles`, Len: `discard_rectangle_count`
  pub CmdSetDiscardRectangleEXT: PFN_vkCmdSetDiscardRectangleEXT,

  /// Khronos: [vkCmdSetDiscardRectangleEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `discard_rectangle_enable`
  pub CmdSetDiscardRectangleEnableEXT: PFN_vkCmdSetDiscardRectangleEnableEXT,

  /// Khronos: [vkCmdSetDiscardRectangleModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleModeEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `discard_rectangle_mode`
  pub CmdSetDiscardRectangleModeEXT: PFN_vkCmdSetDiscardRectangleModeEXT,

  /// Khronos: [vkCmdSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: synchronization
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `event`
  /// * `stage_mask`, Optional: true
  pub CmdSetEvent: PFN_vkCmdSetEvent,

  /// Khronos: [vkCmdSetEvent2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: synchronization
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `event`
  /// * `dependency_info`
  pub CmdSetEvent2: PFN_vkCmdSetEvent2,

  /// Khronos: [vkCmdSetExclusiveScissorEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorEnableNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_exclusive_scissor`
  /// * `exclusive_scissor_count`
  /// * `exclusive_scissor_enables`, Len: `exclusive_scissor_count`
  pub CmdSetExclusiveScissorEnableNV: PFN_vkCmdSetExclusiveScissorEnableNV,

  /// Khronos: [vkCmdSetExclusiveScissorNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_exclusive_scissor`
  /// * `exclusive_scissor_count`
  /// * `exclusive_scissors`, Len: `exclusive_scissor_count`
  pub CmdSetExclusiveScissorNV: PFN_vkCmdSetExclusiveScissorNV,

  /// Khronos: [vkCmdSetExtraPrimitiveOverestimationSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `extra_primitive_overestimation_size`
  pub CmdSetExtraPrimitiveOverestimationSizeEXT:
    PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,

  /// Khronos: [vkCmdSetFragmentShadingRateEnumNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `shading_rate`
  /// * `combiner_ops`
  pub CmdSetFragmentShadingRateEnumNV: PFN_vkCmdSetFragmentShadingRateEnumNV,

  /// Khronos: [vkCmdSetFragmentShadingRateKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `fragment_size`
  /// * `combiner_ops`
  pub CmdSetFragmentShadingRateKHR: PFN_vkCmdSetFragmentShadingRateKHR,

  /// Khronos: [vkCmdSetFrontFace](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `front_face`
  pub CmdSetFrontFace: PFN_vkCmdSetFrontFace,

  /// Khronos: [vkCmdSetLineRasterizationModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `line_rasterization_mode`
  pub CmdSetLineRasterizationModeEXT: PFN_vkCmdSetLineRasterizationModeEXT,

  /// Khronos: [vkCmdSetLineStippleEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `line_stipple_factor`
  /// * `line_stipple_pattern`
  pub CmdSetLineStippleEXT: PFN_vkCmdSetLineStippleEXT,

  /// Khronos: [vkCmdSetLineStippleEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `stippled_line_enable`
  pub CmdSetLineStippleEnableEXT: PFN_vkCmdSetLineStippleEnableEXT,

  /// Khronos: [vkCmdSetLineWidth](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `line_width`
  pub CmdSetLineWidth: PFN_vkCmdSetLineWidth,

  /// Khronos: [vkCmdSetLogicOpEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `logic_op`
  pub CmdSetLogicOpEXT: PFN_vkCmdSetLogicOpEXT,

  /// Khronos: [vkCmdSetLogicOpEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `logic_op_enable`
  pub CmdSetLogicOpEnableEXT: PFN_vkCmdSetLogicOpEnableEXT,

  /// Khronos: [vkCmdSetPatchControlPointsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `patch_control_points`
  pub CmdSetPatchControlPointsEXT: PFN_vkCmdSetPatchControlPointsEXT,

  /// Khronos: [vkCmdSetPerformanceMarkerINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * Queues: graphics, compute, transfer
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  /// * `marker_info`
  pub CmdSetPerformanceMarkerINTEL: PFN_vkCmdSetPerformanceMarkerINTEL,

  /// Khronos: [vkCmdSetPerformanceOverrideINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * Queues: graphics, compute, transfer
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `override_info`
  pub CmdSetPerformanceOverrideINTEL: PFN_vkCmdSetPerformanceOverrideINTEL,

  /// Khronos: [vkCmdSetPerformanceStreamMarkerINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * Queues: graphics, compute, transfer
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action, state
  /// * `command_buffer`, Extern Sync: true
  /// * `marker_info`
  pub CmdSetPerformanceStreamMarkerINTEL: PFN_vkCmdSetPerformanceStreamMarkerINTEL,

  /// Khronos: [vkCmdSetPolygonModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `polygon_mode`
  pub CmdSetPolygonModeEXT: PFN_vkCmdSetPolygonModeEXT,

  /// Khronos: [vkCmdSetPrimitiveRestartEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `primitive_restart_enable`
  pub CmdSetPrimitiveRestartEnable: PFN_vkCmdSetPrimitiveRestartEnable,

  /// Khronos: [vkCmdSetPrimitiveTopology](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `primitive_topology`
  pub CmdSetPrimitiveTopology: PFN_vkCmdSetPrimitiveTopology,

  /// Khronos: [vkCmdSetProvokingVertexModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `provoking_vertex_mode`
  pub CmdSetProvokingVertexModeEXT: PFN_vkCmdSetProvokingVertexModeEXT,

  /// Khronos: [vkCmdSetRasterizationSamplesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `rasterization_samples`
  pub CmdSetRasterizationSamplesEXT: PFN_vkCmdSetRasterizationSamplesEXT,

  /// Khronos: [vkCmdSetRasterizationStreamEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `rasterization_stream`
  pub CmdSetRasterizationStreamEXT: PFN_vkCmdSetRasterizationStreamEXT,

  /// Khronos: [vkCmdSetRasterizerDiscardEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `rasterizer_discard_enable`
  pub CmdSetRasterizerDiscardEnable: PFN_vkCmdSetRasterizerDiscardEnable,

  /// Khronos: [vkCmdSetRayTracingPipelineStackSizeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `pipeline_stack_size`
  pub CmdSetRayTracingPipelineStackSizeKHR: PFN_vkCmdSetRayTracingPipelineStackSizeKHR,

  /// Khronos: [vkCmdSetRepresentativeFragmentTestEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `representative_fragment_test_enable`
  pub CmdSetRepresentativeFragmentTestEnableNV:
    PFN_vkCmdSetRepresentativeFragmentTestEnableNV,

  /// Khronos: [vkCmdSetSampleLocationsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `sample_locations_info`
  pub CmdSetSampleLocationsEXT: PFN_vkCmdSetSampleLocationsEXT,

  /// Khronos: [vkCmdSetSampleLocationsEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `sample_locations_enable`
  pub CmdSetSampleLocationsEnableEXT: PFN_vkCmdSetSampleLocationsEnableEXT,

  /// Khronos: [vkCmdSetSampleMaskEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `samples`
  /// * `sample_mask`, Len: `(samples + 31) / 32`
  pub CmdSetSampleMaskEXT: PFN_vkCmdSetSampleMaskEXT,

  /// Khronos: [vkCmdSetScissor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_scissor`
  /// * `scissor_count`
  /// * `scissors`, Len: `scissor_count`
  pub CmdSetScissor: PFN_vkCmdSetScissor,

  /// Khronos: [vkCmdSetScissorWithCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `scissor_count`
  /// * `scissors`, Len: `scissor_count`
  pub CmdSetScissorWithCount: PFN_vkCmdSetScissorWithCount,

  /// Khronos: [vkCmdSetShadingRateImageEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `shading_rate_image_enable`
  pub CmdSetShadingRateImageEnableNV: PFN_vkCmdSetShadingRateImageEnableNV,

  /// Khronos: [vkCmdSetStencilCompareMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `face_mask`
  /// * `compare_mask`
  pub CmdSetStencilCompareMask: PFN_vkCmdSetStencilCompareMask,

  /// Khronos: [vkCmdSetStencilOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOp.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `face_mask`
  /// * `fail_op`
  /// * `pass_op`
  /// * `depth_fail_op`
  /// * `compare_op`
  pub CmdSetStencilOp: PFN_vkCmdSetStencilOp,

  /// Khronos: [vkCmdSetStencilReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `face_mask`
  /// * `reference`
  pub CmdSetStencilReference: PFN_vkCmdSetStencilReference,

  /// Khronos: [vkCmdSetStencilTestEnable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnable.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `stencil_test_enable`
  pub CmdSetStencilTestEnable: PFN_vkCmdSetStencilTestEnable,

  /// Khronos: [vkCmdSetStencilWriteMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `face_mask`
  /// * `write_mask`
  pub CmdSetStencilWriteMask: PFN_vkCmdSetStencilWriteMask,

  /// Khronos: [vkCmdSetTessellationDomainOriginEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `domain_origin`
  pub CmdSetTessellationDomainOriginEXT: PFN_vkCmdSetTessellationDomainOriginEXT,

  /// Khronos: [vkCmdSetVertexInputEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `vertex_binding_description_count`, Optional: true
  /// * `vertex_binding_descriptions`, Len: `vertex_binding_description_count`
  /// * `vertex_attribute_description_count`, Optional: true
  /// * `vertex_attribute_descriptions`, Len:
  ///   `vertex_attribute_description_count`
  pub CmdSetVertexInputEXT: PFN_vkCmdSetVertexInputEXT,

  /// Khronos: [vkCmdSetViewport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_viewport`
  /// * `viewport_count`
  /// * `viewports`, Len: `viewport_count`
  pub CmdSetViewport: PFN_vkCmdSetViewport,

  /// Khronos: [vkCmdSetViewportShadingRatePaletteNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_viewport`
  /// * `viewport_count`
  /// * `shading_rate_palettes`, Len: `viewport_count`
  pub CmdSetViewportShadingRatePaletteNV: PFN_vkCmdSetViewportShadingRatePaletteNV,

  /// Khronos: [vkCmdSetViewportSwizzleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_viewport`
  /// * `viewport_count`
  /// * `viewport_swizzles`, Len: `viewport_count`
  pub CmdSetViewportSwizzleNV: PFN_vkCmdSetViewportSwizzleNV,

  /// Khronos: [vkCmdSetViewportWScalingEnableNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `viewport_w_scaling_enable`
  pub CmdSetViewportWScalingEnableNV: PFN_vkCmdSetViewportWScalingEnableNV,

  /// Khronos: [vkCmdSetViewportWScalingNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `first_viewport`
  /// * `viewport_count`
  /// * `viewport_w_scalings`, Len: `viewport_count`
  pub CmdSetViewportWScalingNV: PFN_vkCmdSetViewportWScalingNV,

  /// Khronos: [vkCmdSetViewportWithCount](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html)
  /// * Queues: graphics
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: state
  /// * `command_buffer`, Extern Sync: true
  /// * `viewport_count`
  /// * `viewports`, Len: `viewport_count`
  pub CmdSetViewportWithCount: PFN_vkCmdSetViewportWithCount,

  /// Khronos: [vkCmdSubpassShadingHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html)
  /// * Queues: graphics
  /// * Render Pass: inside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  pub CmdSubpassShadingHUAWEI: PFN_vkCmdSubpassShadingHUAWEI,

  /// Khronos: [vkCmdTraceRaysIndirect2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `indirect_device_address`
  pub CmdTraceRaysIndirect2KHR: PFN_vkCmdTraceRaysIndirect2KHR,

  /// Khronos: [vkCmdTraceRaysIndirectKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `raygen_shader_binding_table`
  /// * `miss_shader_binding_table`
  /// * `hit_shader_binding_table`
  /// * `callable_shader_binding_table`
  /// * `indirect_device_address`
  pub CmdTraceRaysIndirectKHR: PFN_vkCmdTraceRaysIndirectKHR,

  /// Khronos: [vkCmdTraceRaysKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `raygen_shader_binding_table`
  /// * `miss_shader_binding_table`
  /// * `hit_shader_binding_table`
  /// * `callable_shader_binding_table`
  /// * `width`
  /// * `height`
  /// * `depth`
  pub CmdTraceRaysKHR: PFN_vkCmdTraceRaysKHR,

  /// Khronos: [vkCmdTraceRaysNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `raygen_shader_binding_table_buffer`
  /// * `raygen_shader_binding_offset`
  /// * `miss_shader_binding_table_buffer`, Optional: true
  /// * `miss_shader_binding_offset`
  /// * `miss_shader_binding_stride`
  /// * `hit_shader_binding_table_buffer`, Optional: true
  /// * `hit_shader_binding_offset`
  /// * `hit_shader_binding_stride`
  /// * `callable_shader_binding_table_buffer`, Optional: true
  /// * `callable_shader_binding_offset`
  /// * `callable_shader_binding_stride`
  /// * `width`
  /// * `height`
  /// * `depth`
  pub CmdTraceRaysNV: PFN_vkCmdTraceRaysNV,

  /// Khronos: [vkCmdUpdateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `dst_buffer`
  /// * `dst_offset`
  /// * `data_size`
  /// * `data`, Len: `data_size`
  pub CmdUpdateBuffer: PFN_vkCmdUpdateBuffer,

  /// Khronos: [vkCmdWaitEvents](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: synchronization
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `event_count`
  /// * `events`, Len: `event_count`
  /// * `src_stage_mask`, Optional: true
  /// * `dst_stage_mask`, Optional: true
  /// * `memory_barrier_count`, Optional: true
  /// * `memory_barriers`, Len: `memory_barrier_count`
  /// * `buffer_memory_barrier_count`, Optional: true
  /// * `buffer_memory_barriers`, Len: `buffer_memory_barrier_count`
  /// * `image_memory_barrier_count`, Optional: true
  /// * `image_memory_barriers`, Len: `image_memory_barrier_count`
  pub CmdWaitEvents: PFN_vkCmdWaitEvents,

  /// Khronos: [vkCmdWaitEvents2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html)
  /// * Queues: graphics, compute, decode, encode
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: synchronization
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `event_count`
  /// * `events`, Len: `event_count`
  /// * `dependency_infos`, Len: `event_count`
  pub CmdWaitEvents2: PFN_vkCmdWaitEvents2,

  /// Khronos: [vkCmdWriteAccelerationStructuresPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `acceleration_structure_count`
  /// * `acceleration_structures`, Len: `acceleration_structure_count`
  /// * `query_type`
  /// * `query_pool`
  /// * `first_query`
  pub CmdWriteAccelerationStructuresPropertiesKHR:
    PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,

  /// Khronos: [vkCmdWriteAccelerationStructuresPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `acceleration_structure_count`
  /// * `acceleration_structures`, Len: `acceleration_structure_count`
  /// * `query_type`
  /// * `query_pool`
  /// * `first_query`
  pub CmdWriteAccelerationStructuresPropertiesNV:
    PFN_vkCmdWriteAccelerationStructuresPropertiesNV,

  /// Khronos: [vkCmdWriteBufferMarker2AMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `stage`, Optional: true
  /// * `dst_buffer`
  /// * `dst_offset`
  /// * `marker`
  pub CmdWriteBufferMarker2AMD: PFN_vkCmdWriteBufferMarker2AMD,

  /// Khronos: [vkCmdWriteBufferMarkerAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html)
  /// * Queues: transfer, graphics, compute
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `pipeline_stage`, Optional: true
  /// * `dst_buffer`
  /// * `dst_offset`
  /// * `marker`
  pub CmdWriteBufferMarkerAMD: PFN_vkCmdWriteBufferMarkerAMD,

  /// Khronos: [vkCmdWriteMicromapsPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteMicromapsPropertiesEXT.html)
  /// * Queues: compute
  /// * Render Pass: outside
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * `command_buffer`, Extern Sync: true
  /// * `micromap_count`
  /// * `micromaps`, Len: `micromap_count`
  /// * `query_type`
  /// * `query_pool`
  /// * `first_query`
  pub CmdWriteMicromapsPropertiesEXT: PFN_vkCmdWriteMicromapsPropertiesEXT,

  /// Khronos: [vkCmdWriteTimestamp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html)
  /// * Queues: transfer, graphics, compute, decode, encode, opticalflow
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `pipeline_stage`
  /// * `query_pool`
  /// * `query`
  pub CmdWriteTimestamp: PFN_vkCmdWriteTimestamp,

  /// Khronos: [vkCmdWriteTimestamp2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html)
  /// * Queues: transfer, graphics, compute, decode, encode
  /// * Render Pass: both
  /// * Command Buffer Level: primary, secondary
  /// * Tasks: action
  /// * Video Coding: both
  /// * `command_buffer`, Extern Sync: true
  /// * `stage`, Optional: true
  /// * `query_pool`
  /// * `query`
  pub CmdWriteTimestamp2: PFN_vkCmdWriteTimestamp2,

  /// Khronos: [vkCompileDeferredNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `pipeline`
  /// * `shader`
  pub CompileDeferredNV: PFN_vkCompileDeferredNV,

  /// Khronos: [vkCopyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_OPERATION_DEFERRED_KHR`,
  ///   `VK_OPERATION_NOT_DEFERRED_KHR`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `deferred_operation`, Optional: true
  /// * `info`
  pub CopyAccelerationStructureKHR: PFN_vkCopyAccelerationStructureKHR,

  /// Khronos: [vkCopyMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapEXT.html)
  /// * Success: `VK_SUCCESS`, `VK_OPERATION_DEFERRED_KHR`,
  ///   `VK_OPERATION_NOT_DEFERRED_KHR`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `deferred_operation`, Optional: true
  /// * `info`
  pub CopyMicromapEXT: PFN_vkCopyMicromapEXT,

  /// Khronos: [vkCreateAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `acceleration_structure`
  pub CreateAccelerationStructureKHR: PFN_vkCreateAccelerationStructureKHR,

  /// Khronos: [vkCreateAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `acceleration_structure`
  pub CreateAccelerationStructureNV: PFN_vkCreateAccelerationStructureNV,

  /// Khronos: [vkCreateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `buffer`
  pub CreateBuffer: PFN_vkCreateBuffer,

  /// Khronos: [vkCreateBufferCollectionFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`,
  ///   `VK_ERROR_INVALID_EXTERNAL_HANDLE`, `VK_ERROR_INITIALIZATION_FAILED`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `collection`
  pub CreateBufferCollectionFUCHSIA: PFN_vkCreateBufferCollectionFUCHSIA,

  /// Khronos: [vkCreateBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `view`
  pub CreateBufferView: PFN_vkCreateBufferView,

  /// Khronos: [vkCreateCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `command_pool`
  pub CreateCommandPool: PFN_vkCreateCommandPool,

  /// Khronos: [vkCreateComputePipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html)
  /// * Success: `VK_SUCCESS`, `VK_PIPELINE_COMPILE_REQUIRED_EXT`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_SHADER_NV`
  /// * `device`
  /// * `pipeline_cache`, Optional: true
  /// * `create_info_count`
  /// * `create_infos`, Len: `create_info_count`
  /// * `allocator`, Optional: true
  /// * `pipelines`, Len: `create_info_count`
  pub CreateComputePipelines: PFN_vkCreateComputePipelines,

  /// Khronos: [vkCreateCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INITIALIZATION_FAILED`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `function`
  pub CreateCuFunctionNVX: PFN_vkCreateCuFunctionNVX,

  /// Khronos: [vkCreateCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INITIALIZATION_FAILED`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `module`
  pub CreateCuModuleNVX: PFN_vkCreateCuModuleNVX,

  /// Khronos: [vkCreateDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `allocator`, Optional: true
  /// * `deferred_operation`
  pub CreateDeferredOperationKHR: PFN_vkCreateDeferredOperationKHR,

  /// Khronos: [vkCreateDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_FRAGMENTATION_EXT`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `descriptor_pool`
  pub CreateDescriptorPool: PFN_vkCreateDescriptorPool,

  /// Khronos: [vkCreateDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `set_layout`
  pub CreateDescriptorSetLayout: PFN_vkCreateDescriptorSetLayout,

  /// Khronos: [vkCreateDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `descriptor_update_template`
  pub CreateDescriptorUpdateTemplate: PFN_vkCreateDescriptorUpdateTemplate,

  /// Khronos: [vkCreateEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `event`
  pub CreateEvent: PFN_vkCreateEvent,

  /// Khronos: [vkCreateFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `fence`
  pub CreateFence: PFN_vkCreateFence,

  /// Khronos: [vkCreateFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `framebuffer`
  pub CreateFramebuffer: PFN_vkCreateFramebuffer,

  /// Khronos: [vkCreateGraphicsPipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html)
  /// * Success: `VK_SUCCESS`, `VK_PIPELINE_COMPILE_REQUIRED_EXT`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_SHADER_NV`
  /// * `device`
  /// * `pipeline_cache`, Optional: true
  /// * `create_info_count`
  /// * `create_infos`, Len: `create_info_count`
  /// * `allocator`, Optional: true
  /// * `pipelines`, Len: `create_info_count`
  pub CreateGraphicsPipelines: PFN_vkCreateGraphicsPipelines,

  /// Khronos: [vkCreateImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_COMPRESSION_EXHAUSTED_EXT`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `image`
  pub CreateImage: PFN_vkCreateImage,

  /// Khronos: [vkCreateImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `view`
  pub CreateImageView: PFN_vkCreateImageView,

  /// Khronos: [vkCreateIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `indirect_commands_layout`
  pub CreateIndirectCommandsLayoutNV: PFN_vkCreateIndirectCommandsLayoutNV,

  /// Khronos: [vkCreateMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMicromapEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `micromap`
  pub CreateMicromapEXT: PFN_vkCreateMicromapEXT,

  /// Khronos: [vkCreateOpticalFlowSessionNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateOpticalFlowSessionNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INITIALIZATION_FAILED`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `session`
  pub CreateOpticalFlowSessionNV: PFN_vkCreateOpticalFlowSessionNV,

  /// Khronos: [vkCreatePipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `pipeline_cache`
  pub CreatePipelineCache: PFN_vkCreatePipelineCache,

  /// Khronos: [vkCreatePipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `pipeline_layout`
  pub CreatePipelineLayout: PFN_vkCreatePipelineLayout,

  /// Khronos: [vkCreatePrivateDataSlot](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `private_data_slot`
  pub CreatePrivateDataSlot: PFN_vkCreatePrivateDataSlot,

  /// Khronos: [vkCreateQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `query_pool`
  pub CreateQueryPool: PFN_vkCreateQueryPool,

  /// Khronos: [vkCreateRayTracingPipelinesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_OPERATION_DEFERRED_KHR`,
  ///   `VK_OPERATION_NOT_DEFERRED_KHR`, `VK_PIPELINE_COMPILE_REQUIRED_EXT`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`
  /// * `device`
  /// * `deferred_operation`, Optional: true
  /// * `pipeline_cache`, Optional: true
  /// * `create_info_count`
  /// * `create_infos`, Len: `create_info_count`
  /// * `allocator`, Optional: true
  /// * `pipelines`, Len: `create_info_count`
  pub CreateRayTracingPipelinesKHR: PFN_vkCreateRayTracingPipelinesKHR,

  /// Khronos: [vkCreateRayTracingPipelinesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html)
  /// * Success: `VK_SUCCESS`, `VK_PIPELINE_COMPILE_REQUIRED_EXT`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_SHADER_NV`
  /// * `device`
  /// * `pipeline_cache`, Optional: true
  /// * `create_info_count`
  /// * `create_infos`, Len: `create_info_count`
  /// * `allocator`, Optional: true
  /// * `pipelines`, Len: `create_info_count`
  pub CreateRayTracingPipelinesNV: PFN_vkCreateRayTracingPipelinesNV,

  /// Khronos: [vkCreateRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `render_pass`
  pub CreateRenderPass: PFN_vkCreateRenderPass,

  /// Khronos: [vkCreateRenderPass2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `render_pass`
  pub CreateRenderPass2: PFN_vkCreateRenderPass2,

  /// Khronos: [vkCreateSampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `sampler`
  pub CreateSampler: PFN_vkCreateSampler,

  /// Khronos: [vkCreateSamplerYcbcrConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `ycbcr_conversion`
  pub CreateSamplerYcbcrConversion: PFN_vkCreateSamplerYcbcrConversion,

  /// Khronos: [vkCreateSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `semaphore`
  pub CreateSemaphore: PFN_vkCreateSemaphore,

  /// Khronos: [vkCreateShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INVALID_SHADER_NV`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `shader_module`
  pub CreateShaderModule: PFN_vkCreateShaderModule,

  /// Khronos: [vkCreateSharedSwapchainsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INCOMPATIBLE_DISPLAY_KHR`, `VK_ERROR_DEVICE_LOST`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `device`
  /// * `swapchain_count`
  /// * `create_infos`, Extern Sync:
  ///   pCreateInfos[].surface,pCreateInfos[].oldSwapchain, Len:
  ///   `swapchain_count`
  /// * `allocator`, Optional: true
  /// * `swapchains`, Len: `swapchain_count`
  pub CreateSharedSwapchainsKHR: PFN_vkCreateSharedSwapchainsKHR,

  /// Khronos: [vkCreateSwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`, `VK_ERROR_SURFACE_LOST_KHR`,
  ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`, `VK_ERROR_INITIALIZATION_FAILED`,
  ///   `VK_ERROR_COMPRESSION_EXHAUSTED_EXT`
  /// * `device`
  /// * `create_info`, Extern Sync:
  ///   pCreateInfo-&gt;surface,pCreateInfo-&gt;oldSwapchain
  /// * `allocator`, Optional: true
  /// * `swapchain`
  pub CreateSwapchainKHR: PFN_vkCreateSwapchainKHR,

  /// Khronos: [vkCreateValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `validation_cache`
  pub CreateValidationCacheEXT: PFN_vkCreateValidationCacheEXT,

  /// Khronos: [vkCreateVideoSessionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`,
  ///   `VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `video_session`
  pub CreateVideoSessionKHR: PFN_vkCreateVideoSessionKHR,

  /// Khronos: [vkCreateVideoSessionParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_INITIALIZATION_FAILED`
  /// * `device`
  /// * `create_info`
  /// * `allocator`, Optional: true
  /// * `video_session_parameters`
  pub CreateVideoSessionParametersKHR: PFN_vkCreateVideoSessionParametersKHR,

  /// Khronos: [vkDebugMarkerSetObjectNameEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `name_info`, Extern Sync: pNameInfo-&gt;object
  pub DebugMarkerSetObjectNameEXT: PFN_vkDebugMarkerSetObjectNameEXT,

  /// Khronos: [vkDebugMarkerSetObjectTagEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `tag_info`, Extern Sync: pTagInfo-&gt;object
  pub DebugMarkerSetObjectTagEXT: PFN_vkDebugMarkerSetObjectTagEXT,

  /// Khronos: [vkDeferredOperationJoinKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_THREAD_DONE_KHR`, `VK_THREAD_IDLE_KHR`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `operation`
  pub DeferredOperationJoinKHR: PFN_vkDeferredOperationJoinKHR,

  /// Khronos: [vkDestroyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html)
  /// * `device`
  /// * `acceleration_structure`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyAccelerationStructureKHR: PFN_vkDestroyAccelerationStructureKHR,

  /// Khronos: [vkDestroyAccelerationStructureNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html)
  /// * `device`
  /// * `acceleration_structure`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyAccelerationStructureNV: PFN_vkDestroyAccelerationStructureNV,

  /// Khronos: [vkDestroyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)
  /// * `device`
  /// * `buffer`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyBuffer: PFN_vkDestroyBuffer,

  /// Khronos: [vkDestroyBufferCollectionFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html)
  /// * `device`
  /// * `collection`
  /// * `allocator`, Optional: true
  pub DestroyBufferCollectionFUCHSIA: PFN_vkDestroyBufferCollectionFUCHSIA,

  /// Khronos: [vkDestroyBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html)
  /// * `device`
  /// * `buffer_view`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyBufferView: PFN_vkDestroyBufferView,

  /// Khronos: [vkDestroyCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)
  /// * `device`
  /// * `command_pool`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyCommandPool: PFN_vkDestroyCommandPool,

  /// Khronos: [vkDestroyCuFunctionNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html)
  /// * `device`
  /// * `function`
  /// * `allocator`, Optional: true
  pub DestroyCuFunctionNVX: PFN_vkDestroyCuFunctionNVX,

  /// Khronos: [vkDestroyCuModuleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html)
  /// * `device`
  /// * `module`
  /// * `allocator`, Optional: true
  pub DestroyCuModuleNVX: PFN_vkDestroyCuModuleNVX,

  /// Khronos: [vkDestroyDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html)
  /// * `device`
  /// * `operation`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyDeferredOperationKHR: PFN_vkDestroyDeferredOperationKHR,

  /// Khronos: [vkDestroyDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html)
  /// * `device`
  /// * `descriptor_pool`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyDescriptorPool: PFN_vkDestroyDescriptorPool,

  /// Khronos: [vkDestroyDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)
  /// * `device`
  /// * `descriptor_set_layout`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyDescriptorSetLayout: PFN_vkDestroyDescriptorSetLayout,

  /// Khronos: [vkDestroyDescriptorUpdateTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html)
  /// * `device`
  /// * `descriptor_update_template`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyDescriptorUpdateTemplate: PFN_vkDestroyDescriptorUpdateTemplate,

  /// Khronos: [vkDestroyDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)
  /// * Implicit Extern Sync: all [VkQueue] objects created from `device`
  /// * `device`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyDevice: PFN_vkDestroyDevice,

  /// Khronos: [vkDestroyEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html)
  /// * `device`
  /// * `event`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyEvent: PFN_vkDestroyEvent,

  /// Khronos: [vkDestroyFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html)
  /// * `device`
  /// * `fence`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyFence: PFN_vkDestroyFence,

  /// Khronos: [vkDestroyFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html)
  /// * `device`
  /// * `framebuffer`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyFramebuffer: PFN_vkDestroyFramebuffer,

  /// Khronos: [vkDestroyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html)
  /// * `device`
  /// * `image`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyImage: PFN_vkDestroyImage,

  /// Khronos: [vkDestroyImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html)
  /// * `device`
  /// * `image_view`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyImageView: PFN_vkDestroyImageView,

  /// Khronos: [vkDestroyIndirectCommandsLayoutNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html)
  /// * `device`
  /// * `indirect_commands_layout`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyIndirectCommandsLayoutNV: PFN_vkDestroyIndirectCommandsLayoutNV,

  /// Khronos: [vkDestroyMicromapEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyMicromapEXT.html)
  /// * `device`
  /// * `micromap`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyMicromapEXT: PFN_vkDestroyMicromapEXT,

  /// Khronos: [vkDestroyOpticalFlowSessionNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyOpticalFlowSessionNV.html)
  /// * `device`
  /// * `session`
  /// * `allocator`, Optional: true
  pub DestroyOpticalFlowSessionNV: PFN_vkDestroyOpticalFlowSessionNV,

  /// Khronos: [vkDestroyPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html)
  /// * `device`
  /// * `pipeline`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyPipeline: PFN_vkDestroyPipeline,

  /// Khronos: [vkDestroyPipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html)
  /// * `device`
  /// * `pipeline_cache`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyPipelineCache: PFN_vkDestroyPipelineCache,

  /// Khronos: [vkDestroyPipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)
  /// * `device`
  /// * `pipeline_layout`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyPipelineLayout: PFN_vkDestroyPipelineLayout,

  /// Khronos: [vkDestroyPrivateDataSlot](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html)
  /// * `device`
  /// * `private_data_slot`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyPrivateDataSlot: PFN_vkDestroyPrivateDataSlot,

  /// Khronos: [vkDestroyQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html)
  /// * `device`
  /// * `query_pool`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyQueryPool: PFN_vkDestroyQueryPool,

  /// Khronos: [vkDestroyRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html)
  /// * `device`
  /// * `render_pass`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyRenderPass: PFN_vkDestroyRenderPass,

  /// Khronos: [vkDestroySampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html)
  /// * `device`
  /// * `sampler`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroySampler: PFN_vkDestroySampler,

  /// Khronos: [vkDestroySamplerYcbcrConversion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html)
  /// * `device`
  /// * `ycbcr_conversion`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroySamplerYcbcrConversion: PFN_vkDestroySamplerYcbcrConversion,

  /// Khronos: [vkDestroySemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)
  /// * `device`
  /// * `semaphore`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroySemaphore: PFN_vkDestroySemaphore,

  /// Khronos: [vkDestroySemaphoreSciSyncPoolNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphoreSciSyncPoolNV.html)
  /// * `device`
  /// * `semaphore_pool`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroySemaphoreSciSyncPoolNV: PFN_vkDestroySemaphoreSciSyncPoolNV,

  /// Khronos: [vkDestroyShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html)
  /// * `device`
  /// * `shader_module`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyShaderModule: PFN_vkDestroyShaderModule,

  /// Khronos: [vkDestroySwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html)
  /// * `device`
  /// * `swapchain`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroySwapchainKHR: PFN_vkDestroySwapchainKHR,

  /// Khronos: [vkDestroyValidationCacheEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html)
  /// * `device`
  /// * `validation_cache`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyValidationCacheEXT: PFN_vkDestroyValidationCacheEXT,

  /// Khronos: [vkDestroyVideoSessionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html)
  /// * `device`
  /// * `video_session`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyVideoSessionKHR: PFN_vkDestroyVideoSessionKHR,

  /// Khronos: [vkDestroyVideoSessionParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionParametersKHR.html)
  /// * `device`
  /// * `video_session_parameters`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub DestroyVideoSessionParametersKHR: PFN_vkDestroyVideoSessionParametersKHR,

  /// Khronos: [vkDeviceWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * Implicit Extern Sync: all [VkQueue] objects created from `device`
  /// * `device`
  pub DeviceWaitIdle: PFN_vkDeviceWaitIdle,

  /// Khronos: [vkDisplayPowerControlEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `display`
  /// * `display_power_info`
  pub DisplayPowerControlEXT: PFN_vkDisplayPowerControlEXT,

  /// Khronos: [vkEndCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * Implicit Extern Sync: the [VkCommandPool] that `commandBuffer` was
  ///   allocated from
  /// * `command_buffer`, Extern Sync: true
  pub EndCommandBuffer: PFN_vkEndCommandBuffer,

  /// Khronos: [vkExportMetalObjectsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkExportMetalObjectsEXT.html)
  /// * `device`
  /// * `metal_objects_info`
  pub ExportMetalObjectsEXT: PFN_vkExportMetalObjectsEXT,

  /// Khronos: [vkFlushMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `memory_range_count`
  /// * `memory_ranges`, Len: `memory_range_count`
  pub FlushMappedMemoryRanges: PFN_vkFlushMappedMemoryRanges,

  /// Khronos: [vkFreeCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)
  /// * `device`
  /// * `command_pool`, Extern Sync: true
  /// * `command_buffer_count`
  /// * `command_buffers`, Extern Sync: true, Len: `command_buffer_count`, true
  pub FreeCommandBuffers: PFN_vkFreeCommandBuffers,

  /// Khronos: [vkFreeDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html)
  /// * Success: `VK_SUCCESS`
  /// * `device`
  /// * `descriptor_pool`, Extern Sync: true
  /// * `descriptor_set_count`
  /// * `descriptor_sets`, Extern Sync: true, Len: `descriptor_set_count`, true
  pub FreeDescriptorSets: PFN_vkFreeDescriptorSets,

  /// Khronos: [vkFreeMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)
  /// * `device`
  /// * `memory`, Optional: true, Extern Sync: true
  /// * `allocator`, Optional: true
  pub FreeMemory: PFN_vkFreeMemory,

  /// Khronos: [vkGetAccelerationStructureDeviceAddressKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html)
  /// * `device`
  /// * `info`
  pub GetAccelerationStructureDeviceAddressKHR:
    PFN_vkGetAccelerationStructureDeviceAddressKHR,

  /// Khronos: [vkGetAccelerationStructureHandleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `acceleration_structure`
  /// * `data_size`
  /// * `data`, Len: `data_size`
  pub GetAccelerationStructureHandleNV: PFN_vkGetAccelerationStructureHandleNV,

  /// Khronos: [vkGetAccelerationStructureMemoryRequirementsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html)
  /// * `device`
  /// * `info`
  /// * `memory_requirements`
  pub GetAccelerationStructureMemoryRequirementsNV:
    PFN_vkGetAccelerationStructureMemoryRequirementsNV,

  /// Khronos: [vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `info`
  /// * `data`
  pub GetAccelerationStructureOpaqueCaptureDescriptorDataEXT:
    PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT,

  /// Khronos: [vkGetAndroidHardwareBufferPropertiesANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`,
  ///   `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`
  /// * `device`
  /// * `buffer`
  /// * `properties`
  pub GetAndroidHardwareBufferPropertiesANDROID:
    PFN_vkGetAndroidHardwareBufferPropertiesANDROID,

  /// Khronos: [vkGetBufferCollectionPropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INITIALIZATION_FAILED`
  /// * `device`
  /// * `collection`
  /// * `properties`
  pub GetBufferCollectionPropertiesFUCHSIA: PFN_vkGetBufferCollectionPropertiesFUCHSIA,

  /// Khronos: [vkGetBufferDeviceAddress](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html)
  /// * `device`
  /// * `info`
  pub GetBufferDeviceAddress: PFN_vkGetBufferDeviceAddress,

  /// Khronos: [vkGetBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html)
  /// * `device`
  /// * `buffer`
  /// * `memory_requirements`
  pub GetBufferMemoryRequirements: PFN_vkGetBufferMemoryRequirements,

  /// Khronos: [vkGetBufferMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html)
  /// * `device`
  /// * `info`
  /// * `memory_requirements`
  pub GetBufferMemoryRequirements2: PFN_vkGetBufferMemoryRequirements2,

  /// Khronos: [vkGetBufferOpaqueCaptureAddress](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html)
  /// * `device`
  /// * `info`
  pub GetBufferOpaqueCaptureAddress: PFN_vkGetBufferOpaqueCaptureAddress,

  /// Khronos: [vkGetBufferOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureDescriptorDataEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `info`
  /// * `data`
  pub GetBufferOpaqueCaptureDescriptorDataEXT:
    PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT,

  /// Khronos: [vkGetCalibratedTimestampsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `timestamp_count`
  /// * `timestamp_infos`, Len: `timestamp_count`
  /// * `timestamps`, Len: `timestamp_count`
  /// * `max_deviation`
  pub GetCalibratedTimestampsEXT: PFN_vkGetCalibratedTimestampsEXT,

  /// Khronos: [vkGetDeferredOperationMaxConcurrencyKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html)
  /// * `device`
  /// * `operation`
  pub GetDeferredOperationMaxConcurrencyKHR: PFN_vkGetDeferredOperationMaxConcurrencyKHR,

  /// Khronos: [vkGetDeferredOperationResultKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_NOT_READY`
  /// * `device`
  /// * `operation`
  pub GetDeferredOperationResultKHR: PFN_vkGetDeferredOperationResultKHR,

  /// Khronos: [vkGetDescriptorSetHostMappingVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetHostMappingVALVE.html)
  /// * `device`
  /// * `descriptor_set`
  /// * `data`
  pub GetDescriptorSetHostMappingVALVE: PFN_vkGetDescriptorSetHostMappingVALVE,

  /// Khronos: [vkGetDescriptorSetLayoutBindingOffsetEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html)
  /// * `device`
  /// * `layout`
  /// * `binding`
  /// * `offset`
  pub GetDescriptorSetLayoutBindingOffsetEXT:
    PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,

  /// Khronos: [vkGetDescriptorSetLayoutHostMappingInfoVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html)
  /// * `device`
  /// * `binding_reference`
  /// * `host_mapping`
  pub GetDescriptorSetLayoutHostMappingInfoVALVE:
    PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,

  /// Khronos: [vkGetDescriptorSetLayoutSizeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSizeEXT.html)
  /// * `device`
  /// * `layout`
  /// * `layout_size_in_bytes`
  pub GetDescriptorSetLayoutSizeEXT: PFN_vkGetDescriptorSetLayoutSizeEXT,

  /// Khronos: [vkGetDescriptorSetLayoutSupport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html)
  /// * `device`
  /// * `create_info`
  /// * `support`
  pub GetDescriptorSetLayoutSupport: PFN_vkGetDescriptorSetLayoutSupport,

  /// Khronos: [vkGetDeviceAccelerationStructureCompatibilityKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html)
  /// * `device`
  /// * `version_info`
  /// * `compatibility`
  pub GetDeviceAccelerationStructureCompatibilityKHR:
    PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,

  /// Khronos: [vkGetDeviceBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html)
  /// * `device`
  /// * `info`
  /// * `memory_requirements`
  pub GetDeviceBufferMemoryRequirements: PFN_vkGetDeviceBufferMemoryRequirements,

  /// Khronos: [vkGetDeviceFaultInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceFaultInfoEXT.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `fault_counts`
  /// * `fault_info`, Optional: true
  pub GetDeviceFaultInfoEXT: PFN_vkGetDeviceFaultInfoEXT,

  /// Khronos: [vkGetDeviceGroupPeerMemoryFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html)
  /// * `device`
  /// * `heap_index`
  /// * `local_device_index`
  /// * `remote_device_index`
  /// * `peer_memory_features`
  pub GetDeviceGroupPeerMemoryFeatures: PFN_vkGetDeviceGroupPeerMemoryFeatures,

  /// Khronos: [vkGetDeviceGroupPresentCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `device_group_present_capabilities`
  pub GetDeviceGroupPresentCapabilitiesKHR: PFN_vkGetDeviceGroupPresentCapabilitiesKHR,

  /// Khronos: [vkGetDeviceGroupSurfacePresentModes2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `device`
  /// * `surface_info`
  /// * `modes`, Optional: false,true
  pub GetDeviceGroupSurfacePresentModes2EXT: PFN_vkGetDeviceGroupSurfacePresentModes2EXT,

  /// Khronos: [vkGetDeviceGroupSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `device`
  /// * `surface`, Extern Sync: true
  /// * `modes`, Optional: false,true
  pub GetDeviceGroupSurfacePresentModesKHR: PFN_vkGetDeviceGroupSurfacePresentModesKHR,

  /// Khronos: [vkGetDeviceImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html)
  /// * `device`
  /// * `info`
  /// * `memory_requirements`
  pub GetDeviceImageMemoryRequirements: PFN_vkGetDeviceImageMemoryRequirements,

  /// Khronos: [vkGetDeviceImageSparseMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html)
  /// * `device`
  /// * `info`
  /// * `sparse_memory_requirement_count`, Optional: false,true
  /// * `sparse_memory_requirements`, Optional: true, Len:
  ///   `sparse_memory_requirement_count`
  pub GetDeviceImageSparseMemoryRequirements:
    PFN_vkGetDeviceImageSparseMemoryRequirements,

  /// Khronos: [vkGetDeviceMemoryCommitment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html)
  /// * `device`
  /// * `memory`
  /// * `committed_memory_in_bytes`
  pub GetDeviceMemoryCommitment: PFN_vkGetDeviceMemoryCommitment,

  /// Khronos: [vkGetDeviceMemoryOpaqueCaptureAddress](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html)
  /// * `device`
  /// * `info`
  pub GetDeviceMemoryOpaqueCaptureAddress: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,

  /// Khronos: [vkGetDeviceMicromapCompatibilityEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMicromapCompatibilityEXT.html)
  /// * `device`
  /// * `version_info`
  /// * `compatibility`
  pub GetDeviceMicromapCompatibilityEXT: PFN_vkGetDeviceMicromapCompatibilityEXT,

  /// Khronos: [vkGetDeviceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)
  /// * `device`
  /// * `name`, Len: `null_terminated`
  pub GetDeviceProcAddr: PFN_vkGetDeviceProcAddr,

  /// Khronos: [vkGetDeviceQueue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html)
  /// * `device`
  /// * `queue_family_index`
  /// * `queue_index`
  /// * `queue`
  pub GetDeviceQueue: PFN_vkGetDeviceQueue,

  /// Khronos: [vkGetDeviceQueue2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html)
  /// * `device`
  /// * `queue_info`
  /// * `queue`
  pub GetDeviceQueue2: PFN_vkGetDeviceQueue2,

  /// Khronos: [vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `device`
  /// * `renderpass`
  /// * `max_workgroup_size`
  pub GetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI:
    PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,

  /// Khronos: [vkGetDynamicRenderingTilePropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html)
  /// * Success: `VK_SUCCESS`
  /// * `device`
  /// * `rendering_info`
  /// * `properties`
  pub GetDynamicRenderingTilePropertiesQCOM: PFN_vkGetDynamicRenderingTilePropertiesQCOM,

  /// Khronos: [vkGetEventStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html)
  /// * Success: `VK_EVENT_SET`, `VK_EVENT_RESET`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `device`
  /// * `event`
  pub GetEventStatus: PFN_vkGetEventStatus,

  /// Khronos: [vkGetFenceFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `get_fd_info`
  /// * `fd`
  pub GetFenceFdKHR: PFN_vkGetFenceFdKHR,

  /// Khronos: [vkGetFenceSciSyncFenceNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceSciSyncFenceNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INVALID_EXTERNAL_HANDLE`, `VK_ERROR_NOT_PERMITTED_EXT`
  /// * `device`
  /// * `get_sci_sync_handle_info`
  /// * `handle`
  pub GetFenceSciSyncFenceNV: PFN_vkGetFenceSciSyncFenceNV,

  /// Khronos: [vkGetFenceSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceSciSyncObjNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INVALID_EXTERNAL_HANDLE`, `VK_ERROR_NOT_PERMITTED_EXT`
  /// * `device`
  /// * `get_sci_sync_handle_info`
  /// * `handle`
  pub GetFenceSciSyncObjNV: PFN_vkGetFenceSciSyncObjNV,

  /// Khronos: [vkGetFenceStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html)
  /// * Success: `VK_SUCCESS`, `VK_NOT_READY`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `device`
  /// * `fence`
  pub GetFenceStatus: PFN_vkGetFenceStatus,

  /// Khronos: [vkGetFenceWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `get_win_32_handle_info`
  /// * `handle`
  pub GetFenceWin32HandleKHR: PFN_vkGetFenceWin32HandleKHR,

  /// Khronos: [vkGetFramebufferTilePropertiesQCOM](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFramebufferTilePropertiesQCOM.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * `device`
  /// * `framebuffer`
  /// * `properties_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `properties_count`
  pub GetFramebufferTilePropertiesQCOM: PFN_vkGetFramebufferTilePropertiesQCOM,

  /// Khronos: [vkGetGeneratedCommandsMemoryRequirementsNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html)
  /// * `device`
  /// * `info`
  /// * `memory_requirements`
  pub GetGeneratedCommandsMemoryRequirementsNV:
    PFN_vkGetGeneratedCommandsMemoryRequirementsNV,

  /// Khronos: [vkGetImageDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `image`
  /// * `properties`
  pub GetImageDrmFormatModifierPropertiesEXT:
    PFN_vkGetImageDrmFormatModifierPropertiesEXT,

  /// Khronos: [vkGetImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html)
  /// * `device`
  /// * `image`
  /// * `memory_requirements`
  pub GetImageMemoryRequirements: PFN_vkGetImageMemoryRequirements,

  /// Khronos: [vkGetImageMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html)
  /// * `device`
  /// * `info`
  /// * `memory_requirements`
  pub GetImageMemoryRequirements2: PFN_vkGetImageMemoryRequirements2,

  /// Khronos: [vkGetImageOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageOpaqueCaptureDescriptorDataEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `info`
  /// * `data`
  pub GetImageOpaqueCaptureDescriptorDataEXT:
    PFN_vkGetImageOpaqueCaptureDescriptorDataEXT,

  /// Khronos: [vkGetImageSparseMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html)
  /// * `device`
  /// * `image`
  /// * `sparse_memory_requirement_count`, Optional: false,true
  /// * `sparse_memory_requirements`, Optional: true, Len:
  ///   `sparse_memory_requirement_count`
  pub GetImageSparseMemoryRequirements: PFN_vkGetImageSparseMemoryRequirements,

  /// Khronos: [vkGetImageSparseMemoryRequirements2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html)
  /// * `device`
  /// * `info`
  /// * `sparse_memory_requirement_count`, Optional: false,true
  /// * `sparse_memory_requirements`, Optional: true, Len:
  ///   `sparse_memory_requirement_count`
  pub GetImageSparseMemoryRequirements2: PFN_vkGetImageSparseMemoryRequirements2,

  /// Khronos: [vkGetImageSubresourceLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html)
  /// * `device`
  /// * `image`
  /// * `subresource`
  /// * `layout`
  pub GetImageSubresourceLayout: PFN_vkGetImageSubresourceLayout,

  /// Khronos: [vkGetImageSubresourceLayout2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html)
  /// * `device`
  /// * `image`
  /// * `subresource`
  /// * `layout`
  pub GetImageSubresourceLayout2EXT: PFN_vkGetImageSubresourceLayout2EXT,

  /// Khronos: [vkGetImageViewAddressNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_UNKNOWN`
  /// * `device`
  /// * `image_view`
  /// * `properties`
  pub GetImageViewAddressNVX: PFN_vkGetImageViewAddressNVX,

  /// Khronos: [vkGetImageViewHandleNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html)
  /// * `device`
  /// * `info`
  pub GetImageViewHandleNVX: PFN_vkGetImageViewHandleNVX,

  /// Khronos: [vkGetImageViewOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `info`
  /// * `data`
  pub GetImageViewOpaqueCaptureDescriptorDataEXT:
    PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT,

  /// Khronos: [vkGetMemoryAndroidHardwareBufferANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `info`
  /// * `buffer`
  pub GetMemoryAndroidHardwareBufferANDROID: PFN_vkGetMemoryAndroidHardwareBufferANDROID,

  /// Khronos: [vkGetMemoryFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `get_fd_info`
  /// * `fd`
  pub GetMemoryFdKHR: PFN_vkGetMemoryFdKHR,

  /// Khronos: [vkGetMemoryFdPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `handle_type`
  /// * `fd`
  /// * `memory_fd_properties`
  pub GetMemoryFdPropertiesKHR: PFN_vkGetMemoryFdPropertiesKHR,

  /// Khronos: [vkGetMemoryHostPointerPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `handle_type`
  /// * `host_pointer`, Optional: false
  /// * `memory_host_pointer_properties`
  pub GetMemoryHostPointerPropertiesEXT: PFN_vkGetMemoryHostPointerPropertiesEXT,

  /// Khronos: [vkGetMemoryRemoteAddressNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `memory_get_remote_address_info`
  /// * `address`
  pub GetMemoryRemoteAddressNV: PFN_vkGetMemoryRemoteAddressNV,

  /// Khronos: [vkGetMemoryWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `get_win_32_handle_info`
  /// * `handle`
  pub GetMemoryWin32HandleKHR: PFN_vkGetMemoryWin32HandleKHR,

  /// Khronos: [vkGetMemoryWin32HandleNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `memory`
  /// * `handle_type`
  /// * `handle`
  pub GetMemoryWin32HandleNV: PFN_vkGetMemoryWin32HandleNV,

  /// Khronos: [vkGetMemoryWin32HandlePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `handle_type`
  /// * `handle`
  /// * `memory_win_32_handle_properties`
  pub GetMemoryWin32HandlePropertiesKHR: PFN_vkGetMemoryWin32HandlePropertiesKHR,

  /// Khronos: [vkGetMemoryZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `get_zircon_handle_info`
  /// * `zircon_handle`
  pub GetMemoryZirconHandleFUCHSIA: PFN_vkGetMemoryZirconHandleFUCHSIA,

  /// Khronos: [vkGetMemoryZirconHandlePropertiesFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `handle_type`
  /// * `zircon_handle`
  /// * `memory_zircon_handle_properties`
  pub GetMemoryZirconHandlePropertiesFUCHSIA:
    PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,

  /// Khronos: [vkGetPastPresentationTimingGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_DEVICE_LOST`,
  ///   `VK_ERROR_OUT_OF_DATE_KHR`, `VK_ERROR_SURFACE_LOST_KHR`
  /// * `device`
  /// * `swapchain`, Extern Sync: true
  /// * `presentation_timing_count`, Optional: false,true
  /// * `presentation_timings`, Optional: true, Len: `presentation_timing_count`
  pub GetPastPresentationTimingGOOGLE: PFN_vkGetPastPresentationTimingGOOGLE,

  /// Khronos: [vkGetPipelineCacheData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `pipeline_cache`
  /// * `data_size`, Optional: false,true
  /// * `data`, Optional: true, Len: `data_size`
  pub GetPipelineCacheData: PFN_vkGetPipelineCacheData,

  /// Khronos: [vkGetPipelineExecutableInternalRepresentationsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `executable_info`
  /// * `internal_representation_count`, Optional: false,true
  /// * `internal_representations`, Optional: true, Len:
  ///   `internal_representation_count`
  pub GetPipelineExecutableInternalRepresentationsKHR:
    PFN_vkGetPipelineExecutableInternalRepresentationsKHR,

  /// Khronos: [vkGetPipelineExecutablePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `pipeline_info`
  /// * `executable_count`, Optional: false,true
  /// * `properties`, Optional: true, Len: `executable_count`
  pub GetPipelineExecutablePropertiesKHR: PFN_vkGetPipelineExecutablePropertiesKHR,

  /// Khronos: [vkGetPipelinePropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `pipeline_info`
  /// * `pipeline_properties`, true, Valid Structs:
  ///   VkPipelinePropertiesIdentifierEXT
  pub GetPipelinePropertiesEXT: PFN_vkGetPipelinePropertiesEXT,

  /// Khronos: [vkGetPrivateData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html)
  /// * `device`
  /// * `object_type`
  /// * `object_handle`, Object Type: objectType
  /// * `private_data_slot`
  /// * `data`
  pub GetPrivateData: PFN_vkGetPrivateData,

  /// Khronos: [vkGetQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html)
  /// * Success: `VK_SUCCESS`, `VK_NOT_READY`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `device`
  /// * `query_pool`
  /// * `first_query`
  /// * `query_count`
  /// * `data_size`
  /// * `data`, Len: `data_size`
  /// * `stride`
  /// * `flags`, Optional: true
  pub GetQueryPoolResults: PFN_vkGetQueryPoolResults,

  /// Khronos: [vkGetQueueCheckpointData2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html)
  /// * `queue`
  /// * `checkpoint_data_count`, Optional: false,true
  /// * `checkpoint_data`, Optional: true, Len: `checkpoint_data_count`
  pub GetQueueCheckpointData2NV: PFN_vkGetQueueCheckpointData2NV,

  /// Khronos: [vkGetQueueCheckpointDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html)
  /// * `queue`
  /// * `checkpoint_data_count`, Optional: false,true
  /// * `checkpoint_data`, Optional: true, Len: `checkpoint_data_count`
  pub GetQueueCheckpointDataNV: PFN_vkGetQueueCheckpointDataNV,

  /// Khronos: [vkGetRayTracingCaptureReplayShaderGroupHandlesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `pipeline`
  /// * `first_group`
  /// * `group_count`
  /// * `data_size`
  /// * `data`, Len: `data_size`
  pub GetRayTracingCaptureReplayShaderGroupHandlesKHR:
    PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,

  /// Khronos: [vkGetRayTracingShaderGroupHandlesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `pipeline`
  /// * `first_group`
  /// * `group_count`
  /// * `data_size`
  /// * `data`, Len: `data_size`
  pub GetRayTracingShaderGroupHandlesKHR: PFN_vkGetRayTracingShaderGroupHandlesKHR,

  /// Khronos: [vkGetRayTracingShaderGroupStackSizeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html)
  /// * `device`
  /// * `pipeline`
  /// * `group`
  /// * `group_shader`
  pub GetRayTracingShaderGroupStackSizeKHR: PFN_vkGetRayTracingShaderGroupStackSizeKHR,

  /// Khronos: [vkGetRefreshCycleDurationGOOGLE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_DEVICE_LOST`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `device`
  /// * `swapchain`, Extern Sync: true
  /// * `display_timing_properties`
  pub GetRefreshCycleDurationGOOGLE: PFN_vkGetRefreshCycleDurationGOOGLE,

  /// Khronos: [vkGetRenderAreaGranularity](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html)
  /// * `device`
  /// * `render_pass`
  /// * `granularity`
  pub GetRenderAreaGranularity: PFN_vkGetRenderAreaGranularity,

  /// Khronos: [vkGetSamplerOpaqueCaptureDescriptorDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `info`
  /// * `data`
  pub GetSamplerOpaqueCaptureDescriptorDataEXT:
    PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT,

  /// Khronos: [vkGetSemaphoreCounterValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `device`
  /// * `semaphore`
  /// * `value`
  pub GetSemaphoreCounterValue: PFN_vkGetSemaphoreCounterValue,

  /// Khronos: [vkGetSemaphoreFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `get_fd_info`
  /// * `fd`
  pub GetSemaphoreFdKHR: PFN_vkGetSemaphoreFdKHR,

  /// Khronos: [vkGetSemaphoreSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreSciSyncObjNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INVALID_EXTERNAL_HANDLE`, `VK_ERROR_NOT_PERMITTED_EXT`
  /// * `device`
  /// * `get_sci_sync_info`
  /// * `handle`
  pub GetSemaphoreSciSyncObjNV: PFN_vkGetSemaphoreSciSyncObjNV,

  /// Khronos: [vkGetSemaphoreWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `get_win_32_handle_info`
  /// * `handle`
  pub GetSemaphoreWin32HandleKHR: PFN_vkGetSemaphoreWin32HandleKHR,

  /// Khronos: [vkGetSemaphoreZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `get_zircon_handle_info`
  /// * `zircon_handle`
  pub GetSemaphoreZirconHandleFUCHSIA: PFN_vkGetSemaphoreZirconHandleFUCHSIA,

  /// Khronos: [vkGetShaderInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_FEATURE_NOT_PRESENT`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `pipeline`
  /// * `shader_stage`
  /// * `info_type`
  /// * `info_size`, Optional: false,true
  /// * `info`, Optional: true, Len: `info_size`
  pub GetShaderInfoAMD: PFN_vkGetShaderInfoAMD,

  /// Khronos: [vkGetShaderModuleCreateInfoIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleCreateInfoIdentifierEXT.html)
  /// * `device`
  /// * `create_info`
  /// * `identifier`
  pub GetShaderModuleCreateInfoIdentifierEXT:
    PFN_vkGetShaderModuleCreateInfoIdentifierEXT,

  /// Khronos: [vkGetShaderModuleIdentifierEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleIdentifierEXT.html)
  /// * `device`
  /// * `shader_module`
  /// * `identifier`
  pub GetShaderModuleIdentifierEXT: PFN_vkGetShaderModuleIdentifierEXT,

  /// Khronos: [vkGetSwapchainCounterEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_DEVICE_LOST`,
  ///   `VK_ERROR_OUT_OF_DATE_KHR`
  /// * `device`
  /// * `swapchain`
  /// * `counter`
  /// * `counter_value`
  pub GetSwapchainCounterEXT: PFN_vkGetSwapchainCounterEXT,

  /// Khronos: [vkGetSwapchainGrallocUsage2ANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainGrallocUsage2ANDROID.html)
  /// * `device`
  /// * `format`
  /// * `image_usage`
  /// * `swapchain_image_usage`
  /// * `gralloc_consumer_usage`
  /// * `gralloc_producer_usage`
  pub GetSwapchainGrallocUsage2ANDROID: PFN_vkGetSwapchainGrallocUsage2ANDROID,

  /// Khronos: [vkGetSwapchainGrallocUsageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainGrallocUsageANDROID.html)
  /// * `device`
  /// * `format`
  /// * `image_usage`
  /// * `gralloc_usage`
  pub GetSwapchainGrallocUsageANDROID: PFN_vkGetSwapchainGrallocUsageANDROID,

  /// Khronos: [vkGetSwapchainImagesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `swapchain`
  /// * `swapchain_image_count`, Optional: false,true
  /// * `swapchain_images`, Optional: true, Len: `swapchain_image_count`
  pub GetSwapchainImagesKHR: PFN_vkGetSwapchainImagesKHR,

  /// Khronos: [vkGetSwapchainStatusKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_SUBOPTIMAL_KHR`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`, `VK_ERROR_OUT_OF_DATE_KHR`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`,
  ///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
  /// * `device`
  /// * `swapchain`, Extern Sync: true
  pub GetSwapchainStatusKHR: PFN_vkGetSwapchainStatusKHR,

  /// Khronos: [vkGetValidationCacheDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `validation_cache`
  /// * `data_size`, Optional: false,true
  /// * `data`, Optional: true, Len: `data_size`
  pub GetValidationCacheDataEXT: PFN_vkGetValidationCacheDataEXT,

  /// Khronos: [vkGetVideoSessionMemoryRequirementsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_INCOMPLETE`
  /// * `device`
  /// * `video_session`
  /// * `memory_requirements_count`, Optional: false,true
  /// * `memory_requirements`, Optional: true, Len: `memory_requirements_count`
  pub GetVideoSessionMemoryRequirementsKHR: PFN_vkGetVideoSessionMemoryRequirementsKHR,

  /// Khronos: [vkImportFenceFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `import_fence_fd_info`
  pub ImportFenceFdKHR: PFN_vkImportFenceFdKHR,

  /// Khronos: [vkImportFenceSciSyncFenceNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceSciSyncFenceNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INVALID_EXTERNAL_HANDLE`, `VK_ERROR_NOT_PERMITTED_EXT`
  /// * `device`
  /// * `import_fence_sci_sync_info`
  pub ImportFenceSciSyncFenceNV: PFN_vkImportFenceSciSyncFenceNV,

  /// Khronos: [vkImportFenceSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceSciSyncObjNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INVALID_EXTERNAL_HANDLE`, `VK_ERROR_NOT_PERMITTED_EXT`
  /// * `device`
  /// * `import_fence_sci_sync_info`
  pub ImportFenceSciSyncObjNV: PFN_vkImportFenceSciSyncObjNV,

  /// Khronos: [vkImportFenceWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `import_fence_win_32_handle_info`
  pub ImportFenceWin32HandleKHR: PFN_vkImportFenceWin32HandleKHR,

  /// Khronos: [vkImportSemaphoreFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `import_semaphore_fd_info`
  pub ImportSemaphoreFdKHR: PFN_vkImportSemaphoreFdKHR,

  /// Khronos: [vkImportSemaphoreSciSyncObjNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreSciSyncObjNV.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INVALID_EXTERNAL_HANDLE`, `VK_ERROR_NOT_PERMITTED_EXT`,
  ///   `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `import_semaphore_sci_sync_info`
  pub ImportSemaphoreSciSyncObjNV: PFN_vkImportSemaphoreSciSyncObjNV,

  /// Khronos: [vkImportSemaphoreWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `import_semaphore_win_32_handle_info`
  pub ImportSemaphoreWin32HandleKHR: PFN_vkImportSemaphoreWin32HandleKHR,

  /// Khronos: [vkImportSemaphoreZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  /// * `device`
  /// * `import_semaphore_zircon_handle_info`
  pub ImportSemaphoreZirconHandleFUCHSIA: PFN_vkImportSemaphoreZirconHandleFUCHSIA,

  /// Khronos: [vkInitializePerformanceApiINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `initialize_info`
  pub InitializePerformanceApiINTEL: PFN_vkInitializePerformanceApiINTEL,

  /// Khronos: [vkInvalidateMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `memory_range_count`
  /// * `memory_ranges`, Len: `memory_range_count`
  pub InvalidateMappedMemoryRanges: PFN_vkInvalidateMappedMemoryRanges,

  /// Khronos: [vkMapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_MEMORY_MAP_FAILED`
  /// * `device`
  /// * `memory`, Extern Sync: true
  /// * `offset`
  /// * `size`
  /// * `flags`, Optional: true
  /// * `data`, Optional: false,true
  pub MapMemory: PFN_vkMapMemory,

  /// Khronos: [vkMergePipelineCaches](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `dst_cache`, Extern Sync: true
  /// * `src_cache_count`
  /// * `src_caches`, Len: `src_cache_count`
  pub MergePipelineCaches: PFN_vkMergePipelineCaches,

  /// Khronos: [vkMergeValidationCachesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `dst_cache`, Extern Sync: true
  /// * `src_cache_count`
  /// * `src_caches`, Len: `src_cache_count`
  pub MergeValidationCachesEXT: PFN_vkMergeValidationCachesEXT,

  /// Khronos: [vkQueueBeginDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html)
  /// * `queue`
  /// * `label_info`
  pub QueueBeginDebugUtilsLabelEXT: PFN_vkQueueBeginDebugUtilsLabelEXT,

  /// Khronos: [vkQueueBindSparse](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * Queues: sparse_binding
  /// * `queue`, Extern Sync: true
  /// * `bind_info_count`, Optional: true
  /// * `bind_info`, Len: `bind_info_count`
  /// * `fence`, Optional: true, Extern Sync: true
  pub QueueBindSparse: PFN_vkQueueBindSparse,

  /// Khronos: [vkQueueEndDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html)
  /// * `queue`
  pub QueueEndDebugUtilsLabelEXT: PFN_vkQueueEndDebugUtilsLabelEXT,

  /// Khronos: [vkQueueInsertDebugUtilsLabelEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html)
  /// * `queue`
  /// * `label_info`
  pub QueueInsertDebugUtilsLabelEXT: PFN_vkQueueInsertDebugUtilsLabelEXT,

  /// Khronos: [vkQueuePresentKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_SUBOPTIMAL_KHR`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`, `VK_ERROR_OUT_OF_DATE_KHR`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`,
  ///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
  /// * `queue`, Extern Sync: true
  /// * `present_info`, Extern Sync:
  ///   pPresentInfo-&gt;pWaitSemaphores[],pPresentInfo-&gt;pSwapchains[]
  pub QueuePresentKHR: PFN_vkQueuePresentKHR,

  /// Khronos: [vkQueueSetPerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `queue`
  /// * `configuration`
  pub QueueSetPerformanceConfigurationINTEL: PFN_vkQueueSetPerformanceConfigurationINTEL,

  /// Khronos: [vkQueueSignalReleaseImageANDROID](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSignalReleaseImageANDROID.html)
  /// * `queue`
  /// * `wait_semaphore_count`
  /// * `wait_semaphores`, Len: `wait_semaphore_count`
  /// * `image`
  /// * `native_fence_fd`
  pub QueueSignalReleaseImageANDROID: PFN_vkQueueSignalReleaseImageANDROID,

  /// Khronos: [vkQueueSubmit](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `queue`, Extern Sync: true
  /// * `submit_count`, Optional: true
  /// * `submits`, Len: `submit_count`
  /// * `fence`, Optional: true, Extern Sync: true
  pub QueueSubmit: PFN_vkQueueSubmit,

  /// Khronos: [vkQueueSubmit2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `queue`, Extern Sync: true
  /// * `submit_count`, Optional: true
  /// * `submits`, Len: `submit_count`
  /// * `fence`, Optional: true, Extern Sync: true
  pub QueueSubmit2: PFN_vkQueueSubmit2,

  /// Khronos: [vkQueueWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `queue`, Extern Sync: true
  pub QueueWaitIdle: PFN_vkQueueWaitIdle,

  /// Khronos: [vkRegisterDeviceEventEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkRegisterDeviceEventEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `device_event_info`
  /// * `allocator`, Optional: true
  /// * `fence`
  pub RegisterDeviceEventEXT: PFN_vkRegisterDeviceEventEXT,

  /// Khronos: [vkRegisterDisplayEventEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `display`
  /// * `display_event_info`
  /// * `allocator`, Optional: true
  /// * `fence`
  pub RegisterDisplayEventEXT: PFN_vkRegisterDisplayEventEXT,

  /// Khronos: [vkReleaseFullScreenExclusiveModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`
  /// * `device`
  /// * `swapchain`
  pub ReleaseFullScreenExclusiveModeEXT: PFN_vkReleaseFullScreenExclusiveModeEXT,

  /// Khronos: [vkReleasePerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_TOO_MANY_OBJECTS`, `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `configuration`, Optional: true, Extern Sync: true
  pub ReleasePerformanceConfigurationINTEL: PFN_vkReleasePerformanceConfigurationINTEL,

  /// Khronos: [vkReleaseProfilingLockKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html)
  /// * `device`
  pub ReleaseProfilingLockKHR: PFN_vkReleaseProfilingLockKHR,

  /// Khronos: [vkReleaseSwapchainImagesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseSwapchainImagesEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_SURFACE_LOST_KHR`
  /// * `device`
  /// * `release_info`
  pub ReleaseSwapchainImagesEXT: PFN_vkReleaseSwapchainImagesEXT,

  /// Khronos: [vkResetCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * Implicit Extern Sync: the [VkCommandPool] that `commandBuffer` was
  ///   allocated from
  /// * `command_buffer`, Extern Sync: true
  /// * `flags`, Optional: true
  pub ResetCommandBuffer: PFN_vkResetCommandBuffer,

  /// Khronos: [vkResetCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `command_pool`, Extern Sync: true
  /// * `flags`, Optional: true
  pub ResetCommandPool: PFN_vkResetCommandPool,

  /// Khronos: [vkResetDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html)
  /// * Success: `VK_SUCCESS`
  /// * Implicit Extern Sync: any [VkDescriptorSet] objects allocated from
  ///   `descriptorPool`
  /// * `device`
  /// * `descriptor_pool`, Extern Sync: true
  /// * `flags`, Optional: true
  pub ResetDescriptorPool: PFN_vkResetDescriptorPool,

  /// Khronos: [vkResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `event`, Extern Sync: true
  pub ResetEvent: PFN_vkResetEvent,

  /// Khronos: [vkResetFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetFences.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `fence_count`
  /// * `fences`, Extern Sync: true, Len: `fence_count`
  pub ResetFences: PFN_vkResetFences,

  /// Khronos: [vkResetQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html)
  /// * `device`
  /// * `query_pool`
  /// * `first_query`
  /// * `query_count`
  pub ResetQueryPool: PFN_vkResetQueryPool,

  /// Khronos: [vkSetBufferCollectionBufferConstraintsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INITIALIZATION_FAILED`, `VK_ERROR_OUT_OF_HOST_MEMORY`,
  ///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
  /// * `device`
  /// * `collection`
  /// * `buffer_constraints_info`
  pub SetBufferCollectionBufferConstraintsFUCHSIA:
    PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,

  /// Khronos: [vkSetBufferCollectionImageConstraintsFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_INITIALIZATION_FAILED`, `VK_ERROR_OUT_OF_HOST_MEMORY`,
  ///   `VK_ERROR_FORMAT_NOT_SUPPORTED`
  /// * `device`
  /// * `collection`
  /// * `image_constraints_info`
  pub SetBufferCollectionImageConstraintsFUCHSIA:
    PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,

  /// Khronos: [vkSetDebugUtilsObjectNameEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `name_info`, Extern Sync: pNameInfo-&gt;objectHandle
  pub SetDebugUtilsObjectNameEXT: PFN_vkSetDebugUtilsObjectNameEXT,

  /// Khronos: [vkSetDebugUtilsObjectTagEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `tag_info`, Extern Sync: pTagInfo-&gt;objectHandle
  pub SetDebugUtilsObjectTagEXT: PFN_vkSetDebugUtilsObjectTagEXT,

  /// Khronos: [vkSetDeviceMemoryPriorityEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html)
  /// * `device`
  /// * `memory`
  /// * `priority`
  pub SetDeviceMemoryPriorityEXT: PFN_vkSetDeviceMemoryPriorityEXT,

  /// Khronos: [vkSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `event`, Extern Sync: true
  pub SetEvent: PFN_vkSetEvent,

  /// Khronos: [vkSetHdrMetadataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html)
  /// * `device`
  /// * `swapchain_count`
  /// * `swapchains`, Len: `swapchain_count`
  /// * `metadata`, Len: `swapchain_count`
  pub SetHdrMetadataEXT: PFN_vkSetHdrMetadataEXT,

  /// Khronos: [vkSetLocalDimmingAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html)
  /// * `device`
  /// * `swap_chain`
  /// * `local_dimming_enable`
  pub SetLocalDimmingAMD: PFN_vkSetLocalDimmingAMD,

  /// Khronos: [vkSetPrivateData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`
  /// * `device`
  /// * `object_type`
  /// * `object_handle`, Object Type: objectType
  /// * `private_data_slot`
  /// * `data`
  pub SetPrivateData: PFN_vkSetPrivateData,

  /// Khronos: [vkSignalSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `signal_info`
  pub SignalSemaphore: PFN_vkSignalSemaphore,

  /// Khronos: [vkTrimCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html)
  /// * `device`
  /// * `command_pool`, Extern Sync: true
  /// * `flags`, Optional: true
  pub TrimCommandPool: PFN_vkTrimCommandPool,

  /// Khronos: [vkUninitializePerformanceApiINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html)
  /// * `device`
  pub UninitializePerformanceApiINTEL: PFN_vkUninitializePerformanceApiINTEL,

  /// Khronos: [vkUnmapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html)
  /// * `device`
  /// * `memory`, Extern Sync: true
  pub UnmapMemory: PFN_vkUnmapMemory,

  /// Khronos: [vkUpdateDescriptorSetWithTemplate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html)
  /// * `device`
  /// * `descriptor_set`
  /// * `descriptor_update_template`
  /// * `data`, true
  pub UpdateDescriptorSetWithTemplate: PFN_vkUpdateDescriptorSetWithTemplate,

  /// Khronos: [vkUpdateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html)
  /// * `device`
  /// * `descriptor_write_count`, Optional: true
  /// * `descriptor_writes`, Len: `descriptor_write_count`
  /// * `descriptor_copy_count`, Optional: true
  /// * `descriptor_copies`, Len: `descriptor_copy_count`
  pub UpdateDescriptorSets: PFN_vkUpdateDescriptorSets,

  /// Khronos: [vkUpdateVideoSessionParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `video_session_parameters`
  /// * `update_info`
  pub UpdateVideoSessionParametersKHR: PFN_vkUpdateVideoSessionParametersKHR,

  /// Khronos: [vkWaitForFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html)
  /// * Success: `VK_SUCCESS`, `VK_TIMEOUT`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `device`
  /// * `fence_count`
  /// * `fences`, Len: `fence_count`
  /// * `wait_all`
  /// * `timeout`
  pub WaitForFences: PFN_vkWaitForFences,

  /// Khronos: [vkWaitForPresentKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html)
  /// * Success: `VK_SUCCESS`, `VK_TIMEOUT`, `VK_SUBOPTIMAL_KHR`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`, `VK_ERROR_OUT_OF_DATE_KHR`,
  ///   `VK_ERROR_SURFACE_LOST_KHR`,
  ///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
  /// * `device`
  /// * `swapchain`, Extern Sync: true
  /// * `present_id`
  /// * `timeout`
  pub WaitForPresentKHR: PFN_vkWaitForPresentKHR,

  /// Khronos: [vkWaitSemaphores](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html)
  /// * Success: `VK_SUCCESS`, `VK_TIMEOUT`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`,
  ///   `VK_ERROR_DEVICE_LOST`
  /// * `device`
  /// * `wait_info`
  /// * `timeout`
  pub WaitSemaphores: PFN_vkWaitSemaphores,

  /// Khronos: [vkWriteAccelerationStructuresPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `acceleration_structure_count`
  /// * `acceleration_structures`, Len: `acceleration_structure_count`
  /// * `query_type`
  /// * `data_size`
  /// * `data`, Len: `data_size`
  /// * `stride`
  pub WriteAccelerationStructuresPropertiesKHR:
    PFN_vkWriteAccelerationStructuresPropertiesKHR,

  /// Khronos: [vkWriteMicromapsPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWriteMicromapsPropertiesEXT.html)
  /// * Success: `VK_SUCCESS`
  /// * Error: `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  /// * `device`
  /// * `micromap_count`
  /// * `micromaps`, Len: `micromap_count`
  /// * `query_type`
  /// * `data_size`
  /// * `data`, Len: `data_size`
  /// * `stride`
  pub WriteMicromapsPropertiesEXT: PFN_vkWriteMicromapsPropertiesEXT,
}
impl DeviceFns {
  pub unsafe fn load(&mut self, device: VkDevice, loader: vkGetDeviceProcAddr_t) {
    use core::mem::transmute;
    self.AcquireFullScreenExclusiveModeEXT =
      transmute(loader(device, vkAcquireFullScreenExclusiveModeEXT_NAME.as_ptr()));
    self.AcquireImageANDROID =
      transmute(loader(device, vkAcquireImageANDROID_NAME.as_ptr()));
    self.AcquireNextImage2KHR =
      transmute(loader(device, vkAcquireNextImage2KHR_NAME.as_ptr()));
    self.AcquireNextImageKHR =
      transmute(loader(device, vkAcquireNextImageKHR_NAME.as_ptr()));
    self.AcquirePerformanceConfigurationINTEL =
      transmute(loader(device, vkAcquirePerformanceConfigurationINTEL_NAME.as_ptr()));
    self.AcquireProfilingLockKHR =
      transmute(loader(device, vkAcquireProfilingLockKHR_NAME.as_ptr()));
    self.AllocateCommandBuffers =
      transmute(loader(device, vkAllocateCommandBuffers_NAME.as_ptr()));
    self.AllocateDescriptorSets =
      transmute(loader(device, vkAllocateDescriptorSets_NAME.as_ptr()));
    self.AllocateMemory = transmute(loader(device, vkAllocateMemory_NAME.as_ptr()));
    self.BeginCommandBuffer =
      transmute(loader(device, vkBeginCommandBuffer_NAME.as_ptr()));
    self.BindAccelerationStructureMemoryNV =
      transmute(loader(device, vkBindAccelerationStructureMemoryNV_NAME.as_ptr()));
    self.BindBufferMemory = transmute(loader(device, vkBindBufferMemory_NAME.as_ptr()));
    self.BindBufferMemory2 = transmute(loader(device, vkBindBufferMemory2_NAME.as_ptr()));
    self.BindImageMemory = transmute(loader(device, vkBindImageMemory_NAME.as_ptr()));
    self.BindImageMemory2 = transmute(loader(device, vkBindImageMemory2_NAME.as_ptr()));
    self.BindOpticalFlowSessionImageNV =
      transmute(loader(device, vkBindOpticalFlowSessionImageNV_NAME.as_ptr()));
    self.BindVideoSessionMemoryKHR =
      transmute(loader(device, vkBindVideoSessionMemoryKHR_NAME.as_ptr()));
    self.CmdBeginConditionalRenderingEXT =
      transmute(loader(device, vkCmdBeginConditionalRenderingEXT_NAME.as_ptr()));
    self.CmdBeginDebugUtilsLabelEXT =
      transmute(loader(device, vkCmdBeginDebugUtilsLabelEXT_NAME.as_ptr()));
    self.CmdBeginQuery = transmute(loader(device, vkCmdBeginQuery_NAME.as_ptr()));
    self.CmdBeginQueryIndexedEXT =
      transmute(loader(device, vkCmdBeginQueryIndexedEXT_NAME.as_ptr()));
    self.CmdBeginRenderPass =
      transmute(loader(device, vkCmdBeginRenderPass_NAME.as_ptr()));
    self.CmdBeginRenderPass2 =
      transmute(loader(device, vkCmdBeginRenderPass2_NAME.as_ptr()));
    self.CmdBeginRendering = transmute(loader(device, vkCmdBeginRendering_NAME.as_ptr()));
    self.CmdBeginTransformFeedbackEXT =
      transmute(loader(device, vkCmdBeginTransformFeedbackEXT_NAME.as_ptr()));
    self.CmdBeginVideoCodingKHR =
      transmute(loader(device, vkCmdBeginVideoCodingKHR_NAME.as_ptr()));
    self.CmdBindDescriptorBufferEmbeddedSamplersEXT = transmute(loader(
      device,
      vkCmdBindDescriptorBufferEmbeddedSamplersEXT_NAME.as_ptr(),
    ));
    self.CmdBindDescriptorBuffersEXT =
      transmute(loader(device, vkCmdBindDescriptorBuffersEXT_NAME.as_ptr()));
    self.CmdBindDescriptorSets =
      transmute(loader(device, vkCmdBindDescriptorSets_NAME.as_ptr()));
    self.CmdBindIndexBuffer =
      transmute(loader(device, vkCmdBindIndexBuffer_NAME.as_ptr()));
    self.CmdBindInvocationMaskHUAWEI =
      transmute(loader(device, vkCmdBindInvocationMaskHUAWEI_NAME.as_ptr()));
    self.CmdBindPipeline = transmute(loader(device, vkCmdBindPipeline_NAME.as_ptr()));
    self.CmdBindPipelineShaderGroupNV =
      transmute(loader(device, vkCmdBindPipelineShaderGroupNV_NAME.as_ptr()));
    self.CmdBindShadingRateImageNV =
      transmute(loader(device, vkCmdBindShadingRateImageNV_NAME.as_ptr()));
    self.CmdBindTransformFeedbackBuffersEXT =
      transmute(loader(device, vkCmdBindTransformFeedbackBuffersEXT_NAME.as_ptr()));
    self.CmdBindVertexBuffers =
      transmute(loader(device, vkCmdBindVertexBuffers_NAME.as_ptr()));
    self.CmdBindVertexBuffers2 =
      transmute(loader(device, vkCmdBindVertexBuffers2_NAME.as_ptr()));
    self.CmdBlitImage = transmute(loader(device, vkCmdBlitImage_NAME.as_ptr()));
    self.CmdBlitImage2 = transmute(loader(device, vkCmdBlitImage2_NAME.as_ptr()));
    self.CmdBuildAccelerationStructureNV =
      transmute(loader(device, vkCmdBuildAccelerationStructureNV_NAME.as_ptr()));
    self.CmdClearAttachments =
      transmute(loader(device, vkCmdClearAttachments_NAME.as_ptr()));
    self.CmdClearColorImage =
      transmute(loader(device, vkCmdClearColorImage_NAME.as_ptr()));
    self.CmdClearDepthStencilImage =
      transmute(loader(device, vkCmdClearDepthStencilImage_NAME.as_ptr()));
    self.CmdControlVideoCodingKHR =
      transmute(loader(device, vkCmdControlVideoCodingKHR_NAME.as_ptr()));
    self.CmdCopyAccelerationStructureKHR =
      transmute(loader(device, vkCmdCopyAccelerationStructureKHR_NAME.as_ptr()));
    self.CmdCopyAccelerationStructureNV =
      transmute(loader(device, vkCmdCopyAccelerationStructureNV_NAME.as_ptr()));
    self.CmdCopyBuffer = transmute(loader(device, vkCmdCopyBuffer_NAME.as_ptr()));
    self.CmdCopyBuffer2 = transmute(loader(device, vkCmdCopyBuffer2_NAME.as_ptr()));
    self.CmdCopyBufferToImage =
      transmute(loader(device, vkCmdCopyBufferToImage_NAME.as_ptr()));
    self.CmdCopyBufferToImage2 =
      transmute(loader(device, vkCmdCopyBufferToImage2_NAME.as_ptr()));
    self.CmdCopyImage = transmute(loader(device, vkCmdCopyImage_NAME.as_ptr()));
    self.CmdCopyImage2 = transmute(loader(device, vkCmdCopyImage2_NAME.as_ptr()));
    self.CmdCopyImageToBuffer =
      transmute(loader(device, vkCmdCopyImageToBuffer_NAME.as_ptr()));
    self.CmdCopyImageToBuffer2 =
      transmute(loader(device, vkCmdCopyImageToBuffer2_NAME.as_ptr()));
    self.CmdCopyMemoryIndirectNV =
      transmute(loader(device, vkCmdCopyMemoryIndirectNV_NAME.as_ptr()));
    self.CmdCopyMemoryToImageIndirectNV =
      transmute(loader(device, vkCmdCopyMemoryToImageIndirectNV_NAME.as_ptr()));
    self.CmdCopyMicromapEXT =
      transmute(loader(device, vkCmdCopyMicromapEXT_NAME.as_ptr()));
    self.CmdCopyQueryPoolResults =
      transmute(loader(device, vkCmdCopyQueryPoolResults_NAME.as_ptr()));
    self.CmdCuLaunchKernelNVX =
      transmute(loader(device, vkCmdCuLaunchKernelNVX_NAME.as_ptr()));
    self.CmdDebugMarkerBeginEXT =
      transmute(loader(device, vkCmdDebugMarkerBeginEXT_NAME.as_ptr()));
    self.CmdDebugMarkerEndEXT =
      transmute(loader(device, vkCmdDebugMarkerEndEXT_NAME.as_ptr()));
    self.CmdDebugMarkerInsertEXT =
      transmute(loader(device, vkCmdDebugMarkerInsertEXT_NAME.as_ptr()));
    self.CmdDecodeVideoKHR = transmute(loader(device, vkCmdDecodeVideoKHR_NAME.as_ptr()));
    self.CmdDecompressMemoryIndirectCountNV =
      transmute(loader(device, vkCmdDecompressMemoryIndirectCountNV_NAME.as_ptr()));
    self.CmdDecompressMemoryNV =
      transmute(loader(device, vkCmdDecompressMemoryNV_NAME.as_ptr()));
    self.CmdDispatch = transmute(loader(device, vkCmdDispatch_NAME.as_ptr()));
    self.CmdDispatchBase = transmute(loader(device, vkCmdDispatchBase_NAME.as_ptr()));
    self.CmdDispatchIndirect =
      transmute(loader(device, vkCmdDispatchIndirect_NAME.as_ptr()));
    self.CmdDraw = transmute(loader(device, vkCmdDraw_NAME.as_ptr()));
    self.CmdDrawClusterHUAWEI =
      transmute(loader(device, vkCmdDrawClusterHUAWEI_NAME.as_ptr()));
    self.CmdDrawClusterIndirectHUAWEI =
      transmute(loader(device, vkCmdDrawClusterIndirectHUAWEI_NAME.as_ptr()));
    self.CmdDrawIndexed = transmute(loader(device, vkCmdDrawIndexed_NAME.as_ptr()));
    self.CmdDrawIndexedIndirect =
      transmute(loader(device, vkCmdDrawIndexedIndirect_NAME.as_ptr()));
    self.CmdDrawIndexedIndirectCount =
      transmute(loader(device, vkCmdDrawIndexedIndirectCount_NAME.as_ptr()));
    self.CmdDrawIndirect = transmute(loader(device, vkCmdDrawIndirect_NAME.as_ptr()));
    self.CmdDrawIndirectByteCountEXT =
      transmute(loader(device, vkCmdDrawIndirectByteCountEXT_NAME.as_ptr()));
    self.CmdDrawIndirectCount =
      transmute(loader(device, vkCmdDrawIndirectCount_NAME.as_ptr()));
    self.CmdDrawMeshTasksEXT =
      transmute(loader(device, vkCmdDrawMeshTasksEXT_NAME.as_ptr()));
    self.CmdDrawMeshTasksIndirectCountEXT =
      transmute(loader(device, vkCmdDrawMeshTasksIndirectCountEXT_NAME.as_ptr()));
    self.CmdDrawMeshTasksIndirectCountNV =
      transmute(loader(device, vkCmdDrawMeshTasksIndirectCountNV_NAME.as_ptr()));
    self.CmdDrawMeshTasksIndirectEXT =
      transmute(loader(device, vkCmdDrawMeshTasksIndirectEXT_NAME.as_ptr()));
    self.CmdDrawMeshTasksIndirectNV =
      transmute(loader(device, vkCmdDrawMeshTasksIndirectNV_NAME.as_ptr()));
    self.CmdDrawMeshTasksNV =
      transmute(loader(device, vkCmdDrawMeshTasksNV_NAME.as_ptr()));
    self.CmdDrawMultiEXT = transmute(loader(device, vkCmdDrawMultiEXT_NAME.as_ptr()));
    self.CmdDrawMultiIndexedEXT =
      transmute(loader(device, vkCmdDrawMultiIndexedEXT_NAME.as_ptr()));
    self.CmdEndConditionalRenderingEXT =
      transmute(loader(device, vkCmdEndConditionalRenderingEXT_NAME.as_ptr()));
    self.CmdEndDebugUtilsLabelEXT =
      transmute(loader(device, vkCmdEndDebugUtilsLabelEXT_NAME.as_ptr()));
    self.CmdEndQuery = transmute(loader(device, vkCmdEndQuery_NAME.as_ptr()));
    self.CmdEndQueryIndexedEXT =
      transmute(loader(device, vkCmdEndQueryIndexedEXT_NAME.as_ptr()));
    self.CmdEndRenderPass = transmute(loader(device, vkCmdEndRenderPass_NAME.as_ptr()));
    self.CmdEndRenderPass2 = transmute(loader(device, vkCmdEndRenderPass2_NAME.as_ptr()));
    self.CmdEndRendering = transmute(loader(device, vkCmdEndRendering_NAME.as_ptr()));
    self.CmdEndTransformFeedbackEXT =
      transmute(loader(device, vkCmdEndTransformFeedbackEXT_NAME.as_ptr()));
    self.CmdEndVideoCodingKHR =
      transmute(loader(device, vkCmdEndVideoCodingKHR_NAME.as_ptr()));
    self.CmdExecuteCommands =
      transmute(loader(device, vkCmdExecuteCommands_NAME.as_ptr()));
    self.CmdExecuteGeneratedCommandsNV =
      transmute(loader(device, vkCmdExecuteGeneratedCommandsNV_NAME.as_ptr()));
    self.CmdFillBuffer = transmute(loader(device, vkCmdFillBuffer_NAME.as_ptr()));
    self.CmdInsertDebugUtilsLabelEXT =
      transmute(loader(device, vkCmdInsertDebugUtilsLabelEXT_NAME.as_ptr()));
    self.CmdNextSubpass = transmute(loader(device, vkCmdNextSubpass_NAME.as_ptr()));
    self.CmdNextSubpass2 = transmute(loader(device, vkCmdNextSubpass2_NAME.as_ptr()));
    self.CmdOpticalFlowExecuteNV =
      transmute(loader(device, vkCmdOpticalFlowExecuteNV_NAME.as_ptr()));
    self.CmdPipelineBarrier =
      transmute(loader(device, vkCmdPipelineBarrier_NAME.as_ptr()));
    self.CmdPipelineBarrier2 =
      transmute(loader(device, vkCmdPipelineBarrier2_NAME.as_ptr()));
    self.CmdPreprocessGeneratedCommandsNV =
      transmute(loader(device, vkCmdPreprocessGeneratedCommandsNV_NAME.as_ptr()));
    self.CmdPushConstants = transmute(loader(device, vkCmdPushConstants_NAME.as_ptr()));
    self.CmdPushDescriptorSetKHR =
      transmute(loader(device, vkCmdPushDescriptorSetKHR_NAME.as_ptr()));
    self.CmdPushDescriptorSetWithTemplateKHR =
      transmute(loader(device, vkCmdPushDescriptorSetWithTemplateKHR_NAME.as_ptr()));
    self.CmdRefreshObjectsKHR =
      transmute(loader(device, vkCmdRefreshObjectsKHR_NAME.as_ptr()));
    self.CmdResetEvent = transmute(loader(device, vkCmdResetEvent_NAME.as_ptr()));
    self.CmdResetEvent2 = transmute(loader(device, vkCmdResetEvent2_NAME.as_ptr()));
    self.CmdResetQueryPool = transmute(loader(device, vkCmdResetQueryPool_NAME.as_ptr()));
    self.CmdResolveImage = transmute(loader(device, vkCmdResolveImage_NAME.as_ptr()));
    self.CmdResolveImage2 = transmute(loader(device, vkCmdResolveImage2_NAME.as_ptr()));
    self.CmdSetAlphaToCoverageEnableEXT =
      transmute(loader(device, vkCmdSetAlphaToCoverageEnableEXT_NAME.as_ptr()));
    self.CmdSetAlphaToOneEnableEXT =
      transmute(loader(device, vkCmdSetAlphaToOneEnableEXT_NAME.as_ptr()));
    self.CmdSetBlendConstants =
      transmute(loader(device, vkCmdSetBlendConstants_NAME.as_ptr()));
    self.CmdSetCheckpointNV =
      transmute(loader(device, vkCmdSetCheckpointNV_NAME.as_ptr()));
    self.CmdSetCoarseSampleOrderNV =
      transmute(loader(device, vkCmdSetCoarseSampleOrderNV_NAME.as_ptr()));
    self.CmdSetColorBlendAdvancedEXT =
      transmute(loader(device, vkCmdSetColorBlendAdvancedEXT_NAME.as_ptr()));
    self.CmdSetColorBlendEnableEXT =
      transmute(loader(device, vkCmdSetColorBlendEnableEXT_NAME.as_ptr()));
    self.CmdSetColorBlendEquationEXT =
      transmute(loader(device, vkCmdSetColorBlendEquationEXT_NAME.as_ptr()));
    self.CmdSetColorWriteEnableEXT =
      transmute(loader(device, vkCmdSetColorWriteEnableEXT_NAME.as_ptr()));
    self.CmdSetColorWriteMaskEXT =
      transmute(loader(device, vkCmdSetColorWriteMaskEXT_NAME.as_ptr()));
    self.CmdSetConservativeRasterizationModeEXT =
      transmute(loader(device, vkCmdSetConservativeRasterizationModeEXT_NAME.as_ptr()));
    self.CmdSetCoverageModulationModeNV =
      transmute(loader(device, vkCmdSetCoverageModulationModeNV_NAME.as_ptr()));
    self.CmdSetCoverageModulationTableEnableNV =
      transmute(loader(device, vkCmdSetCoverageModulationTableEnableNV_NAME.as_ptr()));
    self.CmdSetCoverageModulationTableNV =
      transmute(loader(device, vkCmdSetCoverageModulationTableNV_NAME.as_ptr()));
    self.CmdSetCoverageReductionModeNV =
      transmute(loader(device, vkCmdSetCoverageReductionModeNV_NAME.as_ptr()));
    self.CmdSetCoverageToColorEnableNV =
      transmute(loader(device, vkCmdSetCoverageToColorEnableNV_NAME.as_ptr()));
    self.CmdSetCoverageToColorLocationNV =
      transmute(loader(device, vkCmdSetCoverageToColorLocationNV_NAME.as_ptr()));
    self.CmdSetCullMode = transmute(loader(device, vkCmdSetCullMode_NAME.as_ptr()));
    self.CmdSetDepthBias = transmute(loader(device, vkCmdSetDepthBias_NAME.as_ptr()));
    self.CmdSetDepthBiasEnable =
      transmute(loader(device, vkCmdSetDepthBiasEnable_NAME.as_ptr()));
    self.CmdSetDepthBounds = transmute(loader(device, vkCmdSetDepthBounds_NAME.as_ptr()));
    self.CmdSetDepthBoundsTestEnable =
      transmute(loader(device, vkCmdSetDepthBoundsTestEnable_NAME.as_ptr()));
    self.CmdSetDepthClampEnableEXT =
      transmute(loader(device, vkCmdSetDepthClampEnableEXT_NAME.as_ptr()));
    self.CmdSetDepthClipEnableEXT =
      transmute(loader(device, vkCmdSetDepthClipEnableEXT_NAME.as_ptr()));
    self.CmdSetDepthClipNegativeOneToOneEXT =
      transmute(loader(device, vkCmdSetDepthClipNegativeOneToOneEXT_NAME.as_ptr()));
    self.CmdSetDepthCompareOp =
      transmute(loader(device, vkCmdSetDepthCompareOp_NAME.as_ptr()));
    self.CmdSetDepthTestEnable =
      transmute(loader(device, vkCmdSetDepthTestEnable_NAME.as_ptr()));
    self.CmdSetDepthWriteEnable =
      transmute(loader(device, vkCmdSetDepthWriteEnable_NAME.as_ptr()));
    self.CmdSetDescriptorBufferOffsetsEXT =
      transmute(loader(device, vkCmdSetDescriptorBufferOffsetsEXT_NAME.as_ptr()));
    self.CmdSetDeviceMask = transmute(loader(device, vkCmdSetDeviceMask_NAME.as_ptr()));
    self.CmdSetDiscardRectangleEXT =
      transmute(loader(device, vkCmdSetDiscardRectangleEXT_NAME.as_ptr()));
    self.CmdSetDiscardRectangleEnableEXT =
      transmute(loader(device, vkCmdSetDiscardRectangleEnableEXT_NAME.as_ptr()));
    self.CmdSetDiscardRectangleModeEXT =
      transmute(loader(device, vkCmdSetDiscardRectangleModeEXT_NAME.as_ptr()));
    self.CmdSetEvent = transmute(loader(device, vkCmdSetEvent_NAME.as_ptr()));
    self.CmdSetEvent2 = transmute(loader(device, vkCmdSetEvent2_NAME.as_ptr()));
    self.CmdSetExclusiveScissorEnableNV =
      transmute(loader(device, vkCmdSetExclusiveScissorEnableNV_NAME.as_ptr()));
    self.CmdSetExclusiveScissorNV =
      transmute(loader(device, vkCmdSetExclusiveScissorNV_NAME.as_ptr()));
    self.CmdSetExtraPrimitiveOverestimationSizeEXT = transmute(loader(
      device,
      vkCmdSetExtraPrimitiveOverestimationSizeEXT_NAME.as_ptr(),
    ));
    self.CmdSetFragmentShadingRateEnumNV =
      transmute(loader(device, vkCmdSetFragmentShadingRateEnumNV_NAME.as_ptr()));
    self.CmdSetFragmentShadingRateKHR =
      transmute(loader(device, vkCmdSetFragmentShadingRateKHR_NAME.as_ptr()));
    self.CmdSetFrontFace = transmute(loader(device, vkCmdSetFrontFace_NAME.as_ptr()));
    self.CmdSetLineRasterizationModeEXT =
      transmute(loader(device, vkCmdSetLineRasterizationModeEXT_NAME.as_ptr()));
    self.CmdSetLineStippleEXT =
      transmute(loader(device, vkCmdSetLineStippleEXT_NAME.as_ptr()));
    self.CmdSetLineStippleEnableEXT =
      transmute(loader(device, vkCmdSetLineStippleEnableEXT_NAME.as_ptr()));
    self.CmdSetLineWidth = transmute(loader(device, vkCmdSetLineWidth_NAME.as_ptr()));
    self.CmdSetLogicOpEXT = transmute(loader(device, vkCmdSetLogicOpEXT_NAME.as_ptr()));
    self.CmdSetLogicOpEnableEXT =
      transmute(loader(device, vkCmdSetLogicOpEnableEXT_NAME.as_ptr()));
    self.CmdSetPatchControlPointsEXT =
      transmute(loader(device, vkCmdSetPatchControlPointsEXT_NAME.as_ptr()));
    self.CmdSetPerformanceMarkerINTEL =
      transmute(loader(device, vkCmdSetPerformanceMarkerINTEL_NAME.as_ptr()));
    self.CmdSetPerformanceOverrideINTEL =
      transmute(loader(device, vkCmdSetPerformanceOverrideINTEL_NAME.as_ptr()));
    self.CmdSetPerformanceStreamMarkerINTEL =
      transmute(loader(device, vkCmdSetPerformanceStreamMarkerINTEL_NAME.as_ptr()));
    self.CmdSetPolygonModeEXT =
      transmute(loader(device, vkCmdSetPolygonModeEXT_NAME.as_ptr()));
    self.CmdSetPrimitiveRestartEnable =
      transmute(loader(device, vkCmdSetPrimitiveRestartEnable_NAME.as_ptr()));
    self.CmdSetPrimitiveTopology =
      transmute(loader(device, vkCmdSetPrimitiveTopology_NAME.as_ptr()));
    self.CmdSetProvokingVertexModeEXT =
      transmute(loader(device, vkCmdSetProvokingVertexModeEXT_NAME.as_ptr()));
    self.CmdSetRasterizationSamplesEXT =
      transmute(loader(device, vkCmdSetRasterizationSamplesEXT_NAME.as_ptr()));
    self.CmdSetRasterizationStreamEXT =
      transmute(loader(device, vkCmdSetRasterizationStreamEXT_NAME.as_ptr()));
    self.CmdSetRasterizerDiscardEnable =
      transmute(loader(device, vkCmdSetRasterizerDiscardEnable_NAME.as_ptr()));
    self.CmdSetRayTracingPipelineStackSizeKHR =
      transmute(loader(device, vkCmdSetRayTracingPipelineStackSizeKHR_NAME.as_ptr()));
    self.CmdSetRepresentativeFragmentTestEnableNV =
      transmute(loader(device, vkCmdSetRepresentativeFragmentTestEnableNV_NAME.as_ptr()));
    self.CmdSetSampleLocationsEXT =
      transmute(loader(device, vkCmdSetSampleLocationsEXT_NAME.as_ptr()));
    self.CmdSetSampleLocationsEnableEXT =
      transmute(loader(device, vkCmdSetSampleLocationsEnableEXT_NAME.as_ptr()));
    self.CmdSetSampleMaskEXT =
      transmute(loader(device, vkCmdSetSampleMaskEXT_NAME.as_ptr()));
    self.CmdSetScissor = transmute(loader(device, vkCmdSetScissor_NAME.as_ptr()));
    self.CmdSetScissorWithCount =
      transmute(loader(device, vkCmdSetScissorWithCount_NAME.as_ptr()));
    self.CmdSetShadingRateImageEnableNV =
      transmute(loader(device, vkCmdSetShadingRateImageEnableNV_NAME.as_ptr()));
    self.CmdSetStencilCompareMask =
      transmute(loader(device, vkCmdSetStencilCompareMask_NAME.as_ptr()));
    self.CmdSetStencilOp = transmute(loader(device, vkCmdSetStencilOp_NAME.as_ptr()));
    self.CmdSetStencilReference =
      transmute(loader(device, vkCmdSetStencilReference_NAME.as_ptr()));
    self.CmdSetStencilTestEnable =
      transmute(loader(device, vkCmdSetStencilTestEnable_NAME.as_ptr()));
    self.CmdSetStencilWriteMask =
      transmute(loader(device, vkCmdSetStencilWriteMask_NAME.as_ptr()));
    self.CmdSetTessellationDomainOriginEXT =
      transmute(loader(device, vkCmdSetTessellationDomainOriginEXT_NAME.as_ptr()));
    self.CmdSetVertexInputEXT =
      transmute(loader(device, vkCmdSetVertexInputEXT_NAME.as_ptr()));
    self.CmdSetViewport = transmute(loader(device, vkCmdSetViewport_NAME.as_ptr()));
    self.CmdSetViewportShadingRatePaletteNV =
      transmute(loader(device, vkCmdSetViewportShadingRatePaletteNV_NAME.as_ptr()));
    self.CmdSetViewportSwizzleNV =
      transmute(loader(device, vkCmdSetViewportSwizzleNV_NAME.as_ptr()));
    self.CmdSetViewportWScalingEnableNV =
      transmute(loader(device, vkCmdSetViewportWScalingEnableNV_NAME.as_ptr()));
    self.CmdSetViewportWScalingNV =
      transmute(loader(device, vkCmdSetViewportWScalingNV_NAME.as_ptr()));
    self.CmdSetViewportWithCount =
      transmute(loader(device, vkCmdSetViewportWithCount_NAME.as_ptr()));
    self.CmdSubpassShadingHUAWEI =
      transmute(loader(device, vkCmdSubpassShadingHUAWEI_NAME.as_ptr()));
    self.CmdTraceRaysIndirect2KHR =
      transmute(loader(device, vkCmdTraceRaysIndirect2KHR_NAME.as_ptr()));
    self.CmdTraceRaysIndirectKHR =
      transmute(loader(device, vkCmdTraceRaysIndirectKHR_NAME.as_ptr()));
    self.CmdTraceRaysKHR = transmute(loader(device, vkCmdTraceRaysKHR_NAME.as_ptr()));
    self.CmdTraceRaysNV = transmute(loader(device, vkCmdTraceRaysNV_NAME.as_ptr()));
    self.CmdUpdateBuffer = transmute(loader(device, vkCmdUpdateBuffer_NAME.as_ptr()));
    self.CmdWaitEvents = transmute(loader(device, vkCmdWaitEvents_NAME.as_ptr()));
    self.CmdWaitEvents2 = transmute(loader(device, vkCmdWaitEvents2_NAME.as_ptr()));
    self.CmdWriteAccelerationStructuresPropertiesKHR = transmute(loader(
      device,
      vkCmdWriteAccelerationStructuresPropertiesKHR_NAME.as_ptr(),
    ));
    self.CmdWriteAccelerationStructuresPropertiesNV = transmute(loader(
      device,
      vkCmdWriteAccelerationStructuresPropertiesNV_NAME.as_ptr(),
    ));
    self.CmdWriteBufferMarker2AMD =
      transmute(loader(device, vkCmdWriteBufferMarker2AMD_NAME.as_ptr()));
    self.CmdWriteBufferMarkerAMD =
      transmute(loader(device, vkCmdWriteBufferMarkerAMD_NAME.as_ptr()));
    self.CmdWriteMicromapsPropertiesEXT =
      transmute(loader(device, vkCmdWriteMicromapsPropertiesEXT_NAME.as_ptr()));
    self.CmdWriteTimestamp = transmute(loader(device, vkCmdWriteTimestamp_NAME.as_ptr()));
    self.CmdWriteTimestamp2 =
      transmute(loader(device, vkCmdWriteTimestamp2_NAME.as_ptr()));
    self.CompileDeferredNV = transmute(loader(device, vkCompileDeferredNV_NAME.as_ptr()));
    self.CopyAccelerationStructureKHR =
      transmute(loader(device, vkCopyAccelerationStructureKHR_NAME.as_ptr()));
    self.CopyMicromapEXT = transmute(loader(device, vkCopyMicromapEXT_NAME.as_ptr()));
    self.CreateAccelerationStructureKHR =
      transmute(loader(device, vkCreateAccelerationStructureKHR_NAME.as_ptr()));
    self.CreateAccelerationStructureNV =
      transmute(loader(device, vkCreateAccelerationStructureNV_NAME.as_ptr()));
    self.CreateBuffer = transmute(loader(device, vkCreateBuffer_NAME.as_ptr()));
    self.CreateBufferCollectionFUCHSIA =
      transmute(loader(device, vkCreateBufferCollectionFUCHSIA_NAME.as_ptr()));
    self.CreateBufferView = transmute(loader(device, vkCreateBufferView_NAME.as_ptr()));
    self.CreateCommandPool = transmute(loader(device, vkCreateCommandPool_NAME.as_ptr()));
    self.CreateComputePipelines =
      transmute(loader(device, vkCreateComputePipelines_NAME.as_ptr()));
    self.CreateCuFunctionNVX =
      transmute(loader(device, vkCreateCuFunctionNVX_NAME.as_ptr()));
    self.CreateCuModuleNVX = transmute(loader(device, vkCreateCuModuleNVX_NAME.as_ptr()));
    self.CreateDeferredOperationKHR =
      transmute(loader(device, vkCreateDeferredOperationKHR_NAME.as_ptr()));
    self.CreateDescriptorPool =
      transmute(loader(device, vkCreateDescriptorPool_NAME.as_ptr()));
    self.CreateDescriptorSetLayout =
      transmute(loader(device, vkCreateDescriptorSetLayout_NAME.as_ptr()));
    self.CreateDescriptorUpdateTemplate =
      transmute(loader(device, vkCreateDescriptorUpdateTemplate_NAME.as_ptr()));
    self.CreateEvent = transmute(loader(device, vkCreateEvent_NAME.as_ptr()));
    self.CreateFence = transmute(loader(device, vkCreateFence_NAME.as_ptr()));
    self.CreateFramebuffer = transmute(loader(device, vkCreateFramebuffer_NAME.as_ptr()));
    self.CreateGraphicsPipelines =
      transmute(loader(device, vkCreateGraphicsPipelines_NAME.as_ptr()));
    self.CreateImage = transmute(loader(device, vkCreateImage_NAME.as_ptr()));
    self.CreateImageView = transmute(loader(device, vkCreateImageView_NAME.as_ptr()));
    self.CreateIndirectCommandsLayoutNV =
      transmute(loader(device, vkCreateIndirectCommandsLayoutNV_NAME.as_ptr()));
    self.CreateMicromapEXT = transmute(loader(device, vkCreateMicromapEXT_NAME.as_ptr()));
    self.CreateOpticalFlowSessionNV =
      transmute(loader(device, vkCreateOpticalFlowSessionNV_NAME.as_ptr()));
    self.CreatePipelineCache =
      transmute(loader(device, vkCreatePipelineCache_NAME.as_ptr()));
    self.CreatePipelineLayout =
      transmute(loader(device, vkCreatePipelineLayout_NAME.as_ptr()));
    self.CreatePrivateDataSlot =
      transmute(loader(device, vkCreatePrivateDataSlot_NAME.as_ptr()));
    self.CreateQueryPool = transmute(loader(device, vkCreateQueryPool_NAME.as_ptr()));
    self.CreateRayTracingPipelinesKHR =
      transmute(loader(device, vkCreateRayTracingPipelinesKHR_NAME.as_ptr()));
    self.CreateRayTracingPipelinesNV =
      transmute(loader(device, vkCreateRayTracingPipelinesNV_NAME.as_ptr()));
    self.CreateRenderPass = transmute(loader(device, vkCreateRenderPass_NAME.as_ptr()));
    self.CreateRenderPass2 = transmute(loader(device, vkCreateRenderPass2_NAME.as_ptr()));
    self.CreateSampler = transmute(loader(device, vkCreateSampler_NAME.as_ptr()));
    self.CreateSamplerYcbcrConversion =
      transmute(loader(device, vkCreateSamplerYcbcrConversion_NAME.as_ptr()));
    self.CreateSemaphore = transmute(loader(device, vkCreateSemaphore_NAME.as_ptr()));
    self.CreateShaderModule =
      transmute(loader(device, vkCreateShaderModule_NAME.as_ptr()));
    self.CreateSharedSwapchainsKHR =
      transmute(loader(device, vkCreateSharedSwapchainsKHR_NAME.as_ptr()));
    self.CreateSwapchainKHR =
      transmute(loader(device, vkCreateSwapchainKHR_NAME.as_ptr()));
    self.CreateValidationCacheEXT =
      transmute(loader(device, vkCreateValidationCacheEXT_NAME.as_ptr()));
    self.CreateVideoSessionKHR =
      transmute(loader(device, vkCreateVideoSessionKHR_NAME.as_ptr()));
    self.CreateVideoSessionParametersKHR =
      transmute(loader(device, vkCreateVideoSessionParametersKHR_NAME.as_ptr()));
    self.DebugMarkerSetObjectNameEXT =
      transmute(loader(device, vkDebugMarkerSetObjectNameEXT_NAME.as_ptr()));
    self.DebugMarkerSetObjectTagEXT =
      transmute(loader(device, vkDebugMarkerSetObjectTagEXT_NAME.as_ptr()));
    self.DeferredOperationJoinKHR =
      transmute(loader(device, vkDeferredOperationJoinKHR_NAME.as_ptr()));
    self.DestroyAccelerationStructureKHR =
      transmute(loader(device, vkDestroyAccelerationStructureKHR_NAME.as_ptr()));
    self.DestroyAccelerationStructureNV =
      transmute(loader(device, vkDestroyAccelerationStructureNV_NAME.as_ptr()));
    self.DestroyBuffer = transmute(loader(device, vkDestroyBuffer_NAME.as_ptr()));
    self.DestroyBufferCollectionFUCHSIA =
      transmute(loader(device, vkDestroyBufferCollectionFUCHSIA_NAME.as_ptr()));
    self.DestroyBufferView = transmute(loader(device, vkDestroyBufferView_NAME.as_ptr()));
    self.DestroyCommandPool =
      transmute(loader(device, vkDestroyCommandPool_NAME.as_ptr()));
    self.DestroyCuFunctionNVX =
      transmute(loader(device, vkDestroyCuFunctionNVX_NAME.as_ptr()));
    self.DestroyCuModuleNVX =
      transmute(loader(device, vkDestroyCuModuleNVX_NAME.as_ptr()));
    self.DestroyDeferredOperationKHR =
      transmute(loader(device, vkDestroyDeferredOperationKHR_NAME.as_ptr()));
    self.DestroyDescriptorPool =
      transmute(loader(device, vkDestroyDescriptorPool_NAME.as_ptr()));
    self.DestroyDescriptorSetLayout =
      transmute(loader(device, vkDestroyDescriptorSetLayout_NAME.as_ptr()));
    self.DestroyDescriptorUpdateTemplate =
      transmute(loader(device, vkDestroyDescriptorUpdateTemplate_NAME.as_ptr()));
    self.DestroyDevice = transmute(loader(device, vkDestroyDevice_NAME.as_ptr()));
    self.DestroyEvent = transmute(loader(device, vkDestroyEvent_NAME.as_ptr()));
    self.DestroyFence = transmute(loader(device, vkDestroyFence_NAME.as_ptr()));
    self.DestroyFramebuffer =
      transmute(loader(device, vkDestroyFramebuffer_NAME.as_ptr()));
    self.DestroyImage = transmute(loader(device, vkDestroyImage_NAME.as_ptr()));
    self.DestroyImageView = transmute(loader(device, vkDestroyImageView_NAME.as_ptr()));
    self.DestroyIndirectCommandsLayoutNV =
      transmute(loader(device, vkDestroyIndirectCommandsLayoutNV_NAME.as_ptr()));
    self.DestroyMicromapEXT =
      transmute(loader(device, vkDestroyMicromapEXT_NAME.as_ptr()));
    self.DestroyOpticalFlowSessionNV =
      transmute(loader(device, vkDestroyOpticalFlowSessionNV_NAME.as_ptr()));
    self.DestroyPipeline = transmute(loader(device, vkDestroyPipeline_NAME.as_ptr()));
    self.DestroyPipelineCache =
      transmute(loader(device, vkDestroyPipelineCache_NAME.as_ptr()));
    self.DestroyPipelineLayout =
      transmute(loader(device, vkDestroyPipelineLayout_NAME.as_ptr()));
    self.DestroyPrivateDataSlot =
      transmute(loader(device, vkDestroyPrivateDataSlot_NAME.as_ptr()));
    self.DestroyQueryPool = transmute(loader(device, vkDestroyQueryPool_NAME.as_ptr()));
    self.DestroyRenderPass = transmute(loader(device, vkDestroyRenderPass_NAME.as_ptr()));
    self.DestroySampler = transmute(loader(device, vkDestroySampler_NAME.as_ptr()));
    self.DestroySamplerYcbcrConversion =
      transmute(loader(device, vkDestroySamplerYcbcrConversion_NAME.as_ptr()));
    self.DestroySemaphore = transmute(loader(device, vkDestroySemaphore_NAME.as_ptr()));
    self.DestroySemaphoreSciSyncPoolNV =
      transmute(loader(device, vkDestroySemaphoreSciSyncPoolNV_NAME.as_ptr()));
    self.DestroyShaderModule =
      transmute(loader(device, vkDestroyShaderModule_NAME.as_ptr()));
    self.DestroySwapchainKHR =
      transmute(loader(device, vkDestroySwapchainKHR_NAME.as_ptr()));
    self.DestroyValidationCacheEXT =
      transmute(loader(device, vkDestroyValidationCacheEXT_NAME.as_ptr()));
    self.DestroyVideoSessionKHR =
      transmute(loader(device, vkDestroyVideoSessionKHR_NAME.as_ptr()));
    self.DestroyVideoSessionParametersKHR =
      transmute(loader(device, vkDestroyVideoSessionParametersKHR_NAME.as_ptr()));
    self.DeviceWaitIdle = transmute(loader(device, vkDeviceWaitIdle_NAME.as_ptr()));
    self.DisplayPowerControlEXT =
      transmute(loader(device, vkDisplayPowerControlEXT_NAME.as_ptr()));
    self.EndCommandBuffer = transmute(loader(device, vkEndCommandBuffer_NAME.as_ptr()));
    self.ExportMetalObjectsEXT =
      transmute(loader(device, vkExportMetalObjectsEXT_NAME.as_ptr()));
    self.FlushMappedMemoryRanges =
      transmute(loader(device, vkFlushMappedMemoryRanges_NAME.as_ptr()));
    self.FreeCommandBuffers =
      transmute(loader(device, vkFreeCommandBuffers_NAME.as_ptr()));
    self.FreeDescriptorSets =
      transmute(loader(device, vkFreeDescriptorSets_NAME.as_ptr()));
    self.FreeMemory = transmute(loader(device, vkFreeMemory_NAME.as_ptr()));
    self.GetAccelerationStructureDeviceAddressKHR =
      transmute(loader(device, vkGetAccelerationStructureDeviceAddressKHR_NAME.as_ptr()));
    self.GetAccelerationStructureHandleNV =
      transmute(loader(device, vkGetAccelerationStructureHandleNV_NAME.as_ptr()));
    self.GetAccelerationStructureMemoryRequirementsNV = transmute(loader(
      device,
      vkGetAccelerationStructureMemoryRequirementsNV_NAME.as_ptr(),
    ));
    self.GetAccelerationStructureOpaqueCaptureDescriptorDataEXT = transmute(loader(
      device,
      vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT_NAME.as_ptr(),
    ));
    self.GetAndroidHardwareBufferPropertiesANDROID = transmute(loader(
      device,
      vkGetAndroidHardwareBufferPropertiesANDROID_NAME.as_ptr(),
    ));
    self.GetBufferCollectionPropertiesFUCHSIA =
      transmute(loader(device, vkGetBufferCollectionPropertiesFUCHSIA_NAME.as_ptr()));
    self.GetBufferDeviceAddress =
      transmute(loader(device, vkGetBufferDeviceAddress_NAME.as_ptr()));
    self.GetBufferMemoryRequirements =
      transmute(loader(device, vkGetBufferMemoryRequirements_NAME.as_ptr()));
    self.GetBufferMemoryRequirements2 =
      transmute(loader(device, vkGetBufferMemoryRequirements2_NAME.as_ptr()));
    self.GetBufferOpaqueCaptureAddress =
      transmute(loader(device, vkGetBufferOpaqueCaptureAddress_NAME.as_ptr()));
    self.GetBufferOpaqueCaptureDescriptorDataEXT =
      transmute(loader(device, vkGetBufferOpaqueCaptureDescriptorDataEXT_NAME.as_ptr()));
    self.GetCalibratedTimestampsEXT =
      transmute(loader(device, vkGetCalibratedTimestampsEXT_NAME.as_ptr()));
    self.GetDeferredOperationMaxConcurrencyKHR =
      transmute(loader(device, vkGetDeferredOperationMaxConcurrencyKHR_NAME.as_ptr()));
    self.GetDeferredOperationResultKHR =
      transmute(loader(device, vkGetDeferredOperationResultKHR_NAME.as_ptr()));
    self.GetDescriptorSetHostMappingVALVE =
      transmute(loader(device, vkGetDescriptorSetHostMappingVALVE_NAME.as_ptr()));
    self.GetDescriptorSetLayoutBindingOffsetEXT =
      transmute(loader(device, vkGetDescriptorSetLayoutBindingOffsetEXT_NAME.as_ptr()));
    self.GetDescriptorSetLayoutHostMappingInfoVALVE = transmute(loader(
      device,
      vkGetDescriptorSetLayoutHostMappingInfoVALVE_NAME.as_ptr(),
    ));
    self.GetDescriptorSetLayoutSizeEXT =
      transmute(loader(device, vkGetDescriptorSetLayoutSizeEXT_NAME.as_ptr()));
    self.GetDescriptorSetLayoutSupport =
      transmute(loader(device, vkGetDescriptorSetLayoutSupport_NAME.as_ptr()));
    self.GetDeviceAccelerationStructureCompatibilityKHR = transmute(loader(
      device,
      vkGetDeviceAccelerationStructureCompatibilityKHR_NAME.as_ptr(),
    ));
    self.GetDeviceBufferMemoryRequirements =
      transmute(loader(device, vkGetDeviceBufferMemoryRequirements_NAME.as_ptr()));
    self.GetDeviceFaultInfoEXT =
      transmute(loader(device, vkGetDeviceFaultInfoEXT_NAME.as_ptr()));
    self.GetDeviceGroupPeerMemoryFeatures =
      transmute(loader(device, vkGetDeviceGroupPeerMemoryFeatures_NAME.as_ptr()));
    self.GetDeviceGroupPresentCapabilitiesKHR =
      transmute(loader(device, vkGetDeviceGroupPresentCapabilitiesKHR_NAME.as_ptr()));
    self.GetDeviceGroupSurfacePresentModes2EXT =
      transmute(loader(device, vkGetDeviceGroupSurfacePresentModes2EXT_NAME.as_ptr()));
    self.GetDeviceGroupSurfacePresentModesKHR =
      transmute(loader(device, vkGetDeviceGroupSurfacePresentModesKHR_NAME.as_ptr()));
    self.GetDeviceImageMemoryRequirements =
      transmute(loader(device, vkGetDeviceImageMemoryRequirements_NAME.as_ptr()));
    self.GetDeviceImageSparseMemoryRequirements =
      transmute(loader(device, vkGetDeviceImageSparseMemoryRequirements_NAME.as_ptr()));
    self.GetDeviceMemoryCommitment =
      transmute(loader(device, vkGetDeviceMemoryCommitment_NAME.as_ptr()));
    self.GetDeviceMemoryOpaqueCaptureAddress =
      transmute(loader(device, vkGetDeviceMemoryOpaqueCaptureAddress_NAME.as_ptr()));
    self.GetDeviceMicromapCompatibilityEXT =
      transmute(loader(device, vkGetDeviceMicromapCompatibilityEXT_NAME.as_ptr()));
    self.GetDeviceProcAddr = transmute(loader(device, vkGetDeviceProcAddr_NAME.as_ptr()));
    self.GetDeviceQueue = transmute(loader(device, vkGetDeviceQueue_NAME.as_ptr()));
    self.GetDeviceQueue2 = transmute(loader(device, vkGetDeviceQueue2_NAME.as_ptr()));
    self.GetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = transmute(loader(
      device,
      vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI_NAME.as_ptr(),
    ));
    self.GetDynamicRenderingTilePropertiesQCOM =
      transmute(loader(device, vkGetDynamicRenderingTilePropertiesQCOM_NAME.as_ptr()));
    self.GetEventStatus = transmute(loader(device, vkGetEventStatus_NAME.as_ptr()));
    self.GetFenceFdKHR = transmute(loader(device, vkGetFenceFdKHR_NAME.as_ptr()));
    self.GetFenceSciSyncFenceNV =
      transmute(loader(device, vkGetFenceSciSyncFenceNV_NAME.as_ptr()));
    self.GetFenceSciSyncObjNV =
      transmute(loader(device, vkGetFenceSciSyncObjNV_NAME.as_ptr()));
    self.GetFenceStatus = transmute(loader(device, vkGetFenceStatus_NAME.as_ptr()));
    self.GetFenceWin32HandleKHR =
      transmute(loader(device, vkGetFenceWin32HandleKHR_NAME.as_ptr()));
    self.GetFramebufferTilePropertiesQCOM =
      transmute(loader(device, vkGetFramebufferTilePropertiesQCOM_NAME.as_ptr()));
    self.GetGeneratedCommandsMemoryRequirementsNV =
      transmute(loader(device, vkGetGeneratedCommandsMemoryRequirementsNV_NAME.as_ptr()));
    self.GetImageDrmFormatModifierPropertiesEXT =
      transmute(loader(device, vkGetImageDrmFormatModifierPropertiesEXT_NAME.as_ptr()));
    self.GetImageMemoryRequirements =
      transmute(loader(device, vkGetImageMemoryRequirements_NAME.as_ptr()));
    self.GetImageMemoryRequirements2 =
      transmute(loader(device, vkGetImageMemoryRequirements2_NAME.as_ptr()));
    self.GetImageOpaqueCaptureDescriptorDataEXT =
      transmute(loader(device, vkGetImageOpaqueCaptureDescriptorDataEXT_NAME.as_ptr()));
    self.GetImageSparseMemoryRequirements =
      transmute(loader(device, vkGetImageSparseMemoryRequirements_NAME.as_ptr()));
    self.GetImageSparseMemoryRequirements2 =
      transmute(loader(device, vkGetImageSparseMemoryRequirements2_NAME.as_ptr()));
    self.GetImageSubresourceLayout =
      transmute(loader(device, vkGetImageSubresourceLayout_NAME.as_ptr()));
    self.GetImageSubresourceLayout2EXT =
      transmute(loader(device, vkGetImageSubresourceLayout2EXT_NAME.as_ptr()));
    self.GetImageViewAddressNVX =
      transmute(loader(device, vkGetImageViewAddressNVX_NAME.as_ptr()));
    self.GetImageViewHandleNVX =
      transmute(loader(device, vkGetImageViewHandleNVX_NAME.as_ptr()));
    self.GetImageViewOpaqueCaptureDescriptorDataEXT = transmute(loader(
      device,
      vkGetImageViewOpaqueCaptureDescriptorDataEXT_NAME.as_ptr(),
    ));
    self.GetMemoryAndroidHardwareBufferANDROID =
      transmute(loader(device, vkGetMemoryAndroidHardwareBufferANDROID_NAME.as_ptr()));
    self.GetMemoryFdKHR = transmute(loader(device, vkGetMemoryFdKHR_NAME.as_ptr()));
    self.GetMemoryFdPropertiesKHR =
      transmute(loader(device, vkGetMemoryFdPropertiesKHR_NAME.as_ptr()));
    self.GetMemoryHostPointerPropertiesEXT =
      transmute(loader(device, vkGetMemoryHostPointerPropertiesEXT_NAME.as_ptr()));
    self.GetMemoryRemoteAddressNV =
      transmute(loader(device, vkGetMemoryRemoteAddressNV_NAME.as_ptr()));
    self.GetMemoryWin32HandleKHR =
      transmute(loader(device, vkGetMemoryWin32HandleKHR_NAME.as_ptr()));
    self.GetMemoryWin32HandleNV =
      transmute(loader(device, vkGetMemoryWin32HandleNV_NAME.as_ptr()));
    self.GetMemoryWin32HandlePropertiesKHR =
      transmute(loader(device, vkGetMemoryWin32HandlePropertiesKHR_NAME.as_ptr()));
    self.GetMemoryZirconHandleFUCHSIA =
      transmute(loader(device, vkGetMemoryZirconHandleFUCHSIA_NAME.as_ptr()));
    self.GetMemoryZirconHandlePropertiesFUCHSIA =
      transmute(loader(device, vkGetMemoryZirconHandlePropertiesFUCHSIA_NAME.as_ptr()));
    self.GetPastPresentationTimingGOOGLE =
      transmute(loader(device, vkGetPastPresentationTimingGOOGLE_NAME.as_ptr()));
    self.GetPipelineCacheData =
      transmute(loader(device, vkGetPipelineCacheData_NAME.as_ptr()));
    self.GetPipelineExecutableInternalRepresentationsKHR = transmute(loader(
      device,
      vkGetPipelineExecutableInternalRepresentationsKHR_NAME.as_ptr(),
    ));
    self.GetPipelineExecutablePropertiesKHR =
      transmute(loader(device, vkGetPipelineExecutablePropertiesKHR_NAME.as_ptr()));
    self.GetPipelinePropertiesEXT =
      transmute(loader(device, vkGetPipelinePropertiesEXT_NAME.as_ptr()));
    self.GetPrivateData = transmute(loader(device, vkGetPrivateData_NAME.as_ptr()));
    self.GetQueryPoolResults =
      transmute(loader(device, vkGetQueryPoolResults_NAME.as_ptr()));
    self.GetQueueCheckpointData2NV =
      transmute(loader(device, vkGetQueueCheckpointData2NV_NAME.as_ptr()));
    self.GetQueueCheckpointDataNV =
      transmute(loader(device, vkGetQueueCheckpointDataNV_NAME.as_ptr()));
    self.GetRayTracingCaptureReplayShaderGroupHandlesKHR = transmute(loader(
      device,
      vkGetRayTracingCaptureReplayShaderGroupHandlesKHR_NAME.as_ptr(),
    ));
    self.GetRayTracingShaderGroupHandlesKHR =
      transmute(loader(device, vkGetRayTracingShaderGroupHandlesKHR_NAME.as_ptr()));
    self.GetRayTracingShaderGroupStackSizeKHR =
      transmute(loader(device, vkGetRayTracingShaderGroupStackSizeKHR_NAME.as_ptr()));
    self.GetRefreshCycleDurationGOOGLE =
      transmute(loader(device, vkGetRefreshCycleDurationGOOGLE_NAME.as_ptr()));
    self.GetRenderAreaGranularity =
      transmute(loader(device, vkGetRenderAreaGranularity_NAME.as_ptr()));
    self.GetSamplerOpaqueCaptureDescriptorDataEXT =
      transmute(loader(device, vkGetSamplerOpaqueCaptureDescriptorDataEXT_NAME.as_ptr()));
    self.GetSemaphoreCounterValue =
      transmute(loader(device, vkGetSemaphoreCounterValue_NAME.as_ptr()));
    self.GetSemaphoreFdKHR = transmute(loader(device, vkGetSemaphoreFdKHR_NAME.as_ptr()));
    self.GetSemaphoreSciSyncObjNV =
      transmute(loader(device, vkGetSemaphoreSciSyncObjNV_NAME.as_ptr()));
    self.GetSemaphoreWin32HandleKHR =
      transmute(loader(device, vkGetSemaphoreWin32HandleKHR_NAME.as_ptr()));
    self.GetSemaphoreZirconHandleFUCHSIA =
      transmute(loader(device, vkGetSemaphoreZirconHandleFUCHSIA_NAME.as_ptr()));
    self.GetShaderInfoAMD = transmute(loader(device, vkGetShaderInfoAMD_NAME.as_ptr()));
    self.GetShaderModuleCreateInfoIdentifierEXT =
      transmute(loader(device, vkGetShaderModuleCreateInfoIdentifierEXT_NAME.as_ptr()));
    self.GetShaderModuleIdentifierEXT =
      transmute(loader(device, vkGetShaderModuleIdentifierEXT_NAME.as_ptr()));
    self.GetSwapchainCounterEXT =
      transmute(loader(device, vkGetSwapchainCounterEXT_NAME.as_ptr()));
    self.GetSwapchainGrallocUsage2ANDROID =
      transmute(loader(device, vkGetSwapchainGrallocUsage2ANDROID_NAME.as_ptr()));
    self.GetSwapchainGrallocUsageANDROID =
      transmute(loader(device, vkGetSwapchainGrallocUsageANDROID_NAME.as_ptr()));
    self.GetSwapchainImagesKHR =
      transmute(loader(device, vkGetSwapchainImagesKHR_NAME.as_ptr()));
    self.GetSwapchainStatusKHR =
      transmute(loader(device, vkGetSwapchainStatusKHR_NAME.as_ptr()));
    self.GetValidationCacheDataEXT =
      transmute(loader(device, vkGetValidationCacheDataEXT_NAME.as_ptr()));
    self.GetVideoSessionMemoryRequirementsKHR =
      transmute(loader(device, vkGetVideoSessionMemoryRequirementsKHR_NAME.as_ptr()));
    self.ImportFenceFdKHR = transmute(loader(device, vkImportFenceFdKHR_NAME.as_ptr()));
    self.ImportFenceSciSyncFenceNV =
      transmute(loader(device, vkImportFenceSciSyncFenceNV_NAME.as_ptr()));
    self.ImportFenceSciSyncObjNV =
      transmute(loader(device, vkImportFenceSciSyncObjNV_NAME.as_ptr()));
    self.ImportFenceWin32HandleKHR =
      transmute(loader(device, vkImportFenceWin32HandleKHR_NAME.as_ptr()));
    self.ImportSemaphoreFdKHR =
      transmute(loader(device, vkImportSemaphoreFdKHR_NAME.as_ptr()));
    self.ImportSemaphoreSciSyncObjNV =
      transmute(loader(device, vkImportSemaphoreSciSyncObjNV_NAME.as_ptr()));
    self.ImportSemaphoreWin32HandleKHR =
      transmute(loader(device, vkImportSemaphoreWin32HandleKHR_NAME.as_ptr()));
    self.ImportSemaphoreZirconHandleFUCHSIA =
      transmute(loader(device, vkImportSemaphoreZirconHandleFUCHSIA_NAME.as_ptr()));
    self.InitializePerformanceApiINTEL =
      transmute(loader(device, vkInitializePerformanceApiINTEL_NAME.as_ptr()));
    self.InvalidateMappedMemoryRanges =
      transmute(loader(device, vkInvalidateMappedMemoryRanges_NAME.as_ptr()));
    self.MapMemory = transmute(loader(device, vkMapMemory_NAME.as_ptr()));
    self.MergePipelineCaches =
      transmute(loader(device, vkMergePipelineCaches_NAME.as_ptr()));
    self.MergeValidationCachesEXT =
      transmute(loader(device, vkMergeValidationCachesEXT_NAME.as_ptr()));
    self.QueueBeginDebugUtilsLabelEXT =
      transmute(loader(device, vkQueueBeginDebugUtilsLabelEXT_NAME.as_ptr()));
    self.QueueBindSparse = transmute(loader(device, vkQueueBindSparse_NAME.as_ptr()));
    self.QueueEndDebugUtilsLabelEXT =
      transmute(loader(device, vkQueueEndDebugUtilsLabelEXT_NAME.as_ptr()));
    self.QueueInsertDebugUtilsLabelEXT =
      transmute(loader(device, vkQueueInsertDebugUtilsLabelEXT_NAME.as_ptr()));
    self.QueuePresentKHR = transmute(loader(device, vkQueuePresentKHR_NAME.as_ptr()));
    self.QueueSetPerformanceConfigurationINTEL =
      transmute(loader(device, vkQueueSetPerformanceConfigurationINTEL_NAME.as_ptr()));
    self.QueueSignalReleaseImageANDROID =
      transmute(loader(device, vkQueueSignalReleaseImageANDROID_NAME.as_ptr()));
    self.QueueSubmit = transmute(loader(device, vkQueueSubmit_NAME.as_ptr()));
    self.QueueSubmit2 = transmute(loader(device, vkQueueSubmit2_NAME.as_ptr()));
    self.QueueWaitIdle = transmute(loader(device, vkQueueWaitIdle_NAME.as_ptr()));
    self.RegisterDeviceEventEXT =
      transmute(loader(device, vkRegisterDeviceEventEXT_NAME.as_ptr()));
    self.RegisterDisplayEventEXT =
      transmute(loader(device, vkRegisterDisplayEventEXT_NAME.as_ptr()));
    self.ReleaseFullScreenExclusiveModeEXT =
      transmute(loader(device, vkReleaseFullScreenExclusiveModeEXT_NAME.as_ptr()));
    self.ReleasePerformanceConfigurationINTEL =
      transmute(loader(device, vkReleasePerformanceConfigurationINTEL_NAME.as_ptr()));
    self.ReleaseProfilingLockKHR =
      transmute(loader(device, vkReleaseProfilingLockKHR_NAME.as_ptr()));
    self.ReleaseSwapchainImagesEXT =
      transmute(loader(device, vkReleaseSwapchainImagesEXT_NAME.as_ptr()));
    self.ResetCommandBuffer =
      transmute(loader(device, vkResetCommandBuffer_NAME.as_ptr()));
    self.ResetCommandPool = transmute(loader(device, vkResetCommandPool_NAME.as_ptr()));
    self.ResetDescriptorPool =
      transmute(loader(device, vkResetDescriptorPool_NAME.as_ptr()));
    self.ResetEvent = transmute(loader(device, vkResetEvent_NAME.as_ptr()));
    self.ResetFences = transmute(loader(device, vkResetFences_NAME.as_ptr()));
    self.ResetQueryPool = transmute(loader(device, vkResetQueryPool_NAME.as_ptr()));
    self.SetBufferCollectionBufferConstraintsFUCHSIA = transmute(loader(
      device,
      vkSetBufferCollectionBufferConstraintsFUCHSIA_NAME.as_ptr(),
    ));
    self.SetBufferCollectionImageConstraintsFUCHSIA = transmute(loader(
      device,
      vkSetBufferCollectionImageConstraintsFUCHSIA_NAME.as_ptr(),
    ));
    self.SetDebugUtilsObjectNameEXT =
      transmute(loader(device, vkSetDebugUtilsObjectNameEXT_NAME.as_ptr()));
    self.SetDebugUtilsObjectTagEXT =
      transmute(loader(device, vkSetDebugUtilsObjectTagEXT_NAME.as_ptr()));
    self.SetDeviceMemoryPriorityEXT =
      transmute(loader(device, vkSetDeviceMemoryPriorityEXT_NAME.as_ptr()));
    self.SetEvent = transmute(loader(device, vkSetEvent_NAME.as_ptr()));
    self.SetHdrMetadataEXT = transmute(loader(device, vkSetHdrMetadataEXT_NAME.as_ptr()));
    self.SetLocalDimmingAMD =
      transmute(loader(device, vkSetLocalDimmingAMD_NAME.as_ptr()));
    self.SetPrivateData = transmute(loader(device, vkSetPrivateData_NAME.as_ptr()));
    self.SignalSemaphore = transmute(loader(device, vkSignalSemaphore_NAME.as_ptr()));
    self.TrimCommandPool = transmute(loader(device, vkTrimCommandPool_NAME.as_ptr()));
    self.UninitializePerformanceApiINTEL =
      transmute(loader(device, vkUninitializePerformanceApiINTEL_NAME.as_ptr()));
    self.UnmapMemory = transmute(loader(device, vkUnmapMemory_NAME.as_ptr()));
    self.UpdateDescriptorSetWithTemplate =
      transmute(loader(device, vkUpdateDescriptorSetWithTemplate_NAME.as_ptr()));
    self.UpdateDescriptorSets =
      transmute(loader(device, vkUpdateDescriptorSets_NAME.as_ptr()));
    self.UpdateVideoSessionParametersKHR =
      transmute(loader(device, vkUpdateVideoSessionParametersKHR_NAME.as_ptr()));
    self.WaitForFences = transmute(loader(device, vkWaitForFences_NAME.as_ptr()));
    self.WaitForPresentKHR = transmute(loader(device, vkWaitForPresentKHR_NAME.as_ptr()));
    self.WaitSemaphores = transmute(loader(device, vkWaitSemaphores_NAME.as_ptr()));
    self.WriteAccelerationStructuresPropertiesKHR =
      transmute(loader(device, vkWriteAccelerationStructuresPropertiesKHR_NAME.as_ptr()));
    self.WriteMicromapsPropertiesEXT =
      transmute(loader(device, vkWriteMicromapsPropertiesEXT_NAME.as_ptr()));
  }
}
