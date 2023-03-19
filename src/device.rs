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
impl Device {
  /// Creates a new [Swapchain].
  #[inline]
  pub fn create_swapchain_khr(
    &self, surface: &Surface, surface_format: VkSurfaceFormatKHR, min_image_count: u32,
    image_extent: VkExtent2D, image_usage: VkImageUsageFlags,
    present_mode: VkPresentModeKHR,
  ) -> Result<Swapchain, VkError> {
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
    let vk_surface_khr = surface.0.vk_surface_khr.write().unwrap();
    let image_format = surface_format.format;
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
      pre_transform: VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
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
        drop(Swapchain(Arc::new(DestroySwapchainOnDrop {
          vk_swapchain_khr,
          _surface_format: surface_format,
          _images: images,
          image_views,
          _surface: surface.0.clone(),
          device: self.0.clone(),
        })));
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
          drop(Swapchain(Arc::new(DestroySwapchainOnDrop {
            vk_swapchain_khr,
            _surface_format: surface_format,
            _images: images,
            image_views,
            _surface: surface.0.clone(),
            device: self.0.clone(),
          })));
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
        drop(Swapchain(Arc::new(DestroySwapchainOnDrop {
          vk_swapchain_khr,
          _surface_format: surface_format,
          _images: images,
          image_views,
          _surface: surface.0.clone(),
          device: self.0.clone(),
        })));
        return Err(VkError::new(r.0).unwrap());
      } else {
        image_views.push(vk_image_view);
      }
    }

    Ok(Swapchain(Arc::new(DestroySwapchainOnDrop {
      vk_swapchain_khr,
      _surface_format: surface_format,
      _images: images,
      image_views,
      _surface: surface.0.clone(),
      device: self.0.clone(),
    })))
  }
}

pub(crate) struct DestroySwapchainOnDrop {
  pub(crate) vk_swapchain_khr: VkSwapchainKHR,
  pub(crate) _surface_format: VkSurfaceFormatKHR,
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
