/// cargo run --example flag_bits_generator >target/flag_bits.rs
fn main() {
  //flag_bits("VkImageCreate", Some("KHR"));
  flag_bits("VkImageViewCreate", None);
}

fn flag_bits(name: &str, opt_ext: Option<&str>) {
  let ext = opt_ext.unwrap_or("");
  println!(
    "/// Khronos: [{name}FlagBits{ext}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}FlagBits{ext}.html)
    pub type {name}Flags{ext} = {name}FlagBits{ext};
    /// Khronos: [{name}FlagBits{ext}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}FlagBits{ext}.html)
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct {name}FlagBits{ext}(pub u32);"
  );
  println!();
}
