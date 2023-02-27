#![allow(dead_code)]
#![allow(nonstandard_style)]

use crate::prelude::*;

// NoHandle
/*
vkCreateInstance
vkEnumerateInstanceExtensionProperties
vkEnumerateInstanceLayerProperties
*/

// VkDevice
/*
vkAllocateCommandBuffers
vkAllocateDescriptorSets
vkAllocateMemory
vkBindBufferMemory
vkBindImageMemory
vkCreateBuffer
vkCreateBufferView
vkCreateCommandPool
vkCreateComputePipelines
vkCreateDescriptorPool
vkCreateDescriptorSetLayout
vkCreateEvent
vkCreateFence
vkCreateFramebuffer
vkCreateGraphicsPipelines
vkCreateImage
vkCreateImageView
vkCreatePipelineCache
vkCreatePipelineLayout
vkCreateQueryPool
vkCreateRenderPass
vkCreateSampler
vkCreateSemaphore
vkCreateShaderModule
vkDestroyBuffer
vkDestroyBufferView
vkDestroyCommandPool
vkDestroyDescriptorPool
vkDestroyDescriptorSetLayout
vkDestroyDevice
vkDestroyEvent
vkDestroyFence
vkDestroyFramebuffer
vkDestroyImage
vkDestroyImageView
vkDestroyPipeline
vkDestroyPipelineCache
vkDestroyPipelineLayout
vkDestroyQueryPool
vkDestroyRenderPass
vkDestroySampler
vkDestroySemaphore
vkDestroyShaderModule
vkDeviceWaitIdle
vkFlushMappedMemoryRanges
vkFreeCommandBuffers
vkFreeDescriptorSets
vkFreeMemory
vkGetBufferMemoryRequirements
vkGetDeviceMemoryCommitment
vkGetDeviceProcAddr
vkGetDeviceQueue
vkGetEventStatus
vkGetFenceStatus
vkGetImageMemoryRequirements
vkGetImageSparseMemoryRequirements
vkGetImageSubresourceLayout
vkGetPipelineCacheData
vkGetQueryPoolResults
vkGetRenderAreaGranularity
vkInvalidateMappedMemoryRanges
vkMapMemory
vkMergePipelineCaches
vkResetCommandPool
vkResetDescriptorPool
vkResetEvent
vkResetFences
vkSetEvent
vkUnmapMemory
vkUpdateDescriptorSets
vkWaitForFences
*/

// VkDevice_VkCommandBuffer
/*
vkBeginCommandBuffer
vkCmdBeginQuery
vkCmdBeginRenderPass
vkCmdBindDescriptorSets
vkCmdBindIndexBuffer
vkCmdBindPipeline
vkCmdBindVertexBuffers
vkCmdBlitImage
vkCmdClearAttachments
vkCmdClearColorImage
vkCmdClearDepthStencilImage
vkCmdCopyBuffer
vkCmdCopyBufferToImage
vkCmdCopyImage
vkCmdCopyImageToBuffer
vkCmdCopyQueryPoolResults
vkCmdDispatch
vkCmdDispatchIndirect
vkCmdDraw
vkCmdDrawIndexed
vkCmdDrawIndexedIndirect
vkCmdDrawIndirect
vkCmdEndQuery
vkCmdEndRenderPass
vkCmdExecuteCommands
vkCmdFillBuffer
vkCmdNextSubpass
vkCmdPipelineBarrier
vkCmdPushConstants
vkCmdResetEvent
vkCmdResetQueryPool
vkCmdResolveImage
vkCmdSetBlendConstants
vkCmdSetDepthBias
vkCmdSetDepthBounds
vkCmdSetEvent
vkCmdSetLineWidth
vkCmdSetScissor
vkCmdSetStencilCompareMask
vkCmdSetStencilReference
vkCmdSetStencilWriteMask
vkCmdSetViewport
vkCmdUpdateBuffer
vkCmdWaitEvents
vkCmdWriteTimestamp
vkEndCommandBuffer
vkResetCommandBuffer
*/

// VkDevice_VkQueue
/*
vkQueueBindSparse
vkQueueSubmit
vkQueueWaitIdle
*/

// VkInstance
/*
vkDestroyInstance
vkEnumeratePhysicalDevices
vkGetInstanceProcAddr
*/

// VkInstance_VkPhysicalDevice
/*
vkCreateDevice
vkEnumerateDeviceExtensionProperties
vkEnumerateDeviceLayerProperties
vkGetPhysicalDeviceFeatures
vkGetPhysicalDeviceFormatProperties
vkGetPhysicalDeviceImageFormatProperties
vkGetPhysicalDeviceMemoryProperties
vkGetPhysicalDeviceProperties
vkGetPhysicalDeviceQueueFamilyProperties
vkGetPhysicalDeviceSparseImageFormatProperties
*/

