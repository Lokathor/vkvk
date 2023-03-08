#![allow(clippy::from_str_radix_10)]
#![allow(clippy::useless_format)]

use core::fmt::Display;
use std::{
  collections::{btree_map::Entry, BTreeMap},
  fs::OpenOptions,
  io::Write,
  path::Path,
};
use vk_dot_xml_parser::*;

mod enumerations;
pub use enumerations::*;

mod bitmasks;
pub use bitmasks::*;

mod unions;
pub use unions::*;

mod structures;
pub use structures::*;

mod aliases;
pub use aliases::*;

/// Given an `api` attribute, is this item "vulkan" compatible?
///
/// The logic is that if "vulkan" is in the comma separated list then it's
/// vulkan compatible, OR if the value is `None` then it's compatible with all
/// APIs (including "vulkan").
fn api_vulkan(api: Option<StaticStr>) -> bool {
  api.map(|s| s.split(',').any(|s| s == "vulkan")).unwrap_or(true)
}

/// Breaks off the vendor tag off of the end of an item name, if there is one.
pub fn break_vendor<'v>(
  registry: &VulkanRegistry, item: &'v str,
) -> (&'v str, Option<&'v str>) {
  for vendor in registry.vendors.iter().copied() {
    if let Some(stripped) = item.strip_suffix(vendor.name) {
      let stripped_len = stripped.len();
      return (&item[..stripped_len], Some(&item[stripped_len..]));
    }
  }
  (item, None)
}

pub fn break_number(item: &str) -> (&str, Option<&str>) {
  if let Some(stripped) = item.strip_suffix(|ch: char| ch.is_ascii_digit()) {
    let stripped_len = stripped.len();
    return (&item[..stripped_len], Some(&item[stripped_len..]));
  }
  (item, None)
}

pub fn format_ty_and_variant(ty: &str, ty_variant: TypeVariant) -> String {
  match ty_variant {
    TypeVariant::Normal => format!("{ty}"),
    TypeVariant::ConstPtr => format!("*const {ty}"),
    TypeVariant::MutPtr => format!("*mut {ty}"),
    TypeVariant::ArraySym(s) => format!("[{ty}; {s}]"),
    TypeVariant::ArrayInt(n) => format!("[{ty}; {n}]"),
    TypeVariant::ArrayArrayInt(j, k) => format!("[[{ty}; {j}]; {k}]"),
    TypeVariant::ConstPtrConstPtr => format!("*const *const {ty}"),
    TypeVariant::MutPtrMutPtr => format!("*mut *mut {ty}"),
    TypeVariant::ConstPtrArrayInt(n) => format!("*const [{ty}; {n}]"),
  }
}

pub fn filter_ty(ty: &str) -> &str {
  match ty {
    "int" => "c_int",
    "char" => "u8",
    "int8_t" => "i8",
    "uint8_t" => "u8",
    "uint16_t" => "u16",
    "int32_t" => "i32",
    "int64_t" => "i64",
    "uint32_t" => "u32",
    "uint64_t" => "u64",
    "size_t" => "c_size_t",
    "float" => "c_float",
    "double" => "c_double",
    "void" => "c_void",
    "Display" => "XlibDisplay",
    "Window" => "XlibWindow",
    other => other,
  }
}

const BLOCKED_TYPES: &[&str] = &[
  // Blocked because they're part of the private Google Games Platform. Since that was
  // part of Stadia, and Stadia was shut down, they will probably never matter.
  "GgpFrameToken",
  "GgpStreamDescriptor",
  // TODO: handle bitfiles in struct members properly so that we can allow these types
  // back in.
  "VkAccelerationStructureInstanceKHR",
  "VkAccelerationStructureSRTMotionInstanceNV",
  "VkAccelerationStructureMotionInstanceDataNV",
  // beta types
  "VkPhysicalDevicePortabilitySubsetPropertiesKHR",
  "VkPhysicalDevicePortabilitySubsetFeaturesKHR",
  // vulkansc types i'm not sure how to filter out any other obvious way.
  "VkPipelinePoolSize",
  "VkPipelineOfflineCreateInfo",
  "VkPhysicalDeviceVulkanSC10Properties",
  "VkPhysicalDeviceVulkanSC10Features",
  "VkPerformanceQueryReservationInfoKHR",
  "VkFaultData",
  "VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV",
  "VkCommandPoolMemoryReservationCreateInfo",
  "VkCommandPoolMemoryConsumption",
  // TODO: (maybe) Support the "Vulkan Video" API?
  "StdVideoDecodeH264PictureInfo",
  "StdVideoDecodeH264ReferenceInfo",
  "StdVideoDecodeH265PictureInfo",
  "StdVideoDecodeH265ReferenceInfo",
  "StdVideoEncodeH264PictureInfo",
  "StdVideoEncodeH264ReferenceInfo",
  "StdVideoEncodeH264RefMemMgmtCtrlOperations",
  "StdVideoEncodeH264SliceHeader",
  "StdVideoEncodeH265PictureInfo",
  "StdVideoEncodeH265ReferenceInfo",
  "StdVideoEncodeH265ReferenceModifications",
  "StdVideoEncodeH265SliceSegmentHeader",
  "StdVideoH264LevelIdc",
  "StdVideoH264PictureParameterSet",
  "StdVideoH264ProfileIdc",
  "StdVideoH264SequenceParameterSet",
  "StdVideoH265LevelIdc",
  "StdVideoH265PictureParameterSet",
  "StdVideoH265ProfileIdc",
  "StdVideoH265SequenceParameterSet",
  "StdVideoH265VideoParameterSet",
  "VkVideoEncodeH264SessionParametersAddInfoEXT",
  "VkVideoEncodeH265SessionParametersAddInfoEXT",
  "VkVideoDecodeH264SessionParametersAddInfoKHR",
  "VkVideoDecodeH265SessionParametersAddInfoKHR",
  "VkVideoEncodeUsageInfoKHR",
  "VkVideoEncodeRateControlLayerInfoKHR",
  "VkVideoEncodeInfoKHR",
  "VkVideoEncodeH265RateControlLayerInfoEXT",
  "VkVideoEncodeH265CapabilitiesEXT",
  "VkVideoEncodeH265RateControlInfoEXT",
  "VkVideoEncodeH265EmitPictureParametersInfoEXT",
  "VkVideoEncodeH264RateControlLayerInfoEXT",
  "VkVideoEncodeH264RateControlInfoEXT",
  "VkVideoEncodeH264EmitPictureParametersInfoEXT",
  "VkVideoEncodeH264CapabilitiesEXT",
  "VkVideoEncodeCapabilitiesKHR",
  // TODO: support whatever nvidia thing this stuff is for.
  "NvSciBufAttrList",
  "NvSciBufObj",
  "NvSciSyncAttrList",
  "NvSciSyncFence",
  "NvSciSyncObj",
];
