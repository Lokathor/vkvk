use super::*;

mod vk_device_queue_create_info;
pub use vk_device_queue_create_info::*;

mod vk_image_format_properties;
pub use vk_image_format_properties::*;

mod vk_image_view_create_info;
pub use vk_image_view_create_info::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAllocationCallbacks {
  pub user_data: *mut c_void,
  pub allocation: PFN_vkAllocationFunction,
  pub reallocation: PFN_vkReallocationFunction,
  pub free: PFN_vkFreeFunction,
  pub internal_allocation: PFN_vkInternalAllocationNotification,
  pub internal_free: PFN_vkInternalFreeNotification,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VkLayerProperties {
  pub layer_name: ArrayZStr<VK_MAX_EXTENSION_NAME_SIZE>,
  pub spec_version: VkVersion,
  pub implementation_version: uint32_t,
  pub description: ArrayZStr<VK_MAX_DESCRIPTION_SIZE>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VkExtensionProperties {
  pub extension_name: ArrayZStr<VK_MAX_EXTENSION_NAME_SIZE>,
  pub spec_version: uint32_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkApplicationInfo {
  pub ty: VkStructureType,
  pub next: *const c_void,
  pub application_name: *const u8,
  pub application_version: uint32_t,
  pub engine_name: *const u8,
  pub engine_version: uint32_t,
  pub api_version: VkVersion,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
  pub ty: VkStructureType,
  pub next: *const c_void,
  pub flags: VkInstanceCreateFlags,
  pub application_info: *const VkApplicationInfo,
  pub enabled_layer_count: uint32_t,
  pub enabled_layer_names: *const *const u8,
  pub enabled_extension_count: uint32_t,
  pub enabled_extension_names: *const *const u8,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
  pub max_image_dimension_1d: uint32_t,
  pub max_image_dimension_2d: uint32_t,
  pub max_image_dimension_3d: uint32_t,
  pub max_image_dimension_cube: uint32_t,
  pub max_image_array_layers: uint32_t,
  pub max_texel_buffer_elements: uint32_t,
  pub max_uniform_buffer_range: uint32_t,
  pub max_storage_buffer_range: uint32_t,
  pub max_push_constants_size: uint32_t,
  pub max_memory_allocation_count: uint32_t,
  pub max_sampler_allocation_count: uint32_t,
  pub buffer_image_granularity: VkDeviceSize,
  pub sparse_address_space_size: VkDeviceSize,
  pub max_bound_descriptor_sets: uint32_t,
  pub max_per_stage_descriptor_samplers: uint32_t,
  pub max_per_stage_descriptor_uniform_buffers: uint32_t,
  pub max_per_stage_descriptor_storage_buffers: uint32_t,
  pub max_per_stage_descriptor_sampled_images: uint32_t,
  pub max_per_stage_descriptor_storage_images: uint32_t,
  pub max_per_stage_descriptor_input_attachments: uint32_t,
  pub max_per_stage_resources: uint32_t,
  pub max_descriptor_set_samplers: uint32_t,
  pub max_descriptor_set_uniform_buffers: uint32_t,
  pub max_descriptor_set_uniform_buffers_dynamic: uint32_t,
  pub max_descriptor_set_storage_buffers: uint32_t,
  pub max_descriptor_set_storage_buffers_dynamic: uint32_t,
  pub max_descriptor_set_sampled_images: uint32_t,
  pub max_descriptor_set_storage_images: uint32_t,
  pub max_descriptor_set_input_attachments: uint32_t,
  pub max_vertex_input_attributes: uint32_t,
  pub max_vertex_input_bindings: uint32_t,
  pub max_vertex_input_attribute_offset: uint32_t,
  pub max_vertex_input_binding_stride: uint32_t,
  pub max_vertex_output_components: uint32_t,
  pub max_tessellation_generation_level: uint32_t,
  pub max_tessellation_patch_size: uint32_t,
  pub max_tessellation_control_per_vertex_input_components: uint32_t,
  pub max_tessellation_control_per_vertex_output_components: uint32_t,
  pub max_tessellation_control_per_patch_output_components: uint32_t,
  pub max_tessellation_control_total_output_components: uint32_t,
  pub max_tessellation_evaluation_input_components: uint32_t,
  pub max_tessellation_evaluation_output_components: uint32_t,
  pub max_geometry_shader_invocations: uint32_t,
  pub max_geometry_input_components: uint32_t,
  pub max_geometry_output_components: uint32_t,
  pub max_geometry_output_vertices: uint32_t,
  pub max_geometry_total_output_components: uint32_t,
  pub max_fragment_input_components: uint32_t,
  pub max_fragment_output_attachments: uint32_t,
  pub max_fragment_dual_src_attachments: uint32_t,
  pub max_fragment_combined_output_resources: uint32_t,
  pub max_compute_shared_memory_size: uint32_t,
  pub max_compute_work_group_count: [uint32_t; 3],
  pub max_compute_work_group_invocations: uint32_t,
  pub max_compute_work_group_size: [uint32_t; 3],
  pub sub_pixel_precision_bits: uint32_t,
  pub sub_texel_precision_bits: uint32_t,
  pub mipmap_precision_bits: uint32_t,
  pub max_draw_indexed_index_value: uint32_t,
  pub max_draw_indirect_count: uint32_t,
  pub max_sampler_lod_bias: float,
  pub max_sampler_anisotropy: float,
  pub max_viewports: uint32_t,
  pub max_viewport_dimensions: [uint32_t; 2],
  pub viewport_bounds_range: [float; 2],
  pub viewport_sub_pixel_bits: uint32_t,
  pub min_memory_map_alignment: size_t,
  pub min_texel_buffer_offset_alignment: VkDeviceSize,
  pub min_uniform_buffer_offset_alignment: VkDeviceSize,
  pub min_storage_buffer_offset_alignment: VkDeviceSize,
  pub min_texel_offset: int32_t,
  pub max_texel_offset: uint32_t,
  pub min_texel_gather_offset: int32_t,
  pub max_texel_gather_offset: uint32_t,
  pub min_interpolation_offset: float,
  pub max_interpolation_offset: float,
  pub sub_pixel_interpolation_offset_bits: uint32_t,
  pub max_framebuffer_width: uint32_t,
  pub max_framebuffer_height: uint32_t,
  pub max_framebuffer_layers: uint32_t,
  pub framebuffer_color_sample_counts: VkSampleCountFlags,
  pub framebuffer_depth_sample_counts: VkSampleCountFlags,
  pub framebuffer_stencil_sample_counts: VkSampleCountFlags,
  pub framebuffer_no_attachments_sample_counts: VkSampleCountFlags,
  pub max_color_attachments: uint32_t,
  pub sampled_image_color_sample_counts: VkSampleCountFlags,
  pub sampled_image_integer_sample_counts: VkSampleCountFlags,
  pub sampled_image_depth_sample_counts: VkSampleCountFlags,
  pub sampled_image_stencil_sample_counts: VkSampleCountFlags,
  pub storage_image_sample_counts: VkSampleCountFlags,
  pub max_sample_mask_words: uint32_t,
  pub timestamp_compute_and_graphics: VkBool32,
  pub timestamp_period: float,
  pub max_clip_distances: uint32_t,
  pub max_cull_distances: uint32_t,
  pub max_combined_clip_and_cull_distances: uint32_t,
  pub discrete_queue_priorities: uint32_t,
  pub point_size_range: [float; 2],
  pub line_width_range: [float; 2],
  pub point_size_granularity: float,
  pub line_width_granularity: float,
  pub strict_lines: VkBool32,
  pub standard_sample_locations: VkBool32,
  pub optimal_buffer_copy_offset_alignment: VkDeviceSize,
  pub optimal_buffer_copy_row_pitch_alignment: VkDeviceSize,
  pub non_coherent_atom_size: VkDeviceSize,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
  pub residency_standard_2d_block_shape: VkBool32,
  pub residency_standard_2d_multisample_block_shape: VkBool32,
  pub residency_standard_3d_block_shape: VkBool32,
  pub residency_aligned_mip_size: VkBool32,
  pub residency_non_resident_strict: VkBool32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
  pub api_version: VkVersion,
  pub driver_version: uint32_t,
  pub vendor_id: uint32_t,
  pub device_id: uint32_t,
  pub device_type: VkPhysicalDeviceType,
  pub device_name: ArrayZStr<VK_MAX_PHYSICAL_DEVICE_NAME_SIZE>,
  pub pipeline_cache_uuid: [uint8_t; VK_UUID_SIZE],
  pub limits: VkPhysicalDeviceLimits,
  pub sparse_properties: VkPhysicalDeviceSparseProperties,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
  pub robust_buffer_access: VkBool32,
  pub full_draw_index_uint32: VkBool32,
  pub image_cube_array: VkBool32,
  pub independent_blend: VkBool32,
  pub geometry_shader: VkBool32,
  pub tessellation_shader: VkBool32,
  pub sample_rate_shading: VkBool32,
  pub dual_src_blend: VkBool32,
  pub logic_op: VkBool32,
  pub multi_draw_indirect: VkBool32,
  pub draw_indirect_first_instance: VkBool32,
  pub depth_clamp: VkBool32,
  pub depth_bias_clamp: VkBool32,
  pub fill_mode_non_solid: VkBool32,
  pub depth_bounds: VkBool32,
  pub wide_lines: VkBool32,
  pub large_points: VkBool32,
  pub alpha_to_one: VkBool32,
  pub multi_viewport: VkBool32,
  pub sampler_anisotropy: VkBool32,
  pub texture_compression_etc2: VkBool32,
  pub texture_compression_astc_ldr: VkBool32,
  pub texture_compression_bc: VkBool32,
  pub occlusion_query_precise: VkBool32,
  pub pipeline_statistics_query: VkBool32,
  pub vertex_pipeline_stores_and_atomics: VkBool32,
  pub fragment_stores_and_atomics: VkBool32,
  pub shader_tessellation_and_geometry_point_size: VkBool32,
  pub shader_image_gather_extended: VkBool32,
  pub shader_storage_image_extended_formats: VkBool32,
  pub shader_storage_image_multisample: VkBool32,
  pub shader_storage_image_read_without_format: VkBool32,
  pub shader_storage_image_write_without_format: VkBool32,
  pub shader_uniform_buffer_array_dynamic_indexing: VkBool32,
  pub shader_sampled_image_array_dynamic_indexing: VkBool32,
  pub shader_storage_buffer_array_dynamic_indexing: VkBool32,
  pub shader_storage_image_array_dynamic_indexing: VkBool32,
  pub shader_clip_distance: VkBool32,
  pub shader_cull_distance: VkBool32,
  pub shader_float64: VkBool32,
  pub shader_int64: VkBool32,
  pub shader_int16: VkBool32,
  pub shader_resource_residency: VkBool32,
  pub shader_resource_min_lod: VkBool32,
  pub sparse_binding: VkBool32,
  pub sparse_residency_buffer: VkBool32,
  pub sparse_residency_image_2d: VkBool32,
  pub sparse_residency_image_3d: VkBool32,
  pub sparse_residency_2_samples: VkBool32,
  pub sparse_residency_4_samples: VkBool32,
  pub sparse_residency_8_samples: VkBool32,
  pub sparse_residency_16_samples: VkBool32,
  pub sparse_residency_aliased: VkBool32,
  pub variable_multisample_rate: VkBool32,
  pub inherited_queries: VkBool32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkExtent2D {
  pub width: uint32_t,
  pub height: uint32_t,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkExtent3D {
  pub width: uint32_t,
  pub height: uint32_t,
  pub depth: uint32_t,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkQueueFamilyProperties {
  pub queue_flags: VkQueueFlags,
  pub queue_count: uint32_t,
  pub timestamp_valid_bits: uint32_t,
  pub min_image_transfer_granularity: VkExtent3D,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceCreateInfo {
  pub ty: VkStructureType,
  pub next: *const c_void,
  pub flags: VkDeviceCreateFlags,
  pub queue_create_info_count: uint32_t,
  pub queue_create_infos: *const VkDeviceQueueCreateInfo,
  pub enabled_layer_count: uint32_t,
  pub enabled_layer_names: *const *const u8,
  pub enabled_extension_count: uint32_t,
  pub enabled_extension_names: *const *const u8,
  pub enabled_features: *const VkPhysicalDeviceFeatures,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
  pub min_image_count: uint32_t,
  pub max_image_count: uint32_t,
  pub current_extent: VkExtent2D,
  pub min_image_extent: VkExtent2D,
  pub max_image_extent: VkExtent2D,
  pub max_image_array_layers: uint32_t,
  pub supported_transforms: VkSurfaceTransformFlagsKHR,
  pub current_transform: VkSurfaceTransformFlagBitsKHR,
  pub supported_composite_alpha: VkCompositeAlphaFlagsKHR,
  pub supported_usage_flags: VkImageUsageFlags,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VkSurfaceFormatKHR {
  pub format: VkFormat,
  pub color_space: VkColorSpaceKHR,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
  pub ty: VkStructureType,
  pub next: *const c_void,
  pub flags: VkSwapchainCreateFlagsKHR,
  pub surface: VkSurfaceKHR,
  pub min_image_count: uint32_t,
  pub image_format: VkFormat,
  pub image_color_space: VkColorSpaceKHR,
  pub image_extent: VkExtent2D,
  pub image_array_layers: uint32_t,
  pub image_usage: VkImageUsageFlags,
  pub image_sharing_mode: VkSharingMode,
  pub queue_family_index_count: uint32_t,
  pub queue_family_indices: *const uint32_t,
  pub pre_transform: VkSurfaceTransformFlagBitsKHR,
  pub composite_alpha: VkCompositeAlphaFlagBitsKHR,
  pub present_mode: VkPresentModeKHR,
  pub clipped: VkBool32,
  pub old_swapchain: VkSwapchainKHR,
}
