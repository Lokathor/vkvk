#![allow(dead_code)]

use crate::prelude::*;

#[cfg_attr(windows, link(name = "vulkan-1"))]
#[cfg_attr(not(windows), link(name = "vulkan"))]
#[allow(dead_code)]
extern "system" {
  /// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
  pub(crate) fn vkGetInstanceProcAddr(
    instance: VkInstance, name: *const u8,
  ) -> PFN_vkVoidFunction;
}

#[allow(non_camel_case_types)]
pub(crate) type vkGetInstanceProcAddr_t =
  unsafe extern "system" fn(VkInstance, *const u8) -> PFN_vkVoidFunction;

#[repr(transparent)]
pub struct Entry(vkGetInstanceProcAddr_t);
impl Entry {
  pub const LINKED: Self = Self(vkGetInstanceProcAddr);

  pub fn enumerate_instance_layer_properties(
    &self,
  ) -> Result<Vec<VkLayerProperties>, NonZeroI32> {
    let pfn_void = unsafe {
      vkGetInstanceProcAddr(
        VkInstance::NULL,
        vkEnumerateInstanceLayerProperties_NAME.as_ptr(),
      )
    };
    let f: vkEnumerateInstanceLayerProperties_t = if let Some(v) = pfn_void {
      unsafe { core::mem::transmute(v) }
    } else {
      return Err(VK_ERROR_UNKNOWN.0.unwrap());
    };
    let mut count: u32 = 0;
    if let Some(err) = unsafe { f(&mut count, null_mut()) }.0 {
      return Err(err);
    }
    let mut buf = Vec::with_capacity(count.try_into().unwrap());
    if let Some(err) = unsafe { f(&mut count, buf.as_mut_ptr()) }.0 {
      Err(err)
    } else {
      unsafe { buf.set_len(count.try_into().unwrap()) };
      Ok(buf)
    }
  }

  pub fn enumerate_instance_extension_properties(
    &self, layer: Option<&str>,
  ) -> Result<Vec<VkExtensionProperties>, NonZeroI32> {
    let pfn_void = unsafe {
      vkGetInstanceProcAddr(
        VkInstance::NULL,
        vkEnumerateInstanceExtensionProperties_NAME.as_ptr(),
      )
    };
    let f: vkEnumerateInstanceExtensionProperties_t = if let Some(v) = pfn_void {
      unsafe { core::mem::transmute(v) }
    } else {
      return Err(VK_ERROR_UNKNOWN.0.unwrap());
    };
    let layer = layer.map(|s| alloc::format!("{s}\0"));
    let layer_p = match layer.as_ref() {
      None => core::ptr::null(),
      Some(string_ref) => string_ref.as_ptr(),
    };
    let mut count: u32 = 0;
    if let Some(err) = unsafe { f(layer_p, &mut count, null_mut()) }.0 {
      return Err(err);
    }
    let mut buf = Vec::with_capacity(count.try_into().unwrap());
    if let Some(err) = unsafe { f(layer_p, &mut count, buf.as_mut_ptr()) }.0 {
      Err(err)
    } else {
      unsafe { buf.set_len(count.try_into().unwrap()) };
      Ok(buf)
    }
  }

  #[allow(clippy::too_many_arguments)]
  pub fn create_instance(
    &self, application_name: Option<ZStr<'_>>, application_version: u32,
    engine_name: Option<ZStr<'_>>, engine_version: u32, api_version: VkVersion,
    instance_create_flags: VkInstanceCreateFlags, layers: &[ZStr<'_>],
    extensions: &[ZStr<'_>],
  ) -> Result<Instance, NonZeroI32> {
    let pfn_void =
      unsafe { vkGetInstanceProcAddr(VkInstance::NULL, vkCreateInstance_NAME.as_ptr()) };
    let vk_create_instance: vkCreateInstance_t = if let Some(v) = pfn_void {
      unsafe { core::mem::transmute(v) }
    } else {
      return Err(VK_ERROR_UNKNOWN.0.unwrap());
    };
    //
    let application_info = VkApplicationInfo {
      application_name: application_name.map(ZStr::as_ptr).unwrap_or(null()),
      engine_name: engine_name.map(ZStr::as_ptr).unwrap_or(null()),
      application_version,
      engine_version,
      api_version,
      ..VkApplicationInfo::default()
    };
    let instance_crate_info = VkInstanceCreateInfo {
      flags: instance_create_flags,
      application_info: &application_info,
      enabled_layer_count: layers.len().try_into().unwrap(),
      enabled_layer_names: layers.as_ptr().cast(),
      enabled_extension_count: extensions.len().try_into().unwrap(),
      enabled_extension_names: extensions.as_ptr().cast(),
      ..VkInstanceCreateInfo::default()
    };
    let mut vk_instance = VkInstance::NULL;
    if let Some(err) = unsafe {
      vk_create_instance(&instance_crate_info, core::ptr::null(), &mut vk_instance).0
    } {
      Err(err)
    } else {
      unsafe { Instance::new(self.0, vk_instance) }
    }
  }
}
