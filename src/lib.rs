#![no_std]
#![allow(dead_code)]
#![warn(clippy::missing_inline_in_public_items)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

use core::{
  ffi::*,
  ptr::{null, null_mut},
};

mod array_zstr;
pub use array_zstr::*;

mod vk_version;
pub use vk_version::*;

mod vk_bool;
pub use vk_bool::*;

mod vk_result;
pub use vk_result::*;

mod handles;
pub use handles::*;

mod flag_bits;
pub use flag_bits::*;

mod enumerations;
pub use enumerations::*;

mod structs;
pub use structs::*;

mod fn_types;
use fn_types::*;

mod instance;
pub use instance::*;

mod entry;
pub use entry::*;

mod device;
pub use device::*;

const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
const VK_MAX_DESCRIPTION_SIZE: usize = 256;
const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
const VK_UUID_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct VkDeviceSize(pub u64);

use handwritten_types::*;
mod handwritten_types {
  #![allow(nonstandard_style)]

  use super::*;

  pub(crate) type uint8_t = u8;
  pub(crate) type uint32_t = u32;
  pub(crate) type int32_t = i32;
  pub(crate) type size_t = usize;
  pub(crate) type float = c_float;

  pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn()>;
  pub type vkAllocationFunction_t = unsafe extern "system" fn(
    user_data: *mut c_void,
    size: size_t,
    alignment: size_t,
    allocation_scope: VkSystemAllocationScope,
  );
  pub type vkReallocationFunction_t = unsafe extern "system" fn(
    user_data: *mut c_void,
    original: *mut c_void,
    size: size_t,
    alignment: size_t,
    allocation_scope: VkSystemAllocationScope,
  );
  pub type vkFreeFunction_t =
    unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
  pub type vkInternalAllocationNotification_t = unsafe extern "system" fn(
    user_data: *mut c_void,
    size: size_t,
    allocation_type: VkInternalAllocationType,
    allocation_scope: VkSystemAllocationScope,
  );
  pub type vkInternalFreeNotification_t = unsafe extern "system" fn(
    user_data: *mut c_void,
    size: size_t,
    allocation_type: VkInternalAllocationType,
    allocation_scope: VkSystemAllocationScope,
  );
  pub type PFN_vkAllocationFunction = Option<vkAllocationFunction_t>;
  pub type PFN_vkReallocationFunction = Option<vkReallocationFunction_t>;
  pub type PFN_vkFreeFunction = Option<vkFreeFunction_t>;
  pub type PFN_vkInternalAllocationNotification = Option<vkInternalAllocationNotification_t>;
  pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification_t>;
}