/// Khronos: [vkAllocateCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)
#[rustfmt::skip]
pub(crate) type vkAllocateCommandBuffers_t = unsafe extern "system" fn(
  device: VkDevice,
  allocate_info: *const VkCommandBufferAllocateInfo,
  command_buffers: *mut VkCommandBuffer,
) -> VkResult;
/// Khronos: [vkAllocateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html)
#[rustfmt::skip]
pub(crate) type vkAllocateDescriptorSets_t = unsafe extern "system" fn(
  device: VkDevice,
  allocate_info: *const VkDescriptorSetAllocateInfo,
  descriptor_sets: *mut VkDescriptorSet,
) -> VkResult;
/// Khronos: [vkAllocateMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)
#[rustfmt::skip]
pub(crate) type vkAllocateMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  allocate_info: *const VkMemoryAllocateInfo,
  allocator: *const VkAllocationCallbacks,
  memory: *mut VkDeviceMemory,
) -> VkResult;
/// Khronos: [vkBeginCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)
#[rustfmt::skip]
pub(crate) type vkBeginCommandBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  begin_info: *const VkCommandBufferBeginInfo,
) -> VkResult;
/// Khronos: [vkBindBufferMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html)
#[rustfmt::skip]
pub(crate) type vkBindBufferMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  buffer: VkBuffer,
  memory: VkDeviceMemory,
  memory_offset: VkDeviceSize,
) -> VkResult;
/// Khronos: [vkBindImageMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html)
#[rustfmt::skip]
pub(crate) type vkBindImageMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  memory: VkDeviceMemory,
  memory_offset: VkDeviceSize,
) -> VkResult;
/// Khronos: [vkCmdBeginQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html)
#[rustfmt::skip]
pub(crate) type vkCmdBeginQuery_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  query: u32,
  flags: VkQueryControlFlags,
) -> ();
/// Khronos: [vkCmdBeginRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html)
#[rustfmt::skip]
pub(crate) type vkCmdBeginRenderPass_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  render_pass_begin: *const VkRenderPassBeginInfo,
  contents: VkSubpassContents,
) -> ();
/// Khronos: [vkCmdBindDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html)
#[rustfmt::skip]
pub(crate) type vkCmdBindDescriptorSets_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_bind_point: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  first_set: u32,
  descriptor_set_count: u32,
  descriptor_sets: *const VkDescriptorSet,
  dynamic_offset_count: u32,
  dynamic_offsets: *const u32,
) -> ();
/// Khronos: [vkCmdBindIndexBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html)
#[rustfmt::skip]
pub(crate) type vkCmdBindIndexBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  index_type: VkIndexType,
) -> ();
/// Khronos: [vkCmdBindPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html)
#[rustfmt::skip]
pub(crate) type vkCmdBindPipeline_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_bind_point: VkPipelineBindPoint,
  pipeline: VkPipeline,
) -> ();
/// Khronos: [vkCmdBindVertexBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html)
#[rustfmt::skip]
pub(crate) type vkCmdBindVertexBuffers_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_binding: u32,
  binding_count: u32,
  buffers: *const VkBuffer,
  offsets: *const VkDeviceSize,
) -> ();
/// Khronos: [vkCmdBlitImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html)
#[rustfmt::skip]
pub(crate) type vkCmdBlitImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_image: VkImage,
  src_image_layout: VkImageLayout,
  dst_image: VkImage,
  dst_image_layout: VkImageLayout,
  region_count: u32,
  regions: *const VkImageBlit,
  filter: VkFilter,
) -> ();
/// Khronos: [vkCmdClearAttachments](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html)
#[rustfmt::skip]
pub(crate) type vkCmdClearAttachments_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  attachment_count: u32,
  attachments: *const VkClearAttachment,
  rect_count: u32,
  rects: *const VkClearRect,
) -> ();
/// Khronos: [vkCmdClearColorImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html)
#[rustfmt::skip]
pub(crate) type vkCmdClearColorImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  image: VkImage,
  image_layout: VkImageLayout,
  color: *const VkClearColorValue,
  range_count: u32,
  ranges: *const VkImageSubresourceRange,
) -> ();
/// Khronos: [vkCmdClearDepthStencilImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html)
#[rustfmt::skip]
pub(crate) type vkCmdClearDepthStencilImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  image: VkImage,
  image_layout: VkImageLayout,
  depth_stencil: *const VkClearDepthStencilValue,
  range_count: u32,
  ranges: *const VkImageSubresourceRange,
) -> ();
/// Khronos: [vkCmdCopyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html)
#[rustfmt::skip]
pub(crate) type vkCmdCopyBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_buffer: VkBuffer,
  dst_buffer: VkBuffer,
  region_count: u32,
  regions: *const VkBufferCopy,
) -> ();
/// Khronos: [vkCmdCopyBufferToImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html)
#[rustfmt::skip]
pub(crate) type vkCmdCopyBufferToImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_buffer: VkBuffer,
  dst_image: VkImage,
  dst_image_layout: VkImageLayout,
  region_count: u32,
  regions: *const VkBufferImageCopy,
) -> ();
/// Khronos: [vkCmdCopyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html)
#[rustfmt::skip]
pub(crate) type vkCmdCopyImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_image: VkImage,
  src_image_layout: VkImageLayout,
  dst_image: VkImage,
  dst_image_layout: VkImageLayout,
  region_count: u32,
  regions: *const VkImageCopy,
) -> ();
/// Khronos: [vkCmdCopyImageToBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html)
#[rustfmt::skip]
pub(crate) type vkCmdCopyImageToBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_image: VkImage,
  src_image_layout: VkImageLayout,
  dst_buffer: VkBuffer,
  region_count: u32,
  regions: *const VkBufferImageCopy,
) -> ();
/// Khronos: [vkCmdCopyQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html)
#[rustfmt::skip]
pub(crate) type vkCmdCopyQueryPoolResults_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  first_query: u32,
  query_count: u32,
  dst_buffer: VkBuffer,
  dst_offset: VkDeviceSize,
  stride: VkDeviceSize,
  flags: VkQueryResultFlags,
) -> ();
/// Khronos: [vkCmdDispatch](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)
#[rustfmt::skip]
pub(crate) type vkCmdDispatch_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  group_count_x: u32,
  group_count_y: u32,
  group_count_z: u32,
) -> ();
/// Khronos: [vkCmdDispatchIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html)
#[rustfmt::skip]
pub(crate) type vkCmdDispatchIndirect_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
) -> ();
/// Khronos: [vkCmdDraw](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html)
#[rustfmt::skip]
pub(crate) type vkCmdDraw_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  vertex_count: u32,
  instance_count: u32,
  first_vertex: u32,
  first_instance: u32,
) -> ();
/// Khronos: [vkCmdDrawIndexed](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html)
#[rustfmt::skip]
pub(crate) type vkCmdDrawIndexed_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  index_count: u32,
  instance_count: u32,
  first_index: u32,
  vertex_offset: i32,
  first_instance: u32,
) -> ();
/// Khronos: [vkCmdDrawIndexedIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html)
#[rustfmt::skip]
pub(crate) type vkCmdDrawIndexedIndirect_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  draw_count: u32,
  stride: u32,
) -> ();
/// Khronos: [vkCmdDrawIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html)
#[rustfmt::skip]
pub(crate) type vkCmdDrawIndirect_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  draw_count: u32,
  stride: u32,
) -> ();
/// Khronos: [vkCmdEndQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html)
#[rustfmt::skip]
pub(crate) type vkCmdEndQuery_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  query: u32,
) -> ();
/// Khronos: [vkCmdEndRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html)
#[rustfmt::skip]
pub(crate) type vkCmdEndRenderPass_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
) -> ();
/// Khronos: [vkCmdExecuteCommands](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html)
#[rustfmt::skip]
pub(crate) type vkCmdExecuteCommands_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  command_buffer_count: u32,
  command_buffers: *const VkCommandBuffer,
) -> ();
/// Khronos: [vkCmdFillBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html)
#[rustfmt::skip]
pub(crate) type vkCmdFillBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  dst_buffer: VkBuffer,
  dst_offset: VkDeviceSize,
  size: VkDeviceSize,
  data: u32,
) -> ();
/// Khronos: [vkCmdNextSubpass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html)
#[rustfmt::skip]
pub(crate) type vkCmdNextSubpass_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  contents: VkSubpassContents,
) -> ();
/// Khronos: [vkCmdPipelineBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html)
#[rustfmt::skip]
pub(crate) type vkCmdPipelineBarrier_t = unsafe extern "system" fn(
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
) -> ();
/// Khronos: [vkCmdPushConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html)
#[rustfmt::skip]
pub(crate) type vkCmdPushConstants_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  layout: VkPipelineLayout,
  stage_flags: VkShaderStageFlags,
  offset: u32,
  size: u32,
  values: *const c_void,
) -> ();
/// Khronos: [vkCmdResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html)
#[rustfmt::skip]
pub(crate) type vkCmdResetEvent_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  event: VkEvent,
  stage_mask: VkPipelineStageFlags,
) -> ();
/// Khronos: [vkCmdResetQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html)
#[rustfmt::skip]
pub(crate) type vkCmdResetQueryPool_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  query_pool: VkQueryPool,
  first_query: u32,
  query_count: u32,
) -> ();
/// Khronos: [vkCmdResolveImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html)
#[rustfmt::skip]
pub(crate) type vkCmdResolveImage_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  src_image: VkImage,
  src_image_layout: VkImageLayout,
  dst_image: VkImage,
  dst_image_layout: VkImageLayout,
  region_count: u32,
  regions: *const VkImageResolve,
) -> ();
/// Khronos: [vkCmdSetBlendConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetBlendConstants_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  blend_constants: *const [c_float; 4],
) -> ();
/// Khronos: [vkCmdSetDepthBias](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetDepthBias_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  depth_bias_constant_factor: c_float,
  depth_bias_clamp: c_float,
  depth_bias_slope_factor: c_float,
) -> ();
/// Khronos: [vkCmdSetDepthBounds](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetDepthBounds_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  min_depth_bounds: c_float,
  max_depth_bounds: c_float,
) -> ();
/// Khronos: [vkCmdSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetEvent_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  event: VkEvent,
  stage_mask: VkPipelineStageFlags,
) -> ();
/// Khronos: [vkCmdSetLineWidth](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetLineWidth_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  line_width: c_float,
) -> ();
/// Khronos: [vkCmdSetScissor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetScissor_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_scissor: u32,
  scissor_count: u32,
  scissors: *const VkRect2D,
) -> ();
/// Khronos: [vkCmdSetStencilCompareMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetStencilCompareMask_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  face_mask: VkStencilFaceFlags,
  compare_mask: u32,
) -> ();
/// Khronos: [vkCmdSetStencilReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetStencilReference_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  face_mask: VkStencilFaceFlags,
  reference: u32,
) -> ();
/// Khronos: [vkCmdSetStencilWriteMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetStencilWriteMask_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  face_mask: VkStencilFaceFlags,
  write_mask: u32,
) -> ();
/// Khronos: [vkCmdSetViewport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html)
#[rustfmt::skip]
pub(crate) type vkCmdSetViewport_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  first_viewport: u32,
  viewport_count: u32,
  viewports: *const VkViewport,
) -> ();
/// Khronos: [vkCmdUpdateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html)
#[rustfmt::skip]
pub(crate) type vkCmdUpdateBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  dst_buffer: VkBuffer,
  dst_offset: VkDeviceSize,
  data_size: VkDeviceSize,
  data: *const c_void,
) -> ();
/// Khronos: [vkCmdWaitEvents](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html)
#[rustfmt::skip]
pub(crate) type vkCmdWaitEvents_t = unsafe extern "system" fn(
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
) -> ();
/// Khronos: [vkCmdWriteTimestamp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html)
#[rustfmt::skip]
pub(crate) type vkCmdWriteTimestamp_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  pipeline_stage: VkPipelineStageFlagBits,
  query_pool: VkQueryPool,
  query: u32,
) -> ();
/// Khronos: [vkCreateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)
#[rustfmt::skip]
pub(crate) type vkCreateBuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkBufferCreateInfo,
  allocator: *const VkAllocationCallbacks,
  buffer: *mut VkBuffer,
) -> VkResult;
/// Khronos: [vkCreateBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html)
#[rustfmt::skip]
pub(crate) type vkCreateBufferView_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkBufferViewCreateInfo,
  allocator: *const VkAllocationCallbacks,
  view: *mut VkBufferView,
) -> VkResult;
/// Khronos: [vkCreateCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)
#[rustfmt::skip]
pub(crate) type vkCreateCommandPool_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkCommandPoolCreateInfo,
  allocator: *const VkAllocationCallbacks,
  command_pool: *mut VkCommandPool,
) -> VkResult;
/// Khronos: [vkCreateComputePipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html)
#[rustfmt::skip]
pub(crate) type vkCreateComputePipelines_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_cache: VkPipelineCache,
  create_info_count: u32,
  create_infos: *const VkComputePipelineCreateInfo,
  allocator: *const VkAllocationCallbacks,
  pipelines: *mut VkPipeline,
) -> VkResult;
/// Khronos: [vkCreateDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html)
#[rustfmt::skip]
pub(crate) type vkCreateDescriptorPool_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkDescriptorPoolCreateInfo,
  allocator: *const VkAllocationCallbacks,
  descriptor_pool: *mut VkDescriptorPool,
) -> VkResult;
/// Khronos: [vkCreateDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)
#[rustfmt::skip]
pub(crate) type vkCreateDescriptorSetLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkDescriptorSetLayoutCreateInfo,
  allocator: *const VkAllocationCallbacks,
  set_layout: *mut VkDescriptorSetLayout,
) -> VkResult;
/// Khronos: [vkCreateDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)
#[rustfmt::skip]
pub(crate) type vkCreateDevice_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  create_info: *const VkDeviceCreateInfo,
  allocator: *const VkAllocationCallbacks,
  device: *mut VkDevice,
) -> VkResult;
/// Khronos: [vkCreateEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html)
#[rustfmt::skip]
pub(crate) type vkCreateEvent_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkEventCreateInfo,
  allocator: *const VkAllocationCallbacks,
  event: *mut VkEvent,
) -> VkResult;
/// Khronos: [vkCreateFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html)
#[rustfmt::skip]
pub(crate) type vkCreateFence_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkFenceCreateInfo,
  allocator: *const VkAllocationCallbacks,
  fence: *mut VkFence,
) -> VkResult;
/// Khronos: [vkCreateFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html)
#[rustfmt::skip]
pub(crate) type vkCreateFramebuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkFramebufferCreateInfo,
  allocator: *const VkAllocationCallbacks,
  framebuffer: *mut VkFramebuffer,
) -> VkResult;
/// Khronos: [vkCreateGraphicsPipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html)
#[rustfmt::skip]
pub(crate) type vkCreateGraphicsPipelines_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_cache: VkPipelineCache,
  create_info_count: u32,
  create_infos: *const VkGraphicsPipelineCreateInfo,
  allocator: *const VkAllocationCallbacks,
  pipelines: *mut VkPipeline,
) -> VkResult;
/// Khronos: [vkCreateImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html)
#[rustfmt::skip]
pub(crate) type vkCreateImage_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkImageCreateInfo,
  allocator: *const VkAllocationCallbacks,
  image: *mut VkImage,
) -> VkResult;
/// Khronos: [vkCreateImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html)
#[rustfmt::skip]
pub(crate) type vkCreateImageView_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkImageViewCreateInfo,
  allocator: *const VkAllocationCallbacks,
  view: *mut VkImageView,
) -> VkResult;
/// Khronos: [vkCreateInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)
#[rustfmt::skip]
pub(crate) type vkCreateInstance_t = unsafe extern "system" fn(
  create_info: *const VkInstanceCreateInfo,
  allocator: *const VkAllocationCallbacks,
  instance: *mut VkInstance,
) -> VkResult;
/// Khronos: [vkCreatePipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html)
#[rustfmt::skip]
pub(crate) type vkCreatePipelineCache_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkPipelineCacheCreateInfo,
  allocator: *const VkAllocationCallbacks,
  pipeline_cache: *mut VkPipelineCache,
) -> VkResult;
/// Khronos: [vkCreatePipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)
#[rustfmt::skip]
pub(crate) type vkCreatePipelineLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkPipelineLayoutCreateInfo,
  allocator: *const VkAllocationCallbacks,
  pipeline_layout: *mut VkPipelineLayout,
) -> VkResult;
/// Khronos: [vkCreateQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html)
#[rustfmt::skip]
pub(crate) type vkCreateQueryPool_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkQueryPoolCreateInfo,
  allocator: *const VkAllocationCallbacks,
  query_pool: *mut VkQueryPool,
) -> VkResult;
/// Khronos: [vkCreateRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html)
#[rustfmt::skip]
pub(crate) type vkCreateRenderPass_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkRenderPassCreateInfo,
  allocator: *const VkAllocationCallbacks,
  render_pass: *mut VkRenderPass,
) -> VkResult;
/// Khronos: [vkCreateSampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html)
#[rustfmt::skip]
pub(crate) type vkCreateSampler_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkSamplerCreateInfo,
  allocator: *const VkAllocationCallbacks,
  sampler: *mut VkSampler,
) -> VkResult;
/// Khronos: [vkCreateSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)
#[rustfmt::skip]
pub(crate) type vkCreateSemaphore_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkSemaphoreCreateInfo,
  allocator: *const VkAllocationCallbacks,
  semaphore: *mut VkSemaphore,
) -> VkResult;
/// Khronos: [vkCreateShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html)
#[rustfmt::skip]
pub(crate) type vkCreateShaderModule_t = unsafe extern "system" fn(
  device: VkDevice,
  create_info: *const VkShaderModuleCreateInfo,
  allocator: *const VkAllocationCallbacks,
  shader_module: *mut VkShaderModule,
) -> VkResult;
/// Khronos: [vkDestroyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)
#[rustfmt::skip]
pub(crate) type vkDestroyBuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  buffer: VkBuffer,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html)
#[rustfmt::skip]
pub(crate) type vkDestroyBufferView_t = unsafe extern "system" fn(
  device: VkDevice,
  buffer_view: VkBufferView,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)
