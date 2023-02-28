//! The device module mostly has the [`Device`] type.
//!
//! The device is the primary way that you can actually make Vulkan "do stuff".

use crate::prelude::*;

/// Represents an open connection to a Vulkan device.
///
/// The Device, or its child objects, are what you'll actually use for most of
/// controlling Vulkan during any given program.
pub struct Device<'i, 'p: 'i> {
  parent: core::marker::PhantomData<&'p PhysicalDevice<'i>>,
  vk_device: VkDevice,
  fns: DeviceFnTable,
  #[cfg(feature = "VK_KHR_swapchain")]
  vk_khr_swapchain_fns: Option<DeviceFnTable_VkKhrSwapchain>,
}
impl Drop for Device<'_, '_> {
  fn drop(&mut self) {
    if cfg!(debug_assertions) {
      panic!("Bug: you shouldn't drop a `Device`, please destroy it properly with `Device::destroy`");
    }
  }
}
impl<'i, 'p: 'i> Device<'i, 'p> {
  pub(crate) unsafe fn new(
    vkGetDeviceProcAddr: vkGetDeviceProcAddr_t, vk_device: VkDevice,
  ) -> Result<Self, NonZeroI32> {
    Ok(Self {
      parent: core::marker::PhantomData,
      vk_device,
      fns: unsafe {
        DeviceFnTable::new(vkGetDeviceProcAddr, vk_device)
          .ok_or(VK_ERROR_UNKNOWN.0.unwrap())?
      },
      vk_khr_swapchain_fns: unsafe {
        DeviceFnTable_VkKhrSwapchain::new(vkGetDeviceProcAddr, vk_device)
      },
    })
  }

  /// Destroy the device.
  ///
  /// You should call this rather than letting the device simply drop.
  #[inline]
  pub fn destroy(self) {
    unsafe { (self.fns.vkDestroyDevice)(self.vk_device, null()) };
    core::mem::forget(self);
  }

  /// Creates a swapchain.
  ///
  /// See:
  /// * Khronos: [VkSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateInfoKHR.html)
  /// * Khronos: [vkCreateSwapchainKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html)
  #[cfg(feature = "VK_KHR_swapchain")]
  pub fn create_swapchain_khr(
    &self, surface: VkSurfaceKHR, surface_format: VkSurfaceFormatKHR,
    image_extent: VkExtent2D, present_mode: VkPresentModeKHR, min_image_count: u32,
    image_usage: VkImageUsageFlags,
  ) -> Result<VkSwapchainKHR, NonZeroI32> {
    if let Some(swapchain_fns) = self.vk_khr_swapchain_fns {
      let swapchain_create_info = VkSwapchainCreateInfoKHR {
        surface,
        min_image_count,
        image_format: surface_format.format,
        image_color_space: surface_format.color_space,
        image_extent,
        image_array_layers: 1,
        image_usage,
        image_sharing_mode: VK_SHARING_MODE_EXCLUSIVE,
        pre_transform: VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
        composite_alpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
        present_mode,
        clipped: true.into(),
        ..VkSwapchainCreateInfoKHR::default()
      };
      let mut vk_swapchain = VkSwapchainKHR::NULL;
      let create_ret = unsafe {
        (swapchain_fns.vkCreateSwapchainKHR)(
          self.vk_device,
          &swapchain_create_info,
          null(),
          &mut vk_swapchain,
        )
      };
      if let Some(err_code) = create_ret.0 {
        Err(err_code)
      } else {
        Ok(vk_swapchain)
      }
    } else {
      Err(VK_ERROR_EXTENSION_NOT_PRESENT.0.unwrap())
    }
  }
}

