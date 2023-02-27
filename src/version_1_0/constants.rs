#![allow(nonstandard_style)]

use crate::prelude::*;

/// Controls coherency of color attachment reads
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 7);

/// Controls coherency of color attachment writes
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 8);

/// Controls coherency of depth/stencil attachment reads
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits =
  VkAccessFlagBits(1_u32 << 9);

/// Controls coherency of depth/stencil attachment writes
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits =
  VkAccessFlagBits(1_u32 << 10);

/// Controls coherency of host reads
pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 13);

/// Controls coherency of host writes
pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 14);

/// Controls coherency of index reads
pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 1);

/// Controls coherency of indirect command reads
pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 0);

/// Controls coherency of input attachment reads
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 4);

/// Controls coherency of memory reads
pub const VK_ACCESS_MEMORY_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 15);

/// Controls coherency of memory writes
pub const VK_ACCESS_MEMORY_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 16);

/// Controls coherency of shader reads
pub const VK_ACCESS_SHADER_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 5);

/// Controls coherency of shader writes
pub const VK_ACCESS_SHADER_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 6);

/// Controls coherency of transfer reads
pub const VK_ACCESS_TRANSFER_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 11);

/// Controls coherency of transfer writes
pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 12);

/// Controls coherency of uniform buffer reads
pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 3);

/// Controls coherency of vertex attribute reads
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1_u32 << 2);

/// The attachment may alias physical memory of another attachment in the same
/// render pass
pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlagBits =
  VkAttachmentDescriptionFlagBits(1_u32 << 0);

pub const VK_ATTACHMENT_LOAD_OP_CLEAR: VkAttachmentLoadOp = VkAttachmentLoadOp(1);

pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: VkAttachmentLoadOp = VkAttachmentLoadOp(2);

pub const VK_ATTACHMENT_LOAD_OP_LOAD: VkAttachmentLoadOp = VkAttachmentLoadOp(0);

pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: VkAttachmentStoreOp = VkAttachmentStoreOp(1);

pub const VK_ATTACHMENT_STORE_OP_STORE: VkAttachmentStoreOp = VkAttachmentStoreOp(0);

pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: VkBlendFactor = VkBlendFactor(12);

pub const VK_BLEND_FACTOR_CONSTANT_COLOR: VkBlendFactor = VkBlendFactor(10);

pub const VK_BLEND_FACTOR_DST_ALPHA: VkBlendFactor = VkBlendFactor(8);

pub const VK_BLEND_FACTOR_DST_COLOR: VkBlendFactor = VkBlendFactor(4);

pub const VK_BLEND_FACTOR_ONE: VkBlendFactor = VkBlendFactor(1);

pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: VkBlendFactor = VkBlendFactor(13);

pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: VkBlendFactor = VkBlendFactor(11);

pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: VkBlendFactor = VkBlendFactor(9);

pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: VkBlendFactor = VkBlendFactor(5);

pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: VkBlendFactor = VkBlendFactor(18);

pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: VkBlendFactor = VkBlendFactor(16);

pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: VkBlendFactor = VkBlendFactor(7);

pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: VkBlendFactor = VkBlendFactor(3);

pub const VK_BLEND_FACTOR_SRC1_ALPHA: VkBlendFactor = VkBlendFactor(17);

pub const VK_BLEND_FACTOR_SRC1_COLOR: VkBlendFactor = VkBlendFactor(15);

pub const VK_BLEND_FACTOR_SRC_ALPHA: VkBlendFactor = VkBlendFactor(6);

pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: VkBlendFactor = VkBlendFactor(14);

pub const VK_BLEND_FACTOR_SRC_COLOR: VkBlendFactor = VkBlendFactor(2);

pub const VK_BLEND_FACTOR_ZERO: VkBlendFactor = VkBlendFactor(0);

pub const VK_BLEND_OP_ADD: VkBlendOp = VkBlendOp(0);

pub const VK_BLEND_OP_MAX: VkBlendOp = VkBlendOp(4);

pub const VK_BLEND_OP_MIN: VkBlendOp = VkBlendOp(3);

pub const VK_BLEND_OP_REVERSE_SUBTRACT: VkBlendOp = VkBlendOp(2);

pub const VK_BLEND_OP_SUBTRACT: VkBlendOp = VkBlendOp(1);

pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: VkBorderColor = VkBorderColor(2);

pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: VkBorderColor = VkBorderColor(4);

pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: VkBorderColor = VkBorderColor(0);

pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: VkBorderColor = VkBorderColor(3);

pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: VkBorderColor = VkBorderColor(5);

pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: VkBorderColor = VkBorderColor(1);

/// Buffer should support constant data access to physical memory ranges mapped
/// into multiple locations of sparse buffers
pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkBufferCreateFlagBits =
  VkBufferCreateFlagBits(1_u32 << 2);

/// Buffer should support sparse backing
pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkBufferCreateFlagBits =
  VkBufferCreateFlagBits(1_u32 << 0);

/// Buffer should support sparse backing with partial residency
pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkBufferCreateFlagBits =
  VkBufferCreateFlagBits(1_u32 << 1);

/// Can be used as source of fixed-function index fetch (index buffer)
pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1_u32 << 6);

/// Can be the source of indirect parameters (e.g. indirect buffer, parameter
/// buffer)
pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1_u32 << 8);

/// Can be used as SSBO
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1_u32 << 5);

/// Can be used as IBO
pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1_u32 << 3);

/// Can be used as a destination of transfer operations
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1_u32 << 1);

/// Can be used as a source of transfer operations
pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1_u32 << 0);

/// Can be used as UBO
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1_u32 << 4);

/// Can be used as TBO
pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1_u32 << 2);

/// Can be used as source of fixed-function vertex fetch (VBO)
pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkBufferUsageFlagBits =
  VkBufferUsageFlagBits(1_u32 << 7);

pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1_u32 << 3);

pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1_u32 << 2);

pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1_u32 << 1);

pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1_u32 << 0);

pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = VkCommandBufferLevel(0);

pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: VkCommandBufferLevel = VkCommandBufferLevel(1);

