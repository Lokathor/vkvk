#[rustfmt::skip]
fn main() {
  println!("#![allow(nonstandard_style)]");
  println!("use super::*;");
  fn_type("vkGetInstanceProcAddr", "(instance: VkInstance, name: *const u8) -> PFN_vkVoidFunction");
  fn_type("vkEnumerateInstanceVersion", "(api_version: *mut VkVersion) -> VkResult");
  fn_type("vkEnumerateInstanceLayerProperties", "(property_count: *mut uint32_t, properties: *mut VkLayerProperties) -> VkResult");
  fn_type("vkEnumerateInstanceExtensionProperties", "(layer_name: *const u8, property_count: *mut uint32_t,  properties: *mut VkExtensionProperties) -> VkResult");
}

fn fn_type(fn_name: &str, sig: &str) {
  println!(
    "/// Khronos: [{fn_name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{fn_name}.html)
    pub type {fn_name}_t = unsafe extern \"system\" fn{sig};"
  )
}
