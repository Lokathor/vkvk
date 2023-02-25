//! Vulkan core API interface definitions

macro_rules! impl_bitops_for {
  ($name:ident) => {
    // TODO
  };
}

#[allow(non_camel_case_types)]
pub(crate) type uint8_t = u8;

#[allow(non_camel_case_types)]
pub(crate) type int32_t = i32;

#[allow(non_camel_case_types)]
pub(crate) type uint32_t = u32;

#[allow(non_camel_case_types)]
pub(crate) type size_t = usize;

#[allow(non_camel_case_types)]
pub(crate) type float = core::ffi::c_float;

#[allow(non_camel_case_types)]
pub(crate) type void = core::ffi::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkBool32(u32);
impl VkBool32 {
  pub const FALSE: Self = Self(false as _);
  pub const TRUE: Self = Self(true as _);
}
impl From<bool> for VkBool32 {
  #[inline]
  #[must_use]
  fn from(value: bool) -> Self {
    Self(value as _)
  }
}
impl From<VkBool32> for bool {
  #[inline]
  #[must_use]
  fn from(value: VkBool32) -> Self {
    value.0 != 0
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDeviceAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDeviceSize(pub u64);

/// Khronos: [VkExtent2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtent2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtent2D {
  pub width: uint32_t,
  pub height: uint32_t,
}

/// Khronos: [VkExtent3D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtent3D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtent3D {
  pub width: uint32_t,
  pub height: uint32_t,
  pub depth: uint32_t,
}

/// Khronos: [VkOffset2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOffset2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOffset2D {
  pub x: int32_t,
  pub y: int32_t,
}

/// Khronos: [VkOffset3D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkOffset3D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkOffset3D {
  pub x: int32_t,
  pub y: int32_t,
  pub z: int32_t,
}

/// Khronos: [VkRect2D](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRect2D.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
}

/// Khronos: [VkResult](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResult.html)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct VkResult(pub(crate) Option<core::num::NonZeroI32>);

/// Khronos: [VkStructureType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStructureType.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkStructureType(pub u32);

pub const VK_ATTACHMENT_UNUSED: u32 = u32::MAX;
pub const VK_FALSE: VkBool32 = VkBool32::FALSE;
pub const VK_LOD_CLAMP_NONE: core::ffi::c_float = 1000.0;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = u32::MAX;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = u32::MAX;
pub const VK_REMAINING_MIP_LEVELS: u32 = u32::MAX;
pub const VK_SUBPASS_EXTERNAL: u32 = u32::MAX;
pub const VK_TRUE: VkBool32 = VkBool32::TRUE;
pub const VK_WHOLE_SIZE: u64 = u64::MAX;
pub const VK_MAX_MEMORY_TYPES: u32 = 32;
pub const VK_MAX_EXTENSION_NAME_SIZE: u32 = 8;
pub const VK_MAX_MEMORY_HEAPS: u32 = 16;
//
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;

/// Khronos: [VkPipelineCacheHeaderVersion](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersion.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineCacheHeaderVersion(pub u32);

/// Khronos: [VkBaseInStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseInStructure.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseInStructure {
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const VkBaseInStructure,
}

/// Khronos: [VkBaseOutStructure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBaseOutStructure.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBaseOutStructure {
  pub ty: VkStructureType,
  /// * Optional
  pub next: *mut VkBaseOutStructure,
}

/// Khronos: [VkAccessFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkAccessFlagBits(pub u32);
impl VkAccessFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkAccessFlagBits);

/// Khronos: [VkAccessFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html)
pub type VkAccessFlags = VkAccessFlagBits;

/// Khronos: [VkBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuffer.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_BUFFER`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkBuffer(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkBuffer {}
unsafe impl Sync for VkBuffer {}
impl VkBuffer {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    #[cfg(target_pointer_width = "64")]
    return Self(core::ptr::null_mut());
    #[cfg(not(target_pointer_width = "64"))]
    return Self(0);
  }
}
impl Default for VkBuffer {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}

/// Khronos: [VkBufferMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferMemoryBarrier {
  /// * Values: [`VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * No Auto Validity
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * No Auto Validity
  pub dst_access_mask: VkAccessFlags,
  /// Queue family to transition ownership from
  pub src_queue_family_index: uint32_t,
  /// Queue family to transition ownership to
  pub dst_queue_family_index: uint32_t,
  /// Buffer to sync
  pub buffer: VkBuffer,
  /// Offset within the buffer to sync
  pub offset: VkDeviceSize,
  /// Amount of bytes to sync
  pub size: VkDeviceSize,
}
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: VkStructureType = VkStructureType(44);

/// Khronos: [VkDispatchIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDispatchIndirectCommand.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDispatchIndirectCommand {
  /// * No Auto Validity
  pub x: uint32_t,
  /// * No Auto Validity
  pub y: uint32_t,
  /// * No Auto Validity
  pub z: uint32_t,
}

/// Khronos: [VkDrawIndexedIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawIndexedIndirectCommand.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
  pub index_count: uint32_t,
  pub instance_count: uint32_t,
  pub first_index: uint32_t,
  pub vertex_offset: int32_t,
  /// * No Auto Validity
  pub first_instance: uint32_t,
}

/// Khronos: [VkDrawIndirectCommand](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrawIndirectCommand.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDrawIndirectCommand {
  pub vertex_count: uint32_t,
  pub instance_count: uint32_t,
  pub first_vertex: uint32_t,
  /// * No Auto Validity
  pub first_instance: uint32_t,
}

/// Khronos: [VkImageLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageLayout.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageLayout(pub u32);

/// Khronos: [VkImage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImage.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_IMAGE`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkImage(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkImage {}
unsafe impl Sync for VkImage {}
impl VkImage {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    #[cfg(target_pointer_width = "64")]
    return Self(core::ptr::null_mut());
    #[cfg(not(target_pointer_width = "64"))]
    return Self(0);
  }
}
impl Default for VkImage {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}

/// Khronos: [VkImageAspectFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageAspectFlagBits(pub u32);
impl VkImageAspectFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkImageAspectFlagBits);

/// Khronos: [VkImageAspectFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html)
pub type VkImageAspectFlags = VkImageAspectFlagBits;

/// Khronos: [VkImageSubresourceRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceRange.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresourceRange {
  pub aspect_mask: VkImageAspectFlags,
  pub base_mip_level: uint32_t,
  pub level_count: uint32_t,
  pub base_array_layer: uint32_t,
  pub layer_count: uint32_t,
}

/// Khronos: [VkImageMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageMemoryBarrier {
  /// * Values: [`VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * No Auto Validity
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * No Auto Validity
  pub dst_access_mask: VkAccessFlags,
  /// Current layout of the image
  pub old_layout: VkImageLayout,
  /// New layout to transition the image to
  pub new_layout: VkImageLayout,
  /// Queue family to transition ownership from
  pub src_queue_family_index: uint32_t,
  /// Queue family to transition ownership to
  pub dst_queue_family_index: uint32_t,
  /// Image to sync
  pub image: VkImage,
  /// Subresource range to sync
  pub subresource_range: VkImageSubresourceRange,
}
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: VkStructureType = VkStructureType(45);

