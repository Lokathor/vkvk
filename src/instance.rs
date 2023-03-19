//! Provides the [Instance] type, and related helper types.

#![allow(non_snake_case)]

use std::sync::RwLock;

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
  #[inline]
  fn drop(&mut self) {
    if let Some(f) = self.fns.DestroyInstance {
      unsafe { f(self.vk_instance, null()) }
    }
  }
}

pub(crate) struct DestroySurfaceOnDrop {
  pub(crate) vk_surface_khr: RwLock<VkSurfaceKHR>,
  pub(crate) parent: Arc<DestroyInstanceOnDrop>,
}
impl Drop for DestroySurfaceOnDrop {
  #[inline]
  fn drop(&mut self) {
    if let Some(f) = self.parent.fns.DestroySurfaceKHR {
      let vk_surface_khr = self.vk_surface_khr.write().unwrap();
      unsafe { f(self.parent.vk_instance, *vk_surface_khr, null()) }
    }
  }
}

/// A Surface is the backing data for Vulkan to support a particular window.
pub struct Surface(pub(crate) Arc<DestroySurfaceOnDrop>);

/// Reprisents an open connection to a Vulkan API with a particular set of
/// layers and extensions enabled.
///
/// This doesn't do much itself. Generally, you call
/// [`enumerate_physical_devices`](Self::enumerate_physical_devices) and then
/// you can go through the physical devices for more information on what's
/// available on this system.
pub struct Instance(pub(crate) Arc<DestroyInstanceOnDrop>);
impl core::fmt::Debug for Instance {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    core::fmt::Debug::fmt(&self.0.vk_instance, f)
  }
}
impl Instance {
  /// Gets the [VkInstance] that this is wrapping.
  ///
  /// This is of use if you want to make `vkvk` interact with other libraries
  /// that also use Vulkan. The `vkvk` crate expects to be the one that
  /// destroys the instance, so you should not destroy the instance yourself.
  #[inline]
  #[must_use]
  pub fn vk_instance(&self) -> VkInstance {
    self.0.vk_instance
  }

  /// Makes a raw surface handle be wrapped as a child of this instance.
  ///
  /// ## Safety
  /// * The surface must have been made off of this instance and with no
  ///   allocation callback.
  #[inline]
  pub unsafe fn make_raw_surface_a_child_of_this(
    &self, vk_surface_khr: VkSurfaceKHR,
  ) -> Surface {
    Surface(Arc::new(DestroySurfaceOnDrop {
      vk_surface_khr: RwLock::new(vk_surface_khr),
      parent: self.0.clone(),
    }))
  }

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
      if r != VK_SUCCESS {
        return Err(NonZeroI32::new(r.0).unwrap());
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
