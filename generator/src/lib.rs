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

mod ext_consts;
pub use ext_consts::*;

mod functions;
pub use functions::*;

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
    TypeVariant::ArraySym(s) if ty == "u8" && s != "VK_UUID_SIZE" => {
      format!("zstring::ArrayZString<{s}>")
    }
    TypeVariant::ArraySym(s) => format!("[{ty}; {s}]"),
    TypeVariant::ArrayInt(n) => format!("[{ty}; {n}]"),
    TypeVariant::ArrayArrayInt(j, k) => format!("[[{ty}; {j}]; {k}]"),
    TypeVariant::ConstPtrConstPtr => format!("*const *const {ty}"),
    TypeVariant::MutPtrMutPtr => format!("*mut *mut {ty}"),
    TypeVariant::ConstPtrArrayInt(n) => format!("*const [{ty}; {n}]"),
  }
}

pub fn fix_ty(ty: &str) -> &str {
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

pub fn fix_member_name(name: &str) -> String {
  use convert_case::*;
  let mut s = match name {
    "sType" => "struct_ty",
    "type" => "ty",
    other => other,
  }
  .to_case(Case::Snake);
  if let Some(x) = s.strip_prefix("p_") {
    s = x.to_string()
  }
  if let Some(x) = s.strip_prefix("pp_") {
    s = x.to_string()
  }
  s
}

const HANDWRITTEN_TYPES: &[&str] = &["VkClearAttachment"];

const BLOCKED_TYPES: &[&str] = &[
  // these struct have bitpacked fields, currently unsupported.
  "VkAccelerationStructureInstanceKHR",
  "VkAccelerationStructureSRTMotionInstanceNV",
  "VkAccelerationStructureMotionInstanceDataNV",
  // these structs are blocked because they've got unions and so they're hard
  // for the generator to handle automatically right now.
  "VkAccelerationStructureBuildGeometryInfoKHR",
  "VkAccelerationStructureGeometryAabbsDataKHR",
  "VkAccelerationStructureGeometryInstancesDataKHR",
  "VkAccelerationStructureGeometryKHR",
  "VkAccelerationStructureGeometryMotionTrianglesDataNV",
  "VkAccelerationStructureGeometryTrianglesDataKHR",
  "VkAccelerationStructureMotionInstanceNV",
  "VkAccelerationStructureTrianglesOpacityMicromapEXT",
  "VkCopyAccelerationStructureToMemoryInfoKHR",
  "VkCopyMemoryToAccelerationStructureInfoKHR",
  "VkCopyMemoryToMicromapInfoEXT",
  "VkCopyMicromapToMemoryInfoEXT",
  "VkDescriptorGetInfoEXT",
  "VkMicromapBuildInfoEXT",
  "VkPerformanceValueINTEL",
  "VkPipelineExecutableStatisticKHR",
  "VkSamplerCustomBorderColorCreateInfoEXT",
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
  // this is some windowing thing, i think?
  "RROutput",
  // TODO: support whatever nvidia thing this stuff is for?
  "NvSciBufAttrList",
  "NvSciBufObj",
  "NvSciSyncAttrList",
  "NvSciSyncFence",
  "NvSciSyncObj",
  "VkSemaphoreSciSyncPoolCreateInfoNV",
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
  // Blocked because they're part of the private Google Games Platform.
  "GgpFrameToken",
  "GgpStreamDescriptor",
  "VkStreamDescriptorSurfaceCreateInfoGGP",
];

const BLOCKED_EXTENSIONS: &[&str] = &[
  // creates an alias to a vulkansc bit
  "VK_GOOGLE_extension_196",
];
