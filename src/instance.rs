//! The instance module mostly has the [`Instance`] type.
//!
//! This is what lets you query info about the "physical devices" of the system,
//! and you can open one (or more) of them as a "device".

use crate::prelude::*;

/// The Instance type is a wrapper for a [`VkInstance`].
///
/// An instance is a specific "connection" to the Vulkan API, with a particular
/// set of layers and extensions enabled.
///
/// The main purpose of the instance is that it lets you check on the physical
/// devices available, and then use one of them to create a [`Device`]. The
/// `Device` you make does most of how you actually get Vulkan to "do stuff".
pub struct Instance {
  vk_instance: VkInstance,
  fns: InstanceFnTable,
  // Note(Lokathor): Extension function tables are only optionally present,
  // because even if they're configured into the build the extension might not be used by
  // a particular instance.
  #[cfg(feature = "VK_KHR_surface")]
  vk_khr_surface_fns: Option<InstanceFnTable_VkKhrSurface>,
}
impl Drop for Instance {
  fn drop(&mut self) {
    if cfg!(debug_assertions) {
      panic!("Bug: you shouldn't drop an `Instance`, please destroy it properly with `Instance::destroy`");
    }
  }
}
impl Instance {
  /// Gets the wrapped [`VkInstance`], in case you need need to interact with
  /// Vulkan via other libraries.
  #[inline]
  #[must_use]
  pub const fn vk_instance(&self) -> VkInstance {
    self.vk_instance
  }

  #[inline]
  pub(crate) unsafe fn new(
    vkGetInstanceProcAddr: vkGetInstanceProcAddr_t, vk_instance: VkInstance,
  ) -> Result<Self, NonZeroI32> {
    Ok(Self {
      vk_instance,
      fns: unsafe {
        InstanceFnTable::new(vkGetInstanceProcAddr, vk_instance)
          .ok_or(VK_ERROR_UNKNOWN.0.unwrap())?
      },
      #[cfg(feature = "VK_KHR_surface")]
      vk_khr_surface_fns: unsafe {
        InstanceFnTable_VkKhrSurface::new(vkGetInstanceProcAddr, vk_instance)
      },
    })
  }

  /// Destroys a surface
  ///
  /// ## Safety
  /// * This must be a valid surface value that came from this instance.
  /// * The surface must have been created without using any allocation
  ///   callbacks (which is the case for SDL surfaces, which is the main other
  ///   library that I want this library to work with).
  #[inline]
  pub unsafe fn destroy_surface(&self, surface: VkSurfaceKHR) -> Result<(), NonZeroI32> {
    if let Some(surface_fns) = self.vk_khr_surface_fns {
      Ok((surface_fns.vkDestroySurfaceKHR)(self.vk_instance, surface, null()))
    } else {
      Err(VK_ERROR_EXTENSION_NOT_PRESENT.0.unwrap())
    }
  }

  /// Destroy the instance.
  ///
  /// You should call this rather than letting the instance simply drop.
  #[inline]
  pub fn destroy(self) {
    unsafe { (self.fns.vkDestroyInstance)(self.vk_instance, null()) };
    core::mem::forget(self);
  }

  /// Gets a list of physical devices that are available.
  ///
  /// The physical devices have a lifetime back to this instance, but you do not
  /// need to explicitly drop them.
  #[inline]
  pub fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>, NonZeroI32> {
    let mut count = 0_u32;
    if let Some(err) = unsafe {
      (self.fns.vkEnumeratePhysicalDevices)(self.vk_instance, &mut count, null_mut()).0
    } {
      return Err(err);
    }
    let mut buf = Vec::with_capacity(count.try_into().unwrap());
    if let Some(err) = unsafe {
      (self.fns.vkEnumeratePhysicalDevices)(
        self.vk_instance,
        &mut count,
        buf.as_mut_ptr(),
      )
      .0
    } {
      Err(err)
    } else {
      unsafe { buf.set_len(count.try_into().unwrap()) };
      Ok(
        buf
          .into_iter()
          .map(|pd| PhysicalDevice { parent: self, vk_physical_device: pd })
          .collect(),
      )
    }
  }
}

