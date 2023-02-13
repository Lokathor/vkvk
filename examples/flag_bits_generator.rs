fn main() {
  flag_bits(
    "VkInstanceCreate",
    &[("VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR", "0x00000001", Some("specifies that the instance will enumerate available Vulkan Portability-compliant physical devices and groups in addition to the Vulkan physical devices and groups that are enumerated by default."))],
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
}
