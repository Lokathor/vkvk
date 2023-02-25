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
//
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 8;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
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

/// Khronos: [VkPhysicalDeviceMemoryProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
  pub memory_type_count: uint32_t,
  pub memory_types: *const [VkMemoryType; VK_MAX_MEMORY_TYPES],
  pub memory_heap_count: uint32_t,
  pub memory_heaps: *const [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

/// Khronos: [VkPhysicalDeviceSparseProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
  /// Sparse resources support: GPU will access all 2D (single sample) sparse
  /// resources using the standard sparse image block shapes (based on pixel
  /// format)
  /// * Limit Type: bitmask
  pub residency_standard_2d_block_shape: VkBool32,
  /// Sparse resources support: GPU will access all 2D (multisample) sparse
  /// resources using the standard sparse image block shapes (based on pixel
  /// format)
  /// * Limit Type: bitmask
  pub residency_standard_2d_multisample_block_shape: VkBool32,
  /// Sparse resources support: GPU will access all 3D sparse resources using
  /// the standard sparse image block shapes (based on pixel format)
  /// * Limit Type: bitmask
  pub residency_standard_3d_block_shape: VkBool32,
  /// Sparse resources support: Images with mip level dimensions that are NOT a
  /// multiple of the sparse image block dimensions will be placed in the mip
  /// tail
  /// * Limit Type: bitmask
  pub residency_aligned_mip_size: VkBool32,
  /// Sparse resources support: GPU can consistently access non-resident regions
  /// of a resource, all reads return as if data is 0, writes are discarded
  /// * Limit Type: bitmask
  pub residency_non_resident_strict: VkBool32,
}

/// Khronos: [VkPhysicalDeviceType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceType.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPhysicalDeviceType(pub u32);

/// Khronos: [VkQueueFamilyProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueueFamilyProperties {
  /// Queue flags
  /// * Limit Type: bitmask
  /// * Optional
  pub queue_flags: VkQueueFlags,
  /// * Limit Type: max
  pub queue_count: uint32_t,
  /// * Limit Type: bits
  pub timestamp_valid_bits: uint32_t,
  /// Minimum alignment requirement for image transfers
  /// * Limit Type: min,mul
  pub min_image_transfer_granularity: VkExtent3D,
}

/// Khronos: [VkQueueFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkQueueFlagBits(pub u32);
impl VkQueueFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkQueueFlagBits);

/// Khronos: [VkQueueFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html)
pub type VkQueueFlags = VkQueueFlagBits;

/// Khronos: [VkDevice](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDevice.html) (handle)/// * Parent: [VkPhysicalDevice]/// * Object Type Enum: [`VK_OBJECT_TYPE_DEVICE`]#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDevice(*mut core::ffi::c_void);
unsafe impl Send for VkDevice {}
unsafe impl Sync for VkDevice {}
impl Default for VkDevice {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkDevice {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}

/// Khronos: [VkDeviceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkDeviceCreateFlagBits(pub u32);
impl VkDeviceCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkDeviceCreateFlagBits);

/// Khronos: [VkDeviceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateFlagBits.html)
pub type VkDeviceCreateFlags = VkDeviceCreateFlagBits;

/// Khronos: [VkDeviceQueueCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkDeviceQueueCreateFlagBits(pub u32);
impl VkDeviceQueueCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkDeviceQueueCreateFlagBits);

/// Khronos: [VkDeviceQueueCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html)
pub type VkDeviceQueueCreateFlags = VkDeviceQueueCreateFlagBits;

/// Khronos: [VkDeviceQueueCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkDeviceQueueCreateFlags,
  pub queue_family_index: uint32_t,
  pub queue_count: uint32_t,
  /// * Len: `queueCount`
  pub queue_priorities: *const float,
}
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = VkStructureType(2);

/// Khronos: [VkDeviceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkDeviceCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkDeviceCreateFlags,
  pub queue_create_info_count: uint32_t,
  /// * Len: `queueCreateInfoCount`
  pub queue_create_infos: *const VkDeviceQueueCreateInfo,
  /// * Deprecated: ignored
  /// * Optional
  pub enabled_layer_count: uint32_t,
  /// Ordered list of layer names to be enabled
  /// * Deprecated: ignored
  /// * Len: `enabledLayerCount,null-terminated`
  pub enabled_layer_names: *const *const u8,
  /// * Optional
  pub enabled_extension_count: uint32_t,
  /// * Len: `enabledExtensionCount,null-terminated`
  pub enabled_extension_names: *const *const u8,
  /// * Optional
  pub enabled_features: *const VkPhysicalDeviceFeatures,
}
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = VkStructureType(3);

/// Khronos: [VkExtensionProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExtensionProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkExtensionProperties {
  /// extension name
  pub extension_name: *const [u8; VK_MAX_EXTENSION_NAME_SIZE],
  /// version of the extension specification implemented
  pub spec_version: uint32_t,
}

/// Khronos: [VkLayerProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLayerProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkLayerProperties {
  /// layer name
  pub layer_name: *const [u8; VK_MAX_EXTENSION_NAME_SIZE],
  /// version of the layer specification implemented
  pub spec_version: uint32_t,
  /// build or release version of the layer's library
  pub implementation_version: uint32_t,
  /// Free-form description of the layer
  pub description: *const [u8; VK_MAX_DESCRIPTION_SIZE],
}

/// Khronos: [VkPipelineStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineStageFlagBits(pub u32);
impl VkPipelineStageFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineStageFlagBits);

/// Khronos: [VkPipelineStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html)
pub type VkPipelineStageFlags = VkPipelineStageFlagBits;

/// Khronos: [VkSubpassDependency](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubpassDependency {
  pub src_subpass: uint32_t,
  pub dst_subpass: uint32_t,
  /// * Optional
  pub src_stage_mask: VkPipelineStageFlags,
  /// * Optional
  pub dst_stage_mask: VkPipelineStageFlags,
  /// Memory accesses from the source of the dependency to synchronize
  /// * Optional
  pub src_access_mask: VkAccessFlags,
  /// Memory accesses from the destination of the dependency to synchronize
  /// * Optional
  pub dst_access_mask: VkAccessFlags,
  /// * Optional
  pub dependency_flags: VkDependencyFlags,
}

/// Khronos: [VkDependencyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkDependencyFlagBits(pub u32);
impl VkDependencyFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkDependencyFlagBits);