/// Khronos: [VkMemoryBarrier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryBarrier {
  /// * Values: [`VK_STRUCTURE_TYPE_MEMORY_BARRIER`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Memory accesses from the source of the dependency to synchronize
  /// * Optional
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * Optional
  pub dst_access_mask: VkAccessFlags,
}
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: VkStructureType = VkStructureType(46);

/// Khronos: [VkObjectType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkObjectType.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkObjectType(pub u32);

/// Khronos: [VkPipelineCacheHeaderVersionOne](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersionOne.html)
///
/// The fields in this structure are non-normative since structure packing is
/// implementation-defined in C. The specification defines the normative layout.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheHeaderVersionOne {
  pub header_size: uint32_t,
  pub header_version: VkPipelineCacheHeaderVersion,
  pub vendor_id: uint32_t, // TODO: should we make this into VkVendorId?
  pub device_id: uint32_t,
  pub pipeline_cache_uuid: *const [uint8_t; VK_UUID_SIZE],
}

/// Khronos: [VkVendorId](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVendorId.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkVendorId(pub u32);

pub use vk_version::*;
mod vk_version {
  /// A vulkan version value.
  ///
  /// Vulkan uses an `x.y.z` version system, with the addition that an
  /// implementation can have a non-zero "variant" value to indicate that it is
  /// non-standard in some way.
  #[derive(Clone, Copy, Default, PartialEq, Eq)]
  #[repr(transparent)]
  pub struct VkVersion(pub u32);
  #[allow(missing_docs)]
  impl VkVersion {
    pub const API_1_0: Self = Self::major_minor_patch(1, 0, 0);
    pub const API_1_1: Self = Self::major_minor_patch(1, 1, 0);
    pub const API_1_2: Self = Self::major_minor_patch(1, 2, 0);
    pub const API_1_3: Self = Self::major_minor_patch(1, 3, 0);
    //
    pub const API_SC_1_0: Self = Self::variant_major_minor_patch(1, 1, 3, 0);
    //
    pub const HEADER: Self = Self::major_minor_patch(1, 3, 241);

    #[inline]
    #[must_use]
    pub const fn patch(self) -> u32 {
      u32_get_value(0, 11, self.0)
    }
    #[inline]
    #[must_use]
    pub const fn minor(self) -> u32 {
      u32_get_value(12, 21, self.0)
    }
    #[inline]
    #[must_use]
    pub const fn major(self) -> u32 {
      u32_get_value(22, 28, self.0)
    }
    #[inline]
    #[must_use]
    pub const fn variant(self) -> u32 {
      u32_get_value(29, 31, self.0)
    }
    #[inline]
    #[must_use]
    pub const fn major_minor_patch(major: u32, minor: u32, patch: u32) -> Self {
      let patch_and_minor = u32_with_value(12, 21, patch, minor);
      Self(u32_with_value(22, 28, patch_and_minor, major))
    }
    #[inline]
    #[must_use]
    pub const fn variant_major_minor_patch(
      variant: u32, major: u32, minor: u32, patch: u32,
    ) -> Self {
      let patch_and_minor = u32_with_value(12, 21, patch, minor);
      let major_and_minor_and_patch = u32_with_value(22, 28, patch_and_minor, major);
      Self(u32_with_value(29, 31, major_and_minor_and_patch, variant))
    }
  }
  impl core::fmt::Debug for VkVersion {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      let major = self.major();
      let minor = self.minor();
      let patch = self.patch();
      write!(f, "{major}.{minor}.{patch}")?;
      match self.variant() {
        0 => Ok(()),
        variant => write!(f, " [variant {variant}]"),
      }
    }
  }
  impl core::fmt::Display for VkVersion {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      core::fmt::Debug::fmt(self, f)
    }
  }
  impl core::cmp::PartialOrd for VkVersion {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
      use core::cmp::Ordering;
      if self.variant() != other.variant() {
        return None;
      }
      Some(match self.major().cmp(&other.major()) {
        Ordering::Equal => match self.minor().cmp(&other.minor()) {
          Ordering::Equal => self.patch().cmp(&other.patch()),
          otherwise => otherwise,
        },
        otherwise => otherwise,
      })
    }
  }

  // Pulled from the bitfrob crate and just pasted in here to avoid having a full
  // dependency.

  #[inline]
  #[must_use]
  const fn u32_get_value(low: u32, high: u32, u: u32) -> u32 {
    u32_get_region(0, high - low, u >> low)
  }
  #[inline]
  #[must_use]
  const fn u32_get_region(low: u32, high: u32, u: u32) -> u32 {
    let mask = u32_region_mask(low, high);
    u & mask
  }
  #[inline]
  #[must_use]
  const fn u32_with_value(low: u32, high: u32, old: u32, replacement: u32) -> u32 {
    u32_with_region(low, high, old, replacement << low)
  }
  #[inline]
  #[must_use]
  const fn u32_with_region(low: u32, high: u32, old: u32, replacement: u32) -> u32 {
    let mask = u32_region_mask(low, high);
    (old & !mask) | (replacement & mask)
  }
  #[inline]
  #[must_use]
  const fn u32_region_mask(low: u32, high: u32) -> u32 {
    assert!(low < <u32>::BITS);
    assert!(high < <u32>::BITS);
    assert!(low < high);
    (<u32>::MAX >> ((<u32>::BITS - 1) - (high - low))) << low
  }
}

/// Khronos: [VkSystemAllocationScope](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSystemAllocationScope.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSystemAllocationScope(pub u32);

/// Khronos: [VkInternalAllocationType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInternalAllocationType.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkInternalAllocationType(pub u32);

pub use manual_fn_tys::*;
#[allow(nonstandard_style)]
mod manual_fn_tys {
  use super::*;
  use core::ffi::c_void;

  pub type vkVoidFunction_t = unsafe extern "system" fn();
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
  pub type PFN_vkVoidFunction = Option<vkVoidFunction_t>;
  pub type PFN_vkAllocationFunction = Option<vkAllocationFunction_t>;
  pub type PFN_vkReallocationFunction = Option<vkReallocationFunction_t>;
  pub type PFN_vkFreeFunction = Option<vkFreeFunction_t>;
  pub type PFN_vkInternalAllocationNotification = Option<vkInternalAllocationNotification_t>;
  pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification_t>;
}

/// Khronos: [VkAllocationCallbacks](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAllocationCallbacks.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkAllocationCallbacks {
  /// * Optional
  pub user_data: *mut void,
  /// * No Auto Validity
  pub allocation: PFN_vkAllocationFunction,
  /// * No Auto Validity
  pub reallocation: PFN_vkReallocationFunction,
  /// * No Auto Validity
  pub free: PFN_vkFreeFunction,
  /// * Optional
  /// * No Auto Validity
  pub internal_allocation: PFN_vkInternalAllocationNotification,
  /// * Optional
  /// * No Auto Validity
  pub internal_free: PFN_vkInternalFreeNotification,
}

/// Khronos: [VkApplicationInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkApplicationInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkApplicationInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_APPLICATION_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Len: `null-terminated`
  /// * Optional
  pub application_name: *const u8,
  pub application_version: uint32_t,
  /// * Len: `null-terminated`
  /// * Optional
  pub engine_name: *const u8,
  pub engine_version: uint32_t,
  pub api_version: VkVersion,
}
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = VkStructureType(0);

/// Khronos: [VkFormat](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormat.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkFormat(pub u32);

