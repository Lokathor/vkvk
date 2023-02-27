use crate::{api_constants::*, base_types::*, vk_version::*};

define_bitmask!(
  /// Khronos: [VkAccessFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html) (bitmask)
  VkAccessFlagBits
);
/// Khronos: [VkAccessFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html) (bitmask)
pub type VkAccessFlags = VkAccessFlagBits;

/// Khronos: [VkAllocationCallbacks](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAllocationCallbacks.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAllocationCallbacks {
  /// * Optional
  pub user_data: *mut c_void,
  /// * No Auto Validity
  pub allocation: PFN_vkAllocationFunction,
  /// * No Auto Validity
  pub reallocation: PFN_vkReallocationFunction,
  /// * No Auto Validity
  pub free: PFN_vkFreeFunction,
  /// * Optional
  /// * No Auto Validity
  pub internal_allocation: PFN_vkInternalAllocationNotification,
  /// * Optional
  /// * No Auto Validity
  pub internal_free: PFN_vkInternalFreeNotification,
}

/// Khronos: [VkApplicationInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkApplicationInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkApplicationInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_APPLICATION_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Len: `null-terminated`
  /// * Optional
  pub application_name: *const u8,
  pub application_version: u32,
  /// * Len: `null-terminated`
  /// * Optional
  pub engine_name: *const u8,
  pub engine_version: u32,
  pub api_version: u32,
}

/// Khronos: [VkAttachmentDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentDescription {
  /// * Optional
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

define_bitmask!(
  /// Khronos: [VkAttachmentDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionFlagBits.html) (bitmask)
  VkAttachmentDescriptionFlagBits
);
/// Khronos: [VkAttachmentDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionFlagBits.html) (bitmask)
pub type VkAttachmentDescriptionFlags = VkAttachmentDescriptionFlagBits;

define_enumeration!(
  /// Khronos: [VkAttachmentLoadOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentLoadOp.html) (enumeration)
  VkAttachmentLoadOp
);

/// Khronos: [VkAttachmentReference](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAttachmentReference {
  pub attachment: u32,
  pub layout: VkImageLayout,
}

define_enumeration!(
  /// Khronos: [VkAttachmentStoreOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentStoreOp.html) (enumeration)
  VkAttachmentStoreOp
);

/// Khronos: [VkBaseInStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseInStructure.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseInStructure {
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const VkBaseInStructure,
}

/// Khronos: [VkBaseOutStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseOutStructure.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseOutStructure {
  pub ty: VkStructureType,
  /// * Optional
  pub next: *mut VkBaseOutStructure,
}

/// Khronos: [VkBindSparseInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindSparseInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindSparseInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_BIND_SPARSE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub wait_semaphore_count: u32,
  /// * Len: `waitSemaphoreCount`
  pub wait_semaphores: *const VkSemaphore,
  /// * Optional
  pub buffer_bind_count: u32,
  /// * Len: `bufferBindCount`
  pub buffer_binds: *const VkSparseBufferMemoryBindInfo,
  /// * Optional
  pub image_opaque_bind_count: u32,
  /// * Len: `imageOpaqueBindCount`
  pub image_opaque_binds: *const VkSparseImageOpaqueMemoryBindInfo,
  /// * Optional
  pub image_bind_count: u32,
  /// * Len: `imageBindCount`
  pub image_binds: *const VkSparseImageMemoryBindInfo,
  /// * Optional
  pub signal_semaphore_count: u32,
  /// * Len: `signalSemaphoreCount`
  pub signal_semaphores: *const VkSemaphore,
}

define_enumeration!(
  /// Khronos: [VkBlendFactor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlendFactor.html) (enumeration)
  VkBlendFactor
);

define_enumeration!(
  /// Khronos: [VkBlendOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlendOp.html) (enumeration)
  VkBlendOp
);

define_enumeration!(
  /// Khronos: [VkBorderColor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBorderColor.html) (enumeration)
  VkBorderColor
);

define_non_dispatchable_handle!(
  /// Khronos: [VkBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuffer.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_BUFFER`]
  VkBuffer
);

/// Khronos: [VkBufferCopy](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCopy.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCopy {
  /// Specified in bytes
  pub src_offset: VkDeviceSize,
  /// Specified in bytes
  pub dst_offset: VkDeviceSize,
  /// Specified in bytes
  /// * No Auto Validity
  pub size: VkDeviceSize,
}

define_bitmask!(
  /// Khronos: [VkBufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html) (bitmask)
  VkBufferCreateFlagBits
);
/// Khronos: [VkBufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html) (bitmask)
pub type VkBufferCreateFlags = VkBufferCreateFlagBits;

/// Khronos: [VkBufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Buffer creation flags
  /// * Optional
  pub flags: VkBufferCreateFlags,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Buffer usage flags
  pub usage: VkBufferUsageFlags,
  pub sharing_mode: VkSharingMode,
  /// * Optional
  pub queue_family_index_count: u32,
  /// * Len: `queueFamilyIndexCount`
  /// * No Auto Validity
  pub queue_family_indices: *const u32,
}

/// Khronos: [VkBufferImageCopy](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy.html)
#[derive(Clone, Copy)]
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

/// Khronos: [VkBufferMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferMemoryBarrier {
  /// * Values: [`VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * No Auto Validity
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * No Auto Validity
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

define_bitmask!(
  /// Khronos: [VkBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html) (bitmask)
  VkBufferUsageFlagBits
);
/// Khronos: [VkBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html) (bitmask)
pub type VkBufferUsageFlags = VkBufferUsageFlagBits;

define_non_dispatchable_handle!(
  /// Khronos: [VkBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferView.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_BUFFER_VIEW`]
  VkBufferView
);

define_bitmask!(
  /// Khronos: [VkBufferViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateFlagBits.html) (bitmask)
  VkBufferViewCreateFlagBits
);
/// Khronos: [VkBufferViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateFlagBits.html) (bitmask)
pub type VkBufferViewCreateFlags = VkBufferViewCreateFlagBits;

/// Khronos: [VkBufferViewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferViewCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkBufferViewCreateFlags,
  pub buffer: VkBuffer,
  /// Optionally specifies format of elements
  pub format: VkFormat,
  /// Specified in bytes
  pub offset: VkDeviceSize,
  /// View size specified in bytes
  pub range: VkDeviceSize,
}

/// Khronos: [VkClearAttachment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearAttachment.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearAttachment {
  pub aspect_mask: VkImageAspectFlags,
  pub color_attachment: u32,
  /// * No Auto Validity
  pub clear_value: VkClearValue,
}

/// Khronos: [VkClearColorValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearColorValue.html)
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
  pub base_array_layer: u32,
  pub layer_count: u32,
}

/// Khronos: [VkClearValue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearValue.html)
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

define_bitmask!(
  /// Khronos: [VkColorComponentFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html) (bitmask)
  VkColorComponentFlagBits
);
/// Khronos: [VkColorComponentFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html) (bitmask)
pub type VkColorComponentFlags = VkColorComponentFlagBits;

define_handle!(
  /// Khronos: [VkCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBuffer.html) (handle)
  /// * Parent: [VkCommandPool]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_COMMAND_BUFFER`]
  VkCommandBuffer
);

/// Khronos: [VkCommandBufferAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferAllocateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  pub command_pool: VkCommandPool,
  pub level: VkCommandBufferLevel,
  pub command_buffer_count: u32,
}

/// Khronos: [VkCommandBufferBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferBeginInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferBeginInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Command buffer usage flags
  /// * Optional
  pub flags: VkCommandBufferUsageFlags,
  /// Pointer to inheritance info for secondary command buffers
  /// * Optional
  /// * No Auto Validity
  pub inheritance_info: *const VkCommandBufferInheritanceInfo,
}

/// Khronos: [VkCommandBufferInheritanceInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Render pass for secondary command buffers
  /// * Optional
  /// * No Auto Validity
  pub render_pass: VkRenderPass,
  pub subpass: u32,
  /// Framebuffer for secondary command buffers
  /// * Optional
  /// * No Auto Validity
  pub framebuffer: VkFramebuffer,
  /// Whether this secondary command buffer may be executed during an occlusion
  /// query
  pub occlusion_query_enable: VkBool32,
  /// Query flags used by this secondary command buffer, if executed during an
  /// occlusion query
  /// * Optional
  /// * No Auto Validity
  pub query_flags: VkQueryControlFlags,
  /// Pipeline statistics that may be counted for this secondary command buffer
  /// * Optional
  /// * No Auto Validity
  pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}

