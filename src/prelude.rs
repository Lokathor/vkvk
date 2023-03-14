//! This module re-exports all the stuff from the crate.
//!
//! In case you want to just pull in the whole crate with one import:
//!
//! ```
//! // unleash the power of a glob import
//! use vkvk::prelude::*;
//! ```

#![allow(nonstandard_style)] // for the PFN definitions.

pub use crate::{
  api_constants::*,
  base_types::*,
  entry::*,
  generated::{
    aliases::*, bitmasks::*, enumerations::*, ext_consts::*, structures::*, unions::*,
  },
  instance::*,
  physical_device::*,
};
pub use core::{
  ffi::{c_double, c_float, c_int, c_void},
  mem::ManuallyDrop,
  num::{NonZeroI32, NonZeroU32},
  ptr::{null, null_mut},
};
pub use raw_vulkan_handle::*;
pub use zstring::*;

pub(crate) use crate::generated::fn_types::*;
pub(crate) use std::sync::Arc;

// TODO: these type aliases need a better home. They are written by hand so they
// don't go in the `generated` code files, but they also require the generated
// data to work so they can't go into the `base_types` file.

/// Vulkan error codes are non-zero values.
///
/// Generally: negative is unrecoverable, and positive is recoverable.
pub type VkError = NonZeroI32;

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

/// Khronos: [PFN_vkGetInstanceProcAddrLUNARG](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkGetInstanceProcAddrLUNARG.html)
pub type PFN_vkGetInstanceProcAddrLUNARG = Option<vkGetInstanceProcAddrLUNARG_t>;

/// non-null version of Khronos: [PFN_vkFaultCallbackFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkFaultCallbackFunction.html)
#[cfg(feature = "vulkansc")]
pub type vkFaultCallbackFunction_t = unsafe extern "system" fn(
  unrecorded_faults: VkBool32,
  fault_count: u32,
  faults: *const VkFaultData,
);

/// Khronos: [PFN_vkFaultCallbackFunction](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkFaultCallbackFunction.html)
#[cfg(feature = "vulkansc")]
pub type PFN_vkFaultCallbackFunction = Option<vkFaultCallbackFunction_t>;

/// Khronos:
/// [VkClearAttachment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearAttachment.html)
/// (structure)
///
/// The fields of this struct aren't `pub` because they have a slightly complex
/// interaction. Instead, make a [ClearAttachment] value and then call
/// `VkClearAttachment::from(clear_attachment)`.
///
/// ## Safety
/// Unsafe code must take care that that the `clearValue` field (which is a
/// `union`) is properly initialized according to the bits set in `aspectMask`.
/// See [ClearAttachment] for the valid possible combinations.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkClearAttachment {
  /// A mask selecting the color, depth and/or stencil aspects of the attachment
  /// to be cleared. **must not be 0**
  aspect_mask: VkImageAspectFlags,
  /// Only meaningful if `VK_IMAGE_ASPECT_COLOR_BIT` is set in `aspectMask`, in
  /// which case it is an index into the currently bound color attachments.
  color_attachment: u32,
  /// The color or depth/stencil value to clear the attachment to.
  clear_value: VkClearValue,
}
impl Default for VkClearAttachment {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::from(ClearAttachment::default())
  }
}
impl From<ClearAttachment> for VkClearAttachment {
  #[inline]
  #[must_use]
  fn from(value: ClearAttachment) -> Self {
    match value {
      ClearAttachment::Color(colorAttachment, color) => VkClearAttachment {
        aspect_mask: VK_IMAGE_ASPECT_COLOR_BIT,
        color_attachment: colorAttachment,
        clear_value: VkClearValue { color },
      },
      ClearAttachment::Depth(depth) => VkClearAttachment {
        aspect_mask: VK_IMAGE_ASPECT_DEPTH_BIT,
        color_attachment: 0,
        clear_value: VkClearValue {
          depth_stencil: VkClearDepthStencilValue { depth, stencil: 0 },
        },
      },
      ClearAttachment::Stencil(stencil) => VkClearAttachment {
        aspect_mask: VK_IMAGE_ASPECT_STENCIL_BIT,
        color_attachment: 0,
        clear_value: VkClearValue {
          depth_stencil: VkClearDepthStencilValue { depth: 0.0, stencil },
        },
      },
      ClearAttachment::DepthAndStencil(depth, stencil) => VkClearAttachment {
        aspect_mask: VK_IMAGE_ASPECT_DEPTH_BIT | VK_IMAGE_ASPECT_STENCIL_BIT,
        color_attachment: 0,
        clear_value: VkClearValue {
          depth_stencil: VkClearDepthStencilValue { depth, stencil },
        },
      },
    }
  }
}

/// A Rust enum to model the valid forms of [VkClearAttachment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkClearAttachment.html).
#[derive(Clone, Copy)]
pub enum ClearAttachment {
  /// color attachment index and clear value
  Color(u32, VkClearColorValue),
  /// depth clear value
  Depth(c_float),
  /// stencil clear value
  Stencil(u32),
  /// depth clear value *and* stencil clear value
  DepthAndStencil(c_float, u32),
}
impl Default for ClearAttachment {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::Depth(0.0)
  }
}