/// Khronos: [VkFormatFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkFormatFeatureFlagBits(pub u32);
impl VkFormatFeatureFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkFormatFeatureFlagBits);

/// Khronos: [VkFormatFeatureFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html)
pub type VkFormatFeatureFlags = VkFormatFeatureFlagBits;

/// Khronos: [VkFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormatProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFormatProperties {
  /// Format features in case of linear tiling
  /// * Limit Type: bitmask
  /// * Optional
  pub linear_tiling_features: VkFormatFeatureFlags,
  /// Format features in case of optimal tiling
  /// * Limit Type: bitmask
  /// * Optional
  pub optimal_tiling_features: VkFormatFeatureFlags,
  /// Format features supported by buffers
  /// * Limit Type: bitmask
  /// * Optional
  pub buffer_features: VkFormatFeatureFlags,
}

/// Khronos: [VkImageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageCreateFlagBits(pub u32);
impl VkImageCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkImageCreateFlagBits);

/// Khronos: [VkImageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html)
pub type VkImageCreateFlags = VkImageCreateFlagBits;

/// Khronos: [VkImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageFormatProperties {
  /// max image dimensions for this resource type
  pub max_extent: VkExtent3D,
  /// max number of mipmap levels for this resource type
  pub max_mip_levels: uint32_t,
  /// max array size for this resource type
  pub max_array_layers: uint32_t,
  /// supported sample counts for this resource type
  /// * Optional
  pub sample_counts: VkSampleCountFlags,
  /// max size (in bytes) of this resource type
  pub max_resource_size: VkDeviceSize,
}

/// Khronos: [VkSampleCountFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSampleCountFlagBits(pub u32);
impl VkSampleCountFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkSampleCountFlagBits);

/// Khronos: [VkSampleCountFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html)
pub type VkSampleCountFlags = VkSampleCountFlagBits;

/// Khronos: [VkImageTiling](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageTiling.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageTiling(pub u32);

/// Khronos: [VkImageType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageType.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageType(pub u32);

/// Khronos: [VkImageUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageUsageFlagBits(pub u32);
impl VkImageUsageFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkImageUsageFlagBits);

/// Khronos: [VkImageUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html)
pub type VkImageUsageFlags = VkImageUsageFlagBits;

/// Khronos: [VkInstance](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstance.html) (handle)
/// * Object Type Enum: [`VK_OBJECT_TYPE_INSTANCE`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkInstance(*mut core::ffi::c_void);
unsafe impl Send for VkInstance {}
unsafe impl Sync for VkInstance {}
impl Default for VkInstance {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkInstance {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}
pub const VK_OBJECT_TYPE_INSTANCE: VkObjectType = VkObjectType(1);

/// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkInstanceCreateFlagBits(pub u32);
impl VkInstanceCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkInstanceCreateFlagBits);

/// Khronos: [VkInstanceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html)
pub type VkInstanceCreateFlags = VkInstanceCreateFlagBits;

/// Khronos: [VkInstanceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkInstanceCreateFlags,
  /// * Optional
  pub application_info: *const VkApplicationInfo,
  /// * Optional
  pub enabled_layer_count: uint32_t,
  /// Ordered list of layer names to be enabled
  /// * Len: `enabledLayerCount,null-terminated`
  pub enabled_layer_names: *const *const u8,
  /// * Optional
  pub enabled_extension_count: uint32_t,
  /// Extension names to be enabled
  /// * Len: `enabledExtensionCount,null-terminated`
  pub enabled_extension_names: *const *const u8,
}
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = VkStructureType(1);

/// Khronos: [VkMemoryHeapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkMemoryHeapFlagBits(pub u32);
impl VkMemoryHeapFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkMemoryHeapFlagBits);

/// Khronos: [VkMemoryHeapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html)
pub type VkMemoryHeapFlags = VkMemoryHeapFlagBits;

/// Khronos: [VkMemoryHeap](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHeap.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryHeap {
  /// Available memory in the heap
  pub size: VkDeviceSize,
  /// Flags for the heap
  /// * Optional
  pub flags: VkMemoryHeapFlags,
}

/// Khronos: [VkMemoryPropertyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkMemoryPropertyFlagBits(pub u32);
impl VkMemoryPropertyFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkMemoryPropertyFlagBits);

/// Khronos: [VkMemoryPropertyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html)
pub type VkMemoryPropertyFlags = VkMemoryPropertyFlagBits;

/// Khronos: [VkMemoryType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryType.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryType {
  /// Memory properties of this memory type
  /// * Optional
  pub property_flags: VkMemoryPropertyFlags,
  /// Index of the memory heap allocations of this memory type are taken from
  pub heap_index: uint32_t,
}

