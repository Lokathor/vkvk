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
  pub fn create_device(
    &self, physical_device: VkPhysicalDevice, queue_family_indexes: &[u32],
    mut layers: Vec<String>, mut extensions: Vec<String>, features: VkPhysicalDeviceFeatures,
  ) -> Result<Device, VkErrorCode> {
    let vkGetInstanceProcAddr = self.entry.0;
    let Some(f) =  (unsafe { vkGetInstanceProcAddr(self.vk_instance, vkCreateDevice_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkCreateDevice: vkCreateDevice_t = unsafe { core::mem::transmute(f) };
    //
    layers.iter_mut().for_each(|s| s.push('\0'));
    extensions.iter_mut().for_each(|s| s.push('\0'));
    let layers_z: Vec<*const u8> = layers.iter().map(|s| s.as_ptr()).collect();
    let extensions_z: Vec<*const u8> = extensions.iter().map(|s| s.as_ptr()).collect();
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
      Ok(Device { entry: Entry(self.entry.0), vk_instance: self.vk_instance, vk_device })
    }
  }
}

// TODO: vkEnumerateDeviceExtensionProperties