#[rustfmt::skip]
pub(crate) type vkDestroyCommandPool_t = unsafe extern "system" fn(
  device: VkDevice,
  command_pool: VkCommandPool,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html)
#[rustfmt::skip]
pub(crate) type vkDestroyDescriptorPool_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_pool: VkDescriptorPool,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)
#[rustfmt::skip]
pub(crate) type vkDestroyDescriptorSetLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_set_layout: VkDescriptorSetLayout,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)
#[rustfmt::skip]
pub(crate) type vkDestroyDevice_t = unsafe extern "system" fn(
  device: VkDevice,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html)
#[rustfmt::skip]
pub(crate) type vkDestroyEvent_t = unsafe extern "system" fn(
  device: VkDevice,
  event: VkEvent,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html)
#[rustfmt::skip]
pub(crate) type vkDestroyFence_t = unsafe extern "system" fn(
  device: VkDevice,
  fence: VkFence,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html)
#[rustfmt::skip]
pub(crate) type vkDestroyFramebuffer_t = unsafe extern "system" fn(
  device: VkDevice,
  framebuffer: VkFramebuffer,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html)
#[rustfmt::skip]
pub(crate) type vkDestroyImage_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html)
#[rustfmt::skip]
pub(crate) type vkDestroyImageView_t = unsafe extern "system" fn(
  device: VkDevice,
  image_view: VkImageView,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)
#[rustfmt::skip]
pub(crate) type vkDestroyInstance_t = unsafe extern "system" fn(
  instance: VkInstance,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html)
#[rustfmt::skip]
pub(crate) type vkDestroyPipeline_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline: VkPipeline,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyPipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html)
#[rustfmt::skip]
pub(crate) type vkDestroyPipelineCache_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_cache: VkPipelineCache,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyPipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)
#[rustfmt::skip]
pub(crate) type vkDestroyPipelineLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_layout: VkPipelineLayout,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html)
#[rustfmt::skip]
pub(crate) type vkDestroyQueryPool_t = unsafe extern "system" fn(
  device: VkDevice,
  query_pool: VkQueryPool,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html)
#[rustfmt::skip]
pub(crate) type vkDestroyRenderPass_t = unsafe extern "system" fn(
  device: VkDevice,
  render_pass: VkRenderPass,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroySampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html)
#[rustfmt::skip]
pub(crate) type vkDestroySampler_t = unsafe extern "system" fn(
  device: VkDevice,
  sampler: VkSampler,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroySemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)
#[rustfmt::skip]
pub(crate) type vkDestroySemaphore_t = unsafe extern "system" fn(
  device: VkDevice,
  semaphore: VkSemaphore,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDestroyShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html)
#[rustfmt::skip]
pub(crate) type vkDestroyShaderModule_t = unsafe extern "system" fn(
  device: VkDevice,
  shader_module: VkShaderModule,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkDeviceWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)