/// Khronos: [VkPhysicalDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice.html) (handle)
/// * Parent: [VkInstance]
/// * Object Type Enum: [`VK_OBJECT_TYPE_PHYSICAL_DEVICE`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPhysicalDevice(*mut core::ffi::c_void);
unsafe impl Send for VkPhysicalDevice {}
unsafe impl Sync for VkPhysicalDevice {}
impl Default for VkPhysicalDevice {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkPhysicalDevice {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}
pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: VkObjectType = VkObjectType(2);

/// Khronos: [VkPhysicalDeviceFeatures](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
  /// out of bounds buffer accesses are well defined
  pub robust_buffer_access: VkBool32,
  /// full 32-bit range of indices for indexed draw calls
  pub full_draw_index_uint32: VkBool32,
  /// image views which are arrays of cube maps
  pub image_cube_array: VkBool32,
  /// blending operations are controlled per-attachment
  pub independent_blend: VkBool32,
  /// geometry stage
  pub geometry_shader: VkBool32,
  /// tessellation control and evaluation stage
  pub tessellation_shader: VkBool32,
  /// per-sample shading and interpolation
  pub sample_rate_shading: VkBool32,
  /// blend operations which take two sources
  pub dual_src_blend: VkBool32,
  /// logic operations
  pub logic_op: VkBool32,
  /// multi draw indirect
  pub multi_draw_indirect: VkBool32,
  /// indirect drawing can use non-zero firstInstance
  pub draw_indirect_first_instance: VkBool32,
  /// depth clamping
  pub depth_clamp: VkBool32,
  /// depth bias clamping
  pub depth_bias_clamp: VkBool32,
  /// point and wireframe fill modes
  pub fill_mode_non_solid: VkBool32,
  /// depth bounds test
  pub depth_bounds: VkBool32,
  /// lines with width greater than 1
  pub wide_lines: VkBool32,
  /// points with size greater than 1
  pub large_points: VkBool32,
  /// the fragment alpha component can be forced to maximum representable alpha
  /// value
  pub alpha_to_one: VkBool32,
  /// viewport arrays
  pub multi_viewport: VkBool32,
  /// anisotropic sampler filtering
  pub sampler_anisotropy: VkBool32,
  /// ETC texture compression formats
  pub texture_compression_etc2: VkBool32,
  /// ASTC LDR texture compression formats
  pub texture_compression_astc_ldr: VkBool32,
  /// BC1-7 texture compressed formats
  pub texture_compression_bc: VkBool32,
  /// precise occlusion queries returning actual sample counts
  pub occlusion_query_precise: VkBool32,
  /// pipeline statistics query
  pub pipeline_statistics_query: VkBool32,
  /// stores and atomic ops on storage buffers and images are supported in
  /// vertex, tessellation, and geometry stages
  pub vertex_pipeline_stores_and_atomics: VkBool32,
  /// stores and atomic ops on storage buffers and images are supported in the
  /// fragment stage
  pub fragment_stores_and_atomics: VkBool32,
  /// tessellation and geometry stages can export point size
  pub shader_tessellation_and_geometry_point_size: VkBool32,
  /// image gather with run-time values and independent offsets
  pub shader_image_gather_extended: VkBool32,
  /// the extended set of formats can be used for storage images
  pub shader_storage_image_extended_formats: VkBool32,
  /// multisample images can be used for storage images
  pub shader_storage_image_multisample: VkBool32,
  /// read from storage image does not require format qualifier
  pub shader_storage_image_read_without_format: VkBool32,
  /// write to storage image does not require format qualifier
  pub shader_storage_image_write_without_format: VkBool32,
  /// arrays of uniform buffers can be accessed with dynamically uniform indices
  pub shader_uniform_buffer_array_dynamic_indexing: VkBool32,
  /// arrays of sampled images can be accessed with dynamically uniform indices
  pub shader_sampled_image_array_dynamic_indexing: VkBool32,
  /// arrays of storage buffers can be accessed with dynamically uniform indices
  pub shader_storage_buffer_array_dynamic_indexing: VkBool32,
  /// arrays of storage images can be accessed with dynamically uniform indices
  pub shader_storage_image_array_dynamic_indexing: VkBool32,
  /// clip distance in shaders
  pub shader_clip_distance: VkBool32,
  /// cull distance in shaders
  pub shader_cull_distance: VkBool32,
  /// 64-bit floats (doubles) in shaders
  pub shader_float64: VkBool32,
  /// 64-bit integers in shaders
  pub shader_int64: VkBool32,
  /// 16-bit integers in shaders
  pub shader_int16: VkBool32,
  /// shader can use texture operations that return resource residency
  /// information (requires sparseNonResident support)
  pub shader_resource_residency: VkBool32,
  /// shader can use texture operations that specify minimum resource LOD
  pub shader_resource_min_lod: VkBool32,
  /// Sparse resources support: Resource memory can be managed at opaque page
  /// level rather than object level
  pub sparse_binding: VkBool32,
  /// Sparse resources support: GPU can access partially resident buffers
  pub sparse_residency_buffer: VkBool32,
  /// Sparse resources support: GPU can access partially resident 2D (non-MSAA
  /// non-depth/stencil) images
  pub sparse_residency_image_2d: VkBool32,
  /// Sparse resources support: GPU can access partially resident 3D images
  pub sparse_residency_image_3d: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 2 samples
  pub sparse_residency_2_samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 4 samples
  pub sparse_residency_4_samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 8 samples
  pub sparse_residency_8_samples: VkBool32,
  /// Sparse resources support: GPU can access partially resident MSAA 2D images
  /// with 16 samples
  pub sparse_residency_16_samples: VkBool32,
  /// Sparse resources support: GPU can correctly access data aliased into
  /// multiple locations (opt-in)
  pub sparse_residency_aliased: VkBool32,
  /// multisample rate must be the same for all pipelines in a subpass
  pub variable_multisample_rate: VkBool32,
  /// Queries may be inherited from primary to secondary command buffers
  pub inherited_queries: VkBool32,
}