/// Khronos: [VkDependencyFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html)
pub type VkDependencyFlags = VkDependencyFlagBits;

/// Khronos: [VkQueue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueue.html) (handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_QUEUE`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkQueue(*mut core::ffi::c_void);
unsafe impl Send for VkQueue {}
unsafe impl Sync for VkQueue {}
impl Default for VkQueue {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkQueue {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}
pub const VK_OBJECT_TYPE_QUEUE: VkObjectType = VkObjectType(4);

/// Khronos: [VkSemaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphore.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_SEMAPHORE`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSemaphore(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkSemaphore {}
unsafe impl Sync for VkSemaphore {}
impl VkSemaphore {
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
impl Default for VkSemaphore {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_SEMAPHORE: VkObjectType = VkObjectType(5);

/// Khronos: [VkCommandBuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBuffer.html) (handle)
/// * Parent: [VkCommandPool]
/// * Object Type Enum: [`VK_OBJECT_TYPE_COMMAND_BUFFER`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkCommandBuffer(*mut core::ffi::c_void);
unsafe impl Send for VkCommandBuffer {}
unsafe impl Sync for VkCommandBuffer {}
impl Default for VkCommandBuffer {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
impl VkCommandBuffer {
  pub const NULL: Self = Self::null();
  #[inline]
  #[must_use]
  pub const fn null() -> Self {
    Self(core::ptr::null_mut())
  }
}
pub const VK_OBJECT_TYPE_COMMAND_BUFFER: VkObjectType = VkObjectType(6);

/// Khronos: [VkSubmitInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubmitInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_SUBMIT_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub wait_semaphore_count: uint32_t,
  /// * Len: `waitSemaphoreCount`
  pub wait_semaphores: *const VkSemaphore,
  /// * Len: `waitSemaphoreCount`
  /// * Must be non-null, but may point at 0.
  pub wait_dst_stage_mask: *const VkPipelineStageFlags,
  /// * Optional
  pub command_buffer_count: uint32_t,
  /// * Len: `commandBufferCount`
  pub command_buffers: *const VkCommandBuffer,
  /// * Optional
  pub signal_semaphore_count: uint32_t,
  /// * Len: `signalSemaphoreCount`
  pub signal_semaphores: *const VkSemaphore,
}
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = VkStructureType(4);

/// Khronos: [VkDeviceMemory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemory.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_DEVICE_MEMORY`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDeviceMemory(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkDeviceMemory {}
unsafe impl Sync for VkDeviceMemory {}
impl VkDeviceMemory {
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
impl Default for VkDeviceMemory {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_DEVICE_MEMORY: VkObjectType = VkObjectType(8);

/// Khronos: [VkMappedMemoryRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMappedMemoryRange.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMappedMemoryRange {
  /// * Values: [`VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Mapped memory object
  pub memory: VkDeviceMemory,
  /// Offset within the memory object where the range starts
  pub offset: VkDeviceSize,
  /// Size of the range within the memory object
  pub size: VkDeviceSize,
}
pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: VkStructureType = VkStructureType(6);

/// Khronos: [VkMemoryAllocateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryAllocateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Size of memory allocation
  pub allocation_size: VkDeviceSize,
  /// Index of the of the memory type to allocate from
  pub memory_type_index: uint32_t,
}
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = VkStructureType(5);

/// Khronos: [VkMemoryMapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkMemoryMapFlagBits(pub u32);
impl VkMemoryMapFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkMemoryMapFlagBits);

/// Khronos: [VkMemoryMapFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlagBits.html)
pub type VkMemoryMapFlags = VkMemoryMapFlagBits;

/// Khronos: [VkMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkMemoryRequirements {
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Specified in bytes
  pub alignment: VkDeviceSize,
  /// Bitmask of the allowed memory type indices into memoryTypes[] for this
  /// object
  pub memory_type_bits: uint32_t,
}

/// Khronos: [VkSparseMemoryBindFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSparseMemoryBindFlagBits(pub u32);
impl VkSparseMemoryBindFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkSparseMemoryBindFlagBits);

/// Khronos: [VkSparseMemoryBindFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html)
pub type VkSparseMemoryBindFlags = VkSparseMemoryBindFlagBits;

/// Khronos: [VkSparseMemoryBind](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBind.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseMemoryBind {
  /// Specified in bytes
  pub resource_offset: VkDeviceSize,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// * Optional
  pub memory: VkDeviceMemory,
  /// Specified in bytes
  pub memory_offset: VkDeviceSize,
  /// * Optional
  pub flags: VkSparseMemoryBindFlags,
}

/// Khronos: [VkSparseBufferMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseBufferMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
  pub buffer: VkBuffer,
  pub bind_count: uint32_t,
  /// * Len: `bindCount`
  pub binds: *const VkSparseMemoryBind,
}

/// Khronos: [VkSparseImageOpaqueMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
  pub image: VkImage,
  pub bind_count: uint32_t,
  /// * Len: `bindCount`
  pub binds: *const VkSparseMemoryBind,
}

/// Khronos: [VkImageSubresource](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSubresource.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageSubresource {
  pub aspect_mask: VkImageAspectFlags,
  pub mip_level: uint32_t,
  pub array_layer: uint32_t,
}

/// Khronos: [VkSparseImageMemoryBind](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBind.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryBind {
  pub subresource: VkImageSubresource,
  pub offset: VkOffset3D,
  pub extent: VkExtent3D,
  /// * Optional
  pub memory: VkDeviceMemory,
  /// Specified in bytes
  pub memory_offset: VkDeviceSize,
  /// * Optional
  pub flags: VkSparseMemoryBindFlags,
}

/// Khronos: [VkSparseImageMemoryBindInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryBindInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
  pub image: VkImage,
  pub bind_count: uint32_t,
  /// * Len: `bindCount`
  pub binds: *const VkSparseImageMemoryBind,
}

/// Khronos: [VkBindSparseInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindSparseInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBindSparseInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_BIND_SPARSE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub wait_semaphore_count: uint32_t,
  /// * Len: `waitSemaphoreCount`
  pub wait_semaphores: *const VkSemaphore,
  /// * Optional
  pub buffer_bind_count: uint32_t,
  /// * Len: `bufferBindCount`
  pub buffer_binds: *const VkSparseBufferMemoryBindInfo,
  /// * Optional
  pub image_opaque_bind_count: uint32_t,
  /// * Len: `imageOpaqueBindCount`
  pub image_opaque_binds: *const VkSparseImageOpaqueMemoryBindInfo,
  /// * Optional
  pub image_bind_count: uint32_t,
  /// * Len: `imageBindCount`
  pub image_binds: *const VkSparseImageMemoryBindInfo,
  /// * Optional
  pub signal_semaphore_count: uint32_t,
  /// * Len: `signalSemaphoreCount`
  pub signal_semaphores: *const VkSemaphore,
}
pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: VkStructureType = VkStructureType(7);

