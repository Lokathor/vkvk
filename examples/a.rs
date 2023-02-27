use vkvk::prelude::*;

fn main() {
  let entry = Entry::LINKED;

  let instance = {
    let mut instance_extensions =
      vec![ZString::try_from(VK_KHR_SURFACE_EXTENSION_NAME).unwrap()];
    let mut instance_layers = Vec::new();

    if cfg!(target_os = "macos") {
      const PORTABILITY: &str = "VK_KHR_portability_enumeration";
      if entry
        .enumerate_instance_extension_properties(None)
        .unwrap_or_default()
        .iter()
        .any(|x| x.extension_name.as_str() == PORTABILITY)
      {
        instance_extensions.push(ZString::try_from(PORTABILITY).unwrap())
      } else {
        println!(
          "Problem: running on mac but wasn't able to find the portability extension!"
        );
      }
    }

    if cfg!(debug_assertions) {
      const VALIDATION: &str = "VK_LAYER_KHRONOS_validation";
      if entry
        .enumerate_instance_layer_properties()
        .unwrap_or_default()
        .iter()
        .any(|l| l.layer_name.as_str() == VALIDATION)
      {
        instance_layers.push(ZString::try_from(VALIDATION).unwrap())
      } else {
        println!("Info: Running a debug build of the program but validation layers aren't available");
      }
    }

    let instance_layer_strs: Vec<ZStr<'_>> =
      instance_layers.iter().map(|s| s.as_zstr()).collect();
    let instance_extension_strs: Vec<ZStr<'_>> =
      instance_extensions.iter().map(|s| s.as_zstr()).collect();
    entry
      .create_instance(
        Some(ZString::try_from("vkvk example").unwrap().as_zstr()),
        1,
        None,
        0,
        VkVersion::API_1_0,
        VkInstanceCreateFlags::none(),
        &instance_layer_strs,
        &instance_extension_strs,
      )
      .unwrap()
  };

  // TODO: enumerate physical devices and pick one

  // TODO: create a device from a physical device.

  instance.destroy();
}
