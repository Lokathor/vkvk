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
    #[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct $name(pub $t);
  };
}

/// Makes a bitmask newtype (including bitwise ops impls).
///
/// * If you want a derived Debug you have to declare that yourself.
/// * This doesn't declare any type aliases, add those outside this macro.
macro_rules! define_bitmask {
  (
    $(#[$flag_bits_meta:meta])*
    $flag_bits_name:ident($t:ty)
  ) => {
    $(#[$flag_bits_meta])*
    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct $flag_bits_name(pub $t);
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

/// This "fakes" a `&mut Vec<_>` by packing up the fields into a
/// `ManuallyDrop<Vec<_>>`, calling the closure, and then unpacking the fields
/// back into place.
macro_rules! fake_ptr_len_cap {
  ($ptr:expr, $len:expr, $cap:expr, $op:expr) => {{
    let mut v: ManuallyDrop<Vec<_>> = ManuallyDrop::new(unsafe {
      Vec::from_raw_parts($ptr, $len.try_into().unwrap(), $cap.try_into().unwrap())
    });
    {
      // Making the fake vec can put the vec data into a bad state where
      // dropping the container will go wrong. So we need to be unwind safe, but
      // we can't mix `ManuallyDrop` and `catch_unwind`, so we'll just go hard
      // and abort if the code tries to unwind out of here.
      let abort_on_drop = AbortOnDrop(());
      $op(&mut v);
      core::mem::forget(abort_on_drop);
    }
    $cap = v.capacity().try_into().unwrap();
    $len = v.len().try_into().unwrap();
    $ptr = v.as_mut_ptr();
  }};
}