/// Release resources owned by the buffer
pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: VkCommandBufferResetFlagBits =
  VkCommandBufferResetFlagBits(1_u32 << 0);

pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlagBits =
  VkCommandBufferUsageFlagBits(1_u32 << 0);

pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlagBits =
  VkCommandBufferUsageFlagBits(1_u32 << 1);

/// Command buffer may be submitted/executed more than once simultaneously
pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlagBits =
  VkCommandBufferUsageFlagBits(1_u32 << 2);

/// Command buffers may release their memory individually
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlagBits =
  VkCommandPoolCreateFlagBits(1_u32 << 1);

/// Command buffers have a short lifetime
pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkCommandPoolCreateFlagBits =
  VkCommandPoolCreateFlagBits(1_u32 << 0);

/// Release resources owned by the pool
pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: VkCommandPoolResetFlagBits =
  VkCommandPoolResetFlagBits(1_u32 << 0);

pub const VK_COMPARE_OP_ALWAYS: VkCompareOp = VkCompareOp(7);

pub const VK_COMPARE_OP_EQUAL: VkCompareOp = VkCompareOp(2);

pub const VK_COMPARE_OP_GREATER: VkCompareOp = VkCompareOp(4);

pub const VK_COMPARE_OP_GREATER_OR_EQUAL: VkCompareOp = VkCompareOp(6);

pub const VK_COMPARE_OP_LESS: VkCompareOp = VkCompareOp(1);

pub const VK_COMPARE_OP_LESS_OR_EQUAL: VkCompareOp = VkCompareOp(3);

pub const VK_COMPARE_OP_NEVER: VkCompareOp = VkCompareOp(0);

pub const VK_COMPARE_OP_NOT_EQUAL: VkCompareOp = VkCompareOp(5);

pub const VK_COMPONENT_SWIZZLE_A: VkComponentSwizzle = VkComponentSwizzle(6);

pub const VK_COMPONENT_SWIZZLE_B: VkComponentSwizzle = VkComponentSwizzle(5);

pub const VK_COMPONENT_SWIZZLE_G: VkComponentSwizzle = VkComponentSwizzle(4);

pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = VkComponentSwizzle(0);

pub const VK_COMPONENT_SWIZZLE_ONE: VkComponentSwizzle = VkComponentSwizzle(2);

pub const VK_COMPONENT_SWIZZLE_R: VkComponentSwizzle = VkComponentSwizzle(3);

pub const VK_COMPONENT_SWIZZLE_ZERO: VkComponentSwizzle = VkComponentSwizzle(1);

pub const VK_CULL_MODE_BACK_BIT: VkCullModeFlagBits = VkCullModeFlagBits(1_u32 << 1);

pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlagBits = VkCullModeFlagBits(0x00000003);

pub const VK_CULL_MODE_FRONT_BIT: VkCullModeFlagBits = VkCullModeFlagBits(1_u32 << 0);

pub const VK_CULL_MODE_NONE: VkCullModeFlagBits = VkCullModeFlagBits(0);

/// Dependency is per pixel region
pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlagBits = VkDependencyFlagBits(1_u32 << 0);

/// Descriptor sets may be freed individually
pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlagBits =
  VkDescriptorPoolCreateFlagBits(1_u32 << 0);

pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: VkDescriptorType = VkDescriptorType(1);

pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: VkDescriptorType = VkDescriptorType(10);

pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: VkDescriptorType = VkDescriptorType(2);

pub const VK_DESCRIPTOR_TYPE_SAMPLER: VkDescriptorType = VkDescriptorType(0);

pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: VkDescriptorType = VkDescriptorType(7);

pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: VkDescriptorType = VkDescriptorType(9);

pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: VkDescriptorType = VkDescriptorType(3);

pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: VkDescriptorType = VkDescriptorType(5);

pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: VkDescriptorType = VkDescriptorType(6);

pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: VkDescriptorType = VkDescriptorType(8);

pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: VkDescriptorType = VkDescriptorType(4);

pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: VkDynamicState = VkDynamicState(4);

pub const VK_DYNAMIC_STATE_DEPTH_BIAS: VkDynamicState = VkDynamicState(3);

pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: VkDynamicState = VkDynamicState(5);

pub const VK_DYNAMIC_STATE_LINE_WIDTH: VkDynamicState = VkDynamicState(2);

pub const VK_DYNAMIC_STATE_SCISSOR: VkDynamicState = VkDynamicState(1);

pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: VkDynamicState = VkDynamicState(6);

pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: VkDynamicState = VkDynamicState(8);

pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: VkDynamicState = VkDynamicState(7);

pub const VK_DYNAMIC_STATE_VIEWPORT: VkDynamicState = VkDynamicState(0);

/// The logical device has been lost. See
/// &lt;&lt;devsandqueues-lost-device&gt;&gt;
pub const VK_ERROR_DEVICE_LOST: VkResult = VkResult(core::num::NonZeroI32::new(-4));

/// Extension specified does not exist
pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = VkResult(core::num::NonZeroI32::new(-7));

/// Requested feature is not available on this device
pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = VkResult(core::num::NonZeroI32::new(-8));

/// Requested format is not supported on this device
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = VkResult(core::num::NonZeroI32::new(-11));

/// A requested pool allocation has failed due to fragmentation of the pool's
/// memory
pub const VK_ERROR_FRAGMENTED_POOL: VkResult = VkResult(core::num::NonZeroI32::new(-12));

/// Unable to find a Vulkan driver
pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = VkResult(core::num::NonZeroI32::new(-9));

/// Initialization of an object has failed
pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = VkResult(core::num::NonZeroI32::new(-3));

/// Layer specified does not exist
pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = VkResult(core::num::NonZeroI32::new(-6));

/// Mapping of a memory object has failed
pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = VkResult(core::num::NonZeroI32::new(-5));

/// A device memory allocation has failed
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = VkResult(core::num::NonZeroI32::new(-2));

/// A host memory allocation has failed
pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = VkResult(core::num::NonZeroI32::new(-1));

/// Too many objects of the type have already been created
pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = VkResult(core::num::NonZeroI32::new(-10));