/// Khronos: [VkSparseImageFormatFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSparseImageFormatFlagBits(pub u32);
impl VkSparseImageFormatFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkSparseImageFormatFlagBits);

/// Khronos: [VkSparseImageFormatFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html)
pub type VkSparseImageFormatFlags = VkSparseImageFormatFlagBits;

/// Khronos: [VkSparseImageFormatProperties](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageFormatProperties {
  /// * Limit Type: bitmask
  /// * Optional
  pub aspect_mask: VkImageAspectFlags,
  /// * Limit Type: min,mul
  pub image_granularity: VkExtent3D,
  /// * Limit Type: bitmask
  /// * Optional
  pub flags: VkSparseImageFormatFlags,
}

/// Khronos: [VkSparseImageMemoryRequirements](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements.html)
/// * Returned Only
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSparseImageMemoryRequirements {
  pub format_properties: VkSparseImageFormatProperties,
  pub image_mip_tail_first_lod: uint32_t,
  /// Specified in bytes, must be a multiple of sparse block size in bytes /
  /// alignment
  pub image_mip_tail_size: VkDeviceSize,
  /// Specified in bytes, must be a multiple of sparse block size in bytes /
  /// alignment
  pub image_mip_tail_offset: VkDeviceSize,
  /// Specified in bytes, must be a multiple of sparse block size in bytes /
  /// alignment
  pub image_mip_tail_stride: VkDeviceSize,
}

/// Khronos: [VkFence](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFence.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_FENCE`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkFence(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkFence {}
unsafe impl Sync for VkFence {}
impl VkFence {
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
impl Default for VkFence {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_FENCE: VkObjectType = VkObjectType(7);

/// Khronos: [VkFenceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkFenceCreateFlagBits(pub u32);
impl VkFenceCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkFenceCreateFlagBits);

/// Khronos: [VkFenceCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html)
pub type VkFenceCreateFlags = VkFenceCreateFlagBits;

/// Khronos: [VkFenceCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkFenceCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Fence creation flags
  /// * Optional
  pub flags: VkFenceCreateFlags,
}
pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = VkStructureType(8);

/// Khronos: [VkSemaphoreCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSemaphoreCreateFlagBits(pub u32);
impl VkSemaphoreCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkSemaphoreCreateFlagBits);

/// Khronos: [VkSemaphoreCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateFlagBits.html)
pub type VkSemaphoreCreateFlags = VkSemaphoreCreateFlagBits;

/// Khronos: [VkSemaphoreCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSemaphoreCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Semaphore creation flags
  /// * Optional
  pub flags: VkSemaphoreCreateFlags,
}
pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = VkStructureType(9);

/// Khronos: [VkEvent](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEvent.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_EVENT`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkEvent(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkEvent {}
unsafe impl Sync for VkEvent {}
impl VkEvent {
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
impl Default for VkEvent {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_EVENT: VkObjectType = VkObjectType(11);

/// Khronos: [VkEventCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkEventCreateFlagBits(pub u32);
impl VkEventCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkEventCreateFlagBits);

/// Khronos: [VkEventCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlagBits.html)
pub type VkEventCreateFlags = VkEventCreateFlagBits;

/// Khronos: [VkEventCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkEventCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkEventCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_EVENT_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Event creation flags
  /// * Optional
  pub flags: VkEventCreateFlags,
}
pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: VkStructureType = VkStructureType(10);

/// Khronos: [VkQueryPipelineStatisticFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkQueryPipelineStatisticFlagBits(pub u32);
impl VkQueryPipelineStatisticFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkQueryPipelineStatisticFlagBits);

/// Khronos: [VkQueryPipelineStatisticFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html)
pub type VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlagBits;

/// Khronos: [VkQueryPool](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPool.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_QUERY_POOL`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkQueryPool(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkQueryPool {}
unsafe impl Sync for VkQueryPool {}
impl VkQueryPool {
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
impl Default for VkQueryPool {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_QUERY_POOL: VkObjectType = VkObjectType(12);

/// Khronos: [VkQueryPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkQueryPoolCreateFlagBits(pub u32);
impl VkQueryPoolCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkQueryPoolCreateFlagBits);

/// Khronos: [VkQueryPoolCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateFlagBits.html)
pub type VkQueryPoolCreateFlags = VkQueryPoolCreateFlagBits;

/// Khronos: [VkQueryType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryType.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkQueryType(pub u32);

/// Khronos: [VkQueryPoolCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkQueryPoolCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkQueryPoolCreateFlags,
  pub query_type: VkQueryType,
  pub query_count: uint32_t,
  /// Optional
  /// * Optional
  /// * No Auto Validity
  pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}
pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: VkStructureType = VkStructureType(11);

/// Khronos: [VkQueryResultFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkQueryResultFlagBits(pub u32);
impl VkQueryResultFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkQueryResultFlagBits);

/// Khronos: [VkQueryResultFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html)
pub type VkQueryResultFlags = VkQueryResultFlagBits;

/// Khronos: [VkBufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkBufferCreateFlagBits(pub u32);
impl VkBufferCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkBufferCreateFlagBits);

/// Khronos: [VkBufferCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html)
pub type VkBufferCreateFlags = VkBufferCreateFlagBits;

/// Khronos: [VkBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkBufferUsageFlagBits(pub u32);
impl VkBufferUsageFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkBufferUsageFlagBits);

/// Khronos: [VkBufferUsageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html)
pub type VkBufferUsageFlags = VkBufferUsageFlagBits;

/// Khronos: [VkSharingMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSharingMode(pub u32);

/// Khronos: [VkBufferCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Buffer creation flags
  /// * Optional
  pub flags: VkBufferCreateFlags,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Buffer usage flags
  pub usage: VkBufferUsageFlags,
  pub sharing_mode: VkSharingMode,
  /// * Optional
  pub queue_family_index_count: uint32_t,
  /// * Len: `queueFamilyIndexCount`
  /// * No Auto Validity
  pub queue_family_indices: *const uint32_t,
}
pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = VkStructureType(12);

