#![allow(missing_docs)]
#![allow(nonstandard_style)]

//! Various base types which don't fit elsewhere.

// Note(Lokathor): this import makes the doc links work properly without having
// to specify the full path on all of them.
#[allow(unused_imports)]
#[cfg(feature = "raw-window-handle")]
use raw_window_handle::*;

pub use core::ffi::c_void;

/// Mild fudge until <https://github.com/rust-lang/rust/issues/88345>
#[allow(non_camel_case_types)]
pub type c_size_t = usize;

/// Part of [AndroidNdkWindowHandle]
pub type ANativeWindow = c_void;
/// Android Hardware Buffer
pub type AHardwareBuffer = c_void;

/// Part of [XcbDisplayHandle]
pub type xcb_connection_t = *mut c_void;
/// Part of [XcbWindowHandle]
pub type xcb_window_t = u32;
pub type xcb_visualid_t = u32;

/// Part of [XlibDisplayHandle]
///
/// Normally this is just defined as `Display`, but that's too generic for a
/// type from outsid this crate so we define it with a slightly better name.
pub type XlibDisplay = *mut c_void;
/// Part of [XlibWindowHandle]
///
/// Normally this is just defined as `Window`, but that's too generic for a
/// type from outsid this crate so we define it with a slightly better name.
pub type XlibWindow = core::ffi::c_ulong;

pub type XlibVisualID = core::ffi::c_uint;

/// Part of [WaylandDisplayHandle]
pub type wl_display = *mut c_void;
/// Part of [WaylandWindowHandle]
pub type wl_surface = *mut c_void;

/// A [Win32](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) data type.
///
/// Part of [Win32WindowHandle]
pub type HWND = *mut c_void;
/// A [Win32](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) data type.
///
/// Part of [Win32WindowHandle]
pub type HINSTANCE = *mut c_void;
/// A [Win32](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) data type.
pub type HANDLE = *mut c_void;
/// A [Win32](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) data type.
pub type HMONITOR = *mut c_void;
/// A [Win32](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) data type.
pub type DWORD = u32;
/// A [Win32](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) data type.
pub type LPCWSTR = *const u16;
/// MSDN: [SECURITY_ATTRIBUTES](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa379560(v=vs.85))
///
/// Defined as `c_void` in this crate since it's only ever passed via pointer in
/// Vulkan, and you can just cast the pointer.
pub type SECURITY_ATTRIBUTES = c_void;

// TODO: docs for these?

pub type CAMetalLayer = c_void;
pub type MTLDevice_id = *mut c_void;
pub type MTLCommandQueue_id = *mut c_void;
pub type MTLBuffer_id = *mut c_void;
pub type MTLTexture_id = *mut c_void;
pub type MTLSharedEvent_id = *mut c_void;
pub type IOSurfaceRef = *mut c_void;
pub type _screen_context = c_void;
pub type _screen_window = c_void;
pub type IDirectFB = c_void;
pub type IDirectFBSurface = c_void;
pub type zx_handle_t = u32;
pub type VkRemoteAddressNV = *mut c_void;

pub type VkBool32 = bool32::Bool32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSampleMask(pub u32);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDeviceSize(pub u64);
// TODO: it would be neat if the debug printing here could automatically shift
// from showing bytes up to kilobytes, megabytes, or gigabytes, depending on how
// big the value is.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDeviceAddress(pub u64);

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
  pub const HEADER: Self = Self::major_minor_patch(1, 3, 243);

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
    Self::variant_major_minor_patch(0, major, minor, patch)
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
      variant => write!(f, " +variant.{variant}"),
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

// Pulled from the bitfrob crate, just copied into here to avoid having a full
// dependency, since only `VkVersion` uses it.

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
