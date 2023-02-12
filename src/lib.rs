#![no_std]
#![allow(nonstandard_style)]
#![allow(dead_code)]

use core::{ffi::*, num::NonZeroI32};

use version::VkVersion;

pub mod version;

#[cfg_attr(windows, link(name = "vulkan-1"))]
#[cfg_attr(not(windows), link(name = "vulkan"))]
extern "system" {
  /// khronos: [vkGetInstanceProcAddr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html)
  pub fn vkGetInstanceProcAddr(instance: VkInstance, name: *const c_char) -> PFN_vkVoidFunction;
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
  ($name:ident ( $($arg_name:ident : $arg_ty:ty),* ) -> $ret_ty:ty) => {
    #[allow(nonstandard_style)]
    type $name = unsafe extern "system" fn( $( $arg_name: $arg_ty ),* ) -> $ret_ty;
  };
  ($name:ident ( $($arg_name:ident : $arg_ty:ty),* )) => {
    #[allow(nonstandard_style)]
    type $name = unsafe extern "system" fn( $( $arg_name: $arg_ty ),* );
  };
}

fn_type!(vkGetInstanceProcAddr_t(instance: VkInstance, name: *const c_char) -> PFN_vkVoidFunction);
fn_type!(vkEnumerateInstanceVersion_t(pApiVersion: *mut VkVersion) -> VkResult);

// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html

// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html

// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html

#[repr(transparent)]
pub struct Entry(vkGetInstanceProcAddr_t);
impl Entry {
  pub const LINKED: Self = Self(vkGetInstanceProcAddr);

  pub fn get_max_instance_version(&self) -> Result<VkVersion, VkErrorCode> {
    let vkGetInstanceProcAddr = self.0;
    const FN_NAME: &str = "vkEnumerateInstanceVersion\0";
    let Some(pfn) =  (unsafe { vkGetInstanceProcAddr(VkInstance::NULL, FN_NAME.as_ptr().cast()) }) else {
      // When the fn is missing we assume that it means Vulkan 1.0
      return Ok(VkVersion(0))
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
}
