#![allow(non_snake_case)]

//! Provides the [Device] type, and related helper types.

use crate::prelude::*;

/// This destroys the held VkDevice when dropped.
///
/// This is intended to be wrapped in an `Arc` so that as many objects can keep
/// their parent device alive as necessary, and then the device won't get
/// destroyed until all children are gone.
pub(crate) struct DestroyDeviceOnDrop {
  pub(crate) vk_device: VkDevice,
  pub(crate) fns: Arc<DeviceFns>,
  pub(crate) _parent: Arc<DestroyInstanceOnDrop>,
}
impl Drop for DestroyDeviceOnDrop {
  #[inline]
  fn drop(&mut self) {
    unsafe { self.fns.DeviceWaitIdle.unwrap()(self.vk_device) };
    //
    if let Some(f) = self.fns.DestroyDevice {
      unsafe { f(self.vk_device, null()) }
    }
  }
}

/// The Device is how you actually "do stuff" with Vulkan.
pub struct Device(pub(crate) Arc<DestroyDeviceOnDrop>);