/// An unknown error has occurred, due to an implementation or application bug
pub const VK_ERROR_UNKNOWN: VkResult = VkResult(core::num::NonZeroI32::new(-13));

/// An event is unsignaled
pub const VK_EVENT_RESET: VkResult = VkResult(core::num::NonZeroI32::new(4));

/// An event is signaled
pub const VK_EVENT_SET: VkResult = VkResult(core::num::NonZeroI32::new(3));

pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlagBits = VkFenceCreateFlagBits(1_u32 << 0);

pub const VK_FILTER_LINEAR: VkFilter = VkFilter(1);

pub const VK_FILTER_NEAREST: VkFilter = VkFilter(0);

pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: VkFormat = VkFormat(8);

pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: VkFormat = VkFormat(69);

pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: VkFormat = VkFormat(65);

pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: VkFormat = VkFormat(67);

pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: VkFormat = VkFormat(68);

pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: VkFormat = VkFormat(64);

pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: VkFormat = VkFormat(66);

pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: VkFormat = VkFormat(63);

pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: VkFormat = VkFormat(59);

pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: VkFormat = VkFormat(61);

pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: VkFormat = VkFormat(62);

pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: VkFormat = VkFormat(58);

pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: VkFormat = VkFormat(60);

pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: VkFormat = VkFormat(56);

pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: VkFormat = VkFormat(52);

pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: VkFormat = VkFormat(57);

pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: VkFormat = VkFormat(54);

pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: VkFormat = VkFormat(55);

pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: VkFormat = VkFormat(51);

pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: VkFormat = VkFormat(53);

pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: VkFormat = VkFormat(180);

pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: VkFormat = VkFormat(179);

pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: VkFormat = VkFormat(174);

pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: VkFormat = VkFormat(173);

pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: VkFormat = VkFormat(176);

pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: VkFormat = VkFormat(175);

pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: VkFormat = VkFormat(178);

pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: VkFormat = VkFormat(177);

pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: VkFormat = VkFormat(182);

pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: VkFormat = VkFormat(181);

pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: VkFormat = VkFormat(184);

pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: VkFormat = VkFormat(183);

pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: VkFormat = VkFormat(158);

pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: VkFormat = VkFormat(157);

pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: VkFormat = VkFormat(160);

pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: VkFormat = VkFormat(159);

pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: VkFormat = VkFormat(162);

pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: VkFormat = VkFormat(161);

pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: VkFormat = VkFormat(164);

pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: VkFormat = VkFormat(163);

pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: VkFormat = VkFormat(166);

pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: VkFormat = VkFormat(165);

pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: VkFormat = VkFormat(168);

pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: VkFormat = VkFormat(167);

pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: VkFormat = VkFormat(170);

pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: VkFormat = VkFormat(169);

pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: VkFormat = VkFormat(172);

pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: VkFormat = VkFormat(171);

pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: VkFormat = VkFormat(122);

pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: VkFormat = VkFormat(3);

pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: VkFormat = VkFormat(7);

pub const VK_FORMAT_B5G6R5_UNORM_PACK16: VkFormat = VkFormat(5);

pub const VK_FORMAT_B8G8R8A8_SINT: VkFormat = VkFormat(49);

pub const VK_FORMAT_B8G8R8A8_SNORM: VkFormat = VkFormat(45);

pub const VK_FORMAT_B8G8R8A8_SRGB: VkFormat = VkFormat(50);

pub const VK_FORMAT_B8G8R8A8_SSCALED: VkFormat = VkFormat(47);

pub const VK_FORMAT_B8G8R8A8_UINT: VkFormat = VkFormat(48);

pub const VK_FORMAT_B8G8R8A8_UNORM: VkFormat = VkFormat(44);

pub const VK_FORMAT_B8G8R8A8_USCALED: VkFormat = VkFormat(46);

pub const VK_FORMAT_B8G8R8_SINT: VkFormat = VkFormat(35);

pub const VK_FORMAT_B8G8R8_SNORM: VkFormat = VkFormat(31);

pub const VK_FORMAT_B8G8R8_SRGB: VkFormat = VkFormat(36);

pub const VK_FORMAT_B8G8R8_SSCALED: VkFormat = VkFormat(33);

pub const VK_FORMAT_B8G8R8_UINT: VkFormat = VkFormat(34);

pub const VK_FORMAT_B8G8R8_UNORM: VkFormat = VkFormat(30);

pub const VK_FORMAT_B8G8R8_USCALED: VkFormat = VkFormat(32);

pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: VkFormat = VkFormat(134);

pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: VkFormat = VkFormat(133);

pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: VkFormat = VkFormat(132);

pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: VkFormat = VkFormat(131);

pub const VK_FORMAT_BC2_SRGB_BLOCK: VkFormat = VkFormat(136);

pub const VK_FORMAT_BC2_UNORM_BLOCK: VkFormat = VkFormat(135);

pub const VK_FORMAT_BC3_SRGB_BLOCK: VkFormat = VkFormat(138);

pub const VK_FORMAT_BC3_UNORM_BLOCK: VkFormat = VkFormat(137);

pub const VK_FORMAT_BC4_SNORM_BLOCK: VkFormat = VkFormat(140);

pub const VK_FORMAT_BC4_UNORM_BLOCK: VkFormat = VkFormat(139);

pub const VK_FORMAT_BC5_SNORM_BLOCK: VkFormat = VkFormat(142);

pub const VK_FORMAT_BC5_UNORM_BLOCK: VkFormat = VkFormat(141);

pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: VkFormat = VkFormat(144);

pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: VkFormat = VkFormat(143);

pub const VK_FORMAT_BC7_SRGB_BLOCK: VkFormat = VkFormat(146);

pub const VK_FORMAT_BC7_UNORM_BLOCK: VkFormat = VkFormat(145);

pub const VK_FORMAT_D16_UNORM: VkFormat = VkFormat(124);

pub const VK_FORMAT_D16_UNORM_S8_UINT: VkFormat = VkFormat(128);

pub const VK_FORMAT_D24_UNORM_S8_UINT: VkFormat = VkFormat(129);

pub const VK_FORMAT_D32_SFLOAT: VkFormat = VkFormat(126);

