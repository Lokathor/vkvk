//! This module re-exports all the stuff from the crate.
//!
//! In case you want to just pull in the whole crate with one import:
//!
//! ```
//! // unleash the power of a glob import
//! use vkvk::prelude::*;
//! ```

#![allow(nonstandard_style)] // for the PFN definitions.

pub use alloc::{boxed::Box, string::String, vec::Vec};
pub use core::{
  ffi::{c_double, c_float, c_int, c_void},
  num::{NonZeroI32, NonZeroU32},
  ptr::{null, null_mut},
};
pub use raw_vulkan_handle::*;

pub use crate::{
  api_constants::*,
  base_types::*,
  generated::{aliases::*, bitmasks::*, enumerations::*, structures::*, unions::*},
};

// TODO: these type aliases need a better home. They are written by hand so they
// don't go in the `generated` code files, but they also require the generated
// data to work so they can't go into the `base_types` file.

/// non-null version of Khronos: [PFN_vkVoidFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkVoidFunction.html)
pub type vkVoidFunction_t = unsafe extern "system" fn();

/// non-null version of Khronos: [PFN_vkAllocationFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkAllocationFunction.html)
pub type vkAllocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: c_size_t,
  alignment: usize,
  allocation_scope: VkSystemAllocationScope,
) -> *mut c_void;

/// non-null version of Khronos: [PFN_vkReallocationFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkReallocationFunction.html)
pub type vkReallocationFunction_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  original: *mut c_void,
  size: c_size_t,
  alignment: c_size_t,
  allocation_scope: VkSystemAllocationScope,
) -> *mut c_void;

/// non-null version of Khronos: [PFN_vkFreeFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkFreeFunction.html)
pub type vkFreeFunction_t =
  unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);

/// non-null version of Khronos: [PFN_vkInternalAllocationNotification](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalAllocationNotification.html)
pub type vkInternalAllocationNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: c_size_t,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);

/// non-null version of Khronos: [PFN_vkInternalFreeNotification](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalFreeNotification.html)
pub type vkInternalFreeNotification_t = unsafe extern "system" fn(
  user_data: *mut c_void,
  size: c_size_t,
  allocation_type: VkInternalAllocationType,
  allocation_scope: VkSystemAllocationScope,
);

/// non-null version of Khronos: [PFN_vkDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugReportCallbackEXT.html)
pub type vkDebugReportCallbackEXT_t = unsafe extern "system" fn(
  flags: VkDebugReportFlagsEXT,
  object_type: VkDebugReportObjectTypeEXT,
  object: u64,
  location: c_size_t,
  message_code: i32,
  layer_prefix: *const u8,
  message: *const u8,
  user_data: *mut c_void,
) -> VkBool32;

/// non-null version of Khronos: [PFN_vkDebugUtilsMessengerCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html)
pub type vkDebugUtilsMessengerCallbackEXT_t = unsafe extern "system" fn(
  message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
  message_types: VkDebugUtilsMessageTypeFlagsEXT,
  callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
  user_data: *mut c_void,
) -> VkBool32;

/// non-null version of Khronos: [PFN_vkDeviceMemoryReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDeviceMemoryReportCallbackEXT.html)
pub type vkDeviceMemoryReportCallbackEXT_t = unsafe extern "system" fn(
  callback_data: *const VkDeviceMemoryReportCallbackDataEXT,
  user_data: *mut c_void,
);

/// non-null version of Khronos: [PFN_vkFaultCallbackFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkFaultCallbackFunction.html)
pub type vkFaultCallbackFunction_t = unsafe extern "system" fn(
  unrecorded_faults: VkBool32,
  fault_count: u32,
  faults: *const VkFaultData,
);

/// non-null version of Khronos: [PFN_vkGetInstanceProcAddrLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkGetInstanceProcAddrLUNARG.html)
pub type vkGetInstanceProcAddrLUNARG_t =
  unsafe extern "system" fn(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction;

/// Khronos: [PFN_vkVoidFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkVoidFunction.html)
pub type PFN_vkVoidFunction = Option<vkVoidFunction_t>;

/// Khronos: [PFN_vkAllocationFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkAllocationFunction.html)
pub type PFN_vkAllocationFunction = Option<vkAllocationFunction_t>;

/// Khronos: [PFN_vkReallocationFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkReallocationFunction.html)
pub type PFN_vkReallocationFunction = Option<vkReallocationFunction_t>;

/// Khronos: [PFN_vkFreeFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkFreeFunction.html)
pub type PFN_vkFreeFunction = Option<vkFreeFunction_t>;

/// Khronos: [PFN_vkInternalAllocationNotification](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalAllocationNotification.html)
pub type PFN_vkInternalAllocationNotification =
  Option<vkInternalAllocationNotification_t>;

/// Khronos: [PFN_vkInternalFreeNotification](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalFreeNotification.html)
pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification_t>;

/// Khronos: [PFN_vkDebugReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugReportCallbackEXT.html)
pub type PFN_vkDebugReportCallbackEXT = Option<vkDebugReportCallbackEXT_t>;

/// Khronos: [PFN_vkDebugUtilsMessengerCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html)
pub type PFN_vkDebugUtilsMessengerCallbackEXT =
  Option<vkDebugUtilsMessengerCallbackEXT_t>;

/// Khronos: [PFN_vkDeviceMemoryReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDeviceMemoryReportCallbackEXT.html)
pub type PFN_vkDeviceMemoryReportCallbackEXT = Option<vkDeviceMemoryReportCallbackEXT_t>;

/// Khronos: [PFN_vkFaultCallbackFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkFaultCallbackFunction.html)
pub type PFN_vkFaultCallbackFunction = Option<vkFaultCallbackFunction_t>;

/// Khronos: [PFN_vkGetInstanceProcAddrLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkGetInstanceProcAddrLUNARG.html)
pub type PFN_vkGetInstanceProcAddrLUNARG = Option<vkGetInstanceProcAddrLUNARG_t>;