define_enumeration!(
  /// Khronos: [VkCommandBufferLevel](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferLevel.html) (enumeration)
  VkCommandBufferLevel
);

define_bitmask!(
  /// Khronos: [VkCommandBufferResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html) (bitmask)
  VkCommandBufferResetFlagBits
);
/// Khronos: [VkCommandBufferResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html) (bitmask)
pub type VkCommandBufferResetFlags = VkCommandBufferResetFlagBits;

define_bitmask!(
  /// Khronos: [VkCommandBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html) (bitmask)
  VkCommandBufferUsageFlagBits
);
/// Khronos: [VkCommandBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html) (bitmask)
pub type VkCommandBufferUsageFlags = VkCommandBufferUsageFlagBits;

define_non_dispatchable_handle!(
  /// Khronos: [VkCommandPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPool.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_COMMAND_POOL`]
  VkCommandPool
);

define_bitmask!(
  /// Khronos: [VkCommandPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html) (bitmask)
  VkCommandPoolCreateFlagBits
);
/// Khronos: [VkCommandPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html) (bitmask)
pub type VkCommandPoolCreateFlags = VkCommandPoolCreateFlagBits;

/// Khronos: [VkCommandPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCommandPoolCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Command pool creation flags
  /// * Optional
  pub flags: VkCommandPoolCreateFlags,
  pub queue_family_index: u32,
}

define_bitmask!(
  /// Khronos: [VkCommandPoolResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html) (bitmask)
  VkCommandPoolResetFlagBits
);
/// Khronos: [VkCommandPoolResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html) (bitmask)
pub type VkCommandPoolResetFlags = VkCommandPoolResetFlagBits;

define_enumeration!(
  /// Khronos: [VkCompareOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompareOp.html) (enumeration)
  VkCompareOp
);

/// Khronos: [VkComponentMapping](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentMapping.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkComponentMapping {
  pub r: VkComponentSwizzle,
  pub g: VkComponentSwizzle,
  pub b: VkComponentSwizzle,
  pub a: VkComponentSwizzle,
}

define_enumeration!(
  /// Khronos: [VkComponentSwizzle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentSwizzle.html) (enumeration)
  VkComponentSwizzle
);

/// Khronos: [VkComputePipelineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComputePipelineCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkComputePipelineCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Pipeline creation flags
  /// * Optional
  pub flags: VkPipelineCreateFlags,
  pub stage: VkPipelineShaderStageCreateInfo,
  /// Interface layout of the pipeline
  pub layout: VkPipelineLayout,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * Optional
  /// * No Auto Validity
  pub base_pipeline_handle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  pub base_pipeline_index: i32,
}

/// Khronos: [VkCopyDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyDescriptorSet.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyDescriptorSet {
  /// * Values: [`VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET`]
  pub ty: VkStructureType,
  /// * Optional
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

define_bitmask!(
  /// Khronos: [VkCullModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html) (bitmask)
  VkCullModeFlagBits
);
/// Khronos: [VkCullModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html) (bitmask)
pub type VkCullModeFlags = VkCullModeFlagBits;

define_bitmask!(
  /// Khronos: [VkDependencyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html) (bitmask)
  VkDependencyFlagBits
);
/// Khronos: [VkDependencyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html) (bitmask)
pub type VkDependencyFlags = VkDependencyFlagBits;

/// Khronos: [VkDescriptorBufferInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBufferInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorBufferInfo {
  /// Buffer used for this descriptor slot.
  /// * Optional
  pub buffer: VkBuffer,
  /// Base offset from buffer start in bytes to update in the descriptor set.
  pub offset: VkDeviceSize,
  /// Size in bytes of the buffer resource for this descriptor update.
  pub range: VkDeviceSize,
}

/// Khronos: [VkDescriptorImageInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorImageInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorImageInfo {
  /// Sampler to write to the descriptor in case it is a SAMPLER or
  /// COMBINED_IMAGE_SAMPLER descriptor. Ignored otherwise.
  /// * No Auto Validity
  pub sampler: VkSampler,
  /// Image view to write to the descriptor in case it is a SAMPLED_IMAGE,
  /// STORAGE_IMAGE, COMBINED_IMAGE_SAMPLER, or INPUT_ATTACHMENT descriptor.
  /// Ignored otherwise.
  /// * No Auto Validity
  pub image_view: VkImageView,
  /// Layout the image is expected to be in when accessed using this descriptor
  /// (only used if imageView is not VK_NULL_HANDLE).
  /// * No Auto Validity
  pub image_layout: VkImageLayout,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkDescriptorPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPool.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_DESCRIPTOR_POOL`]
  VkDescriptorPool
);

define_bitmask!(
  /// Khronos: [VkDescriptorPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html) (bitmask)
  VkDescriptorPoolCreateFlagBits
);
/// Khronos: [VkDescriptorPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html) (bitmask)
pub type VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlagBits;

/// Khronos: [VkDescriptorPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkDescriptorPoolCreateFlags,
  pub max_sets: u32,
  /// * Optional
  pub pool_size_count: u32,
  /// * Len: `poolSizeCount`
  pub pool_sizes: *const VkDescriptorPoolSize,
}

define_bitmask!(
  /// Khronos: [VkDescriptorPoolResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolResetFlagBits.html) (bitmask)
  VkDescriptorPoolResetFlagBits
);
/// Khronos: [VkDescriptorPoolResetFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolResetFlagBits.html) (bitmask)
pub type VkDescriptorPoolResetFlags = VkDescriptorPoolResetFlagBits;

/// Khronos: [VkDescriptorPoolSize](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolSize.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorPoolSize {
  pub ty: VkDescriptorType,
  pub descriptor_count: u32,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSet.html) (non-dispatchable handle)
  /// * Parent: [VkDescriptorPool]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_DESCRIPTOR_SET`]
  VkDescriptorSet
);

/// Khronos: [VkDescriptorSetAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetAllocateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  pub descriptor_pool: VkDescriptorPool,
  pub descriptor_set_count: u32,
  /// * Len: `descriptorSetCount`
  pub set_layouts: *const VkDescriptorSetLayout,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayout.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT`]
  VkDescriptorSetLayout
);

/// Khronos: [VkDescriptorSetLayoutBinding](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBinding.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
  /// Binding number for this entry
  pub binding: u32,
  /// Type of the descriptors in this binding
  pub descriptor_type: VkDescriptorType,
  /// Number of descriptors in this binding
  /// * Optional
  pub descriptor_count: u32,
  /// Shader stages this binding is visible to
  /// * No Auto Validity
  pub stage_flags: VkShaderStageFlags,
  /// Immutable samplers (used if descriptor type is SAMPLER or
  /// COMBINED_IMAGE_SAMPLER, is either NULL or contains count number of
  /// elements)
  /// * Len: `descriptorCount`
  /// * Optional
  /// * No Auto Validity
  pub immutable_samplers: *const VkSampler,
}

define_bitmask!(
  /// Khronos: [VkDescriptorSetLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html) (bitmask)
  VkDescriptorSetLayoutCreateFlagBits
);
/// Khronos: [VkDescriptorSetLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html) (bitmask)
pub type VkDescriptorSetLayoutCreateFlags = VkDescriptorSetLayoutCreateFlagBits;

/// Khronos: [VkDescriptorSetLayoutCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkDescriptorSetLayoutCreateFlags,
  /// Number of bindings in the descriptor set layout
  /// * Optional
  pub binding_count: u32,
  /// Array of descriptor set layout bindings
  /// * Len: `bindingCount`
  pub bindings: *const VkDescriptorSetLayoutBinding,
}

define_enumeration!(
  /// Khronos: [VkDescriptorType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorType.html) (enumeration)
  VkDescriptorType
);

define_handle!(
  /// Khronos: [VkDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDevice.html) (handle)
  /// * Parent: [VkPhysicalDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_DEVICE`]
  VkDevice
);

