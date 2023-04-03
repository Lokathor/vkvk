#![allow(non_snake_case)]

//! Provides the [Device] type, and related helper types.

use core::mem::size_of;
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
  pub(crate) physical_device: PhysicalDevice,
  pub(crate) _instance: Instance,
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
impl Device {
  /// Creates a new [Swapchain].
  #[inline]
  pub fn create_swapchain_khr(&self, surface: &Surface) -> Result<Swapchain, VkError> {
    let Some(vkCreateImageView) = self.0.fns.CreateImageView else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let Some(vkCreateSwapchainKHR) = self.0.fns.CreateSwapchainKHR else {
      return Err(VkError::new(VK_ERROR_EXTENSION_NOT_PRESENT.0).unwrap());
    };
    let Some(vkGetSwapchainImagesKHR) = self.0.fns.GetSwapchainImagesKHR else {
      return Err(VkError::new(VK_ERROR_EXTENSION_NOT_PRESENT.0).unwrap());
    };
    //
    let image_usage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
    let surface_format = self
      .0
      .physical_device
      .get_surface_formats_khr(surface)
      .unwrap()
      .into_iter()
      .find(|sf| {
        sf.color_space == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
          && [VK_FORMAT_R8G8B8A8_SRGB, VK_FORMAT_B8G8R8A8_SRGB].contains(&sf.format)
      })
      .ok_or(VkError::new(VK_ERROR_INITIALIZATION_FAILED.0).unwrap())?;
    let surface_capabilities =
      self.0.physical_device.get_surface_capabilities_khr(surface).unwrap();
    let image_extent = surface_capabilities.current_extent;
    let image_format = surface_format.format;
    let (present_mode, min_image_count) = {
      let surface_present_modes =
        self.0.physical_device.get_surface_present_modes_khr(surface).unwrap();
      if surface_present_modes.contains(&VK_PRESENT_MODE_MAILBOX_KHR) {
        let min = surface_capabilities.min_image_count;
        let max =
          surface_capabilities.max_image_count.map(NonZeroU32::get).unwrap_or(u32::MAX);
        let count = 3_u32.clamp(min, max);
        (VK_PRESENT_MODE_MAILBOX_KHR, count)
      } else if let Some(mode) = surface_present_modes.get(0).copied() {
        (mode, surface_capabilities.min_image_count)
      } else {
        return Err(VkError::new(VK_ERROR_INITIALIZATION_FAILED.0).unwrap());
      }
    };
    let vk_surface_khr = surface.0.vk_surface_khr.write().unwrap();
    let swapchain_create_info_khr = VkSwapchainCreateInfoKHR {
      surface: *vk_surface_khr,
      min_image_count,
      image_format,
      image_color_space: surface_format.color_space,
      image_extent,
      image_array_layers: 1,
      image_usage,
      image_sharing_mode: VK_SHARING_MODE_EXCLUSIVE,
      present_mode,
      pre_transform: surface_capabilities.current_transform,
      composite_alpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
      ..Default::default()
    };
    let vk_device = self.0.vk_device.read().unwrap();
    let mut vk_swapchain_khr = VkSwapchainKHR::NULL;
    let r = unsafe {
      vkCreateSwapchainKHR(
        *vk_device,
        &swapchain_create_info_khr,
        null(),
        &mut vk_swapchain_khr,
      )
    };
    if r != VK_SUCCESS {
      return Err(NonZeroI32::new(r.0).unwrap());
    }

    let mut images = Vec::new();
    let mut image_views = Vec::new();

    'gather_images: loop {
      let mut count = 0_u32;
      let r = unsafe {
        vkGetSwapchainImagesKHR(*vk_device, vk_swapchain_khr, &mut count, null_mut())
      };
      if r != VK_SUCCESS {
        drop(DestroySwapchainOnDrop {
          vk_swapchain_khr,
          surface_format,
          image_extent,
          _images: images,
          image_views,
          min_image_count,
          present_mode,
          _surface: surface.0.clone(),
          device: self.0.clone(),
        });
        return Err(NonZeroI32::new(r.0).unwrap());
      }
      images = Vec::with_capacity(count.try_into().unwrap());
      let r = unsafe {
        vkGetSwapchainImagesKHR(
          *vk_device,
          vk_swapchain_khr,
          &mut count,
          images.as_mut_ptr(),
        )
      };
      match r {
        VK_SUCCESS => {
          unsafe { images.set_len(count.try_into().unwrap()) };
          break 'gather_images;
        }
        VK_INCOMPLETE => continue 'gather_images,
        other => {
          drop(DestroySwapchainOnDrop {
            vk_swapchain_khr,
            surface_format,
            image_extent,
            _images: images,
            image_views,
            min_image_count,
            present_mode,
            _surface: surface.0.clone(),
            device: self.0.clone(),
          });
          return Err(VkError::new(other.0).unwrap());
        }
      }
    }