/// Khronos: [VkBufferView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferView.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_BUFFER_VIEW`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkBufferView(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkBufferView {}
unsafe impl Sync for VkBufferView {}
impl VkBufferView {
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
impl Default for VkBufferView {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}

/// Khronos: [VkBufferViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkBufferViewCreateFlagBits(pub u32);
impl VkBufferViewCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkBufferViewCreateFlagBits);

/// Khronos: [VkBufferViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateFlagBits.html)
pub type VkBufferViewCreateFlags = VkBufferViewCreateFlagBits;

/// Khronos: [VkBufferViewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkBufferViewCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkBufferViewCreateFlags,
  pub buffer: VkBuffer,
  /// Optionally specifies format of elements
  pub format: VkFormat,
  /// Specified in bytes
  pub offset: VkDeviceSize,
  /// View size specified in bytes
  pub range: VkDeviceSize,
}
pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: VkStructureType = VkStructureType(13);

/// Khronos: [VkImageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Image creation flags
  /// * Optional
  pub flags: VkImageCreateFlags,
  pub image_type: VkImageType,
  pub format: VkFormat,
  pub extent: VkExtent3D,
  pub mip_levels: uint32_t,
  pub array_layers: uint32_t,
  pub samples: VkSampleCountFlagBits,
  pub tiling: VkImageTiling,
  /// Image usage flags
  pub usage: VkImageUsageFlags,
  /// Cross-queue-family sharing mode
  pub sharing_mode: VkSharingMode,
  /// Number of queue families to share across
  /// * Optional
  pub queue_family_index_count: uint32_t,
  /// Array of queue family indices to share across
  /// * Len: `queueFamilyIndexCount`
  /// * No Auto Validity
  pub queue_family_indices: *const uint32_t,
  /// Initial image layout for all subresources
  pub initial_layout: VkImageLayout,
}
pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: VkStructureType = VkStructureType(14);

/// Khronos: [VkSubresourceLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubresourceLayout.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSubresourceLayout {
  /// Specified in bytes
  pub offset: VkDeviceSize,
  /// Specified in bytes
  pub size: VkDeviceSize,
  /// Specified in bytes
  pub row_pitch: VkDeviceSize,
  /// Specified in bytes
  pub array_pitch: VkDeviceSize,
  /// Specified in bytes
  pub depth_pitch: VkDeviceSize,
}

/// Khronos: [VkComponentSwizzle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentSwizzle.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkComponentSwizzle(pub u32);

/// Khronos: [VkComponentMapping](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentMapping.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkComponentMapping {
  pub r: VkComponentSwizzle,
  pub g: VkComponentSwizzle,
  pub b: VkComponentSwizzle,
  pub a: VkComponentSwizzle,
}

/// Khronos: [VkImageView](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageView.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_IMAGE_VIEW`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkImageView(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkImageView {}
unsafe impl Sync for VkImageView {}
impl VkImageView {
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
impl Default for VkImageView {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_IMAGE_VIEW: VkObjectType = VkObjectType(14);

/// Khronos: [VkImageViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageViewCreateFlagBits(pub u32);
impl VkImageViewCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkImageViewCreateFlagBits);

/// Khronos: [VkImageViewCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html)
pub type VkImageViewCreateFlags = VkImageViewCreateFlagBits;

/// Khronos: [VkImageViewType](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewType.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkImageViewType(pub u32);

/// Khronos: [VkImageViewCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkImageViewCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkImageViewCreateFlags,
  pub image: VkImage,
  pub view_type: VkImageViewType,
  pub format: VkFormat,
  pub components: VkComponentMapping,
  pub subresource_range: VkImageSubresourceRange,
}
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = VkStructureType(15);

/// Khronos: [VkShaderModule](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModule.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_SHADER_MODULE`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkShaderModule(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkShaderModule {}
unsafe impl Sync for VkShaderModule {}
impl VkShaderModule {
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
impl Default for VkShaderModule {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_SHADER_MODULE: VkObjectType = VkObjectType(15);

/// Khronos: [VkShaderModuleCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkShaderModuleCreateFlagBits(pub u32);
impl VkShaderModuleCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkShaderModuleCreateFlagBits);

/// Khronos: [VkShaderModuleCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateFlagBits.html)
pub type VkShaderModuleCreateFlags = VkShaderModuleCreateFlagBits;

/// Khronos: [VkShaderModuleCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateInfo.html)
/// * Struct Extends: [VkPipelineShaderStageCreateInfo]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkShaderModuleCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// noautovalidity because this structure can be either an explicit parameter,
  /// or passed in a pNext chain
  /// * Optional
  /// * No Auto Validity
  pub next: *const void,
  /// * Optional
  pub flags: VkShaderModuleCreateFlags,
  /// Specified in bytes
  pub code_size: size_t,
  /// Binary code of size `code_size`
  /// * Len: `code_size / 4`
  pub code: *const uint32_t,
}
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = VkStructureType(16);

/// Khronos: [VkPipelineCache](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCache.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_PIPELINE_CACHE`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPipelineCache(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkPipelineCache {}
unsafe impl Sync for VkPipelineCache {}
impl VkPipelineCache {
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
impl Default for VkPipelineCache {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}

/// Khronos: [VkPipelineCacheCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineCacheCreateFlagBits(pub u32);
impl VkPipelineCacheCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineCacheCreateFlagBits);

/// Khronos: [VkPipelineCacheCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html)
pub type VkPipelineCacheCreateFlags = VkPipelineCacheCreateFlagBits;

/// Khronos: [VkPipelineCacheCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineCacheCreateFlags,
  /// Size of initial data to populate cache, in bytes
  /// * Optional
  pub initial_data_size: size_t,
  /// Initial data to populate cache
  /// * Len: `initialDataSize`
  pub initial_data: *const void,
}
pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: VkStructureType = VkStructureType(17);

/// Khronos: [VkBlendFactor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlendFactor.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkBlendFactor(pub u32);

/// Khronos: [VkBlendOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlendOp.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkBlendOp(pub u32);

/// Khronos: [VkColorComponentFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkColorComponentFlagBits(pub u32);
impl VkColorComponentFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkColorComponentFlagBits);

/// Khronos: [VkColorComponentFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html)
pub type VkColorComponentFlags = VkColorComponentFlagBits;

/// Khronos: [VkCompareOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompareOp.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkCompareOp(pub u32);

