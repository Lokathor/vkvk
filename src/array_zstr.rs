/// Array that holds zero-terminated string data.
///
/// This is intended for the driver to be able to copy string data back out to
/// the user program.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ArrayZStr<const N: usize>([u8; N]);
impl<const N: usize> ArrayZStr<N> {
  #[inline]
  pub fn as_str(&self) -> &str {
    let zero_point = self.0.iter().copied().position(|b| b == 0).unwrap();
    core::str::from_utf8(&self.0[..zero_point]).unwrap()
  }
  #[inline]
  pub const fn as_ptr(&self) -> *const u8 {
    self.0.as_ptr()
  }
}
impl<const N: usize> core::fmt::Debug for ArrayZStr<N> {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Debug::fmt(self.as_str(), f)
  }
}
impl<const N: usize> Default for ArrayZStr<N> {
  #[inline]
  fn default() -> Self {
    Self([0_u8; N])
  }
}
