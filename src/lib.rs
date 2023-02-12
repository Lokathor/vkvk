#![no_std]
#![allow(nonstandard_style)]
#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;

use core::{ffi::*, num::NonZeroI32, ptr::null_mut};

use version::VkVersion;

mod version;
pub use version::*;

#[cfg_attr(windows, link(name = "vulkan-1"))]
#[cfg_attr(not(windows), link(name = "vulkan"))]
extern "system" {
  /// khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
  pub fn vkGetInstanceProcAddr(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
}

// TODO: handle
#[repr(transparent)]
pub struct VkInstance(*mut c_void);
impl VkInstance {
  pub const NULL: Self = Self::null();

  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}

pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn()>;

type uint32_t = u32;

#[repr(transparent)]
#[must_use]
pub struct VkResult(Option<VkErrorCode>);

#[derive(Debug)]
#[repr(transparent)]
pub struct VkErrorCode(NonZeroI32);
impl VkErrorCode {
  pub const ERROR_UNKNOWN: Self = Self(match NonZeroI32::new(-13) {
    Some(nz) => nz,
    None => panic!(),
  });
}

macro_rules! fn_type {
  ( $(#[$ty_meta:meta])* $name:ident ( $($arg_name:ident : $arg_ty:ty),* ) $(-> $ret_ty:ty)? ) => {
    $(#[$ty_meta])*
    #[allow(nonstandard_style)]
    type $name = unsafe extern "system" fn( $( $arg_name: $arg_ty ),* ) $(-> $ret_ty)?;
  };
}

fn_type!(
  /// khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
  vkGetInstanceProcAddr_t(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction
);
fn_type!(
  /// khronos: [vkEnumerateInstanceVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html)
  vkEnumerateInstanceVersion_t(api_version: *mut VkVersion) -> VkResult
);
fn_type!(
  /// khronos: [vkEnumerateInstanceLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html)
  vkEnumerateInstanceLayerProperties_t(property_count: *mut uint32_t, properties: *mut VkLayerProperties) -> VkResult
);
fn_type!(
  /// khronos: [vkEnumerateInstanceExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html)
  vkEnumerateInstanceExtensionProperties_t(layer_name: *const u8, property_count: *mut uint32_t,  properties: *mut VkExtensionProperties) -> VkResult
);

const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
const VK_MAX_DESCRIPTION_SIZE: usize = 256;

/// Array that holds zero-terminated string data.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ArrayZStr<const N: usize>([u8; N]);
impl<const N: usize> core::fmt::Debug for ArrayZStr<N> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let zero_point = self.0.iter().copied().position(|b| b == 0).unwrap();
    let s = core::str::from_utf8(&self.0[..zero_point]).unwrap();
    core::fmt::Debug::fmt(s, f)
  }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VkLayerProperties {
  pub layer_name: ArrayZStr<VK_MAX_EXTENSION_NAME_SIZE>,
  pub spec_version: VkVersion,
  pub implementation_version: uint32_t,
  pub description: ArrayZStr<VK_MAX_DESCRIPTION_SIZE>,
}

#[repr(C)]
struct VkExtensionProperties {
  pub extension_name: ArrayZStr<VK_MAX_EXTENSION_NAME_SIZE>,
  pub spec_version: uint32_t,
}

#[repr(transparent)]
pub struct Entry(vkGetInstanceProcAddr_t);
impl Entry {
  pub const LINKED: Self = Self(vkGetInstanceProcAddr);

  pub fn get_max_instance_version(&self) -> Result<VkVersion, VkErrorCode> {
    let vkGetInstanceProcAddr = self.0;
    const FN_NAME: &str = "vkEnumerateInstanceVersion\0";
    let Some(pfn) =  (unsafe { vkGetInstanceProcAddr(VkInstance::NULL, FN_NAME.as_ptr()) }) else {
      // When vkEnumerateInstanceVersion is missing we assume that it means Vulkan 1.0
      return Ok(VkVersion::_1_0)
    };
    let vkEnumerateInstanceVersion: vkEnumerateInstanceVersion_t =
      unsafe { core::mem::transmute(pfn) };
    let mut version = VkVersion(0);
    let res = unsafe { vkEnumerateInstanceVersion(&mut version) };
    match res.0 {
      None => Ok(version),
      Some(err_code) => Err(err_code),
    }
  }

  pub fn get_instance_layer_properties(&self) -> Result<Vec<VkLayerProperties>, VkErrorCode> {
    let vkGetInstanceProcAddr = self.0;
    const FN_NAME: &str = "vkEnumerateInstanceLayerProperties\0";
    let Some(pfn) =  (unsafe { vkGetInstanceProcAddr(VkInstance::NULL, FN_NAME.as_ptr()) }) else {
      return Err(VkErrorCode::ERROR_UNKNOWN)
    };
    let vkEnumerateInstanceLayerProperties: vkEnumerateInstanceLayerProperties_t =
      unsafe { core::mem::transmute(pfn) };
    //
    let mut property_count: u32 = 0;
    let count_ret = unsafe { vkEnumerateInstanceLayerProperties(&mut property_count, null_mut()) };
    if let Some(err_code) = count_ret.0 {
      return Err(err_code);
    }
    let mut buf = Vec::with_capacity(property_count.try_into().unwrap());
    let write_ret =
      unsafe { vkEnumerateInstanceLayerProperties(&mut property_count, buf.as_mut_ptr()) };
    if let Some(err_code) = write_ret.0 {
      Err(err_code)
    } else {
      unsafe { buf.set_len(property_count.try_into().unwrap()) };
      Ok(buf)
    }
  }
}