#[derive(Clone, Copy)]
#[allow(bad_style)]
struct InstanceFnTable {
  vkDestroyInstance: vkDestroyInstance_t,
  vkEnumeratePhysicalDevices: vkEnumeratePhysicalDevices_t,
  vkGetInstanceProcAddr: vkGetInstanceProcAddr_t,
  vkCreateDevice: vkCreateDevice_t,
  vkEnumerateDeviceExtensionProperties: vkEnumerateDeviceExtensionProperties_t,
  vkEnumerateDeviceLayerProperties: vkEnumerateDeviceLayerProperties_t,
  vkGetPhysicalDeviceFeatures: vkGetPhysicalDeviceFeatures_t,
  vkGetPhysicalDeviceFormatProperties: vkGetPhysicalDeviceFormatProperties_t,
  vkGetPhysicalDeviceImageFormatProperties: vkGetPhysicalDeviceImageFormatProperties_t,
  vkGetPhysicalDeviceMemoryProperties: vkGetPhysicalDeviceMemoryProperties_t,
  vkGetPhysicalDeviceProperties: vkGetPhysicalDeviceProperties_t,
  vkGetPhysicalDeviceQueueFamilyProperties: vkGetPhysicalDeviceQueueFamilyProperties_t,
  vkGetPhysicalDeviceSparseImageFormatProperties:
    vkGetPhysicalDeviceSparseImageFormatProperties_t,
  vkGetDeviceProcAddr: vkGetDeviceProcAddr_t,
}
impl InstanceFnTable {
  #[rustfmt::skip]
  #[allow(bad_style)]
  pub(crate) unsafe fn new(
    vkGetInstanceProcAddr: vkGetInstanceProcAddr_t, vk_instance: VkInstance,
  ) -> Option<Self> {
    Some(Self{
      vkDestroyInstance: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkDestroyInstance_NAME.as_ptr())?),
      vkEnumeratePhysicalDevices: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkEnumeratePhysicalDevices_NAME.as_ptr())?),
      vkGetInstanceProcAddr: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetInstanceProcAddr_NAME.as_ptr())?),
      vkCreateDevice: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkCreateDevice_NAME.as_ptr())?),
      vkEnumerateDeviceExtensionProperties: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkEnumerateDeviceExtensionProperties_NAME.as_ptr())?),
      vkEnumerateDeviceLayerProperties: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkEnumerateDeviceLayerProperties_NAME.as_ptr())?),
      vkGetPhysicalDeviceFeatures: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceFeatures_NAME.as_ptr())?),
      vkGetPhysicalDeviceFormatProperties: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceFormatProperties_NAME.as_ptr())?),
      vkGetPhysicalDeviceImageFormatProperties: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceImageFormatProperties_NAME.as_ptr())?),
      vkGetPhysicalDeviceMemoryProperties: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceMemoryProperties_NAME.as_ptr())?),
      vkGetPhysicalDeviceProperties: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceProperties_NAME.as_ptr())?),
      vkGetPhysicalDeviceQueueFamilyProperties: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceQueueFamilyProperties_NAME.as_ptr())?),
      vkGetPhysicalDeviceSparseImageFormatProperties: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceSparseImageFormatProperties_NAME.as_ptr())?),
      vkGetDeviceProcAddr: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetDeviceProcAddr_NAME.as_ptr())?),
    })
  }
}