pub const VK_FORMAT_D32_SFLOAT_S8_UINT: VkFormat = VkFormat(130);

pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: VkFormat = VkFormat(123);

pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: VkFormat = VkFormat(156);

pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: VkFormat = VkFormat(155);

pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: VkFormat = VkFormat(154);

pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: VkFormat = VkFormat(153);

pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: VkFormat = VkFormat(150);

pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: VkFormat = VkFormat(149);

pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: VkFormat = VkFormat(152);

pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: VkFormat = VkFormat(151);

pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: VkFormat = VkFormat(148);

pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: VkFormat = VkFormat(147);

/// Format can be used as the destination image of blits with vkCmdBlitImage
pub const VK_FORMAT_FEATURE_BLIT_DST_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 11);

/// Format can be used as the source image of blits with vkCmdBlitImage
pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 10);

/// Format can be used for color attachment images
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 7);

/// Format supports blending in case it is used for color attachment images
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 8);

/// Format can be used for depth/stencil attachment images
pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 9);

/// Format can be used for sampled images (SAMPLED_IMAGE and
/// COMBINED_IMAGE_SAMPLER descriptor types)
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 0);

/// Format can be filtered with VK_FILTER_LINEAR when being sampled
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 12);

/// Format supports atomic operations in case it is used for storage images
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 2);

/// Format can be used for storage images (STORAGE_IMAGE descriptor type)
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 1);

/// Format supports atomic operations in case it is used for storage texel
/// buffers
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 5);

/// Format can be used for storage texel buffers (IBOs)
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 4);

/// Format can be used for uniform texel buffers (TBOs)
pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 3);

/// Format can be used for vertex buffers (VBOs)
pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits =
  VkFormatFeatureFlagBits(1_u32 << 6);

pub const VK_FORMAT_R16G16B16A16_SFLOAT: VkFormat = VkFormat(97);

pub const VK_FORMAT_R16G16B16A16_SINT: VkFormat = VkFormat(96);

pub const VK_FORMAT_R16G16B16A16_SNORM: VkFormat = VkFormat(92);

pub const VK_FORMAT_R16G16B16A16_SSCALED: VkFormat = VkFormat(94);

pub const VK_FORMAT_R16G16B16A16_UINT: VkFormat = VkFormat(95);

pub const VK_FORMAT_R16G16B16A16_UNORM: VkFormat = VkFormat(91);

pub const VK_FORMAT_R16G16B16A16_USCALED: VkFormat = VkFormat(93);

pub const VK_FORMAT_R16G16B16_SFLOAT: VkFormat = VkFormat(90);

pub const VK_FORMAT_R16G16B16_SINT: VkFormat = VkFormat(89);

pub const VK_FORMAT_R16G16B16_SNORM: VkFormat = VkFormat(85);

pub const VK_FORMAT_R16G16B16_SSCALED: VkFormat = VkFormat(87);

pub const VK_FORMAT_R16G16B16_UINT: VkFormat = VkFormat(88);

pub const VK_FORMAT_R16G16B16_UNORM: VkFormat = VkFormat(84);

pub const VK_FORMAT_R16G16B16_USCALED: VkFormat = VkFormat(86);

pub const VK_FORMAT_R16G16_SFLOAT: VkFormat = VkFormat(83);

pub const VK_FORMAT_R16G16_SINT: VkFormat = VkFormat(82);

pub const VK_FORMAT_R16G16_SNORM: VkFormat = VkFormat(78);

pub const VK_FORMAT_R16G16_SSCALED: VkFormat = VkFormat(80);

pub const VK_FORMAT_R16G16_UINT: VkFormat = VkFormat(81);

pub const VK_FORMAT_R16G16_UNORM: VkFormat = VkFormat(77);

pub const VK_FORMAT_R16G16_USCALED: VkFormat = VkFormat(79);

pub const VK_FORMAT_R16_SFLOAT: VkFormat = VkFormat(76);

pub const VK_FORMAT_R16_SINT: VkFormat = VkFormat(75);

pub const VK_FORMAT_R16_SNORM: VkFormat = VkFormat(71);

pub const VK_FORMAT_R16_SSCALED: VkFormat = VkFormat(73);

pub const VK_FORMAT_R16_UINT: VkFormat = VkFormat(74);

pub const VK_FORMAT_R16_UNORM: VkFormat = VkFormat(70);

pub const VK_FORMAT_R16_USCALED: VkFormat = VkFormat(72);

pub const VK_FORMAT_R32G32B32A32_SFLOAT: VkFormat = VkFormat(109);

pub const VK_FORMAT_R32G32B32A32_SINT: VkFormat = VkFormat(108);

pub const VK_FORMAT_R32G32B32A32_UINT: VkFormat = VkFormat(107);

pub const VK_FORMAT_R32G32B32_SFLOAT: VkFormat = VkFormat(106);

pub const VK_FORMAT_R32G32B32_SINT: VkFormat = VkFormat(105);

pub const VK_FORMAT_R32G32B32_UINT: VkFormat = VkFormat(104);

pub const VK_FORMAT_R32G32_SFLOAT: VkFormat = VkFormat(103);

pub const VK_FORMAT_R32G32_SINT: VkFormat = VkFormat(102);

pub const VK_FORMAT_R32G32_UINT: VkFormat = VkFormat(101);

pub const VK_FORMAT_R32_SFLOAT: VkFormat = VkFormat(100);

pub const VK_FORMAT_R32_SINT: VkFormat = VkFormat(99);

pub const VK_FORMAT_R32_UINT: VkFormat = VkFormat(98);

pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: VkFormat = VkFormat(2);

pub const VK_FORMAT_R4G4_UNORM_PACK8: VkFormat = VkFormat(1);

pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: VkFormat = VkFormat(6);

pub const VK_FORMAT_R5G6B5_UNORM_PACK16: VkFormat = VkFormat(4);

pub const VK_FORMAT_R64G64B64A64_SFLOAT: VkFormat = VkFormat(121);

pub const VK_FORMAT_R64G64B64A64_SINT: VkFormat = VkFormat(120);

