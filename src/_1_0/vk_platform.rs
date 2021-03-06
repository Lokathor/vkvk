#![allow(dead_code)]

//! The parts of the Vulkan API that are presumed to be provided by an outside
//! source.

pub use core::ffi::c_void;

pub(crate) type void = c_void;
pub(crate) type char = u8;
pub(crate) type float = f32;
pub(crate) type double = f64;
pub(crate) type uint8_t = u8;
pub(crate) type uint16_t = u16;
pub(crate) type uint32_t = u32;
pub(crate) type uint64_t = u64;
pub(crate) type int32_t = i32;
pub(crate) type int64_t = i64;
pub(crate) type size_t = usize;

/// Basically this is `i32`
///
/// Technically it could be `i16` or `i164` on rare platforms, so we use this
/// alias to aid any potential future porting. However, for all current
/// platforms that support both Rust and Vulkan, it will be `i32`.
pub type int = i32;
