//! Provides the [Entry] type.

use zstring::ZStr;

use crate::prelude::*;

extern "system" {
  fn vkGetInstanceProcAddr(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
}

/// An entry point to the Vulkan API.
///
/// This allows you to query the limits of [Instance] creation, namely the
/// available layers and extensions. Use this info to create an appropriate
/// instance, which will then allow you to proceed further with initializing
/// Vulkan.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Entry(vkGetInstanceProcAddr_t);
impl Default for Entry {
  /// Gives [`Entry::LINKED`]
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::LINKED
  }
}
impl Entry {
  /// An entry point that uses the dynamically linked Vulkan.
  ///
  /// This is generally fine to use, but you way wish to dynamically load an
  /// alternate Vulkan entry.
  pub const LINKED: Self = Self(vkGetInstanceProcAddr);

  /// Wraps the function pointer as a Vulkan entry point.
  ///
  /// ## Safety
  /// * The pointer must be for a genuine [vkGetInstanceProcAddr][vk] function.
  ///
  /// [vk]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html
  #[inline]
  #[must_use]
  pub const unsafe fn new(get_instance_proc_addr: vkGetInstanceProcAddr_t) -> Self {
    Self(get_instance_proc_addr)
  }

  /// Gets the maximum instance version that this Vulkan loader supports.
  #[inline]
  #[must_use]
  pub fn enumerate_max_instance_version(&self) -> VkVersion {
    #[allow(non_upper_case_globals)]
    const vkEnumerateInstanceVersion_NAME: ZStr<'static> =
      ZStr::from_lit("vkEnumerateInstanceVersion\0");
    let opt_f: PFN_vkEnumerateInstanceVersion = unsafe {
      core::mem::transmute((self.0)(
        VkInstance::NULL,
        vkEnumerateInstanceVersion_NAME.as_ptr(),
      ))
    };
    if let Some(f) = opt_f {
      let mut version = VkVersion::default();
      unsafe { f(&mut version as *mut VkVersion as *mut u32) };
      version
    } else {
      VkVersion::API_1_0
    }
  }

  /// Gets the properties of all available instance layers.
  #[inline]
  pub fn enumerate_instance_layer_properties(
    &self,
  ) -> Result<Vec<VkLayerProperties>, ErrorCode> {
    #[allow(non_upper_case_globals)]
    const vkEnumerateInstanceLayerProperties_NAME: ZStr<'static> =
      ZStr::from_lit("vkEnumerateInstanceLayerProperties\0");
    let opt_f: PFN_vkEnumerateInstanceLayerProperties = unsafe {
      core::mem::transmute((self.0)(
        VkInstance::NULL,
        vkEnumerateInstanceLayerProperties_NAME.as_ptr(),
      ))
    };
    if let Some(f) = opt_f {
      'gather: loop {
        let mut count = 0_u32;
        let r = unsafe { f(&mut count, null_mut()) };
        match r {
          VK_SUCCESS => (),
          other => return Err(ErrorCode::new(other.0).unwrap()),
        }
        let mut buf: Vec<VkLayerProperties> =
          Vec::with_capacity(count.try_into().unwrap());
        let r = unsafe { f(&mut count, buf.as_mut_ptr()) };
        match r {
          VK_SUCCESS => {
            unsafe { buf.set_len(count.try_into().unwrap()) };
            return Ok(buf);
          }
          VK_INCOMPLETE => continue 'gather,
          other => return Err(ErrorCode::new(other.0).unwrap()),
        }
      }
    } else {
      Err(ErrorCode::new(VK_ERROR_UNKNOWN.0).unwrap())
    }
  }
}
