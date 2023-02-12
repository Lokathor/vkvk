use bitfrob::u32_get_value;

use crate::uint32_t;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkVersion(pub uint32_t);
impl VkVersion {
  pub const fn patch(self) -> u32 {
    u32_get_value(0, 11, self.0)
  }
  pub const fn minor(self) -> u32 {
    u32_get_value(12, 21, self.0)
  }
  pub const fn major(self) -> u32 {
    u32_get_value(22, 28, self.0)
  }
  pub const fn variant(self) -> u32 {
    u32_get_value(29, 31, self.0)
  }
}
impl core::fmt::Debug for VkVersion {
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
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Debug::fmt(self, f)
  }
}