#[rustfmt::skip]
pub(crate) type vkDeviceWaitIdle_t = unsafe extern "system" fn(
  device: VkDevice,
) -> VkResult;
/// Khronos: [vkEndCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)
#[rustfmt::skip]
pub(crate) type vkEndCommandBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
) -> VkResult;
/// Khronos: [vkEnumerateDeviceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
#[rustfmt::skip]
pub(crate) type vkEnumerateDeviceExtensionProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  layer_name: *const u8,
  property_count: *mut u32,
  properties: *mut VkExtensionProperties,
) -> VkResult;
/// Khronos: [vkEnumerateDeviceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html)
#[rustfmt::skip]
pub(crate) type vkEnumerateDeviceLayerProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  property_count: *mut u32,
  properties: *mut VkLayerProperties,
) -> VkResult;
/// Khronos: [vkEnumerateInstanceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
#[rustfmt::skip]
pub(crate) type vkEnumerateInstanceExtensionProperties_t = unsafe extern "system" fn(
  layer_name: *const u8,
  property_count: *mut u32,
  properties: *mut VkExtensionProperties,
) -> VkResult;
/// Khronos: [vkEnumerateInstanceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
#[rustfmt::skip]
pub(crate) type vkEnumerateInstanceLayerProperties_t = unsafe extern "system" fn(
  property_count: *mut u32,
  properties: *mut VkLayerProperties,
) -> VkResult;
/// Khronos: [vkEnumeratePhysicalDevices](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)
#[rustfmt::skip]
pub(crate) type vkEnumeratePhysicalDevices_t = unsafe extern "system" fn(
  instance: VkInstance,
  physical_device_count: *mut u32,
  physical_devices: *mut VkPhysicalDevice,
) -> VkResult;
/// Khronos: [vkFlushMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html)
#[rustfmt::skip]
pub(crate) type vkFlushMappedMemoryRanges_t = unsafe extern "system" fn(
  device: VkDevice,
  memory_range_count: u32,
  memory_ranges: *const VkMappedMemoryRange,
) -> VkResult;
/// Khronos: [vkFreeCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)
#[rustfmt::skip]
pub(crate) type vkFreeCommandBuffers_t = unsafe extern "system" fn(
  device: VkDevice,
  command_pool: VkCommandPool,
  command_buffer_count: u32,
  command_buffers: *const VkCommandBuffer,
) -> ();
/// Khronos: [vkFreeDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html)
#[rustfmt::skip]
pub(crate) type vkFreeDescriptorSets_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_pool: VkDescriptorPool,
  descriptor_set_count: u32,
  descriptor_sets: *const VkDescriptorSet,
) -> VkResult;
/// Khronos: [vkFreeMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)
#[rustfmt::skip]
pub(crate) type vkFreeMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
  allocator: *const VkAllocationCallbacks,
) -> ();
/// Khronos: [vkGetBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html)
#[rustfmt::skip]
pub(crate) type vkGetBufferMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  buffer: VkBuffer,
  memory_requirements: *mut VkMemoryRequirements,
) -> ();
/// Khronos: [vkGetDeviceMemoryCommitment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html)
#[rustfmt::skip]
pub(crate) type vkGetDeviceMemoryCommitment_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
  committed_memory_in_bytes: *mut VkDeviceSize,
) -> ();
/// Khronos: [vkGetDeviceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)
#[rustfmt::skip]
pub(crate) type vkGetDeviceProcAddr_t = unsafe extern "system" fn(
  device: VkDevice,
  name: *const u8,
) -> PFN_vkVoidFunction;
/// Khronos: [vkGetDeviceQueue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html)
#[rustfmt::skip]
pub(crate) type vkGetDeviceQueue_t = unsafe extern "system" fn(
  device: VkDevice,
  queue_family_index: u32,
  queue_index: u32,
  queue: *mut VkQueue,
) -> ();
/// Khronos: [vkGetEventStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html)
#[rustfmt::skip]
pub(crate) type vkGetEventStatus_t = unsafe extern "system" fn(
  device: VkDevice,
  event: VkEvent,
) -> VkResult;
/// Khronos: [vkGetFenceStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html)
#[rustfmt::skip]
pub(crate) type vkGetFenceStatus_t = unsafe extern "system" fn(
  device: VkDevice,
  fence: VkFence,
) -> VkResult;
/// Khronos: [vkGetImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html)
#[rustfmt::skip]
pub(crate) type vkGetImageMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  memory_requirements: *mut VkMemoryRequirements,
) -> ();
/// Khronos: [vkGetImageSparseMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html)
#[rustfmt::skip]
pub(crate) type vkGetImageSparseMemoryRequirements_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  sparse_memory_requirement_count: *mut u32,
  sparse_memory_requirements: *mut VkSparseImageMemoryRequirements,
) -> ();
/// Khronos: [vkGetImageSubresourceLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html)
#[rustfmt::skip]
pub(crate) type vkGetImageSubresourceLayout_t = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  subresource: *const VkImageSubresource,
  layout: *mut VkSubresourceLayout,
) -> ();
/// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
#[rustfmt::skip]
pub(crate) type vkGetInstanceProcAddr_t = unsafe extern "system" fn(
  instance: VkInstance,
  name: *const u8,
) -> PFN_vkVoidFunction;
/// Khronos: [vkGetPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceFeatures_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  features: *mut VkPhysicalDeviceFeatures,
) -> ();
/// Khronos: [vkGetPhysicalDeviceFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceFormatProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  format: VkFormat,
  format_properties: *mut VkFormatProperties,
) -> ();
/// Khronos: [vkGetPhysicalDeviceImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceImageFormatProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  format: VkFormat,
  ty: VkImageType,
  tiling: VkImageTiling,
  usage: VkImageUsageFlags,
  flags: VkImageCreateFlags,
  image_format_properties: *mut VkImageFormatProperties,
) -> VkResult;
/// Khronos: [vkGetPhysicalDeviceMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceMemoryProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  memory_properties: *mut VkPhysicalDeviceMemoryProperties,
) -> ();
/// Khronos: [vkGetPhysicalDeviceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  properties: *mut VkPhysicalDeviceProperties,
) -> ();
/// Khronos: [vkGetPhysicalDeviceQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceQueueFamilyProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  queue_family_property_count: *mut u32,
  queue_family_properties: *mut VkQueueFamilyProperties,
) -> ();
/// Khronos: [vkGetPhysicalDeviceSparseImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html)
#[rustfmt::skip]
pub(crate) type vkGetPhysicalDeviceSparseImageFormatProperties_t = unsafe extern "system" fn(
  physical_device: VkPhysicalDevice,
  format: VkFormat,
  ty: VkImageType,
  samples: VkSampleCountFlagBits,
  usage: VkImageUsageFlags,
  tiling: VkImageTiling,
  property_count: *mut u32,
  properties: *mut VkSparseImageFormatProperties,
) -> ();
/// Khronos: [vkGetPipelineCacheData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html)
#[rustfmt::skip]
pub(crate) type vkGetPipelineCacheData_t = unsafe extern "system" fn(
  device: VkDevice,
  pipeline_cache: VkPipelineCache,
  data_size: *mut usize,
  data: *mut c_void,
) -> VkResult;
/// Khronos: [vkGetQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html)
#[rustfmt::skip]
pub(crate) type vkGetQueryPoolResults_t = unsafe extern "system" fn(
  device: VkDevice,
  query_pool: VkQueryPool,
  first_query: u32,
  query_count: u32,
  data_size: usize,
  data: *mut c_void,
  stride: VkDeviceSize,
  flags: VkQueryResultFlags,
) -> VkResult;
/// Khronos: [vkGetRenderAreaGranularity](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html)
#[rustfmt::skip]
pub(crate) type vkGetRenderAreaGranularity_t = unsafe extern "system" fn(
  device: VkDevice,
  render_pass: VkRenderPass,
  granularity: *mut VkExtent2D,
) -> ();
/// Khronos: [vkInvalidateMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html)
#[rustfmt::skip]
pub(crate) type vkInvalidateMappedMemoryRanges_t = unsafe extern "system" fn(
  device: VkDevice,
  memory_range_count: u32,
  memory_ranges: *const VkMappedMemoryRange,
) -> VkResult;
/// Khronos: [vkMapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html)
#[rustfmt::skip]
pub(crate) type vkMapMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
  offset: VkDeviceSize,
  size: VkDeviceSize,
  flags: VkMemoryMapFlags,
  pp_data: *mut *mut c_void,
) -> VkResult;
/// Khronos: [vkMergePipelineCaches](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html)
#[rustfmt::skip]
pub(crate) type vkMergePipelineCaches_t = unsafe extern "system" fn(
  device: VkDevice,
  dst_cache: VkPipelineCache,
  src_cache_count: u32,
  src_caches: *const VkPipelineCache,
) -> VkResult;
/// Khronos: [vkQueueBindSparse](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html)
#[rustfmt::skip]
pub(crate) type vkQueueBindSparse_t = unsafe extern "system" fn(
  queue: VkQueue,
  bind_info_count: u32,
  bind_info: *const VkBindSparseInfo,
  fence: VkFence,
) -> VkResult;
/// Khronos: [vkQueueSubmit](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html)
#[rustfmt::skip]
pub(crate) type vkQueueSubmit_t = unsafe extern "system" fn(
  queue: VkQueue,
  submit_count: u32,
  submits: *const VkSubmitInfo,
  fence: VkFence,
) -> VkResult;
/// Khronos: [vkQueueWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)
#[rustfmt::skip]
pub(crate) type vkQueueWaitIdle_t = unsafe extern "system" fn(
  queue: VkQueue,
) -> VkResult;
/// Khronos: [vkResetCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html)
#[rustfmt::skip]
pub(crate) type vkResetCommandBuffer_t = unsafe extern "system" fn(
  command_buffer: VkCommandBuffer,
  flags: VkCommandBufferResetFlags,
) -> VkResult;
/// Khronos: [vkResetCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)
#[rustfmt::skip]
pub(crate) type vkResetCommandPool_t = unsafe extern "system" fn(
  device: VkDevice,
  command_pool: VkCommandPool,
  flags: VkCommandPoolResetFlags,
) -> VkResult;
/// Khronos: [vkResetDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html)
#[rustfmt::skip]
pub(crate) type vkResetDescriptorPool_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_pool: VkDescriptorPool,
  flags: VkDescriptorPoolResetFlags,
) -> VkResult;
/// Khronos: [vkResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html)
#[rustfmt::skip]
pub(crate) type vkResetEvent_t = unsafe extern "system" fn(
  device: VkDevice,
  event: VkEvent,
) -> VkResult;
/// Khronos: [vkResetFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetFences.html)
#[rustfmt::skip]
pub(crate) type vkResetFences_t = unsafe extern "system" fn(
  device: VkDevice,
  fence_count: u32,
  fences: *const VkFence,
) -> VkResult;
/// Khronos: [vkSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html)
#[rustfmt::skip]
pub(crate) type vkSetEvent_t = unsafe extern "system" fn(
  device: VkDevice,
  event: VkEvent,
) -> VkResult;
/// Khronos: [vkUnmapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html)
#[rustfmt::skip]
pub(crate) type vkUnmapMemory_t = unsafe extern "system" fn(
  device: VkDevice,
  memory: VkDeviceMemory,
) -> ();
/// Khronos: [vkUpdateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html)
#[rustfmt::skip]
pub(crate) type vkUpdateDescriptorSets_t = unsafe extern "system" fn(
  device: VkDevice,
  descriptor_write_count: u32,
  descriptor_writes: *const VkWriteDescriptorSet,
  descriptor_copy_count: u32,
  descriptor_copies: *const VkCopyDescriptorSet,
) -> ();
/// Khronos: [vkWaitForFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html)
#[rustfmt::skip]
pub(crate) type vkWaitForFences_t = unsafe extern "system" fn(
  device: VkDevice,
  fence_count: u32,
  fences: *const VkFence,
  wait_all: VkBool32,
  timeout: u64,
) -> VkResult;
/// Khronos: [vkAllocateCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html)
pub(crate) type PFN_vkAllocateCommandBuffers = Option<vkAllocateCommandBuffers_t>;
/// Khronos: [vkAllocateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html)
pub(crate) type PFN_vkAllocateDescriptorSets = Option<vkAllocateDescriptorSets_t>;
/// Khronos: [vkAllocateMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html)
pub(crate) type PFN_vkAllocateMemory = Option<vkAllocateMemory_t>;
/// Khronos: [vkBeginCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html)
pub(crate) type PFN_vkBeginCommandBuffer = Option<vkBeginCommandBuffer_t>;
/// Khronos: [vkBindBufferMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html)
pub(crate) type PFN_vkBindBufferMemory = Option<vkBindBufferMemory_t>;
/// Khronos: [vkBindImageMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html)
pub(crate) type PFN_vkBindImageMemory = Option<vkBindImageMemory_t>;
/// Khronos: [vkCmdBeginQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html)
pub(crate) type PFN_vkCmdBeginQuery = Option<vkCmdBeginQuery_t>;
/// Khronos: [vkCmdBeginRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html)
pub(crate) type PFN_vkCmdBeginRenderPass = Option<vkCmdBeginRenderPass_t>;
/// Khronos: [vkCmdBindDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html)
pub(crate) type PFN_vkCmdBindDescriptorSets = Option<vkCmdBindDescriptorSets_t>;
/// Khronos: [vkCmdBindIndexBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html)
pub(crate) type PFN_vkCmdBindIndexBuffer = Option<vkCmdBindIndexBuffer_t>;
/// Khronos: [vkCmdBindPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html)
pub(crate) type PFN_vkCmdBindPipeline = Option<vkCmdBindPipeline_t>;
/// Khronos: [vkCmdBindVertexBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html)
pub(crate) type PFN_vkCmdBindVertexBuffers = Option<vkCmdBindVertexBuffers_t>;
/// Khronos: [vkCmdBlitImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html)
pub(crate) type PFN_vkCmdBlitImage = Option<vkCmdBlitImage_t>;
/// Khronos: [vkCmdClearAttachments](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html)
pub(crate) type PFN_vkCmdClearAttachments = Option<vkCmdClearAttachments_t>;
/// Khronos: [vkCmdClearColorImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html)
pub(crate) type PFN_vkCmdClearColorImage = Option<vkCmdClearColorImage_t>;
/// Khronos: [vkCmdClearDepthStencilImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html)
pub(crate) type PFN_vkCmdClearDepthStencilImage = Option<vkCmdClearDepthStencilImage_t>;
/// Khronos: [vkCmdCopyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html)
pub(crate) type PFN_vkCmdCopyBuffer = Option<vkCmdCopyBuffer_t>;
/// Khronos: [vkCmdCopyBufferToImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html)
pub(crate) type PFN_vkCmdCopyBufferToImage = Option<vkCmdCopyBufferToImage_t>;
/// Khronos: [vkCmdCopyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html)
pub(crate) type PFN_vkCmdCopyImage = Option<vkCmdCopyImage_t>;
/// Khronos: [vkCmdCopyImageToBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html)
pub(crate) type PFN_vkCmdCopyImageToBuffer = Option<vkCmdCopyImageToBuffer_t>;
/// Khronos: [vkCmdCopyQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html)
pub(crate) type PFN_vkCmdCopyQueryPoolResults = Option<vkCmdCopyQueryPoolResults_t>;
/// Khronos: [vkCmdDispatch](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html)
pub(crate) type PFN_vkCmdDispatch = Option<vkCmdDispatch_t>;
/// Khronos: [vkCmdDispatchIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html)
pub(crate) type PFN_vkCmdDispatchIndirect = Option<vkCmdDispatchIndirect_t>;
/// Khronos: [vkCmdDraw](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html)
pub(crate) type PFN_vkCmdDraw = Option<vkCmdDraw_t>;
/// Khronos: [vkCmdDrawIndexed](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html)
pub(crate) type PFN_vkCmdDrawIndexed = Option<vkCmdDrawIndexed_t>;
/// Khronos: [vkCmdDrawIndexedIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html)
pub(crate) type PFN_vkCmdDrawIndexedIndirect = Option<vkCmdDrawIndexedIndirect_t>;
/// Khronos: [vkCmdDrawIndirect](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html)
pub(crate) type PFN_vkCmdDrawIndirect = Option<vkCmdDrawIndirect_t>;
/// Khronos: [vkCmdEndQuery](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html)
pub(crate) type PFN_vkCmdEndQuery = Option<vkCmdEndQuery_t>;
/// Khronos: [vkCmdEndRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html)
pub(crate) type PFN_vkCmdEndRenderPass = Option<vkCmdEndRenderPass_t>;
/// Khronos: [vkCmdExecuteCommands](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html)
pub(crate) type PFN_vkCmdExecuteCommands = Option<vkCmdExecuteCommands_t>;
/// Khronos: [vkCmdFillBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html)
pub(crate) type PFN_vkCmdFillBuffer = Option<vkCmdFillBuffer_t>;
/// Khronos: [vkCmdNextSubpass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html)
pub(crate) type PFN_vkCmdNextSubpass = Option<vkCmdNextSubpass_t>;
/// Khronos: [vkCmdPipelineBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html)
pub(crate) type PFN_vkCmdPipelineBarrier = Option<vkCmdPipelineBarrier_t>;
/// Khronos: [vkCmdPushConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html)
pub(crate) type PFN_vkCmdPushConstants = Option<vkCmdPushConstants_t>;
/// Khronos: [vkCmdResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html)
pub(crate) type PFN_vkCmdResetEvent = Option<vkCmdResetEvent_t>;
/// Khronos: [vkCmdResetQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html)
pub(crate) type PFN_vkCmdResetQueryPool = Option<vkCmdResetQueryPool_t>;
/// Khronos: [vkCmdResolveImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html)
pub(crate) type PFN_vkCmdResolveImage = Option<vkCmdResolveImage_t>;
/// Khronos: [vkCmdSetBlendConstants](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html)
pub(crate) type PFN_vkCmdSetBlendConstants = Option<vkCmdSetBlendConstants_t>;
/// Khronos: [vkCmdSetDepthBias](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html)
pub(crate) type PFN_vkCmdSetDepthBias = Option<vkCmdSetDepthBias_t>;
/// Khronos: [vkCmdSetDepthBounds](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html)
pub(crate) type PFN_vkCmdSetDepthBounds = Option<vkCmdSetDepthBounds_t>;
/// Khronos: [vkCmdSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html)
pub(crate) type PFN_vkCmdSetEvent = Option<vkCmdSetEvent_t>;
/// Khronos: [vkCmdSetLineWidth](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html)
pub(crate) type PFN_vkCmdSetLineWidth = Option<vkCmdSetLineWidth_t>;
/// Khronos: [vkCmdSetScissor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html)
pub(crate) type PFN_vkCmdSetScissor = Option<vkCmdSetScissor_t>;
/// Khronos: [vkCmdSetStencilCompareMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html)
pub(crate) type PFN_vkCmdSetStencilCompareMask = Option<vkCmdSetStencilCompareMask_t>;
/// Khronos: [vkCmdSetStencilReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html)
pub(crate) type PFN_vkCmdSetStencilReference = Option<vkCmdSetStencilReference_t>;
/// Khronos: [vkCmdSetStencilWriteMask](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html)
pub(crate) type PFN_vkCmdSetStencilWriteMask = Option<vkCmdSetStencilWriteMask_t>;
/// Khronos: [vkCmdSetViewport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html)
pub(crate) type PFN_vkCmdSetViewport = Option<vkCmdSetViewport_t>;
/// Khronos: [vkCmdUpdateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html)
pub(crate) type PFN_vkCmdUpdateBuffer = Option<vkCmdUpdateBuffer_t>;
/// Khronos: [vkCmdWaitEvents](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html)
pub(crate) type PFN_vkCmdWaitEvents = Option<vkCmdWaitEvents_t>;
/// Khronos: [vkCmdWriteTimestamp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html)
pub(crate) type PFN_vkCmdWriteTimestamp = Option<vkCmdWriteTimestamp_t>;
/// Khronos: [vkCreateBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html)
pub(crate) type PFN_vkCreateBuffer = Option<vkCreateBuffer_t>;
/// Khronos: [vkCreateBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html)
pub(crate) type PFN_vkCreateBufferView = Option<vkCreateBufferView_t>;
/// Khronos: [vkCreateCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html)
pub(crate) type PFN_vkCreateCommandPool = Option<vkCreateCommandPool_t>;
/// Khronos: [vkCreateComputePipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html)
pub(crate) type PFN_vkCreateComputePipelines = Option<vkCreateComputePipelines_t>;
/// Khronos: [vkCreateDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html)
pub(crate) type PFN_vkCreateDescriptorPool = Option<vkCreateDescriptorPool_t>;
/// Khronos: [vkCreateDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html)
pub(crate) type PFN_vkCreateDescriptorSetLayout = Option<vkCreateDescriptorSetLayout_t>;
/// Khronos: [vkCreateDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html)
pub(crate) type PFN_vkCreateDevice = Option<vkCreateDevice_t>;
/// Khronos: [vkCreateEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html)
pub(crate) type PFN_vkCreateEvent = Option<vkCreateEvent_t>;
/// Khronos: [vkCreateFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html)
pub(crate) type PFN_vkCreateFence = Option<vkCreateFence_t>;
/// Khronos: [vkCreateFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html)
pub(crate) type PFN_vkCreateFramebuffer = Option<vkCreateFramebuffer_t>;
/// Khronos: [vkCreateGraphicsPipelines](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html)
pub(crate) type PFN_vkCreateGraphicsPipelines = Option<vkCreateGraphicsPipelines_t>;
/// Khronos: [vkCreateImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html)
pub(crate) type PFN_vkCreateImage = Option<vkCreateImage_t>;
/// Khronos: [vkCreateImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html)
pub(crate) type PFN_vkCreateImageView = Option<vkCreateImageView_t>;
/// Khronos: [vkCreateInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html)
pub(crate) type PFN_vkCreateInstance = Option<vkCreateInstance_t>;
/// Khronos: [vkCreatePipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html)
pub(crate) type PFN_vkCreatePipelineCache = Option<vkCreatePipelineCache_t>;
/// Khronos: [vkCreatePipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html)
pub(crate) type PFN_vkCreatePipelineLayout = Option<vkCreatePipelineLayout_t>;
/// Khronos: [vkCreateQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html)
pub(crate) type PFN_vkCreateQueryPool = Option<vkCreateQueryPool_t>;
/// Khronos: [vkCreateRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html)
pub(crate) type PFN_vkCreateRenderPass = Option<vkCreateRenderPass_t>;
/// Khronos: [vkCreateSampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html)
pub(crate) type PFN_vkCreateSampler = Option<vkCreateSampler_t>;
/// Khronos: [vkCreateSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html)
pub(crate) type PFN_vkCreateSemaphore = Option<vkCreateSemaphore_t>;
/// Khronos: [vkCreateShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html)
pub(crate) type PFN_vkCreateShaderModule = Option<vkCreateShaderModule_t>;
/// Khronos: [vkDestroyBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html)
pub(crate) type PFN_vkDestroyBuffer = Option<vkDestroyBuffer_t>;
/// Khronos: [vkDestroyBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html)
pub(crate) type PFN_vkDestroyBufferView = Option<vkDestroyBufferView_t>;
/// Khronos: [vkDestroyCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html)
pub(crate) type PFN_vkDestroyCommandPool = Option<vkDestroyCommandPool_t>;
/// Khronos: [vkDestroyDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html)
pub(crate) type PFN_vkDestroyDescriptorPool = Option<vkDestroyDescriptorPool_t>;
/// Khronos: [vkDestroyDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html)
pub(crate) type PFN_vkDestroyDescriptorSetLayout = Option<vkDestroyDescriptorSetLayout_t>;
/// Khronos: [vkDestroyDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html)
pub(crate) type PFN_vkDestroyDevice = Option<vkDestroyDevice_t>;
/// Khronos: [vkDestroyEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html)
pub(crate) type PFN_vkDestroyEvent = Option<vkDestroyEvent_t>;
/// Khronos: [vkDestroyFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html)
pub(crate) type PFN_vkDestroyFence = Option<vkDestroyFence_t>;
/// Khronos: [vkDestroyFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html)
pub(crate) type PFN_vkDestroyFramebuffer = Option<vkDestroyFramebuffer_t>;
/// Khronos: [vkDestroyImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html)
pub(crate) type PFN_vkDestroyImage = Option<vkDestroyImage_t>;
/// Khronos: [vkDestroyImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html)
pub(crate) type PFN_vkDestroyImageView = Option<vkDestroyImageView_t>;
/// Khronos: [vkDestroyInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html)
pub(crate) type PFN_vkDestroyInstance = Option<vkDestroyInstance_t>;
/// Khronos: [vkDestroyPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html)
pub(crate) type PFN_vkDestroyPipeline = Option<vkDestroyPipeline_t>;
/// Khronos: [vkDestroyPipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html)
pub(crate) type PFN_vkDestroyPipelineCache = Option<vkDestroyPipelineCache_t>;
/// Khronos: [vkDestroyPipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html)
pub(crate) type PFN_vkDestroyPipelineLayout = Option<vkDestroyPipelineLayout_t>;
/// Khronos: [vkDestroyQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html)
pub(crate) type PFN_vkDestroyQueryPool = Option<vkDestroyQueryPool_t>;
/// Khronos: [vkDestroyRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html)
pub(crate) type PFN_vkDestroyRenderPass = Option<vkDestroyRenderPass_t>;
/// Khronos: [vkDestroySampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html)
pub(crate) type PFN_vkDestroySampler = Option<vkDestroySampler_t>;
/// Khronos: [vkDestroySemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html)
pub(crate) type PFN_vkDestroySemaphore = Option<vkDestroySemaphore_t>;
/// Khronos: [vkDestroyShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html)
pub(crate) type PFN_vkDestroyShaderModule = Option<vkDestroyShaderModule_t>;
/// Khronos: [vkDeviceWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html)
pub(crate) type PFN_vkDeviceWaitIdle = Option<vkDeviceWaitIdle_t>;
/// Khronos: [vkEndCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html)
pub(crate) type PFN_vkEndCommandBuffer = Option<vkEndCommandBuffer_t>;
/// Khronos: [vkEnumerateDeviceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html)
pub(crate) type PFN_vkEnumerateDeviceExtensionProperties =
  Option<vkEnumerateDeviceExtensionProperties_t>;