#[cfg(feature = "VK_KHR_surface")]
#[allow(bad_style)]
#[derive(Clone, Copy)]
struct InstanceFnTable_VkKhrSurface {
  vkDestroySurfaceKHR: vkDestroySurfaceKHR_t,
  vkGetPhysicalDeviceSurfaceSupportKHR: vkGetPhysicalDeviceSurfaceSupportKHR_t,
  vkGetPhysicalDeviceSurfaceCapabilitiesKHR: vkGetPhysicalDeviceSurfaceCapabilitiesKHR_t,
  vkGetPhysicalDeviceSurfaceFormatsKHR: vkGetPhysicalDeviceSurfaceFormatsKHR_t,
  vkGetPhysicalDeviceSurfacePresentModesKHR: vkGetPhysicalDeviceSurfacePresentModesKHR_t,
}
#[cfg(feature = "VK_KHR_surface")]
#[allow(bad_style)]
impl InstanceFnTable_VkKhrSurface {
  #[rustfmt::skip]
  pub(crate) unsafe fn new(
    vkGetInstanceProcAddr: vkGetInstanceProcAddr_t, vk_instance: VkInstance,
  ) -> Option<Self> {
    Some(Self{
      vkDestroySurfaceKHR: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkDestroySurfaceKHR_NAME.as_ptr())?),
      vkGetPhysicalDeviceSurfaceSupportKHR: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceSurfaceSupportKHR_NAME.as_ptr())?),
      vkGetPhysicalDeviceSurfaceCapabilitiesKHR: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceSurfaceCapabilitiesKHR_NAME.as_ptr())?),
      vkGetPhysicalDeviceSurfaceFormatsKHR: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceSurfaceFormatsKHR_NAME.as_ptr())?),
      vkGetPhysicalDeviceSurfacePresentModesKHR: core::mem::transmute(vkGetInstanceProcAddr(vk_instance, vkGetPhysicalDeviceSurfacePresentModesKHR_NAME.as_ptr())?),
    })
  }
}