#[derive(Clone, Copy)]
#[allow(bad_style)]
struct DeviceFnTable {
  vkAllocateCommandBuffers: vkAllocateCommandBuffers_t,
  vkAllocateDescriptorSets: vkAllocateDescriptorSets_t,
  vkAllocateMemory: vkAllocateMemory_t,
  vkBindBufferMemory: vkBindBufferMemory_t,
  vkBindImageMemory: vkBindImageMemory_t,
  vkCreateBuffer: vkCreateBuffer_t,
  vkCreateBufferView: vkCreateBufferView_t,
  vkCreateCommandPool: vkCreateCommandPool_t,
  vkCreateComputePipelines: vkCreateComputePipelines_t,
  vkCreateDescriptorPool: vkCreateDescriptorPool_t,
  vkCreateDescriptorSetLayout: vkCreateDescriptorSetLayout_t,
  vkCreateEvent: vkCreateEvent_t,
  vkCreateFence: vkCreateFence_t,
  vkCreateFramebuffer: vkCreateFramebuffer_t,
  vkCreateGraphicsPipelines: vkCreateGraphicsPipelines_t,
  vkCreateImage: vkCreateImage_t,
  vkCreateImageView: vkCreateImageView_t,
  vkCreatePipelineCache: vkCreatePipelineCache_t,
  vkCreatePipelineLayout: vkCreatePipelineLayout_t,
  vkCreateQueryPool: vkCreateQueryPool_t,
  vkCreateRenderPass: vkCreateRenderPass_t,
  vkCreateSampler: vkCreateSampler_t,
  vkCreateSemaphore: vkCreateSemaphore_t,
  vkCreateShaderModule: vkCreateShaderModule_t,
  vkDestroyBuffer: vkDestroyBuffer_t,
  vkDestroyBufferView: vkDestroyBufferView_t,
  vkDestroyCommandPool: vkDestroyCommandPool_t,
  vkDestroyDescriptorPool: vkDestroyDescriptorPool_t,
  vkDestroyDescriptorSetLayout: vkDestroyDescriptorSetLayout_t,
  vkDestroyDevice: vkDestroyDevice_t,
  vkDestroyEvent: vkDestroyEvent_t,
  vkDestroyFence: vkDestroyFence_t,
  vkDestroyFramebuffer: vkDestroyFramebuffer_t,
  vkDestroyImage: vkDestroyImage_t,
  vkDestroyImageView: vkDestroyImageView_t,
  vkDestroyPipeline: vkDestroyPipeline_t,
  vkDestroyPipelineCache: vkDestroyPipelineCache_t,
  vkDestroyPipelineLayout: vkDestroyPipelineLayout_t,
  vkDestroyQueryPool: vkDestroyQueryPool_t,
  vkDestroyRenderPass: vkDestroyRenderPass_t,
  vkDestroySampler: vkDestroySampler_t,
  vkDestroySemaphore: vkDestroySemaphore_t,
  vkDestroyShaderModule: vkDestroyShaderModule_t,
  vkDeviceWaitIdle: vkDeviceWaitIdle_t,
  vkFlushMappedMemoryRanges: vkFlushMappedMemoryRanges_t,
  vkFreeCommandBuffers: vkFreeCommandBuffers_t,
  vkFreeDescriptorSets: vkFreeDescriptorSets_t,
  vkFreeMemory: vkFreeMemory_t,
  vkGetBufferMemoryRequirements: vkGetBufferMemoryRequirements_t,
  vkGetDeviceMemoryCommitment: vkGetDeviceMemoryCommitment_t,
  vkGetDeviceProcAddr: vkGetDeviceProcAddr_t,
  vkGetDeviceQueue: vkGetDeviceQueue_t,
  vkGetEventStatus: vkGetEventStatus_t,
  vkGetFenceStatus: vkGetFenceStatus_t,
  vkGetImageMemoryRequirements: vkGetImageMemoryRequirements_t,
  vkGetImageSparseMemoryRequirements: vkGetImageSparseMemoryRequirements_t,
  vkGetImageSubresourceLayout: vkGetImageSubresourceLayout_t,
  vkGetPipelineCacheData: vkGetPipelineCacheData_t,
  vkGetQueryPoolResults: vkGetQueryPoolResults_t,
  vkGetRenderAreaGranularity: vkGetRenderAreaGranularity_t,
  vkInvalidateMappedMemoryRanges: vkInvalidateMappedMemoryRanges_t,
  vkMapMemory: vkMapMemory_t,
  vkMergePipelineCaches: vkMergePipelineCaches_t,
  vkResetCommandPool: vkResetCommandPool_t,
  vkResetDescriptorPool: vkResetDescriptorPool_t,
  vkResetEvent: vkResetEvent_t,
  vkResetFences: vkResetFences_t,
  vkSetEvent: vkSetEvent_t,
  vkUnmapMemory: vkUnmapMemory_t,
  vkUpdateDescriptorSets: vkUpdateDescriptorSets_t,
  vkWaitForFences: vkWaitForFences_t,
  vkBeginCommandBuffer: vkBeginCommandBuffer_t,
  vkCmdBeginQuery: vkCmdBeginQuery_t,
  vkCmdBeginRenderPass: vkCmdBeginRenderPass_t,
  vkCmdBindDescriptorSets: vkCmdBindDescriptorSets_t,
  vkCmdBindIndexBuffer: vkCmdBindIndexBuffer_t,
  vkCmdBindPipeline: vkCmdBindPipeline_t,
  vkCmdBindVertexBuffers: vkCmdBindVertexBuffers_t,
  vkCmdBlitImage: vkCmdBlitImage_t,
  vkCmdClearAttachments: vkCmdClearAttachments_t,
  vkCmdClearColorImage: vkCmdClearColorImage_t,
  vkCmdClearDepthStencilImage: vkCmdClearDepthStencilImage_t,
  vkCmdCopyBuffer: vkCmdCopyBuffer_t,
  vkCmdCopyBufferToImage: vkCmdCopyBufferToImage_t,
  vkCmdCopyImage: vkCmdCopyImage_t,
  vkCmdCopyImageToBuffer: vkCmdCopyImageToBuffer_t,
  vkCmdCopyQueryPoolResults: vkCmdCopyQueryPoolResults_t,
  vkCmdDispatch: vkCmdDispatch_t,
  vkCmdDispatchIndirect: vkCmdDispatchIndirect_t,
  vkCmdDraw: vkCmdDraw_t,
  vkCmdDrawIndexed: vkCmdDrawIndexed_t,
  vkCmdDrawIndexedIndirect: vkCmdDrawIndexedIndirect_t,
  vkCmdDrawIndirect: vkCmdDrawIndirect_t,
  vkCmdEndQuery: vkCmdEndQuery_t,
  vkCmdEndRenderPass: vkCmdEndRenderPass_t,
  vkCmdExecuteCommands: vkCmdExecuteCommands_t,
  vkCmdFillBuffer: vkCmdFillBuffer_t,
  vkCmdNextSubpass: vkCmdNextSubpass_t,
  vkCmdPipelineBarrier: vkCmdPipelineBarrier_t,
  vkCmdPushConstants: vkCmdPushConstants_t,
  vkCmdResetEvent: vkCmdResetEvent_t,
  vkCmdResetQueryPool: vkCmdResetQueryPool_t,
  vkCmdResolveImage: vkCmdResolveImage_t,
  vkCmdSetBlendConstants: vkCmdSetBlendConstants_t,
  vkCmdSetDepthBias: vkCmdSetDepthBias_t,
  vkCmdSetDepthBounds: vkCmdSetDepthBounds_t,
  vkCmdSetEvent: vkCmdSetEvent_t,
  vkCmdSetLineWidth: vkCmdSetLineWidth_t,
  vkCmdSetScissor: vkCmdSetScissor_t,
  vkCmdSetStencilCompareMask: vkCmdSetStencilCompareMask_t,
  vkCmdSetStencilReference: vkCmdSetStencilReference_t,
  vkCmdSetStencilWriteMask: vkCmdSetStencilWriteMask_t,
  vkCmdSetViewport: vkCmdSetViewport_t,
  vkCmdUpdateBuffer: vkCmdUpdateBuffer_t,
  vkCmdWaitEvents: vkCmdWaitEvents_t,
  vkCmdWriteTimestamp: vkCmdWriteTimestamp_t,
  vkEndCommandBuffer: vkEndCommandBuffer_t,
  vkResetCommandBuffer: vkResetCommandBuffer_t,
  vkQueueBindSparse: vkQueueBindSparse_t,
  vkQueueSubmit: vkQueueSubmit_t,
  vkQueueWaitIdle: vkQueueWaitIdle_t,
}
impl DeviceFnTable {
  #[rustfmt::skip]
  #[allow(bad_style)]
  pub(crate) unsafe fn new(
    vkGetDeviceProcAddr: vkGetDeviceProcAddr_t, vk_device: VkDevice,
  ) -> Option<Self> {
    Some(Self{
      vkAllocateCommandBuffers: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkAllocateCommandBuffers_NAME.as_ptr())?),
      vkAllocateDescriptorSets: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkAllocateDescriptorSets_NAME.as_ptr())?),
      vkAllocateMemory: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkAllocateMemory_NAME.as_ptr())?),
      vkBindBufferMemory: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkBindBufferMemory_NAME.as_ptr())?),
      vkBindImageMemory: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkBindImageMemory_NAME.as_ptr())?),
      vkCreateBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateBuffer_NAME.as_ptr())?),
      vkCreateBufferView: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateBufferView_NAME.as_ptr())?),
      vkCreateCommandPool: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateCommandPool_NAME.as_ptr())?),
      vkCreateComputePipelines: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateComputePipelines_NAME.as_ptr())?),
      vkCreateDescriptorPool: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateDescriptorPool_NAME.as_ptr())?),
      vkCreateDescriptorSetLayout: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateDescriptorSetLayout_NAME.as_ptr())?),
      vkCreateEvent: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateEvent_NAME.as_ptr())?),
      vkCreateFence: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateFence_NAME.as_ptr())?),
      vkCreateFramebuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateFramebuffer_NAME.as_ptr())?),
      vkCreateGraphicsPipelines: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateGraphicsPipelines_NAME.as_ptr())?),
      vkCreateImage: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateImage_NAME.as_ptr())?),
      vkCreateImageView: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateImageView_NAME.as_ptr())?),
      vkCreatePipelineCache: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreatePipelineCache_NAME.as_ptr())?),
      vkCreatePipelineLayout: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreatePipelineLayout_NAME.as_ptr())?),
      vkCreateQueryPool: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateQueryPool_NAME.as_ptr())?),
      vkCreateRenderPass: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateRenderPass_NAME.as_ptr())?),
      vkCreateSampler: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateSampler_NAME.as_ptr())?),
      vkCreateSemaphore: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateSemaphore_NAME.as_ptr())?),
      vkCreateShaderModule: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateShaderModule_NAME.as_ptr())?),
      vkDestroyBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyBuffer_NAME.as_ptr())?),
      vkDestroyBufferView: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyBufferView_NAME.as_ptr())?),
      vkDestroyCommandPool: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyCommandPool_NAME.as_ptr())?),
      vkDestroyDescriptorPool: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyDescriptorPool_NAME.as_ptr())?),
      vkDestroyDescriptorSetLayout: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyDescriptorSetLayout_NAME.as_ptr())?),
      vkDestroyDevice: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyDevice_NAME.as_ptr())?),
      vkDestroyEvent: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyEvent_NAME.as_ptr())?),
      vkDestroyFence: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyFence_NAME.as_ptr())?),
      vkDestroyFramebuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyFramebuffer_NAME.as_ptr())?),
      vkDestroyImage: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyImage_NAME.as_ptr())?),
      vkDestroyImageView: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyImageView_NAME.as_ptr())?),
      vkDestroyPipeline: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyPipeline_NAME.as_ptr())?),
      vkDestroyPipelineCache: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyPipelineCache_NAME.as_ptr())?),
      vkDestroyPipelineLayout: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyPipelineLayout_NAME.as_ptr())?),
      vkDestroyQueryPool: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyQueryPool_NAME.as_ptr())?),
      vkDestroyRenderPass: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyRenderPass_NAME.as_ptr())?),
      vkDestroySampler: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroySampler_NAME.as_ptr())?),
      vkDestroySemaphore: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroySemaphore_NAME.as_ptr())?),
      vkDestroyShaderModule: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroyShaderModule_NAME.as_ptr())?),
      vkDeviceWaitIdle: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDeviceWaitIdle_NAME.as_ptr())?),
      vkFlushMappedMemoryRanges: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkFlushMappedMemoryRanges_NAME.as_ptr())?),
      vkFreeCommandBuffers: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkFreeCommandBuffers_NAME.as_ptr())?),
      vkFreeDescriptorSets: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkFreeDescriptorSets_NAME.as_ptr())?),
      vkFreeMemory: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkFreeMemory_NAME.as_ptr())?),
      vkGetBufferMemoryRequirements: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetBufferMemoryRequirements_NAME.as_ptr())?),
      vkGetDeviceMemoryCommitment: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetDeviceMemoryCommitment_NAME.as_ptr())?),
      vkGetDeviceProcAddr: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetDeviceProcAddr_NAME.as_ptr())?),
      vkGetDeviceQueue: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetDeviceQueue_NAME.as_ptr())?),
      vkGetEventStatus: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetEventStatus_NAME.as_ptr())?),
      vkGetFenceStatus: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetFenceStatus_NAME.as_ptr())?),
      vkGetImageMemoryRequirements: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetImageMemoryRequirements_NAME.as_ptr())?),
      vkGetImageSparseMemoryRequirements: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetImageSparseMemoryRequirements_NAME.as_ptr())?),
      vkGetImageSubresourceLayout: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetImageSubresourceLayout_NAME.as_ptr())?),
      vkGetPipelineCacheData: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetPipelineCacheData_NAME.as_ptr())?),
      vkGetQueryPoolResults: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetQueryPoolResults_NAME.as_ptr())?),
      vkGetRenderAreaGranularity: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetRenderAreaGranularity_NAME.as_ptr())?),
      vkInvalidateMappedMemoryRanges: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkInvalidateMappedMemoryRanges_NAME.as_ptr())?),
      vkMapMemory: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkMapMemory_NAME.as_ptr())?),
      vkMergePipelineCaches: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkMergePipelineCaches_NAME.as_ptr())?),
      vkResetCommandPool: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkResetCommandPool_NAME.as_ptr())?),
      vkResetDescriptorPool: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkResetDescriptorPool_NAME.as_ptr())?),
      vkResetEvent: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkResetEvent_NAME.as_ptr())?),
      vkResetFences: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkResetFences_NAME.as_ptr())?),
      vkSetEvent: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkSetEvent_NAME.as_ptr())?),
      vkUnmapMemory: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkUnmapMemory_NAME.as_ptr())?),
      vkUpdateDescriptorSets: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkUpdateDescriptorSets_NAME.as_ptr())?),
      vkWaitForFences: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkWaitForFences_NAME.as_ptr())?),
      vkBeginCommandBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkBeginCommandBuffer_NAME.as_ptr())?),
      vkCmdBeginQuery: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdBeginQuery_NAME.as_ptr())?),
      vkCmdBeginRenderPass: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdBeginRenderPass_NAME.as_ptr())?),
      vkCmdBindDescriptorSets: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdBindDescriptorSets_NAME.as_ptr())?),
      vkCmdBindIndexBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdBindIndexBuffer_NAME.as_ptr())?),
      vkCmdBindPipeline: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdBindPipeline_NAME.as_ptr())?),
      vkCmdBindVertexBuffers: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdBindVertexBuffers_NAME.as_ptr())?),
      vkCmdBlitImage: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdBlitImage_NAME.as_ptr())?),
      vkCmdClearAttachments: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdClearAttachments_NAME.as_ptr())?),
      vkCmdClearColorImage: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdClearColorImage_NAME.as_ptr())?),
      vkCmdClearDepthStencilImage: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdClearDepthStencilImage_NAME.as_ptr())?),
      vkCmdCopyBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdCopyBuffer_NAME.as_ptr())?),
      vkCmdCopyBufferToImage: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdCopyBufferToImage_NAME.as_ptr())?),
      vkCmdCopyImage: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdCopyImage_NAME.as_ptr())?),
      vkCmdCopyImageToBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdCopyImageToBuffer_NAME.as_ptr())?),
      vkCmdCopyQueryPoolResults: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdCopyQueryPoolResults_NAME.as_ptr())?),
      vkCmdDispatch: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdDispatch_NAME.as_ptr())?),
      vkCmdDispatchIndirect: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdDispatchIndirect_NAME.as_ptr())?),
      vkCmdDraw: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdDraw_NAME.as_ptr())?),
      vkCmdDrawIndexed: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdDrawIndexed_NAME.as_ptr())?),
      vkCmdDrawIndexedIndirect: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdDrawIndexedIndirect_NAME.as_ptr())?),
      vkCmdDrawIndirect: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdDrawIndirect_NAME.as_ptr())?),
      vkCmdEndQuery: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdEndQuery_NAME.as_ptr())?),
      vkCmdEndRenderPass: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdEndRenderPass_NAME.as_ptr())?),
      vkCmdExecuteCommands: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdExecuteCommands_NAME.as_ptr())?),
      vkCmdFillBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdFillBuffer_NAME.as_ptr())?),
      vkCmdNextSubpass: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdNextSubpass_NAME.as_ptr())?),
      vkCmdPipelineBarrier: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdPipelineBarrier_NAME.as_ptr())?),
      vkCmdPushConstants: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdPushConstants_NAME.as_ptr())?),
      vkCmdResetEvent: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdResetEvent_NAME.as_ptr())?),
      vkCmdResetQueryPool: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdResetQueryPool_NAME.as_ptr())?),
      vkCmdResolveImage: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdResolveImage_NAME.as_ptr())?),
      vkCmdSetBlendConstants: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetBlendConstants_NAME.as_ptr())?),
      vkCmdSetDepthBias: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetDepthBias_NAME.as_ptr())?),
      vkCmdSetDepthBounds: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetDepthBounds_NAME.as_ptr())?),
      vkCmdSetEvent: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetEvent_NAME.as_ptr())?),
      vkCmdSetLineWidth: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetLineWidth_NAME.as_ptr())?),
      vkCmdSetScissor: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetScissor_NAME.as_ptr())?),
      vkCmdSetStencilCompareMask: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetStencilCompareMask_NAME.as_ptr())?),
      vkCmdSetStencilReference: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetStencilReference_NAME.as_ptr())?),
      vkCmdSetStencilWriteMask: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetStencilWriteMask_NAME.as_ptr())?),
      vkCmdSetViewport: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdSetViewport_NAME.as_ptr())?),
      vkCmdUpdateBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdUpdateBuffer_NAME.as_ptr())?),
      vkCmdWaitEvents: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdWaitEvents_NAME.as_ptr())?),
      vkCmdWriteTimestamp: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCmdWriteTimestamp_NAME.as_ptr())?),
      vkEndCommandBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkEndCommandBuffer_NAME.as_ptr())?),
      vkResetCommandBuffer: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkResetCommandBuffer_NAME.as_ptr())?),
      vkQueueBindSparse: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkQueueBindSparse_NAME.as_ptr())?),
      vkQueueSubmit: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkQueueSubmit_NAME.as_ptr())?),
      vkQueueWaitIdle: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkQueueWaitIdle_NAME.as_ptr())?),
    })
  }
}

