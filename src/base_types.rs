#![allow(nonstandard_style)]

//! Various base types which don't fit elsewhere.

pub use core::ffi::c_void;

pub use core::ffi::c_float;
use core::num::NonZeroI32;

/// Android Native Window
pub type ANativeWindow = c_void;
/// Android Hardware Buffer
pub type AHardwareBuffer = c_void;

pub type CAMetalLayer = c_void;
pub type MTLDevice_id = *mut c_void;
pub type MTLCommandQueue_id = *mut c_void;
pub type MTLBuffer_id = *mut c_void;
pub type MTLTexture_id = *mut c_void;
pub type MTLSharedEvent_id = *mut c_void;

pub type IOSurfaceRef = *mut c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSampleMask(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkBool32(u32);
impl VkBool32 {
  pub const FALSE: Self = Self(0);
  pub const TRUE: Self = Self(1);
}
impl From<bool> for VkBool32 {
  #[inline]
  #[must_use]
  fn from(value: bool) -> Self {
    Self(value as _)
  }
}
impl From<VkBool32> for bool {
  #[inline]
  #[must_use]
  fn from(value: VkBool32) -> Self {
    value.0 != 0
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDeviceSize(pub u64);
// TODO: it would be neat if the debug printing here could automatically shift
// from showing bytes up to kilobytes, megabytes, or gigabytes, depending on how
// big the value is.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDeviceAddress(pub u64);

/// Khronos: [VkResult](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResult.html) (enumeration)
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkResult(pub Option<NonZeroI32>);
