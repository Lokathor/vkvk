#![allow(nonstandard_style)]

//! Various base types which don't fit elsewhere.

use crate::prelude::*;

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

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
impl core::fmt::Debug for VkBool32 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Debug::fmt(&bool::from(*self), f)
  }
}
impl core::fmt::Display for VkBool32 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Display::fmt(&bool::from(*self), f)
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// A vulkan version value.
///
/// Vulkan uses an `x.y.z` version system, with the addition that an
/// implementation can have a non-zero "variant" value to indicate that it is
/// non-standard in some way (eg: [Vulkan SC](https://www.khronos.org/vulkansc/)).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkVersion(pub u32);
#[allow(missing_docs)]
impl VkVersion {
  pub const API_1_0: Self = Self::major_minor_patch(1, 0, 0);
  pub const API_1_1: Self = Self::major_minor_patch(1, 1, 0);
  pub const API_1_2: Self = Self::major_minor_patch(1, 2, 0);
  pub const API_1_3: Self = Self::major_minor_patch(1, 3, 0);
  //
  pub const API_SC_1_0: Self = Self::variant_major_minor_patch(1, 1, 3, 0);
  //
  pub const HEADER: Self = Self::major_minor_patch(1, 3, 242);

  #[inline]
  #[must_use]
  pub const fn patch(self) -> u32 {
    u32_get_value(0, 11, self.0)
  }
  #[inline]
  #[must_use]
  pub const fn minor(self) -> u32 {
    u32_get_value(12, 21, self.0)
  }
  #[inline]
  #[must_use]
  pub const fn major(self) -> u32 {
    u32_get_value(22, 28, self.0)
  }
  #[inline]
  #[must_use]
  pub const fn variant(self) -> u32 {
    u32_get_value(29, 31, self.0)
  }
  #[inline]
  #[must_use]
  pub const fn major_minor_patch(major: u32, minor: u32, patch: u32) -> Self {
    let patch_and_minor = u32_with_value(12, 21, patch, minor);
    Self(u32_with_value(22, 28, patch_and_minor, major))
  }
  #[inline]
  #[must_use]
  pub const fn variant_major_minor_patch(
    variant: u32, major: u32, minor: u32, patch: u32,
  ) -> Self {
    let patch_and_minor = u32_with_value(12, 21, patch, minor);
    let major_and_minor_and_patch = u32_with_value(22, 28, patch_and_minor, major);
    Self(u32_with_value(29, 31, major_and_minor_and_patch, variant))
  }
}
impl core::fmt::Debug for VkVersion {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let major = self.major();
    let minor = self.minor();
    let patch = self.patch();
    write!(f, "{major}.{minor}.{patch}")?;
    match self.variant() {
      0 => Ok(()),
      variant => write!(f, " [variant {variant}]"),
    }
  }
}
impl core::fmt::Display for VkVersion {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Debug::fmt(self, f)
  }
}
impl core::cmp::PartialOrd for VkVersion {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    use core::cmp::Ordering;
    if self.variant() != other.variant() {
      return None;
    }
    Some(match self.major().cmp(&other.major()) {
      Ordering::Equal => match self.minor().cmp(&other.minor()) {
        Ordering::Equal => self.patch().cmp(&other.patch()),
        otherwise => otherwise,
      },
      otherwise => otherwise,
    })
  }
}

// Pulled from the bitfrob crate and just pasted in here to avoid having a full
// dependency.

#[inline]
#[must_use]
const fn u32_get_value(low: u32, high: u32, u: u32) -> u32 {
  u32_get_region(0, high - low, u >> low)
}
#[inline]
#[must_use]
const fn u32_get_region(low: u32, high: u32, u: u32) -> u32 {
  let mask = u32_region_mask(low, high);
  u & mask
}
#[inline]
#[must_use]
const fn u32_with_value(low: u32, high: u32, old: u32, replacement: u32) -> u32 {
  u32_with_region(low, high, old, replacement << low)
}
#[inline]
#[must_use]
const fn u32_with_region(low: u32, high: u32, old: u32, replacement: u32) -> u32 {
  let mask = u32_region_mask(low, high);
  (old & !mask) | (replacement & mask)
}
#[inline]
#[must_use]
const fn u32_region_mask(low: u32, high: u32) -> u32 {
  assert!(low < <u32>::BITS);
  assert!(high < <u32>::BITS);
  assert!(low < high);
  (<u32>::MAX >> ((<u32>::BITS - 1) - (high - low))) << low
}

