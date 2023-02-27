//#![no_std]

extern crate alloc;

use core::{num::NonZeroI32, ptr::null_mut};

use crate::prelude::*;
use alloc::vec::Vec;

#[macro_use]
mod macros;

pub mod prelude;

pub mod version_1_0;

pub mod ext;

#[cfg_attr(windows, link(name = "vulkan-1"))]
#[cfg_attr(not(windows), link(name = "vulkan"))]
#[allow(dead_code)]
extern "system" {
  /// Khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
  pub(crate) fn vkGetInstanceProcAddr(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
}

#[allow(non_camel_case_types)]
pub(crate) type vkGetInstanceProcAddr_t =
  unsafe extern "system" fn(VkInstance, *const u8) -> PFN_vkVoidFunction;

#[repr(transparent)]
pub struct Entry(vkGetInstanceProcAddr_t);
impl Entry {
  pub const LINKED: Self = Self(vkGetInstanceProcAddr);

  pub fn enumerate_instance_layer_properties(&self) -> Result<Vec<VkLayerProperties>, NonZeroI32> {
    let pfn_void = unsafe {
      vkGetInstanceProcAddr(VkInstance::NULL, vkEnumerateInstanceLayerProperties_NAME.as_ptr())
    };
    let f: vkEnumerateInstanceLayerProperties_t = if let Some(v) = pfn_void {
      unsafe { core::mem::transmute(v) }
    } else {
      return Err(VK_ERROR_UNKNOWN.0.unwrap());
    };
    let mut count: u32 = 0;
    if let Some(err) = unsafe { f(&mut count, null_mut()) }.0 {
      return Err(err);
    }
    let mut buf = Vec::with_capacity(count.try_into().unwrap());
    if let Some(err) = unsafe { f(&mut count, buf.as_mut_ptr()) }.0 {
      Err(err)
    } else {
      unsafe { buf.set_len(count.try_into().unwrap()) };
      Ok(buf)
    }
  }
}