define_bitmask!(
  /// Khronos: [VkDeviceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlagBits.html) (bitmask)
  VkDeviceCreateFlagBits
);
/// Khronos: [VkDeviceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlagBits.html) (bitmask)
pub type VkDeviceCreateFlags = VkDeviceCreateFlagBits;

/// Khronos: [VkDeviceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkDeviceCreateFlags,
  pub queue_create_info_count: u32,
  /// * Len: `queueCreateInfoCount`
  pub queue_create_infos: *const VkDeviceQueueCreateInfo,
  /// * Deprecated: ignored
  /// * Optional
  pub enabled_layer_count: u32,
  /// Ordered list of layer names to be enabled
  /// * Deprecated: ignored
  /// * Len: `enabledLayerCount,null-terminated`
  pub pp_enabled_layer_names: *const *const u8,
  /// * Optional
  pub enabled_extension_count: u32,
  /// * Len: `enabledExtensionCount,null-terminated`
  pub pp_enabled_extension_names: *const *const u8,
  /// * Optional
  pub enabled_features: *const VkPhysicalDeviceFeatures,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkDeviceMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemory.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_DEVICE_MEMORY`]
  VkDeviceMemory
);

define_bitmask!(
  /// Khronos: [VkDeviceQueueCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html) (bitmask)
  VkDeviceQueueCreateFlagBits
);
/// Khronos: [VkDeviceQueueCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html) (bitmask)
pub type VkDeviceQueueCreateFlags = VkDeviceQueueCreateFlagBits;

/// Khronos: [VkDeviceQueueCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkDeviceQueueCreateFlags,
  pub queue_family_index: u32,
  pub queue_count: u32,
  /// * Len: `queueCount`
  pub queue_priorities: *const c_float,
}

/// Khronos: [VkDispatchIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDispatchIndirectCommand.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDispatchIndirectCommand {
  /// * No Auto Validity
  pub x: u32,
  /// * No Auto Validity
  pub y: u32,
  /// * No Auto Validity
  pub z: u32,
}

/// Khronos: [VkDrawIndexedIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawIndexedIndirectCommand.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
  pub index_count: u32,
  pub instance_count: u32,
  pub first_index: u32,
  pub vertex_offset: i32,
  /// * No Auto Validity
  pub first_instance: u32,
}

/// Khronos: [VkDrawIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawIndirectCommand.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawIndirectCommand {
  pub vertex_count: u32,
  pub instance_count: u32,
  pub first_vertex: u32,
  /// * No Auto Validity
  pub first_instance: u32,
}

define_enumeration!(
  /// Khronos: [VkDynamicState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDynamicState.html) (enumeration)
  VkDynamicState
);

define_non_dispatchable_handle!(
  /// Khronos: [VkEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEvent.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_EVENT`]
  VkEvent
);

define_bitmask!(
  /// Khronos: [VkEventCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlagBits.html) (bitmask)
  VkEventCreateFlagBits
);
/// Khronos: [VkEventCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlagBits.html) (bitmask)
pub type VkEventCreateFlags = VkEventCreateFlagBits;

/// Khronos: [VkEventCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkEventCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_EVENT_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Event creation flags
  /// * Optional
  pub flags: VkEventCreateFlags,
}

/// Khronos: [VkExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtensionProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtensionProperties {
  /// extension name
  pub extension_name: *const [u8; VK_MAX_EXTENSION_NAME_SIZE],
  /// version of the extension specification implemented
  pub spec_version: u32,
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

define_non_dispatchable_handle!(
  /// Khronos: [VkFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFence.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_FENCE`]
  VkFence
);

define_bitmask!(
  /// Khronos: [VkFenceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html) (bitmask)
  VkFenceCreateFlagBits
);
/// Khronos: [VkFenceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html) (bitmask)
pub type VkFenceCreateFlags = VkFenceCreateFlagBits;

/// Khronos: [VkFenceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFenceCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Fence creation flags
  /// * Optional
  pub flags: VkFenceCreateFlags,
}

define_enumeration!(
  /// Khronos: [VkFilter](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFilter.html) (enumeration)
  VkFilter
);

define_enumeration!(
  /// Khronos: [VkFormat](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormat.html) (enumeration)
  VkFormat
);

define_bitmask!(
  /// Khronos: [VkFormatFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html) (bitmask)
  VkFormatFeatureFlagBits
);
/// Khronos: [VkFormatFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html) (bitmask)
pub type VkFormatFeatureFlags = VkFormatFeatureFlagBits;

/// Khronos: [VkFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFormatProperties {
  /// Format features in case of linear tiling
  /// * Limit Type: bitmask
  /// * Optional
  pub linear_tiling_features: VkFormatFeatureFlags,
  /// Format features in case of optimal tiling
  /// * Limit Type: bitmask
  /// * Optional
  pub optimal_tiling_features: VkFormatFeatureFlags,
  /// Format features supported by buffers
  /// * Limit Type: bitmask
  /// * Optional
  pub buffer_features: VkFormatFeatureFlags,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkFramebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebuffer.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_FRAMEBUFFER`]
  VkFramebuffer
);

define_bitmask!(
  /// Khronos: [VkFramebufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlagBits.html) (bitmask)
  VkFramebufferCreateFlagBits
);
/// Khronos: [VkFramebufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlagBits.html) (bitmask)
pub type VkFramebufferCreateFlags = VkFramebufferCreateFlagBits;

/// Khronos: [VkFramebufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFramebufferCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkFramebufferCreateFlags,
  pub render_pass: VkRenderPass,
  /// * Optional
  pub attachment_count: u32,
  /// * Len: `attachmentCount`
  /// * No Auto Validity
  pub attachments: *const VkImageView,
  pub width: u32,
  pub height: u32,
  pub layers: u32,
}

define_enumeration!(
  /// Khronos: [VkFrontFace](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFrontFace.html) (enumeration)
  VkFrontFace
);

/// Khronos: [VkGraphicsPipelineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Pipeline creation flags
  /// * Optional
  pub flags: VkPipelineCreateFlags,
  /// * Optional
  /// * No Auto Validity
  pub stage_count: u32,
  /// One entry for each active shader stage
  /// * Len: `stageCount`
  /// * Optional
  /// * No Auto Validity
  pub stages: *const VkPipelineShaderStageCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub vertex_input_state: *const VkPipelineVertexInputStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub input_assembly_state: *const VkPipelineInputAssemblyStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub tessellation_state: *const VkPipelineTessellationStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub viewport_state: *const VkPipelineViewportStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub rasterization_state: *const VkPipelineRasterizationStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub multisample_state: *const VkPipelineMultisampleStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub depth_stencil_state: *const VkPipelineDepthStencilStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub color_blend_state: *const VkPipelineColorBlendStateCreateInfo,
  /// * Optional
  pub dynamic_state: *const VkPipelineDynamicStateCreateInfo,
  /// Interface layout of the pipeline
  /// * Optional
  /// * No Auto Validity
  pub layout: VkPipelineLayout,
  /// * Optional
  /// * No Auto Validity
  pub render_pass: VkRenderPass,
  /// * No Auto Validity
  pub subpass: u32,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * Optional
  /// * No Auto Validity
  pub base_pipeline_handle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  pub base_pipeline_index: i32,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImage.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_IMAGE`]
  VkImage
);

define_bitmask!(
  /// Khronos: [VkImageAspectFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html) (bitmask)
  VkImageAspectFlagBits
);
/// Khronos: [VkImageAspectFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html) (bitmask)
pub type VkImageAspectFlags = VkImageAspectFlagBits;

/// Khronos: [VkImageBlit](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageBlit.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageBlit {
  pub src_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub src_offsets: [VkOffset3D; 2],
  pub dst_subresource: VkImageSubresourceLayers,
  /// Specified in pixels for both compressed and uncompressed images
  pub dst_offsets: [VkOffset3D; 2],
}

/// Khronos: [VkImageCopy](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCopy.html)
#[derive(Clone, Copy)]
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

define_bitmask!(
  /// Khronos: [VkImageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html) (bitmask)
  VkImageCreateFlagBits
);
/// Khronos: [VkImageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html) (bitmask)
pub type VkImageCreateFlags = VkImageCreateFlagBits;

