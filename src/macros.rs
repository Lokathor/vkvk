#![allow(unused_macros)]

/// Makes an enumeration newtype.
///
/// * If you want a derived Debug you have to declare that yourself.
macro_rules! define_enumeration {
  (
    $(#[$name_meta:meta])*
    $name:ident($t:ty)
  ) => {
    $(#[$name_meta])*
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct $name(pub $t);
  };
  (
    $(#[$name_meta:meta])*
    $name:ident
  ) => {
    $(#[$name_meta])*
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct $name(pub u32);
  };
}

/// Makes a bitmask newtype (including bitwise ops impls).
///
/// * If you want a derived Debug you have to declare that yourself.
/// * This doesn't declare any type aliases, add those outside this macro.
macro_rules! define_bitmask {
  (
    $(#[$flag_bits_meta:meta])*
    $flag_bits_name:ident
  ) => {
    $(#[$flag_bits_meta])*
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct $flag_bits_name(pub u32);
    //
    impl core::ops::BitAnd for $flag_bits_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0.bitand(rhs.0))
      }
    }
    impl core::ops::BitOr for $flag_bits_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
      }
    }
    impl core::ops::BitXor for $flag_bits_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitxor(rhs.0))
      }
    }
    impl core::ops::BitAndAssign for $flag_bits_name {
      #[inline]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $flag_bits_name {
      #[inline]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $flag_bits_name {
      #[inline]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0.bitxor_assign(rhs.0)
      }
    }
    impl core::ops::Not for $flag_bits_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn not(self) -> Self::Output {
        Self(self.0.not())
      }
    }
  };
}