/// Khronos: [VkPipelineCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineCreateFlagBits(pub u32);
impl VkPipelineCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineCreateFlagBits);

/// Khronos: [VkPipelineCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits.html)
pub type VkPipelineCreateFlags = VkPipelineCreateFlagBits;

/// Khronos: [VkPipelineShaderStageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineShaderStageCreateFlagBits(pub u32);
impl VkPipelineShaderStageCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineShaderStageCreateFlagBits);

/// Khronos: [VkPipelineShaderStageCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html)
pub type VkPipelineShaderStageCreateFlags = VkPipelineShaderStageCreateFlagBits;

/// Khronos: [VkShaderStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkShaderStageFlagBits(pub u32);
impl VkShaderStageFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkShaderStageFlagBits);

/// Khronos: [VkShaderStageFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html)
pub type VkShaderStageFlags = VkShaderStageFlagBits;

/// Khronos: [VkSpecializationMapEntry](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSpecializationMapEntry.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSpecializationMapEntry {
  /// The SpecConstant ID specified in the BIL
  pub constant_id: uint32_t,
  /// Offset of the value in the data block
  pub offset: uint32_t,
  /// Size in bytes of the SpecConstant
  /// * No Auto Validity
  pub size: size_t,
}

/// Khronos: [VkSpecializationInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSpecializationInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSpecializationInfo {
  /// Number of entries in the map
  /// * Optional
  pub map_entry_count: uint32_t,
  /// Array of map entries
  /// * Len: `mapEntryCount`
  pub map_entries: *const VkSpecializationMapEntry,
  /// Size in bytes of pData
  /// * Optional
  pub data_size: size_t,
  /// Pointer to SpecConstant data
  /// * Len: `dataSize`
  pub data: *const void,
}

/// Khronos: [VkPipelineShaderStageCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineShaderStageCreateFlags,
  /// Shader stage
  pub stage: VkShaderStageFlagBits,
  /// Module containing entry point
  /// * Optional
  pub module: VkShaderModule,
  /// Null-terminated entry point name
  /// * Len: `null-terminated`
  pub name: *const u8,
  /// * Optional
  pub specialization_info: *const VkSpecializationInfo,
}

/// Khronos: [VkPipelineLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineLayoutCreateFlagBits(pub u32);
impl VkPipelineLayoutCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineLayoutCreateFlagBits);

/// Khronos: [VkPipelineLayoutCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html)
pub type VkPipelineLayoutCreateFlags = VkPipelineLayoutCreateFlagBits;

/// Khronos: [VkPipelineLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayout.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_PIPELINE_LAYOUT`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPipelineLayout(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkPipelineLayout {}
unsafe impl Sync for VkPipelineLayout {}
impl VkPipelineLayout {
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
impl Default for VkPipelineLayout {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: VkObjectType = VkObjectType(17);

/// Khronos: [VkPipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipeline.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_PIPELINE`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkPipeline(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkPipeline {}
unsafe impl Sync for VkPipeline {}
impl VkPipeline {
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
impl Default for VkPipeline {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_PIPELINE: VkObjectType = VkObjectType(19);

/// Khronos: [VkComputePipelineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComputePipelineCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkComputePipelineCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Pipeline creation flags
  /// * Optional
  pub flags: VkPipelineCreateFlags,
  pub stage: VkPipelineShaderStageCreateInfo,
  /// Interface layout of the pipeline
  pub layout: VkPipelineLayout,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * Optional
  /// * No Auto Validity
  pub base_pipeline_handle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  pub base_pipeline_index: int32_t,
}
pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = VkStructureType(29);

/// Khronos: [VkCullModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkCullModeFlagBits(pub u32);
impl VkCullModeFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkCullModeFlagBits);

/// Khronos: [VkCullModeFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html)
pub type VkCullModeFlags = VkCullModeFlagBits;

/// Khronos: [VkDynamicState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDynamicState.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkDynamicState(pub u32);

/// Khronos: [VkFrontFace](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFrontFace.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkFrontFace(pub u32);

/// Khronos: [VkPipelineVertexInputStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineVertexInputStateCreateFlagBits(pub u32);
impl VkPipelineVertexInputStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineVertexInputStateCreateFlagBits);

/// Khronos: [VkPipelineVertexInputStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateFlagBits.html)
pub type VkPipelineVertexInputStateCreateFlags = VkPipelineVertexInputStateCreateFlagBits;

/// Khronos: [VkVertexInputRate](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputRate.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkVertexInputRate(pub u32);

/// Khronos: [VkVertexInputBindingDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputBindingDescription {
  /// Vertex buffer binding id
  pub binding: uint32_t,
  /// Distance between vertices in bytes (0 = no advancement)
  pub stride: uint32_t,
  /// The rate at which the vertex data is consumed
  pub input_rate: VkVertexInputRate,
}

/// Khronos: [VkVertexInputAttributeDescription](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkVertexInputAttributeDescription {
  /// location of the shader vertex attrib
  pub location: uint32_t,
  /// Vertex buffer binding id
  pub binding: uint32_t,
  /// format of source data
  pub format: VkFormat,
  /// Offset of first element in bytes from base of vertex
  pub offset: uint32_t,
}

/// Khronos: [VkPipelineVertexInputStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineVertexInputStateCreateFlags,
  /// number of bindings
  /// * Optional
  pub vertex_binding_description_count: uint32_t,
  /// * Len: `vertexBindingDescriptionCount`
  pub vertex_binding_descriptions: *const VkVertexInputBindingDescription,
  /// number of attributes
  /// * Optional
  pub vertex_attribute_description_count: uint32_t,
  /// * Len: `vertexAttributeDescriptionCount`
  pub vertex_attribute_descriptions: *const VkVertexInputAttributeDescription,
}
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(19);

/// Khronos: [VkPipelineInputAssemblyStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineInputAssemblyStateCreateFlagBits(pub u32);
impl VkPipelineInputAssemblyStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineInputAssemblyStateCreateFlagBits);

/// Khronos: [VkPipelineInputAssemblyStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateFlagBits.html)
pub type VkPipelineInputAssemblyStateCreateFlags = VkPipelineInputAssemblyStateCreateFlagBits;

/// Khronos: [VkPrimitiveTopology](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPrimitiveTopology(pub u32);

/// Khronos: [VkPipelineInputAssemblyStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineInputAssemblyStateCreateFlags,
  pub topology: VkPrimitiveTopology,
  pub primitive_restart_enable: VkBool32,
}
pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(20);

