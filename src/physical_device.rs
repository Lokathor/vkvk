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