pub const VK_FORMAT_R64G64B64A64_UINT: VkFormat = VkFormat(119);

pub const VK_FORMAT_R64G64B64_SFLOAT: VkFormat = VkFormat(118);

pub const VK_FORMAT_R64G64B64_SINT: VkFormat = VkFormat(117);

pub const VK_FORMAT_R64G64B64_UINT: VkFormat = VkFormat(116);

pub const VK_FORMAT_R64G64_SFLOAT: VkFormat = VkFormat(115);

pub const VK_FORMAT_R64G64_SINT: VkFormat = VkFormat(114);

pub const VK_FORMAT_R64G64_UINT: VkFormat = VkFormat(113);

pub const VK_FORMAT_R64_SFLOAT: VkFormat = VkFormat(112);

pub const VK_FORMAT_R64_SINT: VkFormat = VkFormat(111);

pub const VK_FORMAT_R64_UINT: VkFormat = VkFormat(110);

pub const VK_FORMAT_R8G8B8A8_SINT: VkFormat = VkFormat(42);

pub const VK_FORMAT_R8G8B8A8_SNORM: VkFormat = VkFormat(38);

pub const VK_FORMAT_R8G8B8A8_SRGB: VkFormat = VkFormat(43);

pub const VK_FORMAT_R8G8B8A8_SSCALED: VkFormat = VkFormat(40);

pub const VK_FORMAT_R8G8B8A8_UINT: VkFormat = VkFormat(41);

pub const VK_FORMAT_R8G8B8A8_UNORM: VkFormat = VkFormat(37);

pub const VK_FORMAT_R8G8B8A8_USCALED: VkFormat = VkFormat(39);

pub const VK_FORMAT_R8G8B8_SINT: VkFormat = VkFormat(28);

pub const VK_FORMAT_R8G8B8_SNORM: VkFormat = VkFormat(24);

pub const VK_FORMAT_R8G8B8_SRGB: VkFormat = VkFormat(29);

pub const VK_FORMAT_R8G8B8_SSCALED: VkFormat = VkFormat(26);

pub const VK_FORMAT_R8G8B8_UINT: VkFormat = VkFormat(27);

pub const VK_FORMAT_R8G8B8_UNORM: VkFormat = VkFormat(23);

pub const VK_FORMAT_R8G8B8_USCALED: VkFormat = VkFormat(25);

pub const VK_FORMAT_R8G8_SINT: VkFormat = VkFormat(21);

pub const VK_FORMAT_R8G8_SNORM: VkFormat = VkFormat(17);

pub const VK_FORMAT_R8G8_SRGB: VkFormat = VkFormat(22);

pub const VK_FORMAT_R8G8_SSCALED: VkFormat = VkFormat(19);

pub const VK_FORMAT_R8G8_UINT: VkFormat = VkFormat(20);

pub const VK_FORMAT_R8G8_UNORM: VkFormat = VkFormat(16);

pub const VK_FORMAT_R8G8_USCALED: VkFormat = VkFormat(18);

pub const VK_FORMAT_R8_SINT: VkFormat = VkFormat(14);

pub const VK_FORMAT_R8_SNORM: VkFormat = VkFormat(10);

pub const VK_FORMAT_R8_SRGB: VkFormat = VkFormat(15);

pub const VK_FORMAT_R8_SSCALED: VkFormat = VkFormat(12);

pub const VK_FORMAT_R8_UINT: VkFormat = VkFormat(13);

pub const VK_FORMAT_R8_UNORM: VkFormat = VkFormat(9);

pub const VK_FORMAT_R8_USCALED: VkFormat = VkFormat(11);

pub const VK_FORMAT_S8_UINT: VkFormat = VkFormat(127);

pub const VK_FORMAT_UNDEFINED: VkFormat = VkFormat(0);

pub const VK_FORMAT_X8_D24_UNORM_PACK32: VkFormat = VkFormat(125);

pub const VK_FRONT_FACE_CLOCKWISE: VkFrontFace = VkFrontFace(1);

pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: VkFrontFace = VkFrontFace(0);

pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1_u32 << 0);

pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1_u32 << 1);

pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1_u32 << 3);

pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1_u32 << 2);

/// Allows creating image views with cube type from the created image
pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1_u32 << 4);

/// Allows image views to have different format than the base image
pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1_u32 << 3);

/// Image should support constant data access to physical memory ranges mapped
/// into multiple locations of sparse images
pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1_u32 << 2);

/// Image should support sparse backing
pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1_u32 << 0);

/// Image should support sparse backing with partial residency
pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlagBits =
  VkImageCreateFlagBits(1_u32 << 1);

/// Optimal layout when image is only used for color attachment read/write
pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: VkImageLayout = VkImageLayout(2);

/// Optimal layout when image is only used for depth/stencil attachment
/// read/write
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = VkImageLayout(3);

/// Optimal layout when image is used for read only depth/stencil attachment and
/// shader access
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = VkImageLayout(4);

/// General layout when image can be used for any kind of access
pub const VK_IMAGE_LAYOUT_GENERAL: VkImageLayout = VkImageLayout(1);

/// Initial layout used when the data is populated by the CPU
pub const VK_IMAGE_LAYOUT_PREINITIALIZED: VkImageLayout = VkImageLayout(8);

/// Optimal layout when image is used for read only shader access
pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: VkImageLayout = VkImageLayout(5);

/// Optimal layout when image is used only as destination of transfer operations
pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: VkImageLayout = VkImageLayout(7);

/// Optimal layout when image is used only as source of transfer operations
pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: VkImageLayout = VkImageLayout(6);

/// Implicit layout an image is when its contents are undefined due to various
/// reasons (e.g. right after creation)
pub const VK_IMAGE_LAYOUT_UNDEFINED: VkImageLayout = VkImageLayout(0);

pub const VK_IMAGE_TILING_LINEAR: VkImageTiling = VkImageTiling(1);

pub const VK_IMAGE_TILING_OPTIMAL: VkImageTiling = VkImageTiling(0);

pub const VK_IMAGE_TYPE_1D: VkImageType = VkImageType(0);

pub const VK_IMAGE_TYPE_2D: VkImageType = VkImageType(1);

