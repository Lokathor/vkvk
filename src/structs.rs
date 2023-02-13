use super::*;

#[derive(Clone, Copy)]
#[repr(C)]
struct VkAllocationCallbacks {
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
