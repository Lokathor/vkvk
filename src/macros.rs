macro_rules! define_handle {
  (
    $(#[$name_meta:meta])*
    $name:ident
  ) => {
    $(#[$name_meta])*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct $name(*mut core::ffi::c_void);
    unsafe impl Send for $name {}
    unsafe impl Sync for $name {}
    impl Default for $name {
      #[inline]
      #[must_use]
      fn default() -> Self {
        Self::NULL
      }
    }
    impl $name {
      pub const NULL: Self = Self::null();
      #[inline]
      #[must_use]
      pub const fn null() -> Self {
        Self(core::ptr::null_mut())
      }
    }
  };
}

macro_rules! define_non_dispatchable_handle {
  (
    $(#[$name_meta:meta])*
    $name:ident
  ) => {
    $(#[$name_meta])*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct $name(
      #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
      #[cfg(not(target_pointer_width = "64"))] u64,
    );
    unsafe impl Send for $name {}
    unsafe impl Sync for $name {}
    impl $name {
      pub const NULL: Self = Self::null();
      #[inline]
      #[must_use]
      pub const fn null() -> Self {
        #[cfg(target_pointer_width = "64")]
        return Self(core::ptr::null_mut());
        #[cfg(not(target_pointer_width = "64"))]
        return Self(0);
      }
    }
    impl Default for $name {
      #[inline]
      #[must_use]
      fn default() -> Self {
        Self::NULL
      }
    }
  };
}

macro_rules! define_enumeration {
  (
    $(#[$name_meta:meta])*
    $name:ident
  ) => {
    $(#[$name_meta])*
    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct $name(pub u32);
  };
}

macro_rules! define_bitmask {
  (
    $(#[$flag_bits_meta:meta])*
    $flag_bits_name:ident,
    $flags_name:ident
  ) => {
    $(#[$flag_bits_meta])*
    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct $flag_bits_name(pub u32);
    impl $flag_bits_name {
      #[inline]
      #[must_use]
      pub const fn none() -> Self {
        Self(0)
      }
    }
    pub type $flags_name = $flag_bits_name;

    impl core::ops::BitAnd for $flag_bits_name{
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
      }
    }
    impl core::ops::BitAndAssign for $flag_bits_name {
      #[inline]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
      }
    }
    impl core::ops::BitOr for $flag_bits_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $flag_bits_name {
      #[inline]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
      }
    }
    impl core::ops::BitXor for $flag_bits_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $flag_bits_name {
      #[inline]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
      }
    }
    impl core::ops::Not for $flag_bits_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn not(self) -> Self::Output {
        Self(!self.0)
      }
    }
  };
}