/// Khronos: [VkPhysicalDeviceLimits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLimits.html)
///
/// compute stage limits
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
  /// max 1D image dimension
  /// * Limit Type: max
  pub max_image_dimension_1d: uint32_t,
  /// max 2D image dimension
  /// * Limit Type: max
  pub max_image_dimension_2d: uint32_t,
  /// max 3D image dimension
  /// * Limit Type: max
  pub max_image_dimension_3d: uint32_t,
  /// max cubemap image dimension
  /// * Limit Type: max
  pub max_image_dimension_cube: uint32_t,
  /// max layers for image arrays
  /// * Limit Type: max
  pub max_image_array_layers: uint32_t,
  /// max texel buffer size (fstexels)
  /// * Limit Type: max
  pub max_texel_buffer_elements: uint32_t,
  /// max uniform buffer range (bytes)
  /// * Limit Type: max
  pub max_uniform_buffer_range: uint32_t,
  /// max storage buffer range (bytes)
  /// * Limit Type: max
  pub max_storage_buffer_range: uint32_t,
  /// max size of the push constants pool (bytes)
  /// * Limit Type: max
  pub max_push_constants_size: uint32_t,
  /// max number of device memory allocations supported
  /// * Limit Type: max
  pub max_memory_allocation_count: uint32_t,
  /// max number of samplers that can be allocated on a device
  /// * Limit Type: max
  pub max_sampler_allocation_count: uint32_t,
  /// Granularity (in bytes) at which buffers and images can be bound to
  /// adjacent memory for simultaneous usage
  /// * Limit Type: min,mul
  pub buffer_image_granularity: VkDeviceSize,
  /// Total address space available for sparse allocations (bytes)
  /// * Limit Type: max
  pub sparse_address_space_size: VkDeviceSize,
  /// max number of descriptors sets that can be bound to a pipeline
  /// * Limit Type: max
  pub max_bound_descriptor_sets: uint32_t,
  /// max number of samplers allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_samplers: uint32_t,
  /// max number of uniform buffers allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_uniform_buffers: uint32_t,
  /// max number of storage buffers allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_storage_buffers: uint32_t,
  /// max number of sampled images allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_sampled_images: uint32_t,
  /// max number of storage images allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_storage_images: uint32_t,
  /// max number of input attachments allowed per-stage in a descriptor set
  /// * Limit Type: max
  pub max_per_stage_descriptor_input_attachments: uint32_t,
  /// max number of resources allowed by a single stage
  /// * Limit Type: max
  pub max_per_stage_resources: uint32_t,
  /// max number of samplers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_samplers: uint32_t,
  /// max number of uniform buffers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_uniform_buffers: uint32_t,
  /// max number of dynamic uniform buffers allowed in all stages in a
  /// descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_uniform_buffers_dynamic: uint32_t,
  /// max number of storage buffers allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_storage_buffers: uint32_t,
  /// max number of dynamic storage buffers allowed in all stages in a
  /// descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_storage_buffers_dynamic: uint32_t,
  /// max number of sampled images allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_sampled_images: uint32_t,
  /// max number of storage images allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_storage_images: uint32_t,
  /// max number of input attachments allowed in all stages in a descriptor set
  /// * Limit Type: max
  pub max_descriptor_set_input_attachments: uint32_t,
  /// max number of vertex input attribute slots
  /// * Limit Type: max
  pub max_vertex_input_attributes: uint32_t,
  /// max number of vertex input binding slots
  /// * Limit Type: max
  pub max_vertex_input_bindings: uint32_t,
  /// max vertex input attribute offset added to vertex buffer offset
  /// * Limit Type: max
  pub max_vertex_input_attribute_offset: uint32_t,
  /// max vertex input binding stride
  /// * Limit Type: max
  pub max_vertex_input_binding_stride: uint32_t,
  /// max number of output components written by vertex shader
  /// * Limit Type: max
  pub max_vertex_output_components: uint32_t,
  /// max level supported by tessellation primitive generator
  /// * Limit Type: max
  pub max_tessellation_generation_level: uint32_t,
  /// max patch size (vertices)
  /// * Limit Type: max
  pub max_tessellation_patch_size: uint32_t,
  /// max number of input components per-vertex in TCS
  /// * Limit Type: max
  pub max_tessellation_control_per_vertex_input_components: uint32_t,
  /// max number of output components per-vertex in TCS
  /// * Limit Type: max
  pub max_tessellation_control_per_vertex_output_components: uint32_t,
  /// max number of output components per-patch in TCS
  /// * Limit Type: max
  pub max_tessellation_control_per_patch_output_components: uint32_t,
  /// max total number of per-vertex and per-patch output components in TCS
  /// * Limit Type: max
  pub max_tessellation_control_total_output_components: uint32_t,
  /// max number of input components per vertex in TES
  /// * Limit Type: max
  pub max_tessellation_evaluation_input_components: uint32_t,
  /// max number of output components per vertex in TES
  /// * Limit Type: max
  pub max_tessellation_evaluation_output_components: uint32_t,
  /// max invocation count supported in geometry shader
  /// * Limit Type: max
  pub max_geometry_shader_invocations: uint32_t,
  /// max number of input components read in geometry stage
  /// * Limit Type: max
  pub max_geometry_input_components: uint32_t,
  /// max number of output components written in geometry stage
  /// * Limit Type: max
  pub max_geometry_output_components: uint32_t,
  /// max number of vertices that can be emitted in geometry stage
  /// * Limit Type: max
  pub max_geometry_output_vertices: uint32_t,
  /// max total number of components (all vertices) written in geometry stage
  /// * Limit Type: max
  pub max_geometry_total_output_components: uint32_t,
  /// max number of input components read in fragment stage
  /// * Limit Type: max
  pub max_fragment_input_components: uint32_t,
  /// max number of output attachments written in fragment stage
  /// * Limit Type: max
  pub max_fragment_output_attachments: uint32_t,
  /// max number of output attachments written when using dual source blending
  /// * Limit Type: max
  pub max_fragment_dual_src_attachments: uint32_t,
  /// max total number of storage buffers, storage images and output buffers
  /// * Limit Type: max
  pub max_fragment_combined_output_resources: uint32_t,
  /// max total storage size of work group local storage (bytes)
  /// * Limit Type: max
  pub max_compute_shared_memory_size: uint32_t,
  /// max num of compute work groups that may be dispatched by a single command
  /// (x,y,z)
  /// * Limit Type: max
  pub max_compute_work_group_count: [uint32_t; 3],
  /// max total compute invocations in a single local work group
  /// * Limit Type: max
  pub max_compute_work_group_invocations: uint32_t,
  /// max local size of a compute work group (x,y,z)
  /// * Limit Type: max
  pub max_compute_work_group_size: [uint32_t; 3],
  /// number bits of subpixel precision in screen x and y
  /// * Limit Type: bits
  pub sub_pixel_precision_bits: uint32_t,
  /// number bits of precision for selecting texel weights
  /// * Limit Type: bits
  pub sub_texel_precision_bits: uint32_t,
  /// number bits of precision for selecting mipmap weights
  /// * Limit Type: bits
  pub mipmap_precision_bits: uint32_t,
  /// max index value for indexed draw calls (for 32-bit indices)
  /// * Limit Type: max
  pub max_draw_indexed_index_value: uint32_t,
  /// max draw count for indirect drawing calls
  /// * Limit Type: max
  pub max_draw_indirect_count: uint32_t,
  /// max absolute sampler LOD bias
  /// * Limit Type: max
  pub max_sampler_lod_bias: float,
  /// max degree of sampler anisotropy
  /// * Limit Type: max
  pub max_sampler_anisotropy: float,
  /// max number of active viewports
  /// * Limit Type: max
  pub max_viewports: uint32_t,
  /// max viewport dimensions (x,y)
  /// * Limit Type: max
  pub max_viewport_dimensions: [uint32_t; 2],
  /// viewport bounds range (min,max)
  /// * Limit Type: range
  pub viewport_bounds_range: [float; 2],
  /// number bits of subpixel precision for viewport
  /// * Limit Type: bits
  pub viewport_sub_pixel_bits: uint32_t,
  /// min required alignment of pointers returned by MapMemory (bytes)
  /// * Limit Type: min,pot
  pub min_memory_map_alignment: size_t,
  /// min required alignment for texel buffer offsets (bytes)
  /// * Limit Type: min,pot
  pub min_texel_buffer_offset_alignment: VkDeviceSize,
  /// min required alignment for uniform buffer sizes and offsets (bytes)
  /// * Limit Type: min,pot
  pub min_uniform_buffer_offset_alignment: VkDeviceSize,
  /// min required alignment for storage buffer offsets (bytes)
  /// * Limit Type: min,pot
  pub min_storage_buffer_offset_alignment: VkDeviceSize,
  /// min texel offset for OpTextureSampleOffset
  /// * Limit Type: min
  pub min_texel_offset: int32_t,
  /// max texel offset for OpTextureSampleOffset
  /// * Limit Type: max
  pub max_texel_offset: uint32_t,
  /// min texel offset for OpTextureGatherOffset
  /// * Limit Type: min
  pub min_texel_gather_offset: int32_t,
  /// max texel offset for OpTextureGatherOffset
  /// * Limit Type: max
  pub max_texel_gather_offset: uint32_t,
  /// furthest negative offset for interpolateAtOffset
  /// * Limit Type: min
  pub min_interpolation_offset: float,
  /// furthest positive offset for interpolateAtOffset
  /// * Limit Type: max
  pub max_interpolation_offset: float,
  /// number of subpixel bits for interpolateAtOffset
  /// * Limit Type: bits
  pub sub_pixel_interpolation_offset_bits: uint32_t,
  /// max width for a framebuffer
  /// * Limit Type: max
  pub max_framebuffer_width: uint32_t,
  /// max height for a framebuffer
  /// * Limit Type: max
  pub max_framebuffer_height: uint32_t,
  /// max layer count for a layered framebuffer
  /// * Limit Type: max
  pub max_framebuffer_layers: uint32_t,
  /// supported color sample counts for a framebuffer
  /// * Limit Type: bitmask
  /// * Optional
  pub framebuffer_color_sample_counts: VkSampleCountFlags,
  /// supported depth sample counts for a framebuffer
  /// * Limit Type: bitmask
  /// * Optional
  pub framebuffer_depth_sample_counts: VkSampleCountFlags,
  /// supported stencil sample counts for a framebuffer
  /// * Limit Type: bitmask
  /// * Optional
  pub framebuffer_stencil_sample_counts: VkSampleCountFlags,
  /// supported sample counts for a subpass which uses no attachments
  /// * Limit Type: bitmask
  /// * Optional
  pub framebuffer_no_attachments_sample_counts: VkSampleCountFlags,
  /// max number of color attachments per subpass
  /// * Limit Type: max
  pub max_color_attachments: uint32_t,
  /// supported color sample counts for a non-integer sampled image
  /// * Limit Type: bitmask
  /// * Optional
  pub sampled_image_color_sample_counts: VkSampleCountFlags,
  /// supported sample counts for an integer image
  /// * Limit Type: bitmask
  /// * Optional
  pub sampled_image_integer_sample_counts: VkSampleCountFlags,
  /// supported depth sample counts for a sampled image
  /// * Limit Type: bitmask
  /// * Optional
  pub sampled_image_depth_sample_counts: VkSampleCountFlags,
  /// supported stencil sample counts for a sampled image
  /// * Limit Type: bitmask
  /// * Optional
  pub sampled_image_stencil_sample_counts: VkSampleCountFlags,
  /// supported sample counts for a storage image
  /// * Limit Type: bitmask
  /// * Optional
  pub storage_image_sample_counts: VkSampleCountFlags,
  /// max number of sample mask words
  /// * Limit Type: max
  pub max_sample_mask_words: uint32_t,
  /// timestamps on graphics and compute queues
  /// * Limit Type: bitmask
  pub timestamp_compute_and_graphics: VkBool32,
  /// number of nanoseconds it takes for timestamp query value to increment by 1
  /// * Limit Type: min,mul
  pub timestamp_period: float,
  /// max number of clip distances
  /// * Limit Type: max
  pub max_clip_distances: uint32_t,
  /// max number of cull distances
  /// * Limit Type: max
  pub max_cull_distances: uint32_t,
  /// max combined number of user clipping
  /// * Limit Type: max
  pub max_combined_clip_and_cull_distances: uint32_t,
  /// distinct queue priorities available
  /// * Limit Type: max
  pub discrete_queue_priorities: uint32_t,
  /// range (min,max) of supported point sizes
  /// * Limit Type: range
  pub point_size_range: [float; 2],
  /// range (min,max) of supported line widths
  /// * Limit Type: range
  pub line_width_range: [float; 2],
  /// granularity of supported point sizes
  /// * Limit Type: min,mul
  pub point_size_granularity: float,
  /// granularity of supported line widths
  /// * Limit Type: min,mul
  pub line_width_granularity: float,
  /// line rasterization follows preferred rules
  /// * Limit Type: bitmask
  pub strict_lines: VkBool32,
  /// supports standard sample locations for all supported sample counts
  /// * Limit Type: bitmask
  pub standard_sample_locations: VkBool32,
  /// optimal offset of buffer copies
  /// * Limit Type: min,pot
  pub optimal_buffer_copy_offset_alignment: VkDeviceSize,
  /// optimal pitch of buffer copies
  /// * Limit Type: min,pot
  pub optimal_buffer_copy_row_pitch_alignment: VkDeviceSize,
  /// minimum size and alignment for non-coherent host-mapped device memory
  /// access
  /// * Limit Type: min,pot
  pub non_coherent_atom_size: VkDeviceSize,
}

