#![allow(non_snake_case)]

use super::*;

pub struct Instance {
  pub(crate) entry: Entry,
  pub(crate) vk_instance: VkInstance,
}
impl Instance {
  #[inline]
  #[must_use]
  pub const fn vk_instance(&self) -> VkInstance {
    self.vk_instance
  }

  #[inline]
  pub fn get_physical_devices(&self) -> Result<Vec<VkPhysicalDevice>, VkErrorCode> {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkEnumeratePhysicalDevices_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkEnumeratePhysicalDevices: vkEnumeratePhysicalDevices_t =
      unsafe { core::mem::transmute(f) };
    //
    let mut phys_device_count: u32 = 0;
    let count_ret =
      unsafe { vkEnumeratePhysicalDevices(self.vk_instance, &mut phys_device_count, null_mut()) };
    if let Some(err) = count_ret.0 {
      return Err(err);
    }
    let mut buf = Vec::with_capacity(phys_device_count.try_into().unwrap());
    let write_ret = unsafe {
      vkEnumeratePhysicalDevices(self.vk_instance, &mut phys_device_count, buf.as_mut_ptr())
    };
    if let Some(err) = write_ret.0 {
      Err(err)
    } else {
      unsafe { buf.set_len(phys_device_count.try_into().unwrap()) }
      Ok(buf)
    }
  }

