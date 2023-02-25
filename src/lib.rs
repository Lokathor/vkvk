#![no_std]
#![allow(unused_imports)]

macro_rules! impl_bitops_for {
  ($name:ident) => {
    // TODO
  };
}
pub(crate) use impl_bitops_for;

mod vk_version_1_0;
pub use vk_version_1_0::*;

#[cfg(feature = "VK_KHR_surface")]
mod vk_khr_surface;
#[cfg(feature = "VK_KHR_surface")]
pub use vk_khr_surface::*;

#[cfg(feature = "VK_KHR_swapchain")]
mod vk_khr_swapchain;
#[cfg(feature = "VK_KHR_swapchain")]
pub use vk_khr_swapchain::*;
