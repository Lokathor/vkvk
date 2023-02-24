use core::fmt::Write;

/// Defines a handle to a VK object that can be the "target" of function calls.
///
/// These are `Send`/`Sync` on the logic that safe code cannot misuse them even
/// in a multi-threaded context. The vulkan API (which is all foreign/unsafe
/// functions) has strict rules on when you must provide "external
/// synchronization" (sync within the caller's code), and if you follow those
/// rules the handles can be safely shared across threads. If you don't follow
/// those rules then you're breaking the contract of the unsafe functions and
/// it's on you.
pub fn define_handle(name: &str, parent: Option<&str>, obj_type_enum: &str) -> String {
  let mut f = String::new();
  write!(f, "/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (handle)").ok();
  if let Some(parent) = parent {
    write!(f, "/// * Parent: [{parent}]").ok();
  }
  write!(f, "/// * Object Type Enum: [`{obj_type_enum}`]").ok();
  write!(
    f,
    "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct {name}(*mut core::ffi::c_void);
    unsafe impl Send for {name} {{ }}
    unsafe impl Sync for {name} {{ }}
    impl Default for {name} {{
      #[inline]
      #[must_use]
      fn default() -> Self {{
        Self::NULL
      }}
    }}
    impl {name} {{
      pub const NULL: Self = Self::null();
      #[inline]
      #[must_use]
      pub const fn null() -> Self {{
        Self(core::ptr::null_mut())
      }}
    }}
    "
  )
  .ok();
  f
}

/// Defines a handle to a VK object that can't be the direct target of function
/// calls.
///
/// The Non-dispatchable Handle must always be 64-bit, so by default they are a
/// `u64`. When the `VK_USE_64_BIT_PTR_DEFINES` feature is enabled they'll
/// instead be a pointer. This should only ever be enabled on 64-bit platforms.
/// The logic for it being turned on can be put in build.rs, or we could even
/// skip on it entirely. If we only ever use a `u64` then we'll still get the
/// right data from VK and pass back the right data later, so things will work.
///
/// These types are `Send`/`Sync` for the same reasons that the dispatchable
/// handle types are.
pub fn define_non_dispatchable_handle(
  name: &str, parent: Option<&str>, obj_type_enum: &str,
) -> String {
  let mut f = String::new();
  write!(f,"/// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (non-dispatchable handle)").ok();
  if let Some(parent) = parent {
    write!(f, "/// * Parent: [{parent}]").ok();
  }
  write!(f, "/// * Object Type Enum: [`{obj_type_enum}`]").ok();
  write!(
    f,
    "#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct {name}(
      #[cfg(target_pointer_width=\"64\")]
      *mut core::ffi::c_void,
      #[cfg(not(target_pointer_width=\"64\"))]
      u64
    );
    unsafe impl Send for {name} {{ }}
    unsafe impl Sync for {name} {{ }}
    impl {name} {{
      pub const NULL: Self = Self::null();
      #[inline]
      #[must_use]
      pub const fn null() -> Self {{
        #[cfg(target_pointer_width=\"64\")]
        return Self(core::ptr::null_mut());
        #[cfg(not(target_pointer_width=\"64\"))]
        return Self(0);
      }}
    }}
    "
  )
  .ok();
  f
}