/// Khronos: [VkPipelineTessellationStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineTessellationStateCreateFlagBits(pub u32);
impl VkPipelineTessellationStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineTessellationStateCreateFlagBits);

/// Khronos: [VkPipelineTessellationStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateFlagBits.html)
pub type VkPipelineTessellationStateCreateFlags = VkPipelineTessellationStateCreateFlagBits;

/// Khronos: [VkPipelineTessellationStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineTessellationStateCreateFlags,
  pub patch_control_points: uint32_t,
}
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(21);

/// Khronos: [VkPipelineViewportStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineViewportStateCreateFlagBits(pub u32);
impl VkPipelineViewportStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineViewportStateCreateFlagBits);

/// Khronos: [VkPipelineViewportStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateFlagBits.html)
pub type VkPipelineViewportStateCreateFlags = VkPipelineViewportStateCreateFlagBits;

/// Khronos: [VkViewport](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewport.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkViewport {
  /// * No Auto Validity
  pub x: float,
  /// * No Auto Validity
  pub y: float,
  /// * No Auto Validity
  pub width: float,
  /// * No Auto Validity
  pub height: float,
  pub min_depth: float,
  pub max_depth: float,
}

/// Khronos: [VkPipelineViewportStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineViewportStateCreateFlags,
  /// * Optional
  pub viewport_count: uint32_t,
  /// * Len: `viewportCount`
  /// * Optional
  /// * No Auto Validity
  pub viewports: *const VkViewport,
  /// * Optional
  pub scissor_count: uint32_t,
  /// * Len: `scissorCount`
  /// * Optional
  /// * No Auto Validity
  pub scissors: *const VkRect2D,
}
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(22);

/// Khronos: [VkPipelineRasterizationStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineRasterizationStateCreateFlagBits(pub u32);
impl VkPipelineRasterizationStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineRasterizationStateCreateFlagBits);

/// Khronos: [VkPipelineRasterizationStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateFlagBits.html)
pub type VkPipelineRasterizationStateCreateFlags = VkPipelineRasterizationStateCreateFlagBits;

/// Khronos: [VkPolygonMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPolygonMode.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPolygonMode(pub u32);

/// Khronos: [VkPipelineRasterizationStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineRasterizationStateCreateFlags,
  pub depth_clamp_enable: VkBool32,
  pub rasterizer_discard_enable: VkBool32,
  /// optional (GL45)
  pub polygon_mode: VkPolygonMode,
  /// * Optional
  pub cull_mode: VkCullModeFlags,
  pub front_face: VkFrontFace,
  pub depth_bias_enable: VkBool32,
  pub depth_bias_constant_factor: float,
  pub depth_bias_clamp: float,
  pub depth_bias_slope_factor: float,
  pub line_width: float,
}
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(23);

/// Khronos: [VkPipelineMultisampleStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineMultisampleStateCreateFlagBits(pub u32);
impl VkPipelineMultisampleStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineMultisampleStateCreateFlagBits);

/// Khronos: [VkPipelineMultisampleStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateFlagBits.html)
pub type VkPipelineMultisampleStateCreateFlags = VkPipelineMultisampleStateCreateFlagBits;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSampleMask(pub u32);

/// Khronos: [VkPipelineMultisampleStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineMultisampleStateCreateFlags,
  /// Number of samples used for rasterization
  pub rasterization_samples: VkSampleCountFlagBits,
  /// optional (GL45)
  pub sample_shading_enable: VkBool32,
  /// optional (GL45)
  pub min_sample_shading: float,
  /// Array of sampleMask words
  /// * Len: `latexmath:[\lceil{\mathit{rasterizationSamples} \over 32}\rceil]`
  /// * Alt Len: `(rasterizationSamples + 31) / 32`
  /// * Optional
  pub sample_mask: *const VkSampleMask,
  pub alpha_to_coverage_enable: VkBool32,
  pub alpha_to_one_enable: VkBool32,
}
pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(24);

/// Khronos: [VkPipelineDepthStencilStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineDepthStencilStateCreateFlagBits(pub u32);
impl VkPipelineDepthStencilStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineDepthStencilStateCreateFlagBits);

/// Khronos: [VkPipelineDepthStencilStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html)
pub type VkPipelineDepthStencilStateCreateFlags = VkPipelineDepthStencilStateCreateFlagBits;

/// Khronos: [VkStencilOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilOp.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkStencilOp(pub u32);

/// Khronos: [VkStencilOpState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStencilOpState.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkStencilOpState {
  pub fail_op: VkStencilOp,
  pub pass_op: VkStencilOp,
  pub depth_fail_op: VkStencilOp,
  pub compare_op: VkCompareOp,
  pub compare_mask: uint32_t,
  pub write_mask: uint32_t,
  pub reference: uint32_t,
}

/// Khronos: [VkPipelineDepthStencilStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineDepthStencilStateCreateFlags,
  pub depth_test_enable: VkBool32,
  pub depth_write_enable: VkBool32,
  pub depth_compare_op: VkCompareOp,
  /// optional (depth_bounds_test)
  pub depth_bounds_test_enable: VkBool32,
  pub stencil_test_enable: VkBool32,
  pub front: VkStencilOpState,
  pub back: VkStencilOpState,
  pub min_depth_bounds: float,
  pub max_depth_bounds: float,
}
pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(25);

/// Khronos: [VkPipelineColorBlendStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineColorBlendStateCreateFlagBits(pub u32);
impl VkPipelineColorBlendStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineColorBlendStateCreateFlagBits);

/// Khronos: [VkPipelineColorBlendStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html)
pub type VkPipelineColorBlendStateCreateFlags = VkPipelineColorBlendStateCreateFlagBits;

/// Khronos: [VkLogicOp](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLogicOp.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkLogicOp(pub u32);

/// Khronos: [VkPipelineColorBlendAttachmentState](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAttachmentState.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
  pub blend_enable: VkBool32,
  pub src_color_blend_factor: VkBlendFactor,
  pub dst_color_blend_factor: VkBlendFactor,
  pub color_blend_op: VkBlendOp,
  pub src_alpha_blend_factor: VkBlendFactor,
  pub dst_alpha_blend_factor: VkBlendFactor,
  pub alpha_blend_op: VkBlendOp,
  /// * Optional
  pub color_write_mask: VkColorComponentFlags,
}

