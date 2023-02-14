use core::num::NonZeroI32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[must_use]
pub struct VkResult(pub(crate) Option<VkErrorCode>);

/// We need this macro so that we can easily make [NonZeroI32] values in a const
/// context.
macro_rules! nzi32 {
  ($val:literal) => {
    match NonZeroI32::new($val) {
      Some(nz) => nz,
      None => panic!(),
    }
  };
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkErrorCode(NonZeroI32);
impl VkErrorCode {
  pub const NOT_READY: Self = Self(nzi32!(1));
  pub const TIMEOUT: Self = Self(nzi32!(2));
  pub const EVENT_SET: Self = Self(nzi32!(3));
  pub const EVENT_RESET: Self = Self(nzi32!(4));
  pub const INCOMPLETE: Self = Self(nzi32!(5));
  //
  pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(nzi32!(-1));
  pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(nzi32!(-2));
  pub const ERROR_INITIALIZATION_FAILED: Self = Self(nzi32!(-3));
  pub const ERROR_DEVICE_LOST: Self = Self(nzi32!(-4));
  pub const ERROR_MEMORY_MAP_FAILED: Self = Self(nzi32!(-5));
  pub const ERROR_LAYER_NOT_PRESENT: Self = Self(nzi32!(-6));
  pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(nzi32!(-7));
  pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(nzi32!(-8));
  pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(nzi32!(-9));
  pub const ERROR_TOO_MANY_OBJECTS: Self = Self(nzi32!(-10));
  pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(nzi32!(-11));
  pub const ERROR_FRAGMENTED_POOL: Self = Self(nzi32!(-12));
  pub const ERROR_UNKNOWN: Self = Self(nzi32!(-13));
}
impl core::fmt::Debug for VkErrorCode {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      Self::NOT_READY => write!(f, "NOT_READY"),
      Self::TIMEOUT => write!(f, "TIMEOUT"),
      Self::EVENT_SET => write!(f, "EVENT_SET"),
      Self::EVENT_RESET => write!(f, "EVENT_RESET"),
      Self::INCOMPLETE => write!(f, "INCOMPLETE"),
      Self::ERROR_OUT_OF_HOST_MEMORY => write!(f, "ERROR_OUT_OF_HOST_MEMORY"),
      Self::ERROR_OUT_OF_DEVICE_MEMORY => write!(f, "ERROR_OUT_OF_DEVICE_MEMORY"),
      Self::ERROR_INITIALIZATION_FAILED => write!(f, "ERROR_INITIALIZATION_FAILED"),
      Self::ERROR_DEVICE_LOST => write!(f, "ERROR_DEVICE_LOST"),
      Self::ERROR_MEMORY_MAP_FAILED => write!(f, "ERROR_MEMORY_MAP_FAILED"),
      Self::ERROR_LAYER_NOT_PRESENT => write!(f, "ERROR_LAYER_NOT_PRESENT"),
      Self::ERROR_EXTENSION_NOT_PRESENT => write!(f, "ERROR_EXTENSION_NOT_PRESENT"),
      Self::ERROR_FEATURE_NOT_PRESENT => write!(f, "ERROR_FEATURE_NOT_PRESENT"),
      Self::ERROR_INCOMPATIBLE_DRIVER => write!(f, "ERROR_INCOMPATIBLE_DRIVER"),
      Self::ERROR_TOO_MANY_OBJECTS => write!(f, "ERROR_TOO_MANY_OBJECTS"),
      Self::ERROR_FORMAT_NOT_SUPPORTED => write!(f, "ERROR_FORMAT_NOT_SUPPORTED"),
      Self::ERROR_FRAGMENTED_POOL => write!(f, "ERROR_FRAGMENTED_POOL"),
      Self::ERROR_UNKNOWN => write!(f, "ERROR_UNKNOWN"),
      Self(other) => write!(f, "VkErrorCode({other:?})"),
    }
  }
}
