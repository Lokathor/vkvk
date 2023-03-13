//! Provides the [Entry] type, which lets you set up to create an [Instance].

use crate::prelude::*;

extern "system" {
  fn vkGetInstanceProcAddr(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;
}

/// An entry point to the Vulkan API.
///
/// This allows you to query the limits of [Instance] creation, namely the
/// available layers and extensions. Use this info to create an appropriate
/// instance, which will then allow you to proceed further with initializing
/// Vulkan.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Entry(vkGetInstanceProcAddr_t);
impl Default for Entry {
  /// Gives [`Entry::LINKED`]
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::LINKED
  }
}
impl Entry {
  /// An entry point that uses the dynamically linked Vulkan.
  ///
  /// This is generally fine to use, but you way wish to dynamically load an
  /// alternate Vulkan entry.
  pub const LINKED: Self = Self(vkGetInstanceProcAddr);

  /// Wraps the function pointer as a Vulkan entry point.
  ///
  /// ## Safety
  /// * The pointer must be for a genuine [vkGetInstanceProcAddr][vk] function.
  ///
  /// [vk]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html
  #[inline]
  #[must_use]
  pub const unsafe fn new(get_instance_proc_addr: vkGetInstanceProcAddr_t) -> Self {
    Self(get_instance_proc_addr)
  }
}
