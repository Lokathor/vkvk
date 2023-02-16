#![allow(non_snake_case)]

use super::*;

pub struct Device {
  pub(crate) entry: Entry,
  pub(crate) vk_instance: VkInstance,
  pub(crate) vk_device: VkDevice,
  pub(crate) queues: Vec<(u32, VkQueue)>,
  //
  pub(crate) vkGetDeviceProcAddr: vkGetDeviceProcAddr_t,
  pub(crate) vkCreateImageView: vkCreateImageView_t,
  pub(crate) vkDestroyImageView: vkDestroyImageView_t,
  pub(crate) opt_vkCreateSwapchainKHR: Option<vkCreateSwapchainKHR_t>,
  pub(crate) opt_vkDestroySwapchainKHR: Option<vkDestroySwapchainKHR_t>,
  pub(crate) opt_vkGetSwapchainImagesKHR: Option<vkGetSwapchainImagesKHR_t>,
}

impl Device {
  #[inline]
  #[must_use]
  pub const fn vk_device(&self) -> VkDevice {
    self.vk_device
  }
  /// A slice of `(queue_family_index, VkQueue)` pairs
  #[inline]
  #[must_use]
  pub fn queues(&self) -> &[(u32, VkQueue)] {
    &self.queues
  }
  #[inline]
  pub(crate) fn try_from_primary_parts(
    entry: Entry, vk_instance: VkInstance, vk_device: VkDevice, queues: Vec<(u32, VkQueue)>,
  ) -> Result<Self, VkErrorCode> {
    let vkGetInstanceProcAddr = entry.0;
    //
    let Some(f) = (unsafe { vkGetInstanceProcAddr(vk_instance, vkGetDeviceProcAddr_NAME.as_ptr()) }) else {
        // TODO: cleanup the device that exists before returning!
        return Err(VkErrorCode::ERROR_UNKNOWN);
      };
    let vkGetDeviceProcAddr: vkGetDeviceProcAddr_t = unsafe { core::mem::transmute(f) };
    //
    let Some(f) = (unsafe { vkGetDeviceProcAddr(vk_device, vkCreateImageView_NAME.as_ptr()) }) else {
      // TODO: cleanup the device that exists before returning!
      return Err(VkErrorCode::ERROR_UNKNOWN);
    };
    let vkCreateImageView: vkCreateImageView_t = unsafe { core::mem::transmute(f) };
    let Some(f) = (unsafe { vkGetDeviceProcAddr(vk_device, vkDestroyImageView_NAME.as_ptr()) }) else {
      // TODO: cleanup the device that exists before returning!
      return Err(VkErrorCode::ERROR_UNKNOWN);
    };
    let vkDestroyImageView: vkDestroyImageView_t = unsafe { core::mem::transmute(f) };
    //
    let opt_vkCreateSwapchainKHR: Option<vkCreateSwapchainKHR_t> = unsafe {
      core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkCreateSwapchainKHR_NAME.as_ptr()))
    };
    let opt_vkDestroySwapchainKHR: Option<vkDestroySwapchainKHR_t> = unsafe {
      core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkDestroySwapchainKHR_NAME.as_ptr()))
    };
    let opt_vkGetSwapchainImagesKHR: Option<vkGetSwapchainImagesKHR_t> = unsafe {
      core::mem::transmute(vkGetDeviceProcAddr(vk_device, vkGetSwapchainImagesKHR_NAME.as_ptr()))
    };
    //
    Ok(Self {
      entry,
      queues,
      vk_device,
      vk_instance,
      vkGetDeviceProcAddr,
      vkCreateImageView,
      vkDestroyImageView,
      opt_vkCreateSwapchainKHR,
      opt_vkDestroySwapchainKHR,
      opt_vkGetSwapchainImagesKHR,
    })
  }

  #[inline]
  pub fn create_swapchain(
    &self, surface: VkSurfaceKHR, surface_format: VkSurfaceFormatKHR, image_extent: VkExtent2D,
    present_mode: VkPresentModeKHR, min_image_count: u32, image_usage: VkImageUsageFlags,
  ) -> Result<VkSwapchainKHR, VkErrorCode> {
    let Some(vkCreateSwapchainKHR) = self.opt_vkCreateSwapchainKHR else {
      return Err(VkErrorCode::ERROR_EXTENSION_NOT_PRESENT);
    };
    let swapchain_create_info = VkSwapchainCreateInfoKHR {
      ty: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
      next: null(),
      flags: VkSwapchainCreateFlagsKHR::default(),
      surface,
      min_image_count,
      image_format: surface_format.format,
      image_color_space: surface_format.color_space,
      image_extent,
      image_array_layers: 1,
      image_usage,
      image_sharing_mode: VK_SHARING_MODE_EXCLUSIVE,
      queue_family_index_count: 0,
      queue_family_indices: null(),
      pre_transform: VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
      composite_alpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
      present_mode,
      clipped: true.into(),
      old_swapchain: VkSwapchainKHR::zero(),
    };
    let mut vk_swapchain = VkSwapchainKHR::zero();
    let create_ret = unsafe {
      vkCreateSwapchainKHR(self.vk_device, &swapchain_create_info, null(), &mut vk_swapchain)
    };
    if let Some(err_code) = create_ret.0 {
      Err(err_code)
    } else {
      Ok(vk_swapchain)
    }
  }

  #[inline]
  pub fn get_swapchain_images(
    &self, swapchain: VkSwapchainKHR,
  ) -> Result<Vec<VkImage>, VkErrorCode> {
    let Some(vkGetSwapchainImagesKHR) = self.opt_vkGetSwapchainImagesKHR else {
      return Err(VkErrorCode::ERROR_EXTENSION_NOT_PRESENT)
    };
    //
    let mut image_count: u32 = 0;
    let count_ret =
      unsafe { vkGetSwapchainImagesKHR(self.vk_device, swapchain, &mut image_count, null_mut()) };
    if let Some(err_code) = count_ret.0 {
      return Err(err_code);
    }
    let mut buf = Vec::with_capacity(image_count.try_into().unwrap());
    let write_ret = unsafe {
      vkGetSwapchainImagesKHR(self.vk_device, swapchain, &mut image_count, buf.as_mut_ptr())
    };
    if let Some(err_code) = write_ret.0 {
      Err(err_code)
    } else {
      unsafe { buf.set_len(image_count.try_into().unwrap()) };
      Ok(buf)
    }
  }

  #[inline]
  pub fn create_image_view(
    &self, image: VkImage, flags: VkImageViewCreateFlags, view_type: VkImageViewType,
    format: VkFormat, components: VkComponentMapping, subresource_range: VkImageSubresourceRange,
  ) -> Result<VkImageView, VkErrorCode> {
    let info = VkImageViewCreateInfo {
      image,
      view_type,
      flags,
      format,
      components,
      subresource_range,
      ..VkImageViewCreateInfo::default()
    };
    let mut image_view = VkImageView::zero();
    let create_ret =
      unsafe { (self.vkCreateImageView)(self.vk_device, &info, null(), &mut image_view) };
    if let Some(err) = create_ret.0 {
      Err(err)
    } else {
      Ok(image_view)
    }
  }

  /// ## Safety
  /// * Each [VkImageView] can only be destroyed once.
  #[inline]
  pub unsafe fn destroy_image_view(&self, image_view: VkImageView) {
    unsafe { (self.vkDestroyImageView)(self.vk_device, image_view, null()) }
  }

  /// ## Safety
  /// * You must not destroy this while it has any live children.
  /// * You must not destroy a swapchain more than once.
  #[inline]
  pub unsafe fn destroy_swapchain(&self, swapchain: VkSwapchainKHR) {
    let Some(vkDestroySwapchainKHR) = self.opt_vkDestroySwapchainKHR else {
      return;
    };
    unsafe { vkDestroySwapchainKHR(self.vk_device, swapchain, null()) }
  }

  /// ## Safety
  /// * You must not destroy this while it has any live children.
  #[inline]
  pub unsafe fn destroy_device(self) {
    let vkGetDeviceProcAddr = self.vkGetDeviceProcAddr;
    let Some(f) = (unsafe { vkGetDeviceProcAddr(self.vk_device, vkDestroyDevice_NAME.as_ptr()) }) else {
      return;
    };
    let vkDestroyDevice: vkDestroyDevice_t = unsafe { core::mem::transmute(f) };
    //
    unsafe { vkDestroyDevice(self.vk_device, null()) }
  }
}