/// Khronos: [vkEnumerateDeviceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html)
pub(crate) type PFN_vkEnumerateDeviceLayerProperties = Option<vkEnumerateDeviceLayerProperties_t>;
/// Khronos: [vkEnumerateInstanceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
pub(crate) type PFN_vkEnumerateInstanceExtensionProperties =
  Option<vkEnumerateInstanceExtensionProperties_t>;
/// Khronos: [vkEnumerateInstanceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
pub(crate) type PFN_vkEnumerateInstanceLayerProperties =
  Option<vkEnumerateInstanceLayerProperties_t>;
/// Khronos: [vkEnumeratePhysicalDevices](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html)
pub(crate) type PFN_vkEnumeratePhysicalDevices = Option<vkEnumeratePhysicalDevices_t>;
/// Khronos: [vkFlushMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html)
pub(crate) type PFN_vkFlushMappedMemoryRanges = Option<vkFlushMappedMemoryRanges_t>;
/// Khronos: [vkFreeCommandBuffers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html)
pub(crate) type PFN_vkFreeCommandBuffers = Option<vkFreeCommandBuffers_t>;
/// Khronos: [vkFreeDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html)
pub(crate) type PFN_vkFreeDescriptorSets = Option<vkFreeDescriptorSets_t>;
/// Khronos: [vkFreeMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html)
pub(crate) type PFN_vkFreeMemory = Option<vkFreeMemory_t>;
/// Khronos: [vkGetBufferMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html)
pub(crate) type PFN_vkGetBufferMemoryRequirements = Option<vkGetBufferMemoryRequirements_t>;
/// Khronos: [vkGetDeviceMemoryCommitment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html)
pub(crate) type PFN_vkGetDeviceMemoryCommitment = Option<vkGetDeviceMemoryCommitment_t>;
/// Khronos: [vkGetDeviceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html)
pub(crate) type PFN_vkGetDeviceProcAddr = Option<vkGetDeviceProcAddr_t>;
/// Khronos: [vkGetDeviceQueue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html)
pub(crate) type PFN_vkGetDeviceQueue = Option<vkGetDeviceQueue_t>;
/// Khronos: [vkGetEventStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html)
pub(crate) type PFN_vkGetEventStatus = Option<vkGetEventStatus_t>;
/// Khronos: [vkGetFenceStatus](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html)
pub(crate) type PFN_vkGetFenceStatus = Option<vkGetFenceStatus_t>;
/// Khronos: [vkGetImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html)
pub(crate) type PFN_vkGetImageMemoryRequirements = Option<vkGetImageMemoryRequirements_t>;
/// Khronos: [vkGetImageSparseMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html)
pub(crate) type PFN_vkGetImageSparseMemoryRequirements =
  Option<vkGetImageSparseMemoryRequirements_t>;
