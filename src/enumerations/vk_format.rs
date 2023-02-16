#![allow(non_upper_case_globals)]

/// Khronos: [VkFormat](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormat.html)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkFormat(u32);
impl core::fmt::Debug for VkFormat {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match *self {
      VK_FORMAT_UNDEFINED => write!(f, "VK_FORMAT_UNDEFINED"),
      VK_FORMAT_R4G4_UNORM_PACK8 => write!(f, "VK_FORMAT_R4G4_UNORM_PACK8"),
      VK_FORMAT_R4G4B4A4_UNORM_PACK16 => write!(f, "VK_FORMAT_R4G4B4A4_UNORM_PACK16"),
      VK_FORMAT_B4G4R4A4_UNORM_PACK16 => write!(f, "VK_FORMAT_B4G4R4A4_UNORM_PACK16"),
      VK_FORMAT_R5G6B5_UNORM_PACK16 => write!(f, "VK_FORMAT_R5G6B5_UNORM_PACK16"),
      VK_FORMAT_B5G6R5_UNORM_PACK16 => write!(f, "VK_FORMAT_B5G6R5_UNORM_PACK16"),
      VK_FORMAT_R5G5B5A1_UNORM_PACK16 => write!(f, "VK_FORMAT_R5G5B5A1_UNORM_PACK16"),
      VK_FORMAT_B5G5R5A1_UNORM_PACK16 => write!(f, "VK_FORMAT_B5G5R5A1_UNORM_PACK16"),
      VK_FORMAT_A1R5G5B5_UNORM_PACK16 => write!(f, "VK_FORMAT_A1R5G5B5_UNORM_PACK16"),
      VK_FORMAT_R8_UNORM => write!(f, "VK_FORMAT_R8_UNORM"),
      VK_FORMAT_R8_SNORM => write!(f, "VK_FORMAT_R8_SNORM"),
      VK_FORMAT_R8_USCALED => write!(f, "VK_FORMAT_R8_USCALED"),
      VK_FORMAT_R8_SSCALED => write!(f, "VK_FORMAT_R8_SSCALED"),
      VK_FORMAT_R8_UINT => write!(f, "VK_FORMAT_R8_UINT"),
      VK_FORMAT_R8_SINT => write!(f, "VK_FORMAT_R8_SINT"),
      VK_FORMAT_R8_SRGB => write!(f, "VK_FORMAT_R8_SRGB"),
      VK_FORMAT_R8G8_UNORM => write!(f, "VK_FORMAT_R8G8_UNORM"),
      VK_FORMAT_R8G8_SNORM => write!(f, "VK_FORMAT_R8G8_SNORM"),
      VK_FORMAT_R8G8_USCALED => write!(f, "VK_FORMAT_R8G8_USCALED"),
      VK_FORMAT_R8G8_SSCALED => write!(f, "VK_FORMAT_R8G8_SSCALED"),
      VK_FORMAT_R8G8_UINT => write!(f, "VK_FORMAT_R8G8_UINT"),
      VK_FORMAT_R8G8_SINT => write!(f, "VK_FORMAT_R8G8_SINT"),
      VK_FORMAT_R8G8_SRGB => write!(f, "VK_FORMAT_R8G8_SRGB"),
      VK_FORMAT_R8G8B8_UNORM => write!(f, "VK_FORMAT_R8G8B8_UNORM"),
      VK_FORMAT_R8G8B8_SNORM => write!(f, "VK_FORMAT_R8G8B8_SNORM"),
      VK_FORMAT_R8G8B8_USCALED => write!(f, "VK_FORMAT_R8G8B8_USCALED"),
      VK_FORMAT_R8G8B8_SSCALED => write!(f, "VK_FORMAT_R8G8B8_SSCALED"),
      VK_FORMAT_R8G8B8_UINT => write!(f, "VK_FORMAT_R8G8B8_UINT"),
      VK_FORMAT_R8G8B8_SINT => write!(f, "VK_FORMAT_R8G8B8_SINT"),
      VK_FORMAT_R8G8B8_SRGB => write!(f, "VK_FORMAT_R8G8B8_SRGB"),
      VK_FORMAT_B8G8R8_UNORM => write!(f, "VK_FORMAT_B8G8R8_UNORM"),
      VK_FORMAT_B8G8R8_SNORM => write!(f, "VK_FORMAT_B8G8R8_SNORM"),
      VK_FORMAT_B8G8R8_USCALED => write!(f, "VK_FORMAT_B8G8R8_USCALED"),
      VK_FORMAT_B8G8R8_SSCALED => write!(f, "VK_FORMAT_B8G8R8_SSCALED"),
      VK_FORMAT_B8G8R8_UINT => write!(f, "VK_FORMAT_B8G8R8_UINT"),
      VK_FORMAT_B8G8R8_SINT => write!(f, "VK_FORMAT_B8G8R8_SINT"),
      VK_FORMAT_B8G8R8_SRGB => write!(f, "VK_FORMAT_B8G8R8_SRGB"),
      VK_FORMAT_R8G8B8A8_UNORM => write!(f, "VK_FORMAT_R8G8B8A8_UNORM"),
      VK_FORMAT_R8G8B8A8_SNORM => write!(f, "VK_FORMAT_R8G8B8A8_SNORM"),
      VK_FORMAT_R8G8B8A8_USCALED => write!(f, "VK_FORMAT_R8G8B8A8_USCALED"),
      VK_FORMAT_R8G8B8A8_SSCALED => write!(f, "VK_FORMAT_R8G8B8A8_SSCALED"),
      VK_FORMAT_R8G8B8A8_UINT => write!(f, "VK_FORMAT_R8G8B8A8_UINT"),
      VK_FORMAT_R8G8B8A8_SINT => write!(f, "VK_FORMAT_R8G8B8A8_SINT"),
      VK_FORMAT_R8G8B8A8_SRGB => write!(f, "VK_FORMAT_R8G8B8A8_SRGB"),
      VK_FORMAT_B8G8R8A8_UNORM => write!(f, "VK_FORMAT_B8G8R8A8_UNORM"),
      VK_FORMAT_B8G8R8A8_SNORM => write!(f, "VK_FORMAT_B8G8R8A8_SNORM"),
      VK_FORMAT_B8G8R8A8_USCALED => write!(f, "VK_FORMAT_B8G8R8A8_USCALED"),
      VK_FORMAT_B8G8R8A8_SSCALED => write!(f, "VK_FORMAT_B8G8R8A8_SSCALED"),
      VK_FORMAT_B8G8R8A8_UINT => write!(f, "VK_FORMAT_B8G8R8A8_UINT"),
      VK_FORMAT_B8G8R8A8_SINT => write!(f, "VK_FORMAT_B8G8R8A8_SINT"),
      VK_FORMAT_B8G8R8A8_SRGB => write!(f, "VK_FORMAT_B8G8R8A8_SRGB"),
      VK_FORMAT_A8B8G8R8_UNORM_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_UNORM_PACK32"),
      VK_FORMAT_A8B8G8R8_SNORM_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_SNORM_PACK32"),
      VK_FORMAT_A8B8G8R8_USCALED_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_USCALED_PACK32"),
      VK_FORMAT_A8B8G8R8_SSCALED_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_SSCALED_PACK32"),
      VK_FORMAT_A8B8G8R8_UINT_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_UINT_PACK32"),
      VK_FORMAT_A8B8G8R8_SINT_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_SINT_PACK32"),
      VK_FORMAT_A8B8G8R8_SRGB_PACK32 => write!(f, "VK_FORMAT_A8B8G8R8_SRGB_PACK32"),
      VK_FORMAT_A2R10G10B10_UNORM_PACK32 => write!(f, "VK_FORMAT_A2R10G10B10_UNORM_PACK32"),
      VK_FORMAT_A2R10G10B10_SNORM_PACK32 => write!(f, "VK_FORMAT_A2R10G10B10_SNORM_PACK32"),
      VK_FORMAT_A2R10G10B10_USCALED_PACK32 => write!(f, "VK_FORMAT_A2R10G10B10_USCALED_PACK32"),
      VK_FORMAT_A2R10G10B10_SSCALED_PACK32 => write!(f, "VK_FORMAT_A2R10G10B10_SSCALED_PACK32"),
      VK_FORMAT_A2R10G10B10_UINT_PACK32 => write!(f, "VK_FORMAT_A2R10G10B10_UINT_PACK32"),
      VK_FORMAT_A2R10G10B10_SINT_PACK32 => write!(f, "VK_FORMAT_A2R10G10B10_SINT_PACK32"),
      VK_FORMAT_A2B10G10R10_UNORM_PACK32 => write!(f, "VK_FORMAT_A2B10G10R10_UNORM_PACK32"),
      VK_FORMAT_A2B10G10R10_SNORM_PACK32 => write!(f, "VK_FORMAT_A2B10G10R10_SNORM_PACK32"),
      VK_FORMAT_A2B10G10R10_USCALED_PACK32 => write!(f, "VK_FORMAT_A2B10G10R10_USCALED_PACK32"),
      VK_FORMAT_A2B10G10R10_SSCALED_PACK32 => write!(f, "VK_FORMAT_A2B10G10R10_SSCALED_PACK32"),
      VK_FORMAT_A2B10G10R10_UINT_PACK32 => write!(f, "VK_FORMAT_A2B10G10R10_UINT_PACK32"),
      VK_FORMAT_A2B10G10R10_SINT_PACK32 => write!(f, "VK_FORMAT_A2B10G10R10_SINT_PACK32"),
      VK_FORMAT_R16_UNORM => write!(f, "VK_FORMAT_R16_UNORM"),
      VK_FORMAT_R16_SNORM => write!(f, "VK_FORMAT_R16_SNORM"),
      VK_FORMAT_R16_USCALED => write!(f, "VK_FORMAT_R16_USCALED"),
      VK_FORMAT_R16_SSCALED => write!(f, "VK_FORMAT_R16_SSCALED"),
      VK_FORMAT_R16_UINT => write!(f, "VK_FORMAT_R16_UINT"),
      VK_FORMAT_R16_SINT => write!(f, "VK_FORMAT_R16_SINT"),
      VK_FORMAT_R16_SFLOAT => write!(f, "VK_FORMAT_R16_SFLOAT"),
      VK_FORMAT_R16G16_UNORM => write!(f, "VK_FORMAT_R16G16_UNORM"),
      VK_FORMAT_R16G16_SNORM => write!(f, "VK_FORMAT_R16G16_SNORM"),
      VK_FORMAT_R16G16_USCALED => write!(f, "VK_FORMAT_R16G16_USCALED"),
      VK_FORMAT_R16G16_SSCALED => write!(f, "VK_FORMAT_R16G16_SSCALED"),
      VK_FORMAT_R16G16_UINT => write!(f, "VK_FORMAT_R16G16_UINT"),
      VK_FORMAT_R16G16_SINT => write!(f, "VK_FORMAT_R16G16_SINT"),
      VK_FORMAT_R16G16_SFLOAT => write!(f, "VK_FORMAT_R16G16_SFLOAT"),
      VK_FORMAT_R16G16B16_UNORM => write!(f, "VK_FORMAT_R16G16B16_UNORM"),
      VK_FORMAT_R16G16B16_SNORM => write!(f, "VK_FORMAT_R16G16B16_SNORM"),
      VK_FORMAT_R16G16B16_USCALED => write!(f, "VK_FORMAT_R16G16B16_USCALED"),
      VK_FORMAT_R16G16B16_SSCALED => write!(f, "VK_FORMAT_R16G16B16_SSCALED"),
      VK_FORMAT_R16G16B16_UINT => write!(f, "VK_FORMAT_R16G16B16_UINT"),
      VK_FORMAT_R16G16B16_SINT => write!(f, "VK_FORMAT_R16G16B16_SINT"),
      VK_FORMAT_R16G16B16_SFLOAT => write!(f, "VK_FORMAT_R16G16B16_SFLOAT"),
      VK_FORMAT_R16G16B16A16_UNORM => write!(f, "VK_FORMAT_R16G16B16A16_UNORM"),
      VK_FORMAT_R16G16B16A16_SNORM => write!(f, "VK_FORMAT_R16G16B16A16_SNORM"),
      VK_FORMAT_R16G16B16A16_USCALED => write!(f, "VK_FORMAT_R16G16B16A16_USCALED"),
      VK_FORMAT_R16G16B16A16_SSCALED => write!(f, "VK_FORMAT_R16G16B16A16_SSCALED"),
      VK_FORMAT_R16G16B16A16_UINT => write!(f, "VK_FORMAT_R16G16B16A16_UINT"),
      VK_FORMAT_R16G16B16A16_SINT => write!(f, "VK_FORMAT_R16G16B16A16_SINT"),
      VK_FORMAT_R16G16B16A16_SFLOAT => write!(f, "VK_FORMAT_R16G16B16A16_SFLOAT"),
      VK_FORMAT_R32_UINT => write!(f, "VK_FORMAT_R32_UINT"),
      VK_FORMAT_R32_SINT => write!(f, "VK_FORMAT_R32_SINT"),
      VK_FORMAT_R32_SFLOAT => write!(f, "VK_FORMAT_R32_SFLOAT"),
      VK_FORMAT_R32G32_UINT => write!(f, "VK_FORMAT_R32G32_UINT"),
      VK_FORMAT_R32G32_SINT => write!(f, "VK_FORMAT_R32G32_SINT"),
      VK_FORMAT_R32G32_SFLOAT => write!(f, "VK_FORMAT_R32G32_SFLOAT"),
      VK_FORMAT_R32G32B32_UINT => write!(f, "VK_FORMAT_R32G32B32_UINT"),
      VK_FORMAT_R32G32B32_SINT => write!(f, "VK_FORMAT_R32G32B32_SINT"),
      VK_FORMAT_R32G32B32_SFLOAT => write!(f, "VK_FORMAT_R32G32B32_SFLOAT"),
      VK_FORMAT_R32G32B32A32_UINT => write!(f, "VK_FORMAT_R32G32B32A32_UINT"),
      VK_FORMAT_R32G32B32A32_SINT => write!(f, "VK_FORMAT_R32G32B32A32_SINT"),
      VK_FORMAT_R32G32B32A32_SFLOAT => write!(f, "VK_FORMAT_R32G32B32A32_SFLOAT"),
      VK_FORMAT_R64_UINT => write!(f, "VK_FORMAT_R64_UINT"),
      VK_FORMAT_R64_SINT => write!(f, "VK_FORMAT_R64_SINT"),
      VK_FORMAT_R64_SFLOAT => write!(f, "VK_FORMAT_R64_SFLOAT"),
      VK_FORMAT_R64G64_UINT => write!(f, "VK_FORMAT_R64G64_UINT"),
      VK_FORMAT_R64G64_SINT => write!(f, "VK_FORMAT_R64G64_SINT"),
      VK_FORMAT_R64G64_SFLOAT => write!(f, "VK_FORMAT_R64G64_SFLOAT"),
      VK_FORMAT_R64G64B64_UINT => write!(f, "VK_FORMAT_R64G64B64_UINT"),
      VK_FORMAT_R64G64B64_SINT => write!(f, "VK_FORMAT_R64G64B64_SINT"),
      VK_FORMAT_R64G64B64_SFLOAT => write!(f, "VK_FORMAT_R64G64B64_SFLOAT"),
      VK_FORMAT_R64G64B64A64_UINT => write!(f, "VK_FORMAT_R64G64B64A64_UINT"),
      VK_FORMAT_R64G64B64A64_SINT => write!(f, "VK_FORMAT_R64G64B64A64_SINT"),
      VK_FORMAT_R64G64B64A64_SFLOAT => write!(f, "VK_FORMAT_R64G64B64A64_SFLOAT"),
      VK_FORMAT_B10G11R11_UFLOAT_PACK32 => write!(f, "VK_FORMAT_B10G11R11_UFLOAT_PACK32"),
      VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => write!(f, "VK_FORMAT_E5B9G9R9_UFLOAT_PACK32"),
      VK_FORMAT_D16_UNORM => write!(f, "VK_FORMAT_D16_UNORM"),
      VK_FORMAT_X8_D24_UNORM_PACK32 => write!(f, "VK_FORMAT_X8_D24_UNORM_PACK32"),
      VK_FORMAT_D32_SFLOAT => write!(f, "VK_FORMAT_D32_SFLOAT"),
      VK_FORMAT_S8_UINT => write!(f, "VK_FORMAT_S8_UINT"),
      VK_FORMAT_D16_UNORM_S8_UINT => write!(f, "VK_FORMAT_D16_UNORM_S8_UINT"),
      VK_FORMAT_D24_UNORM_S8_UINT => write!(f, "VK_FORMAT_D24_UNORM_S8_UINT"),
      VK_FORMAT_D32_SFLOAT_S8_UINT => write!(f, "VK_FORMAT_D32_SFLOAT_S8_UINT"),
      VK_FORMAT_BC1_RGB_UNORM_BLOCK => write!(f, "VK_FORMAT_BC1_RGB_UNORM_BLOCK"),
      VK_FORMAT_BC1_RGB_SRGB_BLOCK => write!(f, "VK_FORMAT_BC1_RGB_SRGB_BLOCK"),
      VK_FORMAT_BC1_RGBA_UNORM_BLOCK => write!(f, "VK_FORMAT_BC1_RGBA_UNORM_BLOCK"),
      VK_FORMAT_BC1_RGBA_SRGB_BLOCK => write!(f, "VK_FORMAT_BC1_RGBA_SRGB_BLOCK"),
      VK_FORMAT_BC2_UNORM_BLOCK => write!(f, "VK_FORMAT_BC2_UNORM_BLOCK"),
      VK_FORMAT_BC2_SRGB_BLOCK => write!(f, "VK_FORMAT_BC2_SRGB_BLOCK"),
      VK_FORMAT_BC3_UNORM_BLOCK => write!(f, "VK_FORMAT_BC3_UNORM_BLOCK"),
      VK_FORMAT_BC3_SRGB_BLOCK => write!(f, "VK_FORMAT_BC3_SRGB_BLOCK"),
      VK_FORMAT_BC4_UNORM_BLOCK => write!(f, "VK_FORMAT_BC4_UNORM_BLOCK"),
      VK_FORMAT_BC4_SNORM_BLOCK => write!(f, "VK_FORMAT_BC4_SNORM_BLOCK"),
      VK_FORMAT_BC5_UNORM_BLOCK => write!(f, "VK_FORMAT_BC5_UNORM_BLOCK"),
      VK_FORMAT_BC5_SNORM_BLOCK => write!(f, "VK_FORMAT_BC5_SNORM_BLOCK"),
      VK_FORMAT_BC6H_UFLOAT_BLOCK => write!(f, "VK_FORMAT_BC6H_UFLOAT_BLOCK"),
      VK_FORMAT_BC6H_SFLOAT_BLOCK => write!(f, "VK_FORMAT_BC6H_SFLOAT_BLOCK"),
      VK_FORMAT_BC7_UNORM_BLOCK => write!(f, "VK_FORMAT_BC7_UNORM_BLOCK"),
      VK_FORMAT_BC7_SRGB_BLOCK => write!(f, "VK_FORMAT_BC7_SRGB_BLOCK"),
      VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK => write!(f, "VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK"),
      VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK => write!(f, "VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK"),
      VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK => write!(f, "VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK"),
      VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK => write!(f, "VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK"),
      VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK => write!(f, "VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK"),
      VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK => write!(f, "VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK"),
      VK_FORMAT_EAC_R11_UNORM_BLOCK => write!(f, "VK_FORMAT_EAC_R11_UNORM_BLOCK"),
      VK_FORMAT_EAC_R11_SNORM_BLOCK => write!(f, "VK_FORMAT_EAC_R11_SNORM_BLOCK"),
      VK_FORMAT_EAC_R11G11_UNORM_BLOCK => write!(f, "VK_FORMAT_EAC_R11G11_UNORM_BLOCK"),
      VK_FORMAT_EAC_R11G11_SNORM_BLOCK => write!(f, "VK_FORMAT_EAC_R11G11_SNORM_BLOCK"),
      VK_FORMAT_ASTC_4x4_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_4x4_UNORM_BLOCK"),
      VK_FORMAT_ASTC_4x4_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_4x4_SRGB_BLOCK"),
      VK_FORMAT_ASTC_5x4_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_5x4_UNORM_BLOCK"),
      VK_FORMAT_ASTC_5x4_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_5x4_SRGB_BLOCK"),
      VK_FORMAT_ASTC_5x5_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_5x5_UNORM_BLOCK"),
      VK_FORMAT_ASTC_5x5_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_5x5_SRGB_BLOCK"),
      VK_FORMAT_ASTC_6x5_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_6x5_UNORM_BLOCK"),
      VK_FORMAT_ASTC_6x5_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_6x5_SRGB_BLOCK"),
      VK_FORMAT_ASTC_6x6_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_6x6_UNORM_BLOCK"),
      VK_FORMAT_ASTC_6x6_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_6x6_SRGB_BLOCK"),
      VK_FORMAT_ASTC_8x5_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_8x5_UNORM_BLOCK"),
      VK_FORMAT_ASTC_8x5_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_8x5_SRGB_BLOCK"),
      VK_FORMAT_ASTC_8x6_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_8x6_UNORM_BLOCK"),
      VK_FORMAT_ASTC_8x6_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_8x6_SRGB_BLOCK"),
      VK_FORMAT_ASTC_8x8_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_8x8_UNORM_BLOCK"),
      VK_FORMAT_ASTC_8x8_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_8x8_SRGB_BLOCK"),
      VK_FORMAT_ASTC_10x5_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_10x5_UNORM_BLOCK"),
      VK_FORMAT_ASTC_10x5_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_10x5_SRGB_BLOCK"),
      VK_FORMAT_ASTC_10x6_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_10x6_UNORM_BLOCK"),
      VK_FORMAT_ASTC_10x6_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_10x6_SRGB_BLOCK"),
      VK_FORMAT_ASTC_10x8_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_10x8_UNORM_BLOCK"),
      VK_FORMAT_ASTC_10x8_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_10x8_SRGB_BLOCK"),
      VK_FORMAT_ASTC_10x10_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_10x10_UNORM_BLOCK"),
      VK_FORMAT_ASTC_10x10_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_10x10_SRGB_BLOCK"),
      VK_FORMAT_ASTC_12x10_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_12x10_UNORM_BLOCK"),
      VK_FORMAT_ASTC_12x10_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_12x10_SRGB_BLOCK"),
      VK_FORMAT_ASTC_12x12_UNORM_BLOCK => write!(f, "VK_FORMAT_ASTC_12x12_UNORM_BLOCK"),
      VK_FORMAT_ASTC_12x12_SRGB_BLOCK => write!(f, "VK_FORMAT_ASTC_12x12_SRGB_BLOCK"),
      VK_FORMAT_G8B8G8R8_422_UNORM => write!(f, "VK_FORMAT_G8B8G8R8_422_UNORM"),
      VK_FORMAT_B8G8R8G8_422_UNORM => write!(f, "VK_FORMAT_B8G8R8G8_422_UNORM"),
      VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM => write!(f, "VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM"),
      VK_FORMAT_G8_B8R8_2PLANE_420_UNORM => write!(f, "VK_FORMAT_G8_B8R8_2PLANE_420_UNORM"),
      VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM => write!(f, "VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM"),
      VK_FORMAT_G8_B8R8_2PLANE_422_UNORM => write!(f, "VK_FORMAT_G8_B8R8_2PLANE_422_UNORM"),
      VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM => write!(f, "VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM"),
      VK_FORMAT_R10X6_UNORM_PACK16 => write!(f, "VK_FORMAT_R10X6_UNORM_PACK16"),
      VK_FORMAT_R10X6G10X6_UNORM_2PACK16 => write!(f, "VK_FORMAT_R10X6G10X6_UNORM_2PACK16"),
      VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16 => {
        write!(f, "VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16")
      }
      VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
        write!(f, "VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16")
      }
      VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
        write!(f, "VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16")
      }
      VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16")
      }
      VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16")
      }
      VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16")
      }
      VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16")
      }
      VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16")
      }
      VK_FORMAT_R12X4_UNORM_PACK16 => write!(f, "VK_FORMAT_R12X4_UNORM_PACK16"),
      VK_FORMAT_R12X4G12X4_UNORM_2PACK16 => write!(f, "VK_FORMAT_R12X4G12X4_UNORM_2PACK16"),
      VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16 => {
        write!(f, "VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16")
      }
      VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
        write!(f, "VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16")
      }
      VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
        write!(f, "VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16")
      }
      VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16")
      }
      VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16")
      }
      VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16")
      }
      VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16")
      }
      VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16")
      }
      VK_FORMAT_G16B16G16R16_422_UNORM => write!(f, "VK_FORMAT_G16B16G16R16_422_UNORM"),
      VK_FORMAT_B16G16R16G16_422_UNORM => write!(f, "VK_FORMAT_B16G16R16G16_422_UNORM"),
      VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM => write!(f, "VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM"),
      VK_FORMAT_G16_B16R16_2PLANE_420_UNORM => write!(f, "VK_FORMAT_G16_B16R16_2PLANE_420_UNORM"),
      VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM => write!(f, "VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM"),
      VK_FORMAT_G16_B16R16_2PLANE_422_UNORM => write!(f, "VK_FORMAT_G16_B16R16_2PLANE_422_UNORM"),
      VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM => write!(f, "VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM"),
      VK_FORMAT_G8_B8R8_2PLANE_444_UNORM => write!(f, "VK_FORMAT_G8_B8R8_2PLANE_444_UNORM"),
      VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16")
      }
      VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => {
        write!(f, "VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16")
      }
      VK_FORMAT_G16_B16R16_2PLANE_444_UNORM => write!(f, "VK_FORMAT_G16_B16R16_2PLANE_444_UNORM"),
      VK_FORMAT_A4R4G4B4_UNORM_PACK16 => write!(f, "VK_FORMAT_A4R4G4B4_UNORM_PACK16"),
      VK_FORMAT_A4B4G4R4_UNORM_PACK16 => write!(f, "VK_FORMAT_A4B4G4R4_UNORM_PACK16"),
      VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK"),
      VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK => write!(f, "VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK"),
      other => write!(f, "VkFormat({})", other.0),
    }
  }
}
pub const VK_FORMAT_UNDEFINED: VkFormat = VkFormat(0);
pub const VK_FORMAT_R4G4_UNORM_PACK8: VkFormat = VkFormat(1);
pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: VkFormat = VkFormat(2);
pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: VkFormat = VkFormat(3);
pub const VK_FORMAT_R5G6B5_UNORM_PACK16: VkFormat = VkFormat(4);
pub const VK_FORMAT_B5G6R5_UNORM_PACK16: VkFormat = VkFormat(5);
pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: VkFormat = VkFormat(6);
pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: VkFormat = VkFormat(7);
pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: VkFormat = VkFormat(8);
pub const VK_FORMAT_R8_UNORM: VkFormat = VkFormat(9);
pub const VK_FORMAT_R8_SNORM: VkFormat = VkFormat(10);
pub const VK_FORMAT_R8_USCALED: VkFormat = VkFormat(11);
pub const VK_FORMAT_R8_SSCALED: VkFormat = VkFormat(12);
pub const VK_FORMAT_R8_UINT: VkFormat = VkFormat(13);
pub const VK_FORMAT_R8_SINT: VkFormat = VkFormat(14);
pub const VK_FORMAT_R8_SRGB: VkFormat = VkFormat(15);
pub const VK_FORMAT_R8G8_UNORM: VkFormat = VkFormat(16);
pub const VK_FORMAT_R8G8_SNORM: VkFormat = VkFormat(17);
pub const VK_FORMAT_R8G8_USCALED: VkFormat = VkFormat(18);
pub const VK_FORMAT_R8G8_SSCALED: VkFormat = VkFormat(19);
pub const VK_FORMAT_R8G8_UINT: VkFormat = VkFormat(20);
pub const VK_FORMAT_R8G8_SINT: VkFormat = VkFormat(21);
pub const VK_FORMAT_R8G8_SRGB: VkFormat = VkFormat(22);
pub const VK_FORMAT_R8G8B8_UNORM: VkFormat = VkFormat(23);
pub const VK_FORMAT_R8G8B8_SNORM: VkFormat = VkFormat(24);
pub const VK_FORMAT_R8G8B8_USCALED: VkFormat = VkFormat(25);
pub const VK_FORMAT_R8G8B8_SSCALED: VkFormat = VkFormat(26);
pub const VK_FORMAT_R8G8B8_UINT: VkFormat = VkFormat(27);
pub const VK_FORMAT_R8G8B8_SINT: VkFormat = VkFormat(28);
pub const VK_FORMAT_R8G8B8_SRGB: VkFormat = VkFormat(29);
pub const VK_FORMAT_B8G8R8_UNORM: VkFormat = VkFormat(30);
pub const VK_FORMAT_B8G8R8_SNORM: VkFormat = VkFormat(31);
pub const VK_FORMAT_B8G8R8_USCALED: VkFormat = VkFormat(32);
pub const VK_FORMAT_B8G8R8_SSCALED: VkFormat = VkFormat(33);
pub const VK_FORMAT_B8G8R8_UINT: VkFormat = VkFormat(34);
pub const VK_FORMAT_B8G8R8_SINT: VkFormat = VkFormat(35);
pub const VK_FORMAT_B8G8R8_SRGB: VkFormat = VkFormat(36);
pub const VK_FORMAT_R8G8B8A8_UNORM: VkFormat = VkFormat(37);
pub const VK_FORMAT_R8G8B8A8_SNORM: VkFormat = VkFormat(38);
pub const VK_FORMAT_R8G8B8A8_USCALED: VkFormat = VkFormat(39);
pub const VK_FORMAT_R8G8B8A8_SSCALED: VkFormat = VkFormat(40);
pub const VK_FORMAT_R8G8B8A8_UINT: VkFormat = VkFormat(41);
pub const VK_FORMAT_R8G8B8A8_SINT: VkFormat = VkFormat(42);
pub const VK_FORMAT_R8G8B8A8_SRGB: VkFormat = VkFormat(43);
pub const VK_FORMAT_B8G8R8A8_UNORM: VkFormat = VkFormat(44);
pub const VK_FORMAT_B8G8R8A8_SNORM: VkFormat = VkFormat(45);
pub const VK_FORMAT_B8G8R8A8_USCALED: VkFormat = VkFormat(46);
pub const VK_FORMAT_B8G8R8A8_SSCALED: VkFormat = VkFormat(47);
pub const VK_FORMAT_B8G8R8A8_UINT: VkFormat = VkFormat(48);
pub const VK_FORMAT_B8G8R8A8_SINT: VkFormat = VkFormat(49);
pub const VK_FORMAT_B8G8R8A8_SRGB: VkFormat = VkFormat(50);
pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: VkFormat = VkFormat(51);
pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: VkFormat = VkFormat(52);
pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: VkFormat = VkFormat(53);
pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: VkFormat = VkFormat(54);
pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: VkFormat = VkFormat(55);
pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: VkFormat = VkFormat(56);
pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: VkFormat = VkFormat(57);
pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: VkFormat = VkFormat(58);
pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: VkFormat = VkFormat(59);
pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: VkFormat = VkFormat(60);
pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: VkFormat = VkFormat(61);
pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: VkFormat = VkFormat(62);
pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: VkFormat = VkFormat(63);
pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: VkFormat = VkFormat(64);
pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: VkFormat = VkFormat(65);
pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: VkFormat = VkFormat(66);
pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: VkFormat = VkFormat(67);
pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: VkFormat = VkFormat(68);
pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: VkFormat = VkFormat(69);
pub const VK_FORMAT_R16_UNORM: VkFormat = VkFormat(70);
pub const VK_FORMAT_R16_SNORM: VkFormat = VkFormat(71);
pub const VK_FORMAT_R16_USCALED: VkFormat = VkFormat(72);
pub const VK_FORMAT_R16_SSCALED: VkFormat = VkFormat(73);
pub const VK_FORMAT_R16_UINT: VkFormat = VkFormat(74);
pub const VK_FORMAT_R16_SINT: VkFormat = VkFormat(75);
pub const VK_FORMAT_R16_SFLOAT: VkFormat = VkFormat(76);
pub const VK_FORMAT_R16G16_UNORM: VkFormat = VkFormat(77);
pub const VK_FORMAT_R16G16_SNORM: VkFormat = VkFormat(78);
pub const VK_FORMAT_R16G16_USCALED: VkFormat = VkFormat(79);
pub const VK_FORMAT_R16G16_SSCALED: VkFormat = VkFormat(80);
pub const VK_FORMAT_R16G16_UINT: VkFormat = VkFormat(81);
pub const VK_FORMAT_R16G16_SINT: VkFormat = VkFormat(82);
pub const VK_FORMAT_R16G16_SFLOAT: VkFormat = VkFormat(83);
pub const VK_FORMAT_R16G16B16_UNORM: VkFormat = VkFormat(84);
pub const VK_FORMAT_R16G16B16_SNORM: VkFormat = VkFormat(85);
pub const VK_FORMAT_R16G16B16_USCALED: VkFormat = VkFormat(86);
pub const VK_FORMAT_R16G16B16_SSCALED: VkFormat = VkFormat(87);
pub const VK_FORMAT_R16G16B16_UINT: VkFormat = VkFormat(88);
pub const VK_FORMAT_R16G16B16_SINT: VkFormat = VkFormat(89);
pub const VK_FORMAT_R16G16B16_SFLOAT: VkFormat = VkFormat(90);
pub const VK_FORMAT_R16G16B16A16_UNORM: VkFormat = VkFormat(91);
pub const VK_FORMAT_R16G16B16A16_SNORM: VkFormat = VkFormat(92);
pub const VK_FORMAT_R16G16B16A16_USCALED: VkFormat = VkFormat(93);
pub const VK_FORMAT_R16G16B16A16_SSCALED: VkFormat = VkFormat(94);
pub const VK_FORMAT_R16G16B16A16_UINT: VkFormat = VkFormat(95);
pub const VK_FORMAT_R16G16B16A16_SINT: VkFormat = VkFormat(96);
pub const VK_FORMAT_R16G16B16A16_SFLOAT: VkFormat = VkFormat(97);
pub const VK_FORMAT_R32_UINT: VkFormat = VkFormat(98);
pub const VK_FORMAT_R32_SINT: VkFormat = VkFormat(99);
pub const VK_FORMAT_R32_SFLOAT: VkFormat = VkFormat(100);
pub const VK_FORMAT_R32G32_UINT: VkFormat = VkFormat(101);
pub const VK_FORMAT_R32G32_SINT: VkFormat = VkFormat(102);
pub const VK_FORMAT_R32G32_SFLOAT: VkFormat = VkFormat(103);
pub const VK_FORMAT_R32G32B32_UINT: VkFormat = VkFormat(104);
pub const VK_FORMAT_R32G32B32_SINT: VkFormat = VkFormat(105);
pub const VK_FORMAT_R32G32B32_SFLOAT: VkFormat = VkFormat(106);
pub const VK_FORMAT_R32G32B32A32_UINT: VkFormat = VkFormat(107);
pub const VK_FORMAT_R32G32B32A32_SINT: VkFormat = VkFormat(108);
pub const VK_FORMAT_R32G32B32A32_SFLOAT: VkFormat = VkFormat(109);
pub const VK_FORMAT_R64_UINT: VkFormat = VkFormat(110);
pub const VK_FORMAT_R64_SINT: VkFormat = VkFormat(111);
pub const VK_FORMAT_R64_SFLOAT: VkFormat = VkFormat(112);
pub const VK_FORMAT_R64G64_UINT: VkFormat = VkFormat(113);
pub const VK_FORMAT_R64G64_SINT: VkFormat = VkFormat(114);
pub const VK_FORMAT_R64G64_SFLOAT: VkFormat = VkFormat(115);
pub const VK_FORMAT_R64G64B64_UINT: VkFormat = VkFormat(116);
pub const VK_FORMAT_R64G64B64_SINT: VkFormat = VkFormat(117);
pub const VK_FORMAT_R64G64B64_SFLOAT: VkFormat = VkFormat(118);
pub const VK_FORMAT_R64G64B64A64_UINT: VkFormat = VkFormat(119);
pub const VK_FORMAT_R64G64B64A64_SINT: VkFormat = VkFormat(120);
pub const VK_FORMAT_R64G64B64A64_SFLOAT: VkFormat = VkFormat(121);
pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: VkFormat = VkFormat(122);
pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: VkFormat = VkFormat(123);
pub const VK_FORMAT_D16_UNORM: VkFormat = VkFormat(124);
pub const VK_FORMAT_X8_D24_UNORM_PACK32: VkFormat = VkFormat(125);
pub const VK_FORMAT_D32_SFLOAT: VkFormat = VkFormat(126);
pub const VK_FORMAT_S8_UINT: VkFormat = VkFormat(127);
pub const VK_FORMAT_D16_UNORM_S8_UINT: VkFormat = VkFormat(128);
pub const VK_FORMAT_D24_UNORM_S8_UINT: VkFormat = VkFormat(129);
pub const VK_FORMAT_D32_SFLOAT_S8_UINT: VkFormat = VkFormat(130);
pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: VkFormat = VkFormat(131);
pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: VkFormat = VkFormat(132);
pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: VkFormat = VkFormat(133);
pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: VkFormat = VkFormat(134);
pub const VK_FORMAT_BC2_UNORM_BLOCK: VkFormat = VkFormat(135);
pub const VK_FORMAT_BC2_SRGB_BLOCK: VkFormat = VkFormat(136);
pub const VK_FORMAT_BC3_UNORM_BLOCK: VkFormat = VkFormat(137);
pub const VK_FORMAT_BC3_SRGB_BLOCK: VkFormat = VkFormat(138);
pub const VK_FORMAT_BC4_UNORM_BLOCK: VkFormat = VkFormat(139);
pub const VK_FORMAT_BC4_SNORM_BLOCK: VkFormat = VkFormat(140);
pub const VK_FORMAT_BC5_UNORM_BLOCK: VkFormat = VkFormat(141);
pub const VK_FORMAT_BC5_SNORM_BLOCK: VkFormat = VkFormat(142);
pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: VkFormat = VkFormat(143);
pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: VkFormat = VkFormat(144);
pub const VK_FORMAT_BC7_UNORM_BLOCK: VkFormat = VkFormat(145);
pub const VK_FORMAT_BC7_SRGB_BLOCK: VkFormat = VkFormat(146);
pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: VkFormat = VkFormat(147);
pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: VkFormat = VkFormat(148);
pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: VkFormat = VkFormat(149);
pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: VkFormat = VkFormat(150);
pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: VkFormat = VkFormat(151);
pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: VkFormat = VkFormat(152);
pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: VkFormat = VkFormat(153);
pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: VkFormat = VkFormat(154);
pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: VkFormat = VkFormat(155);
pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: VkFormat = VkFormat(156);
pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: VkFormat = VkFormat(157);
pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: VkFormat = VkFormat(158);
pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: VkFormat = VkFormat(159);
pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: VkFormat = VkFormat(160);
pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: VkFormat = VkFormat(161);
pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: VkFormat = VkFormat(162);
pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: VkFormat = VkFormat(163);
pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: VkFormat = VkFormat(164);
pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: VkFormat = VkFormat(165);
pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: VkFormat = VkFormat(166);
pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: VkFormat = VkFormat(167);
pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: VkFormat = VkFormat(168);
pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: VkFormat = VkFormat(169);
pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: VkFormat = VkFormat(170);
pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: VkFormat = VkFormat(171);
pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: VkFormat = VkFormat(172);
pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: VkFormat = VkFormat(173);
pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: VkFormat = VkFormat(174);
pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: VkFormat = VkFormat(175);
pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: VkFormat = VkFormat(176);
pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: VkFormat = VkFormat(177);
pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: VkFormat = VkFormat(178);
pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: VkFormat = VkFormat(179);
pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: VkFormat = VkFormat(180);
pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: VkFormat = VkFormat(181);
pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: VkFormat = VkFormat(182);
pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: VkFormat = VkFormat(183);
pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: VkFormat = VkFormat(184);
pub const VK_FORMAT_G8B8G8R8_422_UNORM: VkFormat = VkFormat(1000156000);
pub const VK_FORMAT_B8G8R8G8_422_UNORM: VkFormat = VkFormat(1000156001);
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: VkFormat = VkFormat(1000156002);
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: VkFormat = VkFormat(1000156003);
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: VkFormat = VkFormat(1000156004);
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: VkFormat = VkFormat(1000156005);
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: VkFormat = VkFormat(1000156006);
pub const VK_FORMAT_R10X6_UNORM_PACK16: VkFormat = VkFormat(1000156007);
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: VkFormat = VkFormat(1000156008);
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: VkFormat = VkFormat(1000156009);
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: VkFormat = VkFormat(1000156010);
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: VkFormat = VkFormat(1000156011);
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: VkFormat = VkFormat(1000156012);
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: VkFormat = VkFormat(1000156013);
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: VkFormat = VkFormat(1000156014);
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: VkFormat = VkFormat(1000156015);
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: VkFormat = VkFormat(1000156016);
pub const VK_FORMAT_R12X4_UNORM_PACK16: VkFormat = VkFormat(1000156017);
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: VkFormat = VkFormat(1000156018);
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: VkFormat = VkFormat(1000156019);
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: VkFormat = VkFormat(1000156020);
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: VkFormat = VkFormat(1000156021);
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: VkFormat = VkFormat(1000156022);
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: VkFormat = VkFormat(1000156023);
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: VkFormat = VkFormat(1000156024);
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: VkFormat = VkFormat(1000156025);
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: VkFormat = VkFormat(1000156026);
pub const VK_FORMAT_G16B16G16R16_422_UNORM: VkFormat = VkFormat(1000156027);
pub const VK_FORMAT_B16G16R16G16_422_UNORM: VkFormat = VkFormat(1000156028);
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: VkFormat = VkFormat(1000156029);
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: VkFormat = VkFormat(1000156030);
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: VkFormat = VkFormat(1000156031);
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: VkFormat = VkFormat(1000156032);
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: VkFormat = VkFormat(1000156033);
pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM: VkFormat = VkFormat(1000330000);
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: VkFormat = VkFormat(1000330001);
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: VkFormat = VkFormat(1000330002);
pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM: VkFormat = VkFormat(1000330003);
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16: VkFormat = VkFormat(1000340000);
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16: VkFormat = VkFormat(1000340001);
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK: VkFormat = VkFormat(1000066000);
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK: VkFormat = VkFormat(1000066001);
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK: VkFormat = VkFormat(1000066002);
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK: VkFormat = VkFormat(1000066003);
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK: VkFormat = VkFormat(1000066004);
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK: VkFormat = VkFormat(1000066005);
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK: VkFormat = VkFormat(1000066006);
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK: VkFormat = VkFormat(1000066007);
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK: VkFormat = VkFormat(1000066008);
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK: VkFormat = VkFormat(1000066009);
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK: VkFormat = VkFormat(1000066010);
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK: VkFormat = VkFormat(1000066011);
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK: VkFormat = VkFormat(1000066012);
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK: VkFormat = VkFormat(1000066013);