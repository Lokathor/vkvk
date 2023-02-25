#![no_std]
#![allow(unused_imports)]
#![allow(unused_macros)]

#[macro_use]
mod macros;

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