/// Khronos: [VkImageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Image creation flags
  /// * Optional
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
  /// * Optional
  pub queue_family_index_count: u32,
  /// Array of queue family indices to share across
  /// * Len: `queueFamilyIndexCount`
  /// * No Auto Validity
  pub queue_family_indices: *const u32,
  /// Initial image layout for all subresources
  pub initial_layout: VkImageLayout,
}

/// Khronos: [VkImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatProperties {
  /// max image dimensions for this resource type
  pub max_extent: VkExtent3D,
  /// max number of mipmap levels for this resource type
  pub max_mip_levels: u32,
  /// max array size for this resource type
  pub max_array_layers: u32,
  /// supported sample counts for this resource type
  /// * Optional
  pub sample_counts: VkSampleCountFlags,
  /// max size (in bytes) of this resource type
  pub max_resource_size: VkDeviceSize,
}

define_enumeration!(
  /// Khronos: [VkImageLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageLayout.html) (enumeration)
  VkImageLayout
);

/// Khronos: [VkImageMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageMemoryBarrier {
  /// * Values: [`VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * No Auto Validity
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * No Auto Validity
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

/// Khronos: [VkImageResolve](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageResolve.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageResolve {
  pub src_subresource: VkImageSubresourceLayers,
  pub src_offset: VkOffset3D,
  pub dst_subresource: VkImageSubresourceLayers,
  pub dst_offset: VkOffset3D,
  pub extent: VkExtent3D,
}

/// Khronos: [VkImageSubresource](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresource.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresource {
  pub aspect_mask: VkImageAspectFlags,
  pub mip_level: u32,
  pub array_layer: u32,
}

/// Khronos: [VkImageSubresourceLayers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceLayers.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresourceLayers {
  pub aspect_mask: VkImageAspectFlags,
  pub mip_level: u32,
  pub base_array_layer: u32,
  pub layer_count: u32,
}

/// Khronos: [VkImageSubresourceRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceRange.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresourceRange {
  pub aspect_mask: VkImageAspectFlags,
  pub base_mip_level: u32,
  pub level_count: u32,
  pub base_array_layer: u32,
  pub layer_count: u32,
}

define_enumeration!(
  /// Khronos: [VkImageTiling](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageTiling.html) (enumeration)
  VkImageTiling
);

define_enumeration!(
  /// Khronos: [VkImageType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageType.html) (enumeration)
  VkImageType
);

define_bitmask!(
  /// Khronos: [VkImageUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html) (bitmask)
  VkImageUsageFlagBits
);
/// Khronos: [VkImageUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html) (bitmask)
pub type VkImageUsageFlags = VkImageUsageFlagBits;

define_non_dispatchable_handle!(
  /// Khronos: [VkImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageView.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_IMAGE_VIEW`]
  VkImageView
);

define_bitmask!(
  /// Khronos: [VkImageViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html) (bitmask)
  VkImageViewCreateFlagBits
);
/// Khronos: [VkImageViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html) (bitmask)
pub type VkImageViewCreateFlags = VkImageViewCreateFlagBits;

/// Khronos: [VkImageViewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkImageViewCreateFlags,
  pub image: VkImage,
  pub view_type: VkImageViewType,
  pub format: VkFormat,
  pub components: VkComponentMapping,
  pub subresource_range: VkImageSubresourceRange,
}

define_enumeration!(
  /// Khronos: [VkImageViewType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewType.html) (enumeration)
  VkImageViewType
);

define_enumeration!(
  /// Khronos: [VkIndexType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkIndexType.html) (enumeration)
  VkIndexType
);

define_handle!(
  /// Khronos: [VkInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstance.html) (handle)
  /// * Object Type Enum: [`VK_OBJECT_TYPE_INSTANCE`]
  VkInstance
);

define_bitmask!(
  /// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html) (bitmask)
  VkInstanceCreateFlagBits
);
/// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html) (bitmask)
pub type VkInstanceCreateFlags = VkInstanceCreateFlagBits;

/// Khronos: [VkInstanceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkInstanceCreateFlags,
  /// * Optional
  pub application_info: *const VkApplicationInfo,
  /// * Optional
  pub enabled_layer_count: u32,
  /// Ordered list of layer names to be enabled
  /// * Len: `enabledLayerCount,null-terminated`
  pub pp_enabled_layer_names: *const *const u8,
  /// * Optional
  pub enabled_extension_count: u32,
  /// Extension names to be enabled
  /// * Len: `enabledExtensionCount,null-terminated`
  pub pp_enabled_extension_names: *const *const u8,
}

define_enumeration!(
  /// Khronos: [VkInternalAllocationType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInternalAllocationType.html) (enumeration)
  VkInternalAllocationType
);

/// Khronos: [VkLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLayerProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkLayerProperties {
  /// layer name
  pub layer_name: *const [u8; VK_MAX_EXTENSION_NAME_SIZE],
  /// version of the layer specification implemented
  pub spec_version: u32,
  /// build or release version of the layer's library
  pub implementation_version: u32,
  /// Free-form description of the layer
  pub description: *const [u8; VK_MAX_DESCRIPTION_SIZE],
}

define_enumeration!(
  /// Khronos: [VkLogicOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLogicOp.html) (enumeration)
  VkLogicOp
);

/// Khronos: [VkMappedMemoryRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMappedMemoryRange.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMappedMemoryRange {
  /// * Values: [`VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Mapped memory object
  pub memory: VkDeviceMemory,
  /// Offset within the memory object where the range starts
  pub offset: VkDeviceSize,
  /// Size of the range within the memory object
  pub size: VkDeviceSize,
}

/// Khronos: [VkMemoryAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryAllocateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Size of memory allocation
  pub allocation_size: VkDeviceSize,
  /// Index of the of the memory type to allocate from
  pub memory_type_index: u32,
}

/// Khronos: [VkMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryBarrier {
  /// * Values: [`VK_STRUCTURE_TYPE_MEMORY_BARRIER`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * Optional
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * Optional
  pub dst_access_mask: VkAccessFlags,
}

/// Khronos: [VkMemoryHeap](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeap.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryHeap {
  /// Available memory in the heap
  pub size: VkDeviceSize,
  /// Flags for the heap
  /// * Optional
  pub flags: VkMemoryHeapFlags,
}

define_bitmask!(
  /// Khronos: [VkMemoryHeapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html) (bitmask)
  VkMemoryHeapFlagBits
);
/// Khronos: [VkMemoryHeapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html) (bitmask)
pub type VkMemoryHeapFlags = VkMemoryHeapFlagBits;

define_bitmask!(
  /// Khronos: [VkMemoryMapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlagBits.html) (bitmask)
  VkMemoryMapFlagBits
);
/// Khronos: [VkMemoryMapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlagBits.html) (bitmask)
pub type VkMemoryMapFlags = VkMemoryMapFlagBits;

define_bitmask!(
  /// Khronos: [VkMemoryPropertyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html) (bitmask)
  VkMemoryPropertyFlagBits
);
/// Khronos: [VkMemoryPropertyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html) (bitmask)
pub type VkMemoryPropertyFlags = VkMemoryPropertyFlagBits;

/// Khronos: [VkMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements.html)
/// * Returned Only
#[derive(Clone, Copy)]
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

/// Khronos: [VkMemoryType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryType.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryType {
  /// Memory properties of this memory type
  /// * Optional
  pub property_flags: VkMemoryPropertyFlags,
  /// Index of the memory heap allocations of this memory type are taken from
  pub heap_index: u32,
}

define_enumeration!(
  /// Khronos: [VkObjectType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkObjectType.html) (enumeration)
  VkObjectType
);

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

define_handle!(
  /// Khronos: [VkPhysicalDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice.html) (handle)
  /// * Parent: [VkInstance]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_PHYSICAL_DEVICE`]
  VkPhysicalDevice
);

/// Khronos: [VkPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html)
#[derive(Clone, Copy)]
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
  pub sparse_residency_image_2d: VkBool32,
  /// Sparse resources support: GPU can access partially resident 3D images
  pub sparse_residency_image_3d: VkBool32,
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

