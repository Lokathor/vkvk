#![allow(missing_docs)]

//! Various constants defined as part of Vulkan
//!
//! Some of the constants here are meant to be used as the length of an array.
//! In the C API they're typed as `uint32_t`. In this module they've been
//! declared as `usize` instead, so they can be used as a Rust array length
//! without any extra conversion.

use crate::prelude::VkBool32;

/// The maximum number of unique memory heaps, each of which support 1 or more
/// memory types
pub const VK_MAX_MEMORY_HEAPS: usize = 16;

pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;

pub const VK_FALSE: VkBool32 = VkBool32::FALSE;
pub const VK_TRUE: VkBool32 = VkBool32::TRUE;

// unsorted

pub const VK_ATTACHMENT_UNUSED: u32 = !0;
pub const VK_LOD_CLAMP_NONE: core::ffi::c_float = 1000.0;
pub const VK_LUID_SIZE: u32 = 8;
pub const VK_LUID_SIZE_KHR: u32 = VK_LUID_SIZE;
pub const VK_MAX_DEVICE_GROUP_SIZE: u32 = 32;
pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: u32 = VK_MAX_DEVICE_GROUP_SIZE;
pub const VK_MAX_DRIVER_INFO_SIZE: u32 = 256;
pub const VK_MAX_DRIVER_INFO_SIZE_KHR: u32 = VK_MAX_DRIVER_INFO_SIZE;
pub const VK_MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const VK_MAX_DRIVER_NAME_SIZE_KHR: u32 = VK_MAX_DRIVER_NAME_SIZE;
pub const VK_MAX_GLOBAL_PRIORITY_SIZE_EXT: u32 = VK_MAX_GLOBAL_PRIORITY_SIZE_KHR;
pub const VK_MAX_GLOBAL_PRIORITY_SIZE_KHR: u32 = 16;
pub const VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = !1;
pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: u32 = VK_QUEUE_FAMILY_EXTERNAL;
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = !2;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;
pub const VK_REMAINING_3D_SLICES_EXT: u32 = !0;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0;
pub const VK_REMAINING_MIP_LEVELS: u32 = !0;
pub const VK_SHADER_UNUSED_KHR: u32 = !0;
pub const VK_SHADER_UNUSED_NV: u32 = VK_SHADER_UNUSED_KHR;
pub const VK_SUBPASS_EXTERNAL: u32 = !0;
pub const VK_WHOLE_SIZE: u64 = !0;