/// Khronos: [vkGetImageSubresourceLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html)
pub(crate) type PFN_vkGetImageSubresourceLayout = Option<vkGetImageSubresourceLayout_t>;
/// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
pub(crate) type PFN_vkGetInstanceProcAddr = Option<vkGetInstanceProcAddr_t>;
/// Khronos: [vkGetPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
pub(crate) type PFN_vkGetPhysicalDeviceFeatures = Option<vkGetPhysicalDeviceFeatures_t>;
/// Khronos: [vkGetPhysicalDeviceFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html)
pub(crate) type PFN_vkGetPhysicalDeviceFormatProperties =
  Option<vkGetPhysicalDeviceFormatProperties_t>;
/// Khronos: [vkGetPhysicalDeviceImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html)
pub(crate) type PFN_vkGetPhysicalDeviceImageFormatProperties =
  Option<vkGetPhysicalDeviceImageFormatProperties_t>;
/// Khronos: [vkGetPhysicalDeviceMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html)
pub(crate) type PFN_vkGetPhysicalDeviceMemoryProperties =
  Option<vkGetPhysicalDeviceMemoryProperties_t>;
/// Khronos: [vkGetPhysicalDeviceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html)
pub(crate) type PFN_vkGetPhysicalDeviceProperties = Option<vkGetPhysicalDeviceProperties_t>;
/// Khronos: [vkGetPhysicalDeviceQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html)
pub(crate) type PFN_vkGetPhysicalDeviceQueueFamilyProperties =
  Option<vkGetPhysicalDeviceQueueFamilyProperties_t>;