/// Khronos: [VkPhysicalDeviceLimits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLimits.html)
///
/// compute stage limits
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
  /// max 1D image dimension
  /// * Limit Type: max
  pub max_image_dimension_1_d: u32,
  /// max 2D image dimension
  /// * Limit Type: max
  pub max_image_dimension_2d: u32,
  /// max 3D image dimension
  /// * Limit Type: max
  pub max_image_dimension_3d: u32,
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
  pub min_memory_map_alignment: usize,
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
  /// * Limit Type: bitmask
  /// * Optional
  pub framebuffer_color_sample_counts: VkSampleCountFlags,
  /// supported depth sample counts for a framebuffer
  /// * Limit Type: bitmask
  /// * Optional
  pub framebuffer_depth_sample_counts: VkSampleCountFlags,
  /// supported stencil sample counts for a framebuffer
  /// * Limit Type: bitmask
  /// * Optional
  pub framebuffer_stencil_sample_counts: VkSampleCountFlags,
  /// supported sample counts for a subpass which uses no attachments
  /// * Limit Type: bitmask
  /// * Optional
  pub framebuffer_no_attachments_sample_counts: VkSampleCountFlags,
  /// max number of color attachments per subpass
  /// * Limit Type: max
  pub max_color_attachments: u32,
  /// supported color sample counts for a non-integer sampled image
  /// * Limit Type: bitmask
  /// * Optional
  pub sampled_image_color_sample_counts: VkSampleCountFlags,
  /// supported sample counts for an integer image
  /// * Limit Type: bitmask
  /// * Optional
  pub sampled_image_integer_sample_counts: VkSampleCountFlags,
  /// supported depth sample counts for a sampled image
  /// * Limit Type: bitmask
  /// * Optional
  pub sampled_image_depth_sample_counts: VkSampleCountFlags,
  /// supported stencil sample counts for a sampled image
  /// * Limit Type: bitmask
  /// * Optional
  pub sampled_image_stencil_sample_counts: VkSampleCountFlags,
  /// supported sample counts for a storage image
  /// * Limit Type: bitmask
  /// * Optional
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

/// Khronos: [VkPhysicalDeviceMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
  pub memory_type_count: u32,
  pub memory_types: *const [VkMemoryType; VK_MAX_MEMORY_TYPES],
  pub memory_heap_count: u32,
  pub memory_heaps: *const [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

/// Khronos: [VkPhysicalDeviceProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
  /// * Limit Type: noauto
  pub api_version: VkVersion,
  /// * Limit Type: noauto
  pub driver_version: u32,
  /// * Limit Type: noauto
  pub vendor_id: u32,
  /// * Limit Type: noauto
  pub device_id: u32,
  /// * Limit Type: noauto
  pub device_type: VkPhysicalDeviceType,
  /// * Limit Type: noauto
  pub device_name: *const [u8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
  /// * Limit Type: noauto
  pub pipeline_cache_uuid: *const [u8; VK_UUID_SIZE],
  /// * Limit Type: struct
  pub limits: VkPhysicalDeviceLimits,
  /// * Limit Type: struct
  pub sparse_properties: VkPhysicalDeviceSparseProperties,
}

/// Khronos: [VkPhysicalDeviceSparseProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
  /// Sparse resources support: GPU will access all 2D (single sample) sparse
  /// resources using the standard sparse image block shapes (based on pixel
  /// format)
  /// * Limit Type: bitmask
  pub residency_standard_2d_block_shape: VkBool32,
  /// Sparse resources support: GPU will access all 2D (multisample) sparse
  /// resources using the standard sparse image block shapes (based on pixel
  /// format)
  /// * Limit Type: bitmask
  pub residency_standard_2d_multisample_block_shape: VkBool32,
  /// Sparse resources support: GPU will access all 3D sparse resources using
  /// the standard sparse image block shapes (based on pixel format)
  /// * Limit Type: bitmask
  pub residency_standard_3d_block_shape: VkBool32,
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

define_enumeration!(
  /// Khronos: [VkPhysicalDeviceType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceType.html) (enumeration)
  VkPhysicalDeviceType
);

define_non_dispatchable_handle!(
  /// Khronos: [VkPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipeline.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_PIPELINE`]
  VkPipeline
);

define_enumeration!(
  /// Khronos: [VkPipelineBindPoint](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineBindPoint.html) (enumeration)
  VkPipelineBindPoint
);

define_non_dispatchable_handle!(
  /// Khronos: [VkPipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCache.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_PIPELINE_CACHE`]
  VkPipelineCache
);

define_bitmask!(
  /// Khronos: [VkPipelineCacheCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html) (bitmask)
  VkPipelineCacheCreateFlagBits
);
/// Khronos: [VkPipelineCacheCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html) (bitmask)
pub type VkPipelineCacheCreateFlags = VkPipelineCacheCreateFlagBits;

/// Khronos: [VkPipelineCacheCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineCacheCreateFlags,
  /// Size of initial data to populate cache, in bytes
  /// * Optional
  pub initial_data_size: usize,
  /// Initial data to populate cache
  /// * Len: `initialDataSize`
  pub initial_data: *const c_void,
}

define_enumeration!(
  /// Khronos: [VkPipelineCacheHeaderVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersion.html) (enumeration)
  VkPipelineCacheHeaderVersion
);

/// Khronos: [VkPipelineCacheHeaderVersionOne](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersionOne.html)
///
/// The fields in this structure are non-normative since structure packing is
/// implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheHeaderVersionOne {
  pub header_size: u32,
  pub header_version: VkPipelineCacheHeaderVersion,
  pub vendor_id: u32,
  pub device_id: u32,
  pub pipeline_cache_uuid: *const [u8; VK_UUID_SIZE],
}

/// Khronos: [VkPipelineColorBlendAttachmentState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAttachmentState.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
  pub blend_enable: VkBool32,
  pub src_color_blend_factor: VkBlendFactor,
  pub dst_color_blend_factor: VkBlendFactor,
  pub color_blend_op: VkBlendOp,
  pub src_alpha_blend_factor: VkBlendFactor,
  pub dst_alpha_blend_factor: VkBlendFactor,
  pub alpha_blend_op: VkBlendOp,
  /// * Optional
  pub color_write_mask: VkColorComponentFlags,
}

define_bitmask!(
  /// Khronos: [VkPipelineColorBlendStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html) (bitmask)
  VkPipelineColorBlendStateCreateFlagBits
);
/// Khronos: [VkPipelineColorBlendStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html) (bitmask)
pub type VkPipelineColorBlendStateCreateFlags = VkPipelineColorBlendStateCreateFlagBits;

/// Khronos: [VkPipelineColorBlendStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineColorBlendStateCreateFlags,
  pub logic_op_enable: VkBool32,
  /// * No Auto Validity
  pub logic_op: VkLogicOp,
  /// # of pAttachments
  /// * Optional
  pub attachment_count: u32,
  /// * Len: `attachmentCount`
  /// * Optional
  pub attachments: *const VkPipelineColorBlendAttachmentState,
  pub blend_constants: [c_float; 4],
}

define_bitmask!(
  /// Khronos: [VkPipelineCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits.html) (bitmask)
  VkPipelineCreateFlagBits
);
/// Khronos: [VkPipelineCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits.html) (bitmask)
pub type VkPipelineCreateFlags = VkPipelineCreateFlagBits;

define_bitmask!(
  /// Khronos: [VkPipelineDepthStencilStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html) (bitmask)
  VkPipelineDepthStencilStateCreateFlagBits
);
/// Khronos: [VkPipelineDepthStencilStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html) (bitmask)
pub type VkPipelineDepthStencilStateCreateFlags = VkPipelineDepthStencilStateCreateFlagBits;

/// Khronos: [VkPipelineDepthStencilStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
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

define_bitmask!(
  /// Khronos: [VkPipelineDynamicStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateFlagBits.html) (bitmask)
  VkPipelineDynamicStateCreateFlagBits
);
/// Khronos: [VkPipelineDynamicStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateFlagBits.html) (bitmask)
pub type VkPipelineDynamicStateCreateFlags = VkPipelineDynamicStateCreateFlagBits;

/// Khronos: [VkPipelineDynamicStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineDynamicStateCreateFlags,
  /// * Optional
  pub dynamic_state_count: u32,
  /// * Len: `dynamicStateCount`
  pub dynamic_states: *const VkDynamicState,
}