/// This is a handle to one of the physical devices available on the system.
///
/// Physical devices have an immense number of properties and qualities. Opening
/// a device requires that your request pick many details. Even if there's only
/// a single physical device present you will likely need to query its
/// properties to be able to successfully open a device.
#[derive(Clone, Copy)]
pub struct PhysicalDevice<'i> {
  parent: &'i Instance,
  vk_physical_device: VkPhysicalDevice,
}
impl PhysicalDevice<'_> {
  /// Gets the [VkPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html)
  #[inline]
  pub fn get_features(&self) -> VkPhysicalDeviceFeatures {
    let mut features = VkPhysicalDeviceFeatures::default();
    unsafe {
      (self.parent.fns.vkGetPhysicalDeviceFeatures)(
        self.vk_physical_device,
        &mut features,
      )
    }
    features
  }

  /// Gets information about the queue families that are supported.
  #[inline]
  pub fn get_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties> {
    let mut count = 0_u32;
    unsafe {
      (self.parent.fns.vkGetPhysicalDeviceQueueFamilyProperties)(
        self.vk_physical_device,
        &mut count,
        null_mut(),
      )
    }
    let mut buf = Vec::with_capacity(count.try_into().unwrap());
    unsafe {
      (self.parent.fns.vkGetPhysicalDeviceQueueFamilyProperties)(
        self.vk_physical_device,
        &mut count,
        buf.as_mut_ptr(),
      )
    }
    unsafe { buf.set_len(count.try_into().unwrap()) };
    buf
  }

  /// Gets the [VkSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html)
  #[inline]
  #[cfg(feature = "VK_KHR_surface")]
  pub fn get_surface_capabilities_khr(
    &self, surface: VkSurfaceKHR,
  ) -> Result<VkSurfaceCapabilitiesKHR, NonZeroI32> {
    if let Some(surface_fns) = self.parent.vk_khr_surface_fns {
      let mut capabilities = VkSurfaceCapabilitiesKHR::default();
      let get_ret = unsafe {
        (surface_fns.vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(
          self.vk_physical_device,
          surface,
          &mut capabilities,
        )
      };
      if let Some(err_code) = get_ret.0 {
        Err(err_code)
      } else {
        Ok(capabilities)
      }
    } else {
      Err(VK_ERROR_EXTENSION_NOT_PRESENT.0.unwrap())
    }
  }

  /// Gets the possible [VkPresentModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html)
  #[inline]
  #[cfg(feature = "VK_KHR_surface")]
  pub fn get_surface_present_modes_khr(
    &self, surface: VkSurfaceKHR,
  ) -> Result<Vec<VkPresentModeKHR>, NonZeroI32> {
    if let Some(surface_fns) = self.parent.vk_khr_surface_fns {
      let mut count = 0_u32;
      if let Some(err) = unsafe {
        (surface_fns.vkGetPhysicalDeviceSurfacePresentModesKHR)(
          self.vk_physical_device,
          surface,
          &mut count,
          null_mut(),
        )
        .0
      } {
        return Err(err);
      }
      let mut buf = Vec::with_capacity(count.try_into().unwrap());
      if let Some(err) = unsafe {
        (surface_fns.vkGetPhysicalDeviceSurfacePresentModesKHR)(
          self.vk_physical_device,
          surface,
          &mut count,
          buf.as_mut_ptr(),
        )
        .0
      } {
        Err(err)
      } else {
        unsafe { buf.set_len(count.try_into().unwrap()) };
        Ok(buf)
      }
    } else {
      Err(VK_ERROR_EXTENSION_NOT_PRESENT.0.unwrap())
    }
  }

  /// Gets the possible [VkSurfaceFormatKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html)
  #[inline]
  #[cfg(feature = "VK_KHR_surface")]
  pub fn get_surface_formats_khr(
    &self, surface: VkSurfaceKHR,
  ) -> Result<Vec<VkSurfaceFormatKHR>, NonZeroI32> {
    if let Some(surface_fns) = self.parent.vk_khr_surface_fns {
      let mut count = 0_u32;
      if let Some(err) = unsafe {
        (surface_fns.vkGetPhysicalDeviceSurfaceFormatsKHR)(
          self.vk_physical_device,
          surface,
          &mut count,
          null_mut(),
        )
        .0
      } {
        return Err(err);
      }
      let mut buf = Vec::with_capacity(count.try_into().unwrap());
      if let Some(err) = unsafe {
        (surface_fns.vkGetPhysicalDeviceSurfaceFormatsKHR)(
          self.vk_physical_device,
          surface,
          &mut count,
          buf.as_mut_ptr(),
        )
        .0
      } {
        Err(err)
      } else {
        unsafe { buf.set_len(count.try_into().unwrap()) };
        Ok(buf)
      }
    } else {
      Err(VK_ERROR_EXTENSION_NOT_PRESENT.0.unwrap())
    }
  }

  /// Opens a specific connection to this physical device, creating a "device".
  ///
  /// This is a simplified version of the full device creation system: You
  /// specify just one queue family's index, and then you get just one queue in
  /// that queue family.
  #[inline]
  pub fn create_device(
    &self, queue_family_index: u32, device_extensions: &[ZStr<'_>],
    enabled_features: Option<&VkPhysicalDeviceFeatures>,
  ) -> Result<Device, NonZeroI32> {
    let priorities = &[1.0];
    let device_queue_create_info = VkDeviceQueueCreateInfo {
      queue_family_index,
      queue_count: priorities.len().try_into().unwrap(),
      queue_priorities: priorities.as_ptr(),
      ..VkDeviceQueueCreateInfo::default()
    };
    let enabled_features =
      enabled_features.map(|f| f as *const VkPhysicalDeviceFeatures).unwrap_or(null());
    let info = VkDeviceCreateInfo {
      queue_create_info_count: 1,
      queue_create_infos: &device_queue_create_info,
      enabled_extension_count: device_extensions.len().try_into().unwrap(),
      enabled_extension_names: device_extensions.as_ptr().cast(),
      enabled_features,
      ..VkDeviceCreateInfo::default()
    };
    let mut vk_device = VkDevice::NULL;
    if let Some(err) = unsafe {
      (self.parent.fns.vkCreateDevice)(
        self.vk_physical_device,
        &info,
        null(),
        &mut vk_device,
      )
      .0
    } {
      Err(err)
    } else {
      unsafe { Device::new(self.parent.fns.vkGetDeviceProcAddr, vk_device) }
    }
  }
}