/*
<type name="VkPhysicalDeviceMemoryProperties"/>
<type name="VkPhysicalDeviceProperties"/>
<type name="VkPhysicalDeviceSparseProperties"/>
<type name="VkPhysicalDeviceType"/>
<type name="VkQueueFamilyProperties"/>
<type name="VkQueueFlagBits"/>
<type name="VkQueueFlags"/>
<type name="VkSampleCountFlagBits"/>
<type name="VkSampleCountFlags"/>
<type name="VkSystemAllocationScope"/>
<command name="vkCreateInstance"/>
<command name="vkDestroyInstance"/>
<command name="vkEnumeratePhysicalDevices"/>
<command name="vkGetPhysicalDeviceFeatures"/>
<command name="vkGetPhysicalDeviceFormatProperties"/>
<command name="vkGetPhysicalDeviceImageFormatProperties"/>
<command name="vkGetPhysicalDeviceProperties"/>
<command name="vkGetPhysicalDeviceQueueFamilyProperties"/>
<command name="vkGetPhysicalDeviceMemoryProperties"/>
<command name="vkGetInstanceProcAddr"/>
<command name="vkGetDeviceProcAddr"/>
<type name="VkDevice"/>
<type name="VkDeviceCreateFlags" comment="Will add VkDeviceCreateFlagBits when bits are defined in the future"/>
<type name="VkDeviceCreateInfo"/>
<type name="VkDeviceQueueCreateFlags" comment="VkDeviceQueueCreateFlagBits was added later"/>
<type name="VkDeviceQueueCreateInfo"/>
<command name="vkCreateDevice"/>
<command name="vkDestroyDevice"/>
<type name="VkExtensionProperties"/>
<command name="vkEnumerateInstanceExtensionProperties"/>
<command name="vkEnumerateDeviceExtensionProperties"/>
<type name="VkLayerProperties"/>
<command name="vkEnumerateInstanceLayerProperties"/>
<command name="vkEnumerateDeviceLayerProperties"/>
<type name="VkPipelineStageFlagBits"/>
<type name="VkPipelineStageFlags"/>
<type name="VkQueue"/>
<type name="VkSubmitInfo"/>
<command name="vkGetDeviceQueue"/>
<command name="vkQueueSubmit"/>
<command name="vkQueueWaitIdle"/>
<command name="vkDeviceWaitIdle"/>
<type name="VkMappedMemoryRange"/>
<type name="VkMemoryAllocateInfo"/>
<type name="VkMemoryMapFlags"/>
<command name="vkAllocateMemory"/>
<command name="vkFreeMemory"/>
<command name="vkMapMemory"/>
<command name="vkUnmapMemory"/>
<command name="vkFlushMappedMemoryRanges"/>
<command name="vkInvalidateMappedMemoryRanges"/>
<command name="vkGetDeviceMemoryCommitment"/>
<type name="VkDeviceMemory"/>
<type name="VkMemoryRequirements"/>
<command name="vkBindBufferMemory"/>
<command name="vkBindImageMemory"/>
<command name="vkGetBufferMemoryRequirements"/>
<command name="vkGetImageMemoryRequirements"/>
<type name="VkBindSparseInfo"/>
<type name="VkImageAspectFlagBits"/>
<type name="VkImageAspectFlags"/>
<type name="VkImageSubresource"/>
<type name="VkSparseBufferMemoryBindInfo"/>
<type name="VkSparseImageFormatFlagBits"/>
<type name="VkSparseImageFormatFlags"/>
<type name="VkSparseImageFormatProperties"/>
<type name="VkSparseImageMemoryBind"/>
<type name="VkSparseImageMemoryBindInfo"/>
<type name="VkSparseImageMemoryRequirements"/>
<type name="VkSparseImageOpaqueMemoryBindInfo"/>
<type name="VkSparseMemoryBind"/>
<type name="VkSparseMemoryBindFlagBits"/>
<type name="VkSparseMemoryBindFlags"/>
<command name="vkGetImageSparseMemoryRequirements"/>
<command name="vkGetPhysicalDeviceSparseImageFormatProperties"/>
<command name="vkQueueBindSparse"/>
<type name="VkFence"/>
<type name="VkFenceCreateFlagBits"/>
<type name="VkFenceCreateFlags"/>
<type name="VkFenceCreateInfo"/>
<command name="vkCreateFence"/>
<command name="vkDestroyFence"/>
<command name="vkResetFences"/>
<command name="vkGetFenceStatus"/>
<command name="vkWaitForFences"/>
<type name="VkSemaphore"/>
<type name="VkSemaphoreCreateFlags" comment="Will add VkSemaphoreCreateFlagBits when bits are defined in the future"/>
<type name="VkSemaphoreCreateInfo"/>
<command name="vkCreateSemaphore"/>
<command name="vkDestroySemaphore"/>
<type name="VkEvent"/>
<type name="VkEventCreateFlags"/>
<type name="VkEventCreateFlagBits"/>
<type name="VkEventCreateInfo"/>
<command name="vkCreateEvent"/>
<command name="vkDestroyEvent"/>
<command name="vkGetEventStatus"/>
<command name="vkSetEvent"/>
<command name="vkResetEvent"/>
<type name="VkQueryPipelineStatisticFlagBits"/>
<type name="VkQueryPipelineStatisticFlags"/>
<type name="VkQueryPool"/>
<type name="VkQueryPoolCreateFlags" comment="Will add VkQueryPoolCreateFlagBits when bits are defined in the future"/>
<type name="VkQueryPoolCreateInfo"/>
<type name="VkQueryResultFlagBits"/>
<type name="VkQueryResultFlags"/>
<type name="VkQueryType"/>
<command name="vkCreateQueryPool"/>
<command name="vkDestroyQueryPool"/>
<command name="vkGetQueryPoolResults"/>
<type name="VkBuffer"/>
<type name="VkBufferCreateFlagBits"/>
<type name="VkBufferCreateFlags"/>
<type name="VkBufferCreateInfo"/>
<type name="VkBufferUsageFlagBits"/>
<type name="VkBufferUsageFlags"/>
<type name="VkSharingMode"/>
<command name="vkCreateBuffer"/>
<command name="vkDestroyBuffer"/>
<type name="VkBufferView"/>
<type name="VkBufferViewCreateFlags" comment="Will add VkBufferViewFlagBits when bits are defined in the future"/>
<type name="VkBufferViewCreateInfo"/>
<command name="vkCreateBufferView"/>
<command name="vkDestroyBufferView"/>
<type name="VkImage"/>
<type name="VkImageCreateInfo"/>
<type name="VkImageLayout"/>
<type name="VkSubresourceLayout"/>
<command name="vkCreateImage"/>
<command name="vkDestroyImage"/>
<command name="vkGetImageSubresourceLayout"/>
<type name="VkComponentMapping"/>
<type name="VkComponentSwizzle"/>
<type name="VkImageSubresourceRange"/>
<type name="VkImageView"/>
<type name="VkImageViewCreateFlagBits"/>
<type name="VkImageViewCreateFlags"/>
<type name="VkImageViewCreateInfo"/>
<type name="VkImageViewType"/>
<command name="vkCreateImageView"/>
<command name="vkDestroyImageView"/>
<type name="VkShaderModule"/>
<type name="VkShaderModuleCreateFlags"/>
<type name="VkShaderModuleCreateInfo"/>
<command name="vkCreateShaderModule"/>
<command name="vkDestroyShaderModule"/>
<type name="VkPipelineCache"/>
<type name="VkPipelineCacheCreateFlags" comment="VkPipelineCacheCreateFlagBits was added later"/>
<type name="VkPipelineCacheCreateInfo"/>
<command name="vkCreatePipelineCache"/>
<command name="vkDestroyPipelineCache"/>
<command name="vkGetPipelineCacheData"/>
<command name="vkMergePipelineCaches"/>
<type name="VkBlendFactor"/>
<type name="VkBlendOp"/>
<type name="VkColorComponentFlagBits"/>
<type name="VkColorComponentFlags"/>
<type name="VkCompareOp"/>
<type name="VkComputePipelineCreateInfo"/>
<type name="VkCullModeFlagBits"/>
<type name="VkCullModeFlags"/>
<type name="VkDynamicState"/>
<type name="VkFrontFace"/>
<type name="VkGraphicsPipelineCreateInfo"/>
<type name="VkLogicOp"/>
<type name="VkPipeline"/>
<type name="VkPipelineColorBlendAttachmentState"/>
<type name="VkPipelineColorBlendStateCreateFlags" comment="Will add VkPipeline*StateFlagBits when bits are defined in the future"/>
<type name="VkPipelineColorBlendStateCreateInfo"/>
<type name="VkPipelineCreateFlagBits"/>
<type name="VkPipelineCreateFlags"/>
<type name="VkPipelineDepthStencilStateCreateFlags" comment="Will add VkPipeline*StateFlagBits when bits are defined in the future"/>
<type name="VkPipelineDepthStencilStateCreateInfo"/>
<type name="VkPipelineDynamicStateCreateFlags" comment="Will add VkPipeline*StateFlagBits when bits are defined in the future"/>
<type name="VkPipelineDynamicStateCreateInfo"/>
<type name="VkPipelineInputAssemblyStateCreateFlags" comment="Will add VkPipeline*StateFlagBits when bits are defined in the future"/>
<type name="VkPipelineInputAssemblyStateCreateInfo"/>
<type name="VkPipelineLayoutCreateFlags" comment="Will add VkPipelineLayoutCreateFlagBits when bits are defined in the future"/>
<type name="VkPipelineMultisampleStateCreateFlags" comment="Will add VkPipelineMultisampleStateCreateFlagBits when bits are defined in the future"/>
<type name="VkPipelineMultisampleStateCreateInfo"/>
<type name="VkPipelineRasterizationStateCreateFlags" comment="Will add VkPipelineRasterizationStateCreateFlagBits when bits are defined in the future"/>
<type name="VkPipelineRasterizationStateCreateInfo"/>
<type name="VkPipelineShaderStageCreateFlagBits"/>
<type name="VkPipelineShaderStageCreateFlags"/>
<type name="VkPipelineShaderStageCreateInfo"/>
<type name="VkPipelineTessellationStateCreateFlags" comment="Will add VkPipelineTessellationStateCreateFlagBits when bits are defined in the future"/>
<type name="VkPipelineTessellationStateCreateInfo"/>
<type name="VkPipelineVertexInputStateCreateFlags" comment="Will add VkPipelineVertexInputStateCreateFlagBits when bits are defined in the future"/>
<type name="VkPipelineVertexInputStateCreateInfo"/>
<type name="VkPipelineViewportStateCreateFlags" comment="Will add VkPipelineViewportStateCreateFlagBits when bits are defined in the future"/>
<type name="VkPipelineViewportStateCreateInfo"/>
<type name="VkPolygonMode"/>
<type name="VkPrimitiveTopology"/>
<type name="VkSampleMask"/>
<type name="VkShaderStageFlagBits"/>
<type name="VkShaderStageFlags"/>
<type name="VkSpecializationInfo"/>
<type name="VkSpecializationMapEntry"/>
<type name="VkStencilOp"/>
<type name="VkStencilOpState"/>
<type name="VkVertexInputAttributeDescription"/>
<type name="VkVertexInputBindingDescription"/>
<type name="VkVertexInputRate"/>
<type name="VkViewport"/>
<command name="vkCreateGraphicsPipelines"/>
<command name="vkCreateComputePipelines"/>
<command name="vkDestroyPipeline"/>
<type name="VkPipelineLayout"/>
<type name="VkPipelineLayoutCreateInfo"/>
<type name="VkPushConstantRange"/>
<command name="vkCreatePipelineLayout"/>
<command name="vkDestroyPipelineLayout"/>
<type name="VkBorderColor"/>
<type name="VkFilter"/>
<type name="VkSampler"/>
<type name="VkSamplerAddressMode"/>
<type name="VkSamplerCreateFlagBits"/>
<type name="VkSamplerCreateFlags"/>
<type name="VkSamplerCreateInfo"/>
<type name="VkSamplerMipmapMode"/>
<command name="vkCreateSampler"/>
<command name="vkDestroySampler"/>
<type name="VkCopyDescriptorSet"/>
<type name="VkDescriptorBufferInfo"/>
<type name="VkDescriptorImageInfo"/>
<type name="VkDescriptorPool"/>
<type name="VkDescriptorPoolCreateFlagBits"/>
<type name="VkDescriptorPoolCreateFlags"/>
<type name="VkDescriptorPoolCreateInfo"/>
<type name="VkDescriptorPoolResetFlags"/>
<type name="VkDescriptorPoolSize"/>
<type name="VkDescriptorSet"/>
<type name="VkDescriptorSetAllocateInfo"/>
<type name="VkDescriptorSetLayout"/>
<type name="VkDescriptorSetLayoutBinding"/>
<type name="VkDescriptorSetLayoutCreateFlagBits"/>
<type name="VkDescriptorSetLayoutCreateFlags"/>
<type name="VkDescriptorSetLayoutCreateInfo"/>
<type name="VkDescriptorType"/>
<type name="VkWriteDescriptorSet"/>
<command name="vkCreateDescriptorSetLayout"/>
<command name="vkDestroyDescriptorSetLayout"/>
<command name="vkCreateDescriptorPool"/>
<command name="vkDestroyDescriptorPool"/>
<command name="vkResetDescriptorPool"/>
<command name="vkAllocateDescriptorSets"/>
<command name="vkFreeDescriptorSets"/>
<command name="vkUpdateDescriptorSets"/>
<type name="VkAccessFlagBits"/>
<type name="VkAccessFlags"/>
<type name="VkAttachmentDescription"/>
<type name="VkAttachmentDescriptionFlagBits"/>
<type name="VkAttachmentDescriptionFlags"/>
<type name="VkAttachmentLoadOp"/>
<type name="VkAttachmentReference"/>
<type name="VkAttachmentStoreOp"/>
<type name="VkDependencyFlagBits"/>
<type name="VkDependencyFlags"/>
<type name="VkFramebuffer"/>
<type name="VkFramebufferCreateFlagBits"/>
<type name="VkFramebufferCreateFlags"/>
<type name="VkFramebufferCreateInfo"/>
<type name="VkPipelineBindPoint"/>
<type name="VkRenderPass"/>
<type name="VkRenderPassCreateFlagBits"/>
<type name="VkRenderPassCreateFlags"/>
<type name="VkRenderPassCreateInfo"/>
<type name="VkSubpassDependency"/>
<type name="VkSubpassDescription"/>
<type name="VkSubpassDescriptionFlagBits"/>
<type name="VkSubpassDescriptionFlags"/>
<command name="vkCreateFramebuffer"/>
<command name="vkDestroyFramebuffer"/>
<command name="vkCreateRenderPass"/>
<command name="vkDestroyRenderPass"/>
<command name="vkGetRenderAreaGranularity"/>
<type name="VkCommandPool"/>
<type name="VkCommandPoolCreateFlagBits"/>
<type name="VkCommandPoolCreateFlags"/>
<type name="VkCommandPoolCreateInfo"/>
<type name="VkCommandPoolResetFlagBits"/>
<type name="VkCommandPoolResetFlags"/>
<command name="vkCreateCommandPool"/>
<command name="vkDestroyCommandPool"/>
<command name="vkResetCommandPool"/>
<type name="VkCommandBuffer"/>
<type name="VkCommandBufferAllocateInfo"/>
<type name="VkCommandBufferBeginInfo"/>
<type name="VkCommandBufferInheritanceInfo"/>
<type name="VkCommandBufferLevel"/>
<type name="VkCommandBufferResetFlagBits"/>
<type name="VkCommandBufferResetFlags"/>
<type name="VkCommandBufferUsageFlagBits"/>
<type name="VkCommandBufferUsageFlags"/>
<type name="VkQueryControlFlagBits"/>
<type name="VkQueryControlFlags"/>
<command name="vkAllocateCommandBuffers"/>
<command name="vkFreeCommandBuffers"/>
<command name="vkBeginCommandBuffer"/>
<command name="vkEndCommandBuffer"/>
<command name="vkResetCommandBuffer"/>
<type name="VkBufferCopy"/>
<type name="VkBufferImageCopy"/>
<type name="VkClearAttachment"/>
<type name="VkClearColorValue"/>
<type name="VkClearDepthStencilValue"/>
<type name="VkClearRect"/>
<type name="VkClearValue"/>
<type name="VkImageBlit"/>
<type name="VkImageCopy"/>
<type name="VkImageResolve"/>
<type name="VkImageSubresourceLayers"/>
<type name="VkIndexType"/>
<type name="VkRenderPassBeginInfo"/>
<type name="VkStencilFaceFlagBits"/>
<type name="VkStencilFaceFlags"/>
<type name="VkSubpassContents"/>
<command name="vkCmdBindPipeline"/>
<command name="vkCmdSetViewport"/>
<command name="vkCmdSetScissor"/>
<command name="vkCmdSetLineWidth"/>
<command name="vkCmdSetDepthBias"/>
<command name="vkCmdSetBlendConstants"/>
<command name="vkCmdSetDepthBounds"/>
<command name="vkCmdSetStencilCompareMask"/>
<command name="vkCmdSetStencilWriteMask"/>
<command name="vkCmdSetStencilReference"/>
<command name="vkCmdBindDescriptorSets"/>
<command name="vkCmdBindIndexBuffer"/>
<command name="vkCmdBindVertexBuffers"/>
<command name="vkCmdDraw"/>
<command name="vkCmdDrawIndexed"/>
<command name="vkCmdDrawIndirect"/>
<command name="vkCmdDrawIndexedIndirect"/>
<command name="vkCmdDispatch"/>
<command name="vkCmdDispatchIndirect"/>
<command name="vkCmdCopyBuffer"/>
<command name="vkCmdCopyImage"/>
<command name="vkCmdBlitImage"/>
<command name="vkCmdCopyBufferToImage"/>
<command name="vkCmdCopyImageToBuffer"/>
<command name="vkCmdUpdateBuffer"/>
<command name="vkCmdFillBuffer"/>
<command name="vkCmdClearColorImage"/>
<command name="vkCmdClearDepthStencilImage"/>
<command name="vkCmdClearAttachments"/>
<command name="vkCmdResolveImage"/>
<command name="vkCmdSetEvent"/>
<command name="vkCmdResetEvent"/>
<command name="vkCmdWaitEvents"/>
<command name="vkCmdPipelineBarrier"/>
<command name="vkCmdBeginQuery"/>
<command name="vkCmdEndQuery"/>
<command name="vkCmdResetQueryPool"/>
<command name="vkCmdWriteTimestamp"/>
<command name="vkCmdCopyQueryPoolResults"/>
<command name="vkCmdPushConstants"/>
<command name="vkCmdBeginRenderPass"/>
<command name="vkCmdNextSubpass"/>
<command name="vkCmdEndRenderPass"/>
<command name="vkCmdExecuteCommands"/>
*/