/// Khronos: [vkGetPhysicalDeviceSparseImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html)
pub(crate) type PFN_vkGetPhysicalDeviceSparseImageFormatProperties =
  Option<vkGetPhysicalDeviceSparseImageFormatProperties_t>;
/// Khronos: [vkGetPipelineCacheData](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html)
pub(crate) type PFN_vkGetPipelineCacheData = Option<vkGetPipelineCacheData_t>;
/// Khronos: [vkGetQueryPoolResults](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html)
pub(crate) type PFN_vkGetQueryPoolResults = Option<vkGetQueryPoolResults_t>;
/// Khronos: [vkGetRenderAreaGranularity](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html)
pub(crate) type PFN_vkGetRenderAreaGranularity = Option<vkGetRenderAreaGranularity_t>;
/// Khronos: [vkInvalidateMappedMemoryRanges](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html)
pub(crate) type PFN_vkInvalidateMappedMemoryRanges = Option<vkInvalidateMappedMemoryRanges_t>;
/// Khronos: [vkMapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html)
pub(crate) type PFN_vkMapMemory = Option<vkMapMemory_t>;
/// Khronos: [vkMergePipelineCaches](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html)
pub(crate) type PFN_vkMergePipelineCaches = Option<vkMergePipelineCaches_t>;
/// Khronos: [vkQueueBindSparse](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html)
pub(crate) type PFN_vkQueueBindSparse = Option<vkQueueBindSparse_t>;
/// Khronos: [vkQueueSubmit](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html)
pub(crate) type PFN_vkQueueSubmit = Option<vkQueueSubmit_t>;
/// Khronos: [vkQueueWaitIdle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html)
pub(crate) type PFN_vkQueueWaitIdle = Option<vkQueueWaitIdle_t>;
/// Khronos: [vkResetCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html)
pub(crate) type PFN_vkResetCommandBuffer = Option<vkResetCommandBuffer_t>;
/// Khronos: [vkResetCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html)
pub(crate) type PFN_vkResetCommandPool = Option<vkResetCommandPool_t>;
/// Khronos: [vkResetDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html)
pub(crate) type PFN_vkResetDescriptorPool = Option<vkResetDescriptorPool_t>;
/// Khronos: [vkResetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html)
pub(crate) type PFN_vkResetEvent = Option<vkResetEvent_t>;
/// Khronos: [vkResetFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkResetFences.html)
pub(crate) type PFN_vkResetFences = Option<vkResetFences_t>;
/// Khronos: [vkSetEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html)
pub(crate) type PFN_vkSetEvent = Option<vkSetEvent_t>;
/// Khronos: [vkUnmapMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html)
pub(crate) type PFN_vkUnmapMemory = Option<vkUnmapMemory_t>;
/// Khronos: [vkUpdateDescriptorSets](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html)
pub(crate) type PFN_vkUpdateDescriptorSets = Option<vkUpdateDescriptorSets_t>;
/// Khronos: [vkWaitForFences](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html)
pub(crate) type PFN_vkWaitForFences = Option<vkWaitForFences_t>;
pub(crate) const vkAllocateCommandBuffers_NAME: &str = "vkAllocateCommandBuffers\0";
pub(crate) const vkAllocateDescriptorSets_NAME: &str = "vkAllocateDescriptorSets\0";
pub(crate) const vkAllocateMemory_NAME: &str = "vkAllocateMemory\0";
pub(crate) const vkBeginCommandBuffer_NAME: &str = "vkBeginCommandBuffer\0";
pub(crate) const vkBindBufferMemory_NAME: &str = "vkBindBufferMemory\0";
pub(crate) const vkBindImageMemory_NAME: &str = "vkBindImageMemory\0";
pub(crate) const vkCmdBeginQuery_NAME: &str = "vkCmdBeginQuery\0";
pub(crate) const vkCmdBeginRenderPass_NAME: &str = "vkCmdBeginRenderPass\0";
pub(crate) const vkCmdBindDescriptorSets_NAME: &str = "vkCmdBindDescriptorSets\0";
pub(crate) const vkCmdBindIndexBuffer_NAME: &str = "vkCmdBindIndexBuffer\0";
pub(crate) const vkCmdBindPipeline_NAME: &str = "vkCmdBindPipeline\0";
pub(crate) const vkCmdBindVertexBuffers_NAME: &str = "vkCmdBindVertexBuffers\0";
pub(crate) const vkCmdBlitImage_NAME: &str = "vkCmdBlitImage\0";
pub(crate) const vkCmdClearAttachments_NAME: &str = "vkCmdClearAttachments\0";
pub(crate) const vkCmdClearColorImage_NAME: &str = "vkCmdClearColorImage\0";
pub(crate) const vkCmdClearDepthStencilImage_NAME: &str = "vkCmdClearDepthStencilImage\0";
pub(crate) const vkCmdCopyBuffer_NAME: &str = "vkCmdCopyBuffer\0";
pub(crate) const vkCmdCopyBufferToImage_NAME: &str = "vkCmdCopyBufferToImage\0";
pub(crate) const vkCmdCopyImage_NAME: &str = "vkCmdCopyImage\0";
pub(crate) const vkCmdCopyImageToBuffer_NAME: &str = "vkCmdCopyImageToBuffer\0";
pub(crate) const vkCmdCopyQueryPoolResults_NAME: &str = "vkCmdCopyQueryPoolResults\0";
pub(crate) const vkCmdDispatch_NAME: &str = "vkCmdDispatch\0";
pub(crate) const vkCmdDispatchIndirect_NAME: &str = "vkCmdDispatchIndirect\0";
pub(crate) const vkCmdDraw_NAME: &str = "vkCmdDraw\0";
pub(crate) const vkCmdDrawIndexed_NAME: &str = "vkCmdDrawIndexed\0";
pub(crate) const vkCmdDrawIndexedIndirect_NAME: &str = "vkCmdDrawIndexedIndirect\0";
pub(crate) const vkCmdDrawIndirect_NAME: &str = "vkCmdDrawIndirect\0";
pub(crate) const vkCmdEndQuery_NAME: &str = "vkCmdEndQuery\0";
pub(crate) const vkCmdEndRenderPass_NAME: &str = "vkCmdEndRenderPass\0";
pub(crate) const vkCmdExecuteCommands_NAME: &str = "vkCmdExecuteCommands\0";
pub(crate) const vkCmdFillBuffer_NAME: &str = "vkCmdFillBuffer\0";
pub(crate) const vkCmdNextSubpass_NAME: &str = "vkCmdNextSubpass\0";
pub(crate) const vkCmdPipelineBarrier_NAME: &str = "vkCmdPipelineBarrier\0";
pub(crate) const vkCmdPushConstants_NAME: &str = "vkCmdPushConstants\0";
pub(crate) const vkCmdResetEvent_NAME: &str = "vkCmdResetEvent\0";
pub(crate) const vkCmdResetQueryPool_NAME: &str = "vkCmdResetQueryPool\0";
pub(crate) const vkCmdResolveImage_NAME: &str = "vkCmdResolveImage\0";
pub(crate) const vkCmdSetBlendConstants_NAME: &str = "vkCmdSetBlendConstants\0";
pub(crate) const vkCmdSetDepthBias_NAME: &str = "vkCmdSetDepthBias\0";
pub(crate) const vkCmdSetDepthBounds_NAME: &str = "vkCmdSetDepthBounds\0";
pub(crate) const vkCmdSetEvent_NAME: &str = "vkCmdSetEvent\0";
pub(crate) const vkCmdSetLineWidth_NAME: &str = "vkCmdSetLineWidth\0";
pub(crate) const vkCmdSetScissor_NAME: &str = "vkCmdSetScissor\0";
pub(crate) const vkCmdSetStencilCompareMask_NAME: &str = "vkCmdSetStencilCompareMask\0";
pub(crate) const vkCmdSetStencilReference_NAME: &str = "vkCmdSetStencilReference\0";
pub(crate) const vkCmdSetStencilWriteMask_NAME: &str = "vkCmdSetStencilWriteMask\0";
pub(crate) const vkCmdSetViewport_NAME: &str = "vkCmdSetViewport\0";
pub(crate) const vkCmdUpdateBuffer_NAME: &str = "vkCmdUpdateBuffer\0";
pub(crate) const vkCmdWaitEvents_NAME: &str = "vkCmdWaitEvents\0";
pub(crate) const vkCmdWriteTimestamp_NAME: &str = "vkCmdWriteTimestamp\0";
pub(crate) const vkCreateBuffer_NAME: &str = "vkCreateBuffer\0";
pub(crate) const vkCreateBufferView_NAME: &str = "vkCreateBufferView\0";
pub(crate) const vkCreateCommandPool_NAME: &str = "vkCreateCommandPool\0";
pub(crate) const vkCreateComputePipelines_NAME: &str = "vkCreateComputePipelines\0";
pub(crate) const vkCreateDescriptorPool_NAME: &str = "vkCreateDescriptorPool\0";
pub(crate) const vkCreateDescriptorSetLayout_NAME: &str = "vkCreateDescriptorSetLayout\0";
pub(crate) const vkCreateDevice_NAME: &str = "vkCreateDevice\0";
pub(crate) const vkCreateEvent_NAME: &str = "vkCreateEvent\0";
pub(crate) const vkCreateFence_NAME: &str = "vkCreateFence\0";
pub(crate) const vkCreateFramebuffer_NAME: &str = "vkCreateFramebuffer\0";
pub(crate) const vkCreateGraphicsPipelines_NAME: &str = "vkCreateGraphicsPipelines\0";
pub(crate) const vkCreateImage_NAME: &str = "vkCreateImage\0";
pub(crate) const vkCreateImageView_NAME: &str = "vkCreateImageView\0";
pub(crate) const vkCreateInstance_NAME: &str = "vkCreateInstance\0";
pub(crate) const vkCreatePipelineCache_NAME: &str = "vkCreatePipelineCache\0";
pub(crate) const vkCreatePipelineLayout_NAME: &str = "vkCreatePipelineLayout\0";
pub(crate) const vkCreateQueryPool_NAME: &str = "vkCreateQueryPool\0";
pub(crate) const vkCreateRenderPass_NAME: &str = "vkCreateRenderPass\0";
pub(crate) const vkCreateSampler_NAME: &str = "vkCreateSampler\0";
pub(crate) const vkCreateSemaphore_NAME: &str = "vkCreateSemaphore\0";
pub(crate) const vkCreateShaderModule_NAME: &str = "vkCreateShaderModule\0";
pub(crate) const vkDestroyBuffer_NAME: &str = "vkDestroyBuffer\0";
pub(crate) const vkDestroyBufferView_NAME: &str = "vkDestroyBufferView\0";
pub(crate) const vkDestroyCommandPool_NAME: &str = "vkDestroyCommandPool\0";
pub(crate) const vkDestroyDescriptorPool_NAME: &str = "vkDestroyDescriptorPool\0";
pub(crate) const vkDestroyDescriptorSetLayout_NAME: &str = "vkDestroyDescriptorSetLayout\0";
pub(crate) const vkDestroyDevice_NAME: &str = "vkDestroyDevice\0";
pub(crate) const vkDestroyEvent_NAME: &str = "vkDestroyEvent\0";
pub(crate) const vkDestroyFence_NAME: &str = "vkDestroyFence\0";
pub(crate) const vkDestroyFramebuffer_NAME: &str = "vkDestroyFramebuffer\0";
pub(crate) const vkDestroyImage_NAME: &str = "vkDestroyImage\0";
pub(crate) const vkDestroyImageView_NAME: &str = "vkDestroyImageView\0";
pub(crate) const vkDestroyInstance_NAME: &str = "vkDestroyInstance\0";
pub(crate) const vkDestroyPipeline_NAME: &str = "vkDestroyPipeline\0";
pub(crate) const vkDestroyPipelineCache_NAME: &str = "vkDestroyPipelineCache\0";
pub(crate) const vkDestroyPipelineLayout_NAME: &str = "vkDestroyPipelineLayout\0";
pub(crate) const vkDestroyQueryPool_NAME: &str = "vkDestroyQueryPool\0";
pub(crate) const vkDestroyRenderPass_NAME: &str = "vkDestroyRenderPass\0";
pub(crate) const vkDestroySampler_NAME: &str = "vkDestroySampler\0";
pub(crate) const vkDestroySemaphore_NAME: &str = "vkDestroySemaphore\0";
pub(crate) const vkDestroyShaderModule_NAME: &str = "vkDestroyShaderModule\0";
pub(crate) const vkDeviceWaitIdle_NAME: &str = "vkDeviceWaitIdle\0";
pub(crate) const vkEndCommandBuffer_NAME: &str = "vkEndCommandBuffer\0";
pub(crate) const vkEnumerateDeviceExtensionProperties_NAME: &str =
  "vkEnumerateDeviceExtensionProperties\0";
