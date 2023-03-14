//! Provides the [Instance] type, and related helper types.

#![allow(non_snake_case)]

use crate::prelude::*;

/// This destroys the held VkInstance when dropped.
///
/// This is intended to be wrapped in an `Arc` so that as many objects can keep
/// their parent instance alive as necessary, and then the instance won't get
/// destroyed until all children are gone.
pub(crate) struct DestroyInstanceOnDrop {
  pub(crate) vk_instance: VkInstance,
  pub(crate) fns: Arc<InstanceFns>,
}
impl Drop for DestroyInstanceOnDrop {
  fn drop(&mut self) {
    if let Some(f) = self.fns.DestroyInstance {
      unsafe { f(self.vk_instance, null()) }
    }
  }
}

pub struct Instance(pub(crate) Arc<DestroyInstanceOnDrop>);
impl core::fmt::Debug for Instance {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    core::fmt::Debug::fmt(&self.0.vk_instance, f)
  }
}
impl Instance {
  /// Get handles for all available [PhysicalDevice] values.
  #[inline]
  pub fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>, VkError> {
    let Some(vkEnumeratePhysicalDevices) = self.0.fns.EnumeratePhysicalDevices else {
      let err_code = VkError::new(VK_ERROR_UNKNOWN.0).unwrap();
      return Err(err_code);
    };
    let instance = self.0.vk_instance;
    'gather: loop {
      let mut count = 0_u32;
      let r = unsafe { vkEnumeratePhysicalDevices(instance, &mut count, null_mut()) };
      match r {
        VK_SUCCESS => (),
        other => return Err(VkError::new(other.0).unwrap()),
      }
      let mut buf: Vec<VkPhysicalDevice> = Vec::with_capacity(count.try_into().unwrap());
      let r =
        unsafe { vkEnumeratePhysicalDevices(instance, &mut count, buf.as_mut_ptr()) };
      match r {
        VK_SUCCESS => {
          unsafe { buf.set_len(count.try_into().unwrap()) };
          return Ok(
            buf
              .into_iter()
              .map(|vk_physical_device| PhysicalDevice {
                vk_physical_device,
                parent: self.0.clone(),
              })
              .collect(),
          );
        }
        VK_INCOMPLETE => continue 'gather,
        other => return Err(VkError::new(other.0).unwrap()),
      }
    }
  }
}
