#![warn(missing_docs)]
//!

/// A vulkan version value.
///
/// Vulkan uses an `x.y.z` version system, with the addition that an
/// implementation can have a non-zero "variant" value to indicate that it is
/// non-standard in some way.
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
  pub const HEADER: Self = Self::major_minor_patch(1, 3, 241);

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
  pub const fn variant_major_minor_patch(variant: u32, major: u32, minor: u32, patch: u32) -> Self {
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