pub(crate) const vkEnumerateDeviceLayerProperties_NAME: &str = "vkEnumerateDeviceLayerProperties\0";
pub(crate) const vkEnumerateInstanceExtensionProperties_NAME: &str =
  "vkEnumerateInstanceExtensionProperties\0";
pub(crate) const vkEnumerateInstanceLayerProperties_NAME: &str =
  "vkEnumerateInstanceLayerProperties\0";
pub(crate) const vkEnumeratePhysicalDevices_NAME: &str = "vkEnumeratePhysicalDevices\0";
pub(crate) const vkFlushMappedMemoryRanges_NAME: &str = "vkFlushMappedMemoryRanges\0";
pub(crate) const vkFreeCommandBuffers_NAME: &str = "vkFreeCommandBuffers\0";
pub(crate) const vkFreeDescriptorSets_NAME: &str = "vkFreeDescriptorSets\0";
pub(crate) const vkFreeMemory_NAME: &str = "vkFreeMemory\0";
pub(crate) const vkGetBufferMemoryRequirements_NAME: &str = "vkGetBufferMemoryRequirements\0";
pub(crate) const vkGetDeviceMemoryCommitment_NAME: &str = "vkGetDeviceMemoryCommitment\0";
pub(crate) const vkGetDeviceProcAddr_NAME: &str = "vkGetDeviceProcAddr\0";
pub(crate) const vkGetDeviceQueue_NAME: &str = "vkGetDeviceQueue\0";
pub(crate) const vkGetEventStatus_NAME: &str = "vkGetEventStatus\0";
pub(crate) const vkGetFenceStatus_NAME: &str = "vkGetFenceStatus\0";
pub(crate) const vkGetImageMemoryRequirements_NAME: &str = "vkGetImageMemoryRequirements\0";
pub(crate) const vkGetImageSparseMemoryRequirements_NAME: &str =
  "vkGetImageSparseMemoryRequirements\0";
pub(crate) const vkGetImageSubresourceLayout_NAME: &str = "vkGetImageSubresourceLayout\0";
pub(crate) const vkGetInstanceProcAddr_NAME: &str = "vkGetInstanceProcAddr\0";
pub(crate) const vkGetPhysicalDeviceFeatures_NAME: &str = "vkGetPhysicalDeviceFeatures\0";
pub(crate) const vkGetPhysicalDeviceFormatProperties_NAME: &str =
  "vkGetPhysicalDeviceFormatProperties\0";
pub(crate) const vkGetPhysicalDeviceImageFormatProperties_NAME: &str =
  "vkGetPhysicalDeviceImageFormatProperties\0";
pub(crate) const vkGetPhysicalDeviceMemoryProperties_NAME: &str =
  "vkGetPhysicalDeviceMemoryProperties\0";
pub(crate) const vkGetPhysicalDeviceProperties_NAME: &str = "vkGetPhysicalDeviceProperties\0";
pub(crate) const vkGetPhysicalDeviceQueueFamilyProperties_NAME: &str =
  "vkGetPhysicalDeviceQueueFamilyProperties\0";
pub(crate) const vkGetPhysicalDeviceSparseImageFormatProperties_NAME: &str =
  "vkGetPhysicalDeviceSparseImageFormatProperties\0";
pub(crate) const vkGetPipelineCacheData_NAME: &str = "vkGetPipelineCacheData\0";
pub(crate) const vkGetQueryPoolResults_NAME: &str = "vkGetQueryPoolResults\0";
pub(crate) const vkGetRenderAreaGranularity_NAME: &str = "vkGetRenderAreaGranularity\0";
pub(crate) const vkInvalidateMappedMemoryRanges_NAME: &str = "vkInvalidateMappedMemoryRanges\0";
pub(crate) const vkMapMemory_NAME: &str = "vkMapMemory\0";
pub(crate) const vkMergePipelineCaches_NAME: &str = "vkMergePipelineCaches\0";
pub(crate) const vkQueueBindSparse_NAME: &str = "vkQueueBindSparse\0";
pub(crate) const vkQueueSubmit_NAME: &str = "vkQueueSubmit\0";
pub(crate) const vkQueueWaitIdle_NAME: &str = "vkQueueWaitIdle\0";
pub(crate) const vkResetCommandBuffer_NAME: &str = "vkResetCommandBuffer\0";
pub(crate) const vkResetCommandPool_NAME: &str = "vkResetCommandPool\0";
pub(crate) const vkResetDescriptorPool_NAME: &str = "vkResetDescriptorPool\0";
pub(crate) const vkResetEvent_NAME: &str = "vkResetEvent\0";
pub(crate) const vkResetFences_NAME: &str = "vkResetFences\0";
pub(crate) const vkSetEvent_NAME: &str = "vkSetEvent\0";
pub(crate) const vkUnmapMemory_NAME: &str = "vkUnmapMemory\0";
pub(crate) const vkUpdateDescriptorSets_NAME: &str = "vkUpdateDescriptorSets\0";
pub(crate) const vkWaitForFences_NAME: &str = "vkWaitForFences\0";