define_bitmask!(
  /// Khronos: [VkPipelineInputAssemblyStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateFlagBits.html) (bitmask)
  VkPipelineInputAssemblyStateCreateFlagBits
);
/// Khronos: [VkPipelineInputAssemblyStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateFlagBits.html) (bitmask)
pub type VkPipelineInputAssemblyStateCreateFlags = VkPipelineInputAssemblyStateCreateFlagBits;

/// Khronos: [VkPipelineInputAssemblyStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineInputAssemblyStateCreateFlags,
  pub topology: VkPrimitiveTopology,
  pub primitive_restart_enable: VkBool32,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkPipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayout.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_PIPELINE_LAYOUT`]
  VkPipelineLayout
);

define_bitmask!(
  /// Khronos: [VkPipelineLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html) (bitmask)
  VkPipelineLayoutCreateFlagBits
);
/// Khronos: [VkPipelineLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html) (bitmask)
pub type VkPipelineLayoutCreateFlags = VkPipelineLayoutCreateFlagBits;

/// Khronos: [VkPipelineLayoutCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineLayoutCreateFlags,
  /// Number of descriptor sets interfaced by the pipeline
  /// * Optional
  pub set_layout_count: u32,
  /// Array of setCount number of descriptor set layout objects defining the
  /// layout of the
  /// * Len: `setLayoutCount`
  /// * Must be non-null, but may point at 0.
  pub set_layouts: *const VkDescriptorSetLayout,
  /// Number of push-constant ranges used by the pipeline
  /// * Optional
  pub push_constant_range_count: u32,
  /// Array of pushConstantRangeCount number of ranges used by various shader
  /// stages
  /// * Len: `pushConstantRangeCount`
  pub push_constant_ranges: *const VkPushConstantRange,
}

define_bitmask!(
  /// Khronos: [VkPipelineMultisampleStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateFlagBits.html) (bitmask)
  VkPipelineMultisampleStateCreateFlagBits
);
/// Khronos: [VkPipelineMultisampleStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateFlagBits.html) (bitmask)
pub type VkPipelineMultisampleStateCreateFlags = VkPipelineMultisampleStateCreateFlagBits;

/// Khronos: [VkPipelineMultisampleStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineMultisampleStateCreateFlags,
  /// Number of samples used for rasterization
  pub rasterization_samples: VkSampleCountFlagBits,
  /// optional (GL45)
  pub sample_shading_enable: VkBool32,
  /// optional (GL45)
  pub min_sample_shading: c_float,
  /// Array of sampleMask words
  /// * Len: `latexmath:[\lceil{\mathit{rasterizationSamples} \over 32}\rceil]`
  /// * Alt Len: `(rasterizationSamples + 31) / 32`
  /// * Optional
  pub sample_mask: *const VkSampleMask,
  pub alpha_to_coverage_enable: VkBool32,
  pub alpha_to_one_enable: VkBool32,
}

define_bitmask!(
  /// Khronos: [VkPipelineRasterizationStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateFlagBits.html) (bitmask)
  VkPipelineRasterizationStateCreateFlagBits
);
/// Khronos: [VkPipelineRasterizationStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateFlagBits.html) (bitmask)
pub type VkPipelineRasterizationStateCreateFlags = VkPipelineRasterizationStateCreateFlagBits;

/// Khronos: [VkPipelineRasterizationStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineRasterizationStateCreateFlags,
  pub depth_clamp_enable: VkBool32,
  pub rasterizer_discard_enable: VkBool32,
  /// optional (GL45)
  pub polygon_mode: VkPolygonMode,
  /// * Optional
  pub cull_mode: VkCullModeFlags,
  pub front_face: VkFrontFace,
  pub depth_bias_enable: VkBool32,
  pub depth_bias_constant_factor: c_float,
  pub depth_bias_clamp: c_float,
  pub depth_bias_slope_factor: c_float,
  pub line_width: c_float,
}

define_bitmask!(
  /// Khronos: [VkPipelineShaderStageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html) (bitmask)
  VkPipelineShaderStageCreateFlagBits
);
/// Khronos: [VkPipelineShaderStageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html) (bitmask)
pub type VkPipelineShaderStageCreateFlags = VkPipelineShaderStageCreateFlagBits;

/// Khronos: [VkPipelineShaderStageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineShaderStageCreateFlags,
  /// Shader stage
  pub stage: VkShaderStageFlagBits,
  /// Module containing entry point
  /// * Optional
  pub module: VkShaderModule,
  /// Null-terminated entry point name
  /// * Len: `null-terminated`
  pub name: *const u8,
  /// * Optional
  pub specialization_info: *const VkSpecializationInfo,
}

define_bitmask!(
  /// Khronos: [VkPipelineStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html) (bitmask)
  VkPipelineStageFlagBits
);
/// Khronos: [VkPipelineStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html) (bitmask)
pub type VkPipelineStageFlags = VkPipelineStageFlagBits;

define_bitmask!(
  /// Khronos: [VkPipelineTessellationStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateFlagBits.html) (bitmask)
  VkPipelineTessellationStateCreateFlagBits
);
/// Khronos: [VkPipelineTessellationStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateFlagBits.html) (bitmask)
pub type VkPipelineTessellationStateCreateFlags = VkPipelineTessellationStateCreateFlagBits;

/// Khronos: [VkPipelineTessellationStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineTessellationStateCreateFlags,
  pub patch_control_points: u32,
}

define_bitmask!(
  /// Khronos: [VkPipelineVertexInputStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateFlagBits.html) (bitmask)
  VkPipelineVertexInputStateCreateFlagBits
);
/// Khronos: [VkPipelineVertexInputStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateFlagBits.html) (bitmask)
pub type VkPipelineVertexInputStateCreateFlags = VkPipelineVertexInputStateCreateFlagBits;

/// Khronos: [VkPipelineVertexInputStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineVertexInputStateCreateFlags,
  /// number of bindings
  /// * Optional
  pub vertex_binding_description_count: u32,
  /// * Len: `vertexBindingDescriptionCount`
  pub vertex_binding_descriptions: *const VkVertexInputBindingDescription,
  /// number of attributes
  /// * Optional
  pub vertex_attribute_description_count: u32,
  /// * Len: `vertexAttributeDescriptionCount`
  pub vertex_attribute_descriptions: *const VkVertexInputAttributeDescription,
}

define_bitmask!(
  /// Khronos: [VkPipelineViewportStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateFlagBits.html) (bitmask)
  VkPipelineViewportStateCreateFlagBits
);
/// Khronos: [VkPipelineViewportStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateFlagBits.html) (bitmask)
pub type VkPipelineViewportStateCreateFlags = VkPipelineViewportStateCreateFlagBits;

/// Khronos: [VkPipelineViewportStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkPipelineViewportStateCreateFlags,
  /// * Optional
  pub viewport_count: u32,
  /// * Len: `viewportCount`
  /// * Optional
  /// * No Auto Validity
  pub viewports: *const VkViewport,
  /// * Optional
  pub scissor_count: u32,
  /// * Len: `scissorCount`
  /// * Optional
  /// * No Auto Validity
  pub scissors: *const VkRect2D,
}

define_enumeration!(
  /// Khronos: [VkPolygonMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPolygonMode.html) (enumeration)
  VkPolygonMode
);

define_enumeration!(
  /// Khronos: [VkPrimitiveTopology](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html) (enumeration)
  VkPrimitiveTopology
);

/// Khronos: [VkPushConstantRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPushConstantRange.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPushConstantRange {
  /// Which stages use the range
  pub stage_flags: VkShaderStageFlags,
  /// Start of the range, in bytes
  pub offset: u32,
  /// Size of the range, in bytes
  pub size: u32,
}

define_bitmask!(
  /// Khronos: [VkQueryControlFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html) (bitmask)
  VkQueryControlFlagBits
);
/// Khronos: [VkQueryControlFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html) (bitmask)
pub type VkQueryControlFlags = VkQueryControlFlagBits;

define_bitmask!(
  /// Khronos: [VkQueryPipelineStatisticFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html) (bitmask)
  VkQueryPipelineStatisticFlagBits
);
/// Khronos: [VkQueryPipelineStatisticFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html) (bitmask)
pub type VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlagBits;

define_non_dispatchable_handle!(
  /// Khronos: [VkQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPool.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_QUERY_POOL`]
  VkQueryPool
);