pub const VK_IMAGE_TYPE_3D: VkImageType = VkImageType(2);

/// Can be used as framebuffer color attachment
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1_u32 << 4);

/// Can be used as framebuffer depth/stencil attachment
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1_u32 << 5);

/// Can be used as framebuffer input attachment
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1_u32 << 7);

/// Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor
/// types)
pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1_u32 << 2);

/// Can be used as storage image (STORAGE_IMAGE descriptor type)
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1_u32 << 3);

/// Can be used as a destination of transfer operations
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1_u32 << 1);

/// Can be used as a source of transfer operations
pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1_u32 << 0);

/// Image data not needed outside of rendering
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlagBits =
  VkImageUsageFlagBits(1_u32 << 6);

pub const VK_IMAGE_VIEW_TYPE_1D: VkImageViewType = VkImageViewType(0);

pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: VkImageViewType = VkImageViewType(4);

pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = VkImageViewType(1);

pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: VkImageViewType = VkImageViewType(5);

pub const VK_IMAGE_VIEW_TYPE_3D: VkImageViewType = VkImageViewType(2);

pub const VK_IMAGE_VIEW_TYPE_CUBE: VkImageViewType = VkImageViewType(3);

pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: VkImageViewType = VkImageViewType(6);

/// A return array was too small for the result
pub const VK_INCOMPLETE: VkResult = VkResult(core::num::NonZeroI32::new(5));

pub const VK_INDEX_TYPE_UINT16: VkIndexType = VkIndexType(0);

pub const VK_INDEX_TYPE_UINT32: VkIndexType = VkIndexType(1);

pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: VkInternalAllocationType =
  VkInternalAllocationType(0);

pub const VK_LOGIC_OP_AND: VkLogicOp = VkLogicOp(1);

pub const VK_LOGIC_OP_AND_INVERTED: VkLogicOp = VkLogicOp(4);

pub const VK_LOGIC_OP_AND_REVERSE: VkLogicOp = VkLogicOp(2);

pub const VK_LOGIC_OP_CLEAR: VkLogicOp = VkLogicOp(0);

pub const VK_LOGIC_OP_COPY: VkLogicOp = VkLogicOp(3);

pub const VK_LOGIC_OP_COPY_INVERTED: VkLogicOp = VkLogicOp(12);

pub const VK_LOGIC_OP_EQUIVALENT: VkLogicOp = VkLogicOp(9);

pub const VK_LOGIC_OP_INVERT: VkLogicOp = VkLogicOp(10);

pub const VK_LOGIC_OP_NAND: VkLogicOp = VkLogicOp(14);

pub const VK_LOGIC_OP_NOR: VkLogicOp = VkLogicOp(8);

pub const VK_LOGIC_OP_NO_OP: VkLogicOp = VkLogicOp(5);

pub const VK_LOGIC_OP_OR: VkLogicOp = VkLogicOp(7);

pub const VK_LOGIC_OP_OR_INVERTED: VkLogicOp = VkLogicOp(13);

pub const VK_LOGIC_OP_OR_REVERSE: VkLogicOp = VkLogicOp(11);

pub const VK_LOGIC_OP_SET: VkLogicOp = VkLogicOp(15);

pub const VK_LOGIC_OP_XOR: VkLogicOp = VkLogicOp(6);

/// If set, heap represents device memory
pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT: VkMemoryHeapFlagBits = VkMemoryHeapFlagBits(1_u32 << 0);

/// If otherwise stated, then allocate memory on device
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1_u32 << 0);

/// Memory will be cached by the host
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1_u32 << 3);

/// Memory will have i/o coherency. If not set, application may need to use
/// vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to
/// flush/invalidate host cache
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1_u32 << 2);

/// Memory is mappable by host
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1_u32 << 1);

/// Memory may be allocated by the driver when it is required
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlagBits =
  VkMemoryPropertyFlagBits(1_u32 << 4);

/// A fence or query has not yet completed
pub const VK_NOT_READY: VkResult = VkResult(core::num::NonZeroI32::new(1));

pub const VK_OBJECT_TYPE_BUFFER: VkObjectType = VkObjectType(9);

pub const VK_OBJECT_TYPE_BUFFER_VIEW: VkObjectType = VkObjectType(13);

pub const VK_OBJECT_TYPE_COMMAND_BUFFER: VkObjectType = VkObjectType(6);

pub const VK_OBJECT_TYPE_COMMAND_POOL: VkObjectType = VkObjectType(25);

pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: VkObjectType = VkObjectType(22);

pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: VkObjectType = VkObjectType(23);

pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: VkObjectType = VkObjectType(20);

pub const VK_OBJECT_TYPE_DEVICE: VkObjectType = VkObjectType(3);

pub const VK_OBJECT_TYPE_DEVICE_MEMORY: VkObjectType = VkObjectType(8);

pub const VK_OBJECT_TYPE_EVENT: VkObjectType = VkObjectType(11);

pub const VK_OBJECT_TYPE_FENCE: VkObjectType = VkObjectType(7);

pub const VK_OBJECT_TYPE_FRAMEBUFFER: VkObjectType = VkObjectType(24);

pub const VK_OBJECT_TYPE_IMAGE: VkObjectType = VkObjectType(10);

pub const VK_OBJECT_TYPE_IMAGE_VIEW: VkObjectType = VkObjectType(14);

pub const VK_OBJECT_TYPE_INSTANCE: VkObjectType = VkObjectType(1);

pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: VkObjectType = VkObjectType(2);

pub const VK_OBJECT_TYPE_PIPELINE: VkObjectType = VkObjectType(19);

pub const VK_OBJECT_TYPE_PIPELINE_CACHE: VkObjectType = VkObjectType(16);

pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: VkObjectType = VkObjectType(17);

pub const VK_OBJECT_TYPE_QUERY_POOL: VkObjectType = VkObjectType(12);

pub const VK_OBJECT_TYPE_QUEUE: VkObjectType = VkObjectType(4);

pub const VK_OBJECT_TYPE_RENDER_PASS: VkObjectType = VkObjectType(18);

pub const VK_OBJECT_TYPE_SAMPLER: VkObjectType = VkObjectType(21);