#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ZString {
  ptr: *mut u8,
}
impl TryFrom<String> for ZString {
  type Error = ();
  fn try_from(mut s: String) -> Result<Self, ()> {
    if s.chars().any(|ch| ch == '\0') {
      Err(())
    } else {
      s.push('\0');
      Ok(Self { ptr: Box::leak(s.into_boxed_str()).as_mut_ptr() })
    }
  }
}
impl TryFrom<&str> for ZString {
  type Error = ();
  /// The input must not have any *internal* zero bytes, but *trailing* zero
  /// bytes are allowed and ignored.
  fn try_from(s: &str) -> Result<Self, ()> {
    let s = s.trim_end_matches('\0');
    if s.chars().any(|ch| ch == '\0') {
      Err(())
    } else {
      let s: String = s.chars().chain(Some('\0')).collect();
      Ok(Self { ptr: Box::leak(s.into_boxed_str()).as_mut_ptr() })
    }
  }
}
impl Drop for ZString {
  fn drop(&mut self) {
    let len = self.len();
    let slice_ptr: *mut [u8] = core::ptr::slice_from_raw_parts_mut(self.ptr, len);
    drop(unsafe { Box::from_raw(slice_ptr) })
  }
}
impl ZString {
  /// The length in bytes.
  #[inline]
  #[must_use]
  pub const fn len(&self) -> usize {
    let mut len = 0;
    // Note(Lokathor): this cast from mut to const lets us make it a const fn.
    let mut p = self.ptr as *const u8;
    unsafe {
      while *p != 0 {
        len += 1;
        p = p.add(1);
      }
    }
    len
  }
  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
  #[inline]
  #[must_use]
  pub const fn as_zstr(&self) -> ZStr<'_> {
    ZStr { ptr: self.ptr, life: core::marker::PhantomData }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ZStr<'b> {
  ptr: *const u8,
  life: core::marker::PhantomData<&'b [u8]>,
}
impl<'b> TryFrom<&'b str> for ZStr<'b> {
  type Error = ();
  /// The input must not contain internal zero bytes, and must end with one or
  /// more zero bytes.
  fn try_from(value: &'b str) -> Result<Self, Self::Error> {
    let stripped = value.trim_end_matches('\0');
    if stripped.as_bytes().iter().copied().any(|b| b == 0) {
      Err(())
    } else if value.ends_with('\0') {
      Ok(Self { ptr: value.as_ptr(), life: core::marker::PhantomData })
    } else {
      Err(())
    }
  }
}
impl ZStr<'_> {
  #[inline]
  #[must_use]
  pub const fn as_ptr(self) -> *const u8 {
    self.ptr
  }
}

/// Holds a zero-terminated string within an array.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ArrayZString<const N: usize>([u8; N]);
impl<const N: usize> ArrayZString<N> {
  #[inline]
  #[must_use]
  pub const fn as_ptr(&self) -> *const u8 {
    self.0.as_ptr()
  }
  #[inline]
  #[must_use]
  pub const fn as_zstr(&self) -> ZStr<'_> {
    ZStr { ptr: self.0.as_ptr(), life: core::marker::PhantomData }
  }
  #[inline]
  #[must_use]
  pub fn as_str(&self) -> &str {
    let zero_point = self.0.iter().copied().position(|b| b == 0).unwrap();
    core::str::from_utf8(&self.0[..zero_point]).unwrap()
  }
}
// Note(Lokathor): At the time of writing this, you still can't derive default
// for const-generics.
impl<const N: usize> Default for ArrayZString<N> {
  #[inline]
  fn default() -> Self {
    Self([0_u8; N])
  }
}
impl<const N: usize> core::fmt::Debug for ArrayZString<N> {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Debug::fmt(self.as_str(), f)
  }
}
impl<const N: usize> core::fmt::Display for ArrayZString<N> {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Display::fmt(self.as_str(), f)
  }
}

#[allow(nonstandard_style)]
pub type vkVoidFunction_t = unsafe extern "system" fn();
#[allow(nonstandard_style)]
pub type vkAllocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: usize,
  alignment: usize,
  allocation_scope: VkSystemAllocationScope,
);
#[allow(nonstandard_style)]
pub type vkReallocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  original: *mut c_void,
  size: usize,
  alignment: usize,
  allocation_scope: VkSystemAllocationScope,
);
#[allow(nonstandard_style)]
pub type vkFreeFunction_t =
  unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
#[allow(nonstandard_style)]
pub type vkInternalAllocationNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: usize,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);
#[allow(nonstandard_style)]
pub type vkInternalFreeNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: usize,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);

#[allow(nonstandard_style)]
pub type PFN_vkVoidFunction = Option<vkVoidFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkAllocationFunction = Option<vkAllocationFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkReallocationFunction = Option<vkReallocationFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkFreeFunction = Option<vkFreeFunction_t>;
#[allow(nonstandard_style)]
pub type PFN_vkInternalAllocationNotification =
  Option<vkInternalAllocationNotification_t>;
#[allow(nonstandard_style)]
pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification_t>;
