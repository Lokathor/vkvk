#![warn(missing_docs)]

use bitfrob::{u32_get_value, u32_with_value};

/// A vulkan version value.
///
/// Vulkan uses a semver-like version system, with the addition that an
/// implementation can have a non-zero "variant" value to indicate that it is
/// non-standard in some way.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkVersion(pub u32);
#[allow(missing_docs)]
impl VkVersion {
  pub const _1_0: Self = Self::major_minor_patch(1, 0, 0);
  pub const _1_1: Self = Self::major_minor_patch(1, 1, 0);
  pub const _1_2: Self = Self::major_minor_patch(1, 2, 0);
  pub const _1_3: Self = Self::major_minor_patch(1, 3, 0);
  //
  pub const HEADER: Self = Self::major_minor_patch(1, 3, 240);

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