#[cfg(feature = "VK_KHR_swapchain")]
#[allow(bad_style)]
#[derive(Clone, Copy)]
struct DeviceFnTable_VkKhrSwapchain {
  vkCreateSwapchainKHR: vkCreateSwapchainKHR_t,
  vkDestroySwapchainKHR: vkDestroySwapchainKHR_t,
  vkGetSwapchainImagesKHR: vkGetSwapchainImagesKHR_t,
  vkAcquireNextImageKHR: vkAcquireNextImageKHR_t,
  vkQueuePresentKHR: vkQueuePresentKHR_t,
}
#[cfg(feature = "VK_KHR_swapchain")]
#[allow(bad_style)]
impl DeviceFnTable_VkKhrSwapchain {
  #[rustfmt::skip]
  pub(crate) unsafe fn new(
    vkGetDeviceProcAddr: vkGetDeviceProcAddr_t, vk_device: VkDevice,
  ) -> Option<Self> {
    Some(Self{
      vkCreateSwapchainKHR: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateSwapchainKHR_NAME.as_ptr())?),
      vkDestroySwapchainKHR: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroySwapchainKHR_NAME.as_ptr())?),
      vkGetSwapchainImagesKHR: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetSwapchainImagesKHR_NAME.as_ptr())?),
      vkAcquireNextImageKHR: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkAcquireNextImageKHR_NAME.as_ptr())?),
      vkQueuePresentKHR: core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkQueuePresentKHR_NAME.as_ptr())?),
    })
  }
}
