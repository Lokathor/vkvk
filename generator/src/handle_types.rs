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
  writeln!(f, "define_handle!(").ok();
  writeln!(f, "  /// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (handle)").ok();
  if let Some(parent) = parent {
    writeln!(f, "  /// * Parent: [{parent}]").ok();
  }
  writeln!(f, "  /// * Object Type Enum: [`{obj_type_enum}`]").ok();
  writeln!(f, "  {name}").ok();
  writeln!(f, ");").ok();
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
  writeln!(f, "define_non_dispatchable_handle!(").ok();
  writeln!(f,"  /// Khronos: [{name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}.html) (non-dispatchable handle)").ok();
  if let Some(parent) = parent {
    writeln!(f, "  /// * Parent: [{parent}]").ok();
  }
  writeln!(f, "  /// * Object Type Enum: [`{obj_type_enum}`]").ok();
  writeln!(f, "  {name}").ok();
  writeln!(f, ");").ok();
  f
}
