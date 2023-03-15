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
    let physical_device = self.vk_physical_device;
    let layer_name: *const u8 = null(); // device layers are deprecated
    'gather: loop {
      let mut count = 0_u32;
      let r = unsafe {
        vkEnumerateDeviceExtensionProperties(
          physical_device,
          layer_name,
          &mut count,
          null_mut(),
        )
      };
      match r {
        VK_SUCCESS => (),
        other => return Err(VkError::new(other.0).unwrap()),
      }
      let mut buf: Vec<VkExtensionProperties> =
        Vec::with_capacity(count.try_into().unwrap());
      let r = unsafe {
        vkEnumerateDeviceExtensionProperties(
          physical_device,
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
  #[allow(clippy::field_reassign_with_default)]
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
    let mut device_queue_create_info = VkDeviceQueueCreateInfo::default();
    device_queue_create_info.queue_family_index = queue_family_index;
    device_queue_create_info.queue_count = queue_priorities.len().try_into().unwrap();
    device_queue_create_info.queue_priorities = queue_priorities.as_ptr();
    //
    let mut vk_device_create_info = VkDeviceCreateInfo::default();
    vk_device_create_info.enabled_extension_count = extensions.len().try_into().unwrap();
    vk_device_create_info.enabled_extension_names = extensions.as_ptr().cast();
    if let Some(features_ref) = features {
      vk_device_create_info.enabled_features = features_ref;
    }
    vk_device_create_info.queue_create_info_count = 1;
    vk_device_create_info.queue_create_infos = &device_queue_create_info;
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
}
