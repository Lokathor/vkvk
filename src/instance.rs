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
  /// Destroy the instance.
  ///
  /// You should call this rather than letting the instance simply drop.
  pub fn destroy(self) {
    unsafe { (self.fns.vkDestroyInstance)(self.vk_instance, null()) };
    core::mem::forget(self);
  }
}
impl Instance {
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
