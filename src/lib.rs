#![no_std]
//#![allow(nonstandard_style)]
#![allow(dead_code)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

use core::{
  ffi::*,
  num::NonZeroI32,
  ptr::{null, null_mut},
};

mod handles;
pub use handles::*;

mod flag_bits;
pub use flag_bits::*;

mod structs;
use structs::*;

mod fn_types;
use fn_types::*;

mod entry;
pub use entry::*;

mod version;
pub use version::*;

type uint32_t = u32;
type uint8_t = u8;
type size_t = usize;
type int32_t = i32;
type float = c_float;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct VkDeviceSize(pub u64);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct VkBool32(u32);
impl From<VkBool32> for bool {
  #[inline]
  #[must_use]
  fn from(value: VkBool32) -> Self {
    value.0 != 0
  }
}
impl core::fmt::Debug for VkBool32 {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Debug::fmt(&bool::from(*self), f)
  }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct VkSystemAllocationScope(u32);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct VkInternalAllocationType(u32);

/// Khronos: [VkStructureType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStructureType.html)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct VkStructureType(u32);
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = VkStructureType(0);
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = VkStructureType(1);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct VkPhysicalDeviceType(u32);

pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn()>;

pub type vkAllocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: size_t,
  alignment: size_t,
  allocation_scope: VkSystemAllocationScope,
);
pub type vkReallocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  original: *mut c_void,
  size: size_t,
  alignment: size_t,
  allocation_scope: VkSystemAllocationScope,
);
pub type vkFreeFunction_t = unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
pub type vkInternalAllocationNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: size_t,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);
pub type vkInternalFreeNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: size_t,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);

pub type PFN_vkAllocationFunction = Option<vkAllocationFunction_t>;
pub type PFN_vkReallocationFunction = Option<vkReallocationFunction_t>;
pub type PFN_vkFreeFunction = Option<vkFreeFunction_t>;
pub type PFN_vkInternalAllocationNotification = Option<vkInternalAllocationNotification_t>;
pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification_t>;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
#[must_use]
pub struct VkResult(Option<VkErrorCode>);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct VkErrorCode(NonZeroI32);
impl VkErrorCode {
  pub const ERROR_UNKNOWN: Self = Self(match NonZeroI32::new(-13) {
    Some(nz) => nz,
    None => panic!(),
  });
}

const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
const VK_MAX_DESCRIPTION_SIZE: usize = 256;
const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
const VK_UUID_SIZE: usize = 16;

/// Array that holds zero-terminated string data.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ArrayZStr<const N: usize>([u8; N]);
impl<const N: usize> ArrayZStr<N> {
  pub fn as_str(&self) -> &str {
    let zero_point = self.0.iter().copied().position(|b| b == 0).unwrap();
    core::str::from_utf8(&self.0[..zero_point]).unwrap()
  }
  pub const fn as_ptr(&self) -> *const u8 {
    self.0.as_ptr()
  }
}
impl<const N: usize> core::fmt::Debug for ArrayZStr<N> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Debug::fmt(self.as_str(), f)
  }
}
impl<const N: usize> Default for ArrayZStr<N> {
  fn default() -> Self {
    Self([0_u8; N])
  }
}
