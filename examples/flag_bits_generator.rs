/// cargo run --example flag_bits_generator >target/flag_bits.rs
fn main() {
  flag_bits("VkDeviceQueueCreate");
}

fn flag_bits(name: &str) {
  println!(
    "/// Khronos: [{name}FlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}FlagBits.html)
    pub type {name}Flags = {name}FlagBits;
    /// Khronos: [{name}FlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{name}FlagBits.html)
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct {name}FlagBits(pub u32);"
  );
}
