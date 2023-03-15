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

pub(crate) struct DestroySwapchainOnDrop {
  vk_swapchain_khr: VkSwapchainKHR,
  image_format: VkFormat,
  _parent_surface: Arc<DestroySurfaceOnDrop>,
  parent_device: Arc<DestroyDeviceOnDrop>,
}
impl Drop for DestroySwapchainOnDrop {
  #[inline]
  fn drop(&mut self) {
    panic!()
  }
}

pub(crate) struct DestroySwapchainImageViewOnDrop {
  _vk_image_view: VkImageView,
  _parent_swapchain: Arc<DestroySwapchainOnDrop>,
}
impl Drop for DestroySwapchainImageViewOnDrop {
  #[inline]
  fn drop(&mut self) {
    panic!()
  }
}

/// An ImageView controls how the texels of an Image are accessed.
///
/// This ImageView is for an Image within a Swapchain.
pub struct SwapchainImageView(pub(crate) Arc<DestroySwapchainImageViewOnDrop>);

/// A Swapchain is the link between a [Surface] and a [Device].
pub struct Swapchain(pub(crate) Arc<DestroySwapchainOnDrop>);
impl Swapchain {
  /// Get the presentable images of this swapchain and makes one Image View for
  /// each Image.
  #[inline]
  pub fn get_images_khr_and_make_views(
    &self,
  ) -> Result<Vec<SwapchainImageView>, VkError> {
    let Some(vkCreateImageView) = self.0.parent_device.fns.CreateImageView else {
      return Err(VkError::new(VK_ERROR_UNKNOWN.0).unwrap());
    };
    let Some(vkGetSwapchainImagesKHR) = self.0.parent_device.fns.GetSwapchainImagesKHR else {
      return Err(VkError::new(VK_ERROR_EXTENSION_NOT_PRESENT.0).unwrap());
    };
    let vk_device = self.0.parent_device.vk_device;
    let vk_swapchain_khr = self.0.vk_swapchain_khr;
    'gather: loop {
      let mut count = 0_u32;
      let r = unsafe {
        vkGetSwapchainImagesKHR(vk_device, vk_swapchain_khr, &mut count, null_mut())
      };
      if r != VK_SUCCESS {
        return Err(NonZeroI32::new(r.0).unwrap());
      }
      let mut buf: Vec<VkImage> = Vec::with_capacity(count.try_into().unwrap());
      let r = unsafe {
        vkGetSwapchainImagesKHR(vk_device, vk_swapchain_khr, &mut count, buf.as_mut_ptr())
      };
      match r {
        VK_SUCCESS => {
          unsafe { buf.set_len(count.try_into().unwrap()) };
          return Ok(
            buf
              .into_iter()
              .map(|image| {
                let info = VkImageViewCreateInfo {
                  image,
                  view_type: VK_IMAGE_VIEW_TYPE_2D,
                  format: self.0.image_format,
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
                unsafe {
                  vkCreateImageView(vk_device, &info, null(), &mut vk_image_view)
                };
                SwapchainImageView(Arc::new(DestroySwapchainImageViewOnDrop {
                  _vk_image_view: vk_image_view,
                  _parent_swapchain: self.0.clone(),
                }))
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

/// A [VkImage] that comes from a swapchain.
pub struct SwapchainImage {
  _vk_image: VkImage,
  _parent: Arc<DestroySwapchainOnDrop>,
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
    let Some(vkCreateSwapchainKHR) = self.0.fns.CreateSwapchainKHR else {
      return Err(NonZeroI32::new(VK_ERROR_EXTENSION_NOT_PRESENT.0).unwrap());
    };
    //
    let image_format = surface_format.format;
    let swapchain_create_info_khr = VkSwapchainCreateInfoKHR {
      surface: surface.0.vk_surface_khr,
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
    let vk_device = self.0.vk_device;
    let mut vk_swapchain_khr = VkSwapchainKHR::NULL;
    let r = unsafe {
      vkCreateSwapchainKHR(
        vk_device,
        &swapchain_create_info_khr,
        null(),
        &mut vk_swapchain_khr,
      )
    };
    if r != VK_SUCCESS {
      return Err(NonZeroI32::new(r.0).unwrap());
    }
    Ok(Swapchain(Arc::new(DestroySwapchainOnDrop {
      vk_swapchain_khr,
      image_format,
      _parent_surface: surface.0.clone(),
      parent_device: self.0.clone(),
    })))
  }
}
