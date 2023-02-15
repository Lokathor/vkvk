/// cargo run --example fn_ty_generator >target/fn_types.rs
#[rustfmt::skip]
fn main() {
  fn_type("vkGetSwapchainImagesKHR", "(
    device:VkDevice                                    device,
    swapchain:VkSwapchainKHR                              swapchain,
    swapchain_image_count: *mut uint32_t,
    swapchain_images: *mut VkImage) -> VkResult");
}

fn fn_type(fn_name: &str, sig: &str) {
  println!(
    "/// Khronos: [{fn_name}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{fn_name}.html)
    pub type {fn_name}_t = unsafe extern \"system\" fn{sig};
    pub const {fn_name}_NAME: &str = \"{fn_name}\\0\";"
  );
  println!();
}
