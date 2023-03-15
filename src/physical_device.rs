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

  /// Gets the possible extensions for Devices created from this Physical
  /// Device.
  #[inline]
  pub fn enumerate_device_extension_properties(
    &self,
  ) -> Result<Vec<VkExtensionProperties>, VkError> {
    let Some(vkEnumerateDeviceExtensionProperties) = self.parent.fns.EnumerateDeviceExtensionProperties else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let vk_physical_device = self.vk_physical_device;
    let layer_name: *const u8 = null(); // device layers are deprecated
    'gather: loop {
      let mut count = 0_u32;
      let r = unsafe {
        vkEnumerateDeviceExtensionProperties(
          vk_physical_device,
          layer_name,
          &mut count,
          null_mut(),
        )
      };
      if r != VK_SUCCESS {
        return Err(NonZeroI32::new(r.0).unwrap());
      }
      let mut buf: Vec<VkExtensionProperties> =
        Vec::with_capacity(count.try_into().unwrap());
      let r = unsafe {
        vkEnumerateDeviceExtensionProperties(
          vk_physical_device,
          layer_name,
          &mut count,
          buf.as_mut_ptr(),
        )
      };
      match r {
        VK_SUCCESS => {
          unsafe { buf.set_len(count.try_into().unwrap()) };
          return Ok(buf);
        }
        VK_INCOMPLETE => continue 'gather,
        other => return Err(VkError::new(other.0).unwrap()),
      }
    }
  }

  /// Obtains the features available with this physical device.
  ///
  /// Physical device features that you wish to use with a Device must be
  /// enabled during that device's creation.
  ///
  /// * Khronos Fn: [vkGetPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html)
  /// * Khronos Struct: [VkPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html)
  #[inline]
  pub fn get_features(&self) -> VkPhysicalDeviceFeatures {
    let mut physical_device_features = VkPhysicalDeviceFeatures::default();
    let Some(vkGetPhysicalDeviceFeatures) = self.parent.fns.GetPhysicalDeviceFeatures else {
      return physical_device_features;
    };
    let physical_device = self.vk_physical_device;
    unsafe {
      vkGetPhysicalDeviceFeatures(physical_device, &mut physical_device_features)
    };
    physical_device_features
  }

  /// Creates a [Device] from this Physical Device.
  ///
  /// This automatically determines the queue family to use. When
  /// `needs_graphics` is set a graphical queue family will be selected. The
  /// number of queues in that family will be based on the number of
  /// `queue_priorities` you pass. If no queue family can satisfy your requested
  /// combination this method will return `VK_ERROR_UNKNOWN` without actually
  /// attempting device creation.
  ///
  /// ## Panics
  /// * All `queue_priorities` values must be within `0.0` to `1.0` (inclusive).
  #[inline]
  pub fn create_device(
    &self, extensions: &[ZStr<'_>], features: Option<&VkPhysicalDeviceFeatures>,
    needs_graphics: bool, queue_priorities: &[f32],
  ) -> Result<Device, VkError> {
    queue_priorities.iter().for_each(|f| assert!((0.0..=1.0).contains(f)));
    //
    let Some(vkCreateDevice) = self.parent.fns.CreateDevice else {
      return Err(NonZeroI32::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let Some(vkGetDeviceProcAddr) = self.parent.fns.GetInstanceProcAddr
      .and_then(|f|unsafe {f(self.parent.vk_instance, vkGetDeviceProcAddr_NAME.as_ptr())})
      .map(|f|unsafe {core::mem::transmute::<_,vkGetDeviceProcAddr_t>(f)}) else {
      return Err(NonZeroI32::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let queue_family_index: u32 = self
      .get_queue_family_properties()
      .iter()
      .position(|p| {
        (needs_graphics && p.queue_flags.graphics())
          && p.queue_count >= queue_priorities.len().try_into().unwrap()
      })
      .ok_or(NonZeroI32::new(VK_ERROR_UNKNOWN.0).unwrap())?
      .try_into()
      .unwrap();
    let device_queue_create_info = VkDeviceQueueCreateInfo {
      queue_family_index,
      queue_count: queue_priorities.len().try_into().unwrap(),
      queue_priorities: queue_priorities.as_ptr(),
      ..Default::default()
    };
    let vk_device_create_info = VkDeviceCreateInfo {
      enabled_extension_count: extensions.len().try_into().unwrap(),
      enabled_extension_names: extensions.as_ptr().cast(),
      enabled_features: features.map(|f| f as *const _).unwrap_or(null()),
      queue_create_info_count: 1,
      queue_create_infos: &device_queue_create_info,
      ..Default::default()
    };
    //
    let vk_physical_device = self.vk_physical_device;
    let mut vk_device = VkDevice::NULL;
    let r = unsafe {
      vkCreateDevice(vk_physical_device, &vk_device_create_info, null(), &mut vk_device)
    };
    if let Some(err_code) = NonZeroI32::new(r.0) {
      return Err(err_code);
    }
    let mut device_fns = DeviceFns::default();
    unsafe { device_fns.load(vk_device, vkGetDeviceProcAddr) };
    Ok(Device(Arc::new(DestroyDeviceOnDrop {
      vk_device,
      fns: Arc::new(device_fns),
      _parent: self.parent.clone(),
    })))
  }

  /// Gets surface formats
  ///
  /// ## Panics
  /// * The surface and the physical device must have the same parent instance.
  #[inline]
  pub fn get_surface_formats_khr(
    &self, surface: &Surface,
  ) -> Result<Vec<VkSurfaceFormatKHR>, VkError> {
    assert_eq!(surface.0.parent.vk_instance, self.parent.vk_instance);
    //
    let Some(vkGetPhysicalDeviceSurfaceFormatsKHR) = self.parent.fns.GetPhysicalDeviceSurfaceFormatsKHR else {
      return Err(VkError::new(VK_ERROR_EXTENSION_NOT_PRESENT.0).unwrap());
    };
    let vk_physical_device = self.vk_physical_device;
    let vk_surface_khr = surface.0.vk_surface_khr;
    'gather: loop {
      let mut count = 0_u32;
      let r = unsafe {
        vkGetPhysicalDeviceSurfaceFormatsKHR(
          vk_physical_device,
          vk_surface_khr,
          &mut count,
          null_mut(),
        )
      };
      if r != VK_SUCCESS {
        return Err(NonZeroI32::new(r.0).unwrap());
      }
      let mut buf: Vec<VkSurfaceFormatKHR> =
        Vec::with_capacity(count.try_into().unwrap());
      let r = unsafe {
        vkGetPhysicalDeviceSurfaceFormatsKHR(
          vk_physical_device,
          vk_surface_khr,
          &mut count,
          buf.as_mut_ptr(),
        )
      };
      match r {
        VK_SUCCESS => {
          unsafe { buf.set_len(count.try_into().unwrap()) };
          return Ok(buf);
        }
        VK_INCOMPLETE => continue 'gather,
        other => return Err(VkError::new(other.0).unwrap()),
      }
    }
  }

  /// ## Panics
  /// * The surface and the physical device must have the same parent instance.
  #[inline]
  pub fn get_surface_present_modes_khr(
    &self, surface: &Surface,
  ) -> Result<Vec<VkPresentModeKHR>, VkError> {
    assert_eq!(surface.0.parent.vk_instance, self.parent.vk_instance);
    //
    let Some(vkGetPhysicalDeviceSurfacePresentModesKHR) = self.parent.fns.GetPhysicalDeviceSurfacePresentModesKHR else {
      return Err(VkError::new(VK_ERROR_EXTENSION_NOT_PRESENT.0).unwrap());
    };
    let vk_physical_device = self.vk_physical_device;
    let vk_surface_khr = surface.0.vk_surface_khr;
    'gather: loop {
      let mut count = 0_u32;
      let r = unsafe {
        vkGetPhysicalDeviceSurfacePresentModesKHR(
          vk_physical_device,
          vk_surface_khr,
          &mut count,
          null_mut(),
        )
      };
      if r != VK_SUCCESS {
        return Err(NonZeroI32::new(r.0).unwrap());
      }
      let mut buf: Vec<VkPresentModeKHR> = Vec::with_capacity(count.try_into().unwrap());
      let r = unsafe {
        vkGetPhysicalDeviceSurfacePresentModesKHR(
          vk_physical_device,
          vk_surface_khr,
          &mut count,
          buf.as_mut_ptr(),
        )
      };
      match r {
        VK_SUCCESS => {
          unsafe { buf.set_len(count.try_into().unwrap()) };
          return Ok(buf);
        }
        VK_INCOMPLETE => continue 'gather,
        other => return Err(VkError::new(other.0).unwrap()),
      }
    }
  }

  /// ## Panics
  /// * The surface and the physical device must have the same parent instance.
  #[inline]
  pub fn get_surface_capabilities_khr(
    &self, surface: &Surface,
  ) -> Result<VkSurfaceCapabilitiesKHR, VkError> {
    assert_eq!(surface.0.parent.vk_instance, self.parent.vk_instance);
    //
    let Some(vkGetPhysicalDeviceSurfaceCapabilitiesKHR) = self.parent.fns.GetPhysicalDeviceSurfaceCapabilitiesKHR else {
      return Err(VkError::new(VK_ERROR_EXTENSION_NOT_PRESENT.0).unwrap());
    };
    let vk_physical_device = self.vk_physical_device;
    let vk_surface_khr = surface.0.vk_surface_khr;
    let mut surface_capabilities_khr = VkSurfaceCapabilitiesKHR::default();
    let r = unsafe {
      vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
        vk_physical_device,
        vk_surface_khr,
        &mut surface_capabilities_khr,
      )
    };
    if r != VK_SUCCESS {
      Err(NonZeroI32::new(r.0).unwrap())
    } else {
      Ok(surface_capabilities_khr)
    }
  }

  /// Gets the image format properties, given the other constraints.
  ///
  /// If the given combination of constraints isn't supported then this will
  /// return an error.
  #[inline]
  pub fn get_image_format_properties(
    &self, format: VkFormat, ty: VkImageType, tiling: VkImageTiling,
    usage: VkImageUsageFlags, flags: VkImageCreateFlags,
  ) -> Result<VkImageFormatProperties, VkError> {
    let Some(vkGetPhysicalDeviceImageFormatProperties) = self.parent.fns.GetPhysicalDeviceImageFormatProperties else {
      return Err(VkError::new(VK_ERROR_EXTENSION_NOT_PRESENT.0).unwrap());
    };
    let vk_physical_device = self.vk_physical_device;
    let mut image_format_properties = VkImageFormatProperties::default();
    let r = unsafe {
      vkGetPhysicalDeviceImageFormatProperties(
        vk_physical_device,
        format,
        ty,
        tiling,
        usage,
        flags,
        &mut image_format_properties,
      )
    };
    if r != VK_SUCCESS {
      Err(NonZeroI32::new(r.0).unwrap())
    } else {
      Ok(image_format_properties)
    }
  }
}
