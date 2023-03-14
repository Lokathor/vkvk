#![allow(non_snake_case)]

//! Provides the [PhysicalDevice] type, and related helper types.

use crate::prelude::*;

/// A physical device can potentially be opened as a `Device`.
///
/// The physical device has to outlive its parent instance. This is done
/// automatically via interior `Arc` values.
pub struct PhysicalDevice {
  pub(crate) vk_physical_device: VkPhysicalDevice,
  pub(crate) parent: Arc<DestroyInstanceOnDrop>,
}
impl core::fmt::Debug for PhysicalDevice {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    core::fmt::Debug::fmt(&self.vk_physical_device, f)
  }
}
impl PhysicalDevice {
  /// Gets the properties of the queue families in this physical device.
  #[inline]
  pub fn get_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties> {
    let Some(vkGetPhysicalDeviceQueueFamilyProperties) = self.parent.fns.GetPhysicalDeviceQueueFamilyProperties else {
      return Vec::new();
    };
    let physical_device = self.vk_physical_device;
    let mut count = 0_u32;
    unsafe {
      vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut count, null_mut())
    };
    let mut buf: Vec<VkQueueFamilyProperties> =
      Vec::with_capacity(count.try_into().unwrap());
    unsafe {
      vkGetPhysicalDeviceQueueFamilyProperties(
        physical_device,
        &mut count,
        buf.as_mut_ptr(),
      )
    };
    unsafe { buf.set_len(count.try_into().unwrap()) };
    buf
  }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct DeviceCreateInfo {
  struct_ty: VkStructureType,
  next: *const c_void,
  flags: VkDeviceCreateFlags,
  queue_create_info_count: u32,
  queue_create_infos: *mut VkDeviceQueueCreateInfo,
  _enabled_layer_count: u32,
  _enabled_layer_names: *const *const u8,
  enabled_extension_count: u32,
  enabled_extension_names: *mut ZString,
  pub enabled_features: Option<Box<VkPhysicalDeviceFeatures>>,
  //
  queue_create_info_capacity: u32,
  enabled_extension_capacity: u32,
}
impl Drop for DeviceCreateInfo {
  #[inline]
  fn drop(&mut self) {
    drop(unsafe {
      Vec::from_raw_parts(
        self.queue_create_infos,
        self.queue_create_info_count.try_into().unwrap(),
        self.queue_create_info_capacity.try_into().unwrap(),
      )
    });
    drop(unsafe {
      Vec::from_raw_parts(
        self.enabled_extension_names,
        self.enabled_extension_count.try_into().unwrap(),
        self.enabled_extension_capacity.try_into().unwrap(),
      )
    });
  }
}
impl Default for DeviceCreateInfo {
  #[inline]
  #[must_use]
  fn default() -> Self {
    let mut queue_create_info = ManuallyDrop::new(Vec::new());
    let mut enabled_extension = ManuallyDrop::new(Vec::new());
    Self {
      struct_ty: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
      next: core::ptr::null(),
      flags: Default::default(),
      queue_create_info_count: queue_create_info.len().try_into().unwrap(),
      _enabled_layer_count: Default::default(),
      _enabled_layer_names: core::ptr::null(),
      enabled_extension_count: enabled_extension.len().try_into().unwrap(),
      enabled_features: None,
      queue_create_info_capacity: queue_create_info.capacity().try_into().unwrap(),
      enabled_extension_capacity: enabled_extension.capacity().try_into().unwrap(),
      queue_create_infos: queue_create_info.as_mut_ptr(),
      enabled_extension_names: enabled_extension.as_mut_ptr(),
    }
  }
}
impl DeviceCreateInfo {
  /// Runs a closure using the queue_create list.
  #[inline]
  pub fn queue_create_mut<F: FnOnce(&mut Vec<VkDeviceQueueCreateInfo>)>(
    &mut self, op: F,
  ) {
    fake_ptr_len_cap!(
      self.queue_create_infos,
      self.queue_create_info_count,
      self.queue_create_info_capacity,
      op
    );
  }

  /// Runs a closure using the extensions list.
  #[inline]
  pub fn extensions_mut<F: FnOnce(&mut Vec<ZString>)>(&mut self, op: F) {
    fake_ptr_len_cap!(
      self.enabled_extension_names,
      self.enabled_extension_count,
      self.enabled_extension_capacity,
      op
    );
  }

  /// View the names of the layers to use
  #[inline]
  #[must_use]
  pub fn layers(&self) -> &[VkDeviceQueueCreateInfo] {
    unsafe {
      core::slice::from_raw_parts(
        self.queue_create_infos,
        self.queue_create_info_count.try_into().unwrap(),
      )
    }
  }

  /// View the names of the extensions to use
  #[inline]
  #[must_use]
  pub fn extensions(&self) -> &[ZString] {
    unsafe {
      core::slice::from_raw_parts(
        self.enabled_extension_names,
        self.enabled_extension_count.try_into().unwrap(),
      )
    }
  }
}