  #[inline]
  pub fn get_physical_device_properties(
    &self, physical_device: VkPhysicalDevice,
  ) -> VkPhysicalDeviceProperties {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkGetPhysicalDeviceProperties_NAME.as_ptr()) }) else {
      return VkPhysicalDeviceProperties::default()
    };
    let vkGetPhysicalDeviceProperties: vkGetPhysicalDeviceProperties_t =
      unsafe { core::mem::transmute(f) };
    //
    let mut properties = VkPhysicalDeviceProperties::default();
    unsafe { vkGetPhysicalDeviceProperties(physical_device, &mut properties) };
    properties
  }

  #[inline]
  pub fn get_physical_device_features(
    &self, physical_device: VkPhysicalDevice,
  ) -> VkPhysicalDeviceFeatures {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkGetPhysicalDeviceFeatures_NAME.as_ptr()) }) else {
      return VkPhysicalDeviceFeatures::default()
    };
    let vkGetPhysicalDeviceFeatures: vkGetPhysicalDeviceFeatures_t =
      unsafe { core::mem::transmute(f) };
    //
    let mut features = VkPhysicalDeviceFeatures::default();
    unsafe { vkGetPhysicalDeviceFeatures(physical_device, &mut features) };
    features
  }

  #[inline]
  pub fn get_physical_device_queue_family_properties(
    &self, physical_device: VkPhysicalDevice,
  ) -> Vec<VkQueueFamilyProperties> {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkGetPhysicalDeviceQueueFamilyProperties_NAME.as_ptr()) }) else {
      return Vec::new()
    };
    let vkGetPhysicalDeviceQueueFamilyProperties: vkGetPhysicalDeviceQueueFamilyProperties_t =
      unsafe { core::mem::transmute(f) };
    //
    let mut queue_family_prop_count: u32 = 0;
    unsafe {
      vkGetPhysicalDeviceQueueFamilyProperties(
        physical_device,
        &mut queue_family_prop_count,
        null_mut(),
      )
    };
    let mut buf = Vec::with_capacity(queue_family_prop_count.try_into().unwrap());
    unsafe {
      vkGetPhysicalDeviceQueueFamilyProperties(
        physical_device,
        &mut queue_family_prop_count,
        buf.as_mut_ptr(),
      )
    };
    unsafe { buf.set_len(queue_family_prop_count.try_into().unwrap()) }
    buf
  }

  #[inline]
  pub fn get_physical_device_extension_properties(
    &self, physical_device: VkPhysicalDevice, layer: Option<&ArrayZStr<VK_MAX_EXTENSION_NAME_SIZE>>,
  ) -> Result<Vec<VkExtensionProperties>, VkErrorCode> {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkEnumerateDeviceExtensionProperties_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkEnumerateDeviceExtensionProperties: vkEnumerateDeviceExtensionProperties_t =
      unsafe { core::mem::transmute(f) };
    //
    let layer_z: *const u8 = match layer {
      Some(l) => l.as_ptr(),
      None => null(),
    };
    //
    let mut property_count: u32 = 0;
    let count_ret = unsafe {
      vkEnumerateDeviceExtensionProperties(
        physical_device,
        layer_z,
        &mut property_count,
        null_mut(),
      )
    };
    if let Some(err_code) = count_ret.0 {
      return Err(err_code);
    }
    let mut buf = Vec::with_capacity(property_count.try_into().unwrap());
    let write_ret = unsafe {
      vkEnumerateDeviceExtensionProperties(
        physical_device,
        layer_z,
        &mut property_count,
        buf.as_mut_ptr(),
      )
    };
    if let Some(err_code) = write_ret.0 {
      Err(err_code)
    } else {
      unsafe { buf.set_len(property_count.try_into().unwrap()) };
      Ok(buf)
    }
  }

  #[inline]
  pub fn get_physical_device_surface_capabilities_khr(
    &self, physical_device: VkPhysicalDevice, surface: VkSurfaceKHR,
  ) -> Result<VkSurfaceCapabilitiesKHR, VkErrorCode> {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkGetPhysicalDeviceSurfaceCapabilitiesKHR_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkGetPhysicalDeviceSurfaceCapabilitiesKHR: vkGetPhysicalDeviceSurfaceCapabilitiesKHR_t =
      unsafe { core::mem::transmute(f) };
    //
    let mut capabilities = VkSurfaceCapabilitiesKHR::default();
    let get_ret = unsafe {
      vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, surface, &mut capabilities)
    };
    if let Some(err_code) = get_ret.0 {
      Err(err_code)
    } else {
      Ok(capabilities)
    }
  }

  #[inline]
  pub fn get_physical_device_surface_formats(
    &self, physical_device: VkPhysicalDevice, surface: VkSurfaceKHR,
  ) -> Result<Vec<VkSurfaceFormatKHR>, VkErrorCode> {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkGetPhysicalDeviceSurfaceFormatsKHR_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkGetPhysicalDeviceSurfaceFormatsKHR: vkGetPhysicalDeviceSurfaceFormatsKHR_t =
      unsafe { core::mem::transmute(f) };
    //
    let mut format_count: u32 = 0;
    let count_ret = unsafe {
      vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, surface, &mut format_count, null_mut())
    };
    if let Some(err_code) = count_ret.0 {
      return Err(err_code);
    }
    let mut buf = Vec::with_capacity(format_count.try_into().unwrap());
    let write_ret = unsafe {
      vkGetPhysicalDeviceSurfaceFormatsKHR(
        physical_device,
        surface,
        &mut format_count,
        buf.as_mut_ptr(),
      )
    };
    if let Some(err_code) = write_ret.0 {
      Err(err_code)
    } else {
      unsafe { buf.set_len(format_count.try_into().unwrap()) };
      Ok(buf)
    }
  }

  #[inline]
  pub fn get_physical_device_surface_present_modes(
    &self, physical_device: VkPhysicalDevice, surface: VkSurfaceKHR,
  ) -> Result<Vec<VkPresentModeKHR>, VkErrorCode> {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkGetPhysicalDeviceSurfacePresentModesKHR_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkGetPhysicalDeviceSurfacePresentModesKHR: vkGetPhysicalDeviceSurfacePresentModesKHR_t =
      unsafe { core::mem::transmute(f) };
    //
    let mut mode_count: u32 = 0;
    let count_ret = unsafe {
      vkGetPhysicalDeviceSurfacePresentModesKHR(
        physical_device,
        surface,
        &mut mode_count,
        null_mut(),
      )
    };
    if let Some(err_code) = count_ret.0 {
      return Err(err_code);
    }
    let mut buf = Vec::with_capacity(mode_count.try_into().unwrap());
    let write_ret = unsafe {
      vkGetPhysicalDeviceSurfacePresentModesKHR(
        physical_device,
        surface,
        &mut mode_count,
        buf.as_mut_ptr(),
      )
    };
    if let Some(err_code) = write_ret.0 {
      Err(err_code)
    } else {
      unsafe { buf.set_len(mode_count.try_into().unwrap()) };
      Ok(buf)
    }
  }

  #[inline]
  pub fn get_physical_device_image_format_properties(
    &self, physical_device: VkPhysicalDevice, format: VkFormat, ty: VkImageType,
    tiling: VkImageTiling, usage: VkImageUsageFlags, create: VkImageCreateFlags,
  ) -> Result<VkImageFormatProperties, VkErrorCode> {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkGetPhysicalDeviceImageFormatProperties_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkGetPhysicalDeviceImageFormatProperties: vkGetPhysicalDeviceImageFormatProperties_t =
      unsafe { core::mem::transmute(f) };
    //
    let mut properties = VkImageFormatProperties::default();
    let get_ret = unsafe {
      vkGetPhysicalDeviceImageFormatProperties(
        physical_device,
        format,
        ty,
        tiling,
        usage,
        create,
        &mut properties,
      )
    };
    if let Some(err_code) = get_ret.0 {
      Err(err_code)
    } else {
      Ok(properties)
    }
  }

  #[inline]
  pub fn create_device(
    &self, physical_device: VkPhysicalDevice, queue_family_indexes: &[u32],
    mut device_layers: Vec<String>, mut device_extensions: Vec<String>,
    features: VkPhysicalDeviceFeatures,
  ) -> Result<Device, VkErrorCode> {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkCreateDevice_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkCreateDevice: vkCreateDevice_t = unsafe { core::mem::transmute(f) };
    //
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkGetDeviceQueue_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkGetDeviceQueue: vkGetDeviceQueue_t = unsafe { core::mem::transmute(f) };
    //
    device_layers.iter_mut().for_each(|s| s.push('\0'));
    device_extensions.iter_mut().for_each(|s| s.push('\0'));
    let layers_z: Vec<*const u8> = device_layers.iter().map(|s| s.as_ptr()).collect();
    let extensions_z: Vec<*const u8> = device_extensions.iter().map(|s| s.as_ptr()).collect();
    let priorities = &[1.0];
    let queue_infos: Vec<VkDeviceQueueCreateInfo> = queue_family_indexes
      .iter()
      .map(|index| VkDeviceQueueCreateInfo {
        queue_family_index: *index,
        queue_count: priorities.len().try_into().unwrap(),
        queue_priorities: priorities.as_ptr(),
        ..VkDeviceQueueCreateInfo::default()
      })
      .collect();
    let device_create_info = VkDeviceCreateInfo {
      ty: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
      next: null(),
      flags: VkDeviceCreateFlags::default(),
      enabled_features: &features,
      enabled_layer_count: layers_z.len().try_into().unwrap(),
      enabled_layer_names: layers_z.as_ptr(),
      enabled_extension_count: extensions_z.len().try_into().unwrap(),
      enabled_extension_names: extensions_z.as_ptr(),
      queue_create_info_count: queue_infos.len().try_into().unwrap(),
      queue_create_infos: queue_infos.as_ptr(),
    };
    let mut vk_device = VkDevice::null();
    let create_ret =
      unsafe { vkCreateDevice(physical_device, &device_create_info, null(), &mut vk_device) };
    if let Some(err_code) = create_ret.0 {
      Err(err_code)
    } else {
      // Get the queues that go with our new device
      let mut queues = Vec::new();
      for queue_family_index in queue_family_indexes.iter().copied() {
        let mut queue = VkQueue::NULL;
        unsafe { vkGetDeviceQueue(vk_device, queue_family_index, 0, &mut queue) };
        queues.push((queue_family_index, queue));
      }
      //
      Device::try_from_primary_parts(Entry(self.entry.0), self.vk_instance, vk_device, queues)
    }
  }

  /// ## Safety
  /// You must not destroy this while it has any live children.
  #[inline]
  pub unsafe fn destroy_surface(&self, surface: VkSurfaceKHR) {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkDestroySurfaceKHR_NAME.as_ptr()) }) else {
      return;
    };
    let vkDestroySurfaceKHR: vkDestroySurfaceKHR_t = unsafe { core::mem::transmute(f) };
    //
    unsafe { vkDestroySurfaceKHR(self.vk_instance, surface, null()) }
  }

  /// ## Safety
  /// You must not destroy this while it has any live children.
  #[inline]
  pub unsafe fn destroy_instance(self) {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkDestroyInstance_NAME.as_ptr()) }) else {
      return;
    };
    let vkDestroyInstance: vkDestroyInstance_t = unsafe { core::mem::transmute(f) };
    //
    unsafe { vkDestroyInstance(self.vk_instance, null()) }
  }
}