/// Khronos: [VkPipelineColorBlendStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineColorBlendStateCreateFlags,
  pub logic_op_enable: VkBool32,
  /// * No Auto Validity
  pub logic_op: VkLogicOp,
  /// # of pAttachments
  /// * Optional
  pub attachment_count: uint32_t,
  /// * Len: `attachmentCount`
  /// * Optional
  pub attachments: *const VkPipelineColorBlendAttachmentState,
  pub blend_constants: [float; 4],
}
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType =
  VkStructureType(26);

/// Khronos: [VkPipelineDynamicStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkPipelineDynamicStateCreateFlagBits(pub u32);
impl VkPipelineDynamicStateCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkPipelineDynamicStateCreateFlagBits);

/// Khronos: [VkPipelineDynamicStateCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateFlagBits.html)
pub type VkPipelineDynamicStateCreateFlags = VkPipelineDynamicStateCreateFlagBits;

/// Khronos: [VkPipelineDynamicStateCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineDynamicStateCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineDynamicStateCreateFlags,
  /// * Optional
  pub dynamic_state_count: uint32_t,
  /// * Len: `dynamicStateCount`
  pub dynamic_states: *const VkDynamicState,
}

/// Khronos: [VkRenderPass](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPass.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_RENDER_PASS`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkRenderPass(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkRenderPass {}
unsafe impl Sync for VkRenderPass {}
impl VkRenderPass {
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
impl Default for VkRenderPass {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}

/// Khronos: [VkGraphicsPipelineCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Pipeline creation flags
  /// * Optional
  pub flags: VkPipelineCreateFlags,
  /// * Optional
  /// * No Auto Validity
  pub stage_count: uint32_t,
  /// One entry for each active shader stage
  /// * Len: `stageCount`
  /// * Optional
  /// * No Auto Validity
  pub stages: *const VkPipelineShaderStageCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub vertex_input_state: *const VkPipelineVertexInputStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub input_assembly_state: *const VkPipelineInputAssemblyStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub tessellation_state: *const VkPipelineTessellationStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub viewport_state: *const VkPipelineViewportStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub rasterization_state: *const VkPipelineRasterizationStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub multisample_state: *const VkPipelineMultisampleStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub depth_stencil_state: *const VkPipelineDepthStencilStateCreateInfo,
  /// * Optional
  /// * No Auto Validity
  pub color_blend_state: *const VkPipelineColorBlendStateCreateInfo,
  /// * Optional
  pub dynamic_state: *const VkPipelineDynamicStateCreateInfo,
  /// Interface layout of the pipeline
  /// * Optional
  /// * No Auto Validity
  pub layout: VkPipelineLayout,
  /// * Optional
  /// * No Auto Validity
  pub render_pass: VkRenderPass,
  /// * No Auto Validity
  pub subpass: uint32_t,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is nonzero, it
  /// specifies the handle of the base pipeline this is a derivative of
  /// * Optional
  /// * No Auto Validity
  pub base_pipeline_handle: VkPipeline,
  /// If VK_PIPELINE_CREATE_DERIVATIVE_BIT is set and this value is not -1, it
  /// specifies an index into pCreateInfos of the base pipeline this is a
  /// derivative of
  pub base_pipeline_index: int32_t,
}
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType = VkStructureType(28);

/// Khronos: [VkDescriptorSetLayout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayout.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDescriptorSetLayout(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkDescriptorSetLayout {}
unsafe impl Sync for VkDescriptorSetLayout {}
impl VkDescriptorSetLayout {
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
impl Default for VkDescriptorSetLayout {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}

/// Khronos: [VkPushConstantRange](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPushConstantRange.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPushConstantRange {
  /// Which stages use the range
  pub stage_flags: VkShaderStageFlags,
  /// Start of the range, in bytes
  pub offset: uint32_t,
  /// Size of the range, in bytes
  pub size: uint32_t,
}

/// Khronos: [VkPipelineLayoutCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkPipelineLayoutCreateFlags,
  /// Number of descriptor sets interfaced by the pipeline
  /// * Optional
  pub set_layout_count: uint32_t,
  /// Array of setCount number of descriptor set layout objects defining the
  /// layout of the
  /// * Len: `setLayoutCount`
  /// * Must be non-null, but may point at 0.
  pub set_layouts: *const VkDescriptorSetLayout,
  /// Number of push-constant ranges used by the pipeline
  /// * Optional
  pub push_constant_range_count: uint32_t,
  /// Array of pushConstantRangeCount number of ranges used by various shader
  /// stages
  /// * Len: `pushConstantRangeCount`
  pub push_constant_ranges: *const VkPushConstantRange,
}

/// Khronos: [VkBorderColor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBorderColor.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkBorderColor(pub u32);

/// Khronos: [VkFilter](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFilter.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkFilter(pub u32);

/// Khronos: [VkSampler](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampler.html) (non-dispatchable handle)
/// * Parent: [VkDevice]
/// * Object Type Enum: [`VK_OBJECT_TYPE_SAMPLER`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkSampler(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkSampler {}
unsafe impl Sync for VkSampler {}
impl VkSampler {
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
impl Default for VkSampler {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}

/// Khronos: [VkSamplerAddressMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerAddressMode.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSamplerAddressMode(pub u32);

/// Khronos: [VkSamplerCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSamplerCreateFlagBits(pub u32);
impl VkSamplerCreateFlagBits {
  #[inline]
  #[must_use]
  pub const fn none() -> Self {
    Self(0)
  }
}
impl_bitops_for!(VkSamplerCreateFlagBits);

/// Khronos: [VkSamplerCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html)
pub type VkSamplerCreateFlags = VkSamplerCreateFlagBits;

/// Khronos: [VkSamplerMipmapMode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerMipmapMode.html)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkSamplerMipmapMode(pub u32);

/// Khronos: [VkSamplerCreateInfo](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateInfo.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkSamplerCreateInfo {
  /// * Values: [`VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// * Optional
  pub flags: VkSamplerCreateFlags,
  /// Filter mode for magnification
  pub mag_filter: VkFilter,
  /// Filter mode for minifiation
  pub min_filter: VkFilter,
  /// Mipmap selection mode
  pub mipmap_mode: VkSamplerMipmapMode,
  pub address_mode_u: VkSamplerAddressMode,
  pub address_mode_v: VkSamplerAddressMode,
  pub address_mode_w: VkSamplerAddressMode,
  pub mip_lod_bias: float,
  pub anisotropy_enable: VkBool32,
  pub max_anisotropy: float,
  pub compare_enable: VkBool32,
  /// * No Auto Validity
  pub compare_op: VkCompareOp,
  pub min_lod: float,
  pub max_lod: float,
  /// * No Auto Validity
  pub border_color: VkBorderColor,
  pub unnormalized_coordinates: VkBool32,
}
pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: VkStructureType = VkStructureType(31);