pub const VK_OBJECT_TYPE_SEMAPHORE: VkObjectType = VkObjectType(5);

pub const VK_OBJECT_TYPE_SHADER_MODULE: VkObjectType = VkObjectType(15);

pub const VK_OBJECT_TYPE_UNKNOWN: VkObjectType = VkObjectType(0);

pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = VkPhysicalDeviceType(4);

pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(2);

pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(1);

pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = VkPhysicalDeviceType(0);

pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(3);

pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint = VkPipelineBindPoint(1);

pub const VK_PIPELINE_BIND_POINT_GRAPHICS: VkPipelineBindPoint = VkPipelineBindPoint(0);

pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE: VkPipelineCacheHeaderVersion =
  VkPipelineCacheHeaderVersion(1);

pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1_u32 << 1);

pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1_u32 << 2);

pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits =
  VkPipelineCreateFlagBits(1_u32 << 0);

/// All stages supported on the queue
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 16);

/// All stages of the graphics pipeline
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 15);

/// After previous commands have completed
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 13);

/// Color attachment writes
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 10);

/// Compute shading
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 11);

/// Draw/DispatchIndirect command fetch
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 1);

/// Early fragment (depth and stencil) tests
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 8);

/// Fragment shading
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 7);

/// Geometry shading
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 6);

/// Indicates host (CPU) is a source/sink of the dependency
pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 14);

/// Late fragment (depth and stencil) tests
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 9);

/// Tessellation control shading
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 4);

/// Tessellation evaluation shading
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 5);

/// Before subsequent commands are processed
pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 0);

/// Transfer/copy operations
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 12);

/// Vertex/index fetch
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 2);

/// Vertex shading
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlagBits =
  VkPipelineStageFlagBits(1_u32 << 3);

pub const VK_POLYGON_MODE_FILL: VkPolygonMode = VkPolygonMode(0);

pub const VK_POLYGON_MODE_LINE: VkPolygonMode = VkPolygonMode(1);

pub const VK_POLYGON_MODE_POINT: VkPolygonMode = VkPolygonMode(2);

pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: VkPrimitiveTopology = VkPrimitiveTopology(1);

pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: VkPrimitiveTopology =
  VkPrimitiveTopology(6);

pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: VkPrimitiveTopology = VkPrimitiveTopology(2);

pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology =
  VkPrimitiveTopology(7);

pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: VkPrimitiveTopology = VkPrimitiveTopology(10);

pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: VkPrimitiveTopology = VkPrimitiveTopology(0);

pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: VkPrimitiveTopology = VkPrimitiveTopology(5);

pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: VkPrimitiveTopology = VkPrimitiveTopology(3);

pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: VkPrimitiveTopology =
  VkPrimitiveTopology(8);

pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: VkPrimitiveTopology = VkPrimitiveTopology(4);

pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology =
  VkPrimitiveTopology(9);

/// Require precise results to be collected by the query
pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlagBits = VkQueryControlFlagBits(1_u32 << 0);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits =
  VkQueryPipelineStatisticFlagBits(1_u32 << 5);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits =
  VkQueryPipelineStatisticFlagBits(1_u32 << 6);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1_u32 << 10);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1_u32 << 7);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1_u32 << 3);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1_u32 << 4);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1_u32 << 1);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1_u32 << 0);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1_u32 << 8);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1_u32 << 9);

/// Optional
pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT:
  VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1_u32 << 2);

/// Results of the queries are written to the destination buffer as 64-bit
/// values
pub const VK_QUERY_RESULT_64_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1_u32 << 0);

/// Copy the partial results of the query even if the final results are not
/// available
pub const VK_QUERY_RESULT_PARTIAL_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1_u32 << 3);

/// Results of the queries are waited on before proceeding with the result copy
pub const VK_QUERY_RESULT_WAIT_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1_u32 << 1);

/// Besides the results of the query, the availability of the results is also
/// written
pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT: VkQueryResultFlagBits =
  VkQueryResultFlagBits(1_u32 << 2);

pub const VK_QUERY_TYPE_OCCLUSION: VkQueryType = VkQueryType(0);

/// Optional
pub const VK_QUERY_TYPE_PIPELINE_STATISTICS: VkQueryType = VkQueryType(1);

pub const VK_QUERY_TYPE_TIMESTAMP: VkQueryType = VkQueryType(2);

/// Queue supports compute operations
pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlagBits = VkQueueFlagBits(1_u32 << 1);

/// Queue supports graphics operations
pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlagBits = VkQueueFlagBits(1_u32 << 0);

/// Queue supports sparse resource memory management operations
pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlagBits = VkQueueFlagBits(1_u32 << 3);

/// Queue supports transfer operations
pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlagBits = VkQueueFlagBits(1_u32 << 2);

pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: VkSamplerAddressMode = VkSamplerAddressMode(3);

pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: VkSamplerAddressMode = VkSamplerAddressMode(2);

pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: VkSamplerAddressMode = VkSamplerAddressMode(1);

pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: VkSamplerAddressMode = VkSamplerAddressMode(0);

/// Linear filter between mip levels
pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: VkSamplerMipmapMode = VkSamplerMipmapMode(1);

/// Choose nearest mip level
pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: VkSamplerMipmapMode = VkSamplerMipmapMode(0);

/// Sample count 16 supported
pub const VK_SAMPLE_COUNT_16_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1_u32 << 4);

/// Sample count 1 supported
pub const VK_SAMPLE_COUNT_1_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1_u32 << 0);

/// Sample count 2 supported
pub const VK_SAMPLE_COUNT_2_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1_u32 << 1);

/// Sample count 32 supported
pub const VK_SAMPLE_COUNT_32_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1_u32 << 5);

/// Sample count 4 supported
pub const VK_SAMPLE_COUNT_4_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1_u32 << 2);

/// Sample count 64 supported
pub const VK_SAMPLE_COUNT_64_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1_u32 << 6);

/// Sample count 8 supported
pub const VK_SAMPLE_COUNT_8_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1_u32 << 3);