define_bitmask!(
  /// Khronos: [VkQueryPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlagBits.html) (bitmask)
  VkQueryPoolCreateFlagBits
);
/// Khronos: [VkQueryPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlagBits.html) (bitmask)
pub type VkQueryPoolCreateFlags = VkQueryPoolCreateFlagBits;

/// Khronos: [VkQueryPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueryPoolCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkQueryPoolCreateFlags,
  pub query_type: VkQueryType,
  pub query_count: u32,
  /// Optional
  /// * Optional
  /// * No Auto Validity
  pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}

define_bitmask!(
  /// Khronos: [VkQueryResultFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html) (bitmask)
  VkQueryResultFlagBits
);
/// Khronos: [VkQueryResultFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html) (bitmask)
pub type VkQueryResultFlags = VkQueryResultFlagBits;

define_enumeration!(
  /// Khronos: [VkQueryType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryType.html) (enumeration)
  VkQueryType
);

define_handle!(
  /// Khronos: [VkQueue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueue.html) (handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_QUEUE`]
  VkQueue
);

/// Khronos: [VkQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyProperties {
  /// Queue flags
  /// * Limit Type: bitmask
  /// * Optional
  pub queue_flags: VkQueueFlags,
  /// * Limit Type: max
  pub queue_count: u32,
  /// * Limit Type: bits
  pub timestamp_valid_bits: u32,
  /// Minimum alignment requirement for image transfers
  /// * Limit Type: min,mul
  pub min_image_transfer_granularity: VkExtent3D,
}

define_bitmask!(
  /// Khronos: [VkQueueFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html) (bitmask)
  VkQueueFlagBits
);
/// Khronos: [VkQueueFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html) (bitmask)
pub type VkQueueFlags = VkQueueFlagBits;

/// Khronos: [VkRect2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRect2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPass.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_RENDER_PASS`]
  VkRenderPass
);

/// Khronos: [VkRenderPassBeginInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassBeginInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassBeginInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  pub render_pass: VkRenderPass,
  pub framebuffer: VkFramebuffer,
  pub render_area: VkRect2D,
  /// * Optional
  pub clear_value_count: u32,
  /// * Len: `clearValueCount`
  /// * No Auto Validity
  pub clear_values: *const VkClearValue,
}

define_bitmask!(
  /// Khronos: [VkRenderPassCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateFlagBits.html) (bitmask)
  VkRenderPassCreateFlagBits
);
/// Khronos: [VkRenderPassCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateFlagBits.html) (bitmask)
pub type VkRenderPassCreateFlags = VkRenderPassCreateFlagBits;

/// Khronos: [VkRenderPassCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRenderPassCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub flags: VkRenderPassCreateFlags,
  /// * Optional
  pub attachment_count: u32,
  /// * Len: `attachmentCount`
  pub attachments: *const VkAttachmentDescription,
  pub subpass_count: u32,
  /// * Len: `subpassCount`
  pub subpasses: *const VkSubpassDescription,
  /// * Optional
  pub dependency_count: u32,
  /// * Len: `dependencyCount`
  pub dependencies: *const VkSubpassDependency,
}

define_bitmask!(
  /// Khronos: [VkSampleCountFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html) (bitmask)
  VkSampleCountFlagBits
);
/// Khronos: [VkSampleCountFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html) (bitmask)
pub type VkSampleCountFlags = VkSampleCountFlagBits;

define_non_dispatchable_handle!(
  /// Khronos: [VkSampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampler.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_SAMPLER`]
  VkSampler
);

define_enumeration!(
  /// Khronos: [VkSamplerAddressMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerAddressMode.html) (enumeration)
  VkSamplerAddressMode
);

define_bitmask!(
  /// Khronos: [VkSamplerCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html) (bitmask)
  VkSamplerCreateFlagBits
);
/// Khronos: [VkSamplerCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html) (bitmask)
pub type VkSamplerCreateFlags = VkSamplerCreateFlagBits;

/// Khronos: [VkSamplerCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
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
  /// * No Auto Validity
  pub compare_op: VkCompareOp,
  pub min_lod: c_float,
  pub max_lod: c_float,
  /// * No Auto Validity
  pub border_color: VkBorderColor,
  pub unnormalized_coordinates: VkBool32,
}

define_enumeration!(
  /// Khronos: [VkSamplerMipmapMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerMipmapMode.html) (enumeration)
  VkSamplerMipmapMode
);

define_non_dispatchable_handle!(
  /// Khronos: [VkSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphore.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_SEMAPHORE`]
  VkSemaphore
);

define_bitmask!(
  /// Khronos: [VkSemaphoreCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateFlagBits.html) (bitmask)
  VkSemaphoreCreateFlagBits
);
/// Khronos: [VkSemaphoreCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateFlagBits.html) (bitmask)
pub type VkSemaphoreCreateFlags = VkSemaphoreCreateFlagBits;

/// Khronos: [VkSemaphoreCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Semaphore creation flags
  /// * Optional
  pub flags: VkSemaphoreCreateFlags,
}

define_non_dispatchable_handle!(
  /// Khronos: [VkShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModule.html) (non-dispatchable handle)
  /// * Parent: [VkDevice]
  /// * Object Type Enum: [`VK_OBJECT_TYPE_SHADER_MODULE`]
  VkShaderModule
);

define_bitmask!(
  /// Khronos: [VkShaderModuleCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateFlagBits.html) (bitmask)
  VkShaderModuleCreateFlagBits
);
/// Khronos: [VkShaderModuleCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateFlagBits.html) (bitmask)
pub type VkShaderModuleCreateFlags = VkShaderModuleCreateFlagBits;

/// Khronos: [VkShaderModuleCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateInfo.html)
/// * Struct Extends: [VkPipelineShaderStageCreateInfo]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderModuleCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// noautovalidity because this structure can be either an explicit parameter,
  /// or passed in a pNext chain
  /// * Optional
  /// * No Auto Validity
  pub next: *const c_void,
  /// * Optional
  pub flags: VkShaderModuleCreateFlags,
  /// Specified in bytes
  pub code_size: usize,
  /// Binary code of size codeSize
  /// * Len: `latexmath:[\textrm{codeSize} \over 4]`
  /// * Alt Len: `codeSize / 4`
  pub code: *const u32,
}

define_bitmask!(
  /// Khronos: [VkShaderStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html) (bitmask)
  VkShaderStageFlagBits
);
/// Khronos: [VkShaderStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html) (bitmask)
pub type VkShaderStageFlags = VkShaderStageFlagBits;

define_enumeration!(
  /// Khronos: [VkSharingMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html) (enumeration)
  VkSharingMode
);

/// Khronos: [VkSparseBufferMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseBufferMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
  pub buffer: VkBuffer,
  pub bind_count: u32,
  /// * Len: `bindCount`
  pub binds: *const VkSparseMemoryBind,
}

define_bitmask!(
  /// Khronos: [VkSparseImageFormatFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html) (bitmask)
  VkSparseImageFormatFlagBits
);
/// Khronos: [VkSparseImageFormatFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html) (bitmask)
pub type VkSparseImageFormatFlags = VkSparseImageFormatFlagBits;

/// Khronos: [VkSparseImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageFormatProperties {
  /// * Limit Type: bitmask
  /// * Optional
  pub aspect_mask: VkImageAspectFlags,
  /// * Limit Type: min,mul
  pub image_granularity: VkExtent3D,
  /// * Limit Type: bitmask
  /// * Optional
  pub flags: VkSparseImageFormatFlags,
}

/// Khronos: [VkSparseImageMemoryBind](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBind.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryBind {
  pub subresource: VkImageSubresource,
  pub offset: VkOffset3D,
  pub extent: VkExtent3D,
  /// * Optional
  pub memory: VkDeviceMemory,
  /// Specified in bytes
  pub memory_offset: VkDeviceSize,
  /// * Optional
  pub flags: VkSparseMemoryBindFlags,
}

/// Khronos: [VkSparseImageMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
  pub image: VkImage,
  pub bind_count: u32,
  /// * Len: `bindCount`
  pub binds: *const VkSparseImageMemoryBind,
}

/// Khronos: [VkSparseImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements.html)
/// * Returned Only
#[derive(Clone, Copy)]
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

/// Khronos: [VkSparseImageOpaqueMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
  pub image: VkImage,
  pub bind_count: u32,
  /// * Len: `bindCount`
  pub binds: *const VkSparseMemoryBind,
}

/// Khronos: [VkSparseMemoryBind](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBind.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseMemoryBind {
  /// Specified in bytes
  pub resource_offset: VkDeviceSize,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// * Optional
  pub memory: VkDeviceMemory,
  /// Specified in bytes
  pub memory_offset: VkDeviceSize,
  /// * Optional
  pub flags: VkSparseMemoryBindFlags,
}

define_bitmask!(
  /// Khronos: [VkSparseMemoryBindFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html) (bitmask)
  VkSparseMemoryBindFlagBits
);
/// Khronos: [VkSparseMemoryBindFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html) (bitmask)
pub type VkSparseMemoryBindFlags = VkSparseMemoryBindFlagBits;

/// Khronos: [VkSpecializationInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSpecializationInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSpecializationInfo {
  /// Number of entries in the map
  /// * Optional
  pub map_entry_count: u32,
  /// Array of map entries
  /// * Len: `mapEntryCount`
  pub map_entries: *const VkSpecializationMapEntry,
  /// Size in bytes of pData
  /// * Optional
  pub data_size: usize,
  /// Pointer to SpecConstant data
  /// * Len: `dataSize`
  pub data: *const c_void,
}

/// Khronos: [VkSpecializationMapEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSpecializationMapEntry.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSpecializationMapEntry {
  /// The SpecConstant ID specified in the BIL
  pub constant_id: u32,
  /// Offset of the value in the data block
  pub offset: u32,
  /// Size in bytes of the SpecConstant
  /// * No Auto Validity
  pub size: usize,
}

define_bitmask!(
  /// Khronos: [VkStencilFaceFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilFaceFlagBits.html) (bitmask)
  VkStencilFaceFlagBits
);
/// Khronos: [VkStencilFaceFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilFaceFlagBits.html) (bitmask)
pub type VkStencilFaceFlags = VkStencilFaceFlagBits;

define_enumeration!(
  /// Khronos: [VkStencilOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilOp.html) (enumeration)
  VkStencilOp
);

/// Khronos: [VkStencilOpState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilOpState.html)
#[derive(Clone, Copy)]
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

define_enumeration!(
  /// Khronos: [VkStructureType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStructureType.html) (enumeration)
  VkStructureType
);

/// Khronos: [VkSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubmitInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_SUBMIT_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// * Optional
  pub wait_semaphore_count: u32,
  /// * Len: `waitSemaphoreCount`
  pub wait_semaphores: *const VkSemaphore,
  /// * Len: `waitSemaphoreCount`
  /// * Must be non-null, but may point at 0.
  pub wait_dst_stage_mask: *const VkPipelineStageFlags,
  /// * Optional
  pub command_buffer_count: u32,
  /// * Len: `commandBufferCount`
  pub command_buffers: *const VkCommandBuffer,
  /// * Optional
  pub signal_semaphore_count: u32,
  /// * Len: `signalSemaphoreCount`
  pub signal_semaphores: *const VkSemaphore,
}

define_enumeration!(
  /// Khronos: [VkSubpassContents](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassContents.html) (enumeration)
  VkSubpassContents
);

/// Khronos: [VkSubpassDependency](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDependency {
  pub src_subpass: u32,
  pub dst_subpass: u32,
  /// * Optional
  pub src_stage_mask: VkPipelineStageFlags,
  /// * Optional
  pub dst_stage_mask: VkPipelineStageFlags,
  /// Memory accesses from the source of the dependency to synchronize
  /// * Optional
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * Optional
  pub dst_access_mask: VkAccessFlags,
  /// * Optional
  pub dependency_flags: VkDependencyFlags,
}

/// Khronos: [VkSubpassDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDescription {
  /// * Optional
  pub flags: VkSubpassDescriptionFlags,
  /// Must be VK_PIPELINE_BIND_POINT_GRAPHICS for now
  pub pipeline_bind_point: VkPipelineBindPoint,
  /// * Optional
  pub input_attachment_count: u32,
  /// * Len: `inputAttachmentCount`
  pub input_attachments: *const VkAttachmentReference,
  /// * Optional
  pub color_attachment_count: u32,
  /// * Len: `colorAttachmentCount`
  pub color_attachments: *const VkAttachmentReference,
  /// * Len: `colorAttachmentCount`
  /// * Optional
  pub resolve_attachments: *const VkAttachmentReference,
  /// * Optional
  pub depth_stencil_attachment: *const VkAttachmentReference,
  /// * Optional
  pub preserve_attachment_count: u32,
  /// * Len: `preserveAttachmentCount`
  pub preserve_attachments: *const u32,
}

define_bitmask!(
  /// Khronos: [VkSubpassDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlagBits.html) (bitmask)
  VkSubpassDescriptionFlagBits
);
/// Khronos: [VkSubpassDescriptionFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlagBits.html) (bitmask)
pub type VkSubpassDescriptionFlags = VkSubpassDescriptionFlagBits;

/// Khronos: [VkSubresourceLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubresourceLayout.html)
#[derive(Clone, Copy)]
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

define_enumeration!(
  /// Khronos: [VkSystemAllocationScope](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSystemAllocationScope.html) (enumeration)
  VkSystemAllocationScope
);

define_enumeration!(
  /// Khronos: [VkVendorId](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVendorId.html) (enumeration)
  VkVendorId
);

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

/// Khronos: [VkVertexInputBindingDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputBindingDescription {
  /// Vertex buffer binding id
  pub binding: u32,
  /// Distance between vertices in bytes (0 = no advancement)
  pub stride: u32,
  /// The rate at which the vertex data is consumed
  pub input_rate: VkVertexInputRate,
}

define_enumeration!(
  /// Khronos: [VkVertexInputRate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputRate.html) (enumeration)
  VkVertexInputRate
);

/// Khronos: [VkViewport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewport.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViewport {
  /// * No Auto Validity
  pub x: c_float,
  /// * No Auto Validity
  pub y: c_float,
  /// * No Auto Validity
  pub width: c_float,
  /// * No Auto Validity
  pub height: c_float,
  pub min_depth: c_float,
  pub max_depth: c_float,
}

/// Khronos: [VkWriteDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSet.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkWriteDescriptorSet {
  /// * Values: [`VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const c_void,
  /// Destination descriptor set
  /// * No Auto Validity
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
  /// * Len: `descriptorCount`
  /// * No Auto Validity
  pub image_info: *const VkDescriptorImageInfo,
  /// Raw buffer, size, and offset for {UNIFORM,STORAGE}_BUFFER\[_DYNAMIC\]
  /// descriptor types.
  /// * Len: `descriptorCount`
  /// * No Auto Validity
  pub buffer_info: *const VkDescriptorBufferInfo,
  /// Buffer view to write to the descriptor for {UNIFORM,STORAGE}_TEXEL_BUFFER
  /// descriptor types.
  /// * Len: `descriptorCount`
  /// * No Auto Validity
  pub texel_buffer_view: *const VkBufferView,
}

#[allow(nonstandard_style)]
pub type vkVoidFunction_t = unsafe extern "system" fn();
#[allow(nonstandard_style)]
pub type vkAllocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: usize,
  alignment: usize,
  allocation_scope: VkSystemAllocationScope,
);
#[allow(nonstandard_style)]
pub type vkReallocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  original: *mut c_void,
  size: usize,
  alignment: usize,
  allocation_scope: VkSystemAllocationScope,
);
#[allow(nonstandard_style)]
pub type vkFreeFunction_t = unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
#[allow(nonstandard_style)]
pub type vkInternalAllocationNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: usize,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);
#[allow(nonstandard_style)]
pub type vkInternalFreeNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: usize,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);

#[allow(nonstandard_style)]
pub type PFN_vkVoidFunction = Option<vkVoidFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkAllocationFunction = Option<vkAllocationFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkReallocationFunction = Option<vkReallocationFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkFreeFunction = Option<vkFreeFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkInternalAllocationNotification = Option<vkInternalAllocationNotification_t>;
#[allow(nonstandard_style)]
pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification_t>;