/// Khronos: [VkDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSet.html) (non-dispatchable handle)
/// * Parent: [VkDescriptorPool]
/// * Object Type Enum: [`VK_OBJECT_TYPE_DESCRIPTOR_SET`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VkDescriptorSet(
  #[cfg(target_pointer_width = "64")] *mut core::ffi::c_void,
  #[cfg(not(target_pointer_width = "64"))] u64,
);
unsafe impl Send for VkDescriptorSet {}
unsafe impl Sync for VkDescriptorSet {}
impl VkDescriptorSet {
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
impl Default for VkDescriptorSet {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::NULL
  }
}
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: VkObjectType = VkObjectType(23);

/// Khronos: [VkCopyDescriptorSet](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyDescriptorSet.html)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkCopyDescriptorSet {
  /// * Values: [`VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET`]
  pub ty: VkStructureType,
  /// * Optional
  pub next: *const void,
  /// Source descriptor set
  pub src_set: VkDescriptorSet,
  /// Binding within the source descriptor set to copy from
  pub src_binding: uint32_t,
  /// Array element within the source binding to copy from
  pub src_array_element: uint32_t,
  /// Destination descriptor set
  pub dst_set: VkDescriptorSet,
  /// Binding within the destination descriptor set to copy to
  pub dst_binding: uint32_t,
  /// Array element within the destination binding to copy to
  pub dst_array_element: uint32_t,
  /// Number of descriptors to write (determines the size of the array pointed
  /// by pDescriptors)
  pub descriptor_count: uint32_t,
}
pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: VkStructureType = VkStructureType(36);

/*
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
<type name="VkCommandPool"/>
<type name="VkCommandPoolCreateFlagBits"/>
<type name="VkCommandPoolCreateFlags"/>
<type name="VkCommandPoolCreateInfo"/>
<type name="VkCommandPoolResetFlagBits"/>
<type name="VkCommandPoolResetFlags"/>
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
*/

/*
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
<command name="vkCreateDevice"/>
<command name="vkDestroyDevice"/>
<command name="vkEnumerateInstanceExtensionProperties"/>
<command name="vkEnumerateDeviceExtensionProperties"/>
<command name="vkEnumerateInstanceLayerProperties"/>
<command name="vkEnumerateDeviceLayerProperties"/>
<command name="vkGetDeviceQueue"/>
<command name="vkQueueSubmit"/>
<command name="vkQueueWaitIdle"/>
<command name="vkDeviceWaitIdle"/>
<command name="vkAllocateMemory"/>
<command name="vkFreeMemory"/>
<command name="vkMapMemory"/>
<command name="vkUnmapMemory"/>
<command name="vkFlushMappedMemoryRanges"/>
<command name="vkInvalidateMappedMemoryRanges"/>
<command name="vkGetDeviceMemoryCommitment"/>
<command name="vkBindBufferMemory"/>
<command name="vkBindImageMemory"/>
<command name="vkGetBufferMemoryRequirements"/>
<command name="vkGetImageMemoryRequirements"/>
<command name="vkGetImageSparseMemoryRequirements"/>
<command name="vkGetPhysicalDeviceSparseImageFormatProperties"/>
<command name="vkQueueBindSparse"/>
<command name="vkCreateFence"/>
<command name="vkDestroyFence"/>
<command name="vkResetFences"/>
<command name="vkGetFenceStatus"/>
<command name="vkWaitForFences"/>
<command name="vkCreateSemaphore"/>
<command name="vkDestroySemaphore"/>
<command name="vkCreateEvent"/>
<command name="vkDestroyEvent"/>
<command name="vkGetEventStatus"/>
<command name="vkSetEvent"/>
<command name="vkResetEvent"/>
<command name="vkCreateQueryPool"/>
<command name="vkDestroyQueryPool"/>
<command name="vkGetQueryPoolResults"/>
<command name="vkCreateBuffer"/>
<command name="vkDestroyBuffer"/>
<command name="vkCreateBufferView"/>
<command name="vkDestroyBufferView"/>
<command name="vkCreateImage"/>
<command name="vkDestroyImage"/>
<command name="vkGetImageSubresourceLayout"/>
<command name="vkCreateImageView"/>
<command name="vkDestroyImageView"/>
<command name="vkCreateShaderModule"/>
<command name="vkDestroyShaderModule"/>
<command name="vkCreatePipelineCache"/>
<command name="vkDestroyPipelineCache"/>
<command name="vkGetPipelineCacheData"/>
<command name="vkMergePipelineCaches"/>
<command name="vkCreateGraphicsPipelines"/>
<command name="vkCreateComputePipelines"/>
<command name="vkDestroyPipeline"/>
<command name="vkCreatePipelineLayout"/>
<command name="vkDestroyPipelineLayout"/>
<command name="vkCreateSampler"/>
<command name="vkDestroySampler"/>
<command name="vkCreateDescriptorSetLayout"/>
<command name="vkDestroyDescriptorSetLayout"/>
<command name="vkCreateDescriptorPool"/>
<command name="vkDestroyDescriptorPool"/>
<command name="vkResetDescriptorPool"/>
<command name="vkAllocateDescriptorSets"/>
<command name="vkFreeDescriptorSets"/>
<command name="vkUpdateDescriptorSets"/>
<command name="vkCreateFramebuffer"/>
<command name="vkDestroyFramebuffer"/>
<command name="vkCreateRenderPass"/>
<command name="vkDestroyRenderPass"/>
<command name="vkGetRenderAreaGranularity"/>
<command name="vkCreateCommandPool"/>
<command name="vkDestroyCommandPool"/>
<command name="vkResetCommandPool"/>
<command name="vkAllocateCommandBuffers"/>
<command name="vkFreeCommandBuffers"/>
<command name="vkBeginCommandBuffer"/>
<command name="vkEndCommandBuffer"/>
<command name="vkResetCommandBuffer"/>
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