    for image in images.iter() {
      let info = VkImageViewCreateInfo {
        image: *image,
        view_type: VK_IMAGE_VIEW_TYPE_2D,
        format: image_format,
        components: VkComponentMapping {
          r: VK_COMPONENT_SWIZZLE_IDENTITY,
          g: VK_COMPONENT_SWIZZLE_IDENTITY,
          b: VK_COMPONENT_SWIZZLE_IDENTITY,
          a: VK_COMPONENT_SWIZZLE_IDENTITY,
        },
        subresource_range: VkImageSubresourceRange {
          aspect_mask: VK_IMAGE_ASPECT_COLOR_BIT,
          base_mip_level: 0,
          level_count: 1,
          base_array_layer: 0,
          layer_count: 1,
        },
        ..Default::default()
      };
      let mut vk_image_view = VkImageView::NULL;
      let r = unsafe { vkCreateImageView(*vk_device, &info, null(), &mut vk_image_view) };
      if r != VK_SUCCESS {
        drop(DestroySwapchainOnDrop {
          vk_swapchain_khr,
          surface_format,
          _images: images,
          image_extent,
          image_views,
          min_image_count,
          present_mode,
          _surface: surface.0.clone(),
          device: self.0.clone(),
        });
        return Err(VkError::new(r.0).unwrap());
      } else {
        image_views.push(vk_image_view);
      }
    }

    Ok(Swapchain(Arc::new(DestroySwapchainOnDrop {
      vk_swapchain_khr,
      surface_format,
      _images: images,
      image_extent,
      image_views,
      min_image_count,
      present_mode,
      _surface: surface.0.clone(),
      device: self.0.clone(),
    })))
  }

  /// Creates a new shader module holding the SPIRV code given.
  #[inline]
  pub fn create_shader_module(&self, code: &[u32]) -> Result<ShaderModule, VkError> {
    let Some(vkCreateShaderModule) = self.0.fns.CreateShaderModule else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let crate_info = VkShaderModuleCreateInfo {
      code_size: code.len().checked_mul(size_of::<u32>()).unwrap(),
      code: code.as_ptr(),
      ..Default::default()
    };
    let vk_device = self.0.vk_device.read().unwrap();
    let mut vk_shader_module = VkShaderModule::NULL;
    let r = unsafe {
      vkCreateShaderModule(*vk_device, &crate_info, null(), &mut vk_shader_module)
    };
    if r == VK_SUCCESS {
      Ok(ShaderModule { vk_shader_module, device: self.0.clone() })
    } else {
      Err(VkError::new(r.0).unwrap())
    }
  }
}

pub(crate) struct DestroySwapchainOnDrop {
  pub(crate) vk_swapchain_khr: VkSwapchainKHR,
  pub(crate) surface_format: VkSurfaceFormatKHR,
  pub(crate) image_extent: VkExtent2D,
  pub(crate) present_mode: VkPresentModeKHR,
  pub(crate) min_image_count: u32,
  pub(crate) _images: Vec<VkImage>,
  pub(crate) image_views: Vec<VkImageView>,
  //
  pub(crate) device: Arc<DestroyDeviceOnDrop>,
  pub(crate) _surface: Arc<DestroySurfaceOnDrop>,
}
impl Drop for DestroySwapchainOnDrop {
  #[inline]
  fn drop(&mut self) {
    let vkDeviceWaitIdle = self.device.fns.DeviceWaitIdle.unwrap();
    let vkDestroySwapchainKHR = self.device.fns.DestroySwapchainKHR.unwrap();
    let vkDestroyImageView = self.device.fns.DestroyImageView.unwrap();
    //
    let vk_device: RwLockWriteGuard<_> = self.device.vk_device.write().unwrap();
    let _vk_queue: RwLockWriteGuard<_> = self.device.vk_queue.write().unwrap();
    // Safety: The device and all its queues must be externally synch'd
    unsafe {
      if vkDeviceWaitIdle(*vk_device) == VK_SUCCESS {
        for image_view in self.image_views.drain(..) {
          vkDestroyImageView(*vk_device, image_view, null())
        }
        vkDestroySwapchainKHR(*vk_device, self.vk_swapchain_khr, null())
      } else {
        // do we want to do anything if we couldn't wait on the device?
      }
    }
  }
}

/// A swapchain connects the GPU to a particular surface so that images can be
/// presented.
pub struct Swapchain(pub(crate) Arc<DestroySwapchainOnDrop>);
impl Swapchain {
  /// The surface format used for this swapchain.
  #[inline]
  pub fn surface_format(&self) -> VkSurfaceFormatKHR {
    self.0.surface_format
  }
  /// The image extent of this swapchain's images.
  #[inline]
  pub fn image_extent(&self) -> VkExtent2D {
    self.0.image_extent
  }
  /// The swapchain's presentation mode.
  #[inline]
  pub fn present_mode(&self) -> VkPresentModeKHR {
    self.0.present_mode
  }
  /// The swapchain's minimum image count (it could have more).
  #[inline]
  pub fn min_image_count(&self) -> u32 {
    self.0.min_image_count
  }
  /// The image views in this swapchain.
  #[inline]
  pub fn image_views(&self) -> &[VkImageView] {
    &self.0.image_views
  }
}

/// Handle to a shader module.
///
/// A shader module **can** be dropped while pipelines created using its shaders
/// are still in use.
pub struct ShaderModule {
  pub(crate) vk_shader_module: VkShaderModule,
  pub(crate) device: Arc<DestroyDeviceOnDrop>,
}
impl Drop for ShaderModule {
  #[inline]
  fn drop(&mut self) {
    if let Some(vkDestroyShaderModule) = self.device.fns.DestroyShaderModule {
      let vk_device = self.device.vk_device.read().unwrap();
      unsafe { vkDestroyShaderModule(*vk_device, self.vk_shader_module, null()) }
    }
  }
}
