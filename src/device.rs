#![allow(non_snake_case)]

//! Provides the [Device] type, and related helper types.

use std::sync::{RwLock, RwLockWriteGuard};

use crate::prelude::*;

/// This destroys the held VkDevice when dropped.
///
/// This is intended to be wrapped in an `Arc` so that as many objects can keep
/// their parent device alive as necessary, and then the device won't get
/// destroyed until all children are gone.
pub(crate) struct DestroyDeviceOnDrop {
  pub(crate) vk_device: RwLock<VkDevice>,
  pub(crate) vk_queue: RwLock<VkQueue>,
  pub(crate) fns: Arc<DeviceFns>,
  pub(crate) _parent: Arc<DestroyInstanceOnDrop>,
}
impl Drop for DestroyDeviceOnDrop {
  #[inline]
  fn drop(&mut self) {
    let vkDeviceWaitIdle = self.fns.DeviceWaitIdle.unwrap();
    let vkDestroyDevice = self.fns.DestroyDevice.unwrap();
    //
    let vk_device: RwLockWriteGuard<_> = self.vk_device.write().unwrap();
    let _vk_queue: RwLockWriteGuard<_> = self.vk_queue.write().unwrap();
    // Safety: The device and all its queues must be externally synch'd
    unsafe {
      if vkDeviceWaitIdle(*vk_device) == VK_SUCCESS {
        vkDestroyDevice(*vk_device, null())
      } else {
        // do we want to do anything if we couldn't wait on the device?
      }
    }
  }
}

/// The Device is how you actually "do stuff" with Vulkan.
pub struct Device(pub(crate) Arc<DestroyDeviceOnDrop>);
