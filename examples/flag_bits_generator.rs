/// cargo run --example flag_bits_generator >target/flag_bits.rs
fn main() {
  flag_bits(
    "VkInstanceCreate",
    &[("VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR", "0x00000001", Some("specifies that the instance will enumerate available Vulkan Portability-compliant physical devices and groups in addition to the Vulkan physical devices and groups that are enumerated by default."))],
  );
  flag_bits(
    "VkSampleCount",
    &[
      ("VK_SAMPLE_COUNT_1_BIT", "0x00000001", Some("specifies an image with 1 sample per pixel.")),
      ("VK_SAMPLE_COUNT_2_BIT", "0x00000002", Some("specifies an image with 2 samples per pixel.")),
      ("VK_SAMPLE_COUNT_4_BIT", "0x00000004", Some("specifies an image with 4 samples per pixel.")),
      ("VK_SAMPLE_COUNT_8_BIT", "0x00000008", Some("specifies an image with 8 samples per pixel.")),
      (
        "VK_SAMPLE_COUNT_16_BIT",
        "0x00000010",
        Some("specifies an image with 16 samples per pixel."),
      ),
      (
        "VK_SAMPLE_COUNT_32_BIT",
        "0x00000020",
        Some("specifies an image with 32 samples per pixel."),
      ),
      (
        "VK_SAMPLE_COUNT_64_BIT",
        "0x00000040",
        Some("specifies an image with 64 samples per pixel."),
      ),
    ],
  );
}

fn flag_bits(name: &str, decls: &[(&str, &str, Option<&str>)]) {
  println!(
    "/// Khronos: [{name}FlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}FlagBits.html)
    pub type {name}Flags = {name}FlagBits;
    /// Khronos: [{name}FlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}FlagBits.html)
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct {name}FlagBits(pub u32);"
  );
  for (decl_name, decl_val, decl_note) in decls {
    if let Some(note) = *decl_note {
      println!("/// {note}");
    }
    println!("pub const {decl_name}: {name}FlagBits = {name}FlagBits({decl_val});");
  }
  println!();
}