pub const VK_SHADER_STAGE_ALL: VkShaderStageFlagBits = VkShaderStageFlagBits(0x7FFFFFFF);

pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlagBits = VkShaderStageFlagBits(0x0000001F);

pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1_u32 << 5);

pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1_u32 << 4);

pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1_u32 << 3);

pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1_u32 << 1);

pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlagBits =
  VkShaderStageFlagBits(1_u32 << 2);

pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1_u32 << 0);

pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = VkSharingMode(1);

pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = VkSharingMode(0);

/// Image requires mip level dimensions to be an integer multiple of the sparse
/// image block dimensions for non-tail mip levels.
pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlagBits =
  VkSparseImageFormatFlagBits(1_u32 << 1);

/// Image uses a non-standard sparse image block dimensions
pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlagBits =
  VkSparseImageFormatFlagBits(1_u32 << 2);

/// Image uses a single mip tail region for all array layers
pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlagBits =
  VkSparseImageFormatFlagBits(1_u32 << 0);

/// Operation binds resource metadata to memory
pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT: VkSparseMemoryBindFlagBits =
  VkSparseMemoryBindFlagBits(1_u32 << 0);

/// Back face
pub const VK_STENCIL_FACE_BACK_BIT: VkStencilFaceFlagBits = VkStencilFaceFlagBits(1_u32 << 1);

/// Front and back faces
pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlagBits = VkStencilFaceFlagBits(0x00000003);

/// Front face
pub const VK_STENCIL_FACE_FRONT_BIT: VkStencilFaceFlagBits = VkStencilFaceFlagBits(1_u32 << 0);

/// * Alias For [`VK_STENCIL_FACE_FRONT_AND_BACK`]
#[deprecated = "aliased"]
pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlagBits = VK_STENCIL_FACE_FRONT_AND_BACK;

pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: VkStencilOp = VkStencilOp(4);

pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: VkStencilOp = VkStencilOp(7);

pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: VkStencilOp = VkStencilOp(3);

pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: VkStencilOp = VkStencilOp(6);

pub const VK_STENCIL_OP_INVERT: VkStencilOp = VkStencilOp(5);

pub const VK_STENCIL_OP_KEEP: VkStencilOp = VkStencilOp(0);

pub const VK_STENCIL_OP_REPLACE: VkStencilOp = VkStencilOp(2);

pub const VK_STENCIL_OP_ZERO: VkStencilOp = VkStencilOp(1);

pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = VkStructureType(0);

pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: VkStructureType = VkStructureType(7);

pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = VkStructureType(12);

pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: VkStructureType = VkStructureType(44);

pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: VkStructureType = VkStructureType(13);

pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = VkStructureType(40);

pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = VkStructureType(42);

pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: VkStructureType = VkStructureType(41);

pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = VkStructureType(39);

pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = VkStructureType(29);

pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: VkStructureType = VkStructureType(36);

pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = VkStructureType(33);

pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = VkStructureType(34);

pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType =
  VkStructureType(32);

pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = VkStructureType(3);

pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = VkStructureType(2);

pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: VkStructureType = VkStructureType(10);

pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = VkStructureType(8);

pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: VkStructureType = VkStructureType(37);

pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType = VkStructureType(28);

pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: VkStructureType = VkStructureType(14);

pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: VkStructureType = VkStructureType(45);

pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = VkStructureType(15);

pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = VkStructureType(1);

/// Reserved for internal use by the loader, layers, and ICDs
pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: VkStructureType = VkStructureType(48);

/// Reserved for internal use by the loader, layers, and ICDs
pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: VkStructureType = VkStructureType(47);

pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: VkStructureType = VkStructureType(6);

pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = VkStructureType(5);

pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: VkStructureType = VkStructureType(46);

pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: VkStructureType = VkStructureType(17);

pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(26);

pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(25);

pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(27);

pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(20);

pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = VkStructureType(30);

pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(24);

pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(23);

pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType =
  VkStructureType(18);

pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(21);

pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(19);

pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(22);

pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: VkStructureType = VkStructureType(11);

pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: VkStructureType = VkStructureType(43);

pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: VkStructureType = VkStructureType(38);

pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: VkStructureType = VkStructureType(31);

pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = VkStructureType(9);

pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = VkStructureType(16);

pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = VkStructureType(4);

pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = VkStructureType(35);

pub const VK_SUBPASS_CONTENTS_INLINE: VkSubpassContents = VkSubpassContents(0);

pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: VkSubpassContents = VkSubpassContents(1);

/// Command completed successfully
pub const VK_SUCCESS: VkResult = VkResult(core::num::NonZeroI32::new(0));

pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: VkSystemAllocationScope = VkSystemAllocationScope(2);

pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: VkSystemAllocationScope = VkSystemAllocationScope(0);

pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: VkSystemAllocationScope = VkSystemAllocationScope(3);

pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: VkSystemAllocationScope = VkSystemAllocationScope(4);

pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: VkSystemAllocationScope = VkSystemAllocationScope(1);

/// A wait operation has not completed in the specified time
pub const VK_TIMEOUT: VkResult = VkResult(core::num::NonZeroI32::new(2));

/// Codeplay Software Ltd. vendor ID
pub const VK_VENDOR_ID_CODEPLAY: VkVendorId = VkVendorId(0x10004);

/// Kazan Software Renderer
pub const VK_VENDOR_ID_KAZAN: VkVendorId = VkVendorId(0x10003);

/// Mesa vendor ID
pub const VK_VENDOR_ID_MESA: VkVendorId = VkVendorId(0x10005);

/// PoCL vendor ID
pub const VK_VENDOR_ID_POCL: VkVendorId = VkVendorId(0x10006);

/// Vivante vendor ID
pub const VK_VENDOR_ID_VIV: VkVendorId = VkVendorId(0x10001);

/// VeriSilicon vendor ID
pub const VK_VENDOR_ID_VSI: VkVendorId = VkVendorId(0x10002);

pub const VK_VERTEX_INPUT_RATE_INSTANCE: VkVertexInputRate = VkVertexInputRate(1);

pub const VK_VERTEX_INPUT_RATE_VERTEX: VkVertexInputRate = VkVertexInputRate(0);
